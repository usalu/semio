import json
def dump_schema(d):
    c=lambda v:json.dumps(v,separators=(",",":"),ensure_ascii=False)
    items=list(d.items());rows=[]
    for k,v in items:
        if k=="$defs" and isinstance(v,dict):
            body=",\n".join(f"    {c(dk)}:{c(dv)}" for dk,dv in v.items())
            rows.append(f'  {c(k)}:{{\n{body}\n  }}')
        else:rows.append(f"  {c(k)}:{c(v)}")
    return "{\n"+",\n".join(rows)+"\n}\n"
