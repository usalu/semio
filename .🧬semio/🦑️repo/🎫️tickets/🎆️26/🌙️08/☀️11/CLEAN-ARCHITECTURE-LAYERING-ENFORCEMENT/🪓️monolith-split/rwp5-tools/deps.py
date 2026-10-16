# 🕸️ Dependency graph of the WP5 nodes (methods and free items) of a Shell tree and the direction check: a node may only reference nodes that live in the same or a lower-ranked WP5 crate.
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, rsedit
from rsedit import Doc
from blank import blank

class Node:
    def __init__(self, kind, name, crate, slug, path, s, e, text):
        self.kind, self.name, self.crate, self.slug, self.path, self.s, self.e, self.text = kind, name, crate, slug, path, s, e, text
        self.refs = set()
    def key(self): return (self.kind, self.name, self.slug) if self.kind == "free" else (self.kind, self.name, self.slug)
    def __repr__(self): return f"{self.kind}:{self.name}@{self.crate}/{self.slug[:10]}"

def load(root):
    nodes = []
    for d, c, _ in cfg.CRATES:
        base = cfg.stg(root, d)
        if not os.path.isdir(base): continue
        for dp, dn, fn in os.walk(base):
            for f in sorted(fn):
                if f != "🦀️.rs": continue
                p = os.path.join(dp, f); slug = os.path.basename(dp)
                doc = Doc(open(p, encoding="utf-8").read())
                file_nodes = []
                impls = []
                for it in doc.items():
                    if it["kind"] == "impl" and not ("ShellState" in it["head"] or re.match(r"impl(?:<[^>]*>)?\s+\w+Ctx", it["head"])):
                        impls.append(it); continue
                    if it["kind"] == "impl" and "ShellState" in it["head"] and " for " not in it["head"] or (it["kind"] == "impl" and re.match(r"impl(?:<[^>]*>)?\s+\w+Ctx", it["head"])):
                        for m in doc.members(it):
                            nodes.append(Node("method", m["name"], c, slug, p, m["s"], m["e"], "\n".join(doc.lines[m["s"]:m["e"] + 1])))
                    elif it["kind"] in ("use", "mod") or not it["name"]:
                        continue
                    else:
                        nm = re.match(r"[A-Za-z_][A-Za-z_0-9]*", it["name"])
                        if it["kind"] == "impl": continue
                        if nm:
                            nd = Node("free", nm.group(0), c, slug, p, it["s"], it["e"], "\n".join(doc.lines[it["s"]:it["e"] + 1])); nodes.append(nd); file_nodes.append(nd)
                for it in impls:
                    hd = re.sub(r"<[^<>]*>", "", it["head"])
                    m = re.match(r"impl\s+(?:[\w:]+\s+for\s+)?&?(?:\w+::)*(\w+)", hd)
                    tname = m.group(1) if m else None
                    owner = next((x for x in file_nodes if x.name == tname), None)
                    if owner is not None: owner.text += "\n" + "\n".join(doc.lines[it["s"]:it["e"] + 1])
                    else: nodes.append(Node("impl", tname or "?", c, slug, p, it["s"], it["e"], "\n".join(doc.lines[it["s"]:it["e"] + 1])))
    return nodes

def link(nodes):
    methods = collections.defaultdict(list); free = collections.defaultdict(list)
    for n in nodes: (methods if n.kind == "method" else free)[n.name].append(n)
    for n in nodes:
        b = blank(n.text)
        for m in re.finditer(r"\b(?:self|Self|ShellState)\s*(?:\.|::)\s*([A-Za-z_][A-Za-z_0-9]*)", b):
            for t in methods.get(m.group(1), []): n.refs.add(t)
        for m in re.finditer(r"(?<![A-Za-z_0-9.])([A-Za-z_][A-Za-z_0-9]*)", b):
            for t in free.get(m.group(1), []):
                if t is not n: n.refs.add(t)
    return methods, free

def violations(nodes):
    out = []
    for n in nodes:
        for t in n.refs:
            if t is n: continue
            if cfg.RANK[t.crate] > cfg.RANK[n.crate]: out.append((n, t))
    return out

if __name__ == "__main__":
    root = sys.argv[1] if len(sys.argv) > 1 else cfg.NEW
    nodes = load(root); link(nodes)
    print(len(nodes), "nodes;", sum(len(n.refs) for n in nodes), "edges")
    v = violations(nodes)
    print("upward references:", len(v))
    for a, b in v[:60]: print("  ", a, "->", b)
