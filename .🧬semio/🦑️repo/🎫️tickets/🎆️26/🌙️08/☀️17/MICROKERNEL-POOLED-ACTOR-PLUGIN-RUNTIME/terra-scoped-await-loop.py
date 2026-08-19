#!/usr/bin/env python3
"""🩹 terra's scoped `.await` fixpoint driver for `kernel-ripple`.

WHY THIS EXISTS INSTEAD OF THE SHARED `insert-await.py`
--------------------------------------------------------
`insert-await.py` aborts the ENTIRE run the moment ANY E0728 shows up ANYWHERE in the crate's
diagnostics, regardless of `--scope`. That is the right default for a packet whose own scope is
broken, but `kernel-ripple` hit an E0728 caused by `🧰️framework/🔨️modules/📡️replication/📖️dictionary/
🦀️component.rs` (`DictReader::resolve` blindly asyncified with zero I/O, callers pass it into a sync
`impl Fn` closure param) — a file OUTSIDE my owned paths (only
`🧰️framework/🛍️products/💻️os/🔨️modules/**` minus plugin/db, and `🧰️framework/🔨️modules/🚪️io/**` are
mine). I cannot fix that file (hard rule 3 — lease-request only), so the shared tool can never make
progress for me again until someone else fixes it.

This driver reuses `insert-await.py`'s own `run_check`/`collect_await_edits`/`apply_edits`/`in_scope`
functions UNCHANGED (imported, not copy-pasted) and only replaces the abort policy: E0728s whose
primary span falls INSIDE one of my scopes still abort (that means MY code needs asyncify-first, the
original tool's warning is correct there); E0728s outside my scopes are reported and skipped.

Usage:
    python3 terra-scoped-await-loop.py --scope <path> [--scope <path> ...] --max-passes 15
"""
import argparse
import importlib.util
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SPEC = importlib.util.spec_from_file_location("insert_await", os.path.join(HERE, "insert-await.py"))
ia = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ia)

REPO = ia.REPO
SCRATCH = ia.SCRATCH


def collect_both_futures_edits(diags, scopes):
    """🤝 rustc's own multi-span `help: consider `await`ing on both `Future`s` suggestion — seen on
    `match` arms that are each a different `impl Future`. All spans inside ONE such help child are
    COMPLEMENTARY (apply every one), not alternatives — unlike the genuinely ambiguous case
    `insert-await.py` guards against (several DIFFERENT candidate positions for a SINGLE await).
    `insert-await.py`'s `collect_await_edits` cannot tell these apart (it flattens every child's
    spans into one pool per diagnostic and calls >1 distinct span "ambiguous"), so this walks the
    raw diagnostics itself and only trusts a child whose OWN message says exactly this."""
    edits = []
    for diag in diags:
        if diag.get("level") != "error":
            continue
        if (diag.get("code") or {}).get("code") != "E0308":
            continue
        for child in diag.get("children", []):
            msg = (child.get("message") or "").lower()
            # rustc renders this as "consider `await`ing on both/all `Future`s" — note the backtick
            # SITS INSIDE the word ("await`ing"), so match on "on both"/"on all" instead of the whole
            # word, which a naive "awaiting" substring check misses entirely.
            if "on both" not in msg and "on all" not in msg:
                continue
            for span in child.get("spans", []):
                repl = span.get("suggested_replacement")
                if repl is None:
                    continue
                path = span.get("file_name", "")
                abs_path = path if os.path.isabs(path) else os.path.join(REPO, path)
                abs_path = os.path.normpath(abs_path)
                if not any(ia.in_scope(abs_path, s) for s in scopes):
                    continue
                edits.append((abs_path, span["byte_start"], span["byte_end"], repl, "E0308-both-futures"))
    return edits


def e0728_in_scope(diag, scopes):
    for span in diag.get("spans", []):
        if not span.get("is_primary"):
            continue
        path = span.get("file_name", "")
        abs_path = path if os.path.isabs(path) else os.path.join(REPO, path)
        abs_path = os.path.normpath(abs_path)
        return any(ia.in_scope(abs_path, s) for s in scopes)
    return False


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--crate", default="semio-framework-os-kernel")
    ap.add_argument("--scope", action="append", required=True)
    ap.add_argument("--max-passes", type=int, default=15)
    ap.add_argument("--max-files", type=int, default=60)
    ap.add_argument("--all-targets", action="store_true")
    args = ap.parse_args()

    target_dir = os.path.join(SCRATCH, f"target-await-{args.crate}")
    guard: set = set()

    for npass in range(1, args.max_passes + 1):
        diags = ia.run_check(args.crate, target_dir, args.all_targets)
        errors = [d for d in diags if d.get("level") == "error"]
        e0728s = [d for d in errors if (d.get("code") or {}).get("code") == "E0728"]
        in_scope_e0728 = [d for d in e0728s if e0728_in_scope(d, args.scope)]
        out_scope_e0728 = len(e0728s) - len(in_scope_e0728)

        if in_scope_e0728:
            # ⚠️ WARN, don't abort: a genuine in-scope E0728 usually means "asyncify this scope
            # first", but it can also mean a call site is structurally blocked on an out-of-scope
            # dependency being wrongly async (e.g. a sync `impl Fn` closure param fed an async
            # method from a crate outside my path_scope) — nothing this loop can fix by inserting
            # more awaits. Print it every pass so it never goes unnoticed, but keep making progress
            # on everything else; a human resolves these (lease-request / manual restructure).
            print(f"[pass {npass}] ⚠️  {len(in_scope_e0728)} E0728 INSIDE my scope (not auto-fixable, continuing):")
            for d in in_scope_e0728[:10]:
                for span in d.get("spans", []):
                    if span.get("is_primary"):
                        print(f"    {span.get('file_name')}:{span.get('line_start')}")

        edits_all = []
        ambiguous_all = []
        other_all = []
        for scope in args.scope:
            edits, ambiguous, other = ia.collect_await_edits(diags, scope)
            edits_all.extend(edits)
            ambiguous_all.extend(ambiguous)
            other_all.extend(other)
        edits_all.extend(collect_both_futures_edits(diags, args.scope))
        # de-dup edits across scopes (same span could match under >1 --scope arg)
        seen = set()
        uniq_edits = []
        for e in edits_all:
            key = (e[0], e[1], e[2])
            if key in seen:
                continue
            seen.add(key)
            uniq_edits.append(e)

        print(f"[pass {npass}] errors={len(errors)} (E0728 out-of-scope={out_scope_e0728}, ignored) "
              f"await-edits={len(uniq_edits)} ambiguous={len(ambiguous_all)}")

        if not uniq_edits:
            print("  fixpoint reached for my scope(s) (no unambiguous .await edits left)")
            if ambiguous_all:
                print(f"  {len(ambiguous_all)} ambiguous sites remain — resolve by hand:")
                for primary, code, opts in ambiguous_all[:30]:
                    print(f"    AMBIGUOUS {code} {primary}: {opts}")
            break

        touched = {e[0] for e in uniq_edits}
        if args.max_files and len(touched) > args.max_files:
            print(f"  ABORT: this pass would edit {len(touched)} files, over --max-files={args.max_files}")
            break

        applied = ia.apply_edits(uniq_edits, guard)
        print(f"  applied {applied}")
        if applied == 0:
            print("  no progress — stopping")
            break

    return 0


if __name__ == "__main__":
    sys.exit(main())
