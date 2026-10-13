#!/usr/bin/env python3
"""🧾️ Wave 3: every workspace whose members name a semio crate through `{ workspace = true }` defines that crate."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from ownership import CRATE, crate_roots

roots = crate_roots()
PKG = {name: (os.path.dirname(roots[k]) + "/📦️packages/🦀️rust" if k != "agg" else SEMIO + "/📦️packages/🦀️rust") for k, name in CRATE.items()}
NAMES_ = set(CRATE.values())
ms = subprocess.run(["rg", "-l", r"semio-s-artifact-stdio-semio", "-g", "Cargo.toml", "-g", "!.🧬semio/**", "-g", "!**/target/**", REPO], capture_output=True, text=True).stdout.split()


def workspace_root(manifest):
    text = read(manifest)
    m = re.search(r'(?m)^workspace = "([^"]+)"', text)
    if m:
        return os.path.normpath(os.path.join(os.path.dirname(manifest), m.group(1), "Cargo.toml"))
    d = os.path.dirname(manifest)
    while d.startswith(REPO):
        c = os.path.join(d, "Cargo.toml")
        if os.path.exists(c) and re.search(r"(?m)^\[workspace\]", read(c)):
            return c
        d = os.path.dirname(d)
    return None


needs = {}
for m in ms:
    if m == SEMIO + "/Cargo.toml" or os.path.dirname(m) in PKG.values():
        continue
    for name in re.findall(r"(?m)^(semio-s-artifact-stdio-semio(?:-[a-z0-9-]+)?) = \{ workspace = true", read(m)):
        if name in NAMES_:
            needs.setdefault(workspace_root(m), set()).add(name)
added = 0
for ws, need in needs.items():
    text = read(ws)
    defined = set(re.findall(r"(?m)^\[workspace\.dependencies\.(semio-s-artifact-stdio-semio[a-z0-9-]*)\]", text)) | set(re.findall(r"(?m)^(semio-s-artifact-stdio-semio[a-z0-9-]*) = \{ ?path", text))
    missing = sorted(need - defined)
    if not missing:
        continue
    table = re.search(r'(?m)^\[workspace\.dependencies\.semio-s-artifact-stdio-semio\]\npath = "[^"]+"\n', text)
    inline = re.search(r'(?m)^semio-s-artifact-stdio-semio = \{ ?path = "[^"]+"[^}\n]*\}[ \t]*\n', text)
    anchor = re.search(r"(?m)^\[workspace\.dependencies\]\n", text)
    rel = lambda n: os.path.relpath(PKG[n], os.path.dirname(ws))
    if table or not (inline or anchor):
        block = "".join('[workspace.dependencies.%s]\npath = "%s"\n\n' % (n, rel(n)) for n in missing)
        at = table.end() if table else len(text)
        text = text[:at] + "\n" + block.rstrip("\n") + "\n" + text[at:]
    else:
        at = inline.end() if inline else anchor.end()
        text = text[:at] + "".join('%s = { path = "%s" }\n' % (n, rel(n)) for n in missing) + text[at:]
    write(ws, text)
    added += len(missing)
    print(os.path.relpath(ws, REPO), len(missing))
print("added", added, "definitions in", len(needs), "workspaces checked")
