import re,sys,subprocess,os,hashlib,shutil
live,cand,base=sys.argv[1],sys.argv[2],sys.argv[3]
files=['🗺️topology/🟦️.ts','🗺️topology/🧪️tests/🟦️.ts','🔁️replica/💾️state/🧬️schema/🔣️.json']
def h(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()[:16]
for f in files:
    L,C=os.path.join(live,f),os.path.join(cand,f)
    b=base+'/'+f
    r=subprocess.run(['git','merge-file','-p',L,b,C],capture_output=True)
    out=r.stdout.decode('utf-8')
    if r.returncode!=0:
        m=re.search(r'<<<<<<< [^\n]*\n(.*?)=======\n(.*?)>>>>>>> [^\n]*\n',out,re.S)
        assert r.returncode==1 and f.endswith('topology/🟦️.ts') and m,(f,r.returncode)
        ours=m.group(1)
        a='RequiresMountsFor=${topology.storeMount.linux}`,"",'
        assert a in ours
        ours=ours.replace(a,'RequiresMountsFor=${topology.storeMount.linux}`,UNLIMITED_STARTS,"",')
        b='`ExecStart=/bin/sh ${entry}/boot.sh run`,'
        assert ours.count(b)==1
        ours=ours.replace(b,b+'...STOP_BOUND,')
        out=out[:m.start()]+ours+out[m.end():]
        assert '<<<<<<<' not in out
    r.stdout=out.encode('utf-8')
    before=h(L);open(L,'wb').write(r.stdout);print('|',f,'|',before,'|',h(L),'|')
p=os.path.join(live,'README.md')
s=open(p,encoding='utf-8').read()
a='- **Limits** are environment variables of the entry:'
assert s.count(a)==1
new='- **Service definitions bound crash loops and stops.** The systemd units (replica, cache, relay) set `StartLimitIntervalSec=0`, so systemd\'s default start limit (5 starts in 10 s) can never leave a crash-looping unit `failed` and dead; with `RestartSec=2` the default sat exactly on that boundary. The replica unit sets `KillMode=mixed` and `TimeoutStopSec=30`: `SIGTERM` reaches the supervisor only (it forwards it and kills a blocked child after 20 s), `SIGKILL` follows for everything at 30 s instead of the default 90 s. The LaunchDaemon plist keeps `KeepAlive` and `ThrottleInterval` 5 (launchd has no start limit) and sets `ExitTimeOut` 30 (default 20 s equals the supervisor grace). The Windows task restarts on failure (every minute, 255 times, `ExecutionTimeLimit` `PT0S`) and is started again by the minute trigger when the count is used up. Verified on Debian 13 (systemd 257): a `/bin/false` unit with `RestartSec=100ms` stays `failed` after 5 restarts with the old text and keeps restarting with the new one; a TERM-ignoring supervisor is killed at 30 s.\n'
s=s.replace(a,new+a)
open(p,'w',encoding='utf-8').write(s);print('| README.md | - |',h(p),'|')
