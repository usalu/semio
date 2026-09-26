#!/usr/bin/env python3
"""R8 session 12 one-off: rebuild the taxonomy's materialized-JCO matrix on THE live staging root.

The taxonomy described the retired second root `🧑‍💻dev/🔌️plugin-modules` + `🧑‍💻dev/🧩️extension-modules`
(last written 09-15) and the pre-codec interface roster. The live root is
`🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules` (`pluginModulesRoot("dev")`), one directory per staged
component, extensions included. Rebuilds, from the staged tree:
  fixedDirectoryContracts  dev-{plugin|extension}-interfaces-<slug>
  fixedFilenameContracts   dev-{plugin|extension}-component-<slug>-{js,declaration,wasm} + dev-jco-interface-<name>
  packageSourceDispositions for every js/declaration companion and every interface contract
  fixedDirectoryContractSets dev-jco-* (one set per distinct emitted-roster)
Existing ids, reasons and authorities are kept where the component/interface still exists.
Usage: python3 jco-matrix-rebuild.py [--apply]
"""
import json, os, re, sys, collections

REPO = "/Users/ueli/Documents/semio"
TAX = os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")
LIVE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules"
RETIRED = ("🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules", "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️extension-modules")
DEV = re.compile(r"^dev-(plugin|extension|jco)-")
SUFFIX = {"js": ".js", "declaration": ".d.ts", "wasm": ".core.wasm"}

apply = "--apply" in sys.argv
raw = open(TAX, encoding="utf8").read()
t = json.loads(raw)
fd, ff, psd, sets = t["fixedDirectoryContracts"], t["fixedFilenameContracts"], t["packageSourceDispositions"], t["fixedDirectoryContractSets"]

old_dir = {}
for key, row in fd.items():
    m = re.match(r"^dev-(plugin|extension)-interfaces-(.+)$", key)
    if m:
        old_dir[os.path.basename(os.path.dirname(row["pathPattern"]))] = (m.group(1), m.group(2), key)

live_root = os.path.join(REPO, LIVE)
components = sorted(d for d in os.listdir(live_root) if os.path.isdir(os.path.join(live_root, d, "interfaces")))
roster = {d: sorted(os.listdir(os.path.join(live_root, d, "interfaces"))) for d in components}

def slug(directory):
    return re.sub(r"^[^a-z0-9]+", "", directory)

new_fd, new_ff, new_psd = {}, {}, {}
kind_of = {}
for d in components:
    kind, s, key = old_dir.get(d, ("plugin", slug(d), f"dev-plugin-interfaces-{slug(d)}"))
    kind_of[d] = (kind, s)
    path = f"{LIVE}/{d}/interfaces"
    previous = fd.get(key, {})
    new_fd[key] = {
        "pathPattern": path,
        "authority": "Bytecode Alliance JCO",
        "reason": previous.get("reason", f"Exact emitted {s} component interface owner"),
        "configurability": "unconfigurable",
        "scope": {"kind": "exact-path", "path": path},
        "verification": "materialized JCO output boundaries: exact interface roster",
        "expires": None,
    }
    comps = sorted(n for n in os.listdir(os.path.join(live_root, d)) if "_component." in n)
    stem = comps[0].split("_component.")[0] + "_component"
    assert sorted(stem + x for x in SUFFIX.values()) == comps, (d, comps)
    for role, ext in SUFFIX.items():
        cid = f"dev-{kind}-component-{s}-{role}"
        before = ff.get(cid, {})
        new_ff[cid] = {
            "pathPattern": f"{LIVE}/{d}/{stem}{ext}",
            "authority": "Bytecode Alliance JCO",
            "reason": before.get("reason", {"js": f"Exact {s} JS companion paired with its declaration", "declaration": f"Exact {s} declaration paired with its JS companion", "wasm": f"Exact {s} core module URL emitted by JCO"}[role]),
            "configurability": "unconfigurable",
            "scope": {"kind": "path-pattern"},
            "verification": "materialized JCO component pairing",
            "expires": None,
        }
        if role != "wasm":
            new_psd[cid] = psd.get(cid) or {"contractKind": "fixed", "disposition": "adapter-source", "validator": "package-glue", "authority": f"JCO {s} {'companion without purity exemption' if role == 'js' else 'declaration pairing'}", "verification": "materialized JCO component pairing"}

