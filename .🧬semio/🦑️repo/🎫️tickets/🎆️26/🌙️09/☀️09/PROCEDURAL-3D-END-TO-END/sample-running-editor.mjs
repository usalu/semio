/** 🔬 Samples active renderer workers over an individually bounded observation window. */
import {mkdirSync} from "node:fs";
import {connectWorkerProfiler} from "./react-worker-profiler.mjs";
const [port,origin,destination,seconds]=process.argv.slice(2);
mkdirSync(destination,{recursive:true});
const profiler=await connectWorkerProfiler(Number(port),destination,origin,(...args)=>console.log("[DEBUG]",...args));
try{
  profiler.start();
  await new Promise(resolve=>setTimeout(resolve,Number(seconds)*1000));
  await profiler.stop();
} finally {profiler.close();}
