import glob,sys
R=glob.glob("/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution")[0]
root=sys.argv[1] if len(sys.argv)>1 else R
p=root+"/⌨️command/🟦️.ts"
s=open(p).read()
def once(a,b):
    global s
    assert s.count(a)==1,(a[:70],s.count(a))
    s=s.replace(a,b)
once('import texts from "../📖️texts/🔣️.json" with {type:"json"};','import {detectWorkerRustcV1} from "../🚀️offload/🧰️toolchain/🟦️.ts";\nimport texts from "../📖️texts/🔣️.json" with {type:"json"};')
once(' await runRemoteExecutionAgentV1({\n  root:context.root,',' const rustc=detectWorkerRustcV1(context.root,context.environment);\n await runRemoteExecutionAgentV1({\n  root:context.root,')
once('extraProperties:properties,signal:context.signal','extraProperties:[...properties,...rustc?.properties??[]],...rustc?{tools:rustc.tools}:{},signal:context.signal')
open(p,"w").write(s)
print("ok")
