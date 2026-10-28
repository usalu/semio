import {availableParallelism,totalmem} from "node:os";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.definitions.Protocol.const,MEGABYTE=1048576;

/** 🪞️ The machine every part of the build system plans for: its memory and cores, and whether they were declared. */
export type HostProfileV1=Readonly<{totalMegabytes:number;cores:number;declared:boolean}>;

/** 🖥️ Reads the real host once: the memory and cores the operating system reports. */
export function realHostProfileV1():HostProfileV1{return Object.freeze({totalMegabytes:Math.floor(totalmem()/MEGABYTE),cores:Math.max(1,availableParallelism()),declared:false});}

const declaration=(text:string|undefined,minimumDigits:number)=>text!==undefined&&new RegExp(`^[1-9][0-9]{${minimumDigits-1},8}$`,"u").test(text)?Number(text):null;

/** 🔑️ The declared machine as one comparable string, empty when nothing valid is declared: processes that share the sample file of the gate share its sample only when they declare the same machine. */
export function hostDeclarationKeyV1(environment:Readonly<Record<string,string|undefined>>=process.env):string{
 const total=declaration(environment[protocol.totalEnvironment],4),cores=declaration(environment[protocol.coresEnvironment],1);
 return total===null&&cores===null?"":`${total??"-"}/${cores??"-"}`;
}

/** 🪞️ Resolves the host a process plans for: `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES` and `SEMIO_HOST_CAPACITY_CORES` declare a smaller machine, so a 16 GB laptop can be measured and tested on a larger host by the capacity gate, the worker policy, the Bazel server planner and the test runners alike; a declaration only ever shrinks the real host (what is not there cannot be declared) and a malformed value means the real host. */
export function hostProfileV1(environment:Readonly<Record<string,string|undefined>>=process.env,real:HostProfileV1=realHostProfileV1()):HostProfileV1{
 const total=declaration(environment[protocol.totalEnvironment],4),cores=declaration(environment[protocol.coresEnvironment],1);
 return Object.freeze({totalMegabytes:total===null?real.totalMegabytes:Math.min(real.totalMegabytes,total),cores:cores===null?real.cores:Math.min(real.cores,cores),declared:total!==null||cores!==null});
}
