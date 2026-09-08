cd /Users/ueli/Documents/semio
python3 - <<'PY'
import json,re,subprocess
SCOPES={
 "hub.inference":("🌎️hub/💡️inference/🧬️schema",{"🦀️rust":"🦀️.rs","🟦️typescript":"🟦️.ts","🔣️jsonschema":"🔣️.json"}),
 "hub.auth":("🌎️hub/🔐️auth/🧬️schema",{"🟦️typescript":"🟦️.ts","🔣️jsonschema":"🔣️.json"}),
 "hub.artifact-authority.creation":("🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema",{"🦀️rust":"🦀️.rs","🔣️jsonschema":"🔣️.json"}),
 "hub.artifact-authority.trusted-catalog":("🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema",{"🦀️rust":"🦀️.rs","🔣️jsonschema":"🔣️.json"}),
}
PAT={"🦀️rust":r"^\s*pub\s+(?:struct|enum|type)\s+%s\b","🟦️typescript":r"^\s*export\s+(?:interface|type|const|class)\s+%s\b"}
bad=0
for scope,(path,formats) in SCOPES.items():
    doc=json.load(open(path+"/🔣️.json",encoding="utf-8"))
    src={f:open(path+"/"+n,encoding="utf-8").read() for f,n in formats.items() if f!="🔣️jsonschema"}
    for name,node in doc["$defs"].items():
        if not name[:1].isupper(): continue
        declared=node.get("x-semio-formats")
        expected=set(declared) if declared else set(formats)
        for f in formats:
            if f=="🔣️jsonschema": present=True
            else: present=re.search(PAT[f]%re.escape(name), src[f], re.M) is not None
            want=f in expected
            if present!=want:
                bad+=1; print("MISMATCH",scope,name,f,"present",present,"expected",want)
        if declared is not None and "🟦️typescript" in expected:
            if re.search(r"export (?:function|const) parse%s\b"%re.escape(name), src["🟦️typescript"], re.M) is None:
                bad+=1; print("MISSING parse",scope,name)
    print(f"{scope}: {len([k for k in doc['$defs'] if k[:1].isupper()])} exports, annotated={len([k for k,v in doc['$defs'].items() if isinstance(v,dict) and 'x-semio-formats' in v])}, formats={sorted(formats)}")
print("mismatches =",bad)
PY
