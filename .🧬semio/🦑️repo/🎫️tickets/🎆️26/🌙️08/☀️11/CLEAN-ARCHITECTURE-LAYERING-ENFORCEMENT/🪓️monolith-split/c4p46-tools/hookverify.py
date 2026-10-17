"""🔎️ Equivalence of the REPL hook split: the original component body (before the final `return`) equals the concatenation of the hook bodies, and the JSX return is verbatim in the parent.

usage: hookverify.py <pre-split 🟦️.tsx> <parent 🟦️.tsx> <hook 🟦️.ts>...   (hooks in call order)
Whitespace is normalised; the scaffolding (signature, `const { .. } = core;` prelude, `return { .. };` bag, region markers, the exported type) is removed from the hooks.
"""
import re, sys

def norm(s):
    return re.sub(r"\s+", " ", s).strip()

orig_path, parent_path, *hooks = sys.argv[1:]
orig = open(orig_path, encoding="utf-8").read()
start = orig.index("}: InteractionReplProps): ReactNode {") + len("}: InteractionReplProps): ReactNode {")
ret = orig.index("\n  return (\n    <div className={fillHost")
end = orig.index("\n}\n", ret)
body = orig[start:ret]
jsx = orig[ret:end]
parts = []
for h in hooks:
    t = open(h, encoding="utf-8").read()
    m = re.search(r"^export function use\w+\((.*?)\) \{\n", t, re.S | re.M)
    a = m.end()
    pre = re.match(r"  const \{[^}]*\} = core;\n", t[a:])
    if pre:
        a += pre.end()
    b = t.rindex("\n  return {")
    parts.append(t[a:b])
joined = "".join(parts)
print("original body statements chars:", len(norm(body)), "hooks concatenated:", len(norm(joined)))
print("BODY EQUAL" if norm(body) == norm(joined) else "BODY DIFFERS")
parent = open(parent_path, encoding="utf-8").read()
print("JSX VERBATIM" if jsx.strip() in parent else "JSX MISSING")
