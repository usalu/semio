#!/usr/bin/env python3
"""🩹 Safer sibling of `fix-repeated-await.py` for a residue shape THAT tool corrupts.

WHY THIS EXISTS (terra-sdk-tests packet, 2026-08-20)
-----------------------------------------------------
`fix-repeated-await.py` was built against `db-dedyn`'s residue shape: an un-awaited constructor
(`ByteWriter::new()`) whose downstream methods are SYNC, so every `IDENT.await.METHOD(...)` site
gets rewritten to `IDENT.METHOD(...).await` (moving the await from the receiver to the call result).

`semio-framework-plugin`'s test module has a DIFFERENT shape: the un-awaited constructor
(`VcsArtifactApp::new(...)`/`new_app::<A>()`/etc.) has downstream methods that are THEMSELVES async
trait methods (`dispatch_typed`, `snapshot`, ...) — many call sites already correctly carry their OWN
trailing `.await` (`app.await.dispatch_typed(...).await.expect(...)`). Running the original tool here
would blindly insert a SECOND `.await` after every `IDENT.await.METHOD(...)` call, producing
`app.dispatch_typed(...).await.await.expect(...)` — an eighth `insert-await.py`-family corruption
class, not yet named in R16/R20 because it lives in THIS tool, not that one.

WHAT IT DOES (deliberately narrower than fix-repeated-await.py)
-----------------------------------------------------------------
Per function body (brace-matched, identical FN_RE/DECL_RE approach as fix-repeated-await.py):
  1. For each `let (mut)? IDENT = EXPR;` whose EXPR does not already end `.await`, if `IDENT.await.`
     appears later in the same body:
       a. Appends `.await` to the declaration's EXPR (IDENT is now the resolved value, not a Future).
       b. Rewrites every `IDENT.await.METHOD` occurrence to `IDENT.METHOD` — strips the now-redundant
          `.await` on the receiver, WITHOUT touching whatever the call itself does or does not await.
Deliberately NOT done: inserting a NEW `.await` after the call's closing paren. Whether METHOD's own
return value needs `.await` is a single-call-site, compiler-provable question — left to
`insert-await.py`'s normal diagnostic-driven pass, which is safe to run again immediately after this
tool because every site it can now see is a clean single-candidate diagnosis.

SAFETY
------
* Same structural (brace/paren-matched) scoping as `fix-repeated-await.py` — R10 concerns itself with
  collisions against unrelated identifiers; requiring the SAME identifier's OWN un-awaited declaration
  to exist in the SAME function body before touching anything makes this unambiguous, not name-keyed.
* `--dry-run` prints every planned edit without writing. `--apply` writes.
* Operates on a single file at a time (pass `--file`), not a directory sweep — keeps blast radius
  reviewable given this file's history of whole-file corruption incidents (see
  `📓️terra-test-attr-restore-report.md`).
"""
import argparse, re, sys

DECL_RE = re.compile(r'\blet\s+(mut\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*')
FN_RE = re.compile(r'\b(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+[a-zA-Z0-9_]+')


def find_fn_bodies(src):
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


def process(src):
    edits = []
    report = []
    for body_start, body_end in find_fn_bodies(src):
        body = src[body_start:body_end]
        for dm in DECL_RE.finditer(body):
            ident = dm.group(2)
            if ident in ('_', 'Self'):
                continue
            decl_expr_start = dm.end()
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
                continue
            if decl_expr.strip() == '':
                continue
            use_pat = re.compile(r'\b' + re.escape(ident) + r'\.await\.')
            uses = list(use_pat.finditer(body, j))
            if not uses:
                continue
            abs_decl_end = body_start + j
            line_no = src.count('\n', 0, abs_decl_end) + 1
            edits.append((abs_decl_end, abs_decl_end, '.await'))
            report.append((line_no, ident, 'decl+.await'))
            for um in uses:
                use_start = body_start + um.start()
                ident_end = use_start + len(ident)
                dot_await_dot_end = body_start + um.end()
                remove_start = ident_end
                remove_end = dot_await_dot_end - 1
                edits.append((remove_start, remove_end, ''))
                use_line = src.count('\n', 0, use_start) + 1
                report.append((use_line, ident, 'strip .await prefix'))
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
    return new_src, report


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--file', required=True)
    ap.add_argument('--apply', action='store_true')
    ap.add_argument('--dry-run', action='store_true')
    args = ap.parse_args()
    with open(args.file, encoding='utf-8') as f:
        src = f.read()
    new_src, report = process(src)
    report.sort()
    for line_no, ident, kind in report:
        print(f"{args.file}:{line_no}: {ident} {kind}")
    print(f"total edit-sites: {len(report)}")
    if args.apply and report:
        with open(args.file, 'w', encoding='utf-8') as f:
            f.write(new_src)
        print("APPLIED")


if __name__ == '__main__':
    main()
