import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import emit, live
HERE = os.path.dirname(os.path.abspath(__file__))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
DEP = B + "/🏃️execution/🔒️hermetic/📦️dependencies/🟦️.ts"
RUN = B + "/🔧️tools/🏗️bazel/🏃️execution/🏭️runner/🟦️.ts"
CONC = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent"
GLUE, SCHEMA = CONC + "/💾️storage/🟦️.ts", CONC + "/🧬️schema/🔣️.json"
edits = []
# (3) .bin shims: only an absent entry or a non-link is "no shim"
edits.append((DEP, "export interface HermeticDependenciesV1{rows:HermeticModulesRowV1[];ids:string[];stores:string[]}\n",
"""export interface HermeticDependenciesV1{rows:HermeticModulesRowV1[];ids:string[];stores:string[]}
/** 🔌️ The two filesystem reads of the shim scan, replaceable so every failure code is testable on any host. */
export interface HermeticShimPortV1{list(directory:string):string[];target(path:string):string}
const systemShims:HermeticShimPortV1={list:directory=>readdirSync(directory),target:path=>readlinkSync(path)},errorCode=(error:unknown)=>(error as NodeJS.ErrnoException)?.code;
/** 🔗️ Lists the `.bin` shims of one `node_modules` directory as name and relative link target. An absent `.bin` and an entry that is not a link (a regular file, as bun writes them on Windows) mean "no shim"; every other failure, such as a refused permission or an I/O error, is thrown, because dropping the shims silently would run the action without its tools. */
export function hermeticShimsV1(modules:string,port:HermeticShimPortV1=systemShims):[string,string][]{
 const bin=join(modules,".bin");let names:string[];
 try{names=port.list(bin).sort(order);}catch(error){if(errorCode(error)==="ENOENT"||errorCode(error)==="ENOTDIR")return [];throw error;}
 const found:[string,string][]=[];
 for(const name of names){try{found.push([name,port.target(join(bin,name))]);}catch(error){if(errorCode(error)!=="ENOENT"&&errorCode(error)!=="EINVAL")throw error;}}
 return found;
}
"""))
edits.append((DEP, ' const shims=(modules:string):[string,string][]=>{try{return readdirSync(join(modules,".bin")).sort(order).map(name=>[name,readlinkSync(join(modules,".bin",name))] as [string,string]);}catch{return [];}};\n', ' const shims=(modules:string)=>hermeticShimsV1(modules);\n'))
# (2) runner: a file symlink needs a privilege on Windows, so a shim file is written there
edits.append((RUN, 'const link=(target,path)=>{mkdirSync(dirname(path),{recursive:true});symlinkSync(target,path,"junction");};\n',
 'const link=(target,path)=>{mkdirSync(dirname(path),{recursive:true});symlinkSync(target,path,"junction");};\n'
 'const shim=(target,path,platform=process.platform)=>{if(platform!=="win32"){symlinkSync(target,path);return;}const real=resolve(dirname(path),target);if(/[%"\\r\\n]/.test(real+executable))fail("a tool path with a percent sign, a quote or a line break cannot be shimmed: "+real);if(/\\.(exe|com)$/i.test(real)){copyFileSync(real,path+".exe");return;}readFileSync(real);writeFileSync(path+".cmd",\'@"\'+executable+\'" "\'+real+\'" %*\\r\\n\');};\n'))
