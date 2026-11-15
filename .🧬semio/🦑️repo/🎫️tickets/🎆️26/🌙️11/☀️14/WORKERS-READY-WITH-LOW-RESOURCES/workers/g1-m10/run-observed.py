import collections,datetime,glob,json,os,pathlib,re,signal,subprocess,sys,time
root=pathlib.Path.home()/"Documents/semio"
own=pathlib.Path(__file__).resolve().parent
ticket=own.parent.parent
round_number=int(sys.argv[1]); attempt=int(sys.argv[2])
prefix=own/("round-%s-try-%s"%(round_number,attempt))
log=pathlib.Path(str(prefix)+".log")
samples=pathlib.Path(str(prefix)+".samples.txt")
top=pathlib.Path(str(prefix)+".top3.txt")
service_copy=pathlib.Path(str(prefix)+".service.txt")
if log.exists():raise SystemExit("Existing try log refused; choose a new try number")
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")
def command(args,timeout=8):
    try:
        done=subprocess.run(args,capture_output=True,text=True,timeout=timeout)
        return done.stdout.strip() if done.returncode==0 else ""
    except (subprocess.TimeoutExpired,OSError):return ""
def closure_count():
    total=0
    for directory in glob.glob(str(root/".cache/b/g/m/inference/selections/*/ce/e")):
        for _,_,files in os.walk(directory):total+=len(files)
    return total
def processes():
    data=command(["ps","-axo","pid=,ppid=,rss=,args="])
    result=[]
    for line in data.splitlines():
        fields=line.strip().split(None,3)
        if len(fields)==4:
            try:result.append((int(fields[0]),int(fields[1]),int(fields[2]),fields[3]))
            except ValueError:pass
    return result
def newest_service():
    files=glob.glob("/var/folders/*/*/T/semio-repo/%s/host-*.log"%os.getuid())
    return max(files,key=os.path.getmtime) if files else None
def append(path,text):
    with path.open("a") as output:output.write(text+"\n")
def stop(child,reason):
    if child.poll() is not None:return
    append(samples,utc()+" stop="+reason+"; one SIGINT to this try's process group")
    os.killpg(child.pid,signal.SIGINT)
    try:child.wait(timeout=30)
    except subprocess.TimeoutExpired:
        append(samples,utc()+" SIGINT grace expired; terminating only this try's process group")
        os.killpg(child.pid,signal.SIGTERM)
        try:child.wait(timeout=10)
        except subprocess.TimeoutExpired:
            os.killpg(child.pid,signal.SIGKILL);child.wait()
