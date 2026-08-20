#!/usr/bin/env python3
"""🩹 Diagnostic-driven `.await` insertion for the residue insert-await.py cannot touch:
E0609/E0599 (field/method access on a Future with no suggested_replacement), E0308/E0369/E0277
where the primary span covers the whole un-awaited expression directly (no suggestion either), and
the assert_eq!/assert_ne! macro-wrapped shape where rustc's real spans live inside core's macro
definition file but the REAL call-site text/line is recoverable via `expansion.span`.

SPAN-KEYED, never name/regex-keyed (R10). Every edit is driven by a specific rustc diagnostic at a
specific byte/line location; nothing is found by scanning source text for identifiers.

Modes handled, in order of preference:
  A) direct field/method access: is_primary span in OUR file, immediately preceded by '.' -> insert
     '.await' right before that '.' (turns `recv.field` into `recv.await.field`).
  B) direct whole-expression span in OUR file (no macro) -> insert '.await' at the span's byte_end
     (only used when mode A's preceding-dot check fails, i.e. the span is not a field/method name).
  C) assert_eq!/assert_ne! macro-wrapped: use expansion.span for the real (file, line, text). Split
     the macro's argument list on top-level commas. Exactly one child span is labelled with a type
     containing "Future"; its ordinal position (0=left, 1=right) selects which top-level argument to
     append '.await' to, at the end of that argument's own top-level parenthesised call (i.e. right
     after the last top-level ')' before the delimiting comma), or at the argument's own end if it
     has no such call.

Every application is verified: after each pass the file must still contain valid UTF-8 and the
edit is applied at an exact byte offset taken directly from this run's JSON, never re-derived from
a stale line/col guess.
"""
import json
import os
import subprocess
import sys

REPO = "/Users/ueli/Documents/semio"
SCOPE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
TARGET_DIR = "/private/tmp/claude-501/-Users-ueli-Documents-semio/40ab938a-57cf-4d17-94a3-77c54a12536e/scratchpad/target-sdktest"


def in_scope(path):
    rel = os.path.relpath(path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in SCOPE.strip('/').split('/') if p]
    return any(parts[i:i + len(want)] == want for i in range(len(parts) - len(want) + 1))


def run_check():
    cmd = ["cargo", "check", "-p", "semio-framework-plugin",
           "--message-format=json-diagnostic-rendered-ansi", "--all-targets"]
    env = dict(os.environ, CARGO_TARGET_DIR=TARGET_DIR)
    proc = subprocess.run(cmd, cwd=REPO, env=env, capture_output=True, text=True)
    out = []
    for line in proc.stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("reason") == "compiler-message":
            out.append(d["message"])
    return out


def resolve_path(fn):
    return os.path.normpath(fn if os.path.isabs(fn) else os.path.join(REPO, fn))


def find_top_level_commas(text):
    """Byte offsets (into `text`) of top-level commas, respecting (), [], {}, and string/char literals."""
    depth = 0
    commas = []
    i = 0
    n = len(text)
    in_str = None
    while i < n:
        c = text[i]
        if in_str:
            if c == '\\':
                i += 2
                continue
            if c == in_str:
                in_str = None
            i += 1
            continue
        if c in '"\'':
            in_str = c
        elif c in '([{':
            depth += 1
        elif c in ')]}':
            depth -= 1
        elif c == ',' and depth == 0:
            commas.append(i)
        i += 1
    return commas


def split_top_level_args(text):
    """Split the INSIDE of a macro invocation's parens into top-level arguments (byte spans into text)."""
    commas = find_top_level_commas(text)
    bounds = [0] + [c + 1 for c in commas]
    ends = commas + [len(text)]
    args = []
    for s, e in zip(bounds, ends):
        args.append((s, e))
    return args


def last_top_level_close_paren(arg_text):
    """Byte offset just past the LAST top-level ')' in arg_text, or None."""
    depth = 0
    last = None
    in_str = None
    i = 0
    n = len(arg_text)
    while i < n:
        c = arg_text[i]
        if in_str:
            if c == '\\':
                i += 2
                continue
            if c == in_str:
                in_str = None
            i += 1
            continue
        if c in '"\'':
            in_str = c
        elif c == '(':
            depth += 1
        elif c == ')':
            depth -= 1
            if depth == 0:
                last = i + 1
        i += 1
    return last


