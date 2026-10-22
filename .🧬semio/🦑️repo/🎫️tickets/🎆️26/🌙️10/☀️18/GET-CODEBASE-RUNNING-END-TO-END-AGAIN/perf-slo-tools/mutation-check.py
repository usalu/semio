#!/usr/bin/env python3
"""Mutation check: applies each seeded defect to the verification overlay copy (after `sh verify.sh`), runs the bench tests and demands that at least one test fails; restores the file after every mutation. Exit 1 when a mutant survives."""
import os,subprocess,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
ov="/private/tmp/perf-slo/ov";base=f"{ov}/{names.BENCHDIR}"
files={"model":f"{base}/{names.TS}","runner":f"{base}/{names.RUNNER}/{names.TS}","command":f"{base}/{names.COMMAND}/{names.TS}"}
test=f"./{names.BENCHDIR}/{names.TESTS}/{names.TS}"
M=[
 ("percentile without interpolation","model","return sorted[low]!+(position-low)*(sorted[high]!-sorted[low]!);","return sorted[Math.round(position)]!;"),
 ("lower within is strict","model","const within=(limit:number)=>lower?value<=limit:value>=limit;","const within=(limit:number)=>lower?value<limit:value>=limit;"),
 ("saturated fail stays fail","model",'if(status==="fail"&&inflated)return','if(false&&inflated)return'),
 ("no minimum sample count","model","if(n<policy.minimumSamples[slo.statistic])return","if(n<1)return"),
 ("tolerance without the absolute margin","model","const up=base*(1+tolerance.relative)+tolerance.absolute,down=base*(1-tolerance.relative)-tolerance.absolute","const up=base*(1+tolerance.relative),down=base*(1-tolerance.relative)"),
 ("saturated regression is a regression","model",'verdict==="regressed"&&row.unit==="ms"&&(saturated(current,row)||saturated(baseline,old))','false'),
 ("host class ignored","model","else if(!sameHost)rows.push","else if(false)rows.push"),
 ("cold probes not first","model","(right.probe.cold?1:0)-(left.probe.cold?1:0)||",""),
 ("quiet threshold inclusive","model","perCore<policy.loadPerCore.quietBelow","perCore<=policy.loadPerCore.quietBelow"),
 ("failed samples enter the statistics","model","const good=input.samples.filter(sample=>sample.ok)","const good=input.samples"),
 ("every failure is blocking","model",'row.status==="fail"&&row.enforcement==="blocking").length}','row.status==="fail").length}'),
 ("a goal without a gate is admitted","model",'if(slo.goal!==null&&slo.gate===null)found.push(`${slo.id}: a goal needs a gate`);',''),
 ("unproven rows are compared","model",'old.status==="unproven"||row.status==="unproven"||',''),
 ("missing rows for every absent probe","model","if(now.has(key(row))||!probes.has(row.probe))continue;","if(now.has(key(row)))continue;"),
 ("start bound ignored","model",'if(bound!=="exact"&&status!=="recorded"&&!(','if(false&&!('),
 ("edit not restored","runner","}finally{await ports.write(unit.file,original);}","}finally{}"),
 ("start time ignores the edit offset","runner","samples.push(sample(samples.length,outcome,outcome.exit===0?{startMs:offset+startOffset(outcome),greenMs:offset+outcome.wallMs}:{},startKindOf(outcome),here));","samples.push(sample(samples.length,outcome,outcome.exit===0?{startMs:startOffset(outcome),greenMs:outcome.wallMs}:{},startKindOf(outcome),here));"),
 ("service not stopped","runner","}finally{if(started)await ports.service.stop();}","}finally{}"),
 ("cold probe ignores a stale copy","runner","if(probe.cold&&!ports.fresh)return unavailable(probe,tool,\"not-cold\");",""),
 ("no prime run","runner","const prime=await attempt(ports,()=>timed());\n   if(prime instanceof Error||prime.exit!==0){samples.push(failed(0,load()));return series();}","const prime=finished0;"),
 ("process tree without descendants","runner","if(!members.has(pid)&&members.has(row.parent))","if(false)"),
 ("marker never detected","runner","if(markers.some(pattern=>pattern.test(line)))markerMs=elapsed;","void line;"),
 ("hit ratio over all tasks","runner","hitRatio:(done.cacheable-done.executedCacheable)/done.cacheable","hitRatio:(done.total-done.executedCacheable)/done.total"),
 ("one-shot lane keeps the service","runner","service?[policy.oneshotFlag]:[]","[]"),
 ("blocking failures do not fail the exit code","command","if(report.tally.blockingFailed)throw Error(","if(false)throw Error("),
 ("reports are written after the failure","command","await emit(report,report.comparison);\n  if(report.tally.blockingFailed)","if(report.tally.blockingFailed)throw Error('x');await emit(report,report.comparison);\n  if(report.tally.blockingFailed)"),
 ("baseline not compared","command","baseline},benchCliPortsV1","baseline:null},benchCliPortsV1"),
]
survivors=[]
for name,key,old,new in M:
    path=files[key];original=open(path,encoding="utf8").read()
    if original.count(old)!=1: print(f"ANCHOR {name}: found {original.count(old)}");survivors.append(name);continue
    mutated=original.replace(old,new.replace("finished0","{exit:0,wallMs:0,startedMs:0,firstOutputMs:null,markerMs:null,peakRssBytes:null}"))
    try:
        open(path,"w",encoding="utf8").write(mutated)
        run=subprocess.run(["nice","-n","10","bun","test","--timeout","120000",test],cwd=ov,env={**os.environ,"SEMIO_TEST_REPO_ROOT":names.R},capture_output=True,text=True)
        killed=run.returncode!=0
        print(("killed  " if killed else "SURVIVED"),name,flush=True)
        if not killed: survivors.append(name)
    finally: open(path,"w",encoding="utf8").write(original)
print(f"{len(M)-len(survivors)} of {len(M)} mutants killed")
sys.exit(1 if survivors else 0)
