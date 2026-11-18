import{readFileSync}from"node:fs";const{all,apply,PN}=await import(import.meta.dir+"/patches4.mjs");
const out=apply(PN,readFileSync("/Users/ueli/Documents/semio/"+PN,"utf8"),all);
const start=out.indexOf("try{let adopted,last="),end=out.indexOf("return await resolveClosureRefsV1(result.projects[name]");
console.log(start>0,end>start,out.includes('if(farm?.exists?.())throw farmJobRefusedV1(name,root,"the owner (the farm gave no record)");return inferProjectBodyV1(projectEnvironment,'),out.includes("stageV1(name,STAGE_VERDICT_V1)"),out.includes("stageV1(name,STAGE_REFS_FARM_V1)"),out.slice(start,end).includes('if(viaFarm())throw farmJobRefusedV1(name,root,"the closure references ('));
