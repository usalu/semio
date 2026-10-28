import {createRequire} from "node:module";
const R="/Users/ueli/Documents/semio",ts=createRequire(R+"/")("typescript"),CONFIG=process.argv[2],mode=process.argv[3]??"inv";
const cmd=ts.parseCommandLine(["--noEmit","-p",CONFIG]);
const cache=new Map();let prev;
for(let i=0;i<3;i++){
 const c=ts.getParsedCommandLineOfConfigFile(CONFIG,cmd.options,{...ts.sys,onUnRecoverableConfigFileDiagnostic:d=>{throw Error("cfg")}});
 const host=ts.createIncrementalCompilerHost(c.options,ts.sys),gsf=host.getSourceFile.bind(host);let hits=0,miss=0;
 host.getSourceFile=(fn,lv,oe,sh)=>{const text=ts.sys.readFile(fn);if(text===undefined)return undefined;const h=cache.get(fn);if(h&&h.text===text){hits++;return h.sf}miss++;const sf=gsf(fn,lv,oe,sh);if(sf)cache.set(fn,{text,sf});return sf};
 if(mode==="inv")host.hasInvalidatedResolutions=()=>true;
 const t0=performance.now(),cpu0=process.cpuUsage();
 const b=ts.createSemanticDiagnosticsBuilderProgram(c.fileNames,c.options,host,prev,ts.getConfigFileParsingDiagnostics(c),c.projectReferences);
 const t1=performance.now();
 const st=b.state;
 const pre={changed:st.changedFilesSet?.size,perFile:st.semanticDiagnosticsPerFile?.size,pending:st.affectedFilesPendingEmit?.size,hasOld:!!st.oldSignatures,filesCnt:st.fileInfos?.size};
 const d=[...b.getSyntacticDiagnostics(),...b.getOptionsDiagnostics(),...b.getGlobalDiagnostics()];
 const t2=performance.now();
 const s=b.getSemanticDiagnostics();
 const t3=performance.now(),cpu=process.cpuUsage(cpu0);
 console.log(JSON.stringify({i,hits,miss,buildMs:Math.round(t1-t0),prefix:Math.round(t2-t1),semMs:Math.round(t3-t2),cpuMs:Math.round((cpu.user+cpu.system)/1000),pre,errors:s.length+d.length,rssMb:Math.round(process.memoryUsage.rss()/1048576),reused:b.getProgram().structureIsReused}));
 prev=b;
}
