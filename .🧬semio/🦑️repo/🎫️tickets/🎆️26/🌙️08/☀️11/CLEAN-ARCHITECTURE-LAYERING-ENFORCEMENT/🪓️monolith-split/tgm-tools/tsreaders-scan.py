import re,subprocess,sys,os
ROOT='/Users/ueli/Documents/semio'
out=subprocess.run(['git','-c','core.quotepath=false','ls-files','-z','--','*.ts','*.tsx','*.mjs','*.cjs','*.js',':!.🧬semio'],cwd=ROOT,capture_output=True).stdout.decode('utf-8','replace').split('\0')
rx=re.compile(r'(⏯️tool-run|🛠️tool-machine|🔄️machine|🗣️dsl|⚙️engine|⏯️layout-run)[\s"\'`,)\]+/]{0,20}🦀️\.rs')
n=0
for rel in out:
    if not rel or 'node_modules' in rel: continue
    p=os.path.join(ROOT,rel)
    try:
        if os.path.getsize(p)>600000: continue
        tx=open(p,encoding='utf-8',errors='replace').read()
    except Exception: continue
    n+=1
    for m in rx.finditer(tx):
        ln=tx.count('\n',0,m.start())+1
        ctx=tx[max(0,m.start()-60):m.end()+20].replace('\n',' ')
        print(rel[-95:],ln,'|',ctx[-140:])
print('scanned',n)
