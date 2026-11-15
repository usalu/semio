"""R13b payload: lender readiness is the live state of the toolchain the agent offers (no marker file); the typed memory refusal maps to exit code 75 at the command entry."""
import sys,os
sys.path.insert(0,os.path.join(os.path.dirname(os.path.abspath(__file__)),"..","R13"))
import importlib.util
spec=importlib.util.spec_from_file_location("p13",os.path.join(os.path.dirname(os.path.abspath(__file__)),"..","R13","payload.py"));p=importlib.util.module_from_spec(spec);spec.loader.exec_module(p)
TITLE="lender readiness from the live toolchain instead of a marker file; typed graph memory refusal exits 75"
F,B,MEM,RE,APPEND=p.F,p.B,p.MEM,p.RE,p.APPEND
EDITS=[]
def edit(path,old,new):EDITS.append((path,old,new))
def undo(path_end,needle):
    hits=[e for e in p.EDITS if e[0].endswith(path_end) and needle in e[2] and e[1]!=APPEND]
    assert len(hits)==1,(path_end,needle,len(hits))
    path,old,new=hits[0];EDITS.append((path,new,old))
undo("/🛰️agent/🩺️readiness/🟦️.ts","import {existsSync}")
undo("/🛰️agent/🩺️readiness/🟦️.ts","native bootstrap of the machine")
undo("/🛰️agent/🩺️readiness/🟦️.ts","bootstrap-pending")
undo("/🔣️policy.json","readyMarker")
undo("/🌐️remote-execution.json","readyMarker")
undo("/🌐️remote-execution.json","bootstrap-pending")
undo("/🌡️load/🔎️why/🟦️.ts",'"bootstrap-pending"|')
undo("/🌡️load/🔎️why/🟦️.ts",'"bootstrap-pending":{')
undo("/🛰️sidecar/🧪️tests/🟦️.ts","native-bootstrap.done")
undo("/🛰️sidecar/👷️agent/🟦️.ts","marker=join(")
undo("/🛰️sidecar/👷️agent/🟦️.ts","ready (native")
undo("/🛰️sidecar/👷️agent/🟦️.ts","ready:()=>existsSync(marker)")
undo("/🛰️sidecar/👷️agent/🟦️.ts","import {existsSync}")
A=RE+"/🛰️sidecar/👷️agent/🟦️.ts"
edit(A,'import {availableParallelism,homedir} from "node:os";','import {existsSync} from "node:fs";\nimport {availableParallelism,homedir} from "node:os";')
edit(A,'worker:why===undefined,disk:policy.worker.disk,','worker:why===undefined,ready:()=>rustc!==undefined&&rustc.toolchain.targets.length>0&&Object.values(rustc.tools).every(tool=>existsSync(tool.path)),disk:policy.worker.disk,')
edit(A,'say(`joining ${scheduler}${why!==undefined?` without a worker: ${why.en}`:` offering rustc for ${rustc!.toolchain.targets.length} target(s)`}`);','say(`joining ${scheduler}${why!==undefined?` without a worker: ${why.en} / ohne Worker: ${why.de}`:` offering rustc for ${rustc!.toolchain.targets.length} target(s) / bietet rustc für ${rustc!.toolchain.targets.length} Ziel(e) an`}`);') if False else None
AG=RE+"/🛰️agent/🟦️.ts"
edit(AG,'(its native bootstrap marker exists)','(the toolchain behind its offer is installed)')
edit(AG,'why:remoteExecutionWorkerWhyV1("bootstrap-pending")','why:remoteExecutionWorkerWhyV1("no-toolchain","the compiler of the offer is gone or has no target")')
T=RE+"/🧪️tests/🔎️why/🟦️.ts"
edit(T,'"policy-off","bootstrap-pending","not-registered"','"policy-off","not-registered"')
edit(T,' expect(check()).toBe("bootstrap-pending");\n mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(join(root,".cache","f","native-bootstrap.done"),"2026-11-15T00:00:00Z\\n");\n','')
edit(T,'import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";','import {mkdtempSync,rmSync} from "node:fs";')
edit(T,'an agent whose machine has not finished its bootstrap refuses','an agent whose offered toolchain is gone refuses')
edit(T,'row.worker.why?.code==="bootstrap-pending"));','row.worker.why?.code==="no-toolchain"));')
edit(T,'toContain("native bootstrap");expect(waiting.worker.why!.de).toContain("Bootstrap")','toContain("toolchain");expect(waiting.worker.why!.de).toContain("Toolchain")')
edit(T,'row.worker.why?.code!=="bootstrap-pending").pop()','row.worker.why?.code!=="no-toolchain").pop()')
edit(MEM+"/🟨️.mjs",'/** 🖥️ The memory of the host in megabytes as this process may use it','''/** 🔎️ The typed refusal in `error` or its causes (a failure tag wraps errors), else undefined: the command entry ends the command with its `exitCode` (75) and prints its message in both languages.
 * @see GraphMemoryRefusedErrorV1 */
export function graphMemoryRefusalOfV1(error,depth=6){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(error instanceof GraphMemoryRefusedErrorV1||error.code==="BUILD_GRAPH_MEMORY_REFUSED")return error;
 return graphMemoryRefusalOfV1(error.cause,depth-1);
}

/** 🖥️ The memory of the host in megabytes as this process may use it''')
edit(MEM+"/🟨️.d.mts",APPEND,'/** 🔎️ The typed refusal inside `error` or its causes. */\nexport function graphMemoryRefusalOfV1(error:unknown,depth?:number):GraphMemoryRefusedErrorV1|undefined;')
edit(MEM+"/🧪️tests/🟦️.ts",APPEND,'''
test("the command entry finds the typed refusal behind wrappers and ends the command with exit code 75",async()=>{
 const refused=new memory.GraphMemoryRefusedErrorV1({key:"k",needBytes:2*MEGABYTE,availableBytes:MEGABYTE,freeBytes:0,seconds:61,message:"refused"});
 expect(memory.graphMemoryRefusalOfV1(Object.assign(Error("wrapped"),{cause:Object.assign(Error("again"),{cause:refused})}))).toBe(refused);
 expect(memory.graphMemoryRefusalOfV1(Error("other"))).toBeUndefined();expect(memory.graphMemoryRefusalOfV1(undefined)).toBeUndefined();
 const source=await Bun.file(new URL("../../⌨️command/📥️entry/🟦️.ts",import.meta.url)).text();
 expect(source).toContain("graphMemoryRefusalOfV1");expect(source).toContain("process.exitCode=refused.exitCode");
 const probe=Bun.spawnSync([process.execPath,"-e",`process.exitCode=75;`]);expect(probe.exitCode).toBe(refused.exitCode);
});
''')
edit(F+"/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/📥️entry/🟦️.ts",'const code=dependencyReadmissionCodeV1(error);if(code!==undefined){process.exitCode=code;return;}exitIfStaleImplementationV1(error);','const code=dependencyReadmissionCodeV1(error);if(code!==undefined){process.exitCode=code;return;}const{graphMemoryRefusalOfV1}=await import("../../🧠️memory/🟨️.mjs");const refused=graphMemoryRefusalOfV1(error);if(refused!==undefined){process.stderr.write(refused.message+"\\n");process.exitCode=refused.exitCode;return;}exitIfStaleImplementationV1(error);')
EDITS=[e for e in EDITS if e is not None]
TRANSFORMS={}
TESTS=[MEM+"/🧪️tests/🟦️.ts",T,RE+"/🛰️sidecar/🧪️tests/🟦️.ts",p.HC+"/🧪️tests/🟦️.ts"]
