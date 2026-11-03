"""🔧 `read_<enum>(text(..)?)` -> `read_<enum>(&text(..)?)` where read_<enum> takes &str and the local `text` helper returns String."""
import re, sys
f = sys.argv[1]
s = open(f, encoding="utf-8").read()
sig = dict(re.findall(r'fn (read_\w+)\s*\(\s*\w+\s*:\s*([^,)]*)\)', s))
n = 0
def sub(m):
    global n
    name = m.group(1)
    if sig.get(name, "").replace(" ", "") != "&str": return m.group(0)
    n += 1
    return f"{name}(&text("
s = re.sub(r'\b(read_\w+)\(text\(', sub, s)
if "--write" in sys.argv: open(f, "w", encoding="utf-8").write(s)
print(f, n)
