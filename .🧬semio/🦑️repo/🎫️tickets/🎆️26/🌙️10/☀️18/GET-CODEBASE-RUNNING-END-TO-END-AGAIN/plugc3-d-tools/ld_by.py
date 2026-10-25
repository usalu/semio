import re,sys
def split_top(s):
    parts=[];d=0;cur='';q=False;i=0
    while i<len(s):
        c=s[i]
        if c=='"' and (i==0 or s[i-1]!='\\'): q=not q
        if not q:
            if c in '<({[': d+=1
            elif c in '>)}]' and not (c=='>' and s[i-1]=='-'): d-=1
            elif c==',' and d==0: parts.append(cur);cur='';i+=1;continue
        cur+=c;i+=1
    if cur.strip(): parts.append(cur)
    return parts
def match_brace(s,i):
    d=1
    while d and i<len(s):
        d+=(s[i]=='{')-(s[i]=='}');i+=1
    return i
# usage: ld_by.py file...  rewrite `key: ident` / `key: T = |row| expr` list deltas (foreign rows) to `= by <Delta>Keys` with a local KeyOf marker
for f in sys.argv[1:]:
    text=open(f).read();markers=[];res='';pos=0;last_end=None
    for m in re.finditer(r'(?:protocol::)?(?:plain_)?list_delta!\s*\{',text):
        i=match_brace(text,m.end())
        body=text[m.end():i-1]
        inner=re.search(r'pub\s+(\w+)\s*\{',body)
        j=match_brace(body,inner.end())
        fields=split_top(body[inner.end():j-1])
        kv={};order=[]
        for p in fields:
            k,v=(p.split(':',1)+[''])[:2];kv[k.strip()]=v.strip();order.append(k.strip())
        keyv=kv.get('key','');row=kv['row'];delta=inner.group(1)
        if re.match(r'.*=\s*(by|keyed)\b',keyv,re.S): continue
        mk=re.sub(r'Delta$','',delta)+'Keys'
        if '=' in keyv:
            kt,expr=keyv.split('=',1);kt=kt.strip();m2=re.match(r'\|(\w+)\|\s*(.*)$',expr.strip(),re.S);arg,ex=m2.group(1),m2.group(2)
            assert arg=='row',arg
            lst=kv['list']
        else:
            kt='String';ex=f'row.{keyv}.clone()';lst=f'Vec<{row}>'
        kv2=dict(kv);kv2.pop('key');kv2['list']=lst
        new=[]
        for k in order:
            if k=='key':
                new.append(f'list: {lst}');new.append(f'key: {kt} = by {mk}')
            elif k=='list': continue
            elif k=='values_only': new.append('values_only')
            else: new.append(f'{k}: {kv[k]}')
        new=[x for x in new if not x.startswith('values_only')]+(['values_only'] if 'values_only' in order else [])
        nb=body[:inner.end()]+' '+', '.join(new)+' }'+body[j:]
        res+=text[pos:m.end()]+nb+'}';pos=i;last_end=len(res)
        markers.append(f'/// 🔑️ Keys the foreign `{row}` rows of their list delta.\n#[derive(Clone, Debug, PartialEq)]\npub struct {mk};\n\nimpl protocol::list_delta::KeyOf<{row}> for {mk} {{\n    type Key = {kt};\n    fn key_of(row: &{row}) -> {kt} {{ {ex} }}\n}}\n')
    if not markers: print('none',f);continue
    res+=text[pos:]
    res=res[:last_end]+'\n\n'+'\n'.join(markers)+res[last_end:]
    open(f,'w').write(res);print('rewrote',f,len(markers))
