#!/usr/bin/env python3
"""🧬 schema-evolve: one artifact-agnostic tool that adds a collection or a leaf to a norm artifact through every layer, idempotently.

  python3 -I evolve.py add-collection <artifact> <spec.json> [--dry-run | --apply]
  python3 -I evolve.py add-leaf <artifact> <entity-path> <spec.json> [--dry-run | --apply]
  python3 -I evolve.py add-enum-value <artifact> <EnumName> <spec.json> [--dry-run | --apply]
  python3 -I evolve.py convert-field-to-collection <artifact> <spec.json> [--dry-run | --apply]   (one-shot: object field -> top-level collection, fixtures rewritten losslessly by the spec's row rules)
  python3 -I evolve.py contract <artifact> [--dry-run | --apply]   (recompute schemaBytes, tableWidths and per-case valueBytes of the SQLite contract with python sqlite3; apply respects freeze.txt)
  python3 -I evolve.py verify set-fields [artifact ...]   (settable schema fields without a set-field mutation kind, repo-wide; exit 1 on a gap; the same rule `set-field-kinds` guards every plan)
  python3 -I evolve.py limits <artifact ...> [--apply]       (physical `minimum`/`maximum` of every numeric snapshot field, from `l_limits.py`; written into the snapshot, artifact and set-field mutation payload schemas as text edits; `l_limitsapply.py fix-fixtures|violations <artifact>` move or list committed fixture values outside the bounds)
  python3 -I evolve.py verify limits [artifact ...]            (numeric snapshot fields without a lower and an upper bound; exit 1; the rule `limits` guards every plan)
  python3 -I evolve.py verify steppers [artifact ...]   (suspicious x-semio-ui: labels that repeat a collection name, duplicate labels in a row type, steps that cannot belong to the field; exit 1; the rule `ui-steppers` guards every plan)
  python3 -I evolve.py repair <artifact|all> [--dry-run | --apply]   (stray type arguments and doubled quotes that earlier inline cloning left in the SQLite TypeScript codecs)
  python3 -I evolve.py verify <artifact>        (strict twin check, schema-sync, handle-paths, cargo authority, build-owners sync)
  python3 -I evolve.py verify <artifact> <command> <args...> <spec.json> [--only layers]   (plan scanners before apply: kind-kebab, nested_patch! order, row_key! presence, crate:: references across crate splits, snapshot initializers; exit 1 on a finding)

`--dry-run` (default) prints the file plan and the layers the artifact layout cannot take; `--apply` writes it, runs `schema twins generate`, replays the SQLite contract and syncs the build owners.
"""
import argparse, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from core import SEMIO, Unsupported
from ctx import Ctx
from spec import Spec

TICKET = os.environ.get("EVOLVE_TICKET") or os.path.abspath(os.path.join(HERE, "..", ".."))
HEAVY = TICKET + "/🗑️generated/build-runner/heavy.sh"


def collection_layers():
    import l_schema, l_rust, l_sqlite, l_wiring, l_fixtures, l_mutations
    return [
        ("schema", lambda c: (l_schema.run_documents(c), l_schema.run_diff(c))),
        ("model", lambda c: l_rust.model_rs(c)),
        ("literals", lambda c: l_rust.snapshot_literals(c)),
        ("artifact", lambda c: l_rust.artifact_rs(c)),
        ("diff", lambda c: (l_rust.diff_rs(c, l_schema.detect_prefix(c)), l_schema.sync_diff_lanes(c), __import__("l_patch").run(c))),
        ("sqlite", lambda c: l_sqlite.run(c)),
        ("wiring", lambda c: l_wiring.run(c)),
        ("fixtures", lambda c: l_fixtures.run(c)),
        ("mutations", lambda c: l_mutations.run(c)),
    ]


def frozen(short):
    path = TICKET + "/🗑️generated/build-runner/freeze.txt"
    return [l.strip() for l in open(path, encoding="utf8") if "build-" + short in l] if os.path.exists(path) else []


def contract(args):
    import l_extent
    spec = Spec.__new__(Spec)
    spec.reference = spec.nested_reference = spec.collection = None
    ctx = Ctx(args.artifact, spec, args.apply)
    ctx.guard(l_extent.refresh, "contract")
    for line in ctx.plan.report():
        print(line)
    for note in ctx.plan.notes:
        print("  note", note)
    for item in ctx.unsupported:
        print("  UNSUPPORTED", item)
    if not args.apply:
        return 0
    if ctx.unsupported:
        return 2
    if frozen(args.artifact):
        print("[schema-evolve] freeze.txt names the artifact (graph-plan running); nothing written:", *frozen(args.artifact))
        return 3
    ctx.plan.commit()
    return 0


def repair(args):
    """🩺 Plans (and with --apply writes) the repairs of `l_repair` for the artifacts named by `artifact` (a short name or `all`)."""
    import l_repair
    from core import Layout
    shorts = [a for a in ("en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999")] if args.artifact == "all" else [args.artifact]
    status = 0
    for short in shorts:
        spec = Spec.__new__(Spec)
        spec.reference = spec.nested_reference = spec.collection = None
        ctx = Ctx(short, spec, args.apply)
        ctx.guard(l_repair.run, "repair")
        for line in ctx.plan.report():
            print(line)
        for note in ctx.plan.notes:
            print("  note", note)
        if args.apply and ctx.plan.files:
            if frozen(short):
                print("[schema-evolve] freeze.txt names", short, "- nothing written")
                status = 3
                continue
            import l_verify
            ctx.plan.commit()
    return status


