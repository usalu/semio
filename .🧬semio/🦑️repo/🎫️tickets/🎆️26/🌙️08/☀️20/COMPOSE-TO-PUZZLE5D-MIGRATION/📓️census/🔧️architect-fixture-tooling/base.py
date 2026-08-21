import json, copy
exec(open('values.py').read())
shape=json.load(open('shape.json'))

EMPTY_CHILD_HASH="7904dd65836c8ff4"
EPOCH={"created":"1970-01-01T00:00:00Z","updated":"1970-01-01T00:00:00Z"}

def child(kind, h):
    return {"childId": f"architect-{kind}-{h}",
            "target": {"artifactId": f"architect-program-{kind}",
                       "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "table"}}}

def stamp(obj):
    """recursively replace empty timestamps with the epoch default"""
    if isinstance(obj, dict):
        if set(obj.keys())=={"created","updated"} and obj["created"]=="" :
            return dict(EPOCH)
        return {k: stamp(v) for k,v in obj.items()}
    if isinstance(obj, list): return [stamp(v) for v in obj]
    return obj

def base_meta():
    m=stamp(struct_value('ProgramMeta'))
    m['schema']="architect.program"; m['documentId']="document-fixture"
    m['title']="Fixture Program"; m['locale']="en"; m['revision']="0"
    return m

def base_project():
    p=stamp(struct_value('ProjectDefinition'))
    p['id']="project-fixture"; p['code']="FIX-000"
    return p

def base_governance():
    g=stamp(struct_value('Governance'))
    g['id']="governance-fixture"; g['framework']="ISO 9001"
    return g

def base_snapshot():
    out={}
    for name, ty in shape['snapshot']:
        key=camel(name)
        if name=='schema': out[key]="architect.program"
        elif name=='meta': out[key]=base_meta()
        elif name=='project': out[key]=base_project()
        elif name=='governance': out[key]=base_governance()
        elif name=='knowledge': out[key]=child("knowledge", EMPTY_CHILD_HASH)
        elif name=='benchmarks': out[key]=child("benchmarks", EMPTY_CHILD_HASH)
        elif ty.startswith('Vec<'): out[key]=[]
        else: raise SystemExit('unhandled snapshot field '+name+': '+ty)
    return out

def empty_diff():
    return {camel(n): None for n,_ in shape['diff']}

def delta(added=None, removed=None, patched=None):
    return {"added": added or [], "removed": removed or [], "patched": patched or [], "reordered": None}

def title_of(slug):
    return ' '.join(w[:1].upper()+w[1:] for w in slug.split('-'))

def row(etype, rid, name):
    r=stamp(struct_value(etype))
    set_path(r, etype, 'header.id' if 'header' in [f['name'] for f in structs[etype]['fields']] else 'id', rid)
    try: set_path(r, etype, 'header.name', name)
    except SystemExit: pass
    return r
