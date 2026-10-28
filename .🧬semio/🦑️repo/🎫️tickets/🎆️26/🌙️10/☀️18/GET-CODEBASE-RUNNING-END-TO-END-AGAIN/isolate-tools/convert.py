"""🧫 [isolate] converts one batch of test files from <src> (the live tree) into <dst> (a mirror or a staging tree): reads of the granted immutable Nx toolkit (`join(<checkout>,".cache/b/t/m/nx",id…`) go through `testNxToolkitDirectoryV1(id)`, then the AST codemod (`codemod.ts`) turns every store-only checkout reference into `repository.root`, then the batch's anchored manual edits (`edits/<batch>.py`: MANUAL = {rel: [(old, new, count)]}, applied to the codemod output; SPLIT = files whose mixed checkout identifiers keep the checkout except in store uses; WHOLE = files whose mixed checkout identifiers become the private repository; HELPERS = extra helper imports; exit 2 naming a lost anchor) and finally the shared private repository is imported and opened once per file that uses it. usage: convert.py <batch> <src> <dst> [--only REL]"""
import importlib.util, json, os, re, shutil, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from conv import OPEN, first_statement, last_import_end, relpath
batch, src, dst = sys.argv[1:4]
only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else None
files = [l.strip() for l in open(os.path.join(HERE, "batches", batch + ".txt"), encoding="utf-8") if l.strip()]
if only: files = [f for f in files if f == only]
spec = os.path.join(HERE, "edits", batch + ".py"); manual = {}; skip_codemod = set(); helpers = {}; split = set(); whole = set()
if os.path.exists(spec):
    module = importlib.util.spec_from_file_location("edits_" + batch, spec); loaded = importlib.util.module_from_spec(module); module.loader.exec_module(loaded)
    manual = getattr(loaded, "MANUAL", {}); skip_codemod = set(getattr(loaded, "NO_CODEMOD", ())); helpers = getattr(loaded, "HELPERS", {}); split = set(getattr(loaded, "SPLIT", ())); whole = set(getattr(loaded, "WHOLE", ()))
SDK = re.compile(r'join\(\s*(process\.cwd\(\)|[A-Za-z_$][\w$]*)\s*,\s*"\.cache/b/t/m/nx"\s*,\s*([A-Za-z_$][\w$.]*)')
def toolkit(text):
    return SDK.sub(lambda m: "join(testNxToolkitDirectoryV1(%s)" % m.group(2) if m.group(1) == "process.cwd()" or re.search(r"\b%s=process\.cwd\(\)" % re.escape(m.group(1)), text) else m.group(0), text)
staging = tempfile.mkdtemp(prefix="convert-", dir=os.path.join(os.path.dirname(HERE), "🗑️generated", "isolate"))
for rel in files:
    os.makedirs(os.path.dirname(os.path.join(staging, rel)), exist_ok=True)
    open(os.path.join(staging, rel), "w", encoding="utf-8").write(toolkit(open(os.path.join(src, rel), encoding="utf-8").read()))
plan = json.loads(subprocess.run(["bun", os.path.join(HERE, "codemod.ts"), staging, *files], capture_output=True, text=True, check=True, env=dict(os.environ, LAW_ROOT=os.environ.get("LAW_ROOT", "/private/tmp/m/is1"), CODEMOD_SPLIT="\n".join(sorted(split)), CODEMOD_WHOLE="\n".join(sorted(whole)))).stdout)
for row in plan:
    rel = row["file"]; text = open(os.path.join(staging, rel), encoding="utf-8").read()
    if row.get("out") and rel not in skip_codemod: text = row["out"]
    for old, new, count in manual.get(rel, []):
        found = text.count(old)
        if found != count: sys.stderr.write("ANCHOR %s: %d != %d: %s\n" % (rel[-80:], found, count, old[:140])); sys.exit(2)
        text = text.replace(old, new)
    names = list(helpers.get(rel, ())) + [name for name, used in (("privateRepositoryWorkspaceV1", "repository.root" in text), ("testNxToolkitDirectoryV1", "testNxToolkitDirectoryV1(" in text), ("testCheckoutRootV1", "testCheckoutRootV1()" in text), ("testToolchainDirectoryV1", "testToolchainDirectoryV1()" in text)) if used and name not in helpers.get(rel, ())]
    if names and "from\"%s\"" % relpath(rel) not in text:
        end = last_import_end(text); text = text[:end] + "\nimport{%s}from\"%s\";" % (",".join(names), relpath(rel)) + text[end:]
        if "privateRepositoryWorkspaceV1" in names and OPEN not in text:
            end = last_import_end(text); start = first_statement(text, end)
            text = text[:end] + "\n" + OPEN + text[end:] if start >= end else text[:start] + OPEN + "\n" + text[start:]
    target = os.path.join(dst, rel); os.makedirs(os.path.dirname(target), exist_ok=True)
    original = open(target, encoding="utf-8").read() if os.path.exists(target) else None
    if original != text:
        with open(target, "w", encoding="utf-8") as f: f.write(text)
    print("%-9s r%-3d m%-2d %s" % ("changed" if original != text else "same", row["replaced"], len(manual.get(rel, [])), rel[-95:]))
shutil.rmtree(staging)
