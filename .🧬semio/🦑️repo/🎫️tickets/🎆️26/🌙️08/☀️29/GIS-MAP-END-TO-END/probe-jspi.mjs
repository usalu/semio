import { chromium, firefox, webkit, devices } from "playwright";
import { mkdirSync, mkdtempSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
const [engine, ...profileNames] = process.argv.slice(2);
const types = { chromium, firefox, webkit };
function capability() {
  return (async () => {
    const w = globalThis.WebAssembly;
    const support = { suspending: typeof w?.Suspending, promising: typeof w?.promising };
    if (support.suspending !== "function" || support.promising !== "function") return support;
    const bytes = new Uint8Array([0,97,115,109,1,0,0,0,1,5,1,96,0,1,127,2,12,1,3,101,110,118,4,115,108,111,119,0,0,3,2,1,0,7,7,1,3,114,117,110,0,1,10,6,1,4,0,16,0,11]);
    try {
      let returned = false;
      const { instance } = await w.instantiate(bytes, { env: { slow: new w.Suspending(async () => {
        await new Promise(resolve => setTimeout(resolve, 10));
        returned = true;
        return 42;
      }) } });
      const answer = w.promising(instance.exports.run)();
      const deferred = !returned;
      return { ...support, deferred, value: await answer };
    } catch (error) {
      return { ...support, error: String(error) };
    }
  })();
}
const stop=setTimeout(() => process.exit(124), 20000);
const env={...process.env};
if(engine==="firefox") {
  const output=fileURLToPath(new URL("./🗑️generated/",import.meta.url));
  mkdirSync(output,{recursive:true});
  env.MOZ_APP_DATA=mkdtempSync(join(output,"jspi-firefox-"));
  env.MOZ_LOCAL_APP_DATA=join(env.MOZ_APP_DATA,"local");
  mkdirSync(env.MOZ_LOCAL_APP_DATA);
}
const browser=await types[engine].launch({
  headless:true,timeout:8000,
  env,
  ...(engine==="chromium" && process.env.GIS_CHROMIUM_CHANNEL ? {channel:process.env.GIS_CHROMIUM_CHANNEL} : {}),
});
try {
  for (const profile of profileNames) {
    const context=await browser.newContext(devices[profile]);
    try {
      const page=await context.newPage();
      const result=await page.evaluate(async ({source})=>{
        const getFeatures=Function("return ("+source+")")();
        const url=URL.createObjectURL(new Blob(["("+source+")().then(x=>postMessage(x),e=>postMessage({error:String(e)}))"],{type:"text/javascript"}));
        const worker=new Worker(url);
        let timer;
        try {
          const workerResult=new Promise(resolve=>{
            timer=setTimeout(()=>resolve({error:"worker timeout"}),2500);
            worker.onmessage=event=>resolve(event.data);
            worker.onerror=event=>resolve({error:event.message});
          });
          return {window:await getFeatures(),worker:await workerResult,ua:navigator.userAgent,touch:navigator.maxTouchPoints};
        }finally{
          clearTimeout(timer);
          worker.terminate();
          URL.revokeObjectURL(url);
        }
      },{source:capability.toString()});
      console.log(JSON.stringify({engine,version:browser.version(),profile,...result}));
    }finally{await context.close();}
  }
}finally{
  clearTimeout(stop);
  await browser.close();
}
