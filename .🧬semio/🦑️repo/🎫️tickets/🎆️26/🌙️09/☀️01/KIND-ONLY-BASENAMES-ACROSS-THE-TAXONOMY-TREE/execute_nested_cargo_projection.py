#!/usr/bin/env python3
"""One-off executor for the nested-cargo-packages-v1 projection (wgpu-renderer + jcoprobe-guest).

Reads the authoritative catalog verbatim and applies: mapping moves, sourceSplices, adapters,
derivedLeaves, authoredSourceFragments. Skips known-blocked items (hash drift / missing source)
and reports them instead of guessing. Run with --apply to actually write; default is dry-run.
"""
import argparse, hashlib, json, os, shutil, sys

REPO = os.getcwd()
CAT_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json"

# Known blocked items, discovered during verification. Excluded from mechanical processing.
BLOCKED_SOURCE_PATHS = {
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts",
}
# Verified via `git show HEAD:<path> | sha256sum` to match the pinned hash exactly, even though
# the file is absent from the working tree (deleted by an unrelated concurrent wave).
GIT_RECOVERABLE_SOURCE_PATHS = {
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧪️vitest.config.ts",
}

def sha256_size(path):
    data = open(path, "rb").read()
    return hashlib.sha256(data).hexdigest(), len(data)

