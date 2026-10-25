import subprocess,re,sys
files=subprocess.run(['rg','-l','RetainedClone','✏️s/🔌️plugins','-g','🦀️.rs'],capture_output=True,text=True).stdout.split('\n')
total=0
for f in files:
    if not f.strip(): continue
    s=open(f,encoding='utf-8').read()
    out=[];last=0;n=0
    for m in re.finditer(r'#\[derive\(([^\]]*)\)\]\s*(?:#\[[^\]]*\]\s*)*pub (?:struct|enum) (\w+)',s):
        d=m.group(1)
        if 'RetainedClone' in d and 'RetireOwned' not in d:
            if re.search(r'impl[^{]*RetireOwned for '+m.group(2)+r'\b',s): continue
            pref='value_derive::' if 'value_derive::RetainedClone' in d else 'semio_framework_value::'
            nd=re.sub(r'((?:value_derive|semio_framework_value)::RetainedClone)', pref+'RetireOwned, \\1', d, count=1)
            if nd==d: nd=d.rstrip()+', '+pref+'RetireOwned'
            out.append(s[last:m.start(1)]);out.append(nd);last=m.end(1);n+=1
    if n:
        out.append(s[last:]);open(f,'w',encoding='utf-8').write(''.join(out));total+=n
print(total)
