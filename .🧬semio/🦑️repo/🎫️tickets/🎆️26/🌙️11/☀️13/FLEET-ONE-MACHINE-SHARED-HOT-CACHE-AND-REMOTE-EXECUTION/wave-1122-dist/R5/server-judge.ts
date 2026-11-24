/** ⚖️ R5: runs the server's own staleness judge (`judgeStalenessV1`) on a read-only snapshot (`snapshot.py`) with the snapshot's tree digests; writes the verdict per entry. Touches nothing in the store. */
import {readFileSync,writeFileSync} from "node:fs";
import {judgeStalenessV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";

const [input,output,tree="tree"]=process.argv.slice(2) as [string,string,string?];
const snap=JSON.parse(readFileSync(input,"utf8")) as {takenAt:number;entries:{key:string;meta:{storedAt:number;refs:string[];declaration?:any}|null}[];tree:Record<string,string|null>;play:Record<string,string|null>};
const digests=tree==="play"?snap.play:snap.tree;
const entries=snap.entries.filter(entry=>entry.key.split("/")[1]!=="cas").map(entry=>({key:entry.key,storedAt:entry.meta?.storedAt??0,refs:entry.meta?.refs??[],declaration:entry.meta?.declaration}));
const verdicts=await judgeStalenessV1(entries,{now:snap.takenAt,graceMilliseconds:600000,digest:async path=>digests[path]??null});
writeFileSync(output,JSON.stringify(Object.fromEntries(verdicts)));
const counts:Record<string,number>={};for(const v of verdicts.values()){const k=v.state+(v.reason?":"+v.reason:"");counts[k]=(counts[k]??0)+1;}
console.log(JSON.stringify(counts));
