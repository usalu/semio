"""🩹 patch-tree-lanes.py: one-off edit of the edits.json/tree batch tooling of gram2-g, gram3-g, walk-g (and graphlat-g with --graphlat) so their new-file `tree/` is a flat tree (manifest.json + f<NNN>); asserts every anchor exactly once. usage: python3 -I patch-tree-lanes.py [--graphlat]"""
import os, sys
C = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))


def edit(rel, pairs):
    p = os.path.join(C, rel)
    s = open(p, encoding="utf8").read()
    for a, b in pairs:
        assert s.count(a) == 1, (rel, a[:70], s.count(a))
        s = s.replace(a, b)
    open(p, "w", encoding="utf8").write(s)


PY_LISTING = '''def listing():
    found = []
    for base, dirs, names in os.walk(TREE):
        dirs[:] = [d for d in dirs if not os.path.islink(os.path.join(base, d))]
        found += [os.path.relpath(os.path.join(base, n), TREE) for n in names if not os.path.islink(os.path.join(base, n))]
    return sorted(found)


'''
PY_STORED = '''FLAT = json.load(open(os.path.join(TREE, "manifest.json"), encoding="utf8")) if os.path.isdir(TREE) else {}


def stored(name):
    return os.path.join(TREE, FLAT[name]["file"])


'''
GENERIC = [
    ('import hashlib, json, os, shutil, sys\nHERE', 'import hashlib, json, os, shutil, sys\nHERE'),
    ('TREE = os.path.join(HERE, "tree")\n', 'TREE = os.path.join(HERE, "tree")\n'),
]


def batch_py(rel, generic):
    pairs = [(PY_LISTING, PY_STORED)]
    pairs.append(('NEW_FILES = listing() if os.path.isdir(TREE) else []' if generic else 'NEW_FILES = listing()', 'NEW_FILES = sorted(FLAT)'))
    p = os.path.join(C, rel)
    s = open(p, encoding="utf8").read()
    n = s.count("os.path.join(TREE, name)")
    assert n >= 3, (rel, n)
    s = s.replace("os.path.join(TREE, name)", "stored(name)")
    s = s.replace("copied from tree/", "copied from the flat tree/ (manifest.json maps the repository path to f<NNN>)").replace("new files from tree/", "new files from the flat tree/ (manifest.json maps the repository path to f<NNN>)")
    open(p, "w", encoding="utf8").write(s)
    edit(rel, pairs)


TS_FN = ('const flatTree=(dir:string)=>{const flat=join(dir,"tree"),raw=readFileSync(join(flat,"manifest.json")),root=join(tmpdir(),"flat-trees",basename(dirname(dirname(dir))),createHash("sha1").update(raw).digest("hex").slice(0,8),"staged",basename(dir)),out=join(root,"tree");'
         'if(!existsSync(join(root,".expanded"))){rmSync(root,{recursive:true,force:true});for(const[rel,row]of Object.entries(JSON.parse(raw.toString("utf8"))as Record<string,{file:string}>)){mkdirSync(dirname(join(out,rel)),{recursive:true});copyFileSync(join(flat,row.file),join(out,rel));}writeFileSync(join(root,".expanded"),"");}return out;};\n')


def preload(rel, has_write=False, arrow=True):
    s = open(os.path.join(C, rel), encoding="utf8").read()
    fs_old = 'import{existsSync,readFileSync,writeFileSync}from"node:fs";' if has_write else 'import{existsSync,readFileSync}from"node:fs";'
    pairs = [
        (fs_old, 'import{copyFileSync,existsSync,mkdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";import{createHash}from"node:crypto";import{tmpdir}from"node:os";'),
        ('import{dirname,join,resolve}from"node:path";', 'import{basename,dirname,join,resolve}from"node:path";'),
        ('const dirs=(process.env.BATCH_DIRS', '/** 🧊 A flat staged tree (manifest.json + f<NNN>) expanded once to a short scratch directory laid out like the repository (.../staged/<batch>/tree); flatten-g-tools/flat.py `scratch` computes the same path. */\n' + TS_FN + 'const dirs=(process.env.BATCH_DIRS'),
        ('trees:string[]=dirs.map(dir=>join(dir,"tree"))', 'trees:string[]=dirs.map(flatTree)'),
    ]
    edit(rel, pairs)


G2, W, G3 = "gram2-g-tools/staged", "walk-g-tools/staged", "gram3-g-tools"
batch_py(G2 + "/batch_generic.py", True)
batch_py(G2 + "/row-table/batch.py", False)
batch_py(W + "/batch_generic.py", True)
preload(G2 + "/preload_generic.ts")
preload(W + "/preload_generic.ts")
preload(G3 + "/preload_tree.ts")
preload(G3 + "/count/preload.ts", has_write=True)
edit(G2 + "/row-table/preload.ts", [
    ('import{readFileSync}from"node:fs";', 'import{copyFileSync,existsSync,mkdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";import{createHash}from"node:crypto";import{tmpdir}from"node:os";'),
    ('import{dirname,join}from"node:path";', 'import{basename,dirname,join}from"node:path";'),
    ('const here=dirname(fileURLToPath(import.meta.url)),', TS_FN + 'const here=dirname(fileURLToPath(import.meta.url)),'),
    ('leaf=join(here,"tree","', 'leaf=join(flatTree(here),"'),
])
edit(G3 + "/law.py", [
    ('import os, re, shutil, subprocess, sys', 'import json, os, re, shutil, subprocess, sys'),
    ('    shutil.copytree(os.path.join(batch, "tree"), scratch, dirs_exist_ok=True, symlinks=True)\n',
     '    flat = os.path.join(batch, "tree")\n    for rel, row in json.load(open(os.path.join(flat, "manifest.json"), encoding="utf8")).items():\n        os.makedirs(os.path.dirname(os.path.join(scratch, rel)), exist_ok=True)\n        shutil.copyfile(os.path.join(flat, row["file"]), os.path.join(scratch, rel))\n'),
])
print("patched")
