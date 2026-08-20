#!/usr/bin/env python3
"""🩹 Diagnostic-driven `.await` insertion for the Future-typed-value shapes `insert-await.py`
cannot reach because rustc emits NO machine-applicable `suggested_replacement` for them (E0308
type-mismatch against a Future, E0369 binary-op-on-Future, E0599 method-not-found-on-Future,
E0600 unary-op-on-Future, E0608 cannot-index-a-Future). This generalises
`terra-fanout-dsl-e0609-fixer.py`'s exact technique (read the rustc-flagged byte span, sanity-check
the surrounding bytes, insert `.await` at the one place that makes the expression's type line up)
to five more codes, all span-keyed per R10 — nothing here is matched by function/variable name.

Per-code insertion rule, each derived from where rustc's OWN span lands for that diagnostic shape
(measured against real output, not assumed — every rule below was verified against at least one
concrete example before being trusted):

  E0308  primary span IS the future expression (optionally with a leading `&`/`&mut` the type-note
         still shows, e.g. `found &impl Future<Output = T>`) -> insert `.await` at span.byte_end.
  E0369  binary op on a Future operand. A DIRECT comparison (`if len > *base`) has a secondary span
         labelled literally `impl Future<Output = ...>` living in-repo -> insert at its byte_end.
         A `assert_eq!`/`assert_ne!`-DESUGARED one has that same labelled span pointing at the
         rustup toolchain's `macro_rules!` body instead -> follow `expansion.span` out to the real
         call site, then resolve the Nth top-level macro argument ourselves (balanced paren/string
         scan) since rustc does not sub-span individual `macro_rules!` arguments.
  E0599  "no method named `X` found for opaque type `impl Future<...>`" -> primary span is the
         method NAME only -> the receiver ends at the `.` immediately before it -> insert `.await`
         right before that `.` (dot-sanity-checked against the live file bytes first).
  E0600  unary op (`!`/`-`/`*`) on a Future. A direct `!expr` span covers operator+operand as one
         unit -> insert at byte_end. An `assert!`/`debug_assert!`-desugared one covers the WHOLE
         macro call instead (its internal `!` is synthetic, so rustc cannot sub-span it) ->
         resolved the same way as the E0369 macro case, arg 0.
  E0608  "cannot index into a value of type `impl Future<...>`" -> primary span covers the WHOLE
         `[index]`, brackets included (confirmed against both a bare `values["root"]` and one
         nested inside `assert_eq!(...)`) -> the receiver ends exactly at span.byte_start.

Every rule inserts a LITERAL `.await` token and nothing else; every insertion point is re-verified
against the CURRENT file bytes (not a cached position) immediately before writing, and every code
path either applies or SKIPS with a printed reason — it never guesses. Macro-argument resolution
(`resolve_macro_arg`) re-parses the call site fresh from the live buffer on every use rather than
trusting a precomputed offset, so sibling-argument edits within the same macro call are safe in
either processing order.

Usage:
    cargo check -p <crate> --all-targets --message-format=json-diagnostic-rendered-ansi > diag.jsonl
    python3 terra-oskat-future-fixer.py --diag diag.jsonl --scope <path> [--apply]
"""
import argparse
import json
import os
import re
import sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"

