import re,sys
t=open(sys.argv[1],encoding='utf8').read()
for b in re.split(r'\n(?=error)',t):
    if not b.startswith('error') or b.startswith('error: could'): continue
    m=re.search(r'--> (\S+?):(\d+):(\d+)',b)
    p=re.sub(r'/📦️packages/🦀️rust/(\.\./)+','/…/',m.group(1)) if m else ''
    ex=[l.strip()[:170] for l in b.split('\n')[1:] if 'expected' in l or 'found' in l or 'missing' in l][:1]
    print(b.split('\n')[0][:130],'\n   ',p.replace('🧰️framework/🛍️products/💻️os/🔨️modules/',''),m.group(2) if m else '','|',ex)
