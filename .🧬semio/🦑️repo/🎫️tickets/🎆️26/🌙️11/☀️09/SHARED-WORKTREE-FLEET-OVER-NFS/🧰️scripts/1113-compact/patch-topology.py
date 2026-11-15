#!/usr/bin/env python3
"""Anchored edits of the topology member for the setting replica.compaction (off by default). Usage: patch-topology.py <module root>."""
import sys, os
root = sys.argv[1]
changes = {}
def load(path):
    if path not in changes:
        with open(os.path.join(root, path), encoding="utf8") as handle:
            changes[path] = handle.read()
    return changes[path]
def rep(path, old, new, count=1):
    text = load(path)
    assert text.count(old) == count, f"{path}: anchor occurs {text.count(old)} times: {old[:90]!r}"
    changes[path] = text.replace(old, new)
P = "🗺️topology/"
COMPACTION = '{"automatic": false, "snapshotEvents": 100000, "safetyHours": 24, "staleHours": 12, "catchUpEvents": 20000}'
rep(P + "🟦️.ts", "retentionDays:number;bootstrapMinutes:number}>}>;", "retentionDays:number;bootstrapMinutes:number;compaction:Readonly<{automatic:boolean;snapshotEvents:number;safetyHours:number;staleHours:number;catchUpEvents:number}>}>}>;")
rep(P + "🔣️.json", '"retentionDays": 14, "bootstrapMinutes": 60}', '"retentionDays": 14, "bootstrapMinutes": 60, "compaction": ' + COMPACTION + '}')
rep(P + "🧪️tests/🧫️fixtures/🔣️.json", '"retentionDays": 14, "bootstrapMinutes": 60}', '"retentionDays": 14, "bootstrapMinutes": 60, "compaction": ' + COMPACTION + '}')
rep(P + "🧪️tests/🧫️fixtures/🔣️.json", '  {"name": "missing bootstrap interval", "path": ["replica", "bootstrapMinutes"], "remove": true},',
    '  {"name": "missing bootstrap interval", "path": ["replica", "bootstrapMinutes"], "remove": true},\n  {"name": "missing compaction", "path": ["replica", "compaction"], "remove": true},\n  {"name": "compaction switch that is no boolean", "path": ["replica", "compaction", "automatic"], "value": "yes"},\n  {"name": "zero safety hours", "path": ["replica", "compaction", "safetyHours"], "value": 0},\n  {"name": "fractional catch-up events", "path": ["replica", "compaction", "catchUpEvents"], "value": 2000.5},')
rep(P + "🧬️schema/🔣️.json", '''  "Replica": {
   "type": "object",
   "additionalProperties": false,
   "required": ["pollMilliseconds", "scanSeconds", "debounceMilliseconds", "retentionDays", "bootstrapMinutes"],''', '''  "Compaction": {
   "description": "Off by default. `automatic` switches on the periodic snapshot, the removal of journal segments and unreferenced blobs on the group server and the jump of a far-behind replica to the newest snapshot; the other fields are its thresholds (see the compaction member).",
   "type": "object",
   "additionalProperties": false,
   "required": ["automatic", "snapshotEvents", "safetyHours", "staleHours", "catchUpEvents"],
   "properties": {
    "automatic": {"type": "boolean"},
    "snapshotEvents": {"type": "integer", "minimum": 1000, "maximum": 100000000},
    "safetyHours": {"type": "integer", "minimum": 1, "maximum": 8760},
    "staleHours": {"type": "integer", "minimum": 1, "maximum": 8760},
    "catchUpEvents": {"type": "integer", "minimum": 1000, "maximum": 100000000}
   }
  },
  "Replica": {
   "type": "object",
   "additionalProperties": false,
   "required": ["pollMilliseconds", "scanSeconds", "debounceMilliseconds", "retentionDays", "bootstrapMinutes", "compaction"],''')
rep(P + "🧬️schema/🔣️.json", '''    "bootstrapMinutes": {"type": "integer", "minimum": 1, "maximum": 10080}
''', '''    "bootstrapMinutes": {"type": "integer", "minimum": 1, "maximum": 10080},
    "compaction": {"$ref": "#/definitions/Compaction"}
''')
rep(P + "🧪️tests/🐍️.py", '''    if exact(replica,["pollMilliseconds","scanSeconds","debounceMilliseconds","retentionDays","bootstrapMinutes"],"replica",out):
        for key,low,high in (("pollMilliseconds",50,60000),("scanSeconds",1,3600),("debounceMilliseconds",10,10000),("retentionDays",1,3650),("bootstrapMinutes",1,10080)):
            if not integer(replica[key],low,high): out.append("replica."+key+" invalid")''', '''    if exact(replica,["pollMilliseconds","scanSeconds","debounceMilliseconds","retentionDays","bootstrapMinutes","compaction"],"replica",out):
        for key,low,high in (("pollMilliseconds",50,60000),("scanSeconds",1,3600),("debounceMilliseconds",10,10000),("retentionDays",1,3650),("bootstrapMinutes",1,10080)):
            if not integer(replica[key],low,high): out.append("replica."+key+" invalid")
        compaction=replica["compaction"]
        if exact(compaction,["automatic","snapshotEvents","safetyHours","staleHours","catchUpEvents"],"replica.compaction",out):
            if type(compaction["automatic"]) is not bool: out.append("replica.compaction.automatic invalid")
            for key,low,high in (("snapshotEvents",1000,100000000),("safetyHours",1,8760),("staleHours",1,8760),("catchUpEvents",1000,100000000)):
                if not integer(compaction[key],low,high): out.append("replica.compaction."+key+" invalid")''')
rep(P + "🧪️tests/🐍️.py", '''"retentionDays":14,"bootstrapMinutes":60}: failures.append("committed: replica")''', '''"retentionDays":14,"bootstrapMinutes":60,"compaction":{"automatic":False,"snapshotEvents":100000,"safetyHours":24,"staleHours":12,"catchUpEvents":20000}}: failures.append("committed: replica")''')
rep(P + "🧪️tests/🟦️.ts", "retentionDays:14,bootstrapMinutes:60});", "retentionDays:14,bootstrapMinutes:60,compaction:{automatic:false,snapshotEvents:100000,safetyHours:24,staleHours:12,catchUpEvents:20000}});")
for path, text in changes.items():
    with open(os.path.join(root, path), "w", encoding="utf8") as handle:
        handle.write(text)
print("patched", len(changes), "files")
