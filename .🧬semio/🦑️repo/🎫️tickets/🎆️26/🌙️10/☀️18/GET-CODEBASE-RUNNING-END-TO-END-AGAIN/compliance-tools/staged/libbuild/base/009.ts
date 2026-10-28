import {GRAPH_MEMO_POLICY_V1} from "../🟦️.ts";
import {canonicalJsonV1,compareCodePointsV1,createGenerationStoreV1,createNodeFilePortV1,type GenerationManifestV1} from "../../../../🕸️graph/🕰️generations/🟨️.mjs";
import {decodeVerifiedScopeV1} from "../../../../🕸️graph/🕰️generations/🔭️coverage/🟨️.mjs";
import {encodeGenerationPinV1,readGenerationPinV1} from "../../../../🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";

/** 🧳️ The parts of two manifests a slice depends on: the generation-wide digests the policy names and the rows of the owners the slice covers. */
export type MemoCarryManifestV1=Readonly<Record<string,unknown>&{owners:Readonly<Record<string,unknown>>}>;
/** 🧾️ What a carry answers: the pin of the newer generation for the same owners, or why the slice is not the same there. */
export type MemoCarryAnswerV1=Readonly<{carried:true;pin:string}>|Readonly<{carried:false;reason:string}>;

/** ⚖️ Whether the slice of `covered` owners is the same in `to` as in `from`: every generation-wide digest the policy names is equal and every covered owner has the identical row (fragment, edges environment, targets, configuration) in both manifests. Owners outside the slice may differ freely. Answers `same` or the first difference (`shared:<name>`, `owner:<name>`).
 * @see ../🧬️schema/🔣️.json `carry` */
export function graphMemoSliceSameV1(from:MemoCarryManifestV1,to:MemoCarryManifestV1,covered:Iterable<string>):string{
 for(const name of GRAPH_MEMO_POLICY_V1.carry.shared)if(canonicalJsonV1(from[name]??null)!==canonicalJsonV1(to[name]??null))return "shared:"+name;
 for(const owner of [...covered].sort(compareCodePointsV1)){const before=from.owners[owner],after=to.owners[owner];if(before===undefined||after===undefined||canonicalJsonV1(before)!==canonicalJsonV1(after))return "owner:"+owner;}
 return "same";
}

/** 🧳️ Carries the pin of a served slice from the generation its memo was sealed with to the generation the pointer names now, when that slice is the same there ({@link graphMemoSliceSameV1}): the planner asked now would verify the same facts (the journal and the recheck already proved them) and assemble the same projects from the same rows, so the slice is served under the newer generation, which is the one retention keeps. A retired or unreadable manifest, a pin of another manifest or a different slice is refused with its reason.
 * @see ../🟦️.ts `lookup` */
export async function carryGraphMemoPinV1(input:Readonly<{generations:string;from:Readonly<{generation:number;manifest:string}>;to:Readonly<{generation:number;manifest:string}>;pin:string;now?:()=>number;signal?:AbortSignal}>):Promise<MemoCarryAnswerV1>{
 const refuse=(reason:string):MemoCarryAnswerV1=>({carried:false,reason});
 let pin:ReturnType<typeof readGenerationPinV1>;try{pin=readGenerationPinV1(input.pin);}catch{return refuse("pin");}
 if(pin.manifest!==input.from.manifest||pin.generation!==input.from.generation)return refuse("pin");
 const store=createGenerationStoreV1({root:input.generations,fs:createNodeFilePortV1(),holds:async()=>true,resources:{facts:"graph-memo"}});
 const [from,to]=await Promise.all([store.readManifestAt(input.from.manifest,input.signal),store.readManifestAt(input.to.manifest,input.signal)]) as [GenerationManifestV1|undefined,GenerationManifestV1|undefined];
 if(!from)return refuse("retired "+input.from.generation);
 if(!to||to.generation!==input.to.generation)return refuse("moved");
 let covered:Set<string>;try{covered=decodeVerifiedScopeV1(Object.keys(from.owners).sort(compareCodePointsV1),pin.covered);}catch{return refuse("pin");}
 const same=graphMemoSliceSameV1(from as unknown as MemoCarryManifestV1,to as unknown as MemoCarryManifestV1,covered);
 if(same!=="same")return refuse(same);
 return{carried:true,pin:encodeGenerationPinV1({owner:pin.owner,generation:to.generation,manifest:input.to.manifest,owners:Object.keys(to.owners).sort(compareCodePointsV1),covered,atMs:(input.now??Date.now)()})};
}
