import subprocess,os,re,collections
A="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model"
S=A+"/🏅️standards/🔖️1/🪆️subsets/✳️any/"
fs=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","--",A],capture_output=True,text=True).stdout.split("\n")
fs=[f for f in fs if f.endswith(".rs") and os.path.isfile(f)]
rows=[]
for f in fs:
    t=open(f).read()
    n=len(re.findall(r'^[ \t]*#\[path = "\."\]',t,flags=re.M))
    if n: rows.append((n,t.count("\n"),f.replace(S,"A/")))
for r in sorted(rows,reverse=True)[:60]: print(*r)
print(len(rows),sum(r[0] for r in rows))
