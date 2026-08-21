import json, re
T=json.load(open('types.json'))
structs, enums, patchable = T['structs'], T['enums'], T['patchable']

def camel(s):
    p=s.split('_'); return p[0]+''.join(x[:1].upper()+x[1:] for x in p[1:])

def rename(name, rule):
    if rule=='camelCase': return camel(name)
    if rule is None: return name
    raise SystemExit('unknown rename rule '+rule)

def variant_name(en, v):
    rule=enums[en]['rename_all']
    if rule=='camelCase': return v[:1].lower()+v[1:]
    if rule is None: return v
    raise SystemExit('unknown enum rule')

def enum_default(en):
    e=enums[en]
    return variant_name(en, e['default'] or e['variants'][0]['name'])

def enum_alt(en):
    e=enums[en]
    d=e['default'] or e['variants'][0]['name']
    for v in e['variants']:
        if v['name']!=d: return variant_name(en, v['name'])
    return variant_name(en, d)

PRIM_INT={'u8','u16','u32','u64','usize','i8','i16','i32','i64','isize'}
PRIM_FLOAT={'f32','f64'}

def value_of(ty, path=()):
    ty=ty.strip()
    if ty.startswith('Option<'): return None
    if ty.startswith('Vec<'): return []
    if ty=='String': return ""
    if ty=='bool': return False
    if ty in PRIM_INT: return 0
    if ty in PRIM_FLOAT: return 0.0
    if ty=='EntityId': return ""
    if ty in enums: return enum_default(ty)
    if ty in structs: return struct_value(ty)
    raise SystemExit('unknown type '+ty+' at '+str(path))

def struct_value(name):
    s=structs[name]
    out={}
    for f in s['fields']:
        if 'serde(skip)' in f['attrs'] and 'skip_serializing_if' not in f['attrs']: continue
        if 'flatten' in f['attrs']:
            out.update(struct_value(f['type']))
            continue
        v=value_of(f['type'], (name,f['name']))
        if 'skip_serializing_if = "Option::is_none"' in f['attrs'] and v is None: continue
        if 'skip_serializing_if = "Vec::is_empty"' in f['attrs'] and v==[]: continue
        out[rename(f['name'], s['rename_all'])]=v
    return out

def entity_fields(name):
    """flattened (json_key, rust_path, rust_type, emitted) list"""
    s=structs[name]; res=[]
    for f in s['fields']:
        if 'flatten' in f['attrs']:
            for k,p,t,a in entity_fields(f['type']):
                res.append((k, f['name']+'.'+p, t, a))
        else:
            res.append((rename(f['name'], s['rename_all']), f['name'], f['type'], f['attrs']))
    return res

def set_path(obj, entity, rust_path, value):
    """set a value in the flattened json object of `entity` addressed by rust path"""
    for k,p,t,a in entity_fields(entity):
        if p==rust_path:
            obj[k]=value; return
    raise SystemExit('no path '+rust_path+' in '+entity)

def get_path(obj, entity, rust_path):
    for k,p,t,a in entity_fields(entity):
        if p==rust_path:
            if k in obj: return obj[k]
            # skipped -> derive from type
            return value_of(t)
    raise SystemExit('no path '+rust_path+' in '+entity)

def path_type(entity, rust_path):
    for k,p,t,a in entity_fields(entity):
        if p==rust_path: return t
    raise SystemExit('no path '+rust_path)
