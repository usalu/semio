#!/usr/bin/env python3
"""🏭️ Builds one staged batch from the CURRENT live files: python3 -I make_batch.py <batch> [--root DIR].
A batch is batches/<batch>.json {title, files:[rel], manual:[{rel, old, new, count?}], emoji:[rel], verify:[bun test paths relative to the root]}.
Order per file: live bytes -> manual edits (exact old strings, exact counts) -> emoji pass (cp-emoji.ts) -> staged/<batch>/{files,base,manifest.json,stage.py,verify.sh}. Re-run it to rebase on whatever the live tree holds now."""
import hashlib, json, os, re, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
def sha(data): return hashlib.sha256(data).hexdigest()
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
args = [a for a in sys.argv[1:] if not a.startswith("--")]
batch = args[0]
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
if "--root" in sys.argv: args = [a for a in args if a != root]
spec = json.load(open(os.path.join(HERE, "batches", batch + ".json"), encoding="utf8"))
out = os.path.join(HERE, "staged", batch)
work = os.path.join(C, "🗑️generated", "cp", "work", batch)
shutil.rmtree(work, ignore_errors=True); shutil.rmtree(out, ignore_errors=True)
for d in (work, os.path.join(out, "files"), os.path.join(out, "base")): os.makedirs(d)
rels = list(dict.fromkeys(spec["files"]))
manual = {}
for m in spec.get("manual", []): manual.setdefault(m["rel"], []).append(m)
emoji = {r for r in rels if r.endswith((".ts", ".mts", ".mjs")) and not r.endswith(".d.ts")} | set(spec.get("emoji", []))
names = {}
for index, rel in enumerate(rels):
    live = os.path.join(root, rel)
    data = open(live, "rb").read()
    ext = os.path.basename(rel).split(".", 1)[1] if "." in os.path.basename(rel) else ""
    name = "%03d.%s" % (index, ext.split(".")[-1] if ext else "txt")
    names[rel] = name
    open(os.path.join(out, "base", name), "wb").write(data)
    text = data.decode("utf8")
    for m in manual.get(rel, []):
        if "add_import" in m:
            line = m["add_import"].replace("{FROM}", os.path.relpath(os.path.join(root, m["from_rel"]), os.path.dirname(os.path.join(root, rel))))
            if not line.startswith("import ") or ("from \"." not in line): sys.exit("bad import line")
            if not os.path.dirname(rel) == "" and not line.split('from "')[1].startswith("."): sys.exit("import path not relative")
            have = set()
            for mm in re.finditer(r'import\s*(?:type\s*)?\{([^}]*)\}\s*from\s*["\'][^"\']+["\']', text):
                have.update(x.strip().split(" as ")[-1].strip() for x in mm.group(1).split(",") if x.strip())
            wanted = re.match(r'import\s*\{([^}]*)\}', line).group(1).split(",")
            need = [x.strip() for x in wanted if x.strip() and x.strip() not in have]
            if not need: continue
            line = line.replace("{" + ",".join(wanted) + "}", "{" + ",".join(need) + "}") if "{" + ",".join(wanted) + "}" in line else line
            if line in text: continue
            rows = text.split("\n")
            last = -1
            for i, row in enumerate(rows[:400]):
                if re.match(r'^(import\b.*|\}\s*from\b.*)$', row.strip() if False else row) and re.search(r'(from\s*["\'][^"\']+["\']|^import\s+["\'][^"\']+["\'])\s*(with\s*\{[^}]*\})?\s*;?\s*(//.*)?$', row): last = i
            if last < 0:
                last = -1
                while last + 1 < len(rows) and (rows[last + 1].lstrip().startswith(("/**", "//", "*", "/*")) or rows[last + 1].strip() == ""): last += 1
                if last >= 0 and rows[last].strip() == "": last -= 1
            rows.insert(last + 1, line)
            text = "\n".join(rows)
            continue
        if "json_add" in m:
            doc = json.loads(text)
            for table, rowsadd in m["json_add"].items():
                for key, leaf in rowsadd.items():
                    if key in doc[table] and doc[table][key] != leaf: sys.exit("catalog key %s exists with other text" % key)
                    doc[table][key] = leaf
            text = json.dumps(doc, indent=m.get("indent", 2), ensure_ascii=False) + "\n"
            continue
        count = text.count(m["old"])
        if count == 0 and m.get("optional"):
            print("[skip] anchor gone (a peer landed it): %s %r" % (rel[-50:], m["old"][:70]), file=sys.stderr); continue
        if m.get("count") == "any" and count >= 1: m = dict(m, count=count)
        if count != m.get("count", 1): sys.exit("manual edit anchor count %s != %s in %s: %r" % (count, m.get("count", 1), rel[-60:], m["old"][:60]))
        text = text.replace(m["old"], m["new"])
    open(os.path.join(work, name), "w", encoding="utf8").write(text)
