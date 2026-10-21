import {pathToFileURL} from "node:url";
import {join} from "node:path";
const root=process.cwd(),L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",B=L+"/🏗️build",P="🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript";
const lists:Record<string,string[]>={
 door:[B+"/📜️script.ts",B+"/🌡️host-capacity/🟦️.ts",B+"/🧭️invocation/🟦️.ts"],
 run:[B+"/📜️script.ts",B+"/🌡️host-capacity/🟦️.ts",B+"/🧭️invocation/🟦️.ts",B+"/📥️caller/🟦️.ts",B+"/🚀️bootstrap/⌨️cli/🟦️.ts",L+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts",L+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts",B+"/⚙️operations/🟦️.ts",B+"/🚀️bootstrap/🟦️.ts",B+"/🧭️selection/🟦️.ts",B+"/🔧️tools/💾️storage/🟦️.ts",B+"/🕸️graph/🟦️.ts",B+"/🕸️graph/🦀️rust/🟦️.ts",L+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts"],
 nx:[L+"/⚡️caching/🚀️bootstrap/📜️script.ts",L+"/⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts",B+"/🧾️store-receipt/🟦️.ts"],
 bazel:[B+"/🔧️tools/🏗️bazel/📜️script.ts",B+"/🧾️store-receipt/🟦️.ts"],
};
const names=(process.argv[2]??"run").split("+"),mods=names.flatMap(name=>lists[name]??[]);
const t0=Bun.nanoseconds(),rows:string[]=[];let last=t0;
for(const m of mods){const s=Bun.nanoseconds();try{await import(pathToFileURL(join(root,m)).href);}catch(e){rows.push(`ERR ${m.split("/").slice(-2).join("/")} ${String(e).slice(0,80)}`);}const e=Bun.nanoseconds();rows.push(`${((e-s)/1e6).toFixed(1).padStart(8)} ms  ${m.replace(L+"/","").split("/").slice(-3).join("/")}`);}
console.log(rows.join("\n"));console.log(`${((Bun.nanoseconds()-t0)/1e6).toFixed(1).padStart(8)} ms  total imports (process age ${(Bun.nanoseconds()/1e6).toFixed(0)} ms)`);
