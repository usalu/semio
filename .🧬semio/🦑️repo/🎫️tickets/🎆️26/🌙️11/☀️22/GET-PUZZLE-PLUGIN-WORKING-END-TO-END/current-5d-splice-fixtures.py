from pathlib import Path
import os,sys,subprocess

ticket = Path(__file__).resolve().parent
base = ticket / 'law-relink-5d.py'
src = base.read_text()
src = src.replace('    for path in sorted(native_paths):', '        if lib.with_suffix(".rmeta").is_file(): common += ["--extern", alias + "=" + str(lib.with_suffix(".rmeta"))]\n    for path in sorted(native_paths):')
src=src.replace('        if checksum not in artifacts:', '        if alias == "semio_s_artifact_puzzle_5d_mut_any_5": continue\n        if checksum not in artifacts:')
src = src[:src.index('if option_case:\n    def reachable')]
src = src.replace('links = scratch / "deps"', 'links = generated / "law-5d-option-schema/deps"\nscratch=generated/"current-5d-splice-fixtures"\nscratch.mkdir(exist_ok=True)')
src += '''
mroot = next(workspace.glob("🏅*/🔖*/🪆*/✳*/🧬*schema/🧬*mutations"))
entry = scratch / "tests.rs"
body = "extern crate semio_os_kernel as protocol;\\nuse protocol::ColdMutation;\\nuse semio_s_artifact_puzzle_5d_mut_any_5::{Puzzle5dSnapshot, standards::v1::subsets::any::schema::mutations::{Puzzle5dMutation,inverse_puzzle5d_mutation}};\\n"
for i,p in enumerate(sorted(mroot.glob("🔀*splice*/🧪*/*/🦀*.rs"))):
    body += f'#[path = "{p}"] mod mutation_fixture_{i};\\n'
entry.write_text(body)
compile_source("puzzle_scene_rows_fixture", fingerprint, entry, executable, test=True, extra_extern={"semio_s_artifact_puzzle_5d_mut_any_5":next(links.glob("libsemio_s_artifact_puzzle_5d_mut_any_5-*-law.rlib"))})
raise SystemExit(subprocess.run([str(executable), "--nocapture"], env={**os.environ,"RUST_MIN_STACK":"134217728"}).returncode)
'''
sys.argv=[str(base),'--symbols-only']
exec(compile(src,str(base),'exec'))
