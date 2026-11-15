import sys,hashlib
root=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding='utf-8').read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:60],s.count(a))
        s=s.replace(a,b)
    open(path,'w',encoding='utf-8').write(s)
t=root+'/🗺️topology/🟦️.ts'
edit(t,[
('const KEEP_ALIVE=["<key>RunAtLoad</key><true/>","<key>KeepAlive</key><true/>","<key>ThrottleInterval</key><integer>5</integer>"];',
 'const KEEP_ALIVE=["<key>RunAtLoad</key><true/>","<key>KeepAlive</key><true/>","<key>ThrottleInterval</key><integer>5</integer>","<key>ExitTimeOut</key><integer>30</integer>"],\n UNLIMITED_STARTS="StartLimitIntervalSec=0",\n STOP_BOUND=["KillMode=mixed","TimeoutStopSec=30"];'),
('"After=network-online.target remote-fs.target",`RequiresMountsFor=${topology.storeMount.linux}`,"",\n  "[Service]","User=%i",`WorkingDirectory=${directory}`,`ExecStart=${REPO} ${SCRIPT} run`,\n',
 '"After=network-online.target remote-fs.target",`RequiresMountsFor=${topology.storeMount.linux}`,UNLIMITED_STARTS,"",\n  "[Service]","User=%i",`WorkingDirectory=${directory}`,`ExecStart=${REPO} ${SCRIPT} run`,...STOP_BOUND,\n'),
('"After=network-online.target","Wants=network-online.target","",\n   "[Service]",`User=${STORE_USER}`,`Group=${STORE_USER}`,`WorkingDirectory=${worktree}`,`EnvironmentFile=',
 '"After=network-online.target","Wants=network-online.target",UNLIMITED_STARTS,"",\n   "[Service]",`User=${STORE_USER}`,`Group=${STORE_USER}`,`WorkingDirectory=${worktree}`,`EnvironmentFile='),
('`ConditionPathExists=${worktree}/.cache/f/machine.json`,"",','`ConditionPathExists=${worktree}/.cache/f/machine.json`,UNLIMITED_STARTS,"",'),
])
edit(root+'/🗺️topology/🧪️tests/🟦️.ts',[
(' expect(linux).toContain("RestartSec=2");\n',' expect(linux).toContain("RestartSec=2");\n expect(linux).toContain("StartLimitIntervalSec=0");\n expect(linux).toContain("KillMode=mixed");\n expect(linux).toContain("TimeoutStopSec=30");\n expect(macos).toContain("<key>ThrottleInterval</key><integer>5</integer>");\n expect(macos).toContain("<key>ExitTimeOut</key><integer>30</integer>");\n expect(windows).toContain("<RestartOnFailure><Interval>PT1M</Interval><Count>255</Count></RestartOnFailure>");\n expect(windows).toContain("<ExecutionTimeLimit>PT0S</ExecutionTimeLimit>");\n'),
(' expect(parts["exports-target"]).toBe("/etc/exports.d/semio-fleet.exports");\n',' expect(parts["exports-target"]).toBe("/etc/exports.d/semio-fleet.exports");\n for(const unit of ["cache-unit","relay-unit","replica-unit"])expect(parts[unit]).toContain("StartLimitIntervalSec=0");\n'),
])
