import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧺️ The declared retention of one graph memo directory; the schema file is its single source. */
export const GRAPH_MEMO_RETENTION_V1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)) as Readonly<{version:1;recordsKept:number;minimumRecordsKept:number;maximumDirectoryBytes:number;graceMilliseconds:number}>);
/** 🧺️ One slice record of a memo directory with the names of the files it keeps alive. */
export type GraphMemoRecordV1=Readonly<{name:string;bytes:number;mtimeMs:number;owned:readonly string[]}>;
/** 🧺️ One file of a memo directory that is not a record. */
export type GraphMemoFileV1=Readonly<{bytes:number;mtimeMs:number}>;
/** 🧺️ What a collection removes and what stays: `overBudget` counts the records the byte budget (not the count) removed. */
export type GraphMemoCollectionV1=Readonly<{removeRecords:readonly string[];removeFiles:readonly string[];keptRecords:number;keptBytes:number;removedBytes:number;overBudget:number}>;

const byAge=(left:GraphMemoRecordV1,right:GraphMemoRecordV1):number=>right.mtimeMs-left.mtimeMs||(left.name<right.name?-1:left.name>right.name?1:0);

/** 📏️ The pure plan of a memo collection: the newest `recordsKept` records stay, then the oldest kept record goes while the directory (kept records plus the files they name, each counted once) exceeds the byte budget and more than `minimumRecordsKept` (at least the newest one) stay; every file no kept record names goes once it is older than the grace. */
export function planGraphMemoCollectionV1(records:readonly GraphMemoRecordV1[],files:Readonly<Record<string,GraphMemoFileV1>>,now:number,retention:Pick<typeof GRAPH_MEMO_RETENTION_V1,"recordsKept"|"minimumRecordsKept"|"maximumDirectoryBytes"|"graceMilliseconds">=GRAPH_MEMO_RETENTION_V1):GraphMemoCollectionV1{
 const order=[...records].sort(byAge),kept=order.slice(0,retention.recordsKept),dropped=order.slice(retention.recordsKept);
 const named=(rows:readonly GraphMemoRecordV1[]):Set<string>=>new Set(rows.flatMap(row=>row.owned));
 const bytes=(rows:readonly GraphMemoRecordV1[]):number=>rows.reduce((sum,row)=>sum+row.bytes,0)+[...named(rows)].reduce((sum,name)=>sum+(files[name]?.bytes??0),0);
 let overBudget=0;
 while(kept.length>Math.max(retention.minimumRecordsKept,1)&&bytes(kept)>retention.maximumDirectoryBytes){dropped.push(kept.pop()!);overBudget++;}
 const alive=named(kept),removeFiles=Object.keys(files).filter(name=>!alive.has(name)&&now-files[name]!.mtimeMs>retention.graceMilliseconds).sort();
 const removedBytes=dropped.reduce((sum,row)=>sum+row.bytes,0)+removeFiles.reduce((sum,name)=>sum+files[name]!.bytes,0);
 return{removeRecords:dropped.map(row=>row.name).sort(),removeFiles,keptRecords:kept.length,keptBytes:bytes(kept),removedBytes,overBudget};
}

const fill=(template:string,values:Readonly<Record<string,string|number>>):string=>template.replace(/\{(\w+)\}/gu,(_all,key:string)=>String(values[key]));
const mib=(value:number):number=>Math.round(value/1048576*10)/10;

/** 📣️ The report line of a byte-driven collection in one language of the schema. */
export function formatGraphMemoCollectionV1(plan:GraphMemoCollectionV1,language:"en"|"de",retention:Pick<typeof GRAPH_MEMO_RETENTION_V1,"maximumDirectoryBytes">=GRAPH_MEMO_RETENTION_V1):string{
 return fill(schema.definitions.Messages.const.collected[language],{budgetMib:mib(retention.maximumDirectoryBytes),records:plan.removeRecords.length,files:plan.removeFiles.length,freedMib:mib(plan.removedBytes),kept:plan.keptRecords,keptMib:mib(plan.keptBytes)});
}
