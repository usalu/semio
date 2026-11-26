from pathlib import Path

ticket = Path(__file__).resolve().parent
recipe = ticket / "current-5d-relink.py"
script = recipe.read_text()
needed = {
    "semio_os_kernel", "semio_framework_async_macros", "semio_framework_pack_json",
    "semio_framework_test_fixtures", "semio_framework_trace", "serde_json",
    "semio_framework_value", "semio_framework_value_persistent",
    "semio_s_artifact_puzzle_2d_snapshot", "semio_s_artifact_puzzle_2d_records",
    "semio_s_artifact_puzzle_2d_mutations", "semio_s_artifact_puzzle_2d_schema",
}
select_dependencies = f"""dependencies = [dep for dep in meta['deps'] if dep[1] in {sorted(needed)!r}]
mutation_archive = libs/'libsemio_s_artifact_puzzle_2d_mutations-9cbb6dff22d59ec0.rlib'
metadata = subprocess.check_output([str(toolchain/'rustc'), '-Zls=root', str(mutation_archive.with_suffix('.rmeta'))], text=True)
for _, alias, _, checksum in dependencies:
    key = checksum.to_bytes(8, 'little').hex()
    if key in artifacts:
        continue
    if alias == 'semio_s_artifact_puzzle_2d_mutations':
        artifacts[key] = mutation_archive
        continue
    resolved = [path for path in libs.glob('lib'+alias+'-*.rlib') if path.stem[3:]+' hash ' in metadata]
    assert len(resolved) == 1, (alias, resolved)
    artifacts[key] = resolved[0]
"""
script = (
    script.replace('gen / "current-native/b/debug/build"', 'gen / "law-storage-native/b/debug/build"')
    .replace('gen / "current-5d-relink"', 'gen / "law-2d-delete-inverse"')
    .replace("semio_s_artifact_puzzle_5d_v1_unit_tests", "semio_s_artifact_puzzle_2d_connect_handles_preparation")
    .replace("semio-s-artifact-puzzle-5d-v1-unit-tests", "semio-s-artifact-puzzle-2d-connect-handles-preparation")
    .replace("if registry_probe else next(fingerprints)", "if registry_probe else next(path for path in fingerprints if path.parents[1].name == 'e5f46b2c44a0e185')")
    .replace("🖐*/🏅*/🔖*/🪆*/✳*/✏️*/🧪*/🔬️unit-2", "◻️2d/🏅*/🔖*/🪆*/✳*/🧬*/🧬*/🔌*/🪢*/🎮*")
    .replace("else manifest/'🦀️.rs'", "else ticket/'law-2d-delete-inverse.rs'")
    .replace("dependencies = meta['deps']", select_dependencies)
    .replace("link.symlink_to(output)", "os.link(output, link)")
    .replace("except FileExistsError:", "except (FileExistsError, FileNotFoundError):")
    .replace("subprocess.run(args, env=env,check=True)", "subprocess.run(args+['-Z','threads=1','-C','link-arg=-Wl,--threads=1'], env=env,check=True)")
    .replace("'--test-threads=2'", "'--test-threads=1'")
)
exec(compile(script, str(recipe), "exec"))
