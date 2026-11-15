#!/usr/bin/env python3
"""Independent oracle of the portable observations: applies the rules of the schema to the rows of every fixture case and prints the portable rows and the local rows as JSON. Usage: portable.oracle.py <schema.json> <fixture.json>"""
import json,re,sys
schema=json.load(open(sys.argv[1],encoding="utf8"));fixture=json.load(open(sys.argv[2],encoding="utf8"))
policy=schema["definitions"]["Policy"]["const"]
DEP=re.escape(policy["dependencyRoot"]);ID=re.compile("("+DEP+r"[\\/])([0-9a-f]{16,64})(?=[\\/]|$)")
def above(path,root):
    a=path.replace("\\","/");a=a if a.endswith("/") else a+"/"
    b=root.replace("\\","/").rstrip("/")+"/"
    return a!=b and b.startswith(a)
def strings(value,fn):
    if isinstance(value,str):return fn(value)
    if isinstance(value,list):return [strings(v,fn) for v in value]
    if isinstance(value,dict):return {k:strings(v,fn) for k,v in value.items()}
    return value
def rows_text(row):return json.dumps([row["path"],row.get("fact")],ensure_ascii=False)
def portable(rows,root):
    kept=[r for r in rows if not (r["kind"] in policy["locationKinds"] and r.get("fact")=="directory" and above(r["path"],root))]
    ids={m.group(2) for r in kept for m in ID.finditer(rows_text(r))}
    if len(ids)!=1:return kept
    gid=next(iter(ids));pat=re.compile("("+DEP+r"[\\/])"+gid+r"(?=[\\/]|$)")
    return [dict(r,path=pat.sub(lambda m:m.group(1)+policy["token"],r["path"]),**({"fact":strings(r["fact"],lambda s:pat.sub(lambda m:m.group(1)+policy["token"],s))} if "fact" in r else {})) for r in kept]
def local(rows,generation):
    put=generation if generation is not None else policy["unavailable"]
    return [dict(r,path=r["path"].replace(policy["token"],put),**({"fact":strings(r["fact"],lambda s:s.replace(policy["token"],put))} if "fact" in r else {})) for r in rows]
out=[]
for case in fixture["cases"]:
    p=portable(case["rows"],case["root"]);out.append({"name":case["name"],"portable":p,"local":local(p,case["generation"])})
print(json.dumps(out,ensure_ascii=False,sort_keys=True))
