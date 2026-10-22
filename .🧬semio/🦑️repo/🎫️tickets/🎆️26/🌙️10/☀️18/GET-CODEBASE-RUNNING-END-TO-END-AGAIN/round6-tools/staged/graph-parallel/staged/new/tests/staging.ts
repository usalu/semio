import{test,expect}from"bun:test";
import{createHash}from"node:crypto";
import{existsSync,mkdtempSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{adoptStagedGraphFilesV1}from"../../../🟨️.mjs";
import{repositoryBuildGraphStagedReadDirectoryV1,repositoryBuildGraphWriterDirectoryV1,withBuildGraphOptionalPublicationV1,withBuildGraphStagingV1}from"../../🟨️.mjs";

const sha=(text:string)=>createHash("sha256").update(text).digest("hex");

test("inside a staged body every graph fact directory and optional publication is private, outside it nothing is",()=>{
 const staging=mkdtempSync(join(tmpdir(),"staging-law-")),shared=mkdtempSync(join(tmpdir(),"staging-shared-"));
 try{
  expect(repositoryBuildGraphStagedReadDirectoryV1("inference")).toBeUndefined();
  withBuildGraphStagingV1({root:staging,readRoot:shared},()=>{
   const directory=repositoryBuildGraphWriterDirectoryV1("/never-opened","build-input-closures");
   expect(directory).toBe(join(staging,"build-input-closures"));expect(existsSync(directory)).toBe(true);
   expect(repositoryBuildGraphStagedReadDirectoryV1("inference")).toBe(join(shared,"inference"));
   let wrote=0;
   expect(withBuildGraphOptionalPublicationV1(join(staging,"inference","ip","x.json"),{peakBytes:1,retainedDeltaBytes:1},()=>{wrote++;})).toBe(true);
   expect(wrote).toBe(1);
   expect(()=>withBuildGraphOptionalPublicationV1(join(shared,"inference","ip","x.json"),{peakBytes:1,retainedDeltaBytes:1},()=>{wrote++;})).toThrow("Private graph staging path required");
   expect(()=>withBuildGraphOptionalPublicationV1(staging,{peakBytes:1,retainedDeltaBytes:1},()=>{wrote++;})).toThrow();
   expect(wrote).toBe(1);
  });
  expect(()=>withBuildGraphStagingV1({root:"relative/root"},()=>{})).toThrow("Absolute private graph staging root required");
  expect(()=>repositoryBuildGraphWriterDirectoryV1("/never-opened","build-input-closures")).toThrow();
 }finally{rmSync(staging,{recursive:true,force:true});rmSync(shared,{recursive:true,force:true});}
},60000);

test("adopting staged files publishes closures by their content and optional leaves into the inference namespace, and refuses everything else",()=>{
 const staging=mkdtempSync(join(tmpdir(),"adopt-law-"));
 try{
  withBuildGraphStagingV1({root:staging},()=>{
   const contract=JSON.stringify({version:1,files:["a"]}),name="build-input-closures/"+sha(contract)+".json";
   adoptStagedGraphFilesV1("/never-opened",[{name,bytes:Buffer.from(contract)},{name:"inference/ip/abc.json",bytes:Buffer.from("leaf")}]);
   expect(readFileSync(join(staging,name),"utf8")).toBe(contract);
   expect(readFileSync(join(staging,"inference","ip","abc.json"),"utf8")).toBe("leaf");
   adoptStagedGraphFilesV1("/never-opened",[{name,bytes:Buffer.from(contract)}]);
   expect(()=>adoptStagedGraphFilesV1("/never-opened",[{name:"build-lock-closures/"+"0".repeat(64)+".json",bytes:Buffer.from(contract)}])).toThrow("Closure content differs from its name");
   expect(()=>adoptStagedGraphFilesV1("/never-opened",[{name:"build-input-closures/short.json",bytes:Buffer.from(contract)}])).toThrow();
   expect(()=>adoptStagedGraphFilesV1("/never-opened",[{name:"inference/../escape.json",bytes:Buffer.from("x")}])).toThrow("Relative staged graph file required");
   expect(()=>adoptStagedGraphFilesV1("/never-opened",[{name:"admission/x.json",bytes:Buffer.from("x")}])).toThrow("Declared graph staging namespace required");
  });
 }finally{rmSync(staging,{recursive:true,force:true});}
},60000);
