import json,re,sys,pathlib
root=pathlib.Path('✏️s/🔌️plugins/📕️norm/🗿️artifacts')
files={'en1992':root/'🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff',
 'en1994':root/'🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff'}
def camel(s):
    p=s.split('_'); return p[0]+''.join(x[:1].upper()+x[1:] for x in p[1:])
def split_top(s):
    s=s.replace('=>','→')
    out=[];d=0;cur=''
    for ch in s:
        if ch in '<([{': d+=1
        if ch in '>)]}': d-=1
        if ch==',' and d==0: out.append(cur.strip());cur=''
        else: cur+=ch
    if cur.strip(): out.append(cur.strip())
    return out
def groups(body):
    res={}
    for g in ('set','within','optional','rows'):
        m=re.search(r'\b'+g+r'\s*\{(.*?)\}\s*(?=(?:within|optional|rows|set)\s*\{|$)',body,re.S) if True else None
        res[g]=m.group(1) if m else ''
    return res
apply=len(sys.argv)>1
for fam,d in files.items():
    schema=json.load(open(d/'🔣️.json'))['$defs']
    for f in [d/'🦀️.rs']+sorted(d.glob('*/🦀️.rs')):
        txt=f.read_text()
        for m in re.finditer(r'nested_patch! \{\n((?:    ///[^\n]*\n)?)    (\w+) for ([\w:]+) \{\n(.*?)\n    \}\n\}',txt,re.S):
            doc,name,row,body=m.groups()
            lines=[l.strip() for l in body.split('\n') if l.strip()]
            ent={}
            for l in lines:
                g=re.match(r'(\w+)\s*\{(.*)\}$',l)
                kind,inner=g.groups()
                for item in split_top(inner):
                    if kind=='set':
                        n,t=item.split(':',1); ent[camel(n.strip())]=('leaf',n.strip(),t.strip(),None)
                    elif kind=='within':
                        a,b=item.split('→',1); n=a.strip(); p,t=b.split(':',1); o,i=[x.strip() for x in p.strip().split('.')]
                        ent[camel(n)]=('nested' if fam=='en1994' else 'nested_option',n,t.strip(),f'({o}.{i})')
                    elif kind=='optional':
                        a,flag=item.split('→'); n,t=a.split(':',1); n=n.strip(); flag=flag.strip()
                        ent[camel(n)]=('clearable',n,t.strip(),f'({flag})'); ent[camel(flag)]=('clear',flag,'bool',f'({n})')
                    elif kind=='rows':
                        n,rest=item.split(':',1); t,lab=rest.split('=',1); ent[camel(n.strip())]=('rows',n.strip(),t.strip(),lab.strip())
            order=list(schema[name]['properties'].keys())
            assert set(order)==set(ent),(name,set(order)^set(ent))
            cur=[camel(x[1]) for x in ent.values()]
            print(fam,name,'same-order' if cur==order else 'DIFFERENT',len(order))
            new=''.join(f'        {ent[k][0]} {ent[k][1]}: {ent[k][2]}'+(f' = {ent[k][3]}' if ent[k][3] else '')+',\n' for k in order)
            repl=f'nested_patch! {{\n{doc}    {name} for {row} {{\n{new}    }}\n}}'
            txt=txt.replace(m.group(0),repl)
        if apply and 'nested_patch! {' in f.read_text(): f.write_text(txt)
