"""📇️ Lists every trinity Cargo manifest (tracked and untracked): crate name, relative path. usage: list-crates.py [--deps]"""
import subprocess,sys,re,os
os.chdir('/Users/ueli/Documents/semio')
roots=['✏️s/🔌️plugins/🔱️trinity','✏️s/🧑‍💻dev/🔱️trinity']
files=set()
for r in roots:
    for a in (['ls-files'],['ls-files','--others','--exclude-standard']):
        out=subprocess.run(['git','-c','core.fsmonitor=false','-c','core.quotepath=false']+a+['--',r],capture_output=True,text=True).stdout
        for l in out.splitlines():
            if l.endswith('Cargo.toml') and os.path.exists(l): files.add(l)
for f in sorted(files):
    t=open(f).read()
    m=re.search(r'^name\s*=\s*"([^"]+)"',t,re.M)
    print(m.group(1) if m else '?', '|', f.replace('✏️s/🔌️plugins/🔱️trinity/','P/').replace('🗿️artifacts/','A/'))
