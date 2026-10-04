"""🧳️ Re-seals the `nested-cargo-packages-v1` authority catalog after ten wgpu destination files moved or were deleted individually.

Moves replace every exact JSON-string occurrence of the old destination path by the live one (files still inside the package);
deletions drop the mapping rows of files that were deleted or left the package. Starts from the previous seal (the index copy, read
with `git show`), writes the catalog, prints the new seal and the ledger row (moves + deletions with each removed row and its original
index, so the ledger's reverse law reproduces the previous seal byte for byte). `--check` verifies the live catalog is re-sealed.
"""
import hashlib
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
CATALOG = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json"
ENGINE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/"
WGPU = ENGINE + "🎯️targets/🧊️wgpu/"
MOVES = [
    (WGPU + "📦️packages/🦀️rust/package.json", WGPU + "📦️packages/🟦️typescript/package.json", "8add1df1473"),
    (WGPU + "📦️packages/🦀️rust/📋️project.json", WGPU + "📦️packages/🟦️typescript/📋️project.json", "8add1df1473"),
    (WGPU + "📦️packages/🦀️rust/📜️script.ts", WGPU + "📦️packages/🟦️typescript/📜️script.ts", "8add1df1473"),
]
DELETIONS = [
    (WGPU + "📦️packages/🦀️rust/Trunk.toml", "b2064cc237b", "Trunk was retired: the browser build is the renderer's own compiler and server lanes."),
    (WGPU + "📦️packages/🦀️rust/🌐️.html", "b2064cc237b", "Trunk's page left the package with Trunk; the server lane owns its own page (`🌐️server/🌐️.html`)."),
    (WGPU + "🧪️tests/🟦️browser-frame-transport.ts", "025ec86a429", "The law left the wgpu package for the engine's shared test tree (`🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts`)."),
    (WGPU + "🧪️tests/🟦️browser-interactive-job-port.ts", "025ec86a429", "The law left the wgpu package for the engine's shared test tree (`🧑‍🎨engine/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts`)."),
    (WGPU + "🧪️tests/🟦️package-integration.ts", "025ec86a429", "The law left the wgpu package for the engine's shared test tree (`🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts`)."),
    (WGPU + "🪢️kernel-seam/🦀️.rs", "7bea15c349b", "The kernel seam module was deleted; its callers use the kernel runtime directly."),
    (WGPU + "📦️packages/🦀️rust/🟦️typescript/🧪️test/🟦️s.ts", "025ec86a429", "The bundled TypeScript smoke test was deleted with the nested TypeScript tree."),
]


def canonical(value):
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def main():
    text = CATALOG.read_text(encoding="utf-8")
    if "--check" not in sys.argv:
        text = subprocess.run(["git", "show", ":" + str(CATALOG.relative_to(ROOT))], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf-8")
    original = json.loads(text)
    assert canonical(json.loads(text)) == text, "the catalog is canonical JSON (2-space indent)"
    if "--check" in sys.argv:
        stale = [old for old, _, _ in MOVES if json.dumps(old, ensure_ascii=False) in text] + [path for path, _, _ in DELETIONS if json.dumps(path, ensure_ascii=False) in text]
        print("re-sealed" if not stale else f"stale: {stale}", sha(text))
        sys.exit(1 if stale else 0)
    previous = sha(text)
    for old, new, _ in MOVES:
        quoted_old, quoted_new = json.dumps(old, ensure_ascii=False), json.dumps(new, ensure_ascii=False)
        assert quoted_new not in text, f"{new} is not yet named"
        assert quoted_old in text, f"{old} is named"
        text = text.replace(quoted_old, quoted_new)
    value = json.loads(text)
    deletions = []
    for path, revision, reason in DELETIONS:
        package = next(package for package in value["packages"] if any(row["destinationPath"] == path for row in package["mappings"]))
        original_rows = next(item for item in original["packages"] if item["id"] == package["id"])["mappings"]
        index = next(position for position, row in enumerate(original_rows) if row["destinationPath"] == path)
        row = next(row for row in package["mappings"] if row["destinationPath"] == path)
        package["mappings"].remove(row)
        deletions.append({"path": path, "reason": reason, "revision": revision, "packageId": package["id"], "index": index, "row": row})
    text = canonical(value)
    for path, _, _ in DELETIONS:
        assert json.dumps(path, ensure_ascii=False) not in text, f"{path} is gone"
    CATALOG.write_text(text, encoding="utf-8")
    ledger = {
        "seal": sha(text),
        "previous": previous,
        "moves": [{"from": old, "to": new, "revision": revision} for old, new, revision in MOVES],
        "deletions": deletions,
    }
    print(json.dumps(ledger, ensure_ascii=False))


if __name__ == "__main__":
    main()
