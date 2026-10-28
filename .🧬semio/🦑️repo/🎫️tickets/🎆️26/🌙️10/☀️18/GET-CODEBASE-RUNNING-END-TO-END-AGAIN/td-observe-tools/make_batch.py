"""📦️ [td-observe] (re)builds one staged batch from the CURRENT live files: copies the named files into a scratch root, runs the batch's
anchored edit scripts there, and writes `staged/<batch>/{manifest.json,base/,files/,stage.py}` (stage.py = C/xplat5-tools/stage.template.py,
which keeps file modes). Rerun right before landing when a file drifted. usage: python3 -I make_batch.py <batch> [--root LIVE]"""
import hashlib, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.abspath(os.path.join(C, *[".."] * 7))
OBS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/"
BATCHES = {
    "obsbare": {
        "doc": "📂️ Staged batch `obsbare` (lane td-observe): the main observation law gives the truth under a bare `bun test ./🟦️.ts` (physical artifact root), and the build suite route births the two Node-child project laws with the immutable Nx SDK (`setup-sdk`).",
        "scripts": ["edit_main_route.py"],
        "files": [OBS + "🧪️tests/🟦️.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🔣️.json"],
    },
    "obslaws": {
        "doc": "🧱️ Staged batch `obslaws` (lane td-observe): block and presence laws run on Bun (namespace spies instead of syncBuiltinESMExports, cold block reads), follow the damaged-block-is-a-miss design (schema enum + corpus), get one law per child row, and accept the process module's SIGINT text.",
        "scripts": ["edit_children.py", "edit_laws.py"],
        "files": [OBS + "🧬️schema/🔣️.json", OBS + "🧪️tests/🧱️blocks/🟦️.ts", OBS + "🧪️tests/🧱️blocks/🧫️cases.json", OBS + "🧪️tests/🧱️blocks/👷️child/🟦️.ts", OBS + "🧪️tests/🪪️presence/🟦️.ts", OBS + "🧪️tests/🪪️presence/👷️child/🟦️.ts"],
    },
}


def sha(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


batch = sys.argv[1]
spec = BATCHES[batch]
out = os.path.join(HERE, "staged", batch)
scratch = os.path.join(C, "🗑️generated", "td-observe", "scratch-" + batch)
shutil.rmtree(scratch, ignore_errors=True)
for rel in spec["files"]:
    os.makedirs(os.path.dirname(os.path.join(scratch, rel)), exist_ok=True)
    shutil.copyfile(os.path.join(LIVE, rel), os.path.join(scratch, rel))
for script in spec["scripts"]:
    subprocess.run([sys.executable, "-I", os.path.join(HERE, script), scratch], check=True)
shutil.rmtree(out, ignore_errors=True)
os.makedirs(os.path.join(out, "base"))
os.makedirs(os.path.join(out, "files"))
manifest = {}
for index, rel in enumerate(spec["files"]):
    name = f"f{index:02d}"
    base, new = os.path.join(LIVE, rel), os.path.join(scratch, rel)
    if sha(base) == sha(new):
        raise SystemExit(f"refused: {rel} unchanged by the edit scripts")
    shutil.copyfile(base, os.path.join(out, "base", name))
    shutil.copyfile(new, os.path.join(out, "files", name))
    manifest[rel] = {"file": name, "base": sha(base), "new": sha(new)}
with open(os.path.join(out, "manifest.json"), "w", encoding="utf8") as handle:
    json.dump(manifest, handle, ensure_ascii=False, indent=1)
with open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8") as handle:
    template = handle.read()
start = template.index('"""') + 3
end = template.index("\n", start)
with open(os.path.join(out, "stage.py"), "w", encoding="utf8") as handle:
    handle.write(template[:start] + spec["doc"] + template[end:])
shutil.copyfile(os.path.join(HERE, "verify.sh"), os.path.join(out, "verify.sh"))
print(f"staged {batch}: {len(manifest)} files -> {out}")
