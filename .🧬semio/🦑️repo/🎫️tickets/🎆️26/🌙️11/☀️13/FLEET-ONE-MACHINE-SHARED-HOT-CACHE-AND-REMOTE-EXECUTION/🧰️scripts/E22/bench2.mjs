import {opendir,lstat,readFile} from "node:fs/promises";
import {join} from "node:path";
const gen="/Users/ueli/Documents/semio/.cache/b/t/m/nx/"+process.argv[2];
const base="/Users/ueli/Documents/semio/.cache/b/t/m/sdk/e921066ebbb960dda44a0bc1564a6691b8757dae055b440ccce291e9c918f62e";
const index=JSON.parse(await readFile(join(base,".semio-sdk-source.json"),"utf8"));
const dirs=index.entries.filter(e=>e.kind==="directory");
const t=async(name,f)=>{const s=performance.now();await f();console.log(name,Math.round(performance.now()-s),"ms")};
await t("opendir all "+dirs.length+" dirs",async()=>{for(const d of dirs){const h=await opendir(join(gen,d.path));for await(const e of h){}}});
await t("lstat all entries",async()=>{for(const e of index.entries)await lstat(join(gen,e.path),{bigint:true})});
