#!/usr/bin/env python3
"""🧱️ Builds the staged batches `staged/<batch>` (wasmlock, vitestext) from the CURRENT live files: rebase tool, rerun after the gate lands anything in these files (`lock.py` first when `✏️s` manifests moved). Edits of hot files are anchored replacements that must match exactly once.
usage: python3 -I make.py <batch>..."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def sha(data): return hashlib.sha256(data).hexdigest()
def live(rel): return open(os.path.join(ROOT, rel), "rb").read()
def src(name): return open(os.path.join(HERE, "src", name), "rb").read()
def once(text, old, new):
    if text.count(old) != 1: sys.exit(f"anchor not unique ({text.count(old)}): {old[:80]}")
    return text.replace(old, new)
def edit(rel, *pairs):
    def build():
        text = live(rel).decode()
        for old, new in pairs: text = once(text, old, new)
        return text.encode()
    return build
def registry():
    data = json.loads(live(LIB + "⚡️caching/🦀️cargo/🕸️wasm-bindgen/🔣️.json"))
    if "✏️s/Cargo.lock" not in data["wasmLocks"]: data["wasmLocks"].insert(data["wasmLocks"].index("🌎️hub/Cargo.lock") + 1, "✏️s/Cargo.lock")
    return (json.dumps(data, ensure_ascii=False, indent=2) + "\n").encode()
BATCHES = {
    "wasmlock": ("🕸️ Staged batch `wasmlock` (lane playdemo): every WASM source owner resolves a declared lockfile pinned to the root wasm-bindgen; `✏️s/Cargo.lock` follows the pin and is declared; a drifted owner lock is refused before its compile.", {
        "✏️s/Cargo.lock": lambda: src("s-Cargo.lock"),
        LIB + "⚡️caching/🦀️cargo/🕸️wasm-bindgen/🔣️.json": registry,
        LIB + "⚡️caching/🦀️cargo/🕸️wasm-bindgen/🟦️.ts": lambda: src("bindgen.ts"),
        LIB + "⚡️caching/🧪️tests/🕸️wasm-bindgen-locks/🟦️.ts": lambda: src("test.ts"),
        LIB + "🏃️process/🕸️wasm/🔣️policy/🟦️.ts": edit(LIB + "🏃️process/🕸️wasm/🔣️policy/🟦️.ts",
            ('import { wasmBindgenVersion } from "../../../⚡️caching/🦀️cargo/🕸️wasm-bindgen/🟦️.ts";', 'import { requireWasmBindgenSourceLockV1, wasmBindgenVersion } from "../../../⚡️caching/🦀️cargo/🕸️wasm-bindgen/🟦️.ts";'),
            ("root = getWorkspaceRoot(compilerEnvironment), bindgen = ", 'root = getWorkspaceRoot(compilerEnvironment);\n  requireWasmBindgenSourceLockV1(root, relative(root, resolve(cwd)).replaceAll("\\\\", "/"));\n  const bindgen = ')),
    }),
    "vitestext": ("🧪️ Staged batch `vitestext` (lane playdemo): the Play and demonstrator unit suites load the build library natively instead of through Vite's module runner, which cannot import the library's captured `blob:` inventory module.", {
        f"{p}/🧪️tests/🎚️config/🟦️.ts": edit(f"{p}/🧪️tests/🎚️config/🟦️.ts", ('    environment: "node",\n', '    environment: "node",\n    server: { deps: { external: [/\\/🦑️repo\\/🔨️modules\\/📚️library\\//] } },\n'))
        for p in ("🏢️semio-tech/🎡️play", "♻️mit-bestand/🧺️demonstrator")
    }),
}
def build(batch):
    doc, files = BATCHES[batch]
    out = os.path.join(HERE, "staged", batch)
    shutil.rmtree(out, ignore_errors=True)
    for sub in ("files", "base"): os.makedirs(os.path.join(out, sub))
    manifest = {}
    for index, (rel, make) in enumerate(files.items()):
        name, base, new = f"{index:02d}", live(rel), make()
        open(os.path.join(out, "base", name), "wb").write(base); open(os.path.join(out, "files", name), "wb").write(new)
        manifest[rel] = {"file": name, "base": sha(base), "new": sha(new)}
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(HERE, "..", "xplat5-tools", "stage.template.py"), encoding="utf8").read().split("\n")
    template[1] = '"""' + doc
    open(os.path.join(out, "stage.py"), "w", encoding="utf8").write("\n".join(template))
    shutil.copyfile(os.path.join(HERE, f"verify-{batch}.sh"), os.path.join(out, "verify.sh"))
    print("[DEBUG] make.py: staged", len(manifest), "files in", out)
for batch in sys.argv[1:] or BATCHES: build(batch)
