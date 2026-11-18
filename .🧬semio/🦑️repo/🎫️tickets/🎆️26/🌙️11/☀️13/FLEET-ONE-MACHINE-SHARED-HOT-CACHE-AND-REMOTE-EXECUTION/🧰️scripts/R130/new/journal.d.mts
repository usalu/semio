/** 📏️ The declared limits of the build journal. */
export const BUILD_JOURNAL_V1:Readonly<{directory:string;version:1;rowsPerLine:number;maximumOwners:number}>;
/** 🧾️ The marker a finished step leaves. */
export interface BuildJournalStepV1{step:string;inputs:string;output:string}
/** 🔖️ The header of one build journal. */
export interface BuildJournalV1{version:1;key:string;state:"waves"|"sealed";parent:string|null;generation:number;identity:string;implementation:string;steps:readonly BuildJournalStepV1[];names?:readonly string[];done?:number;owners?:Readonly<Record<string,unknown>>;dropped?:readonly string[];returned?:readonly string[];derived?:readonly string[];rows?:readonly {file:string;sha256:string;bytes:number}[];manifest?:string;env?:{digest:string;parts?:unknown}}
/** 🔑️ The name of the journal of one build. */
export function buildJournalKeyV1(input:unknown):string;
/** 🔖️ Admits the header of one build journal. */
export function readBuildJournalV1(bytes:Uint8Array):BuildJournalV1;
/** 🧾️ Writes the rows of the owners of one wave. */
export function encodeJournalRowsV1(entries:readonly {name:string;mode:"u"|"a";rows:readonly unknown[]}[]):Buffer;
/** 🧾️ Reads the rows of the owners of one wave back. */
export function decodeJournalRowsV1(bytes:Uint8Array):{name:string;mode:"u"|"a";rows:unknown[]}[];
/** 🔖️ The journal of the build that is under way, on the file port of the store. */
export function createBuildJournalV1(input:{fs:any;root:string;manifestPath:(digest:string)=>string;publish:(options:any)=>Promise<any>;settler:any;say?:(key:"resumed"|"resumedWaves",fields:Readonly<Record<string,string|number>>)=>void;stepped?:(step:"storing"|"indexing"|"sealing")=>void|Promise<void>}):Readonly<{clear(key:string):Promise<void>;sealed(input:unknown):Promise<unknown>;wave(input:unknown):Promise<void>;resume(input:unknown):Promise<any>}>;
