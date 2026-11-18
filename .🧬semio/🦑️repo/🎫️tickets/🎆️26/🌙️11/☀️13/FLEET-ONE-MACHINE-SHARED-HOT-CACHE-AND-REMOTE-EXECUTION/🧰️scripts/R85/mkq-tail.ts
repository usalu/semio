
test("a store file is written whole even where the rename over the old file is refused, and no temporary file is left (the mirrors of g1-m05 stayed at 17:24 for hours)",()=>{
 const directory=temp("store-file"),file=join(directory,"ready-log-tools.txt");
 readyWriteStoreFileV1(file,"one\n");expect(readFileSync(file,"utf8")).toBe("one\n");
 let refused=0;readyWriteStoreFileV1(file,"two\n",()=>{refused++;throw Object.assign(Error("EPERM"),{code:"EPERM"});});
 expect(refused).toBe(1);expect(readFileSync(file,"utf8")).toBe("two\n");expect(readdirSync(directory)).toEqual(["ready-log-tools.txt"]);
});

posix("a stage past its soft cap whose log keeps showing new lines is not stopped as timed out although none of them is a declared progress counter; a standing line is (g1-m04 and g1-m05 hit their cap inside counters that the cap did not know)",async()=>{
 const growing=fakeHome('#!/bin/sh\ni=0\nwhile [ $i -lt 24 ]; do i=$((i+1)); echo "[script] {\\"activity\\":{\\"resource\\":\\"dependencies:workspace-edges:$((i*64))\\",\\"mode\\":\\"exclusive\\",\\"elapsedMs\\":0}}"; sleep 0.3; done\nexit 0\n');
 const policy={...TREE,stallMilliseconds:3500,stallStopMilliseconds:600000};
 const moving=await runReadyStageV1({home:growing.home,root:temp("root"),environment:{},stage:stageOf("tools",2500),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:policy});
 expect(moving.timedOut).toBe(false);expect(moving.exit).toBe(0);
 const standing=fakeHome('#!/bin/sh\nwhile true; do echo "[script] {\\"activity\\":{\\"resource\\":\\"native-artifact-input-stage\\",\\"mode\\":\\"exclusive\\",\\"elapsedMs\\":0}}"; sleep 0.3; done\n');
 const began=Date.now(),stuck=await runReadyStageV1({home:standing.home,root:temp("root"),environment:{},stage:stageOf("tools",2500),signal:new AbortController().signal,jobs:1,onLine:()=>undefined,tree:policy});
 expect(stuck.timedOut).toBe(true);expect(Date.now()-began).toBeLessThan(25000);
},90000);
