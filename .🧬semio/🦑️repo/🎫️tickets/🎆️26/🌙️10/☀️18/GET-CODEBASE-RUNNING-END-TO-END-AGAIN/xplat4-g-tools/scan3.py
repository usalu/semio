import json,re,sys,os,collections
T=os.path.dirname(os.path.abspath(__file__))
d=json.load(open(T+"/enum.json"))
POSIX_CMDS="sh|bash|zsh|dash|ps|kill|chmod|flock|ln|cp|mv|rm|cat|ls|touch|sleep|true|false|id|uname|stat|df|du|mount|umount|lsof|pgrep|pkill|tar|find|grep|sed|awk|head|tail|wc|sort|mkfifo|which|yes|tee|dd|nice|renice|taskset|hdiutil|diskutil|sandbox-exec|sysctl|vm_stat|codesign|xattr|launchctl|touch"
T1={
 "shell":re.compile(r"(?:\b(?:spawn|spawnSync|exec|execSync|execFile|execFileSync|captureOwnedCommandV1|runOwnedCommandV1|runOwnedCommand|captureOwnedCommand|Bun\.spawn|Bun\.spawnSync)\s*\(\s*(?:\[\s*)?[\"'`](?:"+POSIX_CMDS+r")[\"'`])|\bexecSync\(|(?:^|[^\w])(?:/bin/(?:sh|bash)|/usr/bin/\w+)|#!/(?:bin|usr)|[\"'`]sh[\"'`]\s*,\s*\[?\s*[\"'`]-c|\[\s*[\"'`](?:"+POSIX_CMDS+r")[\"'`]\s*,"),
 "symlink":re.compile(r"\bsymlinkSync\s*\(|\bfs\.symlink\w*\(|\bsymlink\s*\(|ln -s"),
 "chmod":re.compile(r"\bchmodSync\s*\(|\bchmod\s*\(|0o[0-7]{3}\b"),
 "pgroup":re.compile(r"process\.kill\(\s*-|\bgetuid\b|\bgetgid\b|\bgeteuid\b|\bsetsid\b|SIGSTOP|SIGCONT|SIGUSR[12]|\bmkfifo|\bflock\b|O_NOFOLLOW"),
 "hostimage":re.compile(r"hdiutil|diskutil|sandbox-exec|vm_stat|sysctl|\bAPFS\b|clonefile|launchctl|\bcodesign\b"),
 "kernelfs":re.compile(r"landlock|/proc/|cgroup"),
 "hardpath":re.compile(r"[\"'`]/(?:tmp|var|etc|dev|opt|usr|Volumes|private)/"),
}
TEST=re.compile(r"^\s*(?:export\s+)?(?:test|it|describe)(?:\.[A-Za-z]+(?:\([^)]*\))?)*\s*\(")
GATE=re.compile(r"skipIf|\.skip\b|test\.todo")
for x in d:
    res=collections.Counter(); ex=[]
    for f in x["files"]:
        p=os.path.join(x["root"],f)
        try: lines=open(p,encoding="utf8",errors="replace").read().split("\n")
        except: continue
        cur=None;g=False
        for i,l in enumerate(lines,1):
            if TEST.match(l): cur=l.strip()[:80]; g=bool(GATE.search(l))
            if g: continue
            for k,rx in T1.items():
                if rx.search(l):
                    res[k]+=1
                    if len(ex)<40: ex.append((k,f.split("/")[-3] if f.count("/")>2 else f,i,l.strip()[:160]))
    x["t1"]=dict(res); x["t1ex"]=ex
json.dump(d,open(T+"/scan3.json","w"),ensure_ascii=False)
c=collections.Counter("clean" if not x["t1"] else "hit" for x in d); print(c)
