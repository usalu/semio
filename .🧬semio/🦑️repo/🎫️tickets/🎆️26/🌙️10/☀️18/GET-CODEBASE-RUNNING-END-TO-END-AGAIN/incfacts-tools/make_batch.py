"""📦️ [incfacts] (re)builds staged batch `libfacts` from the CURRENT live files: copies the touched files into a scratch root, runs the
anchored `edit.py` there and writes `staged/libfacts/{manifest.json,base/,files/,stage.py,verify.sh}` (stage.py = C/xplat5-tools/stage.template.py,
which keeps file modes). Rerun right before landing when a file drifted. usage: python3 -I make_batch.py [--root LIVE]"""
import hashlib, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.abspath(os.path.join(C, *[".."] * 7))
INF = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/"
FILES = [INF + "🧱️project-assembly/🟨️.mjs", INF + "🧱️project-assembly/🟨️.d.mts", INF + "🧱️project-assembly/🧪️tests/🟦️.ts", INF + "🧭️script-inputs/🟨️.mjs",
         INF + "🧬️source-analyses/🟨️.mjs", INF + "♻️revision/🟨️.mjs", INF + "🔖️stamps/🧬️schema/🔣️.json", INF + "🧪️tests/🔖️output-stamps/🔣️.json",
         "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts"]
DOC = "🧲️ Staged batch `libfacts` (lane incfacts): project records carry no library facts; the input precision rules, their schema, the taxonomy file kinds and the presentation resources are facts of the load authority (implementation revision + load-time reads), the precision engine is named by the project-assembly stamp."


def sha(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


out = os.path.join(HERE, "staged", "libfacts")
scratch = os.path.join(C, "🗑️generated", "incfacts", "scratch")
shutil.rmtree(scratch, ignore_errors=True)
for rel in FILES:
    os.makedirs(os.path.dirname(os.path.join(scratch, rel)), exist_ok=True)
    shutil.copyfile(os.path.join(LIVE, rel), os.path.join(scratch, rel))
subprocess.run([sys.executable, "-I", os.path.join(HERE, "edit.py"), scratch], check=True)
shutil.rmtree(out, ignore_errors=True)
os.makedirs(os.path.join(out, "base"))
os.makedirs(os.path.join(out, "files"))
manifest = {}
for index, rel in enumerate(FILES):
    name = f"f{index:02d}"
    base, new = os.path.join(LIVE, rel), os.path.join(scratch, rel)
    if sha(base) == sha(new): raise SystemExit(f"refused: {rel} unchanged by edit.py")
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
    handle.write(template[:start] + DOC + template[end:])
shutil.copyfile(os.path.join(HERE, "verify.sh"), os.path.join(out, "verify.sh"))
shutil.rmtree(scratch, ignore_errors=True)
print(f"staged libfacts: {len(manifest)} files -> {out}")
