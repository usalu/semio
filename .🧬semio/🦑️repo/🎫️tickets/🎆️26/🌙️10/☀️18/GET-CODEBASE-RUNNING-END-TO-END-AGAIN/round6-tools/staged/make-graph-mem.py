#!/usr/bin/env python3
"""Merged copy of the graph-mem set: two anchors follow the live code (loop-hunt collector in the project worker loop; the graph implementation URL list is an arrow function now, not a call argument)."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/graph-mem-tools", "graph-mem")
OPS = T + "/staged/ops.json"
OLD_W = '  const workers=Array.from({length:Math.min(projectConcurrencyV1(options),jobs.length)},async()=>{while(!stopped&&cursor<jobs.length){await inferProject(jobs[cursor++]);await phases?.advance("inference",++finished);}});'
NEW_W = '  const workers=Array.from({length:Math.min(projectConcurrencyV1(options),jobs.length)},async()=>{while(!stopped&&cursor<jobs.length){{const job=jobs[cursor++];if(collector)await collector.guard({project:job.name,root:job.json.root,phase:"project"},()=>inferProject(job));else await inferProject(job);}await phases?.advance("inference",++finished);}});'


def workers(anchor, replacement):
    if anchor != OLD_W:
        return anchor, replacement
    r = replacement.replace("await governor.run(()=>inferProject(job));", 'if(collector)await collector.guard({project:job.name,root:job.json.root,phase:"project"},()=>governor.run(()=>inferProject(job)));else await governor.run(()=>inferProject(job));')
    r = r.replace("const job=jobs[cursor++];if(collector)", "{const job=jobs[cursor++];if(collector)").replace("else await governor.run(()=>inferProject(job));await phases", "else await governor.run(()=>inferProject(job));}await phases")
    return NEW_W, r


_ops = json.load(open(OPS, encoding="utf8"))
_n = len(_ops["edits"])
_ops["edits"] = [op for op in _ops["edits"] if not (op["file"].endswith("project-nodes/\U0001F7E8\ufe0f.mjs") and op["anchor"].startswith("  const workers=Array.from("))]
assert len(_ops["edits"]) == _n - 1, "worker loop edit"
json.dump(_ops, open(OPS, "w", encoding="utf8"), ensure_ascii=False, indent=1)


def urls(anchor, replacement):
    import re
    memory = re.search(r'new URL\("\.\./[^"]*memory/[^"]*",physicalSourceOriginV1\(import\.meta\.url,\(import\.meta as PhysicalSourceModuleMetaV1\)\.semioPhysicalSourceURL\)\)', replacement).group(0)
    return ",graphSnapshotPolicyURLV1,new URL(", "," + memory + ",graphSnapshotPolicyURLV1,new URL("


patch_ops_json(OPS, "edits", "🕸️graph/🟦️.ts", 'new URL("./', urls)
print("graph-mem merged copy:", T)
