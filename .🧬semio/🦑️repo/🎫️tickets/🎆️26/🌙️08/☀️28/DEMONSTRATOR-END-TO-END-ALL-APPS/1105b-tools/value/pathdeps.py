import subprocess,os,re
files=[f for f in subprocess.run(["git","ls-files","--","*Cargo.toml",":!.🧬semio"],capture_output=True,text=True).stdout.split('\n') if f and os.path.exists(f)]
n=0
for f in files:
    for l in open(f).read().split('\n'):
        for m in re.finditer(r'path\s*=\s*"([^"]+)"',l):
            q=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
            if not os.path.exists(os.path.join(q,'Cargo.toml')) and not q.endswith('.rs'):
                if 'workspace' in l and 'members' in l:continue
                n+=1;print(f[-90:],'->',m.group(1)[-80:])
print(n,"dangling path deps")