moves = spec.get("moves", [])
move_rows = []
for index, mv in enumerate(moves):
    live_from = os.path.join(root, mv["from"])
    data = open(live_from, "rb").read()
    mode = os.stat(live_from).st_mode & 0o7777
    text = data.decode("utf8")
    for m_ in manual.get(mv["to"], []):
        count = text.count(m_["old"])
        if count != m_.get("count", 1) and not (m_.get("count") == "any" and count >= 1): sys.exit("manual edit anchor count %d in moved %s: %r" % (count, mv["to"][-60:], m_["old"][:60]))
        text = text.replace(m_["old"], m_["new"])
    ext = os.path.basename(mv["to"]).rsplit(".", 1)[-1]
    nn = "mv%03d.%s" % (index, ext)
    open(os.path.join(out, "files", nn), "wb").write(text.encode("utf8"))
    open(os.path.join(out, "base", nn), "wb").write(data)
    move_rows.append((mv, nn, data, text.encode("utf8"), mode))
listing = os.path.join(work, "emoji.list")
open(listing, "w", encoding="utf8").write("".join("%s\t%s\n" % (os.path.join(work, names[r]), r) for r in rels if r in emoji))
report = os.path.join(work, "emoji.json")
if emoji:
    env = dict(os.environ, CP_ROOT=root)
    run = subprocess.run(["bun", os.path.join(HERE, "cp-emoji.ts"), listing, report], env=env, capture_output=True, text=True)
    if run.returncode != 0: sys.exit(run.stdout + run.stderr)
    failed = {k: v["failed"] for k, v in json.load(open(report, encoding="utf8")).items() if v["failed"]}
    if failed: sys.exit("emoji pass could not fix: %s" % json.dumps(failed, ensure_ascii=False))
allow = spec.get("allowlist")
if allow:
    LAW_ROOTS = ("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", "🧰️framework/🔨️modules/🏃️process/")
    SKIP = {"node_modules", "🧪️tests", "🧫️fixtures", ".git", "target", "dist", ".cache", "🗑️generated"}
    entries = [{"rel": r, "work": os.path.join(work, names[r])} for r in rels if r.endswith((".ts", ".mts", ".mjs", ".cjs")) and r.startswith(LAW_ROOTS) and not (set(r.split("/")) & SKIP)]
    live_allow = os.path.join(root, allow)
    alname = "allowlist.json"
    shutil.copyfile(live_allow, os.path.join(work, alname))
    names[allow] = alname
    json.dump({"allowlist": os.path.join(work, alname), "entries": entries}, open(os.path.join(work, "allow.spec.json"), "w"), ensure_ascii=False)
    shutil.copyfile(live_allow, os.path.join(out, "base", "allowlist.json"))
    rels.append(allow)
    run = subprocess.run(["bun", os.path.join(HERE, "cp-allow.ts"), os.path.join(work, "allow.spec.json")], capture_output=True, text=True, cwd=C)
    if run.returncode != 0: sys.exit(run.stdout + run.stderr)
    print(run.stderr.strip().splitlines()[-1])
manifest = {}
changed = []
for rel in rels:
    name = names[rel]
    new = open(os.path.join(work, name), "rb").read()
    base = open(os.path.join(out, "base", name), "rb").read()
    if new == base:
        os.remove(os.path.join(out, "base", name)); continue
    shutil.copyfile(os.path.join(work, name), os.path.join(out, "files", name))
    manifest[rel] = {"file": name, "base": sha(base), "new": sha(new)}
    changed.append(rel)
for mv, nn, old_bytes, new_bytes, mode in move_rows:
    manifest[mv["to"]] = {"file": nn, "base": None, "new": sha(new_bytes), "mode": mode}
    manifest[mv["from"]] = {"file": nn, "base": sha(old_bytes), "new": None, "mode": mode}
    changed.append(mv["to"])
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
template = open(os.path.join(HERE, "stage.template.py"), encoding="utf8").read()
title = spec["title"]
open_at = template.index('"""') + 3
close_at = template.index('"""', open_at)
template = template[:open_at] + title + "\nusage: python3 -I stage.py check|apply|revert [--root DIR]   (default root: walk up from this file to the directory holding 🧰️framework)\ncheck : exit 0 when every file is `base` or `applied`, 1 otherwise.\napply : every live file must hold its base hash or already the new one, else exit 2 (drift); atomic replace that keeps the mode of the file it replaces.\nrevert: restores base bytes." + template[close_at:]
open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(template)
os.chmod(os.path.join(out, "stage.py"), 0o755)
scan = "\n".join(r for r in changed if not r.endswith(".d.mts") and r.endswith((".ts", ".mts", ".mjs")))
tests = " ".join('"%s"' % t for t in spec.get("verify", []))
verify = f'''#!/bin/sh
# {title}
# usage: VERIFY_ROOT=<repository or mirror root> sh verify.sh   (default: the root above this batch)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${{VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}}
tools=$(cd "$here/../.." && pwd)
c=$(cd "$tools/.." && pwd)
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$tools/verify_batch.py" "$here" "$root"
'''
if tests: verify += f'bun test {tests}\n'
open(os.path.join(out, "verify.sh"), "w", encoding="utf8").write(verify)
os.chmod(os.path.join(out, "verify.sh"), 0o755)
print(batch, "changed", len(changed), "of", len(rels), "->", out)
