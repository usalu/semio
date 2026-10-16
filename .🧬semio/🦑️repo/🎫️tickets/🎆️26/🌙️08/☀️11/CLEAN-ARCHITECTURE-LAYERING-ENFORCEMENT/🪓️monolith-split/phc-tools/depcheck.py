#!/usr/bin/env python3
"""🧷️ Independent dependency check: every external crate path head used in a crate's files (regex over the file text, not the token scanner that generated the manifests) must be declared in that crate's manifest."""
import os, sys, re, json, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import state, plan_host, hcmod, regs_host
from carve import REPO

ws, model, roots = state.load()
hcmod.FS = ws
HEAD = re.compile(r'(?<![A-Za-z0-9_:])((?:semio|wasmtime|serde|tokio|bytes|thiserror|anyhow|futures|async)[A-Za-z0-9_]*)\s*::')
TOML = 'const t=Bun.TOML.parse(await Bun.file(process.argv[1]).text()); console.log(JSON.stringify(t))'
bad = 0
for ident, info in plan_host.CRATES.items():
    pkg = regs_host.pkg_dir_of(info['root'])
    m = json.loads(subprocess.run(['bun', '-e', TOML, pkg + '/Cargo.toml'], capture_output=True, text=True).stdout)
    pkgs = {}
    for tbl in ('dependencies', 'dev-dependencies'):
        for k, v in m.get(tbl, {}).items():
            name = v.get('package', k) if isinstance(v, dict) else k
            pkgs.setdefault(k.replace('-', '_'), set()).add(tbl)
            pkgs.setdefault(name.replace('-', '_'), set()).add(tbl)
    own = info['name'].replace('-', '_')
    files = {}
    for fm in hcmod.walk(model.roots[ident]):
        if getattr(fm, 'missing', False) or fm.inline or not fm.file:
            continue
        files[fm.file] = ws.read(fm.file)
    used_normal, used_any = {}, {}
    for p, text in files.items():
        is_test = '/🧪️tests/' in p or p.endswith('/tests.rs')
        body = re.sub(r'//[^\n]*', '', text)
        for h in set(HEAD.findall(body)):
            used_any.setdefault(h, set()).add(p)
            if not is_test:
                used_normal.setdefault(h, set()).add(p)
    externs = {e for e in re.findall(r'extern crate (\w+)(?: as (\w+))?;', ''.join(files.values())) for e in e if e}
    problems = []
    for h, ps in sorted(used_any.items()):
        if h == own or h in externs or h in ('semio', 'serde_json') and False:
            continue
        got = pkgs.get(h)
        if got is None:
            problems.append(f'undeclared {h} used in {len(ps)} file(s), e.g. {os.path.relpath(sorted(ps)[0], plan_host.H)}')
        elif h in used_normal and 'dependencies' not in got:
            problems.append(f'{h} used in non-test code but only a dev-dependency')
    print(info['name'], 'OK' if not problems else 'PROBLEMS')
    for pr in problems:
        print('   ', pr)
    bad += len(problems)
print('total problems', bad)