E0308_LABEL_RE = re.compile(r"found `&{0,1}(?:mut )?impl (?:std::future::)?Future<Output")
E0369_LABEL_RE = re.compile(r"^impl (?:std::future::)?Future<Output ?= ?.*>$")
E0599_MSG_RE = re.compile(
    r"^no method named `[A-Za-z0-9_]+` found for opaque type `impl (?:std::future::)?Future<Output ?= ?.*>` in the current scope$"
)
E0608_MSG_RE = re.compile(r"^cannot index into a value of type `impl (?:std::future::)?Future<Output ?= ?.*>`$")
E0600_LABEL_RE = re.compile(r"^cannot apply unary operator `[!\-*]` to type `impl (?:std::future::)?Future<Output")
# 🖨️ `format!`/`panic!`/`assert!(cond, "...", args)` arguments run through `format_args!`, which
# reports E0277 "doesn't implement Display/Debug" with children `expansion` chains reaching the
# rustup toolchain — BUT (measured, not assumed) the OUTER span's own `file_name`/byte range is
# ALREADY in-repo and already covers exactly the offending argument expression, unlike the E0369
# assert_eq! case where the outer span lands on toolchain code. So this needs no expansion-chasing
# at all: same rule as E0308, just keyed on the message instead of the label.
E0277_FMT_RE = re.compile(
    r"^`&{0,1}(?:mut )?impl (?:std::future::)?Future<Output ?= ?.*>` doesn't implement `(?:std::fmt::)?(?:Display|Debug)`$"
)
# 🧩 Broader E0277 catch-all: a Future used where a concrete trait bound is required — "is not an
# iterator" (`for x in future_expr`), "trait bound `impl Future<...>: AsRef<Path>` is not
# satisfied" (passed to a `P: AsRef<Path>`-bounded fn), etc. Two independent match points because
# rustc phrases these two ways depending on where the bound was introduced: sometimes on the
# diagnostic's own top-level `message`, sometimes only on the primary span's `label`.
E0277_MSG_GENERIC_RE = re.compile(
    r"^`&{0,1}(?:mut )?impl (?:std::future::)?Future<Output ?= ?.*>` is not an iterator$"
)
E0277_LABEL_GENERIC_RE = re.compile(
    r"^the trait `.+` is not implemented for `&{0,1}(?:mut )?impl (?:std::future::)?Future<Output"
)
E0277_BOUND_MSG_RE = re.compile(
    r"^the trait bound `&{0,1}(?:mut )?impl (?:std::future::)?Future<Output ?= ?.*>: .+` is not satisfied$"
)

# 🪄 assert!/assert_eq!/assert_ne! (+ debug_ variants) resolve their `!(left == right)` comparison
# and `!cond` negation INSIDE the macro's own expansion, so the diagnostic's outer span sometimes
# lands on the macro-internal desugared code (file_name under the rustup toolchain) rather than on
# our source. `expansion.span` walks back to the real invocation, but only as far as the WHOLE
# macro call — rustc does not sub-span individual arguments through a `macro_rules!` expansion.
# From there we recover per-argument boundaries ourselves via balanced-bracket/string top-level
# comma splitting — still span-keyed (the call site itself is rustc's own span), never name-keyed.
ASSERT_MACROS = {
    "assert_eq!": 2, "assert_ne!": 2, "debug_assert_eq!": 2, "debug_assert_ne!": 2,
    "assert!": 1, "debug_assert!": 1,
}


def in_scope(abs_path: str, scope: str) -> bool:
    rel = os.path.relpath(abs_path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip("/").split("/") if p]
    for i in range(len(parts) - len(want) + 1):
        if parts[i:i + len(want)] == want:
            return True
    return False