edits.append((RUN, 'symlinkSync(target,join(directory,".bin",name));}', 'shim(target,join(directory,".bin",name));}'))
# (5) free space: measured through the virtual statfs, refused with a bilingual diagnostic when unreadable
edits.append((GLUE, 'import {concurrentEventV1,concurrentLeaseResourceV1,', 'import {concurrentEventV1,concurrentLanguageV1,concurrentLeaseResourceV1,'))
edits.append((GLUE, 'concurrentProtocolV1 as protocol,concurrentWaitStepV1,', 'concurrentProtocolV1 as protocol,concurrentTextV1,concurrentWaitStepV1,'))
old_a = "export async function concurrentFreeFractionV1(path:string):Promise<number>{\n try{const facts=await nativeStatfsV1(path,{fresh:true});return facts.blocks>0?facts.bavail/facts.blocks:1;}catch{return 1;}\n}\n"
old_b = "export async function concurrentFreeBytesV1(path:string):Promise<number>{\n try{const facts=await nativeStatfsV1(path,{fresh:true});return facts.bavail*facts.bsize;}catch{return Number.MAX_SAFE_INTEGER;}\n}\n"
edits.append((GLUE, old_a, """export async function concurrentFreeFractionV1(path:string,read:ConcurrentVolumeReaderV1=readConcurrentVolumeV1):Promise<number>{
 const facts=await measureConcurrentVolumeV1(path,read);return facts.blocks>0?facts.bavail/facts.blocks:1;
}
/** 🔌️ Reads the volume that holds the generations; replaceable so an unreadable volume is testable on any host. */
export type ConcurrentVolumeReaderV1=(path:string)=>Promise<Readonly<{blocks:number;bavail:number;bsize:number}>>;
const readConcurrentVolumeV1:ConcurrentVolumeReaderV1=path=>nativeStatfsV1(path,{fresh:true});
async function measureConcurrentVolumeV1(path:string,read:ConcurrentVolumeReaderV1){
 try{return await read(path);}catch(error){throw Error(concurrentTextV1(concurrentLanguageV1(process.env),"freeSpaceUnreadable",{path,reason:(error as Error).message}),{cause:error});}
}
"""))
edits.append((GLUE, old_b, "export async function concurrentFreeBytesV1(path:string,read:ConcurrentVolumeReaderV1=readConcurrentVolumeV1):Promise<number>{\n const facts=await measureConcurrentVolumeV1(path,read);return facts.bavail*facts.bsize;\n}\n"))
edits.append((GLUE, "measured now: the mounted image on macOS, the declared envelope minus the measured tree on a directory partition, or 1 when the platform cannot tell. */", "measured now: the mounted image on macOS, the declared envelope minus the measured tree on a directory partition; an unreadable volume is refused with a bilingual diagnostic instead of being assumed to be empty or unlimited. */"))
edits.append((GLUE, "so a before-and-after difference is the size of what an install built; a platform that cannot tell reports no limit. */", "so a before-and-after difference is the size of what an install built; an unreadable volume is refused, never reported as unlimited. */"))
edits.append((SCHEMA, '          "waitingWriter": "Waiting for another process to finish writing a dependency receipt ({seconds} s).",\n',
 '          "waitingWriter": "Waiting for another process to finish writing a dependency receipt ({seconds} s).",\n          "freeSpaceUnreadable": "The free space of the dependencies volume could not be read ({path}): {reason}. Nothing is installed without knowing how much space is left.",\n'))
edits.append((SCHEMA, '          "waitingWriter": "Es wird gewartet, bis ein anderer Prozess eine Abhängigkeits-Quittung fertig geschrieben hat ({seconds} s).",\n',
 '          "waitingWriter": "Es wird gewartet, bis ein anderer Prozess eine Abhängigkeits-Quittung fertig geschrieben hat ({seconds} s).",\n          "freeSpaceUnreadable": "Der freie Platz des Abhängigkeits-Volumes konnte nicht gelesen werden ({path}): {reason}. Es wird nichts installiert, ohne zu wissen, wie viel Platz noch frei ist.",\n'))
new = [B + "/🏃️execution/🔒️hermetic/🧪️tests/🔗️shims/🟦️.ts", B + "/🏃️execution/🔒️hermetic/🧪️tests/🔗️shims/🧫️fixtures/🔣️.json",
       B + "/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️shim/🟦️.ts", B + "/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️shim/🧫️fixtures/🔣️.json",
       CONC + "/🧪️tests/🪟️free-space/🟦️.ts"]
emit("g14", "Windows hazards of the audit: .bin shims as files without a symlink privilege, only an absent or non-link shim is no shim, an unreadable dependency volume is refused in EN and DE", [GLUE], edits, new, os.path.join(HERE, "groups"))
print("g14 written", len(edits))
