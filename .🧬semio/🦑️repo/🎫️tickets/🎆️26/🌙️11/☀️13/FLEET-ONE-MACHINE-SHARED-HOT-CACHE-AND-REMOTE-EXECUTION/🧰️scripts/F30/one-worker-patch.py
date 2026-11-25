import pathlib, shutil, sys, hashlib
F = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm")
MAIN = F / "🟨️.mjs"
TEST = F / "🧪️tests" / "🪜️one-worker" / "🟦️.ts"
HERE = pathlib.Path(__file__).parent
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()[:12]
text = MAIN.read_text(encoding="utf8")
a = "minimumWorkers: lean.lean ? 1 : 2 });"
b = "if (lease.workers < (lean.lean ? 1 : 2)) {"
if a in text and b in text:
    before = sha(MAIN)
    MAIN.write_text(text.replace(a, "minimumWorkers: 1 });").replace(b, "if (lease.workers < 1) {"), encoding="utf8")
    print(MAIN.name, before, "->", sha(MAIN))
elif "minimumWorkers: 1 });" in text: print("already applied")
else: sys.exit("anchors missing")
TEST.parent.mkdir(exist_ok=True)
shutil.copyfile(HERE / "one-worker-test.ts", TEST)
print(TEST, sha(TEST))
