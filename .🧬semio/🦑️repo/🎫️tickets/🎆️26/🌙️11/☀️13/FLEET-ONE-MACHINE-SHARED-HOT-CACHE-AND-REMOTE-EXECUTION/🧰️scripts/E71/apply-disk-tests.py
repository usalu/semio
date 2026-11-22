B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/"
def edit(path,pairs):
    s=open(B+path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:70])
        s=s.replace(old,new)
    open(B+path,"w",encoding="utf8").write(s)
edit("🧪️tests/💽️disk/🟦️.ts",[
("const GIB=2**30,disk={reserveGigabytes:3,perActionGigabytes:0.5};","const GIB=2**30,disk={reserveGigabytes:3,hostReserveGigabytes:0,perActionGigabytes:0.5},authored={reserveGigabytes:0.5,hostReserveGigabytes:3,perActionGigabytes:0.5};"),
('test("the authored disk policy is the measured one and a worker needs 8 GiB for ten slots, 3.5 GiB for the first",()=>{\n expect(readRemoteExecutionPolicyV1().worker.disk).toEqual(disk);',
 'test("the authored disk policy is the measured one and a worker needs 8 GiB for ten slots, 3.5 GiB for the first",()=>{\n expect(readRemoteExecutionPolicyV1().worker.disk).toEqual(authored);'),
("remoteExecutionDiskSlotsV1({freeBytes:1*GIB,concurrency:4,disk:{reserveGigabytes:0,perActionGigabytes:0}})","remoteExecutionDiskSlotsV1({freeBytes:1*GIB,concurrency:4,disk:{reserveGigabytes:0,hostReserveGigabytes:0,perActionGigabytes:0}})"),
("toEqual({slots:5,freeBytes:5.9*GIB,needBytes:8*GIB,oneActionBytes:3.5*GIB});","toEqual({slots:5,freeBytes:5.9*GIB,needBytes:8*GIB,oneActionBytes:3.5*GIB,volume:\"work\"});"),
("disk:{reserveGigabytes:c.reserve,perActionGigabytes:c.per}}).slots,JSON.stringify(c)).toBe(oracle[index]!));\n});",
 """disk:{reserveGigabytes:c.reserve,hostReserveGigabytes:0,perActionGigabytes:c.per}}).slots,JSON.stringify(c)).toBe(oracle[index]!));
});

test("the work volume is not charged the machine's reserve: the machine's reserve is asked of the host volume, and the smaller count of the two limits and is the one reported; equal to an independent implementation on 400 random pairs",()=>{
 expect(remoteExecutionDiskSlotsV1({freeBytes:1.5*GIB,hostFreeBytes:40*GIB,concurrency:10,disk:authored}),"a partition image with 1.5 GiB free serves two actions").toEqual({slots:2,freeBytes:1.5*GIB,needBytes:5.5*GIB,oneActionBytes:1*GIB,volume:"work"});
 expect(remoteExecutionDiskSlotsV1({freeBytes:30*GIB,hostFreeBytes:3.4*GIB,concurrency:10,disk:authored})).toEqual({slots:0,freeBytes:3.4*GIB,needBytes:8*GIB,oneActionBytes:3.5*GIB,volume:"host"});
 expect(remoteExecutionDiskSlotsV1({freeBytes:0.4*GIB,hostFreeBytes:40*GIB,concurrency:10,disk:authored}).slots,"a full image still refuses").toBe(0);
 expect(remoteExecutionDiskSlotsV1({freeBytes:undefined,hostFreeBytes:undefined,concurrency:10,disk:authored}).slots).toBe(10);
 let seed=11;const next=():number=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 const cases=Array.from({length:400},()=>({work:Math.round(next()*30*GIB),host:Math.round(next()*30*GIB),concurrency:1+Math.floor(next()*16),reserve:Math.floor(next()*4*4)/4,hostReserve:Math.floor(next()*6*4)/4,per:Math.floor(next()*6+1)/4}));
 const python=spawnSync("python3",["-I","-c","import json,sys,math\\ndef s(f,r,p,c):\\n return max(0,min(c,math.floor((f-r*2**30)/(p*2**30))))\\nfor c in json.load(sys.stdin):\\n print(min(s(c['work'],c['reserve'],c['per'],c['concurrency']),s(c['host'],c['hostReserve'],c['per'],c['concurrency'])))"],{input:JSON.stringify(cases),encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);
 const oracle=python.stdout.trim().split("\\n").map(Number);
 cases.forEach((c,index)=>expect(remoteExecutionDiskSlotsV1({freeBytes:c.work,hostFreeBytes:c.host,concurrency:c.concurrency,disk:{reserveGigabytes:c.reserve,hostReserveGigabytes:c.hostReserve,perActionGigabytes:c.per}}).slots,JSON.stringify(c)).toBe(oracle[index]!));
});"""),
])
edit("🛰️agent/💽️disk/🧪️tests/🪟️volume/🟦️.ts",[("const disk={reserveGigabytes:4,perActionGigabytes:2};","const disk={reserveGigabytes:4,hostReserveGigabytes:0,perActionGigabytes:2};")])
