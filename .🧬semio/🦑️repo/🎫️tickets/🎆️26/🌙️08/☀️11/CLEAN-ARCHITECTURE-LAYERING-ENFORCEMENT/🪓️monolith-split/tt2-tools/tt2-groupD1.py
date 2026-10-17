import re,os,time,shutil,json
from tt2_edit import *
T=ROOT+'✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts'
assert time.time()-os.stat(T).st_mtime>3600
os.makedirs(SNAP,exist_ok=True); snap=SNAP+'native-owner.pre'; shutil.copy(T,snap)
s=open(T,encoding='utf8').read()
assert s.count('applyPatch(')==30
s=s.replace('applyPatch(','patched(')
s,k=re.subn(r',\s*true,\s*false\)\.newDocument',')',s); assert k==30
old='import {applyPatch} from "fast-json-patch";\n'
assert s.count(old)==1
s=s.replace(old,'import {applyPatch,type Operation} from "fast-json-patch";\n')
old='import { frameworkSourcePath } from "@semio-tech/framework/test/source-text";\n'
assert s.count(old)==1
s=s.replace(old,old+'\nconst patched=(document:Record<string,any>,operations:Operation[]):Record<string,any>=>applyPatch(document,operations,true,false).newDocument;\n')
open(T,'w',encoding='utf8').write(s)
st=json.load(open(STATE,encoding='utf8')); st[T]=snap; json.dump(st,open(STATE,'w',encoding='utf8'),ensure_ascii=False,indent=1)
print('ok')
