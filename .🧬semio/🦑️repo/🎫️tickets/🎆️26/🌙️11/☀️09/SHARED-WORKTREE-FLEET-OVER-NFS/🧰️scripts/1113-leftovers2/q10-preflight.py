#!/usr/bin/env python3
"""q10: topology preflight: watcher mode (X23), head visibility measured by the replica (X18), Windows logon (X22). usage: q10-preflight.py <module-root>"""
import sys
root=sys.argv[1]
def sub(s,old,new,count=1):
    assert s.count(old)==count,(old[:80],s.count(old))
    return s.replace(old,new)
def edit(path,fn):
    p=root+"/"+path
    s=open(p,encoding="utf8").read(); s=fn(s); open(p,"w",encoding="utf8").write(s)

def command(s):
    s=sub(s,'storeWarnBytes:300*2**30,storeFailBytes:50*2**30,lagEvents:1000});','storeWarnBytes:300*2**30,storeFailBytes:50*2**30,lagEvents:1000,visibilityWarnMilliseconds:5000,visibilitySamples:5});')
    s=sub(s,' const health=memo(async()=>{const before=probes.now()',' const status=memo(()=>probes.status(worktree,storeRoot,signal));\n const health=memo(async()=>{const before=probes.now()')
    s=sub(s,'  const report=await probes.status(worktree,storeRoot,signal);\n  if(!report.reachable)','  const report=await status();\n  if(!report.reachable)')
    s=sub(s,'''  return ok("replica.ok",{files:report.files,origins:report.origins.length,incoming:report.queue.incoming,outgoing:report.queue.outgoing??"?"});
 });
''','''  return ok("replica.ok",{files:report.files,origins:report.origins.length,incoming:report.queue.incoming,outgoing:report.queue.outgoing??"?"});
 });
 add("watcher",Math.max(timeoutMilliseconds,15000),async()=>{
  const report=await status().catch(()=>undefined);
  if(report===undefined||!report.daemon.running||report.health===null||report.health===undefined)return skip("watcher.skip");
  return report.health.failures.some(item=>item.kind==="watch")?warn("watcher.degraded",{platform}):ok("watcher.ok");
 });
 add("visibility",Math.max(timeoutMilliseconds,15000),async()=>{
  const report=await status().catch(()=>undefined),latency=report?.latency;
  if(latency===null||latency===undefined||latency.samples.apply<limits.visibilitySamples||latency.applyMs.p95===null)return skip("visibility.skip",{minimum:limits.visibilitySamples});
  const variables={platform,p50:latency.applyMs.p50??"?",p95:latency.applyMs.p95,samples:latency.samples.apply,limit:limits.visibilityWarnMilliseconds};
  return latency.applyMs.p95>limits.visibilityWarnMilliseconds?warn("visibility.slow",variables):ok("visibility.ok",variables);
 });
''')
    s=sub(s,'''  const peers=topology.groups.filter(other=>other.id!==groupId),needSmb=''','''  add("logon",timeoutMilliseconds,async()=>{
   const result=await probes.run("reg.exe",["query","HKLM\\\\SOFTWARE\\\\Microsoft\\\\Windows NT\\\\CurrentVersion\\\\Winlogon","/v","AutoAdminLogon"],timeoutMilliseconds,signal);
   return /AutoAdminLogon\\s+REG_SZ\\s+1\\s*$/mu.test(result.stdout)?ok("logon.auto"):warn("logon.manual");
  });
  const peers=topology.groups.filter(other=>other.id!==groupId),needSmb=''')
    return s
edit("🗺️topology/⌨️command/🟦️.ts",command)

def texts(s):
    s=sub(s,'''    "preflight.name.longpaths": {''','''    "preflight.name.watcher": {
      "en": "file watcher",
      "de": "Dateiwächter"
    },
    "preflight.name.visibility": {
      "en": "event visibility",
      "de": "Sichtbarkeit der Ereignisse"
    },
    "preflight.name.logon": {
      "en": "start without logon",
      "de": "Start ohne Anmeldung"
    },
    "preflight.name.longpaths": {''')
    s=sub(s,'''    "preflight.replica.ok": {''','''    "preflight.watcher.ok": {
      "en": "the recursive file watcher runs; the scans are only the safety net",
      "de": "der rekursive Dateiwächter läuft; die Scans sind nur das Sicherheitsnetz"
    },
    "preflight.watcher.degraded": {
      "en": "the file watcher stopped on {platform}: only the scans remain and an edit can take up to the scan interval to be seen (on Linux raise the inotify limits)",
      "de": "der Dateiwächter wurde unter {platform} beendet: nur die Scans bleiben und eine Änderung kann bis zum Scan-Intervall brauchen (unter Linux die inotify-Grenzen erhöhen)"
    },
    "preflight.watcher.skip": {
      "en": "the daemon is not running, so its watcher cannot be read",
      "de": "der Daemon läuft nicht, sein Dateiwächter kann daher nicht gelesen werden"
    },
    "preflight.visibility.ok": {
      "en": "events from other machines were applied after {p50} ms (median) and {p95} ms (95th percentile) over {samples} samples on {platform}; the limit is {limit} ms",
      "de": "Ereignisse anderer Maschinen wurden nach {p50} ms (Median) und {p95} ms (95. Perzentil) bei {samples} Stichproben unter {platform} angewendet; die Grenze ist {limit} ms"
    },
    "preflight.visibility.slow": {
      "en": "events from other machines reach this {platform} machine slowly: {p95} ms at the 95th percentile (median {p50} ms, {samples} samples, limit {limit} ms); look at the client cache options of the store mount",
      "de": "Ereignisse anderer Maschinen erreichen diese {platform}-Maschine langsam: {p95} ms beim 95. Perzentil (Median {p50} ms, {samples} Stichproben, Grenze {limit} ms); die Cache-Optionen des Store-Mounts prüfen"
    },
    "preflight.visibility.skip": {
      "en": "fewer than {minimum} applied events were measured yet; run the check again after the fleet has been busy",
      "de": "bisher wurden weniger als {minimum} angewendete Ereignisse gemessen; die Prüfung wiederholen, nachdem das Fleet beschäftigt war"
    },
    "preflight.logon.auto": {
      "en": "automatic logon is configured, so the replica task starts after a reboot without anyone logging on",
      "de": "die automatische Anmeldung ist eingerichtet, die Replica-Aufgabe startet daher nach einem Neustart ohne Anmeldung"
    },
    "preflight.logon.manual": {
      "en": "the replica task starts only after this account logs on: a reboot leaves the machine out of the fleet until someone logs on; configure automatic logon (Winlogon AutoAdminLogon) for this account",
      "de": "die Replica-Aufgabe startet erst nach der Anmeldung dieses Kontos: ein Neustart lässt die Maschine aus dem Fleet, bis sich jemand anmeldet; die automatische Anmeldung (Winlogon AutoAdminLogon) für dieses Konto einrichten"
    },
    "preflight.replica.ok": {''')
    return s
