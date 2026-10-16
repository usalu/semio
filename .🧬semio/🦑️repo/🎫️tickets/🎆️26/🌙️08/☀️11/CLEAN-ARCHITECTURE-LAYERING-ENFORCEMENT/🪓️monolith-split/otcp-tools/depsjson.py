"""Update the external-dependency lock `🔒️dependencies.json` users lists for the manifests this slice created or edited. DRY=1 reports only."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import edit as E, register as R


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


touched = [l.rstrip("\n").split("\t") for l in open(E.TOUCHED, encoding="utf-8")]
created = sorted({r for k, r in touched if k == "create" and r.endswith("Cargo.toml")})
edited = sorted({r for k, r in touched if k == "edit" and r.endswith("Cargo.toml") and os.path.exists(os.path.join(E.PRE, r))})
t = E.read("🔒️dependencies.json")
d = json.loads(t)
byname = {}
for e in d["entries"]:
    if e["ecosystem"] == "rust":
        byname.setdefault(e["name"], []).append(e)
log = []
for m in created:
    for n in sorted(external(E.read(m))):
        es = byname.get(n, [])
        if len(es) == 1:
            if m not in es[0]["users"]:
                es[0]["users"] = sorted(es[0]["users"] + [m])
                log.append(f"+ {n} <- {m.split('/')[-4]}/..{m[-30:]}")
        else:
            log.append(f"? {n}: {len(es)} entries, skipped for {m[-60:]}")
for m in edited:
    old = external(open(os.path.join(E.PRE, m), encoding="utf-8").read())
    new = external(E.read(m))
    for n in sorted(old - new):
        for e in byname.get(n, []):
            if m in e["users"]:
                e["users"] = [u for u in e["users"] if u != m]
                log.append(f"- {n} -/- {m[-60:]}")
    for n in sorted(new - old):
        es = byname.get(n, [])
        if len(es) == 1 and m not in es[0]["users"]:
            es[0]["users"] = sorted(es[0]["users"] + [m])
            log.append(f"+ {n} <- {m[-60:]}")
print("\n".join(log))
out = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
if os.environ.get("DRY") != "1" and out != t:
    R.atomic_write("🔒️dependencies.json", out)
    print("written")
