import sys,os
F="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/"
def sub(path,pairs):
    t=open(F+path,encoding="utf8").read()
    for a,b in pairs:
        assert t.count(a)==1,(path,a[:50],t.count(a))
        t=t.replace(a,b)
    open(F+path,"w",encoding="utf8").write(t)
sub("🏊️pool/🟨️.mjs",[
('import { tmpdir } from "node:os";','import { getPriority as osPriorityV1, setPriority as setOsPriorityV1, tmpdir } from "node:os";'),
('const P = GRAPH_FARM_POLICY_V1, MEGABYTE = 1048576;','''const P = GRAPH_FARM_POLICY_V1, MEGABYTE = 1048576;

/** ⏫️ Claims the priority class of the read farm (`priority.readFarm`, normal) for one child: a child that starts below it is set to it, which an unprivileged process may only do where the platform lets it raise priority (Windows); where the raise is refused (macOS and Linux, a niceness only the superuser lowers) the child keeps what it inherited from its starter and the result says so, so the farm can tell the person who started the read instead of staying silently slow.
 * @see ./🧬️schema/🔣️.json the declared classes */
export function claimFarmChildPriorityV1(pid, { get = osPriorityV1, set = setOsPriorityV1 } = {}, wanted = P.priority.readFarm) {
  const none = { before: undefined, after: undefined, raised: false, inherited: false };
  if (pid === undefined) return none;
  let before; try { before = get(pid); } catch { return none; }
  if (before <= wanted) return { before, after: before, raised: false, inherited: false };
  try { set(pid, wanted); } catch {}
  let after = before; try { after = get(pid); } catch {}
  return { before, after, raised: after < before, inherited: after > wanted };
}'''),
('report = () => {}, retired = () => {}, spawnProcess = spawn,','report = () => {}, retired = () => {}, priority = {}, spawnProcess = spawn,'),
('  const retiring = new Set()','  let priorityReported = false;\n  const retiring = new Set()'),
('    slot.child = child;\n    holdChild(child);','    slot.child = child;\n    const claimed = claimFarmChildPriorityV1(child.pid, priority);\n    if (claimed.inherited && !priorityReported) { priorityReported = true; report("priority", { nice: claimed.after, wanted: P.priority.readFarm }); }\n    holdChild(child);'),
])
sub("🏊️pool/🧬️schema/🔣️.json",[
('        "terminateGraceMilliseconds": 2000,\n','        "terminateGraceMilliseconds": 2000,\n        "priority": { "readFarm": 0, "fleetPool": 10 },\n'),
('          "lost": "graph: a worker process was lost ({reason}); its project is inferred again",\n','          "lost": "graph: a worker process was lost ({reason}); its project is inferred again",\n          "priority": "graph: the worker processes run at niceness {nice} instead of {wanted} because the process that started this read already had it (a background job of the shell does, and a process cannot lower its niceness); start the read in the foreground or through the graph server to run at full speed",\n'),
('          "lost": "Graph: ein Worker-Prozess ging verloren ({reason}); sein Projekt wird erneut abgeleitet",\n','          "lost": "Graph: ein Worker-Prozess ging verloren ({reason}); sein Projekt wird erneut abgeleitet",\n          "priority": "Graph: die Worker-Prozesse laufen mit Nice-Wert {nice} statt {wanted}, weil der Prozess, der diesen Lesevorgang gestartet hat, ihn schon hatte (ein Hintergrundauftrag der Shell hat ihn, und ein Prozess kann seinen Nice-Wert nicht senken); den Lesevorgang im Vordergrund oder über den Graph-Server starten, um mit voller Geschwindigkeit zu laufen",\n'),
])
sub("🏊️pool/🟨️.d.mts",[
('terminateGraceMilliseconds:number;','terminateGraceMilliseconds:number;readonly priority:{readonly readFarm:number;readonly fleetPool:number};'),
])
sub("🟨️.mjs",[
('        else if (event === "lost") { counts.lost++; sayHub("lost", fields); }','        else if (event === "lost") { counts.lost++; sayHub("lost", fields); }\n        else if (event === "priority") sayHub("priority", fields);'),
])
print("patched")
