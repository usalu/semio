import subprocess,sys
root=int(sys.argv[1]);needle=sys.argv[2] if len(sys.argv)>2 else ""
rows=[l.split(None,2) for l in subprocess.run(["ps","-axo","pid=,ppid=,command="],capture_output=True,text=True).stdout.splitlines() if l.strip()]
kids={}
for pid,ppid,cmd in rows:kids.setdefault(int(ppid),[]).append((int(pid),cmd))
out=[];stack=[root]
while stack:
    cur=stack.pop()
    for pid,cmd in kids.get(cur,[]):
        out.append((pid,cmd));stack.append(pid)
for pid,cmd in out:
    if needle in cmd:print(pid,cmd[:160])
