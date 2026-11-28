import os,sys
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def edit(path,old,new):
    t=open(path,encoding="utf8").read()
    if t.count(old)!=1: sys.exit("anchor count %d in %s"%(t.count(old),path))
    tmp=path+".h30tmp"
    open(tmp,"w",encoding="utf8").write(t.replace(old,new))
    os.replace(tmp,path)
A=L+"🏗️build/🕸️graph/🚪️admission/🟦️.ts"
edit(A,"""if(args[0]!=="run")return {kind:"all"};
 const excludeDependencies=buildBooleanOptionV1(head,["exclude-dependencies","exclude-task-dependencies"])===true,""",
"""if(args[0]!=="run"&&args[0]!=="inputs")return {kind:"all"};
 const excludeDependencies=args[0]==="inputs"||buildBooleanOptionV1(head,["exclude-dependencies","exclude-task-dependencies"])===true,""")
print("ok")
