#!/usr/bin/env python3
"""🩹 Diagnostic-driven repair for residue shape #2 from the ticket brief: "awaiting one future
repeatedly". The universal-async codemod left MANY test bindings shaped `let x = some_async_fn();`
(the call itself never awaited) with every later use written `x.await` — the FIRST such use
compiles (it moves `x`'s future and resolves it), every subsequent one is E0382 "use of moved
value" because a `Future` is not `Copy`.

rustc's E0382 diagnostic for exactly this shape carries three spans we use verbatim — nothing here
is matched by variable name, only by the DIAGNOSTIC'S OWN LABELS and the byte spans attached to
them (R10: span-keyed, not name-keyed; the variable name is incidental grouping key, never the
match criterion):

  * a secondary span labelled `move occurs because \`X\` has type \`impl Future<Output = ...>\`,
    which does not implement the \`Copy\` trait` -> covers exactly the `X` identifier in its `let X
    = EXPR;` binding.
  * a secondary span labelled `\`X\` moved due to this await` -> covers exactly the `await` token
    of the FIRST (legal) `.await` on that binding.
  * a PRIMARY span labelled `value used here after move` -> covers exactly the `X` identifier at
    each ILLEGAL subsequent use (one diagnostic per illegal use-site; several diagnostics share the
    same binding span, which is how they get grouped back into one edit-group).

Fix, applied per group (one `let` binding may be reused by many E0382 diagnostics):
  1. At the binding: insert `.await` right after the RHS expression, before its terminating `;`
     (found by balanced-bracket/string scanning from `=`, never by assuming a fixed offset).
  2. At the first (`moved due to this await`) span: remove the `.` + `await` there (the binding now
     already carries the await).
  3. At every `value used here after move` span: remove the `.await`/`await.` immediately following
     the identifier (the binding is a plain value now, not a future).

Every removal re-verifies the exact bytes it is about to delete against the CURRENT file content
immediately before writing, and skips with a printed reason rather than guessing if they don't
match what rustc told us to expect.

Usage:
    cargo check -p <crate> --all-targets --message-format=json > diag.json
    python3 terra-oskat-collapse-repeated-await.py --diag diag.json --scope <path> [--apply]
"""
import argparse
import json
import re
import os
import sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"

MOVE_LABEL_RE = re.compile(
    r"^move occurs because `[A-Za-z0-9_]+` has type `impl (?:std::future::)?Future<Output ?= ?.*>`, "
    r"which does not implement the `Copy` trait$"
)
FIRST_AWAIT_LABEL_RE = re.compile(r"^`[A-Za-z0-9_]+` moved due to this await$")
REUSE_LABEL = "value used here after move"


def in_scope(abs_path: str, scope: str) -> bool:
    rel = os.path.relpath(abs_path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip("/").split("/") if p]
    for i in range(len(parts) - len(want) + 1):
        if parts[i:i + len(want)] == want:
            return True
    return False


def abs_of(span):
    path = span.get("file_name", "")
    p = path if os.path.isabs(path) else os.path.join(REPO, path)
    return os.path.normpath(p)


def load_e0382(diag_path):
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
            if (m.get("code") or {}).get("code") != "E0382":
                continue
            yield m


#region 🔖️Binding-end resolution
def find_statement_end(data: bytes, after_name: int):
    """↩️ From just after a `let X` identifier, find `=`, then balanced-scan to the top-level `;`
    that ends the statement. Returns the byte offset of that `;` (insert `.await` right before it),
    or None if the shape doesn't match (defensive — never guess)."""
    i = after_name
    n = len(data)
    while i < n and data[i:i + 1] in (b" ", b"\t", b":"):
        # allow a type ascription `let X: T = ...` — skip past it too, stopping at `=`
        if data[i:i + 1] == b":":
            depth = 0
            i += 1
            while i < n:
                c = data[i:i + 1]
                if c in (b"<", b"("):
                    depth += 1
                elif c in (b">", b")"):
                    depth -= 1
                elif c == b"=" and depth <= 0:
                    break
                i += 1
            break
        i += 1
    while i < n and data[i:i + 1] in (b" ", b"\t"):
        i += 1
    if data[i:i + 1] != b"=":
        return None
    i += 1
    depth = 0
    in_str = None
    escape = False
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
            depth -= 1
        elif c == b";" and depth == 0:
            return i
        i += 1
    return None
