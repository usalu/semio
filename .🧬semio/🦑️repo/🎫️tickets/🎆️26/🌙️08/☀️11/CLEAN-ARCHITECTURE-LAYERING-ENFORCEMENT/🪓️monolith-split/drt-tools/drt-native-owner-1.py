import re,sys
p="✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts"
t=open(p,encoding="utf8").read()
o=t
imp='import { frameworkSourcePath } from "@semio-tech/framework/test/source-text";\n'
assert t.count(imp)==1
helpers=imp+'''import { readModuleTreeSource, sliceModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

const OWNER_ROOTS=["🦀️.rs","🏟️arena/🦀️.rs","🔏️digest/🦀️.rs"];

/** 📖️ The Rust text of the drawing host owner: its own root and the arena and digest crates it was carved into, each with the child modules it mounts. */
const ownerSource=(owner:string):string=>OWNER_ROOTS.map(root=>readModuleTreeSource(join(owner,root))).join("");

/** ✂️ One marker-delimited item of the drawing host owner, taken from the crate that holds `from`. */
const ownerSlice=(owner:string,from:string,to?:string):string=>{
  const root=OWNER_ROOTS.map(name=>join(owner,name)).find(path=>readModuleTreeSource(path).includes(from));
  if(root===undefined)throw Error("Owner marker missing: "+from);
  return sliceModuleTreeSource(root,from,to);
};
'''
t=t.replace(imp,helpers)
n1=len(re.findall(r'readFileSync\(join\(owner,\s*"🦀️\.rs"\),\s*"utf8"\)',t))
t=re.sub(r'readFileSync\(join\(owner,\s*"🦀️\.rs"\),\s*"utf8"\)','ownerSource(owner)',t)
pat=re.compile(r'(\w+)\.slice\(\1\.indexOf\(("[^"]*")\),\s*\1\.indexOf\(("[^"]*")\)\)')
t,n2=pat.subn(lambda m:'ownerSlice(owner,%s,%s)'%(m.group(2),m.group(3)),t)
old='host.slice(host.indexOf("fn observe_owned_string"),host.indexOf("fn observe(",host.indexOf("fn observe_owned_string")))'
assert t.count(old)==1
t=t.replace(old,'ownerSlice(owner,"fn observe_owned_string","fn observe(")')
print(n1,n2)
open(p,"w",encoding="utf8").write(t)
