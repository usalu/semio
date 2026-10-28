"""📋 Prints the existing live files batch `split` edits (one relative path per line), read from the constants of edit_split.py."""
import os, re
HERE = os.path.dirname(os.path.abspath(__file__))
source = open(os.path.join(HERE, "edit_split.py"), encoding="utf8").read()
consts = dict(re.findall(r'^(\w+) = (.+)$', source, re.M))
env = {}
for name, expr in consts.items():
    try: env[name] = eval(expr, {}, env)
    except Exception: pass
for call in re.findall(r'edit_(?:text|json)\(root, ([A-Z][^,]+),', source):
    print(eval(call, {}, env))
