import re, json, os

BASE="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
FILES=[f"{BASE}/🧱️kernel/🦀️component.rs", f"{BASE}/🗄️registers/🦀️component.rs"]

def camel(s):
    parts=s.split('_')
    return parts[0]+''.join(p[:1].upper()+p[1:] for p in parts[1:])

def variant_camel(s):
    return s[:1].lower()+s[1:]

def parse(files):
    structs={}; enums={}; patchable={}
    for path in files:
        src=open(path).read()
        # strip test module? keep, harmless
        lines=src.split('\n')
        i=0
        while i<len(lines):
            line=lines[i]
            m=re.match(r'^pub struct (\w+) \{$', line)
            if m:
                name=m.group(1)
                # gather preceding attrs
                attrs=[]; j=i-1
                while j>=0 and (lines[j].startswith('#[') or lines[j].startswith('///') or lines[j].startswith('//')):
                    attrs.append(lines[j]); j-=1
                attrblob='\n'.join(attrs)
                rn=re.search(r'rename_all = "([^"]+)"', attrblob)
                container_default='serde(' in attrblob and re.search(r'#\[serde\([^)]*\bdefault\b', attrblob) is not None
                fields=[]
                pend=[]
                i+=1
                while i<len(lines) and lines[i]!='}':
                    l=lines[i].strip()
                    if l.startswith('#['): pend.append(l)
                    elif l.startswith('///') or l.startswith('//') or l=='':
                        pass
                    else:
                        fm=re.match(r'pub (\w+): (.+),$', l)
                        if fm:
                            fields.append({'name':fm.group(1),'type':fm.group(2),'attrs':'\n'.join(pend)})
                            pend=[]
                    i+=1
                structs[name]={'rename_all':rn.group(1) if rn else None,'default':container_default,'fields':fields,'file':path}
            m=re.match(r'^pub enum (\w+) \{$', line)
            if m:
                name=m.group(1)
                attrs=[]; j=i-1
                while j>=0 and (lines[j].startswith('#[') or lines[j].startswith('///') or lines[j].startswith('//')):
                    attrs.append(lines[j]); j-=1
                attrblob='\n'.join(attrs)
                rn=re.search(r'rename_all = "([^"]+)"', attrblob)
                variants=[]; default=None; pend=[]
                i+=1
                while i<len(lines) and lines[i]!='}':
                    l=lines[i].strip()
                    if l=='#[default]': pend.append('default')
                    elif l.startswith('#['): pass
                    elif l.startswith('//') or l=='': pass
                    else:
                        vm=re.match(r'(\w+)[,{ ]', l+' ')
                        if vm:
                            variants.append({'name':vm.group(1),'fields': '{' in l or '(' in l})
                            if 'default' in pend: default=vm.group(1)
                            pend=[]
                    i+=1
                enums[name]={'rename_all':rn.group(1) if rn else None,'variants':variants,'default':default}
            i+=1
        for m in re.finditer(r'impl_patchable!\(\s*(\w+),\s*(\w+),\s*\{(.*?)\}\s*\);', src, re.S):
            ent, pat, body = m.group(1), m.group(2), m.group(3)
            pairs=re.findall(r'\[([\w.]+)\]\s*=>\s*(\w+)', body)
            patchable[ent]=(pat,[(f,p) for p,f in pairs])
    return structs, enums, patchable

structs, enums, patchable = parse(FILES)
print("structs", len(structs), "enums", len(enums), "patchable", len(patchable))
missing=[e for e in patchable if e not in structs]
print("patchable missing struct:", missing)
json.dump({'structs':structs,'enums':enums,'patchable':patchable}, open('types.json','w'), ensure_ascii=False, indent=1)
print(json.dumps(structs['Stakeholder'], ensure_ascii=False)[:600])
print(json.dumps(enums['LifecycleStatus'], ensure_ascii=False)[:300])
print(patchable['Stakeholder'][0], patchable['Stakeholder'][1][:4])
