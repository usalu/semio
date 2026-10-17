const ws=new WebSocket(process.argv[2]!);let id=0;
ws.onmessage=e=>{const m=JSON.parse(String(e.data));if(m.method==="Debugger.scriptParsed")return;console.log("<<",JSON.stringify(m).slice(0,2500));};
ws.onopen=()=>{const s=(method:string,params:object={})=>{ws.send(JSON.stringify({id:++id,method,params}));};
s("Inspector.enable");s("Runtime.enable");s("Inspector.initialized");
s("Runtime.evaluate",{expression:"typeof Bun+' '+process.pid+' '+process.uptime()"});
s("ScriptProfiler.startTracking",{includeSamples:true});
setTimeout(()=>s("ScriptProfiler.stopTracking"),5000);
setTimeout(()=>{console.log("done");process.exit(0)},9000);};
