import type {GenerationOwnerRowV1} from "../🟨️.mjs";
type OwnerTableV1=Readonly<Record<string,Pick<GenerationOwnerRowV1,"edgesEnv"|"tag">>>;
/** 🪡️ The owners whose edges are of another environment, untagged ones in `edges`, tagged ones in `tagged`. */
export function staleEdgeOwnersV1(owners:OwnerTableV1,environment:string):{edges:string[];tagged:string[]};
/** 🪡️ The `edgeStale` field of a compute spec: stale untagged owners the build does not infer anyway. */
export function edgeStaleSpecV1(owners:OwnerTableV1,environment:string|null|undefined,inferred:Iterable<string>):{edgeStale?:string[]};
/** 🩹️ The owners a build gives their edges again (`edges`) or infers (`infer`) after its compute left stale ones. */
export function edgeHealPlanV1(input:{owners:OwnerTableV1;environment:string;cold:boolean;healed:number;limit?:number}):{edges:string[];infer:string[]};
/** 🚧️ Refuses a publication whose owner table holds edges of another environment. */
export function requireEdgeEnvironmentV1(input:{owners:OwnerTableV1;environment:string;fail:(message:string,code?:string)=>Error;wave:readonly string[];returned:ReadonlySet<string>;dropped:ReadonlySet<string>;carried?:readonly string[];cold:boolean;repair:boolean;healed:number;resumed:boolean;before:Readonly<Record<string,{edgesEnv?:string}>>;previous:string|null|undefined}):void;
