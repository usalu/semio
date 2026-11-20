import {spawnSync,spawn} from "node:child_process";
const t=performance.now();for(let i=0;i<5;i++)spawnSync("rustc",["--print","cfg"]);console.log("sync rustc --print cfg ms each",((performance.now()-t)/5).toFixed(0));
const t2=performance.now();await Promise.all(Array.from({length:8},()=>new Promise(r=>spawn("rustc",["--print","cfg","--target","wasm32-unknown-unknown"]).on("close",r))));console.log("8 parallel wasm cfg ms total",(performance.now()-t2).toFixed(0));