edit("🗺️topology/📖️texts/🔣️.json",texts)

def tests(s):
    s=s.replace('replica:"ok:replica.ok"};','replica:"ok:replica.ok",watcher:"skip:watcher.skip",visibility:"skip:visibility.skip"};',1)
    assert 'watcher:"skip:watcher.skip"' in s
    s=s.replace("All 22 checks passed.","All 24 checks passed.").replace("22 checks, 1 with warnings","24 checks, 1 with warnings").replace("\\/22\\]","\\/24\\]").replace("Alle 22 Prüfungen","Alle 24 Prüfungen")
    s=sub(s,'for(const id of Object.keys(LINUX_OK).concat(["disk.store","service.cache","service.relay","clientcache","longpaths"]))','for(const id of Object.keys(LINUX_OK).concat(["disk.store","service.cache","service.relay","clientcache","longpaths","logon"]))')
    anchor='test("the preflight catalogue has the same variables in English and German'
    new='''test("the preflight reads the watcher mode and the event visibility from the running replica, and on Windows whether the replica starts without a logon",async()=>{
 const running=(change:object):Partial<PreflightProbesV1>=>({status:async()=>({...HEALTHY,...change}) as unknown as StatusReportV1});
 const health=(failures:readonly object[])=>({version:1,wallMs:1,store:"ok",storeSinceMs:0,code:"x",pending:null,failures});
 const latency=(apply:number,p95:number|null,p50:number|null=40)=>({publishMs:{p50:null,p95:null},applyMs:{p50,p95},samples:{publish:0,apply},tiers:{}});
 const run=async(change:object)=>statusOf(await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),running(change))));
 expect(await run({health:health([])})).toMatchObject({watcher:"ok:watcher.ok",visibility:"skip:visibility.skip"});
 expect(await run({health:health([{kind:"watch",count:1,lastMs:1}])})).toMatchObject({watcher:"warn:watcher.degraded"});
 expect(await run({health:null})).toMatchObject({watcher:"skip:watcher.skip"});
 expect(await run({daemon:{running:false,pid:null},health:null})).toMatchObject({watcher:"skip:watcher.skip"});
 expect(await run({latency:latency(3,90)})).toMatchObject({visibility:"skip:visibility.skip"});
 expect(await run({latency:latency(50,null)})).toMatchObject({visibility:"skip:visibility.skip"});
 expect(await run({latency:latency(50,900)})).toMatchObject({visibility:"ok:visibility.ok"});
 const slow=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),running({latency:latency(50,PREFLIGHT_LIMITS_V1.visibilityWarnMilliseconds+1,2000)})));
 expect(statusOf(slow)["visibility"]).toBe("warn:visibility.slow");
 expect(slow.text).toContain("reach this linux machine slowly");
 const german=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),running({health:health([{kind:"watch",count:1,lastMs:1}])})),{languages:["de"]});
 expect(german.text).toContain("Dateiwächter wurde unter linux beendet");
 const windowsRun=(registry:string)=>preflight("a1","windows",fake({"C:\\\\semio\\\\.cache\\\\f\\\\machine.json":JSON.stringify({originId:"a1-m01",groupId:"a1",storeRoot:"S:\\\\"})},(command,args)=>command==="reg.exe"&&args.join(" ").includes("AutoAdminLogon")?{code:registry===""?1:0,stdout:registry}:{code:127,stdout:""},{status:async()=>HEALTHY}),{worktree:"C:\\\\semio"});
 expect(statusOf(await windowsRun("    AutoAdminLogon    REG_SZ    1\\r\\n"))["logon"]).toBe("ok:logon.auto");
 expect(statusOf(await windowsRun("    AutoAdminLogon    REG_SZ    0\\r\\n"))["logon"]).toBe("warn:logon.manual");
 expect(statusOf(await windowsRun(""))["logon"]).toBe("warn:logon.manual");
 expect(Object.keys(statusOf(await preflight("a1","linux",fake(linuxFiles(),linuxCommands())))).includes("logon")).toBe(false);
});

'''
    s=sub(s,anchor,new+anchor)
    return s
edit("🗺️topology/🧪️tests/🟦️.ts",tests)
print("ok")
