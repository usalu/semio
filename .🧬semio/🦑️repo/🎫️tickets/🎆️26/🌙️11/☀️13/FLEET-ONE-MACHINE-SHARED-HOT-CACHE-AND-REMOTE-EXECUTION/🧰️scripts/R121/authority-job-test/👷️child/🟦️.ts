/** 👷️ The farm child of the model: computes the assembly authority of one owner like the authority job and answers it with the rows it read. @see ../🟦️.ts */
const B=process.env.R121_B!;
const inference=await import(B+"/🕸️graph/📥️inference/🟨️.mjs") as any;await inference.libraryBootstrap;
const PN=await import(B+"/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs") as any;
const{root,nativeRoot}=JSON.parse(process.env.R121_JOB!);
const reply=await PN.authorityOfOwnerV1({workspaceRoot:root,projectsByRoot:new Map([["owner","owner"],["dependency","dependency"]]),componentRoots:undefined,nativeCache:new Map(),nativeClosures:new Map()},{authority:true,nativeRoot,pipeline:false,directory:JSON.parse(process.env.R121_JOB!).directory});
console.log("CHILD "+JSON.stringify(reply));
process.exit(0);
