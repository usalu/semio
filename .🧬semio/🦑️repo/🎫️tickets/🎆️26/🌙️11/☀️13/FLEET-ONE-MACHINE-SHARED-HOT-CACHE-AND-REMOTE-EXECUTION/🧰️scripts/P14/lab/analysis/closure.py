import re,os,sys,collections
root='/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs'
imp=re.compile(r'''(?:from|import)\s*\(?\s*["'](\.{1,2}/[^"']+)["']''')
seen=set();stack=[root]
def res(base,spec):
    p=os.path.normpath(os.path.join(os.path.dirname(base),spec))
    for c in (p,p+'.mjs',p+'.ts',p+'.js',p+'/🟨️.mjs',p+'/🟦️.ts'):
        if os.path.isfile(c): return c
while stack:
    f=stack.pop()
    if f in seen or not os.path.isfile(f): continue
    seen.add(f)
    if f.endswith('.json') : continue
    try: t=open(f,encoding='utf8').read()
    except: continue
    for m in imp.finditer(t):
        r=res(f,m.group(1))
        if r and '/🧪️' not in r: stack.append(r)
print(len(seen),'files')
pat=re.compile(r'process\.platform|process\.arch|platform\(\)|os\.type|os\.arch|\barch\(\)|win32|sys\.platform|EOL|\.sep\b|toLocale|case-?insensitive|caseSensitive|CaseSensitive|hostname\(\)|userInfo|homedir')
for f in sorted(seen):
    if f.endswith('.json'):continue
    try: t=open(f,encoding='utf8').read()
    except: continue
    hits=[(i+1,l.strip()[:200]) for i,l in enumerate(t.split('\n')) if pat.search(l) and not l.strip().startswith(('//','*','/**'))]
    if hits:
        print('==',f.replace('/Users/Shared/semio/','')[-130:]);[print('  ',h) for h in hits[:6]]
