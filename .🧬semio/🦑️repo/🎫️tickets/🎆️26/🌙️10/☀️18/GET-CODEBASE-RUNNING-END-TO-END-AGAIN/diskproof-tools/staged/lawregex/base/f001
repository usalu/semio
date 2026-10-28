/** 🧪️ Law: nothing writes below the native cache root `.cache` outside a declared home, the twin of the law for the repository cache root. */
import {spawnSync} from "node:child_process";
import {existsSync,readFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {expect,test} from "bun:test";
import {cacheLedgerV1,requireDeclaredCacheSegmentsV1} from "../../../🟦️.ts";

function workspace():string{
 if(process.env.SEMIO_CACHE_LAW_ROOT)return resolve(process.env.SEMIO_CACHE_LAW_ROOT);
 for(let path=import.meta.dir;;path=dirname(path)){if(existsSync(join(path,"bunfig.toml"))&&existsSync(join(path,"bun.lock")))return path;if(dirname(path)===path)throw Error("Repository root not found");}
}
const root=workspace(),ledger=cacheLedgerV1(),sources=/\.(ts|tsx|mjs|js|json|rs|go|py|sh|toml|yml|yaml)$/u,exempt=/(^|\/)(🧪️tests|🧫️fixtures|node_modules|target|dist)\/|(^|\/)🗑️generated\/|^\.🧬semio\/|^🌎️hub\/|💾️storage\/🧾️ledger\//u;
const literal=/(?<![A-Za-z0-9_])\.cache(?:\/|["'`]\s*,\s*["'`])([A-Za-z0-9_-]+)/gu,reading=/new Set\(|SKIP|[Ss]kip|[Ee]xclude|[Ii]gnore|IGNORE|[Aa]mbient|readdir|isIgnored/u,foreign=/node_modules|~|HOME|XDG|home/u;

function candidates():string[]{
 if(process.env.SEMIO_CACHE_NATIVE_LAW_FILES)return readFileSync(process.env.SEMIO_CACHE_NATIVE_LAW_FILES,"utf8").split("\n").filter(Boolean);
 const result=spawnSync("git",["-c","core.quotepath=off","grep","-lFz",".cache","--",".",":(exclude).🧬semio"],{cwd:root,encoding:"utf8",maxBuffer:64*1024*1024});
 if(result.status!==0&&result.status!==1)throw Error("git grep failed: "+result.stderr);
 return result.stdout.split("\0").filter(Boolean);
}
const comment=(line:string):boolean=>{const head=line.trimStart();return head.startsWith("//")||head.startsWith("*")||head.startsWith("/*")||head.startsWith("#");};

function scan(file:string,text:string):string[]{
 const violations:string[]=[];
 text.split("\n").forEach((line,number)=>{
  if(!line.includes(".cache")||comment(line)||reading.test(line))return;
  for(const match of line.matchAll(literal)){
   if(foreign.test(line.slice(Math.max(0,match.index!-14),match.index!)))continue;
   try{requireDeclaredCacheSegmentsV1([match[1]!],ledger,"native");}catch{violations.push(file+":"+(number+1)+" "+match[1]);}
  }
 });
 return violations;
}

test("no source writes below the native cache root outside a declared home",()=>{
 const files=candidates().filter(file=>sources.test(file)&&!exempt.test(file)&&existsSync(join(root,file)));
 expect(files.length).toBeGreaterThan(0);
 expect(files.flatMap(file=>scan(file,readFileSync(join(root,file),"utf8")))).toEqual([]);
});

test("the scan itself flags undeclared and retired native homes and ignores comments, skip lists, foreign caches and dotted names",()=>{
 const bad=[
  'resolve(this.repoRoot, ".cache", "framework-actor", "tests")',
  '"outputs": ["{workspaceRoot}/.cache/surprise/x"]',
  'join(root,".cache/other","x")',
  'join(root, ".cache", "bazel")'
 ],good=[
  'join(root, ".cache", "command")',
  '"{workspaceRoot}/.cache/b/t/m/nx"',
  'join(root, ".cache", "semio", "typescript")',
  '"tsBuildInfoFile": "node_modules/.cache/pets-typecheck.tsbuildinfo"',
  '// join(root, ".cache", "x")',
  'const SKIP = new Set(["node_modules", ".cache", "target"]);',
  'join(home, ".cache", "uv")',
  'throw Error("print.chart.inference.cache", "incremental")'
 ];
 for(const line of bad)expect(scan("probe.ts",line).length,line).toBe(1);
 for(const line of good)expect(scan("probe.ts",line),line).toEqual([]);
});

test("the door accepts the declared native homes and the repository homes stay on their own root",()=>{
 for(const parts of [["command"],["boot-recovery"],["b","t"],["b","c"],["semio","typescript"],["test-sandbox","st"]])expect(()=>requireDeclaredCacheSegmentsV1(parts,ledger,"native"),parts.join("/")).not.toThrow();
 for(const parts of [["gg"],["impl-exp"],["bazel"],["go"]])expect(()=>requireDeclaredCacheSegmentsV1(parts,ledger,"native"),parts.join("/")).toThrow("Declared cache area required");
 expect(()=>requireDeclaredCacheSegmentsV1(["go","x"],ledger)).not.toThrow();expect(()=>requireDeclaredCacheSegmentsV1(["command"],ledger)).toThrow("Declared cache area required");
});

test("no cache root exists below a module directory: partitions and their images belong to the checkout root",()=>{
 let listing:string[];
 if(process.env.SEMIO_CACHE_STRAY_LAW_ENTRIES)listing=readFileSync(process.env.SEMIO_CACHE_STRAY_LAW_ENTRIES,"utf8").split("\n");
 else{
  const result=spawnSync("git",["-c","core.quotepath=off","ls-files","-z","--others","--ignored","--exclude-standard","--directory"],{cwd:root,encoding:"utf8",maxBuffer:256*1024*1024});
  if(result.status!==0)return;
  listing=result.stdout.split("\0");
 }
 const strays=listing.filter(entry=>entry.replace(/\/$/u,"").split("/").at(-1)===".cache"&&entry!==".cache/"&&!entry.includes("node_modules/")&&!entry.startsWith(".🧬semio/"));
 expect(strays,"stray cache roots (delete them and run the command from the checkout root)").toEqual([]);
},120000);
