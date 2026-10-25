import re,sys,json
rows=[]
pattern=re.compile(r"^HELD\s+(shared|exclusive)\s+(\S.*?)\s+process (\d+)\s+(\d+) s\s+\[([a-z-]+) (\d+)\]")
for line in open(sys.argv[1],encoding="utf8"):
    match=pattern.match(line)
    if match:rows.append({"mode":match[1],"resource":match[2],"pid":int(match[3]),"age":int(match[4]),"class":match[5],"rank":int(match[6])})
by={}
for row in rows:by.setdefault(row["pid"],[]).append(row)
report=[]
for pid,held in sorted(by.items()):
    writer=[r for r in held if r["resource"].startswith("repo:compiler-writer:")]
    registry=[r for r in held if r["resource"].startswith("cargo:registry:")]
    fetch=[r for r in held if r["resource"].startswith("cargo:registry-fetch:")]
    if not writer:continue
    verdict={"pid":pid,"writerMode":writer[0]["mode"],"writerAge":writer[0]["age"],"registryAge":max([r["age"] for r in registry],default=None),"fetchAge":max([r["age"] for r in fetch],default=None)}
    verdict["registryBeforeWriter"]=None if not registry else verdict["registryAge"]>=verdict["writerAge"]
    verdict["fetchAfterWriter"]=None if not fetch else verdict["fetchAge"]<=verdict["writerAge"]
    report.append(verdict)
print(json.dumps(report,indent=1))
