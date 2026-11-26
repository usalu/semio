import os,sys,json
f=sys.argv[1]
s=open(f,encoding="utf8").read()
lines=s.split("\n")
HOST='    "hostName": {"description": "The state of the operating system host name of the machine as its published address set says: `ok`, `duplicate` (another origin carries the same label, named in `others`), `named-as` (the machine carries the semantic name of another origin, named in `others`) or `unknown` (no published set or no usable label). Never an identity; the way out is `repo fleet hostname apply` on that machine.", "anyOf": [{"type": "null"}, {"type": "object", "additionalProperties": false, "required": ["state", "label", "others"], "properties": {"state": {"enum": ["ok", "duplicate", "named-as", "unknown"]}, "label": {"type": ["string", "null"]}, "others": {"type": "array", "items": {"type": "string"}}}}]},'
if '"hostName"' in s: sys.exit("already")
idx=[i for i,l in enumerate(lines) if l.startswith('    "tree": {"anyOf"')]
assert len(idx)==2,idx
for i in reversed(idx): lines.insert(i,HOST)
s="\n".join(lines)
a='{recent}{refusal}'
assert s.count(a)==2
s=s.replace(a,'{recent}{host}{refusal}')
anchor='    "workers.refusal": {\n'
assert s.count(anchor)==1
msgs='''    "workers.host.ok": {"en": ", host name ok ({label})", "de": ", Rechnername in Ordnung ({label})"},
    "workers.host.duplicate": {"en": ", host name {label} is also on {others}: run `repo fleet hostname apply` on this machine", "de": ", Rechnername {label} liegt auch auf {others}: auf dieser Maschine `repo fleet hostname apply` ausführen"},
    "workers.host.named-as": {"en": ", host name {label} belongs to {others}: run `repo fleet hostname apply` on this machine", "de": ", Rechnername {label} gehört zu {others}: auf dieser Maschine `repo fleet hostname apply` ausführen"},
    "workers.host.unknown": {"en": ", host name unknown", "de": ", Rechnername unbekannt"},
'''
s=s.replace(anchor,msgs+anchor)
anchor2='    "finding.worker-absent": {\n'
assert s.count(anchor2)==1
fm='''    "finding.worker-host-name-duplicate": {
     "en": "worker {origin}: its host name {label} is also carried by {others}; run `repo fleet hostname apply` on that machine",
     "de": "Worker {origin}: sein Rechnername {label} trägt auch {others}; auf dieser Maschine `repo fleet hostname apply` ausführen"
    },
    "finding.worker-host-name-named-as": {
     "en": "worker {origin}: its host name {label} is the name of {others}; run `repo fleet hostname apply` on that machine",
     "de": "Worker {origin}: sein Rechnername {label} ist der Name von {others}; auf dieser Maschine `repo fleet hostname apply` ausführen"
    },
'''
s=s.replace(anchor2,fm+anchor2)
json.loads(s)
tmp=f+".f48.tmp"
open(tmp,"w",encoding="utf8").write(s)
os.replace(tmp,f)
print("ok")
