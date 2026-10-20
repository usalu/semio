import os,re,sys,subprocess
apply=len(sys.argv)>1 and sys.argv[1]=='apply'
files=subprocess.run(['/usr/bin/grep','-rl','--include=🟦️.ts',r'\.\./\.\./🧫️fixtures','✏️s'],capture_output=True,text=True).stdout.split('\n')
rx=re.compile(r'(\$\{here\}|\$\{dir\}|\$\{import\.meta\.dir\}|import\.meta\.dir,\s*)?([^\s`"\']*)\.\./\.\./🧫️fixtures/([^`"\']*)')
for f in files:
    if not f or 'node_modules' in f: continue
    d=os.path.dirname(f);s=open(f).read()
    for m in rx.finditer(s):
        full=m.group(0)
        old=os.path.normpath(os.path.join(d,'../../🧫️fixtures',m.group(3)))
        new=os.path.normpath(os.path.join(d,'../🧫️fixtures',m.group(3)))
        if not os.path.exists(old) and os.path.exists(new): print('FIX',f[-110:],m.group(3))
        elif not os.path.exists(old) and not os.path.exists(new): print('NEITHER',f[-110:],m.group(3))
