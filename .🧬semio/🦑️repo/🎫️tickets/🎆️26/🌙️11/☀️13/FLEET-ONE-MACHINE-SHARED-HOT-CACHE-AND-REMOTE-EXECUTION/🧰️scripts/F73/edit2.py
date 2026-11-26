import os,tempfile
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for a,b in pairs:
        assert s.count(a)==1,(a[:50],s.count(a));s=s.replace(a,b)
    fd,t=tempfile.mkstemp(dir=os.path.dirname(path));os.write(fd,s.encode());os.close(fd);os.replace(t,path)
edit(R+"🧬️schema/🔣️.json",[('"^error: failed to write [`\']{root}[^`\']*[`\']\\\\s*$"','"^error: failed to write [`\']{root}[^`\']*[`\']\\\\s*$", "unexpected failure \\\\(reference [0-9a-f]+\\\\): Error: (?:Complete retained closure publication required|Closure reference proof leaf changed)", "unexpected failure \\\\(reference [0-9a-f]+\\\\): Error: Slice worker exited with code null"')])
edit(R+"🧪️tests/🧱️source-transient/🟦️.ts",[(' {lines:["error: failed to write `/elsewhere/cache/x.timestamp`"],root,want:false},\n',' {lines:["error: failed to write `/elsewhere/cache/x.timestamp`"],root,want:false},\n {lines:["The resident service met an unexpected failure (reference 145eb0b1): Error: Complete retained closure publication required"],root,want:true},\n {lines:["The resident service met an unexpected failure (reference 84438681): Error: Closure reference proof leaf changed"],root,want:true},\n {lines:["The resident service met an unexpected failure (reference d4797b0e): Error: Slice worker exited with code null: [graph-closure] {}"],root,want:true},\n {lines:["The resident service met an unexpected failure (reference 1): Error: Slice worker exited with code 1: out of memory"],root,want:false},\n')])
