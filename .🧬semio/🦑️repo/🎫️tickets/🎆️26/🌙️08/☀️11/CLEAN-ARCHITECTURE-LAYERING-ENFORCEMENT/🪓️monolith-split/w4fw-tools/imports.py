import json, re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lex import mask
from graph import FILES
EXTERN = ['semio_framework_artifact_reference','semio_framework_io_sqlite_snapshot','replication','semio_framework_io_schema','semio_framework_pack_json','semio_framework_diagnostic','semio_framework_dsl','semio_framework_ui_locale','semio_framework_schema_registry','semio_framework_value','semio_framework_ui_contract','semio_framework_actor','semio_framework_job','semio_framework_tool_run','serde','serde_json','ui_wgpu','semio_framework_hash','semio_framework_io_base64','semio_framework_trace','semio_framework_mesh_engine','semio_framework_value_derive','semio_framework_schema','semio_framework_async','pack','semio_framework_async_macros','base64','std','core','alloc']
def flatten(s):
    """flatten a use tree string 'a::b::{c, d as e, f::{g}}' into list of (path, alias)"""
    s = s.strip().rstrip(';').strip()
    out = []
    def rec(prefix, t):
        t = t.strip()
        if not t: return
        if t.startswith('{'):
            inner = t[1:t.rindex('}')]
            parts = []; depth = 0; cur = ''
            for c in inner:
                if c == '{': depth += 1
                if c == '}': depth -= 1
                if c == ',' and depth == 0:
                    parts.append(cur); cur = ''
                else: cur += c
            parts.append(cur)
            for p in parts: rec(prefix, p)
            return
        m = re.match(r'^([A-Za-z_0-9:]+?)::(\{.*\}|\*)$', t, re.S)
        if m:
            if m.group(2) == '*':
                out.append((prefix + m.group(1).split('::'), '*')); return
            rec(prefix + m.group(1).split('::'), m.group(2)); return
        m = re.match(r'^([A-Za-z_0-9:]+)(?:\s+as\s+([A-Za-z_0-9]+))?$', t)
        if m:
            path = prefix + m.group(1).split('::')
            alias = m.group(2) or path[-1]
            out.append((path, alias))
    rec([], s)
    return out
def file_imports(text):
    m = mask(text, keep_strings=False)
    res = []
    for mm in re.finditer(r'(?m)^[ \t]*(?:#\[[^\]]*\]\s*)*(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);', m):
        res.append((bool(mm.group(1)), flatten(mm.group(2).replace('\n',' '))))
    return res
if __name__ == '__main__':
    out = {}
    for k, p in FILES.items():
        t = open(p, encoding='utf-8').read()
        imps = file_imports(t)
        out[k] = [dict(pub=pub, items=[dict(path='::'.join(pa), alias=al) for pa, al in items]) for pub, items in imps]
    json.dump(out, open('imports.json','w'), ensure_ascii=False, indent=1)
    for k, v in out.items():
        print("==", k)
        for u in v:
            print('  ', 'pub' if u['pub'] else '   ', ', '.join(f"{i['path']}" + ('' if i['alias']==i['path'].split('::')[-1] else ' as '+i['alias']) for i in u['items'])[:300])
