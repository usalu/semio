import json
p="/Users/ueli/Documents/semio/package.json"
s=open(p,encoding="utf8").read()
a='''    "sharp": "^0.34.5",
    "three-mesh-bvh": "^0.9.14"
  },'''
assert s.count(a)==1
s=s.replace(a,'''    "sharp": "^0.34.5"
  },''')
b='''    "three": "^0.182.0",\n'''
assert s.count(b)==1
s=s.replace(b,b+'''    "three-mesh-bvh": "^0.9.14",\n''')
d=json.loads(s)
assert "three-mesh-bvh" not in d["dependencies"] and d["devDependencies"]["three-mesh-bvh"]=="^0.9.14"
open(p,"w",encoding="utf8").write(s)
print("ok")
