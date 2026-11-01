import re,os,sys,collections
D="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details"
files=["🦀️"]+"admission candidate_rules command_wire constituent_roster contextual contextual_edit contextual_render domain_edit domain_work entity_remove inverse pointer preimage receive reference source_loan type-guidance type_guidance_candidate type_guidance_request type_input type_input_choices type_input_kind type_input_list_copy type_input_select type_input_set_controls type_input_set_copy type_input_typed_copy type_metadata type_remap value_edit wire work".split()
files+=["dom:core","dom:cursor","dom:🦀️","dom:membership","dom:ifc4"]
def path(f):
    if f.startswith("dom:"): return D+"/../📚️catalog/⚖️domain/"+f[4:]+".rs"
    return D+"/"+f+".rs"
def strip(s):
    s=re.sub(r'//[^\n]*','',s)
    s=re.sub(r'/\*.*?\*/','',s,flags=re.S)
    s=re.sub(r'"(\\.|[^"\\])*"','""',s)
    return s
src={}
for f in files:
    t=open(path(f)).read()
    # drop cfg(test) mod blocks heuristically: cut at '#[cfg(test)]'
    i=t.find("#[cfg(test)]")
    src[f]=strip(t[:i] if i>=0 else t)
defs=collections.defaultdict(set)
pat=re.compile(r'^\s*(?:pub(?:\([a-z]+\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:struct|enum|trait|fn|const|type|static|union)\s+([A-Za-z_][A-Za-z0-9_]*)',re.M)
for f,t in src.items():
    # only top-level-ish: indentation <=4 and not inside impl: approximate by col 0
    for m in re.finditer(r'^(?:pub(?:\([a-z]+\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:struct|enum|trait|fn|const|type|static|union)\s+([A-Za-z_][A-Za-z0-9_]*)',t,re.M):
        defs[m.group(1)].add(f)
mode=sys.argv[1] if len(sys.argv)>1 else "edges"
edges=collections.defaultdict(lambda:collections.defaultdict(set))
for f,t in src.items():
    ids=set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*',t))
    for n,fs in defs.items():
        if n in ids:
            for g in fs:
                if g!=f: edges[f][g].add(n)
for f in files:
    print("==",f,len(src[f]))
    for g,ns in sorted(edges[f].items()):
        print("   ->",g,sorted(ns)[:12],len(ns))
