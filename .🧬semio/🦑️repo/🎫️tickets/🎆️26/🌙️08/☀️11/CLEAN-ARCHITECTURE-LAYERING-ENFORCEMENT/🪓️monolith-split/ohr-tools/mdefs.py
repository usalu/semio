"""🔎 OHR method-definition probe (read only): one `git grep` builds the index of every `fn <name>` definition under the framework and hub trees; each non-stub E0599 method of a harvest TSV is then classified as defined in the renderer tree, defined elsewhere, or defined nowhere.
usage: mdefs.py <tsv>"""
import collections, json, os, re, subprocess, sys
CACHE = os.path.dirname(os.path.abspath(__file__)) + "/../🗑️generated/ohr-fndefs.json"
R = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer"
if os.path.exists(CACHE):
    idx = json.load(open(CACHE, encoding="utf-8"))
else:
    p = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-n", "-o", "-w", "-E", r"fn [A-Za-z_0-9]+", "--", "🧰️framework", "🌎️hub", "✏️s", "*.rs"], cwd="/Users/ueli/Documents/semio", capture_output=True, text=True)
    idx = collections.defaultdict(list)
    for l in p.stdout.split("\n"):
        m = re.match(r"^(.*?):(\d+):fn ([A-Za-z_0-9]+)$", l)
        if m:
            idx[m.group(3)].append(m.group(1))
    json.dump(idx, open(CACHE, "w", encoding="utf-8"))
rows = [l.rstrip("\n").split("\t") for l in open(sys.argv[1], encoding="utf-8")]
names = collections.defaultdict(list)
for r in rows:
    if r[0] != "E0599":
        continue
    m = re.search(r"no (?:method|associated function or constant|variant, associated function, or constant) named `([^`]+)` found for (?:struct|enum|mutable reference|reference|type|array|union) `([^`]+)`", r[5])
    if m:
        names[m.group(1)].append((r[1], r[2].replace("semio_framework_os_renderer_wgpu_", ""), re.sub(r"[a-z_0-9]+::", "", m.group(2))[-40:], r[3].split("/")[-2] + ":" + r[4]))
out = []
for n, uses in sorted(names.items()):
    files = idx.get(n, [])
    inr = [f for f in files if f.startswith(R)]
    out.append((n, len(uses), len(inr), len(files) - len(inr), uses[0], [f.split("/")[-2] for f in inr[:3]]))
for o in sorted(out, key=lambda x: (x[2] == 0, -x[1])):
    print(o)
