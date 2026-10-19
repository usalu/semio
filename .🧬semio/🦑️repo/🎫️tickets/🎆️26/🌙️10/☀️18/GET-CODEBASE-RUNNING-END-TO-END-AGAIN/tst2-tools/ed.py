import os,re
ROOT="/Users/ueli/Documents/semio"
MT=ROOT+"/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts"
def rel(path):
    p=path if path.startswith("/") else os.path.join(ROOT,path)
    return p,os.path.relpath(MT,os.path.dirname(p))
def edit(path, pairs, add_import=False, name="readModuleTreeSource"):
    p,r=rel(path)
    t=open(p,encoding="utf-8").read()
    for old,new,cnt in pairs:
        n=t.count(old)
        if n!=cnt: raise SystemExit(f"count mismatch {n}!={cnt} for {old[:90]!r} in {path}")
        t=t.replace(old,new)
    if add_import and not re.search(r'import[^;]*\b'+name+r'\b[^;]*from',t):
        mm=re.search(r'import \{ ([^}]*) \} from "'+re.escape(r)+'";',t)
        if mm:
            t=t.replace(mm.group(0),'import { '+mm.group(1)+', '+name+' } from "'+r+'";',1)
            open(p,"w",encoding="utf-8").write(t); return
        lines=t.split("\n"); idx=-1
        for i,l in enumerate(lines):
            if re.search(r'from\s*["\'][^"\']+["\']\s*(with\s*\{[^}]*\})?\s*;?\s*$',l) and (l.startswith("import") or l.startswith("}")): idx=i
            if re.match(r'^(export |const |function |test\(|describe\(|let |type |interface )',l): break
        imp=f'import {{ {name} }} from "{r}";'
        lines.insert(idx+1,imp); t="\n".join(lines)
    open(p,"w",encoding="utf-8").write(t)
