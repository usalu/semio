"""✅ Static validation of the staged WP8 output (before and after apply)."""
import sys, os, re, json, subprocess, collections, tempfile
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, testnames, mounts
FE = '️'
def rustfmt_ok(text):
    r = subprocess.run(['rustfmt', '--edition', '2021', '--emit', 'stdout', '--config', 'skip_children=true'], input=text, capture_output=True, text=True)
    return r.returncode == 0, r.stderr[-400:]
PATH_RX = re.compile(r'#\s*\[\s*path\s*=\s*"([^"]+)"\s*\]')
INC_RX = re.compile(r'include(?:_str|_bytes)?!\(\s*(concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*)?"([^"]+)"')
def manifest_dir(path):
    d = os.path.dirname(path)
    while d.startswith(L.R + '/') and len(d) > len(L.R):
        if os.path.isdir(d + '/📦' + FE + 'packages/🦀' + FE + 'rust') and (d + '/📦' + FE + 'packages/🦀' + FE + 'rust/Cargo.toml' in MANIFESTS or os.path.exists(d + '/📦' + FE + 'packages/🦀' + FE + 'rust/Cargo.toml')):
            return d + '/📦' + FE + 'packages/🦀' + FE + 'rust'
        d = os.path.dirname(d)
    return L.R + '/' + L.K_REL
MANIFESTS = set()
def check_paths(out, dele):
    exists = lambda p: (p in out) or (os.path.exists(p) and p not in dele)
    bad = []; longest = 0
    for p, t in out.items():
        if not p.endswith('.rs'): continue
        base = os.path.dirname(p)
        for m in PATH_RX.finditer(t):
            tgt = os.path.normpath(os.path.join(base, m.group(1)))
            if not exists(tgt): bad.append((L.rel(p), m.group(1)))
            longest = max(longest, len(L.rel(tgt).encode('utf8')))
        for m in INC_RX.finditer(t):
            if m.group(1):
                tgt = os.path.normpath(manifest_dir(p) + '/' + m.group(2).lstrip('/'))
                if not exists(tgt): bad.append((L.rel(p), m.group(2)))
                continue
            tgt = os.path.normpath(os.path.join(base, m.group(2)))
            if not exists(tgt): bad.append((L.rel(p), m.group(2)))
            longest = max(longest, len(L.rel(tgt).encode('utf8')))
    return bad, longest
def toml_ok(paths):
    tmp = L.GEN + '/okwp8-toml.json'
    r = subprocess.run(['bun', L.HERE + '/toml2json.ts', tmp] + paths, capture_output=True, text=True, cwd=L.R)
    return r.returncode == 0, r.stderr[-500:]
def region_balance(text):
    st = []; bad = 0
    for line in text.split('\n'):
        s = line.strip()
        if s.startswith('//#region '): st.append(s[10:])
        elif s.startswith('//#endregion '):
            if st and st[-1] == s[13:]: st.pop()
            else: bad += 1
    return bad + len(st)
def run(out, dele):
    res = collections.OrderedDict()
    fmt_fail = []; rg = []
    for p, t in out.items():
        if p.endswith('.rs'):
            ok, err = rustfmt_ok(t)
            if not ok: fmt_fail.append((L.rel(p), err))
            if region_balance(t): rg.append(L.rel(p))
    res['rustfmt_fail'] = fmt_fail; res['region_unbalanced'] = rg
    res['path_bad'], res['longest_path_bytes'] = check_paths(out, set(dele))
    tomls = [p for p in out if p.endswith('Cargo.toml')]
    # materialise toml in a temp dir to parse
    tdir = L.GEN + '/okwp8-stage-toml'; os.makedirs(tdir, exist_ok=True)
    tfiles = []
    for i, p in enumerate(tomls):
        f = tdir + '/%d.toml' % i; open(f, 'w', encoding='utf8').write(out[p]); tfiles.append(f)
    res['toml_ok'] = toml_ok(tfiles) if tfiles else (True, '')
    return res
