#!/usr/bin/env python3
"""Builds a COPY of the complete type and test closure of the edited modules (process tree, schema validator, filesystem observation, ajv) at OUT_DIR, applies the staged set to it and writes a strict tsconfig. usage: fulloverlay.py OUT_DIR [--unpatched] (baseline without the staged set)"""
import json,os,shutil,subprocess,sys
skip=lambda directory,names:[name for name in names if os.path.islink(os.path.join(directory,name))]
HERE=os.path.dirname(os.path.abspath(__file__))
REPO="/Users/ueli/Documents/semio"
out=sys.argv[1]
M="🧰️framework/🔨️modules"
if os.path.isdir(out):shutil.rmtree(out)
os.makedirs(out+"/"+M+"/🧬️schema");os.makedirs(out+"/"+M+"/📁️filesystem")
shutil.copytree(REPO+"/"+M+"/🏃️process",out+"/"+M+"/🏃️process",ignore=skip)
shutil.copytree(REPO+"/"+M+"/🧬️schema/✅️validator",out+"/"+M+"/🧬️schema/✅️validator",ignore=skip)
shutil.copytree(REPO+"/"+M+"/📁️filesystem/🧾️observation",out+"/"+M+"/📁️filesystem/🧾️observation",ignore=skip)
siblings=os.path.dirname(os.path.realpath(REPO+"/node_modules/ajv"));os.makedirs(out+"/node_modules")
for name in ("ajv","fast-deep-equal","fast-uri","json-schema-traverse","require-from-string"):shutil.copytree(siblings+"/"+name,out+"/node_modules/"+name)
S=M+"/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
R=S+"/🌐️platform/🥾️boot/🩹️recovery"
files=[S+"/🟦️.ts",S+"/🧹️cleanup/🎛️command/🟦️.ts",S+"/🧱️fixed/🟦️.ts",S+"/🍎️image/🟦️.ts",S+"/🌱️birth/🧾️claim/🟦️.ts",R+"/🩺️heal/🟦️.ts",R+"/🧪️tests/🟦️.ts",R+"/🧪️tests/🩺️heal/🟦️.ts",R+"/🧪️tests/🔗️wiring/🟦️.ts",R+"/🧪️tests/💥️crash/🟦️.ts"]
json.dump({"compilerOptions":{"target":"ES2022","module":"ESNext","moduleResolution":"bundler","strict":True,"noEmit":True,"skipLibCheck":True,"allowImportingTsExtensions":True,"resolveJsonModule":True,"noUncheckedIndexedAccess":True,"types":["node","bun"],"typeRoots":[REPO+"/node_modules/@types"],"lib":["ES2023","DOM"]},"files":files},open(out+"/tsconfig.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
if "--unpatched" not in sys.argv:subprocess.run([sys.executable,HERE+"/apply.py","--root",out,"--write"],check=True)
print("full overlay",out)
