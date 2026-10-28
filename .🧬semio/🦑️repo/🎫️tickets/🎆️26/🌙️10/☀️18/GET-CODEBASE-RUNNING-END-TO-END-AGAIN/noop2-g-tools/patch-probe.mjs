import fs from "node:fs";
import * as fsns from "node:fs";
import {lstat as lstatNamed} from "node:fs/promises";
import * as fsp from "node:fs/promises";
const seen=[];
const origP=fsp.default??fsp;
console.log("same object default vs ns:", typeof origP.lstat);
const req=await import("node:module").then(m=>m.createRequire(import.meta.url));
const cjs=req("node:fs/promises");
const orig=cjs.lstat;
try{cjs.lstat=async(...a)=>{seen.push(["cjs.lstat",String(a[0])]);return orig(...a)};}catch(e){console.log("assign failed",e.message)}
await lstatNamed("/tmp");
await fsp.lstat("/tmp");
await cjs.lstat("/tmp");
const d=await import("node:fs/promises");await d.lstat("/tmp");
console.log(JSON.stringify(seen));
console.log("process.env proxy assignable:");
try{process.env=new Proxy(process.env,{get(t,k){seen.push(["env",String(k)]);return t[k]}});console.log(process.env.HOME?.length, seen.at(-1))}catch(e){console.log("env assign failed",e.message)}