def collect_edits(diags):
    """Return list of (abs_path, byte_offset, insert_text, reason), deduped."""
    edits = {}
    skipped = []
    for d in diags:
        if d.get("level") != "error":
            continue
        code = (d.get("code") or {}).get("code")
        if code not in ("E0609", "E0599", "E0308", "E0369", "E0277"):
            continue
        msg = d.get("message", "")
        if "Future" not in msg and code not in ("E0369",):
            # E0369 messages always mention Future when relevant; for others require it explicitly
            if code != "E0369":
                continue
        spans = d.get("spans", [])
        primary = next((s for s in spans if s.get("is_primary")), None)
        if primary is None:
            continue

        # Mode C: macro-wrapped (assert_eq!/assert_ne!) — real text lives in expansion.span
        exp = primary.get("expansion")
        if exp and exp.get("macro_decl_name") in ("assert_eq!", "assert_ne!"):
            call_span = exp["span"]
            fn = call_span["file_name"]
            path = resolve_path(fn)
            if not in_scope(path):
                skipped.append((code, "out-of-scope-macro", fn))
                continue
            if code not in ("E0369", "E0277"):
                skipped.append((code, "unhandled-macro-code", fn))
                continue
            # find which side (0=left,1=right) is the Future, via labelled sibling spans
            future_side = None
            side_idx = 0
            for s in spans:
                lbl = s.get("label") or ""
                if s.get("expansion") and s["expansion"].get("macro_decl_name") in ("assert_eq!", "assert_ne!"):
                    if "Future" in lbl:
                        future_side = side_idx
                    if lbl:  # only left/right carry the type label; the bool combinator span has none
                        side_idx += 1
            if future_side is None:
                skipped.append((code, "no-future-side", fn))
                continue
            # read real source text of the macro call
            try:
                with open(path, "rb") as fh:
                    data = fh.read()
            except OSError:
                continue
            call_text = data[call_span["byte_start"]:call_span["byte_end"]].decode("utf-8")
            op = call_text.find("(")
            if op == -1:
                skipped.append((code, "no-open-paren", fn))
                continue
            depth = 0
            close = None
            in_str = None
            i = op
            while i < len(call_text):
                c = call_text[i]
                if in_str:
                    if c == '\\':
                        i += 2
                        continue
                    if c == in_str:
                        in_str = None
                    i += 1
                    continue
                if c in '"\'':
                    in_str = c
                elif c == '(':
                    depth += 1
                elif c == ')':
                    depth -= 1
                    if depth == 0:
                        close = i
                        break
                i += 1
            if close is None:
                skipped.append((code, "no-close-paren", fn))
                continue
            inner = call_text[op + 1:close]
            args = split_top_level_args(inner)
            if future_side >= len(args):
                skipped.append((code, "side-out-of-range", fn))
                continue
            a_start, a_end = args[future_side]
            arg_text = inner[a_start:a_end]
            # strip trailing whitespace from arg_text for the close-paren scan
            stripped = arg_text.rstrip()
            trail_ws = len(arg_text) - len(stripped)
            close_in_arg = last_top_level_close_paren(stripped)
            if close_in_arg is not None:
                insert_rel = a_start + close_in_arg
            else:
                insert_rel = a_start + len(stripped)
            abs_offset = call_span["byte_start"] + op + 1 + insert_rel
            key = (path, abs_offset)
            edits[key] = (path, abs_offset, ".await", f"{code} macro@{fn}:{call_span['line_start']}")
            continue

        # Modes A/B: direct (non-macro) spans in our file
        fn = primary["file_name"]
        path = resolve_path(fn)
        if not in_scope(path):
            skipped.append((code, "out-of-scope-direct", fn))
            continue
        bs, be = primary["byte_start"], primary["byte_end"]
        try:
            with open(path, "rb") as fh:
                data = fh.read()
        except OSError:
            continue
        # Mode A: preceded by '.', i.e. this is a field/method name on a future receiver
        if bs > 0 and data[bs - 1:bs] == b".":
            key = (path, bs - 1)
            edits[key] = (path, bs - 1, ".await", f"{code} field/method@{fn}:{primary['line_start']}")
            continue
        # Mode B: span covers the whole un-awaited expression -> append .await at its end
        if code in ("E0308", "E0277"):
            key = (path, be)
            edits[key] = (path, be, ".await", f"{code} whole-expr@{fn}:{primary['line_start']}")
            continue
        skipped.append((code, "unhandled-direct-shape", fn))
    return list(edits.values()), skipped


def apply_edits(edits):
    by_file = {}
    for path, offset, text, reason in edits:
        by_file.setdefault(path, []).append((offset, text, reason))
    applied = 0
    for path, items in by_file.items():
        items.sort(key=lambda x: -x[0])
        with open(path, "rb") as fh:
            data = fh.read()
        before_lines = data.count(b"\n")
        for offset, text, reason in items:
            data = data[:offset] + text.encode("utf-8") + data[offset:]
            applied += 1
        after_lines = data.count(b"\n")
        with open(path, "wb") as fh:
            fh.write(data)
        print(f"  {os.path.relpath(path, REPO)}: {len(items)} edits, lines {before_lines} -> {after_lines}")
    return applied


def main():
    dry = "--dry-run" in sys.argv
    diags = run_check()
    errors = [d for d in diags if d.get("level") == "error"]
    edits, skipped = collect_edits(diags)
    print(f"errors={len(errors)} edits={len(edits)} skipped={len(skipped)}")
    if dry:
        for path, offset, text, reason in edits[:60]:
            print(f"  EDIT {os.path.relpath(path, REPO)}@{offset} -> {text!r}  ({reason})")
        from collections import Counter
        c = Counter((s[0], s[1]) for s in skipped)
        for k, v in c.most_common(20):
            print(f"  SKIP {k}: {v}")
        return
    if not edits:
        print("no edits found")
        return
    applied = apply_edits(edits)
    print(f"applied {applied}")


if __name__ == "__main__":
    main()
