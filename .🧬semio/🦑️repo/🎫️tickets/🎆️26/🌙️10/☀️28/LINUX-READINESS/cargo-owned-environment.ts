import {test,expect} from "bun:test";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
const root=process.cwd(),load=(p:string)=>import(pathToFileURL(join(root,p)).href);
const {withCargoAdmissionV1}=await load("🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🗄️admission/🟦️.ts");
const {captureOwnedCommandResultV1}=await load("🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts");
test("the real Cargo law owner reports its retained managed plugin readiness",()=>withCargoAdmissionV1(async(admission:any,law:any)=>{
 expect(admission.storage.allocation.kind).toBe("compiler-workspace");
 await admission.storage.recheck();expect(law.environment.CARGO_HOME.startsWith(admission.storage.backing.root)).toBe(true);
 const commands=[],paths=[law.root,law.environment.CARGO_HOME,law.environment.SEMIO_TOOL_CARGO_NEXTEST,law.environment.SEMIO_TOOL_CARGO_LLVM_COV].map(path=>({path,length:path.length}));
 console.log("[DEBUG] "+JSON.stringify({kind:"actual-cargo-owner-paths",paths}));
 expect(paths.every(row=>row.length<=256)).toBe(true);
 for(const name of ["nextest","llvm-cov"]){const result=await captureOwnedCommandResultV1("cargo",[name,"--version"],admission.workspaceRoot,{control:admission.control,env:{...process.env,...law.environment},output:{maximumBytes:8192,maximumLines:64}});commands.push({name,status:result.status,signal:result.signal,output:(result.stdout+result.stderr).slice(0,1024)});expect(result.status).toBe(0);expect(result.signal).toBeNull();expect(law.environment["SEMIO_TOOL_CARGO_"+name.toUpperCase().replaceAll("-","_")].startsWith(law.root)).toBe(true);}
 await admission.storage.recheck();console.log("[DEBUG] "+JSON.stringify({kind:"actual-cargo-owner-readiness",commands}));
}),30000);
