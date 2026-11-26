import os,sys,tempfile
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/"
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:60],s.count(a))
        s=s.replace(a,b)
    fd,tmp=tempfile.mkstemp(dir=os.path.dirname(path));os.write(fd,s.encode());os.close(fd);os.replace(tmp,path)
edit(B+"🧵️process/🟨️.mjs",[
('child.on("exit",code=>{void Promise.race([new Promise(done=>child.once("close",done)),new Promise(done=>setTimeout(done,STDERR_DRAIN_MILLISECONDS_V1).unref?.())]).then(()=>Promise.allSettled([...pending])).then(()=>listener(code));})',
 'child.on("exit",(code,signal)=>{void Promise.race([new Promise(done=>child.once("close",done)),new Promise(done=>setTimeout(done,STDERR_DRAIN_MILLISECONDS_V1).unref?.())]).then(()=>Promise.allSettled([...pending])).then(()=>listener(code,signal));})'),
])
edit(B+"🟨️.mjs",[
('worker.on("exit",code=>{if(!settled){counters.failed++;const tail=typeof worker.stderrTail==="function"?worker.stderrTail():"";finish(reject,Object.assign(new Error("Slice worker exited with code "+code),{exit:code,restart:code===HOST_POLICY_V1.sliceRestartExitCode,ranMilliseconds:Math.round(now()-started),workerCause:tail===""?"":workerCauseV1(tail),drift:implementationDriftPathsV1(tail)}));}});',
 'worker.on("exit",(code,signal)=>{if(!settled){counters.failed++;const killed=code===null&&typeof signal==="string"&&signal!=="",tail=typeof worker.stderrTail==="function"?worker.stderrTail():"";finish(reject,Object.assign(new Error("Slice worker exited with code "+code+(killed?" (killed by "+signal+")":"")),{exit:code,...killed?{code:"slice-killed",signal,currentMegabytes:entry.treePeakMegabytes}:{},restart:killed||code===HOST_POLICY_V1.sliceRestartExitCode,ranMilliseconds:Math.round(now()-started),workerCause:killed?"killed by "+signal+" from outside (resource-transient)":tail===""?"":workerCauseV1(tail),drift:implementationDriftPathsV1(tail)}));}});'),
('const squeezed=(error?.code==="memory-budget"||error?.code==="memory-pressure")&&',
 'const squeezed=(error?.code==="memory-budget"||error?.code==="memory-pressure"||error?.code==="slice-killed")&&'),
('sameCause=unsettled?0:cause!==""',
 'sameCause=unsettled||error.code==="slice-killed"?0:cause!==""'),
])
