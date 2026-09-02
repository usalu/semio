import subprocess, os, re, json, sys

repo = "/Users/ueli/Documents/semio"
os.chdir(repo)

with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/43bfe996-fced-47cc-b279-32d897c6af08/scratchpad/b6-case-dirs.txt") as f:
    cases = [l.strip() for l in f if l.strip()]

URI_RE = re.compile(r'\b(shared|local|asset)://([^\s"\'`,;)\]]+)')
TAG_RE = re.compile(r'^@([a-zA-Z0-9-]+)', re.M)
RUST_SUBSET_RE = re.compile(r'standards::v([0-9_]+)::subsets::([a-zA-Z_0-9]+)')

results = []
for c in cases:
    idx = c.find("/🗿️artifacts/")
    prefix = c[:idx]
    rest = c[idx+len("/🗿️artifacts/"):]
    parts = rest.split("/")
    artifact_name = parts[0]
    plugin = prefix.split("/")[-1]
    artifact_dir = os.path.join(prefix, "🗿️artifacts", artifact_name)
    case_name = c.rstrip("/").split("/")[-1]

    files = []
    for root, dirs, fnames in os.walk(c):
        for fn in fnames:
            files.append(os.path.join(root, fn))

    feature_files = [f for f in files if f.endswith("🥒️.feature")]
    tags = set()
    uris = set()
    feature_text_all = ""
    for ff in feature_files:
        try:
            with open(ff, encoding="utf-8") as fh:
                txt = fh.read()
        except Exception as e:
            txt = ""
        feature_text_all += txt + "\n"
        for m in TAG_RE.finditer(txt):
            tags.add(m.group(1))
        for m in URI_RE.finditer(txt):
            uris.add(f"{m.group(1)}://{m.group(2)}")

    rust_subsets = set()
    adapter_files = [f for f in files if not f.endswith("🥒️.feature") and "/🧫️fixtures/" not in f]
    for af in adapter_files:
        try:
            with open(af, encoding="utf-8", errors="replace") as fh:
                atxt = fh.read()
        except Exception:
            atxt = ""
        for m in RUST_SUBSET_RE.finditer(atxt):
            rust_subsets.add(f"v{m.group(1)}::{m.group(2)}")

    # real subset list for this artifact
    subset_map = {}
    standards_dir = os.path.join(artifact_dir, "🏅️standards")
    if os.path.isdir(standards_dir):
        for ver in sorted(os.listdir(standards_dir)):
            verdir = os.path.join(standards_dir, ver)
            subsets_dir = os.path.join(verdir, "🪆️subsets")
            if os.path.isdir(subsets_dir):
                subset_map[ver] = sorted([s for s in os.listdir(subsets_dir) if os.path.isdir(os.path.join(subsets_dir, s))])

    results.append({
        "plugin": plugin,
        "artifact": artifact_name,
        "case": case_name,
        "case_dir": c,
        "tags": sorted(tags),
        "uris": sorted(uris),
        "rust_subsets": sorted(rust_subsets),
        "subset_map": subset_map,
        "files": [os.path.relpath(f, c) for f in files],
    })

with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/43bfe996-fced-47cc-b279-32d897c6af08/scratchpad/b6/census_raw.json", "w", encoding="utf-8") as f:
    json.dump(results, f, ensure_ascii=False, indent=1)

print(len(results), "cases processed")
