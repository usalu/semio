#!/usr/bin/env python3
"""🪡 Compiler-driven `.await` insertion — the shared tool for the universal-async conversion.

WHY THIS EXISTS
---------------
The universal-async codemods turned ~57,000 `fn` into `async fn`. Every call site of every
converted function now needs `.await`. There are tens of thousands of them. Guessing call sites
from source text is hopeless (method chains, macros, generics), but rustc already knows the exact
answer and prints it as a structured suggestion. This tool applies *rustc's own suggestions* and
nothing else.

THE SAFETY PROPERTY THAT MATTERS
--------------------------------
rustc frequently emits MORE THAN ONE candidate `.await` position for a single error. Real example
from `semio-framework-async`:

    let _ = self.0.local.compare_exchange(CancelState::Park.to_u8(), CancelState::Live.to_u8(), ..)
    help: consider `await`ing on the `Future`   ->  ...Park.to_u8().await, ...
    help: consider `await`ing on the `Future`   ->  ...Live.to_u8().await, ...

**Both of those are correct and must be applied together** - they are two independent futures in one
expression, not two ways to fix one thing. That was measured, not assumed: `geometry-residue` found
9 of its 10 "ambiguous" diagnostics were exactly this shape (`lerp_point(p0, p1, t)` with both
arguments un-awaited), and hand-applied every candidate.

So the discriminator is **span overlap**, not candidate count:
  * candidates at DISJOINT spans  -> independent futures -> apply them all
  * candidates at OVERLAPPING spans -> genuinely alternative rewrites of the same text -> refuse,
    record as ambiguous, leave for a human

That is still the whole difference between this and `cargo fix`, which would happily apply
machine-applicable suggestions it should not - but it no longer punts on the common case.

ORDER MATTERS: ASYNCIFY BEFORE YOU AWAIT
----------------------------------------
If the enclosing functions are still sync, inserting `.await` produces E0728 instead of progress.
Measured on `📡️spr`: one pass applied 197 edits, E0599 fell by exactly 197 and E0728 rose by exactly
197 — a perfectly conserved non-improvement. The module had never been asyncified at all (0 `async fn`,
622 plain, in the index too). Correct sequence for any un-converted or reverted scope:

    asyncify-universal.py --apply <paths>     # signatures first
    insert-await.py --apply --scope <path>    # then call sites, to fixpoint

The tool now ABORTS on any E0728 rather than grinding, and tells you to run the codemod first.

DISCIPLINE THIS ENCODES (learned the hard way on this ticket)
-------------------------------------------------------------
* Span-keyed, never name-keyed. A pass keyed on an identifier hits production code that happens to
  share the name; a pass keyed on a byte span cannot.
* Edits are applied per file sorted by DESCENDING byte offset, so earlier offsets stay valid.
* One edit per (file, span) per run, tracked in a guard set — a span never gets `.await.await`.
* Byte offsets, not line/column: rustc's JSON gives `byte_start`/`byte_end` and the repo is full of
  multi-byte emoji in both paths and source.
* Fixpoint: each pass reveals the next layer of nesting. Converges in ~5-15 passes per crate.

USAGE
-----
    python3 insert-await.py --crate semio-framework-os-kernel-db --dry-run
    python3 insert-await.py --crate semio-framework-os-kernel-db --apply --max-passes 15
    python3 insert-await.py --crate X --apply --scope '🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db'

`--scope` restricts edits to paths UNDER that repo-relative path, matched on path segments
(never as a substring - see in_scope()). Use it to honour packet ownership. `--max-files` aborts a
pass whose blast radius exceeds the cap, which is what an over-broad scope looks like in practice.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"
SCRATCH = (
    "/private/tmp/claude-501/-Users-ueli-Documents-semio/"
    "e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad"
)

# Diagnostics whose "consider awaiting" suggestion we trust. Everything else is reported, not applied.
AWAIT_CODES = {"E0308", "E0599", "E0277", "E0369", "E0605", "E0614", "E0609"}


def run_check(crate: str, target_dir: str, all_targets: bool) -> list[dict]:
    """🩺 Run cargo check and return the parsed JSON diagnostics."""
    cmd = [
        "cargo", "check", "-p", crate,
        "--message-format=json-diagnostic-rendered-ansi",
        "--all-targets" if all_targets else "--lib",
    ]
    env = dict(os.environ, CARGO_TARGET_DIR=target_dir)
    proc = subprocess.run(cmd, cwd=REPO, env=env, capture_output=True, text=True)
    out = []
    for line in proc.stdout.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            msg = json.loads(line)
        except json.JSONDecodeError:
            continue
        if msg.get("reason") == "compiler-message" and msg.get("message"):
            out.append(msg["message"])
    return out


def in_scope(abs_path: str, scope: str) -> bool:
    """🎯 True when `abs_path` lies under `scope`, matched on PATH SEGMENTS, never as a substring.

    WHY THIS IS NOT `scope in path` — a real incident, 2026-08-19
    ------------------------------------------------------------
    A packet ran `--scope '🧰️framework'` believing it confined edits to its own module. As a bare
    substring that matches EVERY file in the framework tree, so one fixpoint pass reached into 314
    files across sibling packets' territory and had to be unwound by hand. **A scope argument that
    silently means "almost the whole repo" is worse than no scope at all, because it is trusted.**

    Now the scope must match a contiguous run of path segments: `🧮️math` matches
    `🧰️framework/🔨️modules/🧮️math/...` through its own segment, and never matches a file merely
    because some ancestor directory name contains that text.
    """
    rel = os.path.relpath(abs_path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip("/").split("/") if p]
    if not want:
        return True
    for i in range(len(parts) - len(want) + 1):
        if parts[i:i + len(want)] == want:
            return True
    return False


def collect_await_edits(diags: list[dict], scope: str | None):
    """🔎 Walk diagnostics and extract unambiguous `.await` insertions.

    Returns (edits, ambiguous, other) where an edit is
    (abs_path, byte_start, byte_end, replacement_text, why).
    """
    edits, ambiguous, other = [], [], []

    def walk(diag, root_code):
        candidates = []
        for child in diag.get("children", []):
            text = (child.get("message") or "").lower()
            is_await_hint = "await" in text
            for span in child.get("spans", []):
                repl = span.get("suggested_replacement")
                if repl is None:
                    continue
                # The replacement must ITSELF add an `.await`. Trusting `is_await_hint` alone was a
                # real defect: a diagnostic whose child message merely mentions "await" can carry a
                # suggestion that does something else entirely (remove an await, annotate a type),
                # and applying it lands text on the wrong token. `kernel-ripple` had to write repair
                # scripts for exactly that across ~10 files. The message is a hint; the replacement
                # is the contract.
                if ".await" not in repl:
                    continue
                if not is_await_hint:
                    continue
                path = span.get("file_name", "")
                abs_path = path if os.path.isabs(path) else os.path.join(REPO, path)
                abs_path = os.path.normpath(abs_path)
                candidates.append(
                    (abs_path, span["byte_start"], span["byte_end"], repl,
                     span.get("suggestion_applicability"))
                )
            walk(child, root_code)

        if not candidates:
            return
        # Deduplicate identical suggestions; ambiguity means genuinely different edits.
        uniq = {(c[0], c[1], c[2], c[3]) for c in candidates}
        primary = ""
        for span in diag.get("spans", []):
            if span.get("is_primary"):
                primary = f'{span.get("file_name")}:{span.get("line_start")}'
                break
        # Multiple candidates are NOT automatically an either/or fork.
        # Measured by `geometry-residue`: 9 of its 10 "ambiguous" diagnostics were rustc offering N
        # candidates because N SIBLING ARGUMENTS in one call were each an un-awaited future —
        # `lerp_point(p0, p1, t)` with both p0 and p1 needing `.await`. Every candidate was correct
        # and they had to be applied together. The same is true of the `compare_exchange(a, b)`
        # example in this file's header: both arguments were futures.
        #
        # The real discriminator is SPAN OVERLAP, not candidate count:
        #   * disjoint spans  -> independent futures in one expression -> apply them ALL
        #   * overlapping spans -> genuinely alternative rewrites of the same text -> refuse
        ordered = sorted(uniq, key=lambda c: (c[0], c[1], c[2]))
        overlapping = any(
            a[0] == b[0] and a[2] > b[1]
            for a, b in zip(ordered, ordered[1:])
        )
        if overlapping:
            ambiguous.append((primary, root_code, sorted(r for _, _, _, r in uniq)))
            return
        for path, bs, be, repl in ordered:
            if scope and not in_scope(path, scope):
                other.append((primary, root_code, "out-of-scope"))
                continue
            edits.append((path, bs, be, repl, f"{root_code} @ {primary}"))

    for diag in diags:
        code = (diag.get("code") or {}).get("code") or ""
        if diag.get("level") != "error":
            continue
        if code in AWAIT_CODES:
            walk(diag, code)
        else:
            primary = ""
            for span in diag.get("spans", []):
                if span.get("is_primary"):
                    primary = f'{span.get("file_name")}:{span.get("line_start")}'
                    break
            other.append((primary, code or "(no code)", diag.get("message", "")[:160]))
    return edits, ambiguous, other


def apply_edits(edits, guard: set) -> int:
    """✍️ Apply edits per file, descending byte offset so earlier offsets stay valid."""
    by_file = defaultdict(list)
    for path, bs, be, repl, why in edits:
        key = (path, bs, be)
        if key in guard:
            continue
        guard.add(key)
        by_file[path].append((bs, be, repl, why))

    applied = 0
    for path, items in by_file.items():
        try:
            with open(path, "rb") as fh:
                data = fh.read()
        except OSError as exc:
            print(f"  !! cannot read {path}: {exc}", file=sys.stderr)
            continue
        # Descending start offset; drop any edit overlapping one already staged.
        items.sort(key=lambda t: t[0], reverse=True)
        taken, last_start = [], None
        for bs, be, repl, why in items:
            if last_start is not None and be > last_start:
                continue  # overlapping edit — leave for the next pass
            taken.append((bs, be, repl, why))
            last_start = bs
        for bs, be, repl, _why in taken:
            data = data[:bs] + repl.encode("utf-8") + data[be:]
        with open(path, "wb") as fh:
            fh.write(data)
        applied += len(taken)
    return applied


def main() -> int:
    ap = argparse.ArgumentParser(description="Compiler-driven .await insertion")
    ap.add_argument("--crate", required=True)
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--max-passes", type=int, default=12)
    ap.add_argument("--scope", default=None,
                    help="only edit paths UNDER this repo-relative path, matched on PATH SEGMENTS "
                         "(not as a substring) - honour packet ownership")
    ap.add_argument("--max-files", type=int, default=60,
                    help="blast-radius guard: abort a pass that would edit more files than this; "
                         "0 disables")
    ap.add_argument("--all-targets", action="store_true")
    ap.add_argument("--target-dir", default=None)
    ap.add_argument("--report", default=None)
    args = ap.parse_args()

    if not args.apply and not args.dry_run:
        ap.error("choose --apply or --dry-run")

    target_dir = args.target_dir or os.path.join(SCRATCH, f"target-await-{args.crate}")
    guard: set = set()
    history = []

    for npass in range(1, args.max_passes + 1):
        diags = run_check(args.crate, target_dir, args.all_targets)
        errors = [d for d in diags if d.get("level") == "error"]
        edits, ambiguous, other = collect_await_edits(diags, args.scope)
        print(f"[pass {npass}] errors={len(errors)} await-edits={len(edits)} "
              f"ambiguous={len(ambiguous)} other={len(other)}")

        # ⛔ Ordering guard. E0728 = "`await` is only allowed inside `async` functions".
        # Its presence means the ENCLOSING fns are still sync, so inserting `.await` converts one
        # error into another instead of fixing anything. Measured the hard way on `📡️spr`: a pass
        # applied 197 edits, E0599 fell by exactly 197 and E0728 rose by exactly 197 — net zero.
        # The tool cannot fix this: the correct repair is to asyncify the enclosing functions first.
        # Count ONLY in-scope E0728. An out-of-scope one belongs to another packet and must not
        # abort your run — that made the guard a denial-of-service on the scoped workflow it exists
        # to protect (reported by `kernel-ripple`, which had to write its own loop to get around it).
        def _primary_path(d):
            for sp in d.get("spans", []):
                if sp.get("is_primary"):
                    fn = sp.get("file_name", "")
                    return os.path.normpath(fn if os.path.isabs(fn) else os.path.join(REPO, fn))
            return None

        e0728 = 0
        for d in errors:
            if (d.get("code") or {}).get("code") != "E0728":
                continue
            pp = _primary_path(d)
            if args.scope and pp and not in_scope(pp, args.scope):
                continue  # another packet's problem, not ours
            e0728 += 1
        if e0728:
            print(f"  ⛔ ABORT: {e0728} × E0728 (`await` outside an async fn).")
            print("  The enclosing functions are still SYNC, so inserting `.await` only trades one")
            print("  error for another. Run the asyncify codemod over this scope FIRST, then re-run:")
            print(f"    python3 asyncify-universal.py --apply <paths under {args.scope or 'the crate'}>")
            break
        history.append({
            "pass": npass, "errors": len(errors), "edits": len(edits),
            "ambiguous": len(ambiguous), "other": len(other),
        })

        if args.dry_run:
            for path, bs, be, repl, why in edits[:40]:
                rel = os.path.relpath(path, REPO)
                print(f"    EDIT {rel} [{bs}:{be}] -> {repl!r}   ({why})")
            for primary, code, opts in ambiguous[:20]:
                print(f"    AMBIGUOUS {code} {primary}: {len(opts)} candidates")
            break

        if not edits:
            print("  fixpoint reached (no unambiguous .await edits left)")
            break
        touched = {e[0] for e in edits}
        if args.max_files and len(touched) > args.max_files:
            print(f"  ABORT: this pass would edit {len(touched)} files, over "
                  f"--max-files={args.max_files}. Refusing.")
            print("  A pass this wide almost always means --scope is broader than you think.")
            for f in sorted(touched)[:25]:
                print(f"    would touch {os.path.relpath(f, REPO)}")
            break
        applied = apply_edits(edits, guard)
        print(f"  applied {applied}")
        if applied == 0:
            print("  no progress — stopping")
            break

    if args.report:
        diags = run_check(args.crate, target_dir, args.all_targets)
        edits, ambiguous, other = collect_await_edits(diags, args.scope)
        with open(args.report, "w", encoding="utf-8") as fh:
            json.dump({
                "crate": args.crate, "history": history,
                "residual_errors": len([d for d in diags if d.get("level") == "error"]),
                "ambiguous": [{"at": a, "code": c, "candidates": o} for a, c, o in ambiguous],
                "other": [{"at": a, "code": c, "msg": m} for a, c, m in other],
            }, fh, indent=2, ensure_ascii=False)
        print(f"report -> {args.report}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
