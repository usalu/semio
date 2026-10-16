#!/usr/bin/env python3
"""🚀️ Stages manifests, then flushes the generated tree and the shared registries to the repository with conflict checks (dry run unless --apply)."""
import os, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import state, finalize_host, plan_host, regs_host, hcmod
from carve import REPO


class Light:
    pass


def main(apply=False):
    ws, model, roots = state.load()
    C = Light()
    C.new = model
    new_crates, mans, feats, tested = finalize_host.stage_files(ws, C)
    texts, out = finalize_host.registries(new_crates)
    longest = max((len(os.path.relpath(p, REPO).encode('utf-8')), os.path.relpath(p, REPO)) for p in ws.changed())
    print('changed files', len(ws.changed()), 'new crates', len(new_crates), 'longest path bytes', longest[0])
    for p in out:
        print('registry', os.path.relpath(p, REPO), len(texts[p]), '->', len(out[p]))
    for name, pkg, pid in new_crates:
        print(' ', name, pid)
    outdir = os.path.join(os.path.dirname(os.path.abspath(__file__)), '../🗑️generated/phc-manifests')
    os.makedirs(outdir, exist_ok=True)
    for ident, (text, usage) in mans.items():
        open(os.path.join(outdir, plan_host.CRATES[ident]['name'] + '.toml'), 'w', encoding='utf-8').write(text)
    if apply:
        flushed = ws.flush()
        finalize_host.write_registries(texts, out)
        print('flushed', len(flushed))
    return ws, model, mans


if __name__ == '__main__':
    main('--apply' in sys.argv)
