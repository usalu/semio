import json,sys
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/package.json"
s=open(p,encoding="utf8").read()
old='''    "@semio-tech/schema-validator-ts": "workspace:*",
    "@eslint/js": "^10.0.1",
    "eslint-plugin-storybook": "10.4.0",
    "typescript-eslint": "^8.50.1"
  },'''
new='''    "@semio-tech/schema-validator-ts": "workspace:*"
  },'''
assert s.count(old)==1
s=s.replace(old,new)
old2='''  "devDependencies": {
    "@semio-tech/ui": "workspace:*",
'''
assert s.count(old2)==1
s=s.replace(old2,old2+'''    "@eslint/js": "^10.0.1",
''')
old3='''    "esbuild": "^0.27.2",
    "fast-glob": "3.3.3",'''
assert s.count(old3)==1
s=s.replace(old3,'''    "esbuild": "^0.27.2",
    "eslint-plugin-storybook": "10.4.0",
    "fast-glob": "3.3.3",''')
old4='''    "typescript": "^5.9.3",
    "vitest": "^4.0.17",
    "yaml": "2.9.0"
  }'''
assert s.count(old4)==1
s=s.replace(old4,'''    "typescript": "^5.9.3",
    "typescript-eslint": "^8.50.1",
    "vitest": "^4.0.17",
    "yaml": "2.9.0"
  }''')
d=json.loads(s)
assert set(d["dependencies"])=={"@semio-tech/process","@semio-tech/repo-lib","@semio-tech/schema-validator-ts"} and {"@eslint/js","eslint-plugin-storybook","typescript-eslint"}<=set(d["devDependencies"])
open(p,"w",encoding="utf8").write(s)
print("ok")
