from pathlib import Path
import os,json
def edit(p,fun):
 s=p.read_text(encoding='utf8');n=fun(s);assert n!=s and p.read_text(encoding='utf8')==s;t=p.with_name(p.name+'.terra.tmp');t.write_text(n,encoding='utf8',newline='');assert p.read_text(encoding='utf8')==s;os.replace(t,p)
r=Path('🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🪙️escrow')
def schema(s):
 o=json.loads(s);o['properties']['metadata']['maximum']=1024;o['properties']['items']['minimum']=21;return json.dumps(o,separators=(',',':'))+'\n'
edit(r/'🧬️schema/reserve.json',schema)
def tests(s):
 s=s.replace('metadata:384','metadata:1024').replace('metadata:385','metadata:1025').replace('items:18','items:21').replace('items:17','items:20').replace('+384','+1024').replace('work:129+88*rows','work:132+88*rows');return s
edit(r/'🧪️tests/reserve.test.ts',tests)
p=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🧪️tests/🟦️.ts')
def poll(s):
 s=s.replace('4400+header+384,work=2945','4400+header+1024,work=2948').replace('[18,copy','[21,copy').replace(',18,copy',',21,copy').replace('?>=18','?>=21');return s
edit(p,poll)
