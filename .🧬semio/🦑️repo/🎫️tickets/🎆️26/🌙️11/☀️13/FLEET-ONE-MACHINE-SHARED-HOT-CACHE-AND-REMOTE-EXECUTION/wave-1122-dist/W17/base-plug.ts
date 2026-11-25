/** 🧪️ Loads the HEAD copy of the host-capacity decision in place of the checkout's, so a test file shows its verdict on the code before W17. */
import{plugin}from"bun";
import{readFileSync}from"node:fs";
import{join}from"node:path";
const target="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts",staged=join(import.meta.dir,"base","decision.ts");
plugin({name:"w17-base",setup(build){build.onLoad({filter:new RegExp("^"+target.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")+"$","u")},()=>({contents:readFileSync(staged,"utf8"),loader:"ts"}));}});
