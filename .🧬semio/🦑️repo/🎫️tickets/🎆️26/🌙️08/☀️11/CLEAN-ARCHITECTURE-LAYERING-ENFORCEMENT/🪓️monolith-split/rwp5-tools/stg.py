# 🗂️ Staging model of WP5: every `impl ShellState` method and free item in the staging folders of the WP5 crates, with the call graph between use-case methods.
import os, re, sys, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import shellsrc as S
from blank import blank

STG = S.SH + "/📥️staging"
CRATES = [("🚶️session", "shell-session"), ("🪝️window-actions", "shell-window-actions"), ("🔀️actions", "shell-actions"), ("🖐️pointer", "shell-pointer"),
          ("🖍️view-paint", "shell-view-paint"), ("🍱️view-chrome", "shell-view-chrome"), ("🗨️view-overlays", "shell-view-overlays"), ("🪀️view-panels", "shell-view-panels")]
DIR = dict(CRATES)
RANK = {c: i for i, (_, c) in enumerate(CRATES)}

def files(crate_dir):
    out = []
    root = os.path.join(STG, crate_dir)
    if not os.path.isdir(root): return out
    for dp, dn, fn in os.walk(root):
        for f in sorted(fn):
            if f == "🦀️.rs": out.append(os.path.join(dp, f))
    return sorted(out)

def load():
    ms = []
    for d, c in CRATES:
        for p in files(d):
            raw = open(p, encoding="utf-8").read()
            for m in S.methods(raw):
                m["crate"] = c; m["path"] = p; m["slug"] = os.path.basename(os.path.dirname(p)); ms.append(m)
    return ms

if __name__ == "__main__":
    ms = load()
    names = collections.defaultdict(list)
    for m in ms: names[m["name"]].append(m)
    c = collections.Counter(m["crate"] for m in ms)
    print(c, len(ms))
    print("duplicate names", {n: [(x["crate"], x["slug"]) for x in v] for n, v in names.items() if len(v) > 1})
    edges = collections.Counter(); up = []
    for m in ms:
        b = blank(m["body"])
        for n in set(re.findall(r"\bself\s*\.\s*([A-Za-z_0-9]+)\s*(?:::<[^>]*>)?\(", b)) | set(re.findall(r"\b(?:Self|ShellState)\s*::\s*([A-Za-z_0-9]+)\s*(?:::<[^>]*>)?\(", b)):
            for t in names.get(n, []):
                if t["crate"] != m["crate"]:
                    edges[(m["crate"], t["crate"])] += 1
                    if RANK[t["crate"]] > RANK[m["crate"]]: up.append((m["crate"], m["name"], t["crate"], n))
    for k, v in sorted(edges.items(), key=lambda kv: (RANK[kv[0][0]], RANK[kv[0][1]])): print(k, v)
    print("UPWARD", len(up)); [print("  ", u) for u in up]