presence = collections.defaultdict(list)
for d in components:
    for name in roster[d]:
        presence[name].append(d)
old_iface = {row["pathPattern"][3:]: key for key, row in ff.items() if key.startswith("dev-jco-interface-")}
all_key = "dev-jco-all-interfaces"
dir_key = {d: next(k for k, v in new_fd.items() if v["pathPattern"] == f"{LIVE}/{d}/interfaces") for d in components}
new_sets = {all_key: [dir_key[d] for d in components]}
set_for_members = {tuple(components): all_key}

def set_id(members, name):
    members = tuple(sorted(members))
    if members in set_for_members:
        return set_for_members[members]
    stem = re.sub(r"\.d\.ts$", "", name)
    base = stem.split("-")[1] if stem.startswith("wasi-") else stem.replace("semio-framework-", "")
    sid = f"dev-jco-{base}-interfaces"
    set_for_members[members] = sid
    new_sets[sid] = [dir_key[d] for d in components if d in members]
    return sid

for name in sorted(presence):
    key = old_iface.get(name) or "dev-jco-interface-" + re.sub(r"\.d\.ts$", "", name).replace("semio-framework-", "").replace("wasi-random-random", "random").replace("wasi-", "")
    before = ff.get(key, {})
    sid = set_id(presence[name], name)
    new_ff[key] = {
        "pathPattern": f"**/{name}",
        "authority": "JCO WIT interface emitter",
        "reason": before.get("reason", f"Exact {name} basename only where emitted"),
        "configurability": "unconfigurable",
        "scope": {"kind": "named-fixed-directory-contract-set", "fixedDirectoryContractSetId": sid},
        "verification": "materialized JCO interface filename boundaries",
        "expires": None,
    }
    new_psd[key] = psd.get(key) or {"contractKind": "fixed", "disposition": "adapter-source", "validator": "package-glue", "authority": "JCO WIT declaration identity without purity exemption", "verification": "materialized JCO interface filename boundaries"}

def splice(section, fresh):
    out, placed = {}, False
    for key, value in section.items():
        if DEV.match(key):
            if not placed:
                out.update(fresh)
                placed = True
            continue
        out[key] = value
    if not placed:
        out.update(fresh)
    return out

counts = {
    "components": len(components),
    "interfaceContracts": sum(1 for k in new_ff if k.startswith("dev-jco-interface-")),
    "companionContracts": sum(1 for k in new_ff if not k.startswith("dev-jco-interface-")),
    "interfaceFiles": sum(len(r) for r in roster.values()),
    "sets": {k: len(v) for k, v in new_sets.items()},
    "removedInterfaceContracts": sorted(set(k for k in ff if k.startswith("dev-jco-interface-")) - set(new_ff)),
    "addedInterfaceContracts": sorted(set(k for k in new_ff if k.startswith("dev-jco-interface-")) - set(ff)),
    "addedComponents": sorted(d for d in components if d not in old_dir),
    "removedSets": sorted(set(k for k in sets if k.startswith("dev-jco")) - set(new_sets)),
}
print(json.dumps(counts, ensure_ascii=False, indent=1))

t["fixedDirectoryContracts"] = splice(fd, new_fd)
t["fixedFilenameContracts"] = splice(ff, new_ff)
t["packageSourceDispositions"] = splice(psd, new_psd)
t["fixedDirectoryContractSets"] = splice(sets, new_sets)
out = json.dumps(t, ensure_ascii=False, indent=2) + "\n"
assert not any(r in out for r in RETIRED if r.endswith("modules")), "retired root still referenced"
if apply:
    if open(TAX, encoding="utf8").read() != raw:
        sys.exit("taxonomy changed underneath; re-run")
    open(TAX, "w", encoding="utf8").write(out)
    print("applied", len(raw), "->", len(out))
else:
    print("dry run", len(raw), "->", len(out))
