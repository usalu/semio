#!/usr/bin/env python3
"""🕊 [buildmem] Builds the staged batch `releasefirst` from the CURRENT files of a root (default: live, walked up from here; stage it on a root WITH the D1 group applied): anchored edits, every anchor must occur exactly once; writes files/, base/, manifest.json beside this script. Rebase = run it again on the moved tree.
usage: python3 -I make.py [--root DIR]"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
EDITS = {
    B + "🕸️graph/🕰️generations/🔌️load/🟨️.mjs": [
        ("const part=await host.discoverPart(spec,signal,control);announceWithheldProjectsV1(host,part);const shaped=shapeComputeResultV1({workspaceRoot:host.workspaceRoot,separator:host.separator,part,stubs:spec.stubs,implementation:scoped});await host.release?.();return shaped;}",
         "const part=await host.discoverPart(spec,signal,control);announceWithheldProjectsV1(host,part);await host.release?.();return shapeComputeResultV1({workspaceRoot:host.workspaceRoot,separator:host.separator,part,stubs:spec.stubs,implementation:scoped});}"),
    ],
    B + "🕸️graph/🕰️generations/🔌️load/🧪️tests/🕊️early-release/🟦️.ts": [
        ("/** 🕊️ The builder drops the inference caches (memoized closures, parsed manifests, observation images, interned rows) as soon as the partial inference is shaped, BEFORE the fact index is patched, packed and written:",
         "/** 🕊️ The builder drops the inference caches (memoized closures, parsed manifests, observation images, interned rows) as soon as the partial inference is discovered, BEFORE it is shaped and the fact index is patched, packed and written:"),
        ("the caches only save recomputation and the index work needs none of them, so its peak no longer rides on the inference working set.",
         "the caches only save recomputation and neither the shaping nor the index work needs any of them, so their peak no longer rides on the inference working set (36 refreshed projects: the release frees 0.9 GB of the 2.65 GB live heap)."),
        ('test("the inference caches are released after the shaping and before the generation is published, and once more when the load ends", async () => {',
         'test("the inference caches are released before the shaping reads the part", async () => {\n'
         '  const dir = mkdtempSync(join(realpathSync(tmpdir()), "graph-release-shape-")), order: string[] = [];\n'
         '  try {\n'
         '    await seed(dir, ringFiles(6));\n'
         '    const mock = createMockHost(dir);\n'
         '    const host = {\n'
         '      ...mock.host,\n'
         '      discoverPart: async (...args: Parameters<typeof mock.host.discoverPart>) => { const part = await mock.host.discoverPart(...args); order.push("discover"); return new Proxy(part, { get: (target, key, receiver) => { if (key === "edgeOnly") order.push("shape"); return Reflect.get(target, key, receiver); } }); },\n'
         '      release: async () => { order.push("release"); },\n'
         '    };\n'
         '    await loadGenerationGraphV1(host as never);\n'
         '    expect(order.indexOf("shape")).toBeGreaterThan(order.indexOf("discover"));\n'
         '    expect(order.indexOf("release")).toBeGreaterThan(order.indexOf("discover"));\n'
         '    expect(order.indexOf("release")).toBeLessThan(order.indexOf("shape"));\n'
         '  } finally { rmSync(dir, {recursive: true, force: true}); }\n'
         '});\n\n'
         'test("the inference caches are released after the discovery and before the generation is published, and once more when the load ends", async () => {'),
    ],
}
def sha(data): return hashlib.sha256(data).hexdigest()
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and here != "/": here = os.path.dirname(here)
    return here
def main():
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
    for sub in ("files", "base"):
        shutil.rmtree(os.path.join(HERE, sub), ignore_errors=True); os.makedirs(os.path.join(HERE, sub))
    manifest = {}
    for index, (rel, edits) in enumerate(EDITS.items()):
        raw = open(os.path.join(root, rel), "rb").read(); text = raw.decode("utf8")
        for old, new in edits:
            if text.count(old) != 1: sys.exit(f"anchor count {text.count(old)} in {rel[-70:]}: {old[:80]!r}")
            text = text.replace(old, new)
        name = f"{index:02d}" + os.path.splitext(rel)[1]
        open(os.path.join(HERE, "base", name), "wb").write(raw); open(os.path.join(HERE, "files", name), "wb").write(text.encode("utf8"))
        manifest[rel] = {"file": name, "base": sha(raw), "new": sha(text.encode("utf8"))}
    json.dump(manifest, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(f"staged {len(manifest)} files from {root}")
main()
