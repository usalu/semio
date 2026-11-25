"""🔗️ Replays only the linker command (`cc`, printed by `rustc --print link-args`) of one finished variant with extra linker flags into a scratch output and prints, for the whole process tree of that one command, the peak resident set (ps, every 0.25 s) and the peak footprint (`footprint`, the measure of the host warden, every second) next to the wall time.
Every second the footprint of the tree is measured; above 7000 MB the whole tree is killed (the host warden does not know this process).
Usage: python3 🔗️link-only.py <variant-dir> <label> [extra cc arguments...]"""
import os,re,shlex,subprocess,sys,time
variant,label,extra=sys.argv[1],sys.argv[2],sys.argv[3:]
text=open(os.path.join(variant,"out.txt"),errors="replace").read()
line=next(row for row in text.splitlines() if '"cc"' in row)
head,_,tail=line.partition('"cc"')
words=[re.sub(r"\\u\{([0-9a-f]+)\}",lambda m:chr(int(m.group(1),16)),word) for word in shlex.split('"cc"'+tail)]
out=os.path.join(variant,"link-only-"+label)
words[words.index("-o")+1]=out
env=dict(os.environ);env["LC_ALL"]="C"
for part in shlex.split(head):
    if "=" in part and not part.startswith("-") and part!="env":
        key,_,value=part.partition("=");env[key]=value
for name in ("IPHONEOS_DEPLOYMENT_TARGET","TVOS_DEPLOYMENT_TARGET","XROS_DEPLOYMENT_TARGET"):env.pop(name,None)
def tree(root):
    rows=subprocess.run(["ps","-axo","pid=,ppid=,rss="],capture_output=True,text=True).stdout.split("\n")
    kids={};rss={}
    for row in rows:
        parts=row.split()
        if len(parts)==3:pid,ppid,r=map(int,parts);kids.setdefault(ppid,[]).append(pid);rss[pid]=r
    pids=[root];seen=set(pids)
    for pid in pids:
        for kid in kids.get(pid,[]):
            if kid not in seen:seen.add(kid);pids.append(kid)
    return pids,sum(rss.get(pid,0) for pid in pids)
started=time.time();child=subprocess.Popen(words+extra,env=env,cwd=variant,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,errors="replace")
peak=0;foot=0;last=0
while child.poll() is None:
    pids,total=tree(child.pid);peak=max(peak,total)
    if time.time()-last>=1 and len(pids)>1:
        last=time.time();args=[a for pid in pids for a in("-p",str(pid))]
        done=subprocess.run(["footprint"]+args,capture_output=True,text=True,errors="replace").stdout
        match=re.findall(r"Footprint:\s+([\d.]+)\s+(KB|MB|GB)",done)
        if match:
            value,unit=match[-1];foot=max(foot,float(value)*{"KB":1/1024,"MB":1,"GB":1024}[unit])
        if foot>7000:
            for pid in reversed(pids):
                try:os.kill(pid,9)
                except OSError:pass
            break
    time.sleep(0.25)
child.wait();stderr=child.stderr.read();wall=round(time.time()-started)
print(label,"wall=%ds"%wall,"rssPeak=%d MB"%(peak//1024),"footprintPeak=%d MB"%foot,"exit=%d"%child.returncode,(stderr.strip().splitlines() or [""])[0][:140] if child.returncode else "")
if os.path.exists(out):os.remove(out)
