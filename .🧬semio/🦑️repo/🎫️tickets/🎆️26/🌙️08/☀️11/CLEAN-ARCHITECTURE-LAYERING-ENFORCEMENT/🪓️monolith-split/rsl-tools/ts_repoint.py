"""📌️ Re-points a store-source law from the store facade root to the store source text: every `readFileSync(join(root,"…/🏪️store/🦀️.rs"),"utf8")` (and the `Bun.file`/`new URL` forms given by --form) becomes `readStoreSourceText()`, and the import of `@semio-tech/framework/test/source-text` gains the reader. usage: ts_repoint.py <file.ts> <anchor-count> dry|write"""
import os, re, shutil, sys
os.chdir("/Users/ueli/Documents/semio")
PRE="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/rsl-pre/"
f=sys.argv[1]; n=int(sys.argv[2]); write=sys.argv[3]=="write"
t=open(f,encoding="utf8").read(); o=t
rx=re.compile(r'readFileSync\(\s*join\(\s*\w+\s*,\s*"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️\.rs"\s*\)\s*,\s*"utf8"\s*\)')
found=rx.findall(t)
assert len(found)==n,(f[-50:],len(found),n)
t=rx.sub("readStoreSourceText()",t)
imp='import { readStoreSourceText } from "@semio-tech/framework/test/source-text";\n'
m=re.search(r'import \{([^}]*)\} from "@semio-tech/framework/test/source-text";\n',t)
if m:
    names=[x.strip() for x in m.group(1).split(",") if x.strip()]
    if "readStoreSourceText" not in names:
        t=t.replace(m.group(0),'import { '+", ".join(names+["readStoreSourceText"])+' } from "@semio-tech/framework/test/source-text";\n')
else:
    # after the last top-level import line
    last=[x for x in re.finditer(r'(?m)^import [^\n]*;\n',t)][-1]
    t=t[:last.end()]+imp+t[last.end():]
print(f[-50:],"replacements",len(found))
if write:
    d=PRE+f
    if not os.path.exists(d): os.makedirs(os.path.dirname(d),exist_ok=True); shutil.copy2(f,d)
    assert open(f,encoding="utf8").read()==o
    open(f,"w",encoding="utf8").write(t)
