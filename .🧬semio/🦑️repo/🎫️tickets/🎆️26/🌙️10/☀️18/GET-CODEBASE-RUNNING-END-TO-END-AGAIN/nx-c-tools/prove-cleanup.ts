const root="/Users/ueli/Documents/semio",m=await import(root+"/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📥️canonical/🧹️cleanup/🟦️.ts");
process.on("unhandledRejection",(e:any,p)=>{console.log("UNHANDLED",e?.stack??e);});process.on("uncaughtException",(e:any,origin)=>{console.log("UNCAUGHT",origin,e?.stack??e);});
await m.proveAcceptedPhysicalCleanupV1();console.log("proved");