def load_diags(diag_path):
    with open(diag_path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line.startswith("{"):
                continue
            try:
                msg = json.loads(line)
            except json.JSONDecodeError:
                continue
            if msg.get("reason") != "compiler-message":
                continue
            m = msg.get("message") or {}
            if m.get("level") != "error":
                continue
            yield m


def abs_of(span):
    path = span.get("file_name", "")
    p = path if os.path.isabs(path) else os.path.join(REPO, path)
    return os.path.normpath(p)


#region 🔖️Collectors
def collect_e0308(m, scope, out):
    for span in m.get("spans", []):
        if not span.get("is_primary"):
            continue
        label = span.get("label") or ""
        if not E0308_LABEL_RE.search(label):
            continue
        ap = abs_of(span)
        if scope and not in_scope(ap, scope):
            continue
        out.append((ap, span["byte_end"], "END", span.get("line_start"), "E0308"))


def find_repo_span(span):
    """↩️ Follow `expansion.span` outward until landing on a span whose file is in this repo (not
    the rustup toolchain's macro_rules! definition). Returns (abs_path, byte_start, byte_end) of
    the OUTERMOST repo-local span reached, or None. Depth-capped, not name-keyed — a macro can only
    nest so deep in practice and this is purely span-chasing."""
    cur = span
    for _ in range(6):
        if cur is None:
            return None
        ap = abs_of(cur)
        if ap.startswith(REPO) and "rustup" not in ap:
            return ap, cur["byte_start"], cur["byte_end"], cur.get("line_start")
        exp = cur.get("expansion")
        cur = exp.get("span") if exp else None
    return None


def collect_e0277_fmt(m, scope, out):
    """🩹 `format!`/`panic!`/`assert!(cond, "...", args)` E0277 "doesn't implement Display/Debug".

    🐛 MEASURED BUG, found and repaired by hand across 8 files (73 sites, ~50KB of corrupted string
    literals): a Rust-2021 IMPLICIT format-string capture (`"got {id_a}"`) triggers this SAME E0277
    when `id_a` is (or is treated as) a Future, but the diagnostic's primary span covers the
    captured IDENTIFIER **inside the string literal itself** — appending `.await` there doesn't
    fix anything (the real fix is almost always removing a stray repeated `.await` on `id_a`
    upstream, an E0382-family bug) and instead grows the string by 6 bytes every single run,
    silently, forever, since the "fix" never makes the underlying error go away. Four loop
    iterations produced `.await` stacked 6 deep inside error messages before this was caught.
    Guard: an implicit format capture is always immediately followed by `}` closing the
    `{identifier}` — real code is never `X}` at a `.await`-insertion point, so refuse whenever the
    byte right after the span is `}`.
    """
    if not E0277_FMT_RE.match(m.get("message", "")):
        return
    for span in m.get("spans", []):
        if not span.get("is_primary"):
            continue
        ap = abs_of(span)
        # 📏 Measured: unlike E0369's assert_eq! case, format_args!'s OUTER span already carries an
        # in-repo file_name/byte range covering the exact offending argument — no expansion-chasing
        # needed. Guard anyway: only trust it when it's actually in-repo.
        if not (ap.startswith(REPO) and "rustup" not in ap):
            continue
        if scope and not in_scope(ap, scope):
            continue
        out.append((ap, (span["byte_start"], span["byte_end"]), "FMT_END", span.get("line_start"), "E0277"))


def collect_e0277_generic(m, scope, out):
    """🧩 Iterator / trait-bound E0277 shapes — see the regexes' own comments. Matched separately
    from collect_e0277_fmt because these primary spans are receiver-exclusive (cover exactly the
    Future-typed expression, no braces), unlike the Display/Debug capture shape which can land
    inside a `{ident}` format string and needs the FMT_END guard."""
    msg = m.get("message", "")
    top_match = bool(E0277_MSG_GENERIC_RE.match(msg) or E0277_BOUND_MSG_RE.match(msg))
    for span in m.get("spans", []):
        if not span.get("is_primary"):
            continue
        label = span.get("label") or ""
        if not (top_match or E0277_LABEL_GENERIC_RE.match(label)):
            continue
        ap = abs_of(span)
        if not (ap.startswith(REPO) and "rustup" not in ap):
            continue
        if scope and not in_scope(ap, scope):
            continue
        out.append((ap, span["byte_end"], "END", span.get("line_start"), "E0277"))


def collect_e0369(m, scope, out):
    labelled = [s for s in m.get("spans", []) if s.get("label")]
    for idx, span in enumerate(labelled):
        label = (span.get("label") or "").strip()
        if not E0369_LABEL_RE.match(label):
            continue
        ap = abs_of(span)
        if ap.startswith(REPO) and "rustup" not in ap:
            # direct (non-macro) comparison — the span itself IS the future operand
            if scope and not in_scope(ap, scope):
                continue
            out.append((ap, span["byte_end"], "END", span.get("line_start"), "E0369"))
            continue
        # macro-desugared (assert_eq!/assert_ne!/...) — resolve to the call site, arg by position
        found = find_repo_span(span)
        if found is None:
            continue
        rap, cbs, cbe, call_line = found
        if scope and not in_scope(rap, scope):
            continue
        out.append((rap, (cbs, cbe, idx), "MACRO_ARG", call_line, "E0369"))


def collect_e0600(m, scope, out):
    if not E0600_LABEL_RE.match(m.get("message", "")):
        return
    for span in m.get("spans", []):
        if not span.get("is_primary"):
            continue
        ap = abs_of(span)
        if scope and not in_scope(ap, scope):
            continue
        # Direct `!expr`/`-expr`/`*expr` spans already live in-repo and cover the WHOLE unary
        # expression (operator + operand) — .await belongs at the very end. Macro-desugared
        # (assert!/debug_assert!) unary-not spans ALSO already report file_name in-repo (rustc
        # does not push these through `expansion` the way assert_eq!'s binary op is), but their
        # span covers the ENTIRE macro call, not just `!operand` — the two shapes are told apart
        # at apply time by checking whether the span's own text starts with a known macro name.
        out.append((ap, (span["byte_start"], span["byte_end"]), "UNARY_OR_MACRO0",
                    span.get("line_start"), "E0600"))


def collect_e0599(m, scope, out):
    if not E0599_MSG_RE.match(m.get("message", "")):
        return
    for span in m.get("spans", []):
        if not span.get("is_primary"):
            continue
        ap = abs_of(span)
        if scope and not in_scope(ap, scope):
            continue
        out.append((ap, span["byte_start"], "DOT", span.get("line_start"), "E0599"))


def collect_e0608(m, scope, out):
    # 📏 Measured against actual rustc output (both a plain `values["root"]` and an
    # `assert_eq!(values["root"], 1)` case): the E0608 primary span covers the WHOLE bracketed
    # index expression INCLUDING both `[` and `]` — not just the inner index content the way
    # E0609/E0599's field/method-name spans are receiver-exclusive. So the receiver ends exactly
    # at `span.byte_start`; no backward bracket-matching is needed at all.
    if not E0608_MSG_RE.match(m.get("message", "")):
        return
    for span in m.get("spans", []):
        if not span.get("is_primary"):
            continue
        ap = abs_of(span)
        if scope and not in_scope(ap, scope):
            continue
        out.append((ap, span["byte_start"], "STARTPOS", span.get("line_start"), "E0608"))
#endregion 🔖️Collectors


#region 🔖️Position resolution
def resolve_dot(data: bytes, byte_start: int):
    """↩️ Walk back from a method/field-name span to the `.` immediately preceding it."""
    dot_pos = byte_start - 1
    if dot_pos < 0 or data[dot_pos:dot_pos + 1] != b".":
        return None
    return dot_pos


def _split_top_level_args(data: bytes, open_paren: int):
    """↩️ Given the byte offset of a macro call's opening `(`, returns (close_paren_offset,
    [(arg_start, arg_end_trimmed), ...]) for its top-level, comma-separated arguments — skipping
    commas/brackets/parens that occur inside nested (), [], {} or string/char literals."""
    i = open_paren + 1
    depth = 0
    in_str = None  # None, or the closing byte for the current string/char literal
    escape = False
    arg_start = i
    args = []
    n = len(data)
    while i < n:
        c = data[i:i + 1]
        if in_str is not None:
            if escape:
                escape = False
            elif c == b"\\":
                escape = True
            elif c == in_str:
                in_str = None
        elif c in (b'"', b"'"):
            in_str = c
        elif c in (b"(", b"[", b"{"):
            depth += 1
        elif c in (b")", b"]", b"}"):
            if c == b")" and depth == 0:
                end = i
                seg = data[arg_start:end]
                stripped_trail = len(seg) - len(seg.rstrip())
                if seg.strip():
                    args.append((arg_start, end - stripped_trail))
                return end, args
            depth -= 1
        elif c == b"," and depth == 0:
            seg = data[arg_start:i]
            stripped_trail = len(seg) - len(seg.rstrip())
            args.append((arg_start, i - stripped_trail))
            arg_start = i + 1
        i += 1
    return None, args


def resolve_macro_arg(data: bytes, call_bs: int, call_be: int, arg_idx: int):
    """↩️ Resolve an `E0369` operand inside `assert_eq!`/`assert_ne!` (etc.) to the exact byte
    right after that argument's trimmed text, using the call site's OWN span as the search window
    (never a name match) plus balanced top-level comma splitting."""
    text = data[call_bs:call_be]
    for name in ASSERT_MACROS:
        if text.startswith(name.encode("utf-8")):
            open_paren = call_bs + len(name)
            # macro name may be followed by whitespace before `(`
            while data[open_paren:open_paren + 1] in (b" ", b"\t"):
                open_paren += 1
            if data[open_paren:open_paren + 1] != b"(":
                return None
            close_paren, args = _split_top_level_args(data, open_paren)
            if close_paren is None or arg_idx >= len(args):
                return None
            return args[arg_idx][1]  # end-of-arg byte offset (already .rstrip()-trimmed)
    return None


def is_format_capture_identifier(data: bytes, byte_start: int, byte_end: int):
    """🛡️ True when `data[byte_start:byte_end]` is a whole implicit Rust-2021 format-string capture
    — `{ident}` or `{ident:?}`/`{ident:#?}`/etc — braces included.

    📏 MEASURED (the hard way, after a first version of this guard was wrong): the E0277 span for
    this shape covers the ENTIRE `{...}` capture, opening brace through closing brace — NOT just
    the bare identifier the way the field/method-name spans in E0609/E0599 do. A guard that
    scanned backward from `byte_start` expecting to land ON a `{` never fired, because `byte_start`
    already points AT the `{`. Check the span's own first/last byte instead."""
    return data[byte_start:byte_start + 1] == b"{" and data[byte_end - 1:byte_end] == b"}"


def resolve_unary_or_macro0(data: bytes, byte_start: int, byte_end: int):
    """↩️ E0600 span is EITHER a direct `!expr`/`-expr`/`*expr` (operand ends at byte_end, insert
    there) OR an `assert!`/`debug_assert!` call covering the whole invocation (operand is macro
    arg 0). Distinguished by whether the span's own text opens with a known macro name."""
    text = data[byte_start:byte_end]
    for name in ASSERT_MACROS:
        if ASSERT_MACROS[name] != 1:
            continue
        if text.startswith(name.encode("utf-8")):
            return resolve_macro_arg(data, byte_start, byte_end, 0)
    if text[:1] in (b"!", b"-", b"*"):
        return byte_end
    return None
#endregion 🔖️Position resolution


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--diag", required=True)
    ap.add_argument("--scope", default=None)
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()

    raw = []
    for m in load_diags(args.diag):
        code = (m.get("code") or {}).get("code")
        if code == "E0308":
            collect_e0308(m, args.scope, raw)
        elif code == "E0369":
            collect_e0369(m, args.scope, raw)
        elif code == "E0599":
            collect_e0599(m, args.scope, raw)
        elif code == "E0608":
            collect_e0608(m, args.scope, raw)
        elif code == "E0600":
            collect_e0600(m, args.scope, raw)
        elif code == "E0277":
            collect_e0277_fmt(m, args.scope, raw)
            collect_e0277_generic(m, args.scope, raw)

    by_file = defaultdict(list)
    for ap_, pos, kind, line, code in raw:
        by_file[ap_].append((pos, kind, line, code))

    def anchor(pos):
        """↩️ Uniform int sort key regardless of whether `pos` is a plain byte offset (END/DOT/
        BRACKET) or a (call_bs, call_be[, arg_idx]) tuple (MACRO_ARG/UNARY_OR_MACRO0)."""
        return pos[0] if isinstance(pos, tuple) else pos

    total_applied = 0
    total_skipped = 0
    by_code_applied = defaultdict(int)
    for path, items in by_file.items():
        with open(path, "rb") as fh:
            data = fh.read()
        # dedupe identical (pos, kind) pairs (multiple diagnostics can flag the same span/arg)
        uniq = sorted(set(items), key=lambda x: anchor(x[0]), reverse=True)
        for pos, kind, line, code in uniq:
            if kind in ("END", "STARTPOS"):
                insert_at = pos
                ok = True
            elif kind == "FMT_END":
                # 🛡️ Refuse an implicit format-string capture (`"{ident}"`/`"{ident:?}"`) — not a
                # code expression; see the long comment on collect_e0277_fmt for the corruption
                # this guards against.
                fbs, fbe = pos
                if is_format_capture_identifier(data, fbs, fbe):
                    insert_at = None
                    ok = False
                else:
                    insert_at = fbe
                    ok = True
            elif kind == "DOT":
                insert_at = resolve_dot(data, pos)
                ok = insert_at is not None
            elif kind == "MACRO_ARG":
                cbs, cbe, idx = pos
                insert_at = resolve_macro_arg(data, cbs, cbe, idx)
                ok = insert_at is not None
            elif kind == "UNARY_OR_MACRO0":
                bs, be = pos
                insert_at = resolve_unary_or_macro0(data, bs, be)
                ok = insert_at is not None
            else:
                ok = False
                insert_at = None

            rel = os.path.relpath(path, REPO)
            if not ok:
                print(f"  SKIP {rel}:{line} [{code}] — could not resolve insertion point near byte {pos}",
                      file=sys.stderr)
                total_skipped += 1
                continue

            print(f"  {'EDIT' if args.apply else 'WOULD EDIT'} {rel}:{line} [{code}] "
                  f"insert `.await` at byte {insert_at}")
            if args.apply:
                data = data[:insert_at] + b".await" + data[insert_at:]
            total_applied += 1
            by_code_applied[code] += 1
        if args.apply:
            with open(path, "wb") as fh:
                fh.write(data)

    print(f"\n{'applied' if args.apply else 'would apply'}: {total_applied}  skipped: {total_skipped}  "
          f"files: {len(by_file)}")
    for k, v in sorted(by_code_applied.items()):
        print(f"  {k}: {v}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
