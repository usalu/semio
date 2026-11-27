import os,sys,tempfile
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/"
def edit(rel,pairs):
    p=R+rel;s=open(p,encoding="utf8").read()
    for a,b in pairs:
        if s.count(a)!=1: sys.exit("anchor not unique/missing in %s: %s"%(rel,a[:70]))
        s=s.replace(a,b)
    fd,t=tempfile.mkstemp(dir=os.path.dirname(p));os.write(fd,s.encode("utf8"));os.close(fd);os.rename(t,p)
edit("🛰️fleet/🧬️schema/🔣️.json",[
 ('"spool": { "minimumEntries": 4096,','"spool": { "minimumEntries": 16384,'),
 ('"maximumJobAgeMilliseconds": 21600000','"maximumJobAgeMilliseconds": 86400000'),
 ('"pinMaximumJobs": 4096','"pinMaximumJobs": 16384')])
edit("🛰️fleet/📤️uploader/🟦️.ts",[
 ('   if(!client.usable()){wait(started+Math.round(policy.retryBackoffMilliseconds*(0.5+Math.random())),"upload-deferred: "+client.state(),settings.identity);await release();return false;}',
  '   if(!client.usable()){wait(started+Math.round(policy.retryBackoffMilliseconds*(0.5+Math.random())),"upload-deferred: "+client.state(),settings.identity);await release();return false;}'),
 ('   if(options.idleMilliseconds!==0){const named=identity();if(named!==undefined&&named!==settings.identity)break;}\n',
  '   if(options.idleMilliseconds!==0){const named=identity();if(named!==undefined&&named!==settings.identity)break;}\n   if(running.size===0&&client.state()==="down"){if(idle===0){result.deferred=(result.deferred??0)+spoolLengthV1(root);break;}quietSince=Date.now();await pause(policy.waitingPollMilliseconds);continue;}\n'),
 ('   else{countSkipV1(workspace,"upload-failed");event("remote-skip","upload-failed");}',
  '   else if(outcome===false){countSkipV1(workspace,"upload-unshareable");event("remote-skip","upload-unshareable: an incremental session or a run that names a directory of this machine is never shared");}\n   else{countSkipV1(workspace,"upload-failed");event("remote-skip","upload-failed");}')])
edit("🛰️fleet/🟦️.ts",[
 (' return unit.client.settings.upload?"spool-full":"read-only";',
  ' return!unit.client.settings.upload?"read-only":spoolLengthV1(unit.root)>=spoolBoundV1(unit.root)?"spool-full":"spool-error";')])
edit("🏗️buildscript/🟦️.ts",[
 (' if(!shareFleetUnitV1(fleet,{claimed:hold,costMs:input.costMs})){countSkipV1(workspace,unsharedReasonV1(fleet));',
  ' if(!await shareFleetUnitAwaitingV1(fleet,{claimed:hold,costMs:input.costMs})){countSkipV1(workspace,unsharedReasonV1(fleet));'),
 ('prepareFleetV1,shareFleetUnitV1,unsharedReasonV1,','prepareFleetV1,shareFleetUnitAwaitingV1,unsharedReasonV1,')])
print("ok")
