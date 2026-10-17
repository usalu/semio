"""🥣 Generate a companion root that mounts per-module unit tests under wrapper modules named like the module they test.
usage: mkroot.py <spec.json>
spec: {crate_dir, lib_name, companion_dir, doc, preamble:[...], modules:[{name, emoji, lib_file, tests, pub_tests:true, extra:[...]}], tail:[...]}
Root: `#[path = "<emoji>️<name>/🦀️.rs"] pub mod <name>;` per module. Wrapper file: `pub use <lib>::<name>::*;` + the private `use` lines of the module that the test mentions + `#[path] [pub] mod tests;`."""
import collections, json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
spec = json.load(open(sys.argv[1], encoding="utf-8"))
R = "/Users/ueli/Documents/semio/"
cdir = R + spec["companion_dir"]
root = [spec["doc"], "#![cfg(test)]", ""] + spec.get("preamble", [])
for m in spec["modules"]:
    libf = R + spec["crate_dir"] + "/" + m["lib_file"]
    tests_spec = m["tests"] if isinstance(m["tests"], list) else [["tests", m["tests"]]]
    tf = R + spec["crate_dir"] + "/" + tests_spec[0][1]
    folder = f"{m['emoji']}\ufe0f{m.get('folder', m['name'])}"
    parent = next((p for p in spec["modules"] if p["name"] == m.get("parent")), None) if m.get("parent") else None
    pfolder = f"{parent['emoji']}\ufe0f{parent.get('folder', parent['name'])}" if parent else None
    wdir = cdir + "/" + (pfolder + "/" if pfolder else "") + folder
    imps = subprocess.run(["python3", HERE + "/useimports.py", libf, tf], capture_output=True, text=True).stdout.strip().split("\n")
    sp_ = m.get("super_path", "crate")
    imps = [i.replace("use super::", f"use {sp_}::", 1) for i in imps if i and not i.startswith("use self")]
    imps = [i for i in imps if i != f"use {sp_}::*;"]
    groups = collections.defaultdict(list)
    for i in imps:
        head, _, name = i[4:-1].rpartition("::")
        groups[head].append(name)
    imps = [f"use {h}::{{{', '.join(sorted(n))}}};" if len(n) > 1 else f"use {h}::{n[0]};" for h, n in sorted(groups.items())]
    rel = os.path.relpath(tf, wdir)
    mount_lines = []
    for modname, trel in tests_spec:
        r_ = os.path.relpath(R + spec["crate_dir"] + "/" + trel, wdir)
        mount_lines += ["", (f'include!("{r_}");' if m.get("include") else f'#[path = "{r_}"]'), ("" if m.get("include") else f"{'pub ' if m.get('pub_tests') else ''}mod {modname};")]
    body = [f"//! {m['emoji']}\ufe0f {m['doc']}", f"pub use {m.get('glob', spec['lib_name'] + '::' + m['name'] + '::*')};"] + imps + m.get("extra", []) + mount_lines
    os.makedirs(wdir, exist_ok=True)
    open(wdir + "/🦀️.rs", "w", encoding="utf-8").write("\n".join(l for l in body if l is not None).replace("\n\n\n", "\n\n") + "\n")
    if parent:
        with open(cdir + "/" + pfolder + "/🦀️.rs", "a", encoding="utf-8") as pf:
            pf.write(f'\n#[path = "{folder}/🦀️.rs"]\npub mod {m["name"]};\n')
    else:
        root += [f'#[path = "{folder}/🦀️.rs"]', f"pub mod {m['name']};", ""]
root += spec.get("tail", [])
text = "\n".join(root).rstrip("\n") + "\n"
open(cdir + "/🦀️.rs", "w", encoding="utf-8").write(text)
print(text)
