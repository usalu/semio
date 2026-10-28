"""📼 Extracts the admission trace of tonight's fleet from the `[capacity]` lines of the ticket logs: one record per queued request (arrival, operation, need, wait, reasons) and one per observed grant (reserved, peak observed, held seconds)."""
import json,os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from stream_logs import ROOT,stream_by_file
WAIT=re.compile(r"^\[capacity\] waiting (\d+)s for ([a-z-]+): (.*?) \(need (\d+) MB; (-?\d+) MB spare above the (\d+) MB floor, load ([\d.]+); (\d+) running, (\d+) ahead\)")
ADMIT=re.compile(r"^\[capacity\] admitted ([a-z-]+)(?: with (\d+) of (\d+) job\(s\)|: (\d+) job\(s\), (\d+) MB reserved)")
RUN=re.compile(r"^\[capacity\]   running (\S+) ([a-z-]+): (\d+) job\(s\), (\d+) MB reserved, (\d+) MB observed, pid (\d+), since (\d+)s")
HOST=re.compile(r"^\[capacity\] (\d+) MB available of (\d+) MB \(floor (\d+) MB, (-?\d+) MB spare\), pressure (\w+), swap (\d+) MB, load ([\d.]+), open files (\d+), attached images (\d+); (\d+) running, (\d+) waiting")
def main():
    requests=[];grants={};hosts=[]
    for path,lines in stream_by_file(r"^\[capacity\]"):
        try: birth=os.stat(path).st_birthtime;mtime=os.stat(path).st_mtime
        except OSError: continue
        current=None;clock=birth
        for text in lines:
            m=WAIT.match(text)
            if m:
                s,op,reason,need,spare,floor,load,running,ahead=m.groups();s=int(s)
                if current is None or current["op"]!=op or s<current["waited"] or int(need)!=current["need"] and s==0:
                    if current: requests.append(current)
                    current={"file":path,"arrival":clock+(0 if current is None else current["waited"]),"op":op,"need":int(need),"waited":s,"ahead0":int(ahead),"aheadN":int(ahead),"reasons":collections.Counter(),"spare":[],"admitted":False}
                current["waited"]=s;current["aheadN"]=int(ahead);current["reasons"][reason]+=1;current["spare"].append(int(spare))
                continue
            m=ADMIT.match(text)
            if m and current and m.group(1)==current["op"]:
                current["admitted"]=True;requests.append(current);clock=current["arrival"]+current["waited"];current=None
                continue
            m=RUN.match(text)
            if m:
                gid,op,jobs,reserved,observed,pid,since=m.groups()
                g=grants.setdefault(gid,{"op":op,"jobs":int(jobs),"reserved":int(reserved),"peak":0,"since":0,"file":path});g["peak"]=max(g["peak"],int(observed));g["since"]=max(g["since"],int(since));continue
            m=HOST.match(text)
            if m: hosts.append(list(map(int,[m.group(1),m.group(2),m.group(3),m.group(6),m.group(8),m.group(9),m.group(10),m.group(11)]))+[m.group(5)])
        if current: requests.append(current)
    for r in requests: r["reasons"]=dict(r["reasons"]);r["spare"]=r["spare"][::max(1,len(r["spare"])//12)]
    out=os.path.join(ROOT,"🗑️generated","ram-g","trace.json")
    json.dump({"requests":requests,"grants":list(grants.values()),"hosts":hosts},open(out,"w"),ensure_ascii=False)
    print(len(requests),"requests",len(grants),"grants",len(hosts),"host samples")
main()
