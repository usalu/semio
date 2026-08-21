import re, json
BASE="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"

def struct_fields(path, name):
    src=open(path).read()
    m=re.search(r'pub struct '+name+r' \{(.*?)\n\}', src, re.S)
    body=m.group(1)
    out=[]
    for line in body.split('\n'):
        l=line.strip()
        fm=re.match(r'pub (\w+): (.+),$', l)
        if fm: out.append((fm.group(1), fm.group(2)))
    return out

snap=struct_fields(f"{BASE}/📸️snapshot/🦀️component.rs", "ProgramSnapshot")
diff=struct_fields(f"{BASE}/🔺️diff/🦀️component.rs", "ProgramDiff")
json.dump({'snapshot':snap,'diff':diff}, open('shape.json','w'), ensure_ascii=False, indent=1)
print(len(snap), len(diff))
print(snap[:4]); print(diff[:6]); print(diff[-6:])
