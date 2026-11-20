"""🧪️ Rebuild the current Norm panel closure for an isolated WASI viewport proof."""

import argparse
import json
import subprocess
from pathlib import Path


def key(package, name, features, wasi):
    return package, name, tuple(sorted(features)), wasi


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--graph', type=Path, required=True)
    parser.add_argument('--lane', type=Path, required=True)
    parser.add_argument('--rustc', type=Path, required=True)
    parser.add_argument('--viewport', type=Path)
    parser.add_argument('--changed', action='append', default=[])
    parser.add_argument('--full-surface', action='store_true')
    args = parser.parse_args()
    root = Path.cwd()
    output = Path(__file__).resolve().parent / '🗑️generated' / 'viewport-panel-libs'
    output.mkdir(exist_ok=True)
    lane = args.lane.resolve()
    units = json.loads(args.graph.read_text())['units']
    artifacts = {}
    for line in args.artifacts.read_text().splitlines():
        event = json.loads(line)
        if event.get('reason') != 'compiler-artifact':
            continue
        paths = tuple(Path(path) for path in event['filenames'] if Path(path).suffix in ('.rlib', '.rmeta', '.dylib', '.so', '.dll'))
        if paths and all(path.is_file() for path in paths):
            wasi = paths[0].is_relative_to(lane / 'wasm32-wasip2')
            artifacts[key(event['package_id'], event['target']['name'], event['features'], wasi)] = paths

    platform = 'wasm32-wasip2'
    viewport = 'semio_s_plugin_norm_app_surface_viewport3d'
    live_core = 'semio_s_plugin_norm_app_surface_live_core'
    dirty = set(args.changed) or {viewport, live_core}
    while True:
        expanded = dirty | {u['target']['name'] for u in units if u['platform'] == platform and any(units[d['index']]['target']['name'] in dirty for d in u['dependencies'])}
        if expanded == dirty:
            break
        dirty = expanded

    search = [f'-Ldependency={path}' for directory in (lane / 'prototype/build', lane / 'wasm32-wasip2/prototype/build') for path in sorted(directory.glob('*/*/out'))]
    search.append(f'-Ldependency={output}')
    if args.viewport:
        search.append(f'-Ldependency={args.viewport.resolve().parent}')
    built = {}

    def build(index):
        if index in built:
            return built[index]
        unit = units[index]
        target = unit['target']
        name = target['name']
        wasi = unit['platform'] == platform
        if not wasi or name not in dirty:
            return artifacts[key(unit['pkg_id'], name, unit['features'], wasi)]
        if name == viewport and args.viewport:
            result = tuple(args.viewport.resolve().with_suffix(extension) for extension in ('.rlib', '.rmeta'))
        else:
            deps = [(d['extern_crate_name'], build(d['index'])) for d in unit['dependencies']]
            externs = [flag for alias, files in deps for path in files for flag in ('--extern', f'{alias}={path}')]
            cfgs = [flag for feature in unit['features'] for flag in ('--cfg', f'feature="{feature}"')]
            result = tuple(output / f'lib{name}.{extension}' for extension in ('rlib', 'rmeta'))
            options = [f'--edition={target["edition"]}', f'--target={platform}', '-Cpanic=abort', '-Copt-level=0', '--crate-type=rlib',
                f'--crate-name={name}', f'--emit=link={result[0]},metadata={result[1]}', target['src_path'], *search, *cfgs, *externs]
            response = output / f'{name}.args'
            response.write_text('\n'.join(options) + '\n')
            print(f'Compiling {name}', flush=True)
            with (output / f'{name}.log').open('w') as log:
                subprocess.run([args.rustc.resolve(), f'@{response}'], cwd=root, stdout=log, stderr=log, check=True)
        assert all(path.is_file() for path in result), result
        built[index] = result
        return result

    target = 'semio_s_plugin_norm_app_surface' if args.full_surface else 'semio_s_plugin_norm_app_surface_panels'
    index, = [i for i, unit in enumerate(units) if unit['target']['name'] == target and unit['platform'] == platform]
    build(index)
    print(output)


if __name__ == '__main__':
    main()
