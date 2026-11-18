
test("a record of the size of the largest shared closure entry is still judged, a stale file in its last row is found",()=>{
 const rows=Array.from({length:440001},(_,index)=>({kind:"file",path:"p"+index,fact:{sha256:"a".repeat(64)}})),stale="p440000";
 const verdict=freshVerdictV1({rows:rows as never,hashOf:path=>path===stale?"b".repeat(64):"a".repeat(64),createdMs:0,now:FRESH_POLICY_V1.minimumAgeMilliseconds+1});
 const oracle=spawnSync("python3",["-I","-c","import json,sys;rows=json.load(sys.stdin);print(sum(1 for r in rows if r['path']=='"+stale+"'))"],{input:JSON.stringify(rows),encoding:"utf8",maxBuffer:1<<28});
 expect(oracle.status,oracle.stderr).toBe(0);
 expect(verdict).toEqual({fresh:false,stale:Number(oracle.stdout.trim()),checked:440001});
});
