import json,re,sys,os,collections
T=os.path.dirname(os.path.abspath(__file__))
d=json.load(open(T+"/enum.json"))
HARD={
 "shell":re.compile(r"""(?:["'`](?:sh|bash|zsh|dash|ps|kill|chmod|flock|ln|cp|mv|rm|cat|ls|touch|sleep|true|false|id|uname|stat|df|du|mount|umount|lsof|pgrep|pkill|tar|find|grep|sed|awk|head|tail|wc|sort|mkfifo|which|yes|tee|dd|nice|renice|taskset)["'`]\s*,\s*\[)|/bin/(?:sh|bash|ls|cat|echo|sleep|kill|ps)|/usr/bin/|#!/|\bsh\s+-c\b|["'`]sh["'`]"""),
 "tmp":re.compile(r"""["'`]/(?:tmp|var|etc|dev|proc|sys|opt|usr|home|Users|Volumes|private)\b"""),
 "symlink":re.compile(r"symlinkSync|\.symlink\(|ln -s"),
 "chmod":re.compile(r"chmodSync|\.chmod\(|statSync\([^)]*\)\.mode|\.mode\s*&"),
 "procgrp":re.compile(r"process\.kill\(\s*-|getuid|getgid|geteuid|setsid|SIGSTOP|SIGCONT|SIGUSR|mkfifo|\bfcntl\b|\bflock\b|O_NOFOLLOW|O_EXLOCK"),
 "darwin":re.compile(r"hdiutil|diskutil|sandbox-exec|vm_stat|sysctl|APFS|Seatbelt|seatbelt|libSystem|\.dmg\b|clonefile|launchctl|codesign|xattr|FICLONE"),
 "linux":re.compile(r"landlock|/proc/|cgroup|smaps|/sys/fs"),
}
TEST=re.compile(r"^\s*(?:export\s+)?(?:test|it|describe)(?:\.[A-Za-z]+(?:\([^)]*\))?)*\s*\(")
GATE=re.compile(r"skipIf|\.skip\b|test\.todo")
out=[]
for x in d:
    rows=[]
    for f in x["files"]:
        p=os.path.join(x["root"],f)
        try: lines=open(p,encoding="utf8",errors="replace").read().split("\n")
        except: continue
        cur=None;curgated=False;curline=0;hits=collections.defaultdict(list)
        for i,l in enumerate(lines,1):
            if TEST.match(l):
                cur=l.strip()[:110];curgated=bool(GATE.search(l)); curline=i
            for k,rx in HARD.items():
                if rx.search(l):
                    hits[(cur,curgated)].append((k,i))
        for (name,g),h in hits.items():
            rows.append(dict(file=f,test=name,gated=g,kinds=sorted(set(k for k,_ in h)),n=len(h),first=h[0][1]))
    x["hits"]=rows
json.dump(d,open(T+"/scan2.json","w"),ensure_ascii=False)
