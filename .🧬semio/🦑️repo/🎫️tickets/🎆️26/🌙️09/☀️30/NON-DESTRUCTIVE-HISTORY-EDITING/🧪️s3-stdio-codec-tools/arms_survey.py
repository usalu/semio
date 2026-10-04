import re,subprocess,sys,collections
root='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
out=subprocess.run(['git','grep','-l','-z','PatchSnapshot(patch_snapshot::PatchSnapshot)','--',root],capture_output=True).stdout.decode().split('\0')
enums=set()
for f in out:
    if not f: continue
    s=open(f,encoding='utf-8').read()
    m=re.search(r'pub enum (\w+)\s*\{[^}]*PatchSnapshot\(patch_snapshot::PatchSnapshot\)',s)
    if m: enums.add(m.group(1))
# also crate-root mounted variants
out2=subprocess.run(['git','grep','-l','-z','PatchSnapshot(super::patch_snapshot::PatchSnapshot)','--',root],capture_output=True).stdout.decode().split('\0')
for f in out2:
    if not f: continue
    s=open(f,encoding='utf-8').read()
    m=re.search(r'pub enum (\w+)\s*\{[^}]*PatchSnapshot\(super::patch_snapshot::PatchSnapshot\)',s)
    if m: enums.add(m.group(1))
print('enums',len(enums),sorted(enums))
pat='|'.join(sorted(enums))
files=subprocess.run(['git','grep','-l','-z','-E',r'('+pat+r')::SetSnapshot','--',root+'/*.rs'],capture_output=True).stdout.decode().split('\0')
for f in files:
    if not f or any(x in f for x in ['📝️md/','🌐️html/','📄️txt/']): continue
    s=open(f,encoding='utf-8').read().split('\n')
    sa=collections.Counter(); pa=collections.Counter(); sc=collections.Counter(); pc=collections.Counter()
    for i,l in enumerate(s):
        for e in enums:
            if re.search(rf'\b{e}::SetSnapshot\b[^=]*=>',l): sa[e]+=1
            if re.search(rf'\b{e}::PatchSnapshot\b[^=]*=>',l): pa[e]+=1
            if re.search(rf'\b{e}::SetSnapshot\(\w+::SetSnapshot \{{',l) and '=>' not in l: sc[e]+=1
            if re.search(rf'\b{e}::PatchSnapshot\(',l) and '=>' not in l: pc[e]+=1
    for e in set(sa)|set(sc):
        if sa[e]>pa[e] or (sc[e] and not pc[e] and ('KINDS' in '\n'.join(s) or 'demo_mutation_cases' in '\n'.join(s) or 'one_per' in '\n'.join(s))):
            print(f"{e} arms {sa[e]}/{pa[e]} cons {sc[e]}/{pc[e]} :: {f.replace(root+'/','')}")
