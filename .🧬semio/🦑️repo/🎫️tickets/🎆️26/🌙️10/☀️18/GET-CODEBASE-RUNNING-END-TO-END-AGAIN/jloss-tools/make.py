"""🧰 Builds the staged batch `jloss` from the CURRENT live files and the lane's work sources: watcher source, its pin (sha256 + bytes of the new source), the new drop law. Exit 2 when the live watcher is not the source the work copy was derived from (rebase first: diff jloss-tools/base/watch.rs against live).
usage: python3 -I make.py"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ROOT = C
while not os.path.isdir(os.path.join(ROOT, "🧰️framework")): ROOT = os.path.dirname(ROOT)
R = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident"
RS, PIN, LAW = R + "/🔭️watch/🦀️rust/🦀️.rs", R + "/🔭️watch/🦀️rust/🔣️.json", R + "/🧪️tests/🔭️watch/🕳️drops/🟦️.ts"
OUT = os.path.join(HERE, "staged", "jloss")
sha = lambda data: hashlib.sha256(data).hexdigest()
read = lambda path: open(path, "rb").read() if os.path.exists(path) else None
live_rs = read(os.path.join(ROOT, RS))
if live_rs != read(os.path.join(HERE, "base", "watch.rs")):
    sys.exit("live watcher drifted from jloss-tools/base/watch.rs: rebase work/watch.rs first")
new_rs = read(os.path.join(HERE, "work", "watch.rs"))
pin = json.loads(read(os.path.join(ROOT, PIN)))
pin["sources"][0].update(sha256=sha(new_rs), bytes=len(new_rs))
new_pin = (json.dumps(pin, indent=1, ensure_ascii=False) + "\n").encode()
rows = {RS: ("00.rs", live_rs, new_rs), PIN: ("01.json", read(os.path.join(ROOT, PIN)), new_pin), LAW: ("02.ts", read(os.path.join(ROOT, LAW)), read(os.path.join(HERE, "work", "drops.ts")))}
for sub in ("files", "base"):
    shutil.rmtree(os.path.join(OUT, sub), ignore_errors=True); os.makedirs(os.path.join(OUT, sub))
manifest = {}
for rel, (name, base, new) in rows.items():
    open(os.path.join(OUT, "files", name), "wb").write(new)
    if base is not None: open(os.path.join(OUT, "base", name), "wb").write(base)
    manifest[rel] = {"file": name, "base": sha(base) if base is not None else None, "new": sha(new)}
json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
print("[DEBUG] staged", len(manifest), "files; pin", pin["sources"][0]["sha256"][:16])
