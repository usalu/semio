import json, os, re, shutil, sys

repo = "/Users/ueli/Documents/semio"
os.chdir(repo)

d = json.load(open("/private/tmp/claude-501/-Users-ueli-Documents-semio/43bfe996-fced-47cc-b279-32d897c6af08/scratchpad/b6/census_raw.json", encoding="utf-8"))
by_key = {}
for r in d:
    key = (r['plugin'], r['artifact'], r['case'])
    by_key[key] = r

def artifact_dir_for(r):
    idx = r['case_dir'].find("/🗿️artifacts/")
    prefix = r['case_dir'][:idx]
    return f"{prefix}/🗿️artifacts/{r['artifact']}"

ASSET_PREFIX_RE = re.compile(r'asset://🏅️standards/([^/]+)/🪆️subsets/([^/]+)/')

def relocate(plugin, artifact, case, ver, subset, dry=True, log=None):
    key = (plugin, artifact, case)
    r = by_key[key]
    artifact_dir = artifact_dir_for(r)
    src = r['case_dir']
    dest_owner = f"{artifact_dir}/🏅️standards/{ver}/🪆️subsets/{subset}"
    dest = f"{dest_owner}/🧪️tests/{case}"
    msg = f"RELOCATE {src} -> {dest}"
    if log is not None: log.append(msg)
    if not dry:
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        if os.path.exists(dest):
            raise RuntimeError(f"dest exists: {dest}")
        shutil.move(src, dest)
        # rewrite asset:// URIs in feature + adapters that are now owned by dest_owner
        for root, dirs, files in os.walk(dest):
            for fn in files:
                if fn.endswith(('🥒️.feature', '🦀️.rs', '🐍️.py', '🟦️.ts')):
                    fp = os.path.join(root, fn)
                    with open(fp, encoding='utf-8') as fh:
                        txt = fh.read()
                    def repl(m):
                        uri_ver, uri_subset = m.group(1), m.group(2)
                        if uri_ver == ver and uri_subset == subset:
                            return "asset://"
                        return m.group(0)
                    new_txt = ASSET_PREFIX_RE.sub(repl, txt)
                    if new_txt != txt:
                        with open(fp, 'w', encoding='utf-8') as fh:
                            fh.write(new_txt)
                        if log is not None: log.append(f"  rewrote asset:// in {fp}")
    return dest

if __name__ == "__main__":
    print("module loaded")