#endregion 🔖️Binding-end resolution


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--diag", required=True)
    ap.add_argument("--scope", default=None)
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()

    # groups keyed by (abs_path, binding_span_byte_start)
    #
    # 🐛 MEASURED BUG (found via real corruption, repaired by hand, then fixed here): rustc's
    # "moved due to this await" note is NOT always the textually-first `.await` — for a binding
    # reused 3+ times, the diagnostic about the THIRD use blames the SECOND use as "where the move
    # happened" (NLL reports the closest dominating move for THAT specific diagnostic, not the
    # global first one). Naively keeping only the LATEST `first_await` seen per group (over-
    # writing on each diagnostic) meant its target byte range could exactly coincide with one
    # already scheduled for deletion via a `reuse` span from an EARLIER diagnostic in the same
    # group — two DELETE edits at the identical range. Applying both in sequence deletes the same
    # 6 bytes twice: the second deletion, now unknowingly operating on already-shifted text, eats
    # 6 bytes of whatever legitimately followed instead, merging two statements into one garbled
    # line. Fix: collect EVERY `first_await` AND every `reuse` span across all diagnostics in the
    # group, convert each independently to its `.await`-removal byte range, then dedupe by the
    # resulting (start, end) tuple before scheduling any deletes — so the SAME source location
    # reported through two different diagnostic shapes only ever gets removed once.
    groups = {}
    for m in load_e0382(args.diag):
        spans = m.get("spans", [])
        binding = None
        first_awaits_here = []
        reuses_here = []
        for sp in spans:
            label = (sp.get("label") or "").strip()
            if MOVE_LABEL_RE.match(label):
                binding = sp
            elif FIRST_AWAIT_LABEL_RE.match(label):
                first_awaits_here.append(sp)
            elif label == REUSE_LABEL:
                # 📎 A single diagnostic CAN carry more than one "used here after move" span (e.g.
                # two illegal uses of the same moved binding within one statement) — collect every
                # one, never just the first/last, so a multi-use statement isn't silently
                # under-collapsed.
                reuses_here.append(sp)
        if binding is None or not reuses_here:
            continue  # not this shape — leave for hand review
        ap_ = abs_of(binding)
        if args.scope and not in_scope(ap_, args.scope):
            continue
        key = (ap_, binding["byte_start"])
        g = groups.setdefault(key, {
            "binding": binding, "first_awaits": [], "reuses": [], "line": binding.get("line_start"),
        })
        g["first_awaits"].extend(first_awaits_here)
        g["reuses"].extend(reuses_here)

    by_file = defaultdict(list)
    for (ap_, _bs), g in groups.items():
        by_file[ap_].append(g)

    total_bindings = 0
    total_reuses = 0
    total_skipped_bindings = 0
    total_skipped_reuses = 0

    for path, group_list in by_file.items():
        with open(path, "rb") as fh:
            data = fh.read()
        rel = os.path.relpath(path, REPO)

        # Collect ALL edits for this file as (byte_pos, kind, payload) then apply in descending
        # byte order so earlier offsets stay valid — same discipline as the other tools here.
        edits = []  # (pos, kind) where kind in {"INSERT_AWAIT", "DELETE"} ; payload = (pos,end) for DELETE
        for g in group_list:
            binding = g["binding"]
            reuses = g["reuses"]

            stmt_end = find_statement_end(data, binding["byte_end"])
            if stmt_end is None:
                print(f"  SKIP-BINDING {rel}:{g['line']} — could not locate `let` statement end after byte {binding['byte_end']}",
                      file=sys.stderr)
                total_skipped_bindings += 1
                continue

            # 🎯 Unified pool: every `first_await` AND every `reuse` span, each converted to its own
            # `.await`-removal byte range, deduped by the resulting range — see the header comment
            # on `groups` above for why two DIFFERENT diagnostic shapes can report the identical
            # location and why deleting it twice is exactly the corruption this guards against.
            del_ranges = set()
            skipped_here = False
            for fa in g["first_awaits"]:
                fa_bs = fa["byte_start"]
                if data[fa_bs - 1:fa_bs] == b".":
                    del_ranges.add((fa_bs - 1, fa["byte_end"]))
                else:
                    print(f"  SKIP-BINDING {rel}:{g['line']} — first-await span byte {fa_bs - 1} is not `.`",
                          file=sys.stderr)
                    skipped_here = True

            for r in reuses:
                rbe = r["byte_end"]
                if data[rbe:rbe + 6] in (b".await", b"await."):
                    del_ranges.add((rbe, rbe + 6))
                else:
                    print(f"  SKIP-REUSE {rel}:{r.get('line_start')} — no `.await` immediately after byte {rbe} "
                          f"(got {data[rbe:rbe+8]!r})", file=sys.stderr)
                    total_skipped_reuses += 1

            if skipped_here:
                total_skipped_bindings += 1
                continue

            # 🔒 Final safety net: NO two scheduled ranges anywhere in this GROUP may overlap. If
            # they do, something about this group's diagnostics is shaped unexpectedly — refuse
            # rather than risk a repeat of the corruption above.
            ordered = sorted(del_ranges)
            overlap = any(a[1] > b[0] for a, b in zip(ordered, ordered[1:]))
            if overlap:
                print(f"  SKIP-BINDING {rel}:{g['line']} — overlapping delete ranges {ordered}, refusing to guess",
                      file=sys.stderr)
                total_skipped_bindings += 1
                continue

            edits.append((stmt_end, "INSERT_AWAIT", None))
            for d in ordered:
                edits.append((d[0], "DELETE", d))
            total_bindings += 1
            total_reuses += len(ordered)
            kind = "EDIT" if args.apply else "WOULD EDIT"
            print(f"  {kind} {rel}:{g['line']} collapse repeated `.await` — 1 binding + {len(ordered)} await-site(s) removed")

        if args.apply and edits:
            # 🔒 File-level cross-group safety net, same reasoning as the per-group check above but
            # covering the (unexpected) case of two DIFFERENT bindings' delete ranges colliding.
            delete_ranges = sorted(payload for _, kind, payload in edits if kind == "DELETE")
            cross_overlap = any(a[1] > b[0] for a, b in zip(delete_ranges, delete_ranges[1:]))
            if cross_overlap:
                print(f"  ABORT {rel} — cross-group overlapping delete ranges detected, file left untouched",
                      file=sys.stderr)
                continue
            edits.sort(key=lambda e: e[0], reverse=True)
            for pos, kind, payload in edits:
                if kind == "INSERT_AWAIT":
                    data = data[:pos] + b".await" + data[pos:]
                elif kind == "DELETE":
                    dbs, dbe = payload
                    data = data[:dbs] + data[dbe:]
            with open(path, "wb") as fh:
                fh.write(data)

    print(f"\nbindings: {total_bindings} (skipped {total_skipped_bindings})  "
          f"reuse-sites collapsed: {total_reuses} (skipped {total_skipped_reuses})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
