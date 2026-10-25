import os,re,subprocess,sys
root=os.getcwd()
pat=sys.argv[1]
out=subprocess.run(["rg","-n","--no-ignore","-g","!node_modules","-g","!.cache","-g","!target","-F",pat,"🧰️framework","--glob","*.ts","--glob","*.mjs","--glob","*.js"],capture_output=True,text=True).stdout.splitlines()
for line in out:
    f,ln,text=line.split(":",2)
    for m in re.finditer(r'["\'`](\.{1,2}/[^"\'`]*)["\'`]',text):
        spec=m.group(1)
        if pat not in spec: continue
        p=os.path.normpath(os.path.join(os.path.dirname(f),spec))
        if not os.path.exists(p): print("DANGLING",f,ln,spec)
