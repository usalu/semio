#!/usr/bin/env python3
"""Revert authoredSourceFragments that were successfully applied by execute_nested_cargo_projection.py
but turned out to depend on a sibling fragment ('single-package-artifact-producer') that failed to
apply because its target span had been rewritten by concurrent, unrelated work (a decodeAstralEscapes
helper added to the wgpu package's script.ts after the ledger was authored). Applying only PART of
this interdependent group left a dangling reference (`runBrowserArtifacts` called but never defined,
`writeFileSync` used but not imported). This reverts exactly that group back to oldValue so the file
is self-consistent again, deferring the whole "producer consolidation" feature rather than forcing it.
"""
import argparse, json, os, sys

CAT_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json"

REVERT_IDS = {
    "trunk-build-package-artifacts",
    "trunk-serve-package-artifacts",
    "browser-check-single-producer",
    "retire-renderer-generation-routes",
    "producer-read-only-fs-import",
    "producer-path-import",
    "retire-renderer-nx-producers",
    "retire-wasm-duplicate-producer",
    "retire-serve-duplicate-producer",
    "retire-dev-duplicate-producer",
}

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()
    apply = args.apply

    with open(CAT_PATH, encoding="utf-8") as f:
        cat = json.load(f)

    results = []
    for pkg in cat["packages"]:
        mapping_by_source = {m["sourcePath"]: m for m in pkg["mappings"]}
        for frag in pkg.get("authoredSourceFragments", []):
            if frag["id"] not in REVERT_IDS:
                continue
            mapping = mapping_by_source[frag["sourcePath"]]
            dp = mapping["destinationPath"]
            if not os.path.exists(dp):
                results.append({"id": frag["id"], "path": dp, "status": "MISSING-TARGET"})
                continue
            content = open(dp, "r", encoding="utf-8").read()
            new, old = frag["oldValue"], frag["newValue"]  # reverse: find newValue, put back oldValue
            count = content.count(old)
            if count != 1:
                results.append({"id": frag["id"], "path": dp, "status": f"newvalue-count-{count}"})
                continue
            if apply:
                content = content.replace(old, new, 1)
                open(dp, "w", encoding="utf-8").write(content)
            results.append({"id": frag["id"], "path": dp, "status": "reverted" if apply else "would-revert"})

    print(json.dumps(results, ensure_ascii=False, indent=2))
    bad = [r for r in results if r["status"] not in ("reverted", "would-revert")]
    if bad:
        sys.exit(1)

if __name__ == "__main__":
    main()
