import os, sys, hashlib
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution"
files = {
 "scheduler": B + "/🗓️scheduler/🟦️.ts",
 "placement": B + "/🗓️scheduler/⚖️placement/🟦️.ts",
 "sidecar": B + "/🛰️sidecar/🗓️scheduler/🟦️.ts",
}
edits = {
 "scheduler": [
  ('import type {SchedulerJournalRowV1} from "./📓️journal/🟦️.ts";',
   'import type {SchedulerJournalRowV1} from "./📓️journal/🟦️.ts";\nimport type {CounterGroupsV1,CounterLedgerV1} from "../../📈️counter-ledger/🟦️.ts";'),
  ('restore?:readonly SchedulerJournalRowV1[];onChange?:()=>void;',
   'restore?:readonly SchedulerJournalRowV1[];ledger?:CounterLedgerV1;onChange?:()=>void;'),
  ('  this.lastTick=this.clock();\n',
   '  this.lastTick=this.clock();\n  options.ledger?.bind(()=>this.persistent());\n'),
  (' /** 📊️ A secret-free view of the scheduler for `GET /status`. */\n',
   ' /** 🧮️ The counts of this run that the ledger keeps over restarts (see `📈️counter-ledger`): the monotonic scheduler counters (`submitted` and `assigned` are the counts `queued` and `executing` of `status()` before they are replaced by the gauges) and, per machine, `offload.<origin>` with its offloaded in, offloaded out and refused counts. */\n private persistent():CounterGroupsV1{\n  const {queued:submitted,executing:assigned,...rest}=this.counters;\n  return {scheduler:{...rest,submitted,assigned},...this.load.counts()};\n }\n\n /** 📈️ The cumulative counts: the ledger of earlier runs plus this run, or this run alone without a ledger. */\n cumulative():CounterGroupsV1{return this.options.ledger?.total()??this.persistent();}\n\n /** 📊️ A secret-free view of the scheduler for `GET /status`. */\n'),
  ('headroom:placementHasHeadroomV1(this.workerView(worker),entry,now),offloadedIn:entry?.offloadedIn??0,refused:worker.refused};',
   'headroom:placementHasHeadroomV1(this.workerView(worker),entry,now),offloadedIn:totals[`offload.${worker.registration.origin}`]?.offloadedIn??entry?.offloadedIn??0,refused:worker.refused};'),
  ('  const now=this.clock(),platforms=new Map<string,{key:string;queued:number;workers:number}>();\n',
   '  const now=this.clock(),totals=this.cumulative(),platforms=new Map<string,{key:string;queued:number;workers:number}>();\n'),
  ('  const now=this.clock(),workers=new Map([...this.workers.values()].map(worker=>[worker.registration.origin,worker]));\n',
   '  const now=this.clock(),totals=this.cumulative(),workers=new Map([...this.workers.values()].map(worker=>[worker.registration.origin,worker]));\n'),
  ('worker:state,offloadedIn:entry.offloadedIn,offloadedOut:entry.offloadedOut,refused:entry.refused,counters:report.counters};',
   'worker:state,offloadedIn:totals[`offload.${report.origin}`]?.offloadedIn??entry.offloadedIn,offloadedOut:totals[`offload.${report.origin}`]?.offloadedOut??entry.offloadedOut,refused:totals[`offload.${report.origin}`]?.refused??entry.refused,counters:report.counters};'),
 ],
 "placement": [
  (' get(origin:string):RemoteExecutionLoadEntryV1|undefined{this.expire();return this.entries.get(origin);}\n',
   ' get(origin:string):RemoteExecutionLoadEntryV1|undefined{this.expire();return this.entries.get(origin);}\n\n /** 🧮️ The counts of every machine seen in this run, as the groups `offload.<origin>` of a counter ledger (a machine that is live counts from its entry, one that expired from the counts kept for it). */\n counts():Readonly<Record<string,Readonly<Record<string,number>>>>{\n  const rows:Record<string,Record<string,number>>={};\n  for(const [origin,kept] of this.kept)rows[`offload.${origin}`]={offloadedIn:kept.offloadedIn,offloadedOut:kept.offloadedOut,refused:kept.refused};\n  for(const [origin,entry] of this.entries)rows[`offload.${origin}`]={offloadedIn:entry.offloadedIn,offloadedOut:entry.offloadedOut,refused:entry.refused};\n  return rows;\n }\n'),
 ],
 "sidecar": [
  ('import {readSchedulerJournalV1,writeSchedulerJournalV1} from "../../🗓️scheduler/📓️journal/🟦️.ts";',
   'import {readSchedulerJournalV1,writeSchedulerJournalV1} from "../../🗓️scheduler/📓️journal/🟦️.ts";\nimport {openCounterLedgerV1} from "../../../📈️counter-ledger/🟦️.ts";'),
  (' const restore=readSchedulerJournalV1(journalPath,Date.now(),server.resumeGraceSeconds*10_000);\n',
   ' const restore=readSchedulerJournalV1(journalPath,Date.now(),server.resumeGraceSeconds*10_000),ledger=openCounterLedgerV1({path:join(dirname(journalPath),"build-scheduler.counters.json"),name:"remote-execution"});\n'),
  ('resumeGraceSeconds:server.resumeGraceSeconds,restore,onChange:changed,language:"none"},',
   'resumeGraceSeconds:server.resumeGraceSeconds,restore,ledger,onChange:changed,language:"none"},'),
  ('  flush();closed=true;\n  await running.stop().catch(()=>{});\n',
   '  flush();closed=true;\n  await running.stop().catch(()=>{});ledger.close();\n'),
 ],
}
mode = "--check" in sys.argv
new = {}
for key, path in files.items():
    text = open(path, encoding="utf-8").read()
    for old, rep in edits[key]:
        n = text.count(old)
        if n != 1:
            print("ANCHOR", key, n, old[:100]); sys.exit(1)
        text = text.replace(old, rep)
    new[key] = text
if mode:
    print("anchors ok"); sys.exit(0)
for key, path in files.items():
    before = hashlib.sha256(open(path, "rb").read()).hexdigest()[:12]
    temporary = path + ".m5.tmp"
    open(temporary, "w", encoding="utf-8").write(new[key])
    os.replace(temporary, path)
    print(key, before, "->", hashlib.sha256(new[key].encode()).hexdigest()[:12])
