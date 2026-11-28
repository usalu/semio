import json,sys,re
root="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/"
suites=root+"🧪️testing/🗂️suites/🔣️.json"
project=root+"📋️project.json"
sid="build-checkout-snapshot"
s=open(suites,encoding="utf8").read()
assert '"'+sid+'"' not in s
anchor='    {\n      "id": "module-basics",'
assert s.count(anchor)==1
row='    {\n      "id": "%s",\n      "budgetMs": 300000,\n      "typecheck": "none",\n      "trees": [\n        "🏗️build/📸️checkout-snapshot"\n      ],\n      "platforms": [\n        "darwin",\n        "linux",\n        "win32"\n      ]\n    },\n'%sid
p=open(project,encoding="utf8").read()
a='    "test-module-basics": {'
assert p.count(a)==1 and '"test-%s"'%sid not in p
i=p.index('    "test-build-facets": {'); j=p.index(a)
block=p[i:j]
new=block.replace("test-build-facets","test-"+sid).replace("test build-facets","test "+sid)
assert new!=block
open(suites,"w",encoding="utf8").write(s.replace(anchor,row+anchor))
open(project,"w",encoding="utf8").write(p[:j]+new+p[j:])
json.load(open(suites,encoding="utf8"));json.load(open(project,encoding="utf8"))
print("ok")
