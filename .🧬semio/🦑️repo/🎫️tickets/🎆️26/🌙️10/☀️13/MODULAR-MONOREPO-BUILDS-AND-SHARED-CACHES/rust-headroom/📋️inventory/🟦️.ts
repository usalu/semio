import{resolve,relative,isAbsolute,sep}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
/** 📏️ Bounds observational work independently of any deletion or authority decision. */
export const readOnlyInventoryPolicyV1=Object.freeze(schema.const);
type Unit=Readonly<{path:string;bytes:number;recencyMs:number;lockHeld:boolean;kind:string}>;
type Plan=Readonly<{units:readonly Unit[];bytes:number;remainingBytes:number;unmetBytes:number}>;
/** 🔎️ Explains a genuine planner result as data without inferring process liveness or retirement custody. */
export function explainCargoInventoryV1(units:readonly Unit[],plan:Plan){
 if(units.length>readOnlyInventoryPolicyV1.maximumUnits||new Set(units.map(row=>row.path)).size!==units.length)throw Error("Bounded unique Cargo inventory required");const selected=new Set(plan.units.map(row=>row.path));let protectedBytes=0,totalBytes=0;
 const rows=units.map(row=>{if(!Number.isSafeInteger(row.bytes)||row.bytes<0||Buffer.byteLength(row.path)>1024)throw Error("Bounded exact Cargo inventory row required");totalBytes+=row.bytes;if(row.lockHeld)protectedBytes+=row.bytes;return{...row,classification:selected.has(row.path)?"selected-by-existing-plan":row.lockHeld?(row.kind==="cargo-incremental"?"working-session-present":"protected-unit"):"not-selected-by-existing-plan"};});
 return{version:1,totalBytes,protectedBytes,selected:[...selected],unmetBytes:plan.unmetBytes,rows};
}

/** 🧭️ Bounds an existing metadata observation; grants no path materialization or filesystem custody. */
export function readOnlyInventoryPathV1(root:string,value:string):string{
 const path=resolve(value),suffix=relative(resolve(root),path);if(suffix===".."||suffix.startsWith(".."+sep)||isAbsolute(suffix)||path.includes("\0")||path.length>readOnlyInventoryPolicyV1.maximumObservedPathCharacters)throw Error("Bounded existing Rust observation path required");return path;
}
