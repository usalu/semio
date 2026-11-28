import sys
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs"
s=open(p,encoding="utf8").read()
bad='if(before.isFile()&&!before.isSymbolicLink()&&before.nlink!==1n&&before.size>=0n&&before.size<=BigInt(maximumBytes))throw concurrentEditV1("identity",path,"Source revision is being linked or replaced");\n '
s=s.replace(bad,"")
a=' const before=lstatSync(path,{bigint:true});\n if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n'
b=' let before=lstatSync(path,{bigint:true});for(let again=0;again<3&&before.isFile()&&!before.isSymbolicLink()&&before.nlink!==1n;again++){Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,25);before=lstatSync(path,{bigint:true});}\n if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n'
if s.count(a)!=1: sys.exit("anchor %d"%s.count(a))
open(p,"w",encoding="utf8").write(s.replace(a,b));print("ok")
