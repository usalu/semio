import {readLookupsV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts";
const root=process.argv[2]!,mark=Number(process.argv[3]??0);
const rows=readLookupsV1(root).sort((a,b)=>a.atMs-b.atMs);
console.log(JSON.stringify({total:rows.length,steps:rows.slice(mark).map(row=>row.crate+":"+row.outcome).join(" ")}));
