/** Rebuild attribution: a graph generation whose implementation digest differs from its parent's must have a cause, a promotion of the sealed implementation inside the window between the two manifests. Streams small files only (manifests are read one at a time, only their header fields are kept). */
import{readFileSync,readdirSync,statSync}from"node:fs";import{join}from"node:path";
const root=process.argv[2]??process.cwd(),manifests=join(root,".cache/b/g/m/generations/m"),store=join(root,".🧬semio/🦑️repo/⚡️cache/implementation");
type Row={generation:number;implementation:string;parent:string;digest:string;mtimeMs:number};
const rows:Row[]=[];
for(const name of readdirSync(manifests)){if(!name.endsWith(".json"))continue;const path=join(manifests,name),value=JSON.parse(readFileSync(path,"utf8")) as{generation:number;implementation:string;parent:string|null};rows.push({generation:value.generation,implementation:value.implementation,parent:value.parent??"",digest:name.slice(0,-5),mtimeMs:statSync(path).mtimeMs});}
rows.sort((a,b)=>a.generation-b.generation);
const promotions:{atMs:number;digest:string;added:number;removed:number;changed:number}[]=[];
try{for(const entry of readdirSync(store)){try{for(const line of readFileSync(join(store,entry,"promotions.jsonl"),"utf8").split("\n").filter(Boolean))promotions.push(JSON.parse(line));}catch{}}}catch{}
const byDigest=new Map(rows.map(row=>[row.digest,row]));
for(const row of rows){
 const parent=byDigest.get(row.parent),moved=parent!==undefined&&parent.implementation!==row.implementation;
 const inWindow=parent?promotions.filter(event=>event.atMs>parent.mtimeMs-1000&&event.atMs<=row.mtimeMs+1000):[];
 console.log(JSON.stringify({generation:row.generation,implementation:row.implementation.slice(0,12),parentGeneration:parent?.generation??null,implementationMoved:moved,promotionsInWindow:inWindow.length,verdict:!moved?"implementation stable":inWindow.length?"explained by a sealed promotion":"UNEXPLAINED (no promotion in the window)"}));
}
console.log(JSON.stringify({promotionsRecorded:promotions.length,rss:Math.round(process.memoryUsage().rss/1048576)}));
