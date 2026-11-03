import json,sys
p="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/"+sys.argv[1]+"/🔣️.json"
s=open(p,encoding="utf-8",newline="").read()
d=json.loads(s)
for nl in ("\n","\r\n"):
    o=json.dumps(d,indent=2,ensure_ascii=False).replace("\n",nl)
    print(nl.encode(), o==s, o==s.rstrip("\r\n"), len(o),len(s))
o=json.dumps(d,indent=2,ensure_ascii=False)+"\n"
print(o==s)
import difflib
for i,(a,b) in enumerate(zip(o.splitlines(),s.splitlines())):
    if a!=b: print(i,repr(a),repr(b)); break
