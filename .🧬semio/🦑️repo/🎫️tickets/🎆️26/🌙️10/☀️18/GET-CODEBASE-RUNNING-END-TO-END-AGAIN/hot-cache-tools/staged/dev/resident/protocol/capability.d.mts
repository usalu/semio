export type CaveatV1={k:"scope";v:string[]}|{k:"projects";v:string[]}|{k:"notAfter";v:number}|{k:"cwd";v:string};
export function canonicalJsonV1(value:unknown):string;
export function createRootKeyV1():Buffer;
export function mintCapabilityV1(key:Uint8Array,caveats:readonly CaveatV1[],id?:string):string;
export function attenuateCapabilityV1(token:string,caveat:CaveatV1):string;
export function verifyCapabilityV1(key:Uint8Array,token:string):CaveatV1[]|undefined;
export function authorizeCapabilityV1(caveats:readonly CaveatV1[],request:{commandClass:string;command:string;cwd:string;now:number;projects:readonly string[]}):{code:"scope"|"capability";class?:string;reason?:string}|undefined;
