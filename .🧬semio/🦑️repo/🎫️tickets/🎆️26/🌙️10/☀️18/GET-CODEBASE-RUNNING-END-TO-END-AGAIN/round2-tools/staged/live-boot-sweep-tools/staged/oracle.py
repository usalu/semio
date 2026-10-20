#!/usr/bin/env python3
"""Independent oracle of the boot-recovery corpus: re-derives every plan, identity binding, backing binding, boot epoch and ledger state from the facts alone. usage: oracle.py CORPUS.json [SCHEMA.json]"""
import json,re,sys,datetime

doc=json.load(open(sys.argv[1],encoding="utf8"))
schema=json.load(open(sys.argv[2],encoding="utf8")) if len(sys.argv)>2 else None
TOL=5000
PHASE={"lease-ticket":10,"lease-claims":20,"publication-residue":30,"directory-swap":40,"tool-server":50,"leaked-image":60}

def publication(f):
    p=".publication"
    if not f["writerFree"]:return "keep","live-writer",[]
    if f["directory"]=="absent":return "keep","settled",[]
    if f["directory"]=="unsafe" or f["unknownEntries"] or "unsafe" in (f["intent"],f["receipt"]):return "refuse","unknown-shape",[]
    close=[("remove-file",p+"/intent.json"),("remove-directory",p)]
    cand=[("remove-file",p+"/receipt.json")] if f["receipt"]!="absent" else []
    if f["intent"]=="absent":
        return ("act","empty-directory",[("remove-directory",p)]) if f["receipt"]=="absent" else ("refuse","candidate-without-intent",[])
    if f["intent"]=="torn":
        return ("act","torn-intent",close) if f["receipt"]=="absent" else ("refuse","torn-intent-with-candidate",[])
    image=f["image"]
    if image["state"]!="valid" or image["sha256"] is None or f["intentReceiptSha256"] is None:return "refuse","receipt-unavailable",[]
    if image["sha256"]==f["intentReceiptSha256"]:return "act","rolled-back",cand+close
    return ("act","completed-before-crash",close) if f["receipt"]=="absent" else ("refuse","ambiguous-candidate",[])

def verdict(f,boot):
    fam=f["family"]
    if fam=="publication-residue":return publication(f)
    if fam=="lease-ticket":
        if not f["parsed"] or f["arrivedAtMs"] is None or f["pid"] is None:return "refuse","unknown-name",[]
        if f["arrivedAtMs"]+TOL<=boot:return "act","previous-boot",[("retire-ticket",f["queue"]+"/"+f["name"])]
        return "keep","current-boot",[]
    if fam=="lease-claims":
        return ("act","previous-boot",[("retire-settled-claims","claims")]) if f["claimDatabases"]>0 and f["previousBootClaimDatabases"]>0 else ("keep","settled",[])
    if fam=="tool-server":
        stale=[x["path"] for x in f["files"] if x["modifiedAtMs"]+TOL<=boot]
        if stale:return "act","previous-boot",[("remove-file",x) for x in stale]
        return "keep",("current-boot" if f["files"] else "settled"),[]
    if fam=="directory-swap":
        if not f["writerFree"]:return "keep","live-writer",[]
        return ("act","pending-swap",[("recover-directory-swap",".")]) if f["intent"]=="present" else ("keep","settled",[])
    if fam=="persistent-record":
        a,b=f["final"],f["temporary"]
        if not a["links"] and not b["links"]:return "keep","settled",[]
        if not a["links"]:return ("keep","owner-resumes-prefix",[]) if b["links"]==1 else ("refuse","unknown-links",[])
        if not b["links"]:return ("keep","published",[]) if a["links"]==1 else ("refuse","unknown-links",[])
        return ("keep","owner-resumes-link",[]) if a["links"]==2 and b["links"]==2 and f["pair"]=="same-inode" else ("refuse","unpaired-transaction",[])
    if fam=="creator-claim":
        if f["sidecar"]=="absent":return "keep","settled",[]
        if f["binding"]=="mismatch":return "refuse","identity-mismatch",[]
        return "keep",("settled" if f["binding"]=="not-applicable" else "owner-retires-rebound" if f["binding"]=="rebound" else "owner-retires"),[]
    if fam=="leaked-image":
        detach=[("detach-image",f["key"])]
        if f["backing"]=="absent":return "act","backing-removed",detach
        if f["location"]!="scratch":return "keep","owned",[]
        if not f["idle"]:return "keep","live",[]
        if f["lease"]=="unavailable":return "keep","unleased",[]
        return ("act","abandoned",detach) if f["lease"]=="free" else ("keep","live",[])
    if fam=="image-receipt":
        if f["state"]=="unreadable":return "refuse","unreadable-receipt",[]
        if f["state"]=="absent":return "keep","unpublished",[]
        if f["binding"]=="mismatch":return "refuse","identity-mismatch",[]
        return "keep",("settled" if f["binding"]=="not-applicable" else "bound-at-open" if f["binding"]=="rebound" else "current"),[]
    raise SystemExit("unknown family "+fam)

