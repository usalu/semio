import hashlib,json,re,sys
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
POLICY=L+'🖼️assets/🏠️owner-projection-policy/🔣️.json'
TAX=L+'🔣️taxonomy.json'
ROWS=[('🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md',L+'🖼️assets/🪪️current-source-approval/🔣️.json'),('.devcontainer/README.md',L+'🖼️assets/🪪️devcontainer-current-source-approval/🔣️.json')]
sha=lambda b:hashlib.sha256(b).hexdigest()
t=open(TAX,encoding='utf8').read(); writes={}
for readme,approval in ROWS:
  rd=open(readme,'rb').read(); a=open(approval,encoding='utf8').read(); d=json.loads(a)
  assert json.dumps(d,indent=2,ensure_ascii=False)+"\n"==a
  cur=d['currentPreimage']; oldpol=d['policy']['sha256']
  d['policy']['sha256']=sha(open(POLICY,'rb').read())
  d['currentPreimage']={'sha256':sha(rd),'size':len(rd),'mode':'0644'}
  na=json.dumps(d,indent=2,ensure_ascii=False)+"\n"; nsha=sha(na.encode()); osha=sha(a.encode())
  pat=re.compile(r'("sha256": ")'+cur['sha256']+r'(",\s*"size": )'+str(cur['size']))
  assert len(pat.findall(t))==1,(readme,len(pat.findall(t)))
  t=pat.sub(lambda m:m.group(1)+d['currentPreimage']['sha256']+m.group(2)+str(d['currentPreimage']['size']),t)
  assert t.count(osha)==1,(approval,t.count(osha)); t=t.replace(osha,nsha)
  writes[approval]=na; print(readme,cur['sha256'][:8],'->',d['currentPreimage']['sha256'][:8],'approval',osha[:8],'->',nsha[:8])
if '--write' in sys.argv:
  for p,v in writes.items(): open(p,'w',encoding='utf8').write(v)
  open(TAX,'w',encoding='utf8').write(t)
