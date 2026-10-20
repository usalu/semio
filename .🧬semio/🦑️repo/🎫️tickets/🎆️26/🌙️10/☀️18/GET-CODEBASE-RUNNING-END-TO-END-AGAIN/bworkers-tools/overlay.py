# 🧪 Builds the scratch overlay: copies (never symlinks) of the files the adapter, the worker modules and their tests import, at their repository-relative paths below a short ASCII root, then lays the staged files over them.
import json, os, shutil, subprocess, sys
import lib

OVERLAY = os.environ.get("BW_OVERLAY", "/private/tmp/bworkers/r")
ENTRIES = [
    lib.BAZEL + "/" + lib.SCRIPT,
    lib.BAZEL + "/" + lib.TESTS + "/" + lib.TS,
    lib.BAZEL + "/" + lib.TESTS + "/\U0001F4E5️actions/" + lib.TS,
    lib.BAZEL + "/" + lib.TESTS + "/\U0001F50F️implementation/" + lib.TS,
    lib.BAZEL + "/" + lib.TESTS + "/\U0001F4E5️sources/" + lib.TS,
    lib.BAZEL + "/" + lib.TESTS + "/\U0001F4E5️fingerprint/" + lib.TS,
    lib.BUILD + "/⚙️operations/" + lib.TESTS + "/" + lib.TS,
    lib.BUILD + "/\U0001F9ED️selection/" + lib.TS,
    lib.BUILD + "/\U0001F9ED️invocation/" + lib.TS,
    lib.BUILD + "/\U0001F9ED️invocation/" + lib.TESTS + "/" + lib.TS,
    lib.BUILD + "/\U0001F9ED️selection/" + lib.TESTS + "/" + lib.TS,
    lib.BOOT + "/" + lib.SCRIPT,
    lib.LIB + "/⚡️caching/" + lib.TESTS + "/⚡️cache-contracts/" + lib.TS,
    lib.NXTOOLS + "/⚙️operations/" + lib.TS,
]
TRACKED_TREES = [lib.LIB, lib.FW + "/" + lib.MODULES + "/\U0001F3C3\uFE0Fprocess", lib.FW + "/" + lib.MODULES + "/\U0001F9EC\uFE0Fschema"]
EXTRA_TREES = []


def metafile(entries):
    out = os.path.join(lib.TOOLS, "..", "\U0001F5D1️generated", "bworkers-overlay")
    os.makedirs(out, exist_ok=True)
    meta = os.path.join(out, "meta.json")
    present = [e for e in entries if os.path.exists(os.path.join(lib.ROOT, e))]
    cmd = ["bun", "build", "--target=bun", "--packages=external", "--metafile=" + meta, "--outdir=" + os.path.join(out, "o")] + ["./" + e for e in present]
    result = subprocess.run(cmd, cwd=lib.ROOT, capture_output=True, text=True)
    if result.returncode != 0:
        sys.stderr.write(result.stderr[-3000:])
    with open(meta, encoding="utf8") as handle:
        return sorted(json.load(handle)["inputs"].keys())


def main():
    files = set(metafile(ENTRIES))
    tracked = subprocess.run(["git", "ls-files", "-z", "--"] + TRACKED_TREES, cwd=lib.ROOT, capture_output=True).stdout.decode("utf8").split("\0")
    files.update(path for path in tracked if path and os.path.isfile(os.path.join(lib.ROOT, path)))
    for tree in EXTRA_TREES:
        for root, _dirs, names in os.walk(os.path.join(lib.ROOT, tree)):
            for name in names:
                files.add(os.path.relpath(os.path.join(root, name), lib.ROOT))
    for extra in ["bunfig.toml", "package.json", ".bazelrc"]:
        files.add(extra)
    copied = 0
    for relative in sorted(files):
        if relative.startswith("node_modules/") or relative.startswith("..") or relative.startswith("/"):
            continue
        source = os.path.join(lib.ROOT, relative)
        if not os.path.isfile(source):
            continue
        target = os.path.join(OVERLAY, relative)
        if len(target) > 256:
            sys.exit("path too long: " + target)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copyfile(source, target)
        copied += 1
    linked = 0
    scope = os.path.join(lib.ROOT, "node_modules", "@semio-tech")
    target_scope = os.path.join(OVERLAY, "node_modules", "@semio-tech")
    os.makedirs(target_scope, exist_ok=True)
    for name in sorted(os.listdir(scope)):
        destination = os.readlink(os.path.join(scope, name))
        if not destination.startswith(lib.ROOT + "/"):
            continue
        inside = os.path.join(OVERLAY, destination[len(lib.ROOT) + 1:])
        link = os.path.join(target_scope, name)
        if os.path.isdir(inside) and not os.path.lexists(link):
            os.symlink(inside, link)
            linked += 1
    store = os.path.join(lib.ROOT, "node_modules", ".bun")
    thirdparty = {"ajv": "ajv@8.20.0", "fast-deep-equal": "fast-deep-equal@3.1.3", "fast-uri": "fast-uri@3.1.8", "json-schema-traverse": "json-schema-traverse@1.0.0", "require-from-string": "require-from-string@2.0.2", "@iarna/toml": "@iarna+toml@2.2.5", "jsonschema": "jsonschema@1.5.0", "typescript": "typescript@5.9.3", "esbuild": "esbuild@0.27.7", "@esbuild/darwin-arm64": "@esbuild+darwin-arm64@0.27.7", "minimatch": "minimatch@9.0.3", "brace-expansion": "brace-expansion@2.1.7", "balanced-match": "balanced-match@1.0.2", "yargs-parser": "yargs-parser@21.1.1"}
    for name, entry in thirdparty.items():
        source = os.path.join(store, entry, "node_modules", name)
        target = os.path.join(OVERLAY, "node_modules", name)
        if os.path.isdir(source) and not os.path.exists(target):
            shutil.copytree(source, target, symlinks=False)
    print("copied", copied, "files to", OVERLAY, "and linked", linked, "workspace packages inside the overlay")


if __name__ == "__main__":
    main()
