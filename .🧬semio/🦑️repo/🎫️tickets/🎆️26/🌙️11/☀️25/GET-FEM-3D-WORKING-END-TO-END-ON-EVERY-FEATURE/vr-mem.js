const mem=async()=>{
  const c=await page.context().newCDPSession(page); await c.send("HeapProfiler.collectGarbage").catch(()=>{}); const hu=await c.send("Runtime.getHeapUsage").catch(()=>({})); await c.detach();
  const ws=[]; for(const w of page.workers()){ try{ const r=await w.evaluate(()=>self.__wasmMB?self.__wasmMB():null); if(r) ws.push(r); }catch(e){} }
  return {jsMB:Math.round((hu.usedSize||0)/1048576), workers:page.workers().length, wasm:JSON.stringify(ws)};
};
