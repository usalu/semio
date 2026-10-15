const flags={selected:"--selected-graph-gate",graph:"--graph-gate",outputs:"--nx-hook-gate",publication:"--nx-publication-owner-gate"},mode=process.argv[2]as keyof typeof flags;
if(!Object.hasOwn(flags,mode??""))throw Error("Expected selected, graph, outputs or publication current-SDK gate");
if(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION!==undefined)throw Error("Queued standalone gate refuses an inherited invocation");
process.argv.push(flags[mode]);
await import("../qualify/📜️script.ts");
