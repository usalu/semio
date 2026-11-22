/** 🙈️ Law: what the doctor listed as undeclared is declared now: an ignored name or path, a retired scratch directory and the files of the operating system inside containers; the classification equals an independent Python reading of the same ledger, and the collector's undeclared scan finds only what nobody declared. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CACHE_ROOT_IDS_V1,cacheLedgerV1,cacheRootDirectoryV1,classifyCachePathV1,type CacheRootIdV1} from "../../🟦️.ts";
import {undeclaredEntriesV1} from "../../../../../🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts";

const ledger=cacheLedgerV1();
let workspace="";
const touch=(root:CacheRootIdV1,path:string,directory=false):void=>{const full=join(cacheRootDirectoryV1(workspace,ledger,root),...path.split("/"));if(directory)mkdirSync(full,{recursive:true});else{mkdirSync(join(full,".."),{recursive:true});writeFileSync(full,"x");}};
const cases:[CacheRootIdV1,string,string][]=[
 ["native","b/image-relocation.json","ignored"],["native","b/zzz.json","undeclared"],["native","test-admission","retired"],["native","b","container"],["native",".DS_Store","ignored"],["native","b/.DS_Store","ignored"],
 ["native","b/t/m/.DS_Store","area"],["repository","tools/.DS_Store","ignored"],["repository","tools/other","undeclared"],["repository","tools/ms-playwright","area"],["repository","tools","container"],["rust",".DS_Store","ignored"]
];
const python=():string[]=>{
 const code="import json,sys\nd=json.loads(sys.stdin.read())\nout=[]\nfor root,path,home in d['cases']:\n parts=path.split('/');ig=d['ignored'][root]\n homes=[(a['directory'].split('/'),'area') for a in d['areas'] if a['root']==root and a.get('directory') and a['kind']!='nested']+[(r['directory'].split('/'),'retired') for r in d['retirements'] if r['root']==root]\n pre=lambda p,c:len(p)<=len(c) and c[:len(p)]==p\n if path in ig:k='ignored'\n else:\n  hit=[kind for h,kind in homes if pre(h,parts)]\n  k=hit[0] if hit else ('container' if any(pre(parts,h) for h,_ in homes) else ('ignored' if parts[-1] in ig else 'undeclared'))\n out.append(k)\nprint(json.dumps(out))";
 const run=spawnSync("python3",["-I","-c",code],{input:JSON.stringify({cases,ignored:Object.fromEntries(CACHE_ROOT_IDS_V1.map(root=>[root,ledger.roots[root].ignored])),areas:ledger.areas,retirements:ledger.retirements}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);
};
beforeAll(()=>{
 workspace=mkdtempSync(join(tmpdir(),"ledger-ignored-"));
 touch("native","b/image-relocation.json");touch("native","test-admission",true);touch("native","b/t/m/data",true);touch("native",".DS_Store");
 touch("repository","tools/.DS_Store");touch("repository","tools/ms-playwright/x");
});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the declared ledger classifies the three entries the doctor listed as declared and equals the independent Python reading",()=>{
 expect(cases.map(([root,path])=>classifyCachePathV1(ledger,root,path).kind)).toEqual(cases.map(row=>row[2]));
 expect(python()).toEqual(cases.map(row=>row[2]));
});

test("the undeclared scan of a workspace holding exactly those entries finds nothing, and a foreign entry beside them is found",async()=>{
 expect(await undeclaredEntriesV1(workspace,ledger,"native")).toEqual([]);
 expect(await undeclaredEntriesV1(workspace,ledger,"repository")).toEqual([]);
 touch("native","b/zzz.json");touch("repository","tools/other",true);
 expect(await undeclaredEntriesV1(workspace,ledger,"native")).toEqual(["b/zzz.json"]);
 expect(await undeclaredEntriesV1(workspace,ledger,"repository")).toEqual(["tools/other"]);
});
