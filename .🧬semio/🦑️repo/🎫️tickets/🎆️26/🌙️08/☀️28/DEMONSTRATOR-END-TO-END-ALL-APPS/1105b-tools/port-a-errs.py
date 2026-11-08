import re,sys
t=open(sys.argv[1]).read()
pk=sys.argv[2]
i=t.find("Checking "+pk+" ")
if i<0:i=t.find("Compiling "+pk+" ")
t=t[i:]
j=t.find("could not compile `"+pk+"`")
if j>=0:t=t[:j]
for b in re.split(r"\n(?=error|warning)",t):
    m=re.match(r"(error|warning)(\[E\d+\])?: (.*)\n\s+--> (.*?):(\d+):(\d+)",b)
    if not m or m.group(1)!="error":continue
    f=m.group(4).split("rust/../../")[-1].split("🔨️modules/")[-1]
    lab=re.findall(r"\n\s*\|[ ]+[\^\-]+ (.*)",b)
    print(m.group(2),f.replace("🏪️store/",""),m.group(5),m.group(3)[:70],"|",(lab[-1] if lab else "")[:130])
