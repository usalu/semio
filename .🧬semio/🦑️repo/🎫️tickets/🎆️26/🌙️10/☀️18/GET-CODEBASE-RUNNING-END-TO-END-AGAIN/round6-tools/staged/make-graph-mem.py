#!/usr/bin/env python3
"""Merged copy of the graph-mem set: two anchors follow the live code (loop-hunt collector in the project worker loop; the graph implementation URL list is an arrow function now, not a call argument)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/graph-mem-tools", "graph-mem")
OPS = T + "/staged/ops.json"
OLD_W = '  const workers=Array.from({length:Math.min(projectConcurrencyV1(options),jobs.length)},async()=>{while(!stopped&&cursor<jobs.length){await inferProject(jobs[cursor++]);await phases?.advance("inference",++finished);}});'
NEW_W = '  const workers=Array.from({length:Math.min(projectConcurrencyV1(options),jobs.length)},async()=>{while(!stopped&&cursor<jobs.length){{const job=jobs[cursor++];if(collector)await collector.guard({project:job.name,root:job.json.root,phase:"project"},()=>inferProject(job));else await inferProject(job);}await phases?.advance("inference",++finished);}});'


def workers(anchor, replacement):
    assert anchor == OLD_W
    r = replacement.replace("await governor.run(()=>inferProject(job));", 'if(collector)await collector.guard({project:job.name,root:job.json.root,phase:"project"},()=>governor.run(()=>inferProject(job)));else await governor.run(()=>inferProject(job));')
    r = r.replace("const job=jobs[cursor++];if(collector)", "{const job=jobs[cursor++];if(collector)").replace("else await governor.run(()=>inferProject(job));await phases", "else await governor.run(()=>inferProject(job));}await phases")
    return NEW_W, r


patch_ops_json(OPS, "edits", "project-nodes/🟨️.mjs", "  const workers=Array.from(", workers)
patch_ops_json(OPS, "edits", "🕸️graph/🟦️.ts", 'new URL("./🧭️file-owner/🟨️.mjs"', lambda a, r: (a[:-1], r[:-1]))
print("graph-mem merged copy:", T)
