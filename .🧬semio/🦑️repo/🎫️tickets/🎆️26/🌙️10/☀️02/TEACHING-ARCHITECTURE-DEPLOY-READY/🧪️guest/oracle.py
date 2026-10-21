import sys,json,sqlite3,hashlib,base64,importlib.util,os
v=json.load(sys.stdin)
db=sqlite3.connect(":memory:")
projection=[]
for case in v["projection"]:
 db.executescript("drop table if exists requested;drop table if exists rows;create table requested(path text);create table rows(path text,bytes integer,sha text);")
 db.executemany("insert into requested values(?)",[(p,) for p in case["paths"]])
 db.executemany("insert into rows values(?,?,?)",[(r["path"],r["bytes"],r["sha256"]) for r in case["rows"]])
 valid=all(p and not p.startswith("/") and not any(x in ("",".","..") for x in p.split("/")) and "\\" not in p and ":" not in p for p in case["paths"])
 valid=valid and db.execute("select count(*)=count(distinct path) from rows").fetchone()[0] and db.execute("select count(*)=count(distinct path) from requested").fetchone()[0]
 valid=valid and not db.execute("select 1 from rows where bytes<0 or length(sha)<>64 or sha glob '*[^0-9a-f]*'").fetchone()
 valid=valid and db.execute("select count(*) from requested join rows using(path)").fetchone()[0]==len(case["paths"])
 projection.append({"id":case["id"],"accepted":bool(valid),"expected":{p:{"bytes":n,"sha256":h} for p,n,h in db.execute("select rows.path,bytes,sha from rows join requested using(path) order by cast(rows.path as blob)")} if valid else None})
settlements=[bool(db.execute("select ?1 and ?2=0 and ?3 is null and ?4 and ?5 and ?6 and ?7",(c["joined"],c["status"],c["signal"],*c["checks"])).fetchone()[0]) for c in v["settlements"]]
packet=base64.b64decode(v["packet"],validate=True)
spec=importlib.util.spec_from_file_location("neutral_guest",v["guest"])
guest=importlib.util.module_from_spec(spec);spec.loader.exec_module(guest)
physical={"supported":os.name!="nt","cases":guest.neutral_physical_cases(v["artifact"]) if os.name!="nt" else []}
def terminal_accept(row):
 try:
  w=row["wire"];t=row["terminal"];r=t["receipt"];b=base64.b64decode(r["base64"],validate=True);c=json.loads(b);n=t["native"]
  return t["version"]==1 and t["kind"]=="captured-neutral-terminal" and all(t[k]==w[k] for k in ("binding","operation","cohort","containerId","packageSha256","manifestSha256")) and t["sourceManifestSha256"]==w["source"]["sha256"] and t["nativeSha256"]==w["native"]["sha256"] and n=={"launched":True,"joined":True,"status":0,"signal":None} and t["drained"] is True and t["accepted"] is True and all(t["checks"][k] is True for k in ("source","ticket","native","receipt")) and len(b)==r["bytes"] and hashlib.sha256(b).hexdigest()==r["sha256"] and c["kind"]=="architecture-combined-neutral-receipt" and c["manifestSha256"]==w["manifestSha256"] and c["accepted"] is True and c["phaseCount"]==len(v["phases"]) and c["sourceInputFiles"]==w["sourceCount"] and c["parentTerminalProven"] is False and [r["id"] for r in c["results"]]==v["phases"] and all(r["accepted"] is True and r["status"]==0 and r["signal"] is None and r["ownedCallSettled"] is True and r["collection"]["accepted"] is True for r in c["results"])
 except (KeyError,ValueError,TypeError): return False
terminals=[terminal_accept(row) for row in v["terminals"]]
print(json.dumps({"projection":projection,"settlements":settlements,"packetSha256":hashlib.sha256(packet).hexdigest(),"physical":physical,"terminals":terminals}))
