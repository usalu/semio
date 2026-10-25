import hashlib,json,re,sys
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
README='🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md'
APPROVAL=L+'🖼️assets/🪪️current-source-approval/🔣️.json'
POLICY=L+'🖼️assets/🏠️owner-projection-policy/🔣️.json'
TAX=L+'🔣️taxonomy.json'
sha=lambda p:hashlib.sha256(open(p,'rb').read()).hexdigest()
rd=open(README,'rb').read()
a=open(APPROVAL,encoding='utf8').read(); d=json.loads(a)
d['policy']['sha256']=sha(POLICY)
d['currentPreimage']={'sha256':hashlib.sha256(rd).hexdigest(),'size':len(rd),'mode':'0644'}
na=json.dumps(d,indent=2,ensure_ascii=False)+("\n" if a.endswith("\n") else "")
t=open(TAX,encoding='utf8').read()
oldsha=re.search(r'"approvalPath": "'+re.escape(APPROVAL)+r'",\s*"approvalSha256": "([0-9a-f]{64})"',t).group(1)
cur=json.loads(a)['currentPreimage']
assert json.dumps(json.loads(a),indent=2,ensure_ascii=False)+'\n'==a or json.dumps(json.loads(a),indent=2,ensure_ascii=False)==a
def rep(s,old,new):
  assert s.count(old)==1,(old,s.count(old)); return s.replace(old,new)
nb=na.encode(); nsha=hashlib.sha256(nb).hexdigest()
t=rep(t,oldsha,nsha)
pat=re.compile(r'("sha256": ")'+cur['sha256']+r'(",\s*"size": )'+str(cur['size']))
assert len(pat.findall(t))==1
t=pat.sub(lambda m:m.group(1)+d['currentPreimage']['sha256']+m.group(2)+str(d['currentPreimage']['size']),t)
print(oldsha,'->',nsha)
if '--write' in sys.argv:
  open(APPROVAL,'w',encoding='utf8').write(na); open(TAX,'w',encoding='utf8').write(t)
