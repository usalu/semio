import re,sys
sys.path.insert(0,"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1101-tools/ifcsplit")
D="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🦀️.rs"
lines=open(D).read().split("\n")
# top-level item starts
starts=[]
for i,l in enumerate(lines):
    m=re.match(r'^(?:pub(?:\([a-z]+\))?\s+)?(?:async\s+)?(fn|struct|enum|trait|impl|const|type|static|mod)\b\s*(?:<[^>]*>\s*)?([A-Za-z_][A-Za-z0-9_]*)?',l)
    if m: starts.append((i,m.group(1),m.group(2) or l[:40]))
    elif l.startswith("#[cfg(test)]") : starts.append((i,"cfgtest","tests"))
items=[]
for k,(i,kind,name) in enumerate(starts):
    j=starts[k+1][0] if k+1<len(starts) else len(lines)
    items.append((name,kind,i+1,j,"\n".join(lines[i:j])))
names=[n for n,k,_,_,_ in items if k in("fn","struct","enum","trait","const","type")]
for n,k,a,b,t in items:
    if k in("mod","cfgtest") and n in("tests",) : continue
    ids=set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*',t))
    refs=sorted(x for x in names if x in ids and x!=n)
    mods=sorted(x for x in "type_input type_guidance type_metadata candidate_rules contextual contextual_render contextual_edit domain_edit domain domain_cursor domain_work receive wire work identity source_index reference pointer value_edit entity_remove constituent_roster admission command_wire inverse preimage source_loan type_remap".split() if re.search(r'\b'+x+r'::',t))
    print(f"{a:5d}-{b:5d} {len(t):6d} {k:6s} {n:40s} refs={refs} mods={mods}")
