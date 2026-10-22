import type {StateConfigV1} from "../state/index.mjs";
export function stateConfigurationFromBuildGraphV1(graph:any,options?:{environmentPaths?:readonly string[];environmentPrefixes?:readonly string[]}):StateConfigV1;
export function createBuildGraphPortV1(options:{loadGraph:()=>Promise<any>;salt:()=>Promise<string>|string;listing:import("../service/index.mjs").ResidentPortsV1["graph"]["listing"];environmentPaths?:readonly string[];environmentPrefixes?:readonly string[];generation?:(graph:any)=>number|null}):import("../service/index.mjs").ResidentPortsV1["graph"];
export function taskClosureV1(graph:unknown,named:readonly string[]):Set<string>;
