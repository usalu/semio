import json, copy, os, re
exec(open('base.py').read())
leaves=json.load(open('leaves.json'))
plan=json.load(open('plan.json'))
MROOT="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

def snake(s): return s.replace('-', '_')

def doc_of(d):
    """The leaf's own 🔺️diff docstring — the oracle sentence, quoted verbatim."""
    src=open(os.path.join(MROOT, d, "🔺️diff/🦀️component.rs")).read()
    body=src.split('pub async fn diff',1)[0]
    lines=[l for l in body.split('\n') if l.startswith('/// ')]
    return ' '.join(l[4:].strip() for l in lines)

#region choose the field a `replace` fixture actually changes
def pick_change(etype):
    fields=[f for f in structs[etype]['fields'] if 'flatten' not in f['attrs']]
    for want in ('String',):
        for f in fields:
            if f['type']==want: return f['name'], f['type']
    for f in fields:
        if f['type']=='bool': return f['name'], f['type']
    for f in fields:
        if f['type'] in ('f64','f32'): return f['name'], f['type']
    for f in fields:
        if f['type'] in enums and len(enums[f['type']]['variants'])>1: return f['name'], f['type']
    for f in fields:
        if f['type']=='Vec<String>': return f['name'], f['type']
    return 'header.name', 'String'

def changed_value(ty, title):
    if ty=='String': return f"Replaced {title} A"
    if ty=='bool': return True
    if ty in ('f64','f32'): return 2.0
    if ty in enums: return enum_alt(ty)
    if ty=='Vec<String>': return ["replaced"]
    raise SystemExit('no change value for '+ty)
#endregion

def full_patch(etype, after_row):
    pt, pairs = patchable[etype]
    out={}
    for pfield, epath in pairs:
        out[camel(pfield)] = get_path(after_row, etype, epath)
    return out

def rename_patch(etype, new_name):
    pt, pairs = patchable[etype]
    out={camel(f): None for f,_ in pairs}
    out['name']=new_name
    return out

CASES={}
def case(d, name, files, wiring_doc):
    CASES[d]={'case':name,'files':files,'doc':wiring_doc}

def mutation_json(kind_variant, payload):
    out={"mutation": kind_variant[:1].lower()+kind_variant[1:]}
    out.update(payload)
    return out
