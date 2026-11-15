#!/usr/bin/env python3
"""P6: read-only. Which executable did macOS record as the responsible process / the subject for the file accesses of bun and repo, and what was the answer?
usage: tcc-attribution.py [--last 3h | --start 'YYYY-MM-DD HH:MM:SS' [--end 'YYYY-MM-DD HH:MM:SS']] [--service DocumentsFolder] [--all]
Reads the unified log of the subsystem com.apple.TCC (needs no privilege for the own user's tccd lines); changes nothing. Times are local time of the machine.
Columns: time, accessing process (identifier pid), responsible process (identifier, path of the executable macOS keeps for it), subject = the identity the consent is stored for, answer (2 allowed, 0 denied; the first line of a pair comes from the system tccd and is normally 0/5, the user tccd decides)."""
import re,subprocess,sys,collections
args=sys.argv[1:]
def opt(name,default=None):
    return args[args.index(name)+1] if name in args else default
service=opt("--service","DocumentsFolder");window=["--start",opt("--start"),"--end",opt("--end")] if opt("--start") and opt("--end") else (["--start",opt("--start")] if opt("--start") else ["--last",opt("--last","3h")])
predicate='subsystem == "com.apple.TCC" AND (eventMessage CONTAINS "AUTHREQ_ATTRIBUTION" OR eventMessage CONTAINS "AUTHREQ_SUBJECT" OR eventMessage CONTAINS "AUTHREQ_RESULT" OR eventMessage CONTAINS "AUTHREQ_CTX")'
text=subprocess.run(["/usr/bin/log","show",*window,"--predicate",predicate,"--style","compact"],capture_output=True,text=True).stdout
rows=collections.OrderedDict()
for line in text.splitlines():
    m=re.search(r"msgID=([\d.]+)",line)
    if not m:continue
    d=rows.setdefault(m.group(1),{"time":line[11:23],"results":[]})
    if "AUTHREQ_CTX" in line:
        s=re.search(r"service=(\w+)",line);d["service"]=s.group(1) if s else ""
    if "AUTHREQ_ATTRIBUTION" in line:
        r=re.search(r"responsible=\{TCCDProcess: identifier=([^,]+), pid=(\d+).*?responsible_path=([^,]+), binary_path=([^}]+)\}",line)
        a=re.search(r"accessing=\{TCCDProcess: identifier=([^,]+), pid=(\d+).*?binary_path=([^}]+)\}",line)
        d["responsible"]=("%s %s"%(r.group(1),r.group(3))) if r else "(the accessing process itself)"
        d["accessing"]=("%s %s %s"%(a.group(1),a.group(2),a.group(3))) if a else "?"
    if "AUTHREQ_SUBJECT" in line:
        s=re.search(r"subject=([^,]*),",line);d["subject"]=s.group(1) if s else ""
    if "AUTHREQ_RESULT" in line:
        r=re.search(r"authValue=(\d+), authReason=(\d+)",line)
        if r:d["results"].append("/".join(r.groups()))
shown=0
for msg,d in rows.items():
    if "accessing" not in d:continue
    if "--all" not in args and not re.search(r"bun|repo|launcher|semio",d["accessing"]+d.get("subject","")+d["responsible"]):continue
    if service and d.get("service") and service not in d["service"]:continue
    print(d["time"],"| accessing:",d["accessing"],"| responsible:",d["responsible"],"| subject:",d.get("subject",""),"| answer:",",".join(d["results"]) or "-");shown+=1
print("%d rows (of %d consulted requests in the window)"%(shown,len(rows)))
