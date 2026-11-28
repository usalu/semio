import os,sys
ROOT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
OUT=sys.argv[1];rel="⚡️caching/🦀️cargo/📦️units/📤️receipt/🟦️.ts"
t=open(os.path.join(ROOT,rel),encoding="utf8").read()
E='throw Error("Rust unit output inventory differs from exact declared artifacts")'
assert t.count(E)==3
a='if(row.isDirectory()){if(!directories.delete(selected))'+E
b='else if(row.isFile()){if(!files.has(selected)||observed.has(selected))'+E
c='if(observed.size!==files.size||[...files].some(name=>!observed.has(name)))'+E
for x in (a,b,c):assert t.count(x)==1,x
t=t.replace(a,'if(row.isDirectory()){if(!directories.delete(selected))throw Error("Rust unit output inventory differs from exact declared artifacts: undeclared directory "+selected)')
t=t.replace(b,'else if(row.isFile()){if(!files.has(selected)||observed.has(selected))throw Error("Rust unit output inventory differs from exact declared artifacts: "+(files.has(selected)?"duplicate":"undeclared")+" file "+selected)')
t=t.replace(c,'if(observed.size!==files.size||[...files].some(name=>!observed.has(name)))throw Error("Rust unit output inventory differs from exact declared artifacts: missing "+[...files].filter(name=>!observed.has(name)).slice(0,8).join(", "))')
p=os.path.join(OUT,rel);os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w",encoding="utf8").write(t);print("ok")
