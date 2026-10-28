"""👥️ agents16: applies the oracle edits of batch `allrun` to the upgrade oracle of <root> (anchored, idempotent) - the port of the decision changes (agent session first job, cores-short upgrade, elastic request) and the agent-session worlds with their own generator.
Usage: python3 -I oracle_agents.py <root>"""
import os, sys
P = os.path.join(sys.argv[1], "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/🐍️oracle/🐍️.py")
s = open(P, encoding="utf8").read()
WORLDS = '''def agents_worlds(policy):
    """👥️ Worlds of several agent sessions on a declared 8-core machine: running 8-job grants of some sessions, quiet grants of other sessions that ask to grow, fresh requests of sessions with and without a running grant, and the elastic farm request of a running session, old enough to be a barrier head; its own generator, so the stored worlds above stay what they were."""
    rng=random.Random(20261027);worlds=[];sessions=["a00000000001.claude","a00000000002.codex","a00000000003.claude","a00000000004.droid"]
    for index in range(120):
        now=1_800_000_000_000+rng.randrange(0,1_000_000);total=16384;cap=total-floor_mb(policy,total)
        host={"version":1,"atMs":now,"platform":"darwin","cores":8,"totalMegabytes":total,"availableMegabytes":rng.randrange(3000,cap) if index%4 else cap,"pressure":"normal","swapUsedMegabytes":0,"swapOutMegabytesPerSecond":0,"load":rng.choice([None,2,9,24]) if index%4 else rng.choice([None,2]),"openFiles":8000,"openFilesMaximum":245760,"attachedImages":10}
        ledger=[];queue=[];owners=rng.sample(sessions,1 if index%8==0 else rng.choice([1,2,3]))
        for owner in owners:
            if index%8==0 or rng.random()<0.75:ledger.append({"id":ident(rng),"operation":"run","jobs":8,"cpu":8,"reservedMegabytes":rng.choice([2861,3649,5119]),"admittedAtMs":now-rng.choice([5000,90000,900000]),"observedMegabytes":rng.choice([None,500,1800,3400]),"pid":rng.randrange(100,90000),"bootAtMs":1_700_000_000_000,"session":owner,"rampMilliseconds":rng.choice([60000,168836,600000])})
        for owner in [name for name in sessions if name not in owners][:0 if index%4==0 else rng.choice([1,2])]:
            g={"id":ident(rng),"operation":"run","jobs":1,"cpu":1,"reservedMegabytes":rng.choice([1502,2048]),"admittedAtMs":now-rng.choice([1000,40000]),"observedMegabytes":rng.choice([None,200,600]),"pid":rng.randrange(100,90000),"bootAtMs":1_700_000_000_000,"session":owner,"quiet":True,"rampMilliseconds":60000}
            g["arrivedAtMs"]=g["admittedAtMs"];ledger.append(g)
            request={"id":g["id"],"operation":"run","priority":"interactive","minimumJobs":8,"preferredJobs":8,"fixedMegabytes":rng.choice([245,312,400]),"perJobMegabytes":rng.choice([327,416,450]),"cpuPerJob":1,"rampMilliseconds":168836}
            if rng.random()<0.3:request["rampSlot"]=True
            queue.append({"request":request,"arrivedAtMs":g["arrivedAtMs"],"pid":g["pid"],"upgrade":True,"session":owner,"weight":1})
        if index%8!=0 and (index%4==0 or rng.random()<0.5):
            owner=rng.choice(sessions);queue.append({"request":{"id":ident(rng),"operation":"run","priority":rng.choice(["batch","interactive"]),"minimumJobs":1,"preferredJobs":8,"fixedMegabytes":rng.choice([245,1536]),"perJobMegabytes":rng.choice([327,2048]),"cpuPerJob":1},"arrivedAtMs":now-rng.choice([0,2000,45000]),"pid":rng.randrange(100,90000),"session":owner,"weight":1})
        running=[g for g in ledger if g.get("quiet") is not True]
        if running and (index%8==0 or rng.random()<0.6):
            owner=rng.choice(running)["session"];queue.append({"request":{"id":ident(rng),"operation":"exec","priority":"batch","minimumJobs":1,"preferredJobs":9,"fixedMegabytes":1536,"perJobMegabytes":2048,"cpuPerJob":1,"elastic":True},"arrivedAtMs":now-rng.choice([5000,40000,200000]),"pid":rng.randrange(100,90000),"session":owner,"weight":1})
        rng.shuffle(queue)
        worlds.append({"name":f"agents-{index}","host":host,"ledger":ledger,"queue":queue,"nowMs":now,"expect":evaluate(policy,host,ledger,queue,now)})
    return worlds

def generate():
'''
DECISION_PORT = [
 ('    for waiter in order(aged,active):\n', '    ordered=order(aged,active)\n    for at,waiter in enumerate(ordered):\n'),
 ('ra=2 if a.get("grow") is True else 0 if a["request"]["priority"]=="interactive" else 1;rb=2 if b.get("grow") is True else 0 if b["request"]["priority"]=="interactive" else 1',
  'ra=2 if a.get("grow") is True or a["request"].get("elastic") is True else 0 if a["request"]["priority"]=="interactive" else 1;rb=2 if b.get("grow") is True or b["request"].get("elastic") is True else 0 if b["request"]["priority"]=="interactive" else 1'),
 ('"room_observed":room_observed,"room_budget":room_budget,"load_room":load_room}', '"room_observed":room_observed,"room_budget":room_budget,"load_room":load_room,"sessions":set()}'),
 ('pause=None if avoids else soft;grows=waiter.get("grow") is True and waiter.get("upgrade") is True\n', 'pause=None if avoids else soft;grows=waiter.get("grow") is True and waiter.get("upgrade") is True;elastic=r.get("elastic") is True\n'),
 ('            if not grows:state["waits"]=True\n', '            if not grows and not elastic:state["waits"]=True\n'),
 ('            state["own"]+=jobs*r["cpuPerJob"]-(replaced["cpu"] if replaced else 0);state["admitted"]+=1\n', '            state["own"]+=jobs*r["cpuPerJob"]-(replaced["cpu"] if replaced else 0);state["admitted"]+=1\n            if waiter.get("session") is not None:state["sessions"].add(waiter["session"])\n'),
 ('        if state["blocked"] and not (quiet and state["quiet_left"]>0):\n', '        if elastic and (state["blocked"] or state["waits"]):count("elasticQueued");refuse("queue-order");continue\n        if state["blocked"] and not (quiet and state["quiet_left"]>0):\n'),
 ('        memory_fits=fits(r["minimumJobs"]);jobs=0\n', '        memory_fits=fits(r["minimumJobs"]);jobs=0\n        first=not elastic and waiter.get("session") is not None and state["own"]<ceiling+host["cores"] and waiter["session"] not in state["sessions"] and all(g["id"]==r["id"] or g.get("session")!=waiter["session"] for g in active)\n'),
 ('        if jobs==0 and pause is None and memory_fits and (avoids or state["own"]-(replaced["cpu"] if replaced else 0)+r["minimumJobs"]*r["cpuPerJob"]<=ceiling):jobs=r["minimumJobs"]\n',
  '        if jobs==0 and pause is None and memory_fits and (avoids or first and replaced is None or state["own"]-(replaced["cpu"] if replaced else 0)+r["minimumJobs"]*r["cpuPerJob"]<=ceiling):\n            if not avoids and state["own"]-(replaced["cpu"] if replaced else 0)+r["minimumJobs"]*r["cpuPerJob"]>ceiling:count("firstJob")\n            jobs=r["minimumJobs"]\n'),
 ('            admit(jobs,"reduced" if jobs<r["preferredJobs"] else "fits");continue\n        reason=',
  '            if elastic:count("elasticAdmit")\n            admit(jobs,"reduced" if jobs<r["preferredJobs"] else "fits");continue\n        if pause is None and memory_fits and replaced is not None:\n            cores=min(r["preferredJobs"],(ceiling-state["own"]+replaced["cpu"]-sum(w["request"]["minimumJobs"]*w["request"]["cpuPerJob"] for w in ordered[at+1:] if w.get("upgrade") is not True and w.get("grow") is not True and w["request"].get("elastic") is not True))//r["cpuPerJob"])\n            if cores>=1 or first:count("upgradeCores" if cores>=1 else "upgradeFirstJob");admit(max(1,cores),"reduced",need(r["minimumJobs"]));continue\n        reason='),
 ('        elif reason!="memory" and len(active)', '        elif reason!="memory" and not elastic and len(active)'),
 ('            if reason in ("memory","load") and not quiet:\n', '            if elastic:count("elasticWait")\n            if reason in ("memory","load") and not quiet and not elastic:\n'),
]
EDITS = DECISION_PORT + [
 ('def generate():\n', WORLDS),
 ('    return {"policy":{"maximumQuiet":4},"decisions":decisions,', '    agents=agents_worlds(policy)\n    return {"policy":{"maximumQuiet":4},"decisions":decisions,"agents":agents,'),
 ('print("wrote",len(value["decisions"]),"decisions,",', 'print("wrote",len(value["decisions"]),"decisions,",len(value["agents"]),"agent worlds,",'),
 ('an upgrade request replaces the held grant of its id and keeps its arrival)', 'an upgrade request replaces the held grant of its id and keeps its arrival, an agent session with nothing running gets its first job past our booked cores, an upgrade short of cores runs with the jobs they leave after the minimum jobs of the fresh requests served after it and keeps its whole memory, an elastic request is decided last and never becomes a barrier)'),
]
for old, new in EDITS:
    if new in s:
        continue
    if s.count(old) != 1:
        print("ANCHOR", s.count(old), old[:70]); sys.exit(1)
    s = s.replace(old, new)
open(P, "w", encoding="utf8").write(s)
print("oracle edited")