before=closure_count()
start=utc(); started=time.monotonic()
append(samples,"start="+start)
append(samples,"closure entries before: "+str(before))
last_progress=time.monotonic()
last_line="not measured"
last_service_line="not measured"
service_offsets={}
initial_service=newest_service()
if initial_service:service_offsets[initial_service]=os.path.getsize(initial_service)
native_offset=0
native_partial=b""
sampled=False
stack_process=None
peak_run=0; peak_slice=0; peak_system=0; peak_at=None
next_30=0
pressure_streak=0
stop_reason=None
wait_since=None
wait_line=None
progress_keys=set()
fleet_delivered=-1
env=dict(os.environ)
env.update(SEMIO_LANGUAGE="en",SEMIO_WORKSPACE_ROOT=str(root))
for name in ("SEMIO_REMOTE_CACHE_TOKEN","SEMIO_REMOTE_CACHE_ENDPOINT","SEMIO_REMOTE_CACHE_ACCESS","SEMIO_REMOTE_CACHE_INSTANCE","SEMIO_FLEET_CACHE_ENDPOINT"):env.pop(name,None)
wrapper_output=pathlib.Path(str(prefix)+".wrapper.txt").open("w")
child=subprocess.Popen(["bash",str(ticket/"native-bootstrap.sh"),str(log)],cwd=root,env=env,stdout=wrapper_output,stderr=subprocess.STDOUT,start_new_session=True)
(own/"active-try.json").write_text(json.dumps({"round":round_number,"try":attempt,"pid":child.pid,"start":start,"log":str(log)})+"\n")
try:
    while child.poll() is None:
        tick=time.monotonic()
        rows=processes()
        descendants={child.pid}
        changed=True
        while changed:
            changed=False
            for pid,parent,rss,args in rows:
                if parent in descendants and pid not in descendants:descendants.add(pid);changed=True
        run_rows=[row for row in rows if row[0] in descendants]
        slices=[row for row in rows if "slice-pool" in row[3] and "worker.ts" in row[3]]
        run_peak=max((row[2]/1024 for row in run_rows),default=0)
        slice_peak=max((row[2]/1024 for row in slices),default=0)
        higher_peak=run_peak>peak_run or slice_peak>peak_slice
        peak_run=max(peak_run,run_peak)
        peak_slice=max(peak_slice,slice_peak)
        service=newest_service()
        if service:
            previous=service_offsets.get(service)
            if previous is None:
                previous=0
                service_offsets[service]=previous
                append(samples,"service log: "+service)
            try:
                with open(service,"rb") as source:
                    if os.path.getsize(service)<previous:previous=0
                    source.seek(previous);new=source.read();service_offsets[service]=source.tell()
                if new:
                    text=new.decode("utf8","replace")
                    append(service_copy,text.rstrip())
                    candidates=[line for line in text.splitlines() if re.search(r"graph:|Graph: Plan-Stufe|Reading the project graph slice|graph-plan-too-heavy|^\[graph-phase\]",line)]
                    if candidates and candidates[-1]!=last_service_line:
                        last_service_line=candidates[-1];last_line=last_service_line;last_progress=tick
            except OSError:pass
        if log.exists():
            with log.open("rb") as native:
                native.seek(native_offset);new=native.read();native_offset=native.tell()
            native_partial+=new
            complete=native_partial.split(b"\n");native_partial=complete.pop()
            for raw in complete:
                line=raw.decode("utf8","replace")
                if "Another process holds the dependency store; waiting for it" in line:
                    if wait_since is None:wait_since=tick
                    wait_line=line
                elif line.strip() and not re.search(r"still (working|running)|waiting for|remains active",line,re.I):
                    key=re.sub(r'"elapsedMs":\s*[0-9.]+','"elapsedMs":0',line)
                    fleet_match=re.search(r"waiting for the fleet:\s*(\d+) of",line,re.I)
                    progress=True
                    if fleet_match:
                        delivered=int(fleet_match.group(1));progress=delivered>fleet_delivered;fleet_delivered=max(fleet_delivered,delivered)
                    if '"stage":"running"' in key and '"activity"' not in key:progress=False
                    if progress and key not in progress_keys:
                        progress_keys.add(key);last_line=line;last_progress=tick
                    if wait_since is not None and re.search(r"succeeded|complete|graph: plan stage|graph: restored",line,re.I):wait_since=None
        fp="not measured"
        if slices:
            pid,rss=slices[0][0],slices[0][2]/1024
            text=command(["footprint","-p",str(pid)],timeout=4)
            fp=next((line.strip()[:180] for line in text.splitlines() if "footprint" in line.lower()),"not measured")
            if rss>4096 and not sampled:
                sampled=True
                stack_path=str(prefix)+".stack-"+datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")+".txt"
                try:stack_process=subprocess.Popen(["sample",str(pid),"5","-file",stack_path],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
                except OSError:append(samples,"stack sample: not measured (sample command unavailable)")
        append(samples,"%s elapsed=%.1fs run_peak=%.1fMiB slice_peak=%.1fMiB footprint=[%s] last=[%s]"%(utc(),tick-started,run_peak,slice_peak,fp,last_service_line[:220]))
        if higher_peak:
            peak_at={"at":utc(),"progress":last_service_line[:500],"run_mib":run_peak,"slice_mib":slice_peak}
        if tick>=next_30:
            next_30=tick+30
            text=command(["/bin/sh","-c","ps -axo rss,command | sort -nr | head -3"])
            append(top,utc()+"\n"+text)
            values=[int(line.split(None,1)[0])/1024 for line in text.splitlines() if line.strip() and line.split(None,1)[0].isdigit()]
            peak_system=max([peak_system]+values)
            pressure=command(["sysctl","-n","kern.memorystatus_vm_pressure_level"])
            append(samples,utc()+" memory_pressure_level="+(pressure or "not measured"))
            pressure_streak=pressure_streak+1 if pressure=="4" else 0
            print("try=%s elapsed=%.0fs run_peak=%.0fMiB slice_peak=%.0fMiB pressure=%s"%(attempt,tick-started,run_peak,slice_peak,pressure or "not measured"),flush=True)
        if max(run_peak,slice_peak)>8192:stop_reason="resident-above-8GiB"
        elif pressure_streak>=2:stop_reason="memory-pressure-level-4-two-samples"
        elif wait_since is not None and tick-wait_since>=300:stop_reason="dependency-store-wait-5-minutes"
        elif tick-last_progress>=900:stop_reason="no-new-progress-15-minutes"
        if stop_reason:
            selected=[str(pid)+" "+str(parent)+" "+args for pid,parent,_,args in rows if re.search(r"bun|nx|repo",args,re.I)]
            (pathlib.Path(str(prefix)+".stop-processes.txt")).write_text(command(["ps","-axo","pid,ppid,etime,command"])+"\n")
            if service:(pathlib.Path(str(prefix)+".stop-service-tail.txt")).write_text(command(["tail","-n","60",service])+"\n")
            stop(child,stop_reason);break
        time.sleep(max(.2,5-(time.monotonic()-tick)))
except KeyboardInterrupt:
    stop_reason="operator-interrupt"
    stop(child,stop_reason)
wrapper_rc=child.wait()
wrapper_output.close()
if stack_process:
    try:stack_process.wait(timeout=10)
    except subprocess.TimeoutExpired:stack_process.terminate();stack_process.wait()
end=utc(); wall=time.monotonic()-started
after=closure_count()
append(samples,"closure entries after: "+str(after))
append(samples,"end="+end)
pathlib.Path(str(samples)+".stop").touch()
text=log.read_text(errors="replace") if log.exists() else ""
matches=re.findall(r"native_rc=(-?\d+)",text)
native_rc=int(matches[-1]) if matches else (128-wrapper_rc if wrapper_rc<0 else wrapper_rc)
marker=(root/".cache/f/native-bootstrap.done").exists()
metrics={"round":round_number,"try":attempt,"start":start,"end":end,"wall_seconds":wall,"wrapper_rc":wrapper_rc,"native_rc":native_rc,"native_rc_from_log":bool(matches),"marker":marker,"ready":native_rc==0 and marker,"stop_reason":stop_reason,"peak_run_rss_mib":peak_run,"peak_slice_rss_mib":peak_slice,"peak_system_rss_mib":peak_system,"peak_at":peak_at,"closure_entries_before":before,"closure_entries_after":after,"dependency_wait_line":wait_line,"service_logs":list(service_offsets)}
pathlib.Path(str(prefix)+".metrics.json").write_text(json.dumps(metrics,ensure_ascii=False,indent=2)+"\n")
pathlib.Path(str(prefix)+".tail.txt").write_text("".join(text.splitlines(keepends=True)[-200:]))
failed=[] if native_rc==0 else ["native-bootstrap setup"]
if native_rc==0 and not marker:failed=["native-bootstrap completion marker"]
(own/"state.json").write_text(json.dumps({"origin":"g1-m10","round":round_number,"native_rc":native_rc,"ready":metrics["ready"],"failed":failed,"at":end.replace("+00:00","Z")})+"\n")
print(json.dumps(metrics,ensure_ascii=False),flush=True)
