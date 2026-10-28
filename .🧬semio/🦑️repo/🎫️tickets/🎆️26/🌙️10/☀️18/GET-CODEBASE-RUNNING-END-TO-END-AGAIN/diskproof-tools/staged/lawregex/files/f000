import {spawnSync} from "node:child_process";
import {existsSync,readFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {expect,test} from "bun:test";
import budgetPolicy from "../../../🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {cacheAreaBudgetBytesV1,cacheLedgerV1,requireDeclaredCacheSegmentsV1} from "../../🟦️.ts";

function workspace():string{
 if(process.env.SEMIO_CACHE_LAW_ROOT)return resolve(process.env.SEMIO_CACHE_LAW_ROOT);
 for(let path=import.meta.dir;;path=dirname(path)){if(existsSync(join(path,"bunfig.toml"))&&existsSync(join(path,"bun.lock")))return path;if(dirname(path)===path)throw Error("Repository root not found");}
}
const root=workspace(),ledger=cacheLedgerV1(),marker="⚡️cache",sources=/\.(ts|tsx|mjs|js|json|rs|go|py|sh|toml|yml|yaml)$/u,exempt=/(^|\/)(🧪️tests|🧫️fixtures|node_modules|target|dist)\/|(^|\/)🗑️generated\/|^\.🧬semio\/|📚️library\/⚡️caching\/🟦️\.ts$/u;

function candidates():string[]{
 if(process.env.SEMIO_CACHE_LAW_FILES)return readFileSync(process.env.SEMIO_CACHE_LAW_FILES,"utf8").split("\n").filter(Boolean);
 const result=spawnSync("git",["-c","core.quotepath=off","grep","-lFz",marker,"--",".",":(exclude).🧬semio"],{cwd:root,encoding:"utf8",maxBuffer:64*1024*1024});
 if(result.status!==0&&result.status!==1)throw Error("git grep failed: "+result.stderr);
 return result.stdout.split("\0").filter(Boolean);
}
const files=candidates().filter(file=>sources.test(file)&&!exempt.test(file)&&!file.includes("💾️storage/🧾️ledger/")&&existsSync(join(root,file)));
const writers=new Set(ledger.areas.flatMap(area=>area.writers));
const comment=(line:string,index:number):boolean=>{const head=line.trimStart();return head.startsWith("//")||head.startsWith("*")||head.startsWith("/*")||head.startsWith("#")||line.lastIndexOf("//",index)>=0&&!line.slice(0,line.lastIndexOf("//",index)).includes("\"");};
const literal=/⚡️cache(?:\/([^"'`\s,)}*{<\\$\[(|]+)|["'`]\s*,\s*["'`]([^"'`\\$]+)["'`]|["'`]\s*\)\s*\.join\(\s*["'`]([^"'`\\$]+)["'`])/gu;
const dynamic=/⚡️cache["'`][ \t]*,[ \t]*(?![ \t"'`])/u,importing=/^\s*(import|export)\b[^\n]*\bfrom\b/u;
const joined=(line:string,index:number):boolean=>/(?:join|Join)\(/u.test(line.slice(0,index));

test("every writer named by the ledger exists",()=>{
 for(const area of ledger.areas)for(const writer of area.writers)expect(existsSync(join(root,writer)),area.id+": "+writer).toBe(true);
});

function scan(file:string,text:string):string[]{
 const violations:string[]=[];
 text.split("\n").forEach((line,number)=>{
  if(!line.includes(marker)||importing.test(line))return;
  for(const match of line.matchAll(literal)){
   if(comment(line,match.index!))continue;
   const slash=match[1]!==undefined,path=(match[1]??match[2]??match[3])!;
   if(!slash&&!joined(line,match.index!))continue;
   const parts=path.split("/").filter(Boolean);
   if(!parts.length||parts.some(part=>part.includes("…")))continue;
   try{requireDeclaredCacheSegmentsV1(parts,ledger);}catch{violations.push(file+":"+(number+1)+" "+parts.join("/"));}
  }
  const index=line.search(dynamic);
  if(index>=0&&!comment(line,index)&&joined(line,index)&&!writers.has(file))violations.push(file+":"+(number+1)+" dynamic segment from an undeclared writer");
 });
 return violations;
}

test("no source writes below the cache root outside a declared home",()=>{
 expect(files.length).toBeGreaterThan(0);
 expect(files.flatMap(file=>scan(file,readFileSync(join(root,file),"utf8")))).toEqual([]);
});

test("the scan itself flags undeclared, retired and dynamic homes and ignores comments, lists and imports",()=>{
 const bad=[
  'join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "e1", "x")',
  'const dir = `${root}/.🧬semio/🦑️repo/⚡️cache/leases/x`;',
  'return base.join("⚡️cache").join("surprise")',
  'filepath.Join(meta, "⚡️cache", hex.EncodeToString(h[:]))',
  '"outputs": ["{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/tools/nx-tooling/x"]'
 ],good=[
  'join(repoRoot, "⚡️cache", "go", "x")',
  '// join(repoRoot, "⚡️cache", "e1")',
  'const SKIP = new Set(["node_modules", "⚡️cache", ".venv"]);',
  'import {x} from "../⚡️cache/🟦️.ts";',
  '"!{workspaceRoot}/**/⚡️cache/**/*"',
  '".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/x"',
  '.join("⚡️cache").join("🎛️dashboard")'
 ];
 for(const line of bad)expect(scan("probe.ts",line).length,line).toBe(1);
 for(const line of good)expect(scan("probe.ts",line),line).toEqual([]);
 expect(scan([...writers][0]!,'filepath.Join(meta, "⚡️cache", hex.EncodeToString(h[:]))')).toEqual([]);
});

test("the runtime door refuses undeclared homes before any path is built",()=>{
 const text=readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts"),"utf8");
 expect(text).toMatch(/export function repoCacheDirectory\(repoRoot: string, \.\.\.segments: string\[\]\): string \{\s*requireDeclaredCacheSegmentsV1\(segments\);/u);
 expect(()=>requireDeclaredCacheSegmentsV1(["e1"],ledger)).toThrow("Declared cache area required");
});

test("the Nx task cache is the declared area and Nx trims it at exactly its budget",()=>{
 const area=ledger.areas.find(row=>row.id==="nx-artifacts")!,configuration=JSON.parse(readFileSync(join(root,"nx.json"),"utf8")) as {cacheDirectory:string;maxCacheSize?:string};
 expect(configuration.cacheDirectory).toBe([...budgetPolicy.partitions.rust.directory.split("/"),"m",...area.directory!.split("/")].join("/"));
 const match=/^(?<size>\d+(?:\.\d+)?)\s?(?<unit>[KMG]?B)?$/u.exec(configuration.maxCacheSize??""),factor={KB:1024,MB:1024**2,GB:1024**3,B:1,undefined:1} as Record<string,number>;
 expect(match,"nx.json maxCacheSize").not.toBeNull();
 expect(Number(match!.groups!.size)*factor[String(match!.groups!.unit)]!).toBe(cacheAreaBudgetBytesV1(ledger,area)!);
});

test("the Bazel disk cache share and the Cargo budget agree with the ledger",()=>{
 const share=(value:unknown):number|undefined=>{if(value&&typeof value==="object"){for(const[key,child]of Object.entries(value)){if(key==="productsShareBasisPoints"&&typeof child==="number")return child;const found=share(child);if(found!==undefined)return found;}}return undefined;};
 const bazel=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧬️schema/🔣️.json"),"utf8")),area=ledger.areas.find(row=>row.id==="bazel-actions")!;
 expect("basisPoints" in area.budget!?area.budget.basisPoints:-1).toBe(share(bazel)!);
 const caching=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"),"utf8")) as {storage:{cargo:{budgetBytes:number;incremental:{budgetBytes:number}}}},cargo=cacheAreaBudgetBytesV1(ledger,ledger.areas.find(row=>row.id==="cargo-store")!);
 expect(caching.storage.cargo.budgetBytes).toBe(cargo!);
 expect(caching.storage.cargo.incremental.budgetBytes).toBeLessThanOrEqual(cargo!);
});

test("only live code reaches the real door",async()=>{
 if(process.env.SEMIO_CACHE_LAW_LIVE!=="1")return;
 const{repoCacheDirectory}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts"));
 expect(repoCacheDirectory("/r","go","x")).toBe(join("/r",".🧬semio","🦑️repo","⚡️cache","go","x"));
 expect(()=>repoCacheDirectory("/r","e1")).toThrow("Declared cache area required");
});
