/** 🔎️ Which journal rows since a memo record's cursor the memo calls relevant, grouped by kind and top directories (read-only): `bun memorows.ts <root> <memo-dir> <record.json>`. */
import {readFileSync} from "node:fs";import {join} from "node:path";
const [root,directory,recordFile]=process.argv.slice(2) as [string,string,string];
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const memo=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/🟦️.ts")),journal=await import(join(L,"🏡️resident/📜️journal/🟦️.ts")),endpoint=await import(join(L,"🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs")),changes=await import(join(root,"🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts"));
const record=JSON.parse(readFileSync(recordFile,"utf8"));
const client=journal.connectChangeJournalV1(endpoint.canonicalRootV1(root),process.env);
const answer=await client.since(changes.parseChangeCursorV1(record.cursor),[""]);
const rel=memo.unpackMemoRelevanceV1(readFileSync(join(directory,record.relevance.id+".rel")));
const hits=answer.rows.filter((r:any)=>memo.firstRelevantChangeV1(rel,[r]));
console.log("rows",answer.rows.length,"relevant",hits.length);
const kinds:Record<string,number>={};for(const r of hits){const k=r[0]+":"+r[1]+":"+(r[2].split("/").slice(0,3).join("/"));kinds[k]=(kinds[k]??0)+1;}
console.log(Object.entries(kinds).sort((a,b)=>b[1]-a[1]).slice(0,40));
console.log(JSON.stringify(hits.slice(0,10)));
process.exit(0);
