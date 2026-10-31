const fs=require("fs"),path=require("path");
const [label,startText,endText]=process.argv.slice(2),start=Date.parse(startText),end=Date.parse(endText);
if(!/^[a-z0-9-]+$/.test(label??"")||!Number.isFinite(start)||end-start!==360000)throw Error("Closed six-minute native window required");
let cancelled=false;const cancel=()=>{cancelled=true};process.once("SIGINT",cancel);
(async()=>{
 console.log("[DEBUG] native-window-ready "+JSON.stringify({at:new Date().toISOString(),label,start:startText,end:endText}));
 let reported=0;
 while(Date.now()<start){if(cancelled){process.exitCode=130;return;}if(Date.now()-reported>=30000){reported=Date.now();console.log("[DEBUG] native-window-wait "+JSON.stringify({label,remainingMilliseconds:start-Date.now()}));}await new Promise(resolve=>setTimeout(resolve,Math.min(1000,start-Date.now())));}
 process.removeListener("SIGINT",cancel);if(cancelled){process.exitCode=130;return;}
 if(Date.now()>=end)throw Error("Authored Source quiet window expired before native admission");
 const receipt={label,start:startText,end:endText,actualStart:new Date().toISOString()};
 fs.mkdirSync(path.join(__dirname,"🗑️generated"),{recursive:true});fs.writeFileSync(path.join(__dirname,"🗑️generated",label+"-window.json"),JSON.stringify(receipt)+"\n");
 console.log("[DEBUG] native-window-start "+JSON.stringify(receipt));
 process.argv=[process.argv[0],path.join(__dirname,"native-layout-acceptance.cjs"),label];require("./native-layout-acceptance.cjs");
})().catch(error=>{console.error("[DEBUG] "+error.stack);process.exitCode=1;});