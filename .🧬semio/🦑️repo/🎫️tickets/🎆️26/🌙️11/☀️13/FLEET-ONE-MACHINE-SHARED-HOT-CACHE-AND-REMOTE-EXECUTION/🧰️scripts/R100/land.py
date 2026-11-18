#!/usr/bin/env python3
"""R100 landing: backs up the live script-inputs module, applies the anchored edits of mk.py to the live bytes (every anchor must occur exactly once), replaces the file atomically and adds the tool-boundary test. usage: land.py [--dry]"""
import os,sys,subprocess,shutil,hashlib
R="/Users/ueli/Documents/semio"
LIB=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
SI=LIB+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
TB=os.path.dirname(SI)+"/🔌️provider-closures/🧪️tests/🛑️tool-boundary"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R100"
D=os.path.dirname(os.path.abspath(__file__))
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
dry="--dry" in sys.argv
st=SC+"/stage-land";shutil.rmtree(st,ignore_errors=True)
subprocess.run([sys.executable,D+"/mk.py",st],check=True)
if dry:print("dry ok",sha(SI),"->",hashlib.sha256(open(st+"/new.mjs","rb").read()).hexdigest()[:12]);sys.exit(0)
assert not os.path.exists(TB),"test directory exists"
os.makedirs(SC+"/backup",exist_ok=True);shutil.copy(SI,SC+"/backup/si.mjs")
before=sha(SI)
tmp=SI+".r100.tmp";shutil.copy(st+"/new.mjs",tmp);os.replace(tmp,SI)
os.makedirs(TB+"/🐍️oracle");shutil.copy(SC+"/test/tool-boundary.ts",TB+"/🟦️.ts");shutil.copy(SC+"/test/oracle/oracle.py",TB+"/🐍️oracle/🐍️.py")
open(SC+"/backup/after.sha","w").write(sha(SI))
print("landed",before,"->",sha(SI),"test",sha(TB+"/🟦️.ts"))
