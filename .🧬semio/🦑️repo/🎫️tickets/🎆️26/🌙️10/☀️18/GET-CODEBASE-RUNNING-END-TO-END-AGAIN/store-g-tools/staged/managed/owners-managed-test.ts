import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{createNativeImageVolumeSilenceSeedV1,nativeImageSeededCreateArgumentsV1,nativeImageSparseGrowArgumentsV1}from"../../../🤫️silence/🟦️.ts";
import{nativeImageAttachArgumentsV1,requireNativeImageOutsideManagedParentV1}from"../../🟦️.ts";
const scratch=()=>realpathSync(mkdtempSync(join(testScratchRootV1("managed-parent"),"mp-")));
const table=(...points:string[])=>["/dev/disk3s5 on / (apfs, local, journaled)",...points.map((point,index)=>"/dev/disk"+(9+index)+"s1 on "+point+" (apfs, local, nodev, nosuid, journaled, nobrowse)")].join("\n");
test("an unfunded image directory below a managed parent mount is refused in both languages, below anything else it is allowed",()=>{
 const base=scratch();try{
  const parents=[join(base,".cache","b","o","m"),join(base,".cache","b","p","m"),join(base,"x","rust","fixed","m")];
  for(const parent of parents){const directory=join(parent,"tmp","gf-1");mkdirSync(directory,{recursive:true});
   let failure:Error|undefined;try{requireNativeImageOutsideManagedParentV1(table(parent),directory);}catch(error){failure=error as Error;}
   expect(failure?.name,parent).toBe("NativeStorageManagedParentError");expect(failure!.message).toContain("testScratchRootV1");expect(failure!.message).toContain("Sandbox-Verzeichnis");expect(failure!.message).toContain(parent);
  }
  const loose=join(base,"loose","fixture");mkdirSync(loose,{recursive:true});
  expect(()=>requireNativeImageOutsideManagedParentV1(table(...parents),loose)).not.toThrow();
  const other=join(base,".cache","b","oo","m","t");mkdirSync(other,{recursive:true});
  expect(()=>requireNativeImageOutsideManagedParentV1(table(join(base,".cache","b","oo","m")),other),"only single-letter partitions are managed").not.toThrow();
 }finally{rmSync(base,{recursive:true,force:true});}
});
const run=(args:readonly string[])=>{const row=spawnSync("/usr/bin/hdiutil",[...args],{encoding:"utf8"});if(row.status!==0)throw Error(args.join(" ")+": "+row.stderr);return row.stdout;};
test.skipIf(process.platform!=="darwin")("a really mounted managed-looking parent refuses a nested unfunded image and the kernel table of the sandbox does not",()=>{
 const base=scratch(),seed=createNativeImageVolumeSilenceSeedV1(),mount=join(base,".cache","b","o","m");
 try{
  mkdirSync(mount,{recursive:true});const image=join(base,"parent.dmg");
  run(nativeImageSeededCreateArgumentsV1(67108864,image,seed.directory));const grow=nativeImageSparseGrowArgumentsV1(67108864,image);if(grow)run(grow);run(nativeImageAttachArgumentsV1(mount,image));
  const nested=join(mount,"tmp","silence-1");mkdirSync(nested,{recursive:true});
  const kernel=spawnSync("/sbin/mount",[],{encoding:"utf8"}).stdout;
  expect(()=>requireNativeImageOutsideManagedParentV1(kernel,nested)).toThrow("managed cache partition");
  expect(()=>requireNativeImageOutsideManagedParentV1(kernel,base)).not.toThrow();
 }finally{seed.dispose();try{run(["detach",mount]);}catch{}rmSync(base,{recursive:true,force:true});}
},60000);
