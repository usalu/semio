"""📦️ [obsopen] (re)builds one staged batch from the CURRENT live files: copies the named files into a scratch root, runs the batch's anchored edit scripts there and writes `staged/<batch>/{manifest.json,base/,files/,stage.py,verify.sh}` (stage.py = C/xplat5-tools/stage.template.py: keeps file modes; new files have base null). Rerun right before landing when a file drifted. usage: python3 -I make_batch.py <batch> [--root LIVE]"""
import hashlib, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.abspath(os.path.join(C, *[".."] * 7))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
OBS = L + "🏗️build/👁️observation/🧪️tests/"
P = "🧰️framework/🔨️modules/🏃️process/"
STORAGE = P + "📦️artifacts/🏗️native-build/💾️storage/"
BOOT = STORAGE + "🌐️platform/🥾️boot/"
SETTLE = P + "🎛️owned-execution/🏁️settlement/"
BATCHES = {
    "nestedfix": {
        "doc": "🧵️ Staged batch `nestedfix` (lane obsopen): an outer Graph fixture reopens its receiver epoch inside the async context it was born in (the nested-fixture custody hang), the nested-fixture laws are bounded and declare their envelope, the presence law passes the consumer's own control to its nested fixture, keeps the primary first beneath aggregated failures and runs one law per child row.",
        "scripts": ["edit_nested.py"],
        "files": [L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts", OBS + "🧱️blocks/🟦️.ts", OBS + "🧱️blocks/👷️child/🟦️.ts", OBS + "🪪️presence/🟦️.ts", OBS + "🪪️presence/👷️child/🟦️.ts", OBS + "📥️reuse/🟦️.ts"],
    },
    "lostbirth": {
        "doc": "🕳️ Staged batch `lostbirth` (lane obsopen): a child whose process vanished while neither exit nor close arrived is refused by the capture with a closed typed cause, and the idempotent boot session read repeats exactly then, bounded by the declared attempts.",
        "scripts": ["edit_lost.py"],
        "files": [SETTLE + "🧬️schema/🔣️.json", SETTLE + "🟦️.ts", SETTLE + "🧪️tests/🟦️.ts", P + "🎛️owned-execution/📥️capture/🟦️.ts", STORAGE + "🧬️schema/🔣️.json", BOOT + "🟦️.ts", BOOT + "🧪️tests/🟦️.ts", BOOT + "🧪️tests/🧫️fixtures/🔣️.json"],
    },
}


def sha(path):
    if not os.path.exists(path):
        return None
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


batch = sys.argv[1]
spec = BATCHES[batch]
out = os.path.join(HERE, "staged", batch)
scratch = os.path.join(C, "🗑️generated", "obsopen", "scratch-" + batch)
shutil.rmtree(scratch, ignore_errors=True)
for rel in spec["files"]:
    if os.path.exists(os.path.join(LIVE, rel)):
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
    if sha(base) is not None:
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
os.chmod(os.path.join(out, "stage.py"), 0o755)
shutil.copyfile(os.path.join(HERE, "verify.sh"), os.path.join(out, "verify.sh"))
print(f"staged {batch}: {len(manifest)} files -> {out}")
