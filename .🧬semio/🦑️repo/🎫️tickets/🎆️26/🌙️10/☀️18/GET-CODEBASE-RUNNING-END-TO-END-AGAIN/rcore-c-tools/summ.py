import re,sys
log=sys.argv[1]
cur=None;rows=[];fails=[];errs=[]
for l in open(log,errors="replace"):
    m=re.search(r"Running (?:unittests )?(\S+)(?: \((\S+)\))?",l)
    if m: cur=(m.group(2) or m.group(1)).split("/")[-1]
    m=re.match(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored",l)
    if m: rows.append((cur,m.group(1),int(m.group(2)),int(m.group(3)),int(m.group(4))))
    if l.startswith("---- ") and l.rstrip().endswith("stdout ----"): fails.append(l.split()[1])
    m=re.search(r"(error(?:\[E\d+\])?: .{0,200})",l)
    if m and "could not compile" in l: errs.append(m.group(1))
for r in rows: print("RESULT",*r)
for f in fails: print("FAIL",f)
for e in errs: print("ERR",e)