def log(*a):
    print(*a, flush=True)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()
    apply = args.apply

    with open(CAT_PATH, encoding="utf-8") as f:
        cat = json.load(f)

    report = {"moved": [], "spliced": [], "adapters_created": [], "derived_created": [],
              "fragments_applied": [], "skipped": [], "dedup_removed": [], "errors": []}

    for pkg in cat["packages"]:
        pkg_id = pkg["id"]
        mapping_by_source = {m["sourcePath"]: m for m in pkg["mappings"]}

        # 1) Verify + move mappings
        for m in pkg["mappings"]:
            sp, dp = m["sourcePath"], m["destinationPath"]
            if sp in BLOCKED_SOURCE_PATHS:
                report["skipped"].append({"reason": "hash-drift-unrelated-wave", "sourcePath": sp})
                continue
            if sp in GIT_RECOVERABLE_SOURCE_PATHS:
                # handled specially below via authoredSourceFragments materialization
                report["skipped"].append({"reason": "source-absent-git-verified", "sourcePath": sp, "destinationPath": dp})
                continue
            if not os.path.exists(sp):
                report["errors"].append({"reason": "source-missing-unexpected", "sourcePath": sp})
                continue
            h, sz = sha256_size(sp)
            if h != m["sourceHash"] or sz != m["sourceSize"]:
                report["errors"].append({"reason": "hash-mismatch-unexpected", "sourcePath": sp, "diskHash": h, "diskSize": sz})
                continue
            if os.path.exists(dp):
                # verify byte-identical dedup case
                dh, dsz = sha256_size(dp)
                if dh == h and dsz == sz:
                    report["dedup_removed"].append({"sourcePath": sp, "destinationPath": dp})
                    if apply:
                        os.remove(sp)
                    continue
                else:
                    report["errors"].append({"reason": "destination-collision-mismatched", "sourcePath": sp, "destinationPath": dp})
                    continue
            report["moved"].append({"sourcePath": sp, "destinationPath": dp})
            if apply:
                os.makedirs(os.path.dirname(dp), exist_ok=True)
                shutil.move(sp, dp)

        # 2) sourceSplices
        for sp_entry in pkg.get("sourceSplices", []):
            sp, dp = sp_entry["sourcePath"], sp_entry["destinationPath"]
            if sp in BLOCKED_SOURCE_PATHS:
                report["skipped"].append({"reason": "splice-skipped-blocked-source", "sourcePath": sp})
                continue
            target = dp
            if not apply:
                report["spliced"].append({"id": sp_entry["id"], "destinationPath": dp, "dryrun": True})
                continue
            if not os.path.exists(target):
                report["errors"].append({"reason": "splice-target-missing", "id": sp_entry["id"], "path": target})
                continue
            content = open(target, "r", encoding="utf-8").read()
            old, new = sp_entry["oldValue"], sp_entry["newValue"]
            count = content.count(old)
            if count != 1:
                report["errors"].append({"reason": f"splice-oldvalue-count-{count}", "id": sp_entry["id"], "path": target})
                continue
            content = content.replace(old, new, 1)
            open(target, "w", encoding="utf-8").write(content)
            report["spliced"].append({"id": sp_entry["id"], "destinationPath": dp})

        # 3) adapters (new files). A few adapter paths coincide with a sourceSplice destination
        # (the splice already writes the identical adapter content there) -- treat a byte-identical
        # pre-existing file as already-satisfied, not an error.
        splice_destinations = {s["destinationPath"] for s in pkg.get("sourceSplices", [])}
        for ad in pkg.get("adapters", []):
            path = ad["path"]
            if os.path.exists(path):
                actual = open(path, "r", encoding="utf-8").read()
                if actual == ad["content"]:
                    report["adapters_created"].append({"id": ad["id"], "path": path, "already_satisfied": True})
                elif path in splice_destinations and not apply:
                    report["adapters_created"].append({"id": ad["id"], "path": path, "pending_via_splice_dryrun": True})
                else:
                    report["errors"].append({"reason": "adapter-exists-content-mismatch", "id": ad["id"], "path": path})
                continue
            if path in splice_destinations:
                # produced by the sourceSplice step above; nothing further to do here.
                report["adapters_created"].append({"id": ad["id"], "path": path, "produced_by_splice": True})
                continue
            report["adapters_created"].append({"id": ad["id"], "path": path})
            if apply:
                os.makedirs(os.path.dirname(path), exist_ok=True)
                open(path, "w", encoding="utf-8").write(ad["content"])

        # 4) derivedLeaves (new files)
        for dl in pkg.get("derivedLeaves", []):
            path = dl["path"]
            if os.path.exists(path):
                actual = open(path, "r", encoding="utf-8").read()
                if actual == dl["content"]:
                    report["derived_created"].append({"id": dl["id"], "path": path, "already_satisfied": True})
                else:
                    report["errors"].append({"reason": "derived-leaf-exists-content-mismatch", "id": dl["id"], "path": path})
                continue
            report["derived_created"].append({"id": dl["id"], "path": path})
            if apply:
                os.makedirs(os.path.dirname(path), exist_ok=True)
                open(path, "w", encoding="utf-8").write(dl["content"])

        # 5) authoredSourceFragments
        for frag in pkg.get("authoredSourceFragments", []):
            sp = frag["sourcePath"]
            if sp in BLOCKED_SOURCE_PATHS:
                report["skipped"].append({"reason": "fragment-skipped-blocked-source", "id": frag["id"], "sourcePath": sp})
                continue
            mapping = mapping_by_source.get(sp)
            if not mapping:
                report["errors"].append({"reason": "fragment-no-mapping", "id": frag["id"], "sourcePath": sp})
                continue
            dp = mapping["destinationPath"]
            if sp in GIT_RECOVERABLE_SOURCE_PATHS:
                # No file was ever moved (source absent). Materialize destination directly with newValue,
                # since oldValue was verified (via git) to equal the WHOLE original file content.
                old_full = frag["oldValue"]
                report["fragments_applied"].append({"id": frag["id"], "destinationPath": dp, "mode": "materialize-from-git-verified-preimage"})
                if apply:
                    if os.path.exists(dp):
                        report["errors"].append({"reason": "materialize-target-exists", "id": frag["id"], "path": dp})
                        continue
                    os.makedirs(os.path.dirname(dp), exist_ok=True)
                    open(dp, "w", encoding="utf-8").write(frag["newValue"])
                continue
            if not apply:
                report["fragments_applied"].append({"id": frag["id"], "destinationPath": dp, "dryrun": True})
                continue
            if not os.path.exists(dp):
                report["errors"].append({"reason": "fragment-target-missing", "id": frag["id"], "path": dp})
                continue
            content = open(dp, "r", encoding="utf-8").read()
            old, new = frag["oldValue"], frag["newValue"]
            count = content.count(old)
            if count != 1:
                report["errors"].append({"reason": f"fragment-oldvalue-count-{count}", "id": frag["id"], "path": dp})
                continue
            content = content.replace(old, new, 1)
            open(dp, "w", encoding="utf-8").write(content)
            report["fragments_applied"].append({"id": frag["id"], "destinationPath": dp})

    log(json.dumps(report, ensure_ascii=False, indent=2))
    log("=== SUMMARY ===")
    for k, v in report.items():
        log(f"{k}: {len(v)}")
    if report["errors"]:
        sys.exit(1)

if __name__ == "__main__":
    main()
