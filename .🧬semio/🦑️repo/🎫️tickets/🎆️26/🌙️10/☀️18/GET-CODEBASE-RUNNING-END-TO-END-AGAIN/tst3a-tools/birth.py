import re,sys
imp='import {withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";\n'
for p in sys.argv[1:]:
    s=open(p,encoding="utf-8").read()
    n=len(re.findall(r"receiveScriptProcessInvocation\(\s*process\.env\s*,\s*",s))
    if not n: print("none",p[-60:]); continue
    s=re.sub(r"receiveScriptProcessInvocation\(\s*process\.env\s*,\s*","withBuildTestInvocationV1(",s)
    rest=len(re.findall(r"\breceiveScriptProcessInvocation\b",s))
    m=re.search(r"^import\s*\{([^}]*)\}\s*from\s*(\"[^\"]*\"|'[^']*')\s*;?\s*$",s,re.M)
    for m in re.finditer(r"^import\s*\{([^}]*)\}\s*from\s*(\"[^\"]*\"|'[^']*')\s*;?[ \t]*\n",s,re.M):
        names=[x.strip() for x in m.group(1).split(",") if x.strip()]
        if "receiveScriptProcessInvocation" in names and rest==1:
            names=[x for x in names if x!="receiveScriptProcessInvocation"]
            repl=("import {"+",".join(names)+"} from "+m.group(2)+";\n") if names else ""
            s=s[:m.start()]+repl+s[m.end():]
            break
    if "withBuildTestInvocationV1}" not in s and "withBuildTestInvocationV1," not in s.split("\n\n")[0]:
        i=s.find("\nimport ")
        s=(imp+s) if s.startswith("import") else s[:0]+imp+s
    open(p,"w",encoding="utf-8").write(s)
    print("ok",n,"rest",rest,p[-60:])
