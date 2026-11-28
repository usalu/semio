import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧱️ The declared limits of one compile unit and the carving debt of the units above them.
 * @see ./🧬️schema/🔣️.json */
export const UNIT_LIMITS_POLICY_V1=schema.policy;
const policy=UNIT_LIMITS_POLICY_V1;

/** 🧱️ How one unit stands against its limits: within them, near them, above them but inside its declared carving budget, above even that, or not measured yet. */
export type UnitLimitStateV1="ok"|"near"|"carving"|"over"|"unmeasured";

/** 🧱️ One unit of a plan with what a past build measured of it (none when it was never built). */
export type PlannedUnitV1=Readonly<{crate:string;target:"host"|"wasm";milliseconds?:number;megabytes?:number}>;

/** 🧱️ The limits that hold for a unit: its carving budget when it is declared as carving debt, else the defaults. */
export function unitLimitsV1(crate:string,target:"host"|"wasm"):Readonly<{maximumMilliseconds:number;maximumMegabytes:number;carving?:(typeof policy.carving)[number]}>{
 const row=policy.carving.find(entry=>entry.crate===crate&&entry.target===target);
 return row===undefined?{maximumMilliseconds:policy.defaults.maximumMilliseconds,maximumMegabytes:policy.defaults.maximumMegabytes}:{maximumMilliseconds:row.budgetMilliseconds,maximumMegabytes:row.budgetMegabytes,carving:row};
}

/** ⚖️ The state of one unit: over its limit is `over` (a carving unit above its own budget) or `carving` (above the defaults, inside its budget), within `warnBasisPoints` of the limit is `near`. */
export function unitLimitStateV1(unit:PlannedUnitV1):UnitLimitStateV1{
 if(unit.milliseconds===undefined&&unit.megabytes===undefined)return"unmeasured";
 const limits=unitLimitsV1(unit.crate,unit.target),warn=policy.defaults.warnBasisPoints/10000,time=unit.milliseconds??0,memory=unit.megabytes??0;
 if(time>limits.maximumMilliseconds||memory>limits.maximumMegabytes)return"over";
 if(limits.carving!==undefined)return"carving";
 return time>=limits.maximumMilliseconds*warn||memory>=limits.maximumMegabytes*warn?"near":"ok";
}

/** 📋️ The warnings of a plan before its build starts, in English and German: every unit above a limit (and the carving action declared for it), every carving unit still above the defaults, every unit near a limit. */
export function planUnitLimitsV1(units:readonly PlannedUnitV1[]):Readonly<{warnings:readonly Readonly<{crate:string;target:string;state:UnitLimitStateV1;en:string;de:string}>[];over:number}>{
 const warnings:Array<{crate:string;target:string;state:UnitLimitStateV1;en:string;de:string}>=[];
 for(const unit of units){
  const state=unitLimitStateV1(unit);if(state==="ok"||state==="unmeasured")continue;
  const limits=unitLimitsV1(unit.crate,unit.target),minutes=Math.round((unit.milliseconds??0)/60000),megabytes=unit.megabytes??0,defaults=policy.defaults;
  const measured=`${minutes} min, ${megabytes} MB`,maximum=`${Math.round(limits.maximumMilliseconds/60000)} min, ${limits.maximumMegabytes} MB`;
  const action=limits.carving===undefined?undefined:limits.carving;
  warnings.push({crate:unit.crate,target:unit.target,state,
   en:state==="near"?`${unit.crate} (${unit.target}) is near its limit: ${measured} of ${maximum}`:state==="carving"?`${unit.crate} (${unit.target}) is declared carving debt (${action!.action}): ${measured}, above the default ${Math.round(defaults.maximumMilliseconds/60000)} min, ${defaults.maximumMegabytes} MB, within its budget ${maximum}: ${action!.reason.en}`:`${unit.crate} (${unit.target}) is over its limit: ${measured} of ${maximum}${action===undefined?"; carve it along the domain taxonomy or declare it":" ("+action.action+")"}`,
   de:state==="near"?`${unit.crate} (${unit.target}) ist nahe an der Grenze: ${measured} von ${maximum}`:state==="carving"?`${unit.crate} (${unit.target}) ist deklarierte Schnitt-Schuld (${action!.action}): ${measured}, über dem Standard ${Math.round(defaults.maximumMilliseconds/60000)} min, ${defaults.maximumMegabytes} MB, innerhalb des Budgets ${maximum}: ${action!.reason.de}`:`${unit.crate} (${unit.target}) überschreitet die Grenze: ${measured} von ${maximum}${action===undefined?"; entlang der Domänen-Taxonomie schneiden oder deklarieren":" ("+action.action+")"}`});
 }
 return{warnings,over:warnings.filter(row=>row.state==="over").length};
}
