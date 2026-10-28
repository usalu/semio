export interface FilterV1{excluded(path:string):boolean}
export function createFilterV1(options?:{rooted?:readonly string[];names?:readonly string[]}):FilterV1;
export function createOwnerIndexV1(roots:Iterable<readonly [string,string]>):{ownerOf(path:string):string|undefined;ownersUnder(prefix:string):string[]};
export interface TaskConfigV1{name:string;command:readonly string[];dependsOn?:readonly string[];outputs?:readonly string[];cacheable?:boolean}
export interface OwnerConfigV1{name:string;root:string;deps?:readonly string[];tasks?:readonly TaskConfigV1[]}
export interface StateConfigV1{owners:readonly OwnerConfigV1[];env?:{paths?:readonly string[];prefixes?:readonly string[]}}
export type ChangeRowV1=readonly [string,number,string];
export interface JournalChangeV1{path:string;kind:"added"|"removed"|"changed";directory:boolean;mode?:number;stamp?:[number,number]}
export type JournalAnswerV1={complete:true;seq:number;changes:JournalChangeV1[]}|{complete:false;reason:"cursor"|"lost";scope?:string};
export interface WorkspaceStateV1{
 configure(config:StateConfigV1):void;reconfigure(config:StateConfigV1):void;load(listing:{directories:readonly (readonly [string,number,number])[];members:readonly (readonly [string,number])[]}):void;
 apply(rows:readonly ChangeRowV1[],at?:number):Promise<number>;lost(scope:string,reason:string,at?:number):number;reconcile(scope?:string):Promise<number>;
 cursor():{instance:string;seq:number};since(from:{instance:string;seq:number}):JournalAnswerV1;dirtyOwnersSince(seq:number):Set<string>;
 refreshOwner(name:string):Promise<string>;closureOf(name:string):string[];taskKey(id:string):Promise<string>;memoCurrent(id:string):Promise<boolean>;stalePlan(id:string):Promise<string[]>;dropCaches():void;
 outputsInvalid(id:string):boolean;clearOutputsInvalid(id:string):void;beginTask(id:string):void;endTask(id:string):void;setEnvironmentSalt(value:string):void;setTrust(value:"exact"|"verify"):void;setGeneration(value:number|null):void;
 memo:{get(id:string):any;set(id:string,entry:unknown):void;delete(id:string):boolean;size():number};
 readonly owners:Map<string,any>;readonly tasks:Map<string,any>;readonly members:Map<string,number>;readonly directories:Map<string,[number,number]>;readonly stats:Record<string,number>;
 readonly seq:number;readonly lostSeq:number;readonly envSeq:number;readonly journalFloor:number;readonly trust:"exact"|"verify";readonly generation:number|null;
 ownerOf(path:string):string|undefined;topological(ids:readonly string[]):string[];
}
export function createWorkspaceStateV1(options:{root:string;instance:string;policy:any;filter:FilterV1;fence?:string;now?:()=>number}):WorkspaceStateV1;
export function outputSignatureV1(root:string,outputs:readonly string[],maximumEntries:number):Promise<string|undefined>;
