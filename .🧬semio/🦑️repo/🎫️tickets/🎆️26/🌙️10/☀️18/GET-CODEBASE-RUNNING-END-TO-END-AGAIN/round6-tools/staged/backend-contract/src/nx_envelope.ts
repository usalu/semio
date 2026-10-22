import {resolve} from "node:path";
import {validateJsonSchemaSubset} from "@@VALID@@";
import {readIssuedScriptProcessEnvelopeV1,readScriptProcessEnvelope,type ScriptInvocation,type ScriptProcessEnvelope} from "@@PROC@@/🧭️routing/🟦️.ts";
import nxCallerSchema from "@@UP@@../⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json" with {type:"json"};
import schema from "@@UP@@🧬️schema/📥️caller.json" with {type:"json"};
import type {NativeBuildCallerCapabilitiesV1} from "@@UP@@📥️caller/🟦️.ts";
import {isStoreRepairSelectionV1} from "@@UP@@🧾️store-receipt/🟨️.mjs";

/** 📨️ Serializes only predeclared Nx child data while retaining the exact private parent invocation. */
export function projectOwnedNxProcessEnvelopeV1(root:string,invocation:ScriptInvocation,selected:readonly string[]):ScriptProcessEnvelope{
 const original=readIssuedScriptProcessEnvelopeV1(invocation),role=original.capabilities as NativeBuildCallerCapabilitiesV1,selection=["nx",...selected];let capabilities:object;
 if(role.tool!==undefined){if(validateJsonSchemaSubset(schema,role).length||resolve(role.workspaceRoot)!==resolve(root)||role.arguments[0]!=="repo")throw Error("Original Repo-to-Nx caller required");const launches=role.launches.filter(launch=>launch[0]==="nx");if(!launches.length||!launches.some(launch=>JSON.stringify(launch)===JSON.stringify(selection))&&!isStoreRepairSelectionV1(selected))throw Error("Undeclared Repo-to-Nx child selection");capabilities={workspaceRoot:role.workspaceRoot,arguments:launches[0],selection,buildMode:role.buildMode,native:role.native,launches};}
 else{const role=original.capabilities as import("@@UP@@../⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts").NativeNxCallerCapabilitiesV1;if(validateJsonSchemaSubset(nxCallerSchema,role).length||resolve(role.workspaceRoot)!==resolve(root)||role.arguments[0]!=="nx"||!role.launches?.some(launch=>JSON.stringify(launch)===JSON.stringify(selection)))throw Error("Undeclared original Nx child selection");capabilities={...role,selection};}
 if(validateJsonSchemaSubset(nxCallerSchema,capabilities).length)throw Error("Closed original Nx child data required");return readScriptProcessEnvelope({...original,capabilities},Date.now());
}
