#!/usr/bin/env python3
"""🧭 Second, independent reading of GitHub's context-availability table (regex over the expression text, a different method than check.ts which parses). Reads the table from https://docs.github.com/en/actions/reference/workflows-and-actions/contexts#context-availability. usage: python3 ctxmodel.py <workflow.yml>...  (needs PyYAML; exit 3 when missing)."""
import re, sys
try: import yaml
except ImportError as e: print("ctxmodel unavailable:", e); sys.exit(3)
J = {"github", "needs", "strategy", "matrix", "vars", "inputs"}
S = J | {"job", "runner", "env", "steps"}
TABLE = {
    "name": {"github", "inputs", "vars"}, "concurrency": {"github", "inputs", "vars"}, "env": {"github", "secrets", "inputs", "vars"},
    "jobs.*.name": J, "jobs.*.runs-on": J, "jobs.*.timeout-minutes": J, "jobs.*.if": {"github", "needs", "vars", "inputs"},
    "jobs.*.strategy": {"github", "needs", "vars", "inputs"}, "jobs.*.env": J | {"secrets"}, "jobs.*.outputs": S | {"secrets"},
    "jobs.*.steps.*": S | {"secrets"}, "jobs.*.steps.*.if": S,
}
def spot(path):
    p = ".".join("*" if re.fullmatch(r"\d+", x) else x for x in path)
    p = re.sub(r"^jobs\.[^.]+", "jobs.*", p)
    for n in ("jobs.*.steps.*.if", "jobs.*.steps.*", "jobs.*.outputs", "jobs.*.env", "jobs.*.strategy", "jobs.*.if", "jobs.*.timeout-minutes", "jobs.*.runs-on", "jobs.*.name", "env", "concurrency", "name"):
        if p == n or p.startswith(n + "."): return n
def walk(node, path, out):
    if isinstance(node, dict):
        for k, v in node.items(): walk(v, path + [str(k)], out)
    elif isinstance(node, list):
        for i, v in enumerate(node): walk(v, path + [str(i)], out)
    elif isinstance(node, str):
        n = spot(path)
        if n is None: return
        if n.endswith(".if") and "${{" not in node: node = "${{ %s }}" % node
        for expr in re.findall(r"\$\{\{(.*?)\}\}", node, re.S):
            text = re.sub(r"'(?:[^']|'')*'", "''", expr)
            for m in re.finditer(r"(?<![\w.\-])(github|needs|strategy|matrix|job|runner|env|vars|secrets|steps|inputs)\b(?!\s*\()", text):
                if m.group(1) not in TABLE[n]: out.append(("/".join(path), m.group(1), expr.strip()))
bad = 0
for f in sys.argv[1:]:
    doc = yaml.safe_load(open(f, encoding="utf8"))
    if True in doc: doc["on"] = doc.pop(True)
    out = []; walk(doc, [], out)
    for p, c, e in out: print("%s: %s: context '%s' not available in %s" % (f, p, c, e)); bad += 1
print("%d finding(s)" % bad); sys.exit(1 if bad else 0)
