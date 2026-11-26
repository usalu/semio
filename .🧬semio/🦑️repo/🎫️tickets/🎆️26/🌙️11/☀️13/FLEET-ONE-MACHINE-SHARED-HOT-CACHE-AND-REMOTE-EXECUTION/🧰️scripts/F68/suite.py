#!/usr/bin/env python3
"""F68: registers the typed fixture test file in the build suites."""
import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f68lib import *
f=LIB+"/🏗️build/🗂️suites/🔣️.json"
old='''        "../🧪️testing/🌐️browser/🧰️preparation/🧪️tests/🟦️.ts"
      ],
      "platforms": [
        "darwin",
        "linux",
        "win32"
      ]
    },
'''
new=old+'''    {
      "id": "typed-fixture",
      "budgetMs": 300000,
      "typecheck": "none",
      "files": [
        "../🧪️testing/🧾️typed-fixture/🧪️tests/🟦️.ts"
      ],
      "platforms": [
        "darwin",
        "linux",
        "win32"
      ]
    },
'''
got,held=take({"suites"},8)
print("held",held)
try:
    if not held:
        s=open(f,encoding="utf8").read()
        if "typed-fixture" not in s:print(edit(f,old,new))
finally:release(got)
