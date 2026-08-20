#!/usr/bin/env python3
"""🩹 Fixes the "repeated `.await` on the same local variable" residue left by insert-await.py.

WHY THIS EXISTS (db-dedyn packet, 2026-08-20)
----------------------------------------------
`pack::ByteWriter::new()` / `ByteReader::new(...)` (an out-of-scope crate this packet cannot edit)
were asyncified by the blind universal-async codemod, so `let mut writer = ByteWriter::new();` binds
an un-awaited `impl Future<Output = ByteWriter>` to `writer`. Every downstream call site
(`writer.write_u64_le(x)`) then failed to typecheck, and rustc's own per-site suggestion is always
"insert `.await` right after the receiver" — i.e. `writer.await.write_u64_le(x)`. Applied
independently at N call sites by insert-await.py's span-keyed passes, this produces N separate
`writer.await` expressions on the SAME local variable. `.await` MOVES its receiver, so only the
FIRST is legal; the rest are `E0382: use of moved value`. This is the same defect class R10 already
names ("awaiting one future repeatedly") — just spread across sequential statements instead of a
loop, so `insert-await.py` (which only adds `.await`, never restructures) cannot fix it and
`remove-bad-await.py` (which only handles E0277 "not a future") doesn't apply either: every
individual `.await` here IS syntactically legal in isolation, only their combination isn't.

WHAT IT DOES
------------
Per function body (brace-matched), for each `let (mut)? IDENT = EXPR;` whose EXPR does NOT already
end in `.await`, if `IDENT.await.` appears anywhere later in that same function body:
  1. Appends `.await` to the declaration's EXPR.
  2. Rewrites every `IDENT.await.METHOD(...)` occurrence in that function body (paren-matched) to
     `IDENT.METHOD(...).await`.

Untouched: any `IDENT.await` occurrence in a function where `IDENT` has no local `let` declaration
in that same function (e.g. a param, or `self.field.await` — different shape, not this bug), and any
declaration whose EXPR already ends `.await` (already correct).

SAFETY
------
* Operates only within `--scope` (path-segment match, same discipline as insert-await.py).
* Structural (brace/paren matched), not a blind name/regex sweep across the whole file — R10 concerns
  itself with collisions against unrelated identifiers; scoping every rewrite to the one function
  where the SAME identifier's OWN un-awaited declaration lives (and requiring it feed at least one
  `IDENT.await.` use before touching anything) makes the shape unambiguous, not name-keyed guessing.
* `--dry-run` prints every planned edit without writing. `--apply` writes.
"""
import argparse, os, re, sys

REPO = "/Users/ueli/Documents/semio"
DECL_RE = re.compile(r'\blet\s+(mut\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*')
FN_RE = re.compile(r'\b(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+[a-zA-Z0-9_]+')


def in_scope(path, scope):
    rel = os.path.relpath(path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip('/').split('/') if p]
    if not want:
        return True
    return any(parts[i:i + len(want)] == want for i in range(len(parts) - len(want) + 1))


def iter_rs(root):
    for dp, dn, fn in os.walk(root):
        if '/target' in dp or '/🤖️generated' in dp:
            continue
        for f in fn:
            if f.endswith('.rs'):
                yield os.path.join(dp, f)


def find_fn_bodies(src):
    """Yield (body_start, body_end) byte spans for every fn's `{ ... }` block."""
    spans = []
    for m in FN_RE.finditer(src):
        brace = src.find('{', m.end())
        if brace == -1:
            continue
        depth = 0
        i = brace
        while i < len(src):
            if src[i] == '{':
                depth += 1
            elif src[i] == '}':
                depth -= 1
                if depth == 0:
                    spans.append((brace, i + 1))
                    break
            i += 1
    return spans


def match_call_end(src, open_paren_idx):
    """Given index of '(' return index just past the matching ')'."""
    depth = 0
    i = open_paren_idx
    while i < len(src):
        if src[i] == '(':
            depth += 1
        elif src[i] == ')':
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return -1


def process_file(path, apply_):
    with open(path, encoding='utf-8') as f:
        src = f.read()
    edits = []  # (start, end, replacement) applied via descending offset
    for body_start, body_end in find_fn_bodies(src):
        body = src[body_start:body_end]
        # find local declarations in this fn body
        for dm in DECL_RE.finditer(body):
            ident = dm.group(2)
            if ident in ('_', 'Self'):
                continue
            decl_expr_start = dm.end()
            # find end of this let statement (top-level ';' respecting parens/braces)
            depth = 0
            j = decl_expr_start
            while j < len(body):
                c = body[j]
                if c in '([{':
                    depth += 1
                elif c in ')]}':
                    depth -= 1
                elif c == ';' and depth == 0:
                    break
                j += 1
            if j >= len(body):
                continue
            decl_expr = body[decl_expr_start:j]
            if decl_expr.rstrip().endswith('.await'):
                continue  # already correct
            use_pat = re.compile(r'\b' + re.escape(ident) + r'\.await\.')
            uses = list(use_pat.finditer(body, j))
            if not uses:
                continue
            # 1. append .await to declaration expr (absolute offsets)
            abs_decl_end = body_start + j
            edits.append((abs_decl_end, abs_decl_end, '.await'))
            # 2. rewrite each IDENT.await.METHOD(...) -> IDENT.METHOD(...).await
            for um in uses:
                use_start = body_start + um.start()
                dot_await_dot_end = body_start + um.end()  # just past "IDENT.await."
                # method name starts right after
                mname_m = re.match(r'[a-zA-Z_][a-zA-Z0-9_]*', src[dot_await_dot_end:])
                if not mname_m:
                    continue
                mname_end = dot_await_dot_end + mname_m.end()
                if mname_end >= len(src) or src[mname_end] != '(':
                    continue
                call_end = match_call_end(src, mname_end)
                if call_end == -1:
                    continue
                ident_end = use_start + len(ident)
                # remove ".await" between ident and method name: [ident_end, dot_await_dot_end-1)
                # dot_await_dot_end points past the trailing '.', so the segment to delete is
                # ".await." minus the final '.', i.e. keep one '.' before method name.
                remove_start = ident_end
                remove_end = dot_await_dot_end - 1  # keep the last '.' before method name
                edits.append((remove_start, remove_end, ''))
                edits.append((call_end, call_end, '.await'))
    if not edits:
        return 0
    # de-dup and sort descending by start offset; guard against overlapping duplicate inserts at same point
    seen = set()
    dedup = []
    for s, e, r in edits:
        key = (s, e, r)
        if key in seen:
            continue
        seen.add(key)
        dedup.append((s, e, r))
    dedup.sort(key=lambda x: -x[0])
    new_src = src
    for s, e, r in dedup:
        new_src = new_src[:s] + r + new_src[e:]
    if apply_:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(new_src)
    return len(dedup)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--scope', required=True)
    ap.add_argument('--apply', action='store_true')
    ap.add_argument('--dry-run', action='store_true')
    args = ap.parse_args()
    root = os.path.join(REPO, args.scope)
    total = 0
    files = 0
    for path in iter_rs(root):
        if not in_scope(path, args.scope):
            continue
        n = process_file(path, args.apply)
        if n:
            files += 1
            total += n
            print(f"{'APPLIED' if args.apply else 'WOULD-EDIT'} {n} edits in {os.path.relpath(path, REPO)}")
    print(f"total: {total} edits across {files} files")


if __name__ == '__main__':
    main()
