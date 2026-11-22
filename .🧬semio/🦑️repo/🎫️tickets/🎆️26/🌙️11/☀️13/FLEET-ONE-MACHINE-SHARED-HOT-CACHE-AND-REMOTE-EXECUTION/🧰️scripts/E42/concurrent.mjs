import{openOwnerHarnessV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔬️owner-harness/🟨️.mjs";
const[mode,...roots]=process.argv.slice(2),workspaceRoot="/Users/ueli/Documents/semio",late=process.env.E42_LATE==="1";
const h=await openOwnerHarnessV1({workspaceRoot,late});
const c0=process.cpuUsage(),t0=performance.now(),rows=[];
const one=async root=>{const t=performance.now();try{const r=await h.infer(root,{});rows.push({root:root.slice(-60),wall:Math.round(performance.now()-t),cpu:r.cpuMilliseconds,rows:r.rows,digest:r.digest});}catch(e){rows.push({root:root.slice(-60),error:String(e.message).slice(0,160)});}};
if(mode==="par")await Promise.all(roots.map(one));else if(mode.startsWith("lanes:")){const queue=[...roots];await Promise.all(Array.from({length:Number(mode.slice(6))},async()=>{for(let root=queue.shift();root!==undefined;root=queue.shift())await one(root);}));}else for(const root of roots)await one(root);
const u=process.cpuUsage(c0);
console.log("[DEBUG] flights "+JSON.stringify(globalThis.__e42));console.log(JSON.stringify({mode,late,totalWall:Math.round(performance.now()-t0),processCpu:Math.round((u.user+u.system)/1000),rows}));
h.close();