def build(command, artifact, rest, apply_=False, only=None):
    """🏗️ Runs the layers of `command` into a plan (nothing is written) and returns the context."""
    spec = Spec(rest[-1])
    ctx = Ctx(artifact, spec, apply_)
    ctx.prefix = None
    if command == "convert-field-to-collection":
        import l_convert
        ctx.rule = l_convert.Rule(spec.raw["convert"], spec.collection.plural)
        layers = l_convert.layers(collection_layers())
    elif command == "add-enum-value":
        import l_enum
        spec.raw["enumValue"]["enum"] = rest[0]
        ctx.enum_value = l_enum.Value(spec.raw["enumValue"])
        layers = l_enum.layers()
    elif command == "add-collection":
        layers = collection_layers()
    else:
        import l_leaf
        ctx.entity_path = rest[0]
        layers = l_leaf.layers()
    for name, fn in layers:
        if only and name not in only:
            continue
        ctx.guard(fn, name)
    return ctx


def scan_plan(ctx, only=None):
    """🛡️ Prints the scanner findings of the plan; returns their count."""
    import l_verify
    findings = l_verify.scan(ctx, only)
    for name, finding in findings:
        print("  FINDING", finding)
    print(f"[schema-evolve] plan scanners: {len(findings)} findings ({', '.join(n for n, _ in l_verify.CHECKS)})")
    return len(findings)


def main(argv):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("command", choices=["add-collection", "add-leaf", "add-enum-value", "convert-field-to-collection", "contract", "repair", "verify", "limits"])
    ap.add_argument("artifact")
    ap.add_argument("rest", nargs="*")
    mode = ap.add_mutually_exclusive_group()
    mode.add_argument("--dry-run", action="store_true")
    mode.add_argument("--apply", action="store_true")
    ap.add_argument("--only", help="comma separated layers")
    ap.add_argument("--diff", metavar="PATH_SUBSTRING", help="print the unified diff of the planned files whose path contains this text")
    args = ap.parse_args(argv)
    only = set(args.only.split(",")) if args.only else None
    if args.command == "verify" and args.artifact == "set-fields":
        import l_verify
        gaps = l_verify.repo_set_field_gaps(args.rest)
        for folder, paths in gaps.items():
            for path in paths:
                print(f"  FINDING set-field-kinds: {folder}: {path} has no set-field mutation kind")
        print(f"[schema-evolve] set-field coverage: {sum(map(len, gaps.values()))} settable schema fields without a kind in {len(gaps)} artifacts")
        return 1 if any(gaps.values()) else 0
    if args.command == "verify" and args.artifact == "limits":
        import l_limitsapply
        found = l_limitsapply.repo_findings(args.rest)
        for folder, items in found.items():
            for item in items:
                print(f"  FINDING limits: {folder}: {item}")
        print(f"[schema-evolve] limits check: {sum(map(len, found.values()))} numeric fields without bounds in {len(found)} artifacts")
        return 1 if any(found.values()) else 0
    if args.command == "limits":
        import l_limitsapply
        return l_limitsapply.run(args.rest, args.apply)
    if args.command == "verify" and args.artifact == "steppers":
        import l_verify
        found = l_verify.repo_ui_findings(args.rest)
        for folder, items in found.items():
            for item in items:
                print(f"  FINDING ui-steppers: {folder}: {item}")
        print(f"[schema-evolve] ui check: {sum(map(len, found.values()))} findings in {len(found)} artifacts")
        return 1 if any(found.values()) else 0
    if args.command == "verify" and args.rest:
        ctx = build(args.rest[0], args.artifact, args.rest[1:], only=only)
        for item in ctx.unsupported:
            print("  UNSUPPORTED", item)
        found = scan_plan(ctx)
        return 1 if found or ctx.unsupported else 0
    if args.command == "verify":
        import post
        from core import Layout
        ctx = type("V", (), {"short": args.artifact, "layout": Layout(args.artifact)})()
        return post.gates(ctx)
    if args.command == "contract":
        return contract(args)
    if args.command == "repair":
        return repair(args)
    ctx = build(args.command, args.artifact, args.rest, args.apply, only)
    print(f"[schema-evolve] {args.command} {args.artifact} {' '.join(args.rest)} ({'apply' if args.apply else 'dry run'})")
    for line in ctx.plan.report():
        print(line)
    if args.diff:
        import difflib
        for path, (old, new, _) in sorted(ctx.plan.files.items()):
            if args.diff in path:
                print("".join(difflib.unified_diff((old or "").splitlines(1), new.splitlines(1), "old", "new", n=0)))
    for note in ctx.plan.notes:
        print("  note", note)
    for item in ctx.unsupported:
        print("  UNSUPPORTED", item)
    print(f"[schema-evolve] {len(ctx.plan.files)} files, {len(ctx.unsupported)} unsupported layers")
    if not args.apply:
        return 0
    if ctx.unsupported:
        print("[schema-evolve] refusing to apply: layers above are unsupported for this artifact (nothing written)")
        return 2
    if scan_plan(ctx):
        print("[schema-evolve] refusing to apply: the plan scanners found known compile or law pitfalls (nothing written)")
        return 2
    ctx.plan.commit()
    import post
    return post.run(ctx)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
