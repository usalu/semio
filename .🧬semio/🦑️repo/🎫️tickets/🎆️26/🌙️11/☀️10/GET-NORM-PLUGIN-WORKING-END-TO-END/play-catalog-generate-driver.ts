import {resolve} from "node:path";
import {readFileSync} from "node:fs";
import {BundleScript,ScriptRouter,runScriptMain,withScriptProcessEnvelope,createScriptProcessEnvelope} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {GenerateScript} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";
import {specificCatalogPublicationV1} from "/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts";

const router=new ScriptRouter("/Users/ueli/Documents/semio/✏️s/🧑‍💻dev","/Users/ueli/Documents/semio");
router.register("generate",class extends BundleScript{
 async run():Promise<void>{
  await new GenerateScript(this.root,this.repoRoot,this.invocation,specificCatalogPublicationV1(path=>readFileSync(resolve(this.repoRoot,path),"utf8"))).run([]);
 }
});
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"play-catalog-generate",maximumElapsedMilliseconds:0},{},Date.now()),original=>runScriptMain(router,{invocation:original}));
