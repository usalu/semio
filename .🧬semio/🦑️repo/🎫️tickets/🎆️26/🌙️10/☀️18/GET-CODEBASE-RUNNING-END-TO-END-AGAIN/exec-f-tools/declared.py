"""python3 declared.py <workspace> : per task of a published Bazel workspace, the declared source files (BUILD srcs -> filegroups -> alias labels -> real paths) as JSON {task id: [paths]}"""
import sys,re,json,os,glob
W=sys.argv[1]
build=open(W+"/BUILD.bazel",encoding="utf8").read()
alias=json.load(open(W+"/sources.json"))["directories"]
groups={}
for m in re.finditer(r'filegroup\(\s*name = "([^"]+)",\s*srcs = \[(.*?)\],\s*\)',build,re.S):
    groups[m.group(1)]=re.findall(r'"([^"]+)"',m.group(2))
tasks={}
for m in re.finditer(r'repo_(?:task|test|launch)\(\s*name = "(t_[0-9a-f]+)",(.*?)\n\)',build,re.S):
    body=m.group(2)
    ident=re.search(r'\n    identity = "([^"]+)"',body).group(1)
    srcs=re.search(r'srcs = \[(.*?)\]',body,re.S)
    labels=re.findall(r':([A-Za-z0-9_]+)',srcs.group(1)) if srcs else []
    files=[]
    def expand(n,seen):
        for x in groups.get(n,[]):
            if x.startswith(":"):
                if x[1:] not in seen: seen.add(x[1:]);expand(x[1:],seen)
            else: files.append(x)
    seen=set()
    for l in labels: expand(l,seen)
    out=[]
    for f in files:
        mm=re.match(r'a/([0-9a-f]+)/(.*)$',f)
        if mm and mm.group(1) in alias: out.append(alias[mm.group(1)]+"/"+mm.group(2))
        else: out.append(f)
    tasks[ident]=sorted(set(out))
json.dump(tasks,sys.stdout,ensure_ascii=False)
