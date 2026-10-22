"""Generates the runtime-neutral command form of every tracked manifest: a command that names the JavaScript runtime (`bun ./x.ts args`, `bun --watch x.ts`, `bun --cwd <root> repo args`) becomes the launcher's script route (`repo ./x.ts args`, `repo --watch x.ts`, `repo args`). Dry run by default; --apply rewrites in place (textual patch, formatting preserved); --check exits 1 while a manifest command still names the runtime (the law).
Usage: manifest-neutral.py [--root DIR] [--pathspec PATH ...] [--apply] [--check] [--report FILE]
Only `command`, `commands` and `scripts` strings of tracked `package.json` and `📋️project.json` files outside test fixtures are touched. `bun --cwd D repo ...` is rewritten only when D resolves to the repository root (the launcher finds the root from any directory); other forms are reported, never guessed."""
import argparse, json, os, re, subprocess, sys
parser = argparse.ArgumentParser()
parser.add_argument("--root", default=os.getcwd()); parser.add_argument("--pathspec", action="append", default=[]); parser.add_argument("--apply", action="store_true"); parser.add_argument("--check", action="store_true"); parser.add_argument("--report")
args = parser.parse_args()
root = os.path.abspath(args.root)
tracked = subprocess.run(["git", "-C", root, "ls-files", "-z", "--", *(args.pathspec or ["."])], capture_output=True, check=True).stdout.decode("utf-8").split("\0")
names = {"package.json", "📋️project.json"}
files = [f for f in tracked if f and os.path.basename(f) in names and "node_modules" not in f and "🎫️tickets" not in f and "🧪️tests/🧫️fixtures" not in f]
SCRIPT = r'(?:"[^"\n]+\.(?:ts|mjs|js)"|[^\s"]+\.(?:ts|mjs|js))'
direct = re.compile(rf"^bun ((?:--watch )?{SCRIPT}(?: .*)?)$", re.S)
cwd = re.compile(r'^bun --cwd ("[^"\n]+"|\S+) repo(?: (.*))?$', re.S)
argv_repo = re.compile(r'(\[\s*)"bun",\s*"repo"')
argv_form = re.compile(r'(\[\s*)"bun"(,\s*"[^"\n]+\.(?:ts|mjs|js)")')
bare = re.compile(r"^bun repo(?: (.*))?$", re.S)
def rewrite(value, directory):
    m = bare.match(value)
    if m: return "repo" + (" " + m.group(1) if m.group(1) else "")
    m = direct.match(value)
    if m: return "repo " + m.group(1)
    m = cwd.match(value)
    if m:
        target = os.path.normpath(os.path.join(root, directory, m.group(1).strip('"')))
        if target == root: return "repo" + (" " + m.group(2) if m.group(2) else "")
    return None
changes, leftovers, manual = {}, [], []
def walk(node, directory, key, path, out):
    if isinstance(node, dict):
        for k, v in node.items(): walk(v, directory, k, path, out)
    elif isinstance(node, list):
        if key == "command" and len(node) > 1 and node[0] == "bun" and isinstance(node[1], str) and (node[1] == "repo" or re.search(r"\.(?:ts|mjs|js)$", node[1])): out["argv"] += 1; node = node[1:]
        for v in node: walk(v, directory, key, path, out)
    elif isinstance(node, str) and key in ("command", "commands", "scripts") or isinstance(node, str) and key is not None and key.startswith("scripts:"):
        if re.match(r"^bun( |$)", node):
            new = rewrite(node, directory)
            (out["changed"] if new else out["left"]).append((node, new))
def scripts_walk(node, directory, path, out):
    if isinstance(node, dict):
        for k, v in node.items():
            if k == "scripts" and isinstance(v, dict):
                for name, value in v.items(): walk(value, directory, "scripts", path, out)
            else: scripts_walk(v, directory, path, out)
    elif isinstance(node, list):
        for v in node: scripts_walk(v, directory, path, out)
total = 0
for rel in files:
    full = os.path.join(root, rel)
    try: raw = open(full, encoding="utf-8", newline="").read(); document = json.loads(raw)
    except Exception: continue
    out = {"changed": [], "left": [], "argv": 0}; directory = os.path.dirname(rel)
    walk(document, directory, None, rel, out)
    scripts_walk(document, directory, rel, out)
    seen, patched, failed = set(), raw, False
    unique = {}
    for old, new in out["changed"]: unique[old] = new
    for old, new in unique.items():
        expected = sum(1 for o, _ in out["changed"] if o == old)
        for ascii_only in (False, True):
            a, b = json.dumps(old, ensure_ascii=ascii_only), json.dumps(new, ensure_ascii=ascii_only)
            if patched.count(a) == expected: patched = patched.replace(a, b); break
        else: failed = True
    patched, repo_count = argv_repo.subn(r'\1"repo"', patched)
    patched, argv_count = argv_form.subn(r'\1"repo"\2', patched)
    argv_count += repo_count
    if argv_count != out["argv"]: failed = True
    for old, _ in out["left"]: leftovers.append((rel, old))
    if failed: manual.append(rel); continue
    if patched != raw:
        changes[rel] = len(out["changed"]) + argv_count; total += len(out["changed"]) + argv_count
        if args.apply: open(full, "w", encoding="utf-8", newline="").write(patched)
forms = {}
for rel, old in leftovers: forms.setdefault(re.sub(r'"[^"]*"|\S+\.(?:ts|mjs|js)', "P", old)[:70], []).append(rel)
summary = {"dryRun": not args.apply, "manifestsScanned": len(files), "manifestsChanged": len(changes), "stringsChanged": total, "needManualReview": manual, "leftoverForms": {k: len(v) for k, v in sorted(forms.items(), key=lambda kv: -len(kv[1]))[:40]}}
if args.report: json.dump({**summary, "changes": changes, "leftovers": leftovers}, open(args.report, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps(summary, ensure_ascii=False, indent=1))
if args.check and (changes or leftovers or manual): sys.exit(1)
