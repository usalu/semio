import re,sys,os,subprocess
log=sys.argv[1]
C='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/'
print(subprocess.run(['python3',C+'1101-tools/replay/autofix.py',log,'🛍️products','--apply'],capture_output=True,text=True).stdout.strip())
s=open(log,errors='replace').read()
files=set()
for m in re.finditer(r'error(?:\[E\d+\])?: ([^\n]*)\n\s*--> ([^\n]*?):(\d+):(\d+)\n',s):
    if 'could not' in m.group(1):continue
    f=os.path.normpath(m.group(2))
    if os.path.isfile(f) and '🛍️products' in f:files.add(f)
n=0
for f in files:
    t=open(f).read()
    u=re.sub(r'(\.retire_owned\(\w+, ?child\)|\.retire\(\w+, ?child\)) \{',r'\1.result {',t)
    if u!=t:open(f,'w').write(u);n+=1
print('retire result files',n,len(files))
