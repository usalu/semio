import os, sys
os.environ["SF_SRC"] = os.environ.get("SF_LIVE", "/Users/ueli/Documents/semio")
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

b = Batch("law", "the policy literal law: a module's policy constants live in its schema file, its sources hold none of their values as a literal, EN+DE descriptions, Ajv strict, a Python lexer as the independent oracle")
for name in ("🟦️.ts", "🐍️.py", "🧬️schema/🔣️.json"):
    b.put(LAW + name, open(os.path.join(LAW_SRC, name), encoding="utf8").read())
b.write(tests=["./" + LAW + "🟦️.ts"])
