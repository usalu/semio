export function serviceModeV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>):"auto"|"off"|"require";
export function serviceEligibleV1(args:readonly string[]):boolean;
export function forwardRepoCommandV1(options:{root:string;args:readonly string[];environment:Readonly<Record<string,string|undefined>>;languageOf:()=>string;stdout?:{write(text:string|Uint8Array):unknown};stderr?:{write(text:string|Uint8Array):unknown};cwd?:string;forward?:typeof import("./index.mjs").forwardCommandV1}):Promise<number|undefined>;
