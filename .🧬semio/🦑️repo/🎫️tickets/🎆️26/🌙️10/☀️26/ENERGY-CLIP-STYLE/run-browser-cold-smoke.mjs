/** 🧊 Exercise a real Playwright missing-cache error and recover with the existing provisioned browser. */
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import {createRequire} from "node:module";
import {fileURLToPath,pathToFileURL} from "node:url";
import {spawn} from "node:child_process";

const root=process.cwd(),require=createRequire(path.join(root,"package.json"));
const work=path.join(path.dirname(fileURLToPath(import.meta.url)),"🗑️generated/browser-cold");
const adapter=path.join(work,"tools.cjs");
const argument=name=>{const i=process.argv.indexOf(name);return i<0?null:process.argv[i+1]};
if(process.argv.includes("--child")){
  process.env.PLAYWRIGHT_BROWSERS_PATH=path.join(work,"missing-cache");
  const {chromium}=await require(adapter).loadPlaywrightV1(root);
  const {launchBrowser}=await import(pathToFileURL(path.join(root,"🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧰️tools.mjs")));
  const calls=[],cancellation=new AbortController();
  let launches=0,missingError;
  const browser=await launchBrowser({signal:cancellation.signal,launch:async()=>{calls.push("launch");launches++;if(launches===1){try{return await chromium.launch({headless:true})}catch(error){missingError=error;throw error}}return chromium.launch({headless:true,executablePath:argument("--healthy-executable")})},provision:async()=>{
    calls.push("provision");
    if(process.argv.includes("--real-provision")){
      const cold=process.env.PLAYWRIGHT_BROWSERS_PATH;
      process.env.PLAYWRIGHT_BROWSERS_PATH=argument("--healthy-cache");
      try{await require(adapter).playwrightV1(["install","chromium"],{cwd:root,heavy:false,signal:cancellation.signal,stdout:"stderr",onProgress:event=>console.error("[DEBUG] Registered browser provisioning "+event.phase)})}
      finally{process.env.PLAYWRIGHT_BROWSERS_PATH=cold}
    }else console.error("[DEBUG] Mock provisioning: existing pinned browser retained; no download");
  }});
  try{
    assert.match(missingError.message,/^browserType[.]launch: Executable doesn't exist at /m);
    assert.deepEqual(calls,["launch","provision","launch"]);
    const page=await browser.newPage();
    await page.goto("data:text/html,<html lang=en><title>Recovered</title><body>Provisioned browser</body></html>");
    assert.equal(await page.title(),"Recovered");
    await page.screenshot({path:path.join(work,"healthy.png")});
    await fs.writeFile(path.join(work,process.argv.includes("--real-provision")?"registered-smoke.json":"smoke.json"),JSON.stringify({calls,missingError:missingError.message,browserVersion:browser.version(),title:await page.title(),provision:process.argv.includes("--real-provision")?"registered":"mock"},null,2));
    console.log("PASS real missing executable → "+(process.argv.includes("--real-provision")?"registered":"mock")+" provision → healthy existing browser");
  }finally{await browser.close()}
}else{
  await fs.mkdir(work,{recursive:true});
  const {build}=require("esbuild");
  const source=path.join(root,"🧰️framework/🔨️modules/🏃️process/🧰️tools/🎭️playwright/🟦️.ts");
  const bundle=await build({entryPoints:[source],bundle:true,write:false,platform:"node",format:"cjs",packages:"external",logLevel:"silent"});
  await fs.writeFile(adapter,bundle.outputFiles[0].contents);
  const {chromium}=await require(adapter).loadPlaywrightV1(root);
  const healthy=chromium.executablePath();
  await fs.access(healthy);
  const repo=process.env.SEMIO_REPO_BINARY??(await fs.readFile(path.join(root,".🧬semio/🦑️repo/native-repo-path"),"utf8")).trim();
  await new Promise((resolve,reject)=>{
    const child=spawn(repo,["tools","exec","--light","node","--",fileURLToPath(import.meta.url),"--child","--healthy-executable",healthy,"--healthy-cache",process.env.PLAYWRIGHT_BROWSERS_PATH,...(process.argv.includes("--real-provision")?["--real-provision"]:[])],{cwd:root,env:{...process.env,PLAYWRIGHT_BROWSERS_PATH:path.join(work,"missing-cache")},windowsHide:true,stdio:["ignore","pipe","pipe"]});
    child.stdout.on("data",value=>process.stdout.write(value));
    child.stderr.on("data",value=>process.stderr.write(value));
    child.on("error",reject);
    child.on("exit",code=>code===0?resolve():reject(Error("Cold browser smoke exited "+code)));
  });
}
