#!/usr/bin/env python3
"""mirror.py <dest>: real copies (no links) of the import closure of the unit-cache, fleet and offload code and tests, plus ajv; baseline.json records the sha256 of every copied file."""
import os,re,sys,shutil,json,hashlib
LIVE="/Users/Shared/semio";DEST=sys.argv[1]
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";U=L+"/⚡️caching/🦀️cargo/🗃️unit-cache"
entries=[U+"/📜️script.ts",U+"/🟦️.ts",U+"/🏗️buildscript/🟦️.ts",L+"/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts"]
for d,_,fs in os.walk(LIVE+"/"+U):
    if "/node_modules" in d:continue
    for f in fs:
        if f.endswith(".ts") and "🧪️tests" in d:entries.append(os.path.relpath(os.path.join(d,f),LIVE))
Q="[\"'`]"
PAT=[re.compile(p,re.M) for p in (r'from\s*'+Q+r'([^"\'`$]+)'+Q,r'import\(\s*'+Q+r'([^"\'`$]+)'+Q+r'\s*\)',r'^\s*import\s*'+Q+r'([^"\'`$]+)'+Q,r'new URL\(\s*'+Q+r'([^"\'`$]+)'+Q+r'\s*,\s*import\.meta\.url',r'(?:resolve|join)\(\s*(?:here|import\.meta\.dir)\s*,\s*'+Q+r'([^"\'`$]+)'+Q)]
seen=set();todo=list(entries)
while todo:
    rel=todo.pop()
    if rel in seen:continue
    p=LIVE+"/"+rel
    if not os.path.isfile(p):continue
    seen.add(rel)
    if not rel.endswith((".ts",".mjs",".mts",".js")):continue
    text=open(p,encoding="utf8",errors="replace").read()
    for pat in PAT:
        for m in pat.finditer(text if pat.pattern.startswith("^") is False else text):
            spec=m.group(1)
            if not spec.startswith("."):continue
            t=os.path.normpath(os.path.join(os.path.dirname(p),spec))
            if os.path.isdir(t):continue
            if not t.startswith(LIVE+"/"):continue
            todo.append(os.path.relpath(t,LIVE))
for rel in list(seen):
    parts=rel.split("/")
    if "🧪️tests" in parts:
        top="/".join(parts[:parts.index("🧪️tests")+1])
        for d,_,fs in os.walk(LIVE+"/"+top):
            if "/node_modules" in d:continue
            for f in fs:
                if not f.endswith((".log",".tmp")):seen.add(os.path.relpath(os.path.join(d,f),LIVE))
    elif rel.endswith("🔣️.json") or rel.endswith("🐍️.py"):
        pass
seen.add("rust-toolchain.toml")
base={};total=0
for rel in sorted(seen):
    src=LIVE+"/"+rel;dst=DEST+"/"+rel;os.makedirs(os.path.dirname(dst),exist_ok=True)
    shutil.copyfile(src,dst);shutil.copymode(src,dst);b=open(src,"rb").read();base[rel]=hashlib.sha256(b).hexdigest();total+=len(b)
for pkg in ("ajv","fast-deep-equal","json-schema-traverse","require-from-string","fast-uri"):
    s=LIVE+"/node_modules/"+pkg
    if os.path.isdir(s):shutil.copytree(s,DEST+"/node_modules/"+pkg,dirs_exist_ok=True,symlinks=False)
json.dump(base,open(DEST+"/../baseline.json","w"))
print(len(base),"files",total//1024,"KiB")
