import {describe,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {ensureWatcherV1,findInstalledWatcherV1} from "../../🔭️watch/🏗️provision/🟨️.mjs";

const SOURCE=join(import.meta.dir,"../../🔭️watch/🦀️rust/🦀️.rs"),PIN=join(import.meta.dir,"../../🔭️watch/🦀️rust/🔣️.json"),TOOLS=join(import.meta.dir,"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts");
/** 🔎️ A Rust toolchain is only needed to build the resident watcher once: an installed one is found without any compiler, and both entries that provision it ask first. */
describe("the installed resident watcher is found before any compiler is opened",()=>{
 test("a located watcher is answered exactly as the provisioning answers it, and no location is no answer",async()=>{
  expect(await findInstalledWatcherV1({sourcePath:SOURCE,pinPath:PIN,locate:()=>undefined})).toBeUndefined();
  const found=await findInstalledWatcherV1({sourcePath:SOURCE,pinPath:PIN,locate:async()=>"/installed/semio-repo-watch"});
  const ensured=await ensureWatcherV1({sourcePath:SOURCE,pinPath:PIN,outputRoot:"/nowhere",locate:()=>"/installed/semio-repo-watch",compile:async()=>{throw Error("compiled");}});
  expect(found).toEqual(ensured);expect(found).toMatchObject({path:"/installed/semio-repo-watch",built:false});
 });
 test("a stale pin is refused here as it is there",async()=>{
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"slw-w161-")));
  try{
   const pin=JSON.parse(readFileSync(PIN,"utf8"));pin.sources[0].sha256="0".repeat(64);
   await Bun.write(join(directory,"pin.json"),JSON.stringify(pin));
   await expect(findInstalledWatcherV1({sourcePath:SOURCE,pinPath:join(directory,"pin.json"),locate:()=>"/x"})).rejects.toThrow(/stale/u);
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 test("both entries that provision the watcher ask for an installed one before they open the compiler store",()=>{
  const text=readFileSync(TOOLS,"utf8");
  for(const entry of["export async function provisionNxTools(","export function withRepositoryResidentWatcherV1<T>("]){
   const from=text.indexOf(entry),body=text.slice(from,from+2500),asked=body.indexOf("residentWatcherInstalledV1(workspace,store)"),compiler=body.indexOf("withRepositoryCompilerStoreV1(")>=0?body.indexOf("withRepositoryCompilerStoreV1("):body.indexOf("provisionNxToolsWithCompilerV1(");
   expect(from).toBeGreaterThan(0);expect(asked).toBeGreaterThan(0);expect(compiler).toBeGreaterThan(asked);
  }
 });
});
