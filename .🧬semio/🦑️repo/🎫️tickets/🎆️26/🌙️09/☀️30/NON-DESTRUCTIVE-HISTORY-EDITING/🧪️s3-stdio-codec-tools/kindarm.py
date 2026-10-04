import re,sys
for p in sys.argv[1:]:
    s=open(p,encoding='utf-8').read()
    out=[];n=0
    lines=s.split('\n')
    for i,l in enumerate(lines):
        out.append(l)
        m=re.match(r'^(\s*)(\w+)::SetSnapshot\([^)]*\) => "set-snapshot",\s*$',l)
        if m and not any('::PatchSnapshot(' in x for x in lines[max(0,i-3):i+4]):
            out.append(f'{m.group(1)}{m.group(2)}::PatchSnapshot(_) => "patch-snapshot",'); n+=1
    open(p,'w',encoding='utf-8').write('\n'.join(out)); print(n,p.split('🗿️artifacts/')[1][:60])
