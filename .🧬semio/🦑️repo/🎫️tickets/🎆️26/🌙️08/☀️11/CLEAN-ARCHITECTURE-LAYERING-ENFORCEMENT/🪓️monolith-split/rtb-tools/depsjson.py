"""🔒️ Updates the `users` lists of 🔒️dependencies.json for the manifests this slice created or edited. DRY=1 only reports."""
import json, os, re
from lib import *

TARGET = "🔒️dependencies.json"


def external(text):
    out = set()
    sec = None
    for line in text.splitlines():
        m = re.match(r"^\[(.+)\]\s*$", line)
        if m:
            sec = m.group(1)
            continue
        if sec and sec.endswith("dependencies") and "=" in line and not line.lstrip().startswith("#"):
            key = line.split("=", 1)[0].strip().split(".")[0]
            if not re.match(r"^[A-Za-z0-9_\-]+$", key):
                continue
            if "path =" in line and "workspace" not in line:
                continue
            pk = re.search(r'package\s*=\s*"([^"]+)"', line)
            out.add(pk.group(1) if pk else key)
    return out


def main():
    full = REPO + "/" + TARGET
    t = read(full)
    d = json.loads(t)
    byname = {}
    for e in d["entries"]:
        if e["ecosystem"] == "rust":
            byname.setdefault(e["name"], []).append(e)
    log = []
    mans = [(s, T + "/" + f + "/📦️packages/🦀️rust/Cargo.toml") for s, f in crate_table().items()] + [("top", T + "/📦️packages/🦀️rust/Cargo.toml")]
    for suffix, mp in mans:
        snap = SNAP + "/E/🎯️targets/🧊️wgpu/" + os.path.relpath(mp, T)
        if os.path.exists(snap) and read(snap) == read(mp):
            continue
        r = os.path.relpath(mp, REPO)
        old = external(read(snap)) if os.path.exists(snap) else set()
        new = external(read(mp))
        for n in sorted(old - new):
            for e in byname.get(n, []):
                if r in e["users"]:
                    e["users"] = [u for u in e["users"] if u != r]
                    log.append(f"- {n} -/- {suffix}")
        for n in sorted(new - old):
            es = byname.get(n, [])
            if len(es) == 1:
                if r not in es[0]["users"]:
                    es[0]["users"] = sorted(es[0]["users"] + [r])
                    log.append(f"+ {n} <- {suffix}")
            else:
                log.append(f"? {n}: {len(es)} entries, skipped for {suffix}")
    print("\n".join(log))
    out = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    if os.environ.get("DRY") != "1" and out != t:
        backup(full)
        edit(full, out)
        print("written", len(log))


main()
