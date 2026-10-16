import os, sys, re, json, shutil, subprocess, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cfg import *
def build_asm(work, census_json, asm, extra_roots=None):
    res = json.load(open(census_json))
    owner = {}
    for r in res:
        if 'list' not in r: continue
        for p, kind, b, l in r['list']:
            owner.setdefault(p, r['id'])
    pkgs = {c['id']: c['pkg'].replace('-', '_') for c in CLUSTERS.values()}
    if extra_roots: pkgs.update(extra_roots)
    names = sorted(pkgs.values(), key=len, reverse=True)
    ext_re = re.compile(r'(?<![A-Za-z0-9_:])(' + '|'.join(names) + r')(?=\s*::)')
    shutil.rmtree(asm, ignore_errors=True)
    for p, c in owner.items():
        src = os.path.join(work, p); dst = os.path.join(asm, p)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        if p.endswith('.rs'):
            t = open(src, encoding='utf-8').read()
            pk = pkgs[c]
            t = re.sub(r'\$crate::', f'$crate::{pk}::', t)
            t = re.sub(r'(?<![A-Za-z0-9_$])crate::', f'crate::{pk}::', t)
            t = ext_re.sub(lambda m: 'crate::' + m.group(1), t)
            open(dst, 'w', encoding='utf-8').write(t)
        else:
            shutil.copy(src, dst)
    rootdir = os.path.join(asm, '🧰️framework/🔨️modules/🛂️asm-root')
    os.makedirs(rootdir, exist_ok=True)
    lines = []
    for cid, c in CLUSTERS.items():
        rel = os.path.relpath(os.path.join(asm, c['root']), rootdir)
        lines.append(f'#[path = "{rel}"]\npub mod {pkgs[cid]};')
    open(os.path.join(rootdir, '🦀️.rs'), 'w', encoding='utf-8').write('\n'.join(lines) + '\n')
    return os.path.join(rootdir, '🦀️.rs')
def run_rustc(root, args, cwd=None):
    cmd = ['rustc', '+nightly-2026-07-20', '--edition', '2021', '--crate-type', 'lib', '--crate-name', 'asm', '--emit=metadata', '-o', '/dev/null', '--error-format=json', '--json=diagnostic-short', '--cap-lints', 'allow'] + args + [root]
    p = subprocess.run(cmd, cwd=cwd or os.path.dirname(root), capture_output=True, text=True)
    errs = []
    for line in p.stderr.split('\n'):
        if not line.startswith('{'): continue
        try: d = json.loads(line)
        except Exception: continue
        if d.get('level') != 'error': continue
        sp = [s for s in d.get('spans', []) if s.get('is_primary')]
        s0 = sp[0] if sp else {}
        errs.append(dict(code=(d.get('code') or {}).get('code'), msg=d['message'], file=s0.get('file_name', ''), line=s0.get('line_start', 0), text=(s0.get('text') or [{}])[0].get('text', '').strip()[:160] if s0.get('text') else ''))
    return errs
EXTN = "semio_framework_value|semio_framework_value_derive|serde|serde_json|semio_framework_pack_json|semio_framework_ui_locale|semio_framework_ui_contract|semio_framework_job|semio_framework_diagnostic|ui_wgpu|semio_framework_actor|semio_framework_trace|semio_framework_schema_registry|replication|semio_framework_tool_run|semio_framework_artifact_reference|semio_framework_hash|semio_framework_io_base64|semio_framework_async_macros|semio_framework_async|pack|base64|semio_framework_dsl|semio_framework_io_schema|semio_framework_mesh_engine|semio_framework_schema|semio_framework_io_sqlite_snapshot"
EXT_MSG = re.compile(r"((cannot find module or crate|unresolved import) `(" + EXTN + r")`|cannot find attribute|cannot find derive macro|cannot find macro)")
def classify(errs):
    ext = [e for e in errs if EXT_MSG.search(e['msg'])]
    non = [e for e in errs if not EXT_MSG.search(e['msg'])]
    return ext, non
if __name__ == '__main__':
    work, census, asm = sys.argv[1:4]
    root = build_asm(work, census, asm)
    for flags in ([], ['--test'], ['--cfg', 'test']):
        errs = run_rustc(root, flags)
        ext, non = classify(errs)
        print(flags, len(errs), 'non-extern', len(non))
        cnt = collections.Counter((e['code'], e['msg'][:110]) for e in non)
        for k, v in cnt.most_common(40): print('  ', v, k)
