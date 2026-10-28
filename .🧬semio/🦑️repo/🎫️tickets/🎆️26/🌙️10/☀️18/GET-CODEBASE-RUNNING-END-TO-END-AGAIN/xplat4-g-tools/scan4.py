import json,re,sys,os,collections
T=os.path.dirname(os.path.abspath(__file__))
d=json.load(open(T+"/enum.json"))
POSIX_CMDS="sh|bash|zsh|dash|ps|kill|chmod|flock|ln|cp|mv|rm|cat|ls|touch|sleep|true|false|id|uname|stat|df|du|mount|umount|lsof|pgrep|pkill|find|grep|sed|awk|head|tail|wc|sort|mkfifo|which|yes|tee|dd|nice|renice|taskset|hdiutil|diskutil|sandbox-exec|sysctl|vm_stat|codesign|xattr|launchctl"
SPAWN=r"(?:spawn|spawnSync|exec|execSync|execFile|execFileSync|captureOwnedCommandV1|runOwnedCommandV1|runOwnedCommand|captureOwnedCommand|Bun\.spawn|Bun\.spawnSync|system|run)"
T1={
 "shell-spawn":re.compile(r"\b"+SPAWN+r"\s*\(\s*(?:\[\s*)?[\"'`](?:/usr/bin/|/bin/)?(?:"+POSIX_CMDS+r")[\"'`]"),
 "shell-text":re.compile(r"#!/(?:bin|usr)|/bin/(?:sh|bash)\b|[\"'`]sh[\"'`]\s*,\s*\[?\s*[\"'`]-c|\bexecSync\s*\("),
 "abs-tool":re.compile(r"[\"'`]/(?:usr/bin|bin|usr/sbin|sbin)/(?:python3|hdiutil|diskutil|sandbox-exec|sh|bash|ps|kill|ls|cat|chmod|df|stat)\b"),
 "python3-name":re.compile(r"\b"+SPAWN+r"\s*\(\s*(?:\[\s*)?[\"'`]python3[\"'`]"),
 "symlink":re.compile(r"\bsymlinkSync\s*\(|\bfs\.symlink\w*\(|\bawait\s+symlink\s*\("),
 "chmod":re.compile(r"\bchmodSync\s*\(|\bfs\.chmod\w*\(|\.mode\s*&\s*0o|mode\s*&\s*0o"),
 "pgroup":re.compile(r"process\.kill\(\s*-|\bSIGSTOP|\bSIGCONT|\bSIGUSR[12]|\bmkfifo|\bimport\s+fcntl|\bfcntl\.flock|O_NOFOLLOW|\bos\.getuid|\bprocess\.getuid\(\)"),
}
LINEGATE=re.compile(r"process\.platform|\bwin32\b|\bposix\b|available|symlinks\b")
TEST=re.compile(r"^\s*(?:export\s+)?(?:test|it|describe)(?:\.[A-Za-z]+(?:\([^)]*\))?)*\s*\(")
GATE=re.compile(r"skipIf|\.skip\b|test\.todo")
for x in d:
    res=collections.Counter(); ex=[]; files=collections.Counter()
    for f in x["files"]:
        p=os.path.join(x["root"],f)
        try: lines=open(p,encoding="utf8",errors="replace").read().split("\n")
        except: continue
        cur=None;g=False
        for i,l in enumerate(lines,1):
            if TEST.match(l): cur=l.strip()[:80]; g=bool(GATE.search(l))
            if g or (LINEGATE.search(l) and not TEST.match(l) and False): continue
            for k,rx in T1.items():
                if rx.search(l):
                    if LINEGATE.search(l) and k in("symlink","chmod","pgroup"): continue
                    res[k]+=1; files[f]+=1
                    if len(ex)<60: ex.append((k,f.split("/")[-3] if f.count("/")>2 else f,i,l.strip()[:170]))
    x["t1"]=dict(res); x["t1ex"]=ex; x["t1files"]=dict(files)
json.dump(d,open(T+"/scan4.json","w"),ensure_ascii=False)
print(collections.Counter("clean" if not x["t1"] else "hit" for x in d))
