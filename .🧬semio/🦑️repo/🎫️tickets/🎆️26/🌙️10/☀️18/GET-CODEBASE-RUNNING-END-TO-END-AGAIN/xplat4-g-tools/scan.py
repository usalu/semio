import json,re,sys,os,collections
T=os.path.dirname(os.path.abspath(__file__))
d=json.load(open(T+"/enum.json"))
F={
 "shell":re.compile(r"""(?:["'`](?:sh|bash|zsh|dash|ps|kill|chmod|flock|ln|cp|mv|rm|cat|ls|touch|sleep|true|false|id|uname|stat|df|du|mount|umount|lsof|pgrep|pkill|tar|find|grep|sed|awk|head|tail|wc|sort|mkfifo|which|env|yes|tee|dd|nice|renice|taskset)["'`]\s*,\s*\[)|/bin/(?:sh|bash|ls|cat|echo|sleep|kill|ps)|/usr/bin/|#!/|\bsh\s+-c\b|\bexecSync\(|\bspawnSync\(["'`](?:sh|bash)"""),
 "tmp":re.compile(r"""["'`]/(?:tmp|var|etc|dev|proc|sys|opt|usr|home|Users|Volumes|private)\b"""),
 "symlink":re.compile(r"symlinkSync|\.symlink\(|ln -s|readlinkSync"),
 "mode":re.compile(r"chmodSync|\.chmod\(|0o[0-7]{3}\b|mode\s*&|\.mode\b"),
 "procgrp":re.compile(r"process\.kill\(\s*-|getuid|getgid|geteuid|setsid|SIGSTOP|SIGCONT|SIGUSR|\bpgid\b|detached\s*:\s*true|mkfifo|\bfcntl\b|\bflock\b|O_NOFOLLOW|O_EXLOCK"),
 "darwin":re.compile(r"hdiutil|diskutil|sandbox-exec|vm_stat|sysctl|APFS|Seatbelt|seatbelt|libSystem|\.dmg\b|clonefile|launchctl|codesign|xattr|FICLONE"),
 "linux":re.compile(r"landlock|/proc/|cgroup|smaps|/sys/fs"),
 "gate":re.compile(r"skipIf|process\.platform|win32|\"darwin\"|\"linux\""),
 "python3":re.compile(r"[\"'`]python3[\"'`]"),
}
def scan(path):
    try: txt=open(path,encoding="utf8",errors="replace").read()
    except Exception as e: return None
    r={}
    for k,rx in F.items():
        m=rx.findall(txt)
        if m: r[k]=len(m)
    return r
rows=[]
for x in d:
    tot=collections.Counter(); per=[]
    for f in x["files"]:
        p=os.path.join(x["root"],f)
        s=scan(p)
        if s is None: per.append((f,"MISSING")); continue
        tot.update(s); per.append((f,s))
    x["features"]=dict(tot); x["per"]=per
json.dump(d,open(T+"/scan.json","w"),ensure_ascii=False)
cnt=collections.Counter()
for x in d:
    hard={k:v for k,v in x["features"].items() if k not in("gate",)}
    cnt["clean" if not hard else "hits"]+=1
print(cnt)
