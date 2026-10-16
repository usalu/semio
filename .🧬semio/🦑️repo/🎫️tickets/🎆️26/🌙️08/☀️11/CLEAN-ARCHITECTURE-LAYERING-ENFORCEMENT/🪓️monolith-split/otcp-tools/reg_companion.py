"""Register one companion crate. usage: reg_companion.py <json spec>
spec: {crate, project, id, dir (repo-rel companion package dir), emoji_name (folder name for taxonomy), order_index, deps [lock names], p_lock {name, remove []}}"""
import json, os, sys
sys.path.insert(0, os.path.dirname(__file__))
import register as R
import edit as E

spec = json.load(open(sys.argv[1], encoding="utf-8"))
state_path = E.F + "/🗑️generated/otcp-work/reg-state.json"
state = json.load(open(state_path)) if os.path.exists(state_path) else {"anchor": "semio-framework-os-store-durable-group-tests 🧪️test 🚀️release"}
base = 97001 + 2 * spec["order_index"]
pkg = spec["dir"]
fw = spec.get("framework", False)
R.add_member("🧰️framework/Cargo.toml" if fw else "Cargo.toml", pkg[len("🧰️framework/"):] if fw else pkg)
R.add_owner(spec["project"], pkg, pkg + "/📋️project.json")
for f in (".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"):
    last = R.add_routes(f, state["anchor"], spec["crate"], spec["project"], (base, base, base))
state["anchor"] = last
json.dump(state, open(state_path, "w"))
R.add_lock("🧰️framework/Cargo.lock" if fw else "Cargo.lock", spec["crate"], spec["deps"])
if spec.get("p_lock"):
    for lock in (["Cargo.lock", "🧰️framework/Cargo.lock"] if fw or spec.get("both_locks") else ["Cargo.lock"]):
        t = E.read(lock)
        name = spec["p_lock"]["name"]
        key = f'[[package]]\nname = "{name}"\n'
        if key not in t:
            continue
        i = t.index(key)
        j = t.index("\n\n", i)
        block = t[i:j]
        nb = block
        for r in spec["p_lock"].get("remove", []):
            if f' "{r}",\n' in nb + "\n":
                nb = nb.replace(f' "{r}",\n', "", 1)
        for a_ in spec["p_lock"].get("add", []):
            if f' "{a_}",' not in nb:
                lines = nb.split("\n")
                k = lines.index("dependencies = [")
                end = lines.index("]", k)
                deps = lines[k + 1:end] + [f' "{a_}",']
                deps.sort()
                lines[k + 1:end] = deps
                nb = "\n".join(lines)
        if nb != block:
            R.atomic_write(lock, t[:i] + nb + t[j:])
added = R.add_taxonomy(spec.get("taxonomy", []))
print("registered", spec["crate"], "taxonomy added", added)
