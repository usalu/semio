import json,sys,re
filt=sys.argv[2] if len(sys.argv)>2 else ''
n=0
for line in open(sys.argv[1]):
    try: m=json.loads(line)
    except Exception: continue
    if m.get('reason')!='compiler-message': continue
    msg=m['message']
    if msg['level']!='error': continue
    r=msg.get('rendered') or ''
    if filt and filt not in r: continue
    n+=1
    r=re.sub(r'🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/(\./)?(\.\./)+','',r)
    lines=[l[:400] for l in r.split('\n') if l.strip() not in ('|','')]
    print('\n'.join(lines[:40]));print('-----')
print('TOTAL',n)