def plan(o):
    rows=[];kept=[];n=0;ticket_scopes=[]
    for f in o["facts"]:
        v,reason,effects=verdict(f,o["bootEpochMs"])
        if v!="act":kept.append({"family":f["family"],"scope":f["scope"],"verdict":v,"reason":reason});continue
        for action,target in effects:
            rows.append((PHASE[f["family"]],f["scope"],n,f["family"],action,target,reason));n+=1
            if action=="retire-ticket":ticket_scopes.append(f["scope"])
    for s in dict.fromkeys(ticket_scopes):
        if not any(r[4]=="retire-settled-claims" and r[1]==s for r in rows):
            rows.append((PHASE["lease-claims"],s,n,"lease-claims","retire-settled-claims","claims","retired-tickets"));n+=1
    rows.sort(key=lambda r:(r[0],r[1],r[2]))
    return {"version":1,"bootSession":o["bootSession"],"steps":[{"index":i,"phase":r[0],"family":r[3],"scope":r[1],"action":r[4],"target":r[5],"reason":r[6]} for i,r in enumerate(rows)],"retained":kept}

def binding_identities(rec,cur):
    if len(rec)!=len(cur) or not rec:return "mismatch"
    if any(a["ino"]!=b["ino"] or a["mode"]!=b["mode"] for a,b in zip(rec,cur)):return "mismatch"
    n=len(rec)
    for i in range(n):
        for j in range(i+1,n):
            if (rec[i]["dev"]==rec[j]["dev"])!=(cur[i]["dev"]==cur[j]["dev"]):return "mismatch"
    return "exact" if all(a["dev"]==b["dev"] for a,b in zip(rec,cur)) else "rebound"

def project(text):
    try:v=json.loads(text)
    except Exception:return None
    ok=lambda x:isinstance(x,int) and not isinstance(x,bool) and x>=0
    if isinstance(v,list) and len(v)==4 and all(ok(x) for x in v):return v[0],("receiver",v[1],v[2])
    if isinstance(v,list) and len(v)==5 and isinstance(v[0],str) and ok(v[1]) and ok(v[2]) and ok(v[3]):return v[1],(v[0],v[2],v[3])
    return None

def binding_backing(rec,cur):
    if rec==cur:return "exact"
    a,b=project(rec),project(cur)
    return "mismatch" if a is None or b is None or a[1]!=b[1] else "rebound"

def epoch(platform,text):
    if platform=="darwin":
        m=re.search(r"sec = ([0-9]{9,11}), usec = ([0-9]{1,6})",text)
        return None if not m else int(m.group(1))*1000+int(m.group(2))//1000
    if platform=="linux":
        for line in text.split("\n"):
            m=re.fullmatch(r"btime ([0-9]{9,11})",line)
            if m:return int(m.group(1))*1000
        return None
    m=re.fullmatch(r"([0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2})(?:\.([0-9]{1,7}))?Z\s*",text)
    if not m:return None
    base=datetime.datetime.strptime(m.group(1),"%Y-%m-%dT%H:%M:%S").replace(tzinfo=datetime.timezone.utc)
    return int(base.timestamp())*1000+int((m.group(2) or "").ljust(7,"0"))//10000

BOOT=re.compile(r"^[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$")
def valid_event(e):
    if not isinstance(e,dict) or e.get("v")!=1:return False
    keys={"begin":{"v","type","boot","at","steps","plan"},"step":{"v","type","boot","index","action","target","outcome","detail"},"commit":{"v","type","boot","at","done","skipped","retained"},"abort":{"v","type","boot","at","index","reason"}}
    t=e.get("type")
    return t in keys and set(e)==keys[t] and isinstance(e["boot"],str) and bool(BOOT.match(e["boot"]))
def ledger(text):
    lines=text.split("\n");tail=lines.pop();torn=tail!=""
    last=None;begin=None
    for line in lines:
        try:e=json.loads(line)
        except Exception:return {"state":"empty","boot":None,"corrupt":True,"tornTail":torn}
        if not valid_event(e):return {"state":"empty","boot":None,"corrupt":True,"tornTail":torn}
        last=e
        if e["type"]=="begin":begin=e["boot"]
    if last is None:return {"state":"empty","boot":None,"corrupt":False,"tornTail":torn}
    if last["type"]=="commit":return {"state":"committed","boot":last["boot"],"corrupt":False,"tornTail":torn}
    return {"state":"incomplete","boot":begin or last["boot"],"corrupt":False,"tornTail":torn}

failures=[]
for c in doc["plans"]:
    got=plan(c["observation"])
    if got!=c["plan"]:failures.append(("plan",c["name"],got))
for c in doc["identities"]:
    got=binding_identities(c["recorded"],c["current"])
    if got!=c["expected"]:failures.append(("identity",c["name"],got))
for c in doc["backings"]:
    got=binding_backing(c["recorded"],c["current"])
    if got!=c["expected"]:failures.append(("backing",c["name"],got))
for c in doc["epochs"]:
    got=epoch(c["platform"],c["text"])
    if got!=c["expectedMs"]:failures.append(("epoch",c["name"],got))
for c in doc["ledgers"]:
    got=ledger(c["text"])
    if got!=c["expected"]:failures.append(("ledger",c["name"],got))
if failures:
    for kind,name,got in failures:print("FAIL",kind,name,json.dumps(got)[:300])
    sys.exit(1)
print(json.dumps({"ok":True,"plans":len(doc["plans"]),"identities":len(doc["identities"]),"backings":len(doc["backings"]),"epochs":len(doc["epochs"]),"ledgers":len(doc["ledgers"])}))
