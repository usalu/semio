import json,subprocess,re,sys
BUN=subprocess.check_output("ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1",shell=True,text=True).strip()
asset=sys.argv[1]
out=subprocess.run([BUN,'volumes-check.ts','furniture/'+asset,'--scale','50','--rules','volumes-thermal-coverage'],capture_output=True,text=True).stdout
new={}
for m in re.finditer(r'areaDeviation:([a-z-]+)=[^ ]+ \[[^\]]*?exposed ([0-9.]+) m2 of',out):
    new[m.group(1)]=round(float(m.group(2)),3)
print(asset,new)
import subprocess as sp
if new and '--apply' in sys.argv:
    print(sp.run(['python3','author-16-area.py',asset,json.dumps(new)],capture_output=True,text=True).stdout)
