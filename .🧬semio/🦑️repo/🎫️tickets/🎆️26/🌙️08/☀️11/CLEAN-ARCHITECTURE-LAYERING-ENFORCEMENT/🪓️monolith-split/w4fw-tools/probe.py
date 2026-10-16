import os, sys, re, json, shutil, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from asm import *
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/"
def probe_lines(prefix):
    d = json.load(open(S + 'consumer-paths.json'))
    paths = sorted(k for k in d if '::' in k)
    lines = []
    for p in paths:
        lines.append((p, 'use ' + p.replace('semio_framework', prefix, 1) + ';'))
    return lines
def run_probe(root, flags, lines, probe_file_text_writer):
    probe_file_text_writer([l for _, l in lines])
    errs = run_rustc(root, flags)
    ext, non = classify(errs)
    bad = {}
    for e in non:
        if e['file'].endswith('probe/🦀️.rs') or '/probe/' in e['file']:
            idx = e['line'] - 1
            if 0 <= idx < len(lines): bad[lines[idx][0]] = e['msg'][:120]
    return bad, non
def old_probe(base):
    work = S + 'sb/oldprobe'
    shutil.rmtree(work, ignore_errors=True)
    shutil.copytree(base, work)
    root = work + '/🧰️framework/📦️packages/🦀️rust/🦀️.rs'
    t = open(root, encoding='utf-8').read()
    os.makedirs(work + '/🧰️framework/📦️packages/🦀️rust/probe', exist_ok=True)
    t += '\n#[allow(unused_imports)]\n#[path = "probe/🦀️.rs"]\nmod probe;\n'
    open(root, 'w', encoding='utf-8').write(t)
    lines = probe_lines('crate')
    def w(ls): open(work + '/🧰️framework/📦️packages/🦀️rust/probe/🦀️.rs', 'w', encoding='utf-8').write('\n'.join(ls) + '\n')
    return run_probe(root, [], lines, w)
def new_probe(work, census):
    asm = S + 'sb/newprobe'
    root = build_asm(work, census, asm, extra_roots={'__facade': 'semio_framework'})
    # add facade module: copy facade root with the same rewrites
    pk = 'semio_framework'
    names = sorted([c['pkg'].replace('-', '_') for c in CLUSTERS.values()], key=len, reverse=True)
    ext_re = re.compile(r'(?<![A-Za-z0-9_:])(' + '|'.join(names) + r')(?=\s*::)')
    t = open(work + '/🧰️framework/📦️packages/🦀️rust/🦀️.rs', encoding='utf-8').read()
    t = re.sub(r'(?<![A-Za-z0-9_$])crate::', 'crate::semio_framework::', t)
    t = ext_re.sub(lambda m: 'crate::' + m.group(1), t)
    fdir = asm + '/🧰️framework/📦️packages/🦀️rust'
    os.makedirs(fdir + '/probe', exist_ok=True)
    open(fdir + '/🦀️.rs', 'w', encoding='utf-8').write(t)
    rootdir = os.path.dirname(root)
    rel = os.path.relpath(fdir + '/🦀️.rs', rootdir)
    # facade test mounts reference modules relative to the facade dir; copy those files too
    for rp in ('🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs',):
        src = work + '/' + rp
        if os.path.exists(src):
            os.makedirs(os.path.dirname(asm + '/' + rp), exist_ok=True); shutil.copy(src, asm + '/' + rp)
    rt = open(root, encoding='utf-8').read()
    rt += f'\n#[path = "{rel}"]\npub mod semio_framework;\n#[allow(unused_imports)]\n#[path = "{os.path.relpath(fdir + "/probe/🦀️.rs", rootdir)}"]\nmod probe;\n'
    open(root, 'w', encoding='utf-8').write(rt)
    lines = probe_lines('crate::semio_framework')
    def w(ls): open(fdir + '/probe/🦀️.rs', 'w', encoding='utf-8').write('\n'.join(ls) + '\n')
    return run_probe(root, [], lines, w)
if __name__ == '__main__':
    base, work, census = sys.argv[1:4]
    ob, onon = old_probe(base)
    nb, nnon = new_probe(work, census)
    only_new = {k: v for k, v in nb.items() if k not in ob}
    print('old failures', len(ob), 'new failures', len(nb), 'new-only', len(only_new))
    for k, v in sorted(only_new.items())[:80]: print('  NEW-ONLY', k, '|', v)
    json.dump(dict(old=ob, new=nb), open(S + 'probe-result.json', 'w'), ensure_ascii=False, indent=1)
