import sys
p=sys.argv[1]
s=open(p).read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:60],s.count(old))
    s=s.replace(old,new)
rep('import { fleetIdleSlotsV1, readFamilyPeakV1,','import { fleetIdleSlotsV1, fleetLinkedSlotsV1, readFamilyPeakV1,')
rep('const family=root===undefined?undefined:readFamilyPeakV1(root),peaks=facts.crates.map(crate=>offloadUnitPeakMegabytesV1(root===undefined?undefined:readUnitPeakV1(root,crate),types,target,family)).sort((left,right)=>left-right),peak=peaks[Math.floor(peaks.length/2)]??offloadUnitPeakMegabytesV1(undefined,types,target,family),slots=await fleetIdleSlotsV1(store.environment,{signal:facts.signal,peakMegabytes:peak}),plan=planComponentJobsV1({local:facts.local,',
    'const family=root===undefined?undefined:readFamilyPeakV1(root),measured=root===undefined?[]:facts.crates.flatMap(crate=>readUnitPeakV1(root,crate)??[]).sort((left,right)=>left-right),peak=measured.length>0?measured[Math.floor(measured.length/2)]!:offloadUnitPeakMegabytesV1(undefined,types,target,family),slots=await fleetLinkedSlotsV1(store.environment,{signal:facts.signal,peakMegabytes:peak}),plan=planComponentJobsV1({local:1,')
open(p,'w').write(s)
