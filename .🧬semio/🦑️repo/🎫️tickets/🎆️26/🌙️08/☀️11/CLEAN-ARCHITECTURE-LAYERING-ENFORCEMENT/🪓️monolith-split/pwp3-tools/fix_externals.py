"""📦️ Adds the external dependencies that the post-lift lib files of a WP3 crate name (use statements included) and its manifest lacks, copying the facade's spec with the path re-relativised to the crate's package folder. DRY=1 prints. usage: CLOSURE=<closure.json> python3 fix_externals.py"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import externals3 as ex
ROOT = ex.ROOT; PL = ex.PL
fac_raw = ex.facade.get('dependencies', {})
PKG = ROOT + PL + '📦️packages/🦀️rust'
def spec_for(libname, crate_dir):
    key = next(k for k, v in fac_raw.items() if (v.get('package', k) if isinstance(v, dict) else k).replace('-', '_') == libname or k.replace('-', '_') == libname)
    v = fac_raw[key]
    if isinstance(v, dict) and 'path' in v:
        target = os.path.normpath(os.path.join(PKG, v['path']))
        v = dict(v); v['path'] = os.path.relpath(target, crate_dir)
    return key, v
def render(key, v):
    if isinstance(v, str): return f'{key} = "{v}"'
    parts = []
    order = ['version', 'path', 'package', 'workspace', 'features', 'default-features', 'optional']
    for k in order + [k for k in v if k not in order]:
        if k not in v: continue
        x = v[k]
        parts.append(f'{k} = ' + (json.dumps(x, ensure_ascii=False) if not isinstance(x, bool) else str(x).lower()))
    return f'{key} = {{ ' + ', '.join(parts) + ' }'
apply = os.environ.get('DRY') != '1'
for crate, root in ex.ROOTS.items():
    files = ex.inp['lib'].get(crate, [])
    used = ex.used_externals(files)
    m = ex.manifest(crate)
    decl = ex.libnames({k: v for k, v in m.get('dependencies', {}).items() if not k.startswith('semio-framework-plugin-') or k in ('semio-framework-plugin-ownership', 'semio-framework-plugin-operation', 'semio-framework-plugin-mounted-owner')})
    declared_all = set(ex.libnames(m.get('dependencies', {})).values())
    missing = sorted(u for u in used if u not in declared_all and u not in ('semio_framework_os_kernel',) and not u.startswith('semio_framework_plugin_') or (u in ('semio_framework_plugin_ownership', 'semio_framework_plugin_operation', 'semio_framework_plugin_mounted_owner') and u not in declared_all))
    # first-party partition crates are never added here
    partition = {'semio_framework_plugin_' + k.replace('-', '_') for k in ex.ROOTS} | {'semio_framework_plugin_decl_views', 'semio_framework_plugin_decl_app', 'semio_framework_plugin_decl_emission', 'semio_framework_plugin_decl_registry', 'semio_framework_plugin_plugin_program', 'semio_framework_plugin_abi', 'semio_framework_plugin_identity', 'semio_framework_plugin_interaction', 'semio_framework_plugin_actor_host', 'semio_framework_plugin_operation_progress'}
    missing = [u for u in missing if u not in partition]
    crate_dir = ROOT + PL + root + '/📦️packages/🦀️rust'
    lines = []
    for u in missing:
        try: key, v = spec_for(u, crate_dir)
        except StopIteration: print('  no facade spec for', u); continue
        lines.append(render(key, v))
    print(crate, 'adds', len(lines))
    for l in lines: print('    ', l[:170])
    if apply and lines:
        p = crate_dir + '/Cargo.toml'
        t = open(p, encoding='utf8').read()
        a = '\n[dev-dependencies]'
        assert t.count(a) == 1, p
        t = t.replace(a, '\n'.join(lines) + '\n' + a, 1) if t.split(a)[0].endswith('\n') else t.replace(a, '\n' + '\n'.join(lines) + '\n' + a, 1)
        open(p, 'w', encoding='utf8').write(t)
