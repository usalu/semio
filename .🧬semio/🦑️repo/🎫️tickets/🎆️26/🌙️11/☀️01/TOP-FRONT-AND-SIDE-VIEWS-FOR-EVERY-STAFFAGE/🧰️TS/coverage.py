"""Per-asset per-scale view coverage of the staffage catalog (E elevation, F front, P plan shape counts) with the orthographic consistency status.

Usage (from anywhere):
  python coverage.py [--asset ID ...] [--missing-only] [--consistency] [--json]

Coverage reads the catalog JSON only. `--consistency` runs the guard's consistency reporter (bun through `repo tools exec`) and adds one status per
asset and scale: ok, or the failing checks. Without the flag a cached result in `../🗑️generated/TS/consistency.json` is used when present.
"""
import argparse
import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
GENERATED = TICKET / "🗑️generated" / "TS"
CACHE = GENERATED / "consistency.json"
SCALES = ("50", "100", "200", "500")


def repository_root():
    for parent in HERE.parents:
        if (parent / "AGENTS.md").exists():
            return parent
    raise SystemExit("repository root (AGENTS.md) not found above " + str(HERE))


ROOT = repository_root()
CATALOG = ROOT / "🎓️teaching" / "🏛️architecture" / "🧍️staffage" / "🗂️catalog"


def load_catalog(only):
    for group in sorted(p for p in CATALOG.iterdir() if p.is_dir()):
        for folder in sorted(p for p in group.iterdir() if p.is_dir()):
            file = folder / "🔣️.json"
            if file.exists() and (not only or folder.name in only):
                yield folder.name, json.loads(file.read_text(encoding="utf-8"))["asset"]


def counts(representation):
    geometry = representation["geometry"]
    return tuple(len(geometry.get(view) or []) for view in ("elevation", "front", "plan"))


def run_consistency(assets):
    GENERATED.mkdir(parents=True, exist_ok=True)
    script = HERE / "consistency.ts"
    result = subprocess.run(["repo", "tools", "exec", "bun", "--", str(script), str(CACHE), *assets], cwd=ROOT, text=True)
    if result.returncode != 0:
        raise SystemExit("consistency reporter failed")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--asset", action="append", default=[])
    parser.add_argument("--missing-only", action="store_true")
    parser.add_argument("--consistency", action="store_true")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()
    if args.consistency:
        run_consistency(args.asset)
    status = json.loads(CACHE.read_text(encoding="utf-8")) if CACHE.exists() else {}
    rows, authored, complete, assets_complete = [], 0, 0, 0
    for asset_id, asset in load_catalog(set(args.asset)):
        cells, asset_ok = {}, True
        for scale in SCALES:
            representation = asset["representations"].get(scale)
            if representation is None or representation["status"] != "authored":
                cells[scale] = None
                continue
            e, f, p = counts(representation)
            authored += 1
            done = e > 0 and f > 0 and p > 0
            complete += done
            asset_ok &= done
            cells[scale] = {"E": e, "F": f, "P": p, "complete": done, "consistency": status.get(asset_id, {}).get(scale)}
        assets_complete += asset_ok
        if args.missing_only and asset_ok:
            continue
        rows.append((asset_id, cells, asset_ok))
    if args.json:
        json.dump({a: c for a, c, _ in rows}, sys.stdout, ensure_ascii=False, indent=2)
        print()
        return
    print(f"{'asset':36} " + " ".join(f"{'1:' + s:^22}" for s in SCALES))
    for asset_id, cells, asset_ok in rows:
        parts = []
        for scale in SCALES:
            cell = cells[scale]
            if cell is None:
                parts.append(f"{'unavailable':^22}")
                continue
            mark = ""
            if cell["consistency"] is not None:
                mark = " ok" if cell["consistency"] == "ok" else " BAD"
            parts.append(f"{cell['E']:>3}/{cell['F']:>3}/{cell['P']:>3}{mark:<4}{'' if cell['complete'] else ' !':<2}".ljust(22))
        print(f"{asset_id:36} " + " ".join(parts))
    print(f"\nrepresentations with E+F+P: {complete}/{authored}; assets complete: {assets_complete}/{len(rows) if not args.missing_only else '-'}")
    if status:
        bad = {a: {s: v for s, v in per.items() if v != "ok"} for a, per in status.items()}
        for a, per in sorted(bad.items()):
            for s, v in sorted(per.items()):
                if args.asset and a not in args.asset:
                    continue
                print(f"  consistency {a} 1:{s}: {v}")


if __name__ == "__main__":
    main()
