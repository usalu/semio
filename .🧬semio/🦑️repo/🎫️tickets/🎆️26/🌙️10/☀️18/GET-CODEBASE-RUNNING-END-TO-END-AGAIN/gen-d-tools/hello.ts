#!/usr/bin/env repo
import{receiveScriptProcessInvocation}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
await receiveScriptProcessInvocation(process.env,async original=>{console.log("hello invocation",typeof original);});
