import {resolve} from "node:path";
const R=resolve(import.meta.dir,"../../../../../../.."),F=`${R}/🧰️framework`;
const {BundleScript,ScriptRouter,runScriptMain,withScriptProcessEnvelope,createScriptProcessEnvelope}=await import(`${F}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`);
const {runVitest}=await import(`${F}/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧪️vitest/🟦️.ts`);
const router=new ScriptRouter(`${R}/🏢️semio-tech/🎡️play`,R);
router.register("test",class extends BundleScript{async run(rest:string[]):Promise<void>{await runVitest(this.root,rest,"./🧪️tests/🎚️config/🟦️.ts");}});
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"play-test-native",maximumElapsedMilliseconds:0},{},Date.now()),(original:unknown)=>runScriptMain(router,{invocation:original}));
