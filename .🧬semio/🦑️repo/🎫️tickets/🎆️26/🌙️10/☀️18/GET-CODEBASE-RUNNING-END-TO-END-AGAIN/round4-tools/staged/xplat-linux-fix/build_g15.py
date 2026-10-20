"""Cuts group g15 from the live test files: every hard requirement on python3, rg or chmod becomes the toolchain interface. Re-run before applying: the files are being edited by other rounds, an anchor that moved is refused by apply.py."""
import os, re, subprocess, sys, json
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import emit, live
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
TOOLCHAIN = "🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts"
OWN = ("🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/",)
pattern = r'Bun\.which\("python3"\)|spawnSync\("python3"|spawnSync\(\["python3"|spawn\("python3"|spawn\(\["python3"|Bun\.which\("rg"\)|spawnSync\("rg"|spawnSync\("chmod"'
listing = subprocess.run(["git", "grep", "-l", "-E", pattern, "--", "🧰️framework", "🎓️teaching", ":!*🎫️tickets*", ":!*.py", ":!*.md", ":!*.go", ":!*.rs"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
files = sorted(f for f in listing if f and not f.startswith(OWN))
replacements = [
 ('Bun.which("python3")', 'testPythonV1()', "testPythonV1"),
 ('spawnSync("python3"', 'spawnSync(testPythonV1()??"python3"', "testPythonV1"),
 ('spawnSync(["python3"', 'spawnSync([testPythonV1()??"python3"', "testPythonV1"),
 ('spawn("python3"', 'spawn(testPythonV1()??"python3"', "testPythonV1"),
 ('spawn(["python3"', 'spawn([testPythonV1()??"python3"', "testPythonV1"),
 ('Bun.which("rg")', 'testRipgrepV1()', "testRipgrepV1"),
 ('spawnSync("rg"', 'spawnSync(testRipgrepV1()??"rg"', "testRipgrepV1"),
]
edits, skipped, migrated = [], [], 0
for path in files:
    text = live(path)
    names, local = set(), []
    for old, new, name in replacements:
        count = text.count(old)
        if count:
            local.append((old, new, count)); names.add(name)
    for match in sorted(set(re.findall(r'spawnSync\("chmod",\["-R","u\+w",[^\]]+\]\)', text))):
        argument = re.search(r'"u\+w",(.+)\]\)$', match).group(1)
        local.append((match, "makeTestTreeWritableV1(" + argument + ")", text.count(match))); names.add("makeTestTreeWritableV1")
    if not local:
        continue
    imports = [line for line in text.split("\n") if re.match(r'^import\b.*\bfrom\s*["\'][^"\']+["\'];?\s*$', line) and text.count(line) == 1]
    if not imports:
        skipped.append((path, "no unique single-line import to anchor the new import on")); continue
    relative = os.path.relpath(os.path.join(ROOT, TOOLCHAIN), os.path.dirname(os.path.join(ROOT, path)))
    spec = relative if relative.startswith(".") else "./" + relative
    anchor = imports[0]
    semicolon = ";" if anchor.rstrip().endswith(";") else ""
    edits.append((path, anchor, anchor + "\nimport {" + ",".join(sorted(names)) + '} from "' + spec + '"' + semicolon))
    for old, new, count in local:
        edits.append((path, old, new, count))
    migrated += 1
emit("g15", "tests name their host tools through the toolchain interface: Python 3 (python3, python, py -3), ripgrep and a writable tree come from one module that reports a missing tool once in English or German instead of skipping silently or failing on Windows", [TOOLCHAIN], edits, [], os.path.join(HERE, "groups"))
json.dump({"migrated": migrated, "skipped": skipped, "files": len(files)}, open(os.path.join(HERE, "g15-report.json"), "w"), ensure_ascii=False, indent=1)
print("g15:", migrated, "files migrated,", len(skipped), "skipped,", len(edits), "edits")
