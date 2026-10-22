"""📦 Independent Python oracle of the pinned tooling install image (standard library only): the package manifest and lock the installer sees, with every patch stored under the sha256 of its bytes. Requests arrive as JSON on stdin: {"cases": [{"recipe", "lock", "sources"}]} (patch bytes as text); patch content validity is not judged here."""
import hashlib, json, sys


def build(case):
    recipe, lock, sources = case["recipe"], case["lock"], case["sources"]
    if not isinstance(lock, dict):
        return {"error": "Pinned tooling lock must be an object"}
    patches_declared = recipe["semio"]["toolPatches"]
    if json.dumps(lock.get("patchedDependencies", "<absent>"), separators=(",", ":"), ensure_ascii=False) != json.dumps(patches_declared, separators=(",", ":"), ensure_ascii=False):
        return {"error": "Pinned tooling patch authority changed"}
    patches, files = {}, {}
    for name, path in patches_declared.items():
        if path not in sources:
            return {"error": "Pinned tooling patch bytes required"}
        data = sources[path].encode("utf-8")
        artifact = "patches/" + hashlib.sha256(data).hexdigest() + ".patch"
        patches[name] = artifact
        files.setdefault(artifact, sources[path])
    dump = lambda value: json.dumps(value, indent=2, ensure_ascii=False) + "\n"
    manifest = {"name": recipe["name"], "private": recipe["private"], "packageManager": recipe["packageManager"], "dependencies": {k: v for k, v in recipe["dependencies"].items() if v != "workspace:*"}, "patchedDependencies": patches}
    return {"files": [["package.json", dump(manifest)], ["bun.lock", dump({**lock, "patchedDependencies": patches})]] + [[name, text] for name, text in files.items()]}


print(json.dumps([build(c) for c in json.load(sys.stdin)["cases"]], ensure_ascii=False))
