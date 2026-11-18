import sys,os
ROOT="/Users/ueli/Documents/semio"
St="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R149/backup-grow"
CALL='const grow=nativeImageSparseGrowArgumentsV1(x.filesystemBytes,scope.image);if(grow)await captureOwnedCommandV1(nativeImageHdiutilProgramV1,[...grow],scope.directory,{control,output:{maximumBytes:limits.maximumOutputBytes,maximumLines:limits.maximumOutputLines}});'
NEW='const grow=nativeImageSparseGrowArgumentsV1(x.filesystemBytes,scope.image);if(grow)await nativeImageGrowRetryV1(()=>captureOwnedCommandV1(nativeImageHdiutilProgramV1,[...grow],scope.directory,{control,output:{maximumBytes:limits.maximumOutputBytes,maximumLines:limits.maximumOutputLines}}).then(()=>undefined));'
files={
St+"🍎️image/🪶️sparse/🟦️.ts":[
('/** 🚪️ Bounds the free host bytes a creation needs:','/** 🔁️ Runs the grow command again (0.5 s, 1 s, 2 s, 4 s apart) while `hdiutil` answers status 35 (`EAGAIN`: another image operation holds the backing for a moment) and fails with the last error otherwise. See https://ss64.com/mac/hdiutil.html */\nexport async function nativeImageGrowRetryV1(run:()=>Promise<void>,pause:(milliseconds:number)=>Promise<void>=milliseconds=>new Promise(done=>setTimeout(done,milliseconds))):Promise<void>{\n for(let attempt=0;;attempt++){try{return await run();}catch(error){if(attempt>=4||!/ exited with status 35 signal null/u.test(error instanceof Error?error.message:String(error)))throw error;await pause(500*2**attempt);}}\n}\n/** 🚪️ Bounds the free host bytes a creation needs:'),
],
St+"🍎️image/🤫️silence/🟦️.ts":[
('export{nativeImageSparseAdmissionBytesV1,nativeImageSparseGrowArgumentsV1}from"../🪶️sparse/🟦️.ts";','export{nativeImageGrowRetryV1,nativeImageSparseAdmissionBytesV1,nativeImageSparseGrowArgumentsV1}from"../🪶️sparse/🟦️.ts";'),
],
St+"🧹️cleanup/🎛️command/🟦️.ts":[
('nativeImageSeededCreateArgumentsV1,nativeImageSparseAdmissionBytesV1,nativeImageSparseGrowArgumentsV1}from"../../🍎️image/🤫️silence/🟦️.ts";','nativeImageGrowRetryV1,nativeImageSeededCreateArgumentsV1,nativeImageSparseAdmissionBytesV1,nativeImageSparseGrowArgumentsV1}from"../../🍎️image/🤫️silence/🟦️.ts";'),
(CALL,NEW),
],
St+"🍎️image/🪶️sparse/🧪️tests/🟦️.ts":[
('import{nativeImageSparseAdmissionBytesV1,nativeImageSparseGrowArgumentsV1,nativeImageSparseInitialBytesV1}from"../🟦️.ts";','import{nativeImageGrowRetryV1,nativeImageSparseAdmissionBytesV1,nativeImageSparseGrowArgumentsV1,nativeImageSparseInitialBytesV1}from"../🟦️.ts";'),
],
}
TEST='''
test("retry grow: status 35 is retried 0.5, 1, 2 and 4 s apart and then given up, any other failure is thrown at once, success stops",async()=>{
 const failure=(status:number)=>Error("/usr/bin/hdiutil resize -size 1m x.dmg exited with status "+status+" signal null");
 const run=async(failures:readonly number[])=>{const pauses:number[]=[];let calls=0;const outcome=await nativeImageGrowRetryV1(async()=>{const status=failures[calls++];if(status!==undefined)throw failure(status);},async milliseconds=>{pauses.push(milliseconds);}).then(()=>"ok",error=>String(error.message));return{calls,pauses,outcome};};
 expect(await run([])).toEqual({calls:1,pauses:[],outcome:"ok"});
 expect(await run([35,35])).toEqual({calls:3,pauses:[500,1000],outcome:"ok"});
 expect(await run([35,35,35,35])).toEqual({calls:5,pauses:[500,1000,2000,4000],outcome:"ok"});
 const spent=await run([35,35,35,35,35,35]);expect(spent.calls).toBe(5);expect(spent.pauses).toEqual([500,1000,2000,4000]);expect(spent.outcome).toContain("status 35");
 const other=await run([35,22,35]);expect(other.calls).toBe(2);expect(other.outcome).toContain("status 22");
});
'''
mode=sys.argv[1]
os.makedirs(SC,exist_ok=True)
root=sys.argv[2] if len(sys.argv)>2 else ROOT
for rel,pairs in files.items():
    path=os.path.join(root,rel);s=open(path,encoding="utf8").read()
    if mode=="apply":
        open(os.path.join(SC,rel.replace("/","_")),"w",encoding="utf8").write(s)
        for a,b in pairs:
            assert s.count(a)==1,("anchor",rel,a[:70],s.count(a));s=s.replace(a,b)
        if rel.endswith("🧪️tests/🟦️.ts"):s=s.rstrip("\n")+"\n"+TEST
    else:
        for a,b in pairs:
            assert s.count(b)==1,("anchor",rel,b[:70],s.count(b));s=s.replace(b,a)
        if rel.endswith("🧪️tests/🟦️.ts"):
            assert s.endswith(TEST);s=s[:-len(TEST)].rstrip("\n")+"\n"
    open(path,"w",encoding="utf8").write(s)
print(mode,"ok")
