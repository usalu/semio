import re,sys,os,subprocess
log=sys.argv[1]
C='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/'
s=open(log,errors='replace').read()
files=set()
for m in re.finditer(r'error(?:\[E\d+\])?: ([^\n]*)\n\s*--> ([^\n]*?):(\d+):(\d+)\n',s):
    if 'could not' in m.group(1):continue
    f=os.path.normpath(m.group(2))
    if os.path.isfile(f) and '🛍️products' in f:files.add(f)
todo=[f for f in files if re.search(r'(?<![\w])(?:semio_framework_job::)?(admit_original_construction|close_original_retained_step)\(',open(f).read()) and 'fn built' not in open(f).read().split('admit_original_construction(')[0][-0:]]
# skip files that already define helpers or are the helper definitions
todo=[f for f in todo if not re.search(r'fn (built|retired)\b',open(f).read())]
if todo:
    r=subprocess.run(['python3',C+'1101-tools/replay/wrapfile.py']+todo,capture_output=True,text=True);print(r.stdout.strip())
    r=subprocess.run(['python3',C+'1101-tools/replay/unwrap_outcome.py']+todo,capture_output=True,text=True)
print('autowrap',len(todo))
