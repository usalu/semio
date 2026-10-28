import{runOwnedCommand,OwnedCommandFailure}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
const lines:string[]=[];
try{await runOwnedCommand("repo",["--version"],"/Users/ueli/Documents/semio","probe",60000,{onLine:l=>{lines.push(l);}});console.log("NESTED OK",JSON.stringify(lines.slice(0,3)));}
catch(e){console.log("NESTED FAILED",e instanceof OwnedCommandFailure?JSON.stringify(e.outcome):String(e),JSON.stringify(lines));}
