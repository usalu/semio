import json,glob,os,re,sys
run=os.path.join(os.path.dirname(os.path.abspath(__file__)),"..","gate-g-tools","run")
def key(p):
    m=re.search(r"g(\d+)(b|r)?-dash(\d?)",p);return (int(m.group(1)),m.group(2) or "",m.group(3) or "")
rows=[]
for j in sorted(glob.glob(run+"/g*-dash*.json"),key=key):
    d=json.load(open(j));b=os.path.basename(j)[:-5]
    log=open(j[:-5]+".log",errors="replace").read()
    infos=re.findall(r"^INFO: (\d+) process(?:es)?: ([^\n]*)$",log,re.M)
    outer=infos[-1][1] if infos else ""
    ex=sum(int(n) for n in re.findall(r"(\d+) (?:worker|local|remote|sandbox|linux-sandbox|darwin-sandbox)",outer))
    memo="served" if "served from the memo" in log else ("light" if "(light)" in log else "?")
    plan="reused" if "plan reused" in log else ("again" if "planning again" in log else "?")
    cur="current" if "[dashboard] prototype is current" in log else ""
    rows.append((b,d["wall_s"],d["exit"],ex,outer,memo,plan,cur))
for r in rows: print("%-12s %7.1f exit=%s exec=%d memo=%-6s plan=%-6s %-8s | %s"%(r[0],r[1],r[2],r[3],r[5],r[6],r[7],r[4]))
ok=[r for r in rows if r[2]==0]
print("successful",len(ok),"exec>=1",sum(1 for r in ok if r[3]>=1),"exec0",[r[0] for r in ok if r[3]==0])
