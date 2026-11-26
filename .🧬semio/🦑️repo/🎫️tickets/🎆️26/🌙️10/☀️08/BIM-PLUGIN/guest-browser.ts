import { createServer } from "vite";
import { chromium } from "playwright";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

const workspace = process.cwd();
const root = import.meta.dirname;
const guest = root + "/🗑️generated/guest";
const lifetime = resolve("🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts");
const wire = [
  "🧰️framework/🔨️modules/🧾️app-channel/🎛️command-codec/🟦️.ts",
  "🧰️framework/🔨️modules/🧾️app-channel/🎞️frame-codec/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts",
].map(file => resolve(file));
const policy = JSON.parse(readFileSync(resolve("🌎️hub/🧩️compositions/🖍️draw/🧪️tests/🌐️guest/scenarios.json"), "utf8")).caller;
const contract = JSON.parse(readFileSync(resolve("🌎️hub/🧩️compositions/🏙️bim/🧪️tests/🧫️fixtures/🛂️committed-descriptor/🔣️.json"), "utf8"));
const server = await createServer({root,configFile:false,cacheDir:root+"/🗑️generated/vite",server:{host:"127.0.0.1",port:0,fs:{allow:[workspace]},watch:null},optimizeDeps:{noDiscovery:true,include:[]}, logLevel:"error"});
await server.listen();
const local = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const browser = await chromium.launch({headless:true, executablePath:process.env.CHROME_BIN || (existsSync(local)?local:undefined), args:["--enable-experimental-webassembly-features"]});
try {
  const origin = server.resolvedUrls.local[0];
  const page = await browser.newPage();
  await page.goto(origin + "@vite/client");
  const result = await page.evaluate(async input => await new Promise((resolve, reject) => {
    const worker = new Worker("/guest-worker.ts", { type: "module" });
    worker.onerror = event => { worker.terminate(); reject(new Error(event.message)); };
    worker.onmessage = event => { worker.terminate(); event.data.error ? reject(new Error(event.data.error)) : resolve(event.data); };
    worker.postMessage(input);
  }),{guest,root,lifetime,policy,ids:contract.applicationIds,sequence:contract.guestReadConfigSequence,wire});
  console.log(await browser.version(), result);
} finally {
  await browser.close();
  await server.close();
}
