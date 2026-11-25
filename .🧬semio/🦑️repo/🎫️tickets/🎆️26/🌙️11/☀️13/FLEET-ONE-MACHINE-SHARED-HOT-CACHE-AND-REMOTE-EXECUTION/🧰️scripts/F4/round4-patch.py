import re
base="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/"
def sub(p,old,new,n=1):
    s=open(base+p).read(); assert s.count(old)==n,(p,old[:70],s.count(old)); open(base+p,'w').write(s.replace(old,new))
sch='🧬️schema/🔣️.json'
sub(sch,'"priority": {"type": "integer", "minimum": 1, "maximum": 100},','"priority": {"type": "integer", "minimum": 1, "maximum": 100},\n        "inodeRank": {"type": "integer", "minimum": 1, "maximum": 100},')
for i,(w,pr) in enumerate([("dependency-generations",50),("cargo-incremental",10),("tool-generations",20),("nx-native-data",25),("closure-layers",30)],1):
    sub(sch,'{"id": "%s", "priority": %d,'%(w,pr),'{"id": "%s", "priority": %d, "inodeRank": %d,'%(w,pr,i))
t='🟦️.ts'
sub(t,'export type DiskWriterRowV1=Readonly<{id:string;priority:number;','export type DiskWriterRowV1=Readonly<{id:string;priority:number;inodeRank?:number;')
sub(t,'ids.add(row.id);priorities.add(row.priority);','ids.add(row.id);priorities.add(row.priority);\n  if(row.inodeRank!==undefined){if(ranks.has(row.inodeRank))throw Error("Unique inode ranks required / Eindeutige Inode-Ränge erforderlich: "+row.id);ranks.add(row.inodeRank);}')
sub(t,'const ids=new Set<string>(),priorities=new Set<number>();','const ids=new Set<string>(),priorities=new Set<number>(),ranks=new Set<number>();')
sub(t,'sweep?:boolean;steps:Array','sweep?:boolean;scarce?:"inodes";steps:Array')
sub(t,'.sort((first,second)=>first.row.priority-second.row.priority);\n for(const writer of order){','.sort((first,second)=>input.scarce==="inodes"?(first.row.inodeRank??Infinity)-(second.row.inodeRank??Infinity)||first.row.priority-second.row.priority:first.row.priority-second.row.priority);\n for(const writer of order){')
sub(t,'export async function sweepDiskV1(input:Readonly<{facts():DiskFactsV1|undefined;needBytes:number;','export async function sweepDiskV1(input:Readonly<{facts():DiskFactsV1|undefined;needBytes:number;scarce?:"inodes";')
sub(t,'startKey:"recovery.start-need",sweep:true,steps}','startKey:"recovery.start-need",sweep:true,scarce:input.scarce,steps}')
r='🔌️real/🟦️.ts'
sub(r,'export async function runLedgerCollectorV1(workspace:string,environment:Readonly<Record<string,string|undefined>>,signal?:AbortSignal,reclaimBytes?:number):Promise<void>{','export async function runLedgerCollectorV1(workspace:string,environment:Readonly<Record<string,string|undefined>>,signal?:AbortSignal,reclaimBytes?:number,areas?:string):Promise<void>{')
sub(r,'"cache","gc",...reclaimBytes','"cache","gc",...areas===undefined?[]:["--area="+areas],...reclaimBytes')
sub(r,'ledger?:(signal?:AbortSignal,reclaimBytes?:number)=>Promise<void>','ledger?:(signal?:AbortSignal,reclaimBytes?:number,areas?:string)=>Promise<void>')
sub(r,'let ledgerRan=false;','const ledgerRan=new Set<string>();')
sub(r,'const ledger=async(signal?:AbortSignal,reclaimBytes?:number):Promise<number>=>{if(ledgerRan)return 0;ledgerRan=true;await(context.ledger??((abort,need)=>runLedgerCollectorV1(context.workspace,context.environment,abort,need)))(signal,reclaimBytes);return 0;};','const ledger=async(signal?:AbortSignal,reclaimBytes?:number,areas?:string):Promise<number>=>{const key=areas??"*";if(ledgerRan.has(key))return 0;ledgerRan.add(key);await(context.ledger??((abort,need,only)=>runLedgerCollectorV1(context.workspace,context.environment,abort,need,only)))(signal,reclaimBytes,areas);return 0;};')
sub(r,'collect:({needBytes,signal})=>ledger(signal,needBytes)}:base;','collect:({needBytes,signal})=>ledger(signal,needBytes,/--area=([a-z0-9,-]+)/u.exec(row.command)?.[1])}:base;')
