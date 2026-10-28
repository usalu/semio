/** ⚖️ How the memo's relevance rule splits the journal rows of a time window (read-only, straight from the journal log files): `bun memorule.ts <root> <memo-dir> <record.json> <from-seq>`; rows of clone bursts (a record of more than 1,000 `m` file rows) are counted apart; the rest is split into rows that name a relevant entry themselves, rows relevant only because their PARENT directory is relevant (by kind), and irrelevant rows, with examples. */
import {readFileSync,readdirSync} from "node:fs";import {join} from "node:path";
const [root,directory,recordFile,fromText]=process.argv.slice(2) as [string,string,string,string];
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const memo=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/🟦️.ts")),journal=await import(join(L,"🏡️resident/📜️journal/🟦️.ts")),endpoint=await import(join(L,"🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs"));
const record=JSON.parse(readFileSync(recordFile,"utf8")),rel=memo.unpackMemoRelevanceV1(readFileSync(join(directory,record.relevance.id+".rel"))),from=Number(fromText);
const logs=journal.changeJournalDirectoryV1(endpoint.canonicalRootV1(root),process.env),out:Record<string,number>={},examples:Record<string,string[]>={};let burst=0,lost=0,first=Infinity,last=0;
for(const name of readdirSync(logs).filter(name=>name.endsWith(".log")).sort())for(const line of readFileSync(join(logs,name),"utf8").split("\n")){
 if(!line)continue;let item:any;try{item=JSON.parse(line);}catch{continue;}if(item.s<=from)continue;first=Math.min(first,item.t);last=Math.max(last,item.t);
 if(item.l){lost++;continue;}const rows=item.r??[];if(rows.filter((row:any)=>row[0]==="m"&&row[1]===0).length>1000){burst+=rows.length;continue;}
 for(const row of rows){const path=row[2];if(path.startsWith(".🧬semio/🦑️repo/🏡️resident-fence"))continue;const cut=path.lastIndexOf("/"),self=rel.holds(path),parent=!(row[0]==="m"&&row[1]===1)&&rel.holds(cut<0?"":path.slice(0,cut));const key=(self?"self":parent?"parent-only":"irrelevant")+":"+row[0]+row[1];out[key]=(out[key]??0)+1;(examples[key]??=[]).length<4&&examples[key]!.push(path.slice(-100));}
}
console.log(JSON.stringify({window:[new Date(first).toTimeString().slice(0,8),new Date(last).toTimeString().slice(0,8)],burstRows:burst,lost,classes:out},null,1));console.log(JSON.stringify(examples,null,1));
