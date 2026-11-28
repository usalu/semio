import sys,os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution"
H=os.path.dirname(os.path.abspath(__file__))
def rw(p,f):
    s=open(p,encoding='utf8').read(); n=f(s); assert n!=s,p
    t=p+".h69tmp"; open(t,'w',encoding='utf8').write(n); os.replace(t,p)
def sub(s,old,new):
    assert s.count(old)==1,(old[:60],s.count(old)); return s.replace(old,new)
fn=open(H+"/function.ts.txt",encoding='utf8').read()
def tool(s):
    s=sub(s,'import {sdkIdentityV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";\n','import {sdkIdentityV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";\nimport {rustSourceRemapArgumentsV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🟦️.ts";\n')
    return s.rstrip("\n")+"\n"+fn
rw(B+"/🚀️offload/🧰️toolchain/🟦️.ts",tool)
def worker(s):
    s=sub(s,'import {OFFLOAD_REFUSAL_PREFIX_V1} from "../🚀️offload/🩺️failure/🟦️.ts";\n','import {OFFLOAD_REFUSAL_PREFIX_V1} from "../🚀️offload/🩺️failure/🟦️.ts";\nimport {workerRustSourceRemapV1} from "../🚀️offload/🧰️toolchain/🟦️.ts";\n')
    return sub(s,"   args[0]=tool.path;readable.push(...tool.readable);\n  }\n  let checkoutRun","   args[0]=tool.path;readable.push(...tool.readable);\n   if(toolName===\"rustc\")args.push(...workerRustSourceRemapV1(tool.path,args.slice(1)));\n  }\n  let checkoutRun")
rw(B+"/👷️worker/🟦️.ts",worker)
def test(s):
    s=sub(s,'import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";','import {createHash} from "node:crypto";\nimport {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";')
    s=sub(s,'import {detectWorkerRustcV1} from','import {detectWorkerRustcV1,workerRustSourceRemapV1} from')
    return s.rstrip("\n")+"\n"+open(H+"/test-append.ts.txt",encoding='utf8').read()
rw(B+"/🚀️offload/🧪️tests/🧰️toolchain/🟦️.ts",test)
print("landed")
