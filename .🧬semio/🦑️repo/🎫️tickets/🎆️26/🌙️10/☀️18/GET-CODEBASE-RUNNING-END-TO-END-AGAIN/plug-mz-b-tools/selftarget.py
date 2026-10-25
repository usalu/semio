import subprocess,re,sys
sys.path.insert(0,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/plug-mz-b-tools")
import tjf
files=subprocess.run(['rg','-l',r'self\.target\(\)','✏️s/🔌️plugins','-g','🦀️.rs'],capture_output=True,text=True).stdout.split('\n')
tot=0
for f in files:
    if not f.strip(): continue
    s=open(f,encoding='utf-8').read();o=s
    for m in reversed(list(re.finditer(r'impl<Op[^>]*>\s*(?:protocol::|::protocol::|semio_framework_os_kernel::)?ColdMutationKind<([\w:]+),\s*Op>\s*for\s+\w+[^{]*\{',s))):
        ob=m.end()-1;cb=tjf.match_brace(s,ob)
        body=s[ob:cb]
        if 'self.target()' in body:
            snap=m.group(1)
            nb=body.replace('self.target()',f'<Self as protocol::MutationKind<{snap}, Op>>::target(self)')
            s=s[:ob]+nb+s[cb:];tot+=1
    if s!=o: open(f,'w',encoding='utf-8').write(s)
print(tot)
