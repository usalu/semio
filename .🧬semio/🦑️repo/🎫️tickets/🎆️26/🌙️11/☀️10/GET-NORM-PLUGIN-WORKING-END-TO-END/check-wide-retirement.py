"""🌳 Replay the committed depth law on the existing WASI libraries with old and current derive code."""
from pathlib import Path
import os
import subprocess
import sys

ticket = Path(__file__).resolve().parent
repo = ticket.parents[6]
work = ticket / '🗑️generated'
lane = repo / '.cache/b/r/m/u/l' / sys.argv[1]
derive = repo / '🧰️framework/🔨️modules/🌱️value/✨️derive'
test_file = derive / '🧪️tests/🧬️generic-owner/🦀️.rs'
fixture_file = derive / '🧪️tests/🧫️fixtures/🧬️generic-owner/🔣️.json'
rustc = os.environ['RUSTC']


def dependencies(target):
    return [f'-Ldependency={out}' for out in target.glob('build/*/*/out')]


def library(target, crate, suffix='rlib'):
    matches = list((target / 'build' / crate).glob(f'*/out/*.{suffix}'))
    if len(matches) != 1:
        raise ValueError((crate, matches))
    return matches[0]


def extern(name, path):
    key = name.replace('-', '_')
    paths = [path, path.with_suffix('.rmeta')] if path.suffix == '.rlib' else [path]
    return [part for item in paths for part in ('--extern', f'{key}={item}')]


host = lane / 'prototype'
wasm = lane / 'wasm32-wasip2/prototype'
macro_suffix = {'darwin': 'dylib', 'win32': 'dll'}.get(sys.platform, 'so')
current = work / f'libsemio_framework_value_derive.{macro_suffix}'
proc = [rustc, '--edition=2021', '--crate-name=semio_framework_value_derive', '--crate-type=proc-macro', '--extern', 'proc_macro', str(derive / '🦀️.rs'), '-o', str(current), *dependencies(host)]
for name in ('syn', 'quote', 'proc-macro2', 'semio-framework-value-derive-expansion'):
    proc.extend(extern(name, library(host, name)))
if subprocess.run(proc).returncode:
    raise SystemExit('RetireOwned proc macro failed to compile')

source = test_file.read_text()
record = source[source.index('macro_rules! wide_retirement_record'):source.index('#[test]\nfn wide_record_retires')]
body = source[source.index('fn wide_record_retires'):].replace('fn wide_record_retires_in_field_order_within_the_original_depth()', 'fn main()')
body = body.replace('fixture_text!("../../🧪️tests/🧫️fixtures/🧬️generic-owner/🔣️.json")', f'include_str!(r#"{fixture_file}"#)')
body = body.replace('    let mut released = ', '    let mut wallet = AuthoringWallet::new(RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 64 }, 1 << 20);\n    let mut released = ')
body = body.replace('let progress = effect::driven(grant, |effect| owner.step(4096, effect)).0.result.unwrap().progress();', '''let work = owner.next_computational_work(4096).unwrap();
        let (turn, _) = wallet.run_settled(grant, work, |effect| owner.step(4096, effect)).unwrap().unwrap();
        let progress = turn.result.unwrap().progress();
        wallet.post(progress).unwrap();''')
body = body.rstrip()[:-1] + '    println!("DISPLAY {} ordered field releases, {} net bytes", released.len(), released_bytes - admitted_bytes);\n}\n'
probe = work / 'wide_retirement.rs'
probe.write_text('use semio_framework_value::{AuthoringWallet, RetainedCloneGrant};\n' + record + body)
old = library(host, 'semio-framework-value-derive', macro_suffix)
for case, macro, code in [('before', old, 1), ('after', current, 0)]:
    component = work / f'wide-retirement-{case}.wasm'
    command = [rustc, '--edition=2021', '--target=wasm32-wasip2', '-Copt-level=1', '-Cpanic=abort', str(probe), '-o', str(component), *dependencies(wasm), *dependencies(host)]
    command.extend(extern('semio-framework-value-derive', macro))
    for name in ('semio-framework-value', 'serde_json'):
        command.extend(extern(name, library(wasm, name)))
    if subprocess.run(command).returncode:
        raise SystemExit(f'{case}: WASI proof failed to compile')
    print('[DEBUG] running', case, flush=True)
    run = subprocess.run(['node_modules/.bin/jco', 'run', '--jco-dir', work / f'wide-retirement-{case}-jco', component], cwd=repo, capture_output=True, text=True)
    print(run.stdout, run.stderr, end='\n', flush=True)
    if run.returncode != code or (code and 'wide record retirement demand' not in run.stderr):
        raise SystemExit(f'{case}: unexpected exit {run.returncode}')
