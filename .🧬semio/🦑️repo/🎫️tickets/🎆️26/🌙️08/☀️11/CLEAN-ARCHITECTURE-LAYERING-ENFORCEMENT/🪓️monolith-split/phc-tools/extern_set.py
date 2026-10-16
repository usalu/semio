#!/usr/bin/env python3
"""🧱️ External-crate identifier set for the stitched differential after the flush: the union of every external dependency declared by the host family manifests (the monolith manifest no longer exists on disk)."""
import os, sys, json, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, regs_host, oracle_host as O

TOML = 'const t=Bun.TOML.parse(await Bun.file(process.argv[1]).text()); console.log(JSON.stringify(t))'


def family_external():
    ids = set(O.U.EXTERNAL)
    fam = {v['name'].replace('-', '_') for v in plan_host.CRATES.values()}
    for v in plan_host.CRATES.values():
        t = json.loads(subprocess.run(['bun', '-e', TOML, regs_host.pkg_dir_of(v['root']) + '/Cargo.toml'], capture_output=True, text=True).stdout)
        for tbl in ('dependencies', 'dev-dependencies'):
            for k, d in t.get(tbl, {}).items():
                for n in (k, d.get('package') if isinstance(d, dict) else None):
                    if n and n.replace('-', '_') not in fam:
                        ids.add(n.replace('-', '_'))
    return ids


def install():
    O.U.EXTERNAL = family_external()
