import re,subprocess,sys
write='--write' in sys.argv
files=subprocess.run(['rg','-l',r'Clone.*FromValue.*Mutation<\w+>|FromValue.*Clone.*Mutation<\w+>','--glob','*.rs','🧰️framework','✏️s','🌎️hub'],capture_output=True,text=True).stdout.split('\n')
pat=re.compile(r'(?m)^(\s*(?:where\s+)?(?:\w+\s*:\s*[^,{;\n]*?,\s*)?(?:Mutation|M)\s*:\s*)([^{;\n]*?\b(?:\w+::)*Mutation<(\w+)>)([^{;\n]*)$')
tot=0;changed=[]
for f in files:
    if not f: continue
    s=open(f,errors='ignore').read()
    def rep(m):
        global tot
        line=m.group(0)
        if 'ColdMutation' in line or 'Clone' not in line or 'FromValue' not in line: return line
        tot+=1
        x=m.group(3)
        return m.group(1)+m.group(2)+f' + semio_framework_os_spr::ColdMutation<{x}>'+m.group(4)
    n=pat.sub(rep,s)
    if n!=s:
        changed.append(f)
        if write: open(f,'w').write(n)
print(tot,len(changed))
for f in changed[:60]: print(f[-100:])
