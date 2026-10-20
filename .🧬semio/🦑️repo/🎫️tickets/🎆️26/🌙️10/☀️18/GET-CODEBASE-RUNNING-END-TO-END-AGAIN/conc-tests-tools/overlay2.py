#!/usr/bin/env python3
"""Builds /private/tmp/conc-tests/ov2: a copy of the process module, the library genuine-birth/test-output modules and the edited test files, then applies the staged set to the copy. usage: overlay2.py"""
import os,shutil,subprocess,sys
HERE=os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0,HERE)
import manifest as m, json
OV="/private/tmp/conc-tests/ov2"; REAL="/Users/ueli/Documents/semio"
shutil.rmtree(OV,ignore_errors=True); shutil.rmtree(OV+".backup",ignore_errors=True)
def cp(rel):
    src=REAL+"/"+rel; dst=OV+"/"+rel
    if os.path.isdir(src): shutil.copytree(src,dst,symlinks=False,ignore=shutil.ignore_patterns("node_modules"))
    else: os.makedirs(os.path.dirname(dst),exist_ok=True); shutil.copyfile(src,dst)
cp(m.FW+"/"+m.MODS+"/"+m.PROC)
cp(m.FW+"/"+m.MODS+"/"+m.e(0x1F9EC,"schema")+"/"+m.e(0x2705,"validator"))
cp(m.FW+"/"+m.MODS+"/"+m.e(0x1F4C1,"filesystem"))
open(OV+"/tsconfig.json","w").write(json.dumps({"compilerOptions":{"baseUrl":".","paths":{"@semio-tech/schema-validator-ts":["./"+m.FW+"/"+m.MODS+"/"+m.e(0x1F9EC,"schema")+"/"+m.e(0x2705,"validator")+"/"+m.e(0x1F4E6,"packages")+"/"+m.e(0x1F7E6,"typescript")+"/"+m.IDX]}}},ensure_ascii=False))
LIB=m.FW+"/"+m.e(0x1F6CD,"products")+"/"+m.e(0x1F991,"repo")+"/"+m.MODS+"/"+m.e(0x1F4DA,"library")
cp(LIB+"/"+m.PROC+"/"+m.e(0x1F33F,"environment"))
cp(LIB+"/"+m.TESTING+"/"+m.e(0x1F331,"genuine-birth"))
for e in json.load(open(HERE+"/edits.json",encoding="utf8")):
    if not os.path.exists(OV+"/"+e["target"]): cp(e["target"])
nm=OV+"/node_modules"; store=os.path.realpath(REAL+"/node_modules")+"/.bun"
for pkg in ["ajv","fast-deep-equal","json-schema-traverse","require-from-string","fast-uri","esbuild","@esbuild/darwin-arm64"]:
    dst=nm+"/"+pkg
    if os.path.isdir(dst): continue
    cand=[d for d in os.listdir(store) if d.startswith(pkg.replace("/","+")+"@")]
    if not cand: print("missing",pkg); continue
    src=store+"/"+sorted(cand)[-1]+"/node_modules/"+pkg
    os.makedirs(os.path.dirname(dst),exist_ok=True); shutil.copytree(src,dst,symlinks=False)
for pkg,store_name in [("typescript","typescript@5.9.3"),("@types/bun","@types+bun@1.4.3"),("bun-types","bun-types@1.4.3"),("@types/node","@types+node@24.19.2")]:
    dst=nm+"/"+pkg
    if not os.path.isdir(dst): os.makedirs(os.path.dirname(dst),exist_ok=True); shutil.copytree(store+"/"+store_name+"/node_modules/"+pkg,dst,symlinks=False)
if not os.path.isdir(nm+"/undici-types"): shutil.copytree(store+"/bun-types@1.4.3/node_modules/undici-types",nm+"/undici-types",symlinks=False)
open(OV+"/tsconfig.check.json","w").write(json.dumps({"compilerOptions":{"target":"ES2022","module":"ESNext","moduleResolution":"bundler","strict":True,"noUncheckedIndexedAccess":True,"noEmit":True,"skipLibCheck":True,"resolveJsonModule":True,"allowImportingTsExtensions":True,"types":["bun","node"],"lib":["ES2023"]},"include":["./"+m.SB_DIR+"/**/*.ts","./"+m.FW+"/"+m.MODS+"/"+m.PROC+"/"+m.TESTING+"/"+m.e(0x1F39B,"execution")+"/"+m.IDX],"exclude":["**/node_modules"]},ensure_ascii=False))
open(OV+"/package.json","w").write('{"name":"ov2","private":true}\n')
r=subprocess.run([sys.executable,HERE+"/apply.py","--root",OV,"--write"],capture_output=True,text=True); print(r.stdout[-600:],r.stderr[-400:])
