"""route2-g: prints the newest census log of a file and its failing assertions. usage: lastlog.py <substring-of-file> [maxlines]"""
import sys,glob,os
base="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/route-g"
hits=[]
for summary in glob.glob(base+"/*/summary.txt")+glob.glob(base+"/*/o/summary.txt"):
    for line in open(summary,errors="replace"):
        parts=line.rstrip("\n").split("\t")
        if parts and sys.argv[1] in parts[0]:
            log=os.path.join(os.path.dirname(summary),parts[-2].split("=")[1]) if len(parts)>=5 else None
            if log and os.path.exists(log): hits.append((os.path.getmtime(log),log,line.strip()))
hits.sort()
if not hits: print("none");sys.exit()
t,log,line=hits[-1];print(line);print(log)
n=int(sys.argv[2]) if len(sys.argv)>2 else 60
out=[]
for l in open(log,errors="replace"):
    if l.startswith("error:") or l.startswith("(fail)") or l.lstrip().startswith(("Expected","Received","at ")) and "node_modules" not in l: out.append(l.rstrip()[:260])
print("\n".join(out[:n]))
