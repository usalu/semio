"""🔏 OHR privacy grouping (read only): reads GEN/ohr-<family>-<label>-result.json, groups the non-stub E0616/E0624/E0603/E0451/E0423 rows by (code, owner item, member) and prints the access crates and lines.
usage: priv.py <family> <label>..."""
import collections, json, os, re, sys
GEN = os.path.dirname(os.path.dirname(os.path.abspath(__file__))) + "/🗑️generated"
fam = sys.argv[1]
groups = collections.defaultdict(lambda: collections.defaultdict(set))
for label in sys.argv[2:]:
    r = json.load(open(f"{GEN}/ohr-{fam}-{label}-result.json", encoding="utf-8"))
    for cfg, e in r["summary"].items():
        for d in e.get("new_list", []):
            if d["stub"] or d["code"] not in ("E0616", "E0624", "E0603", "E0451", "E0423"):
                continue
            m = re.match(r"^__m_([A-Za-z0-9_]+)/(.*)$", d["file"])
            crate = (m.group(1) if m else "?").replace("semio_framework_os_renderer_wgpu_", "")
            msg = re.sub(r"__m_[A-Za-z0-9_]+::", "", d["msg"])
            msg = re.sub(r"[a-z_0-9]+::", "", msg)
            groups[(d["code"], msg)][crate].add(f"{cfg}:{os.path.basename(os.path.dirname(m.group(2))) if m else d['file']}:{d['line']}")
for (code, msg), by in sorted(groups.items()):
    print(code, msg)
    for crate, sites in sorted(by.items()):
        print("    ", crate, len(sites), sorted(sites)[:3])
