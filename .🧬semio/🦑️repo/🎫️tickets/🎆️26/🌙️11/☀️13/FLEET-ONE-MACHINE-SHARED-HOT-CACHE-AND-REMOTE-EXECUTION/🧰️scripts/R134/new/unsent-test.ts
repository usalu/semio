
test("the default spool keeps two waves of finished records whole, not the last 24 of them",()=>{
 const directory=temp();
 for(let index=0;index<96;index++)unsentPutV1(directory,{state:state("w"+index),owner:"o",documents:[{name:"record",bytes:new Uint8Array(10)}],cause:"c",attempts:1,now:1000+index});
 const oracle=spawnSync("python3",["-I","-c","import os,sys;print(len(os.listdir(sys.argv[1])))",directory],{encoding:"utf8"});
 expect(Number(oracle.stdout.trim())).toBe(96);expect(unsentListV1(directory).length).toBe(96);expect(unsentPruneV1(directory,1096)).toBe(0);
});
