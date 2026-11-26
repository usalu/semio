from pathlib import Path
import json
import os
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
cache = generated / "current-native/b/debug/build"
option_fixture = "--option-fixture" in sys.argv
full_history = "--full-history" in sys.argv
option_case = "--option-case" in sys.argv or option_fixture
fresh_schema = "--fresh-schema" in sys.argv
symbols_only = "--symbols-only" in sys.argv
scratch = generated / ("law-5d-option-fixture" if option_fixture else "law-5d-option-schema" if fresh_schema else "law-5d-option-case" if option_case else "law-5d-symbols" if symbols_only else "law-relink-5d")
links = scratch / "deps"
links.mkdir(parents=True, exist_ok=True)
rustc = root / ".cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc"
workspace = next(root.glob("✏️s/🔌*/🧩*/🗿*/🖐️5d"))

artifacts = {}
native_paths = set()
for path in cache.glob("*/*/out/*"):
    if path.suffix == ".a":
        native_paths.add(path.parent)
    if not path.name.startswith("lib") or path.suffix not in (".rlib", ".so", ".rmeta"):
        continue
    link = links / path.name
    if not link.exists():
        link.symlink_to(path)
    if path.suffix == ".rmeta":
        continue
    crate = path.name[3:].rsplit("-", 1)[0]
    fingerprint = path.parent.parent / "fingerprint" / f"lib-{crate}"
    if fingerprint.exists():
        artifacts[fingerprint.read_text().strip()] = (crate, path, fingerprint.with_suffix(".json"))

target = "semio_s_artifact_puzzle_5d_mut_any_4" if fresh_schema else "semio_s_artifact_puzzle_5d_puzzle5d"
replaced = {}
changed = {}


def dependencies(fingerprint):
    return [checksum.to_bytes(8, "little").hex() for _, _, _, checksum in json.loads(fingerprint.read_text())["deps"]]


def needs_rebuild(checksum):
    if checksum not in changed:
        record = artifacts.get(checksum)
        changed[checksum] = record is not None and (
            not symbols_only and (fresh_schema or not option_case)
            and (record[0] == target or any(needs_rebuild(child) for child in dependencies(record[2])))
        )
    return changed[checksum]


def compile_source(crate, fingerprint, source, output, test, extra_extern=None):
    metadata = json.loads(fingerprint.read_text())
    common = [str(rustc), str(source), "--edition=2021", "--crate-name", crate,
              "-C", "debuginfo=0", "-Z", "threads=1", "-L", f"dependency={links}", "-o", str(output)]
    if test:
        common += ["--test", "-Z", "unstable-options", "-C", "linker-features=+lld", "-C", "link-self-contained=+linker"]
        if full_history:
            common += ["-C", "link-arg=-Wl,--threads=2", "-C", "strip=symbols"]
    else:
        common += ["--crate-type", "rlib", "-C", "metadata=law_history"]
    for _, alias, _, identity in metadata["deps"]:
        checksum = identity.to_bytes(8, "little").hex()
        if checksum not in artifacts:
            continue
        lib = rebuild(checksum)
        common += ["--extern", f"{alias}={lib}"]
        if lib.with_suffix(".rmeta").is_file():
            common += ["--extern", f"{alias}={lib.with_suffix('.rmeta')}"]
    for alias, lib in (extra_extern or {}).items():
        common += ["--extern", f"{alias}={lib}"]
    for path in sorted(native_paths):
        common += ["-L", f"native={path}"]
    for feature in json.loads(metadata["features"]):
        common += ["--cfg", f'feature="{feature}"']
    package = fingerprint.parents[2].name
    env = {**os.environ, "ZIG_GLOBAL_CACHE_DIR": str(generated / "zig"),
           "ZIG_LOCAL_CACHE_DIR": str(generated / "zig-local"), "CARGO_MANIFEST_DIR": str(source.parent),
           "CARGO_PKG_VERSION": "0.1.0", "CARGO_PKG_NAME": package, "RUST_MIN_STACK": "134217728",
           "OUT_DIR": str(fingerprint.parents[1] / "out")}
    print(f"[DEBUG] rebuilding {package} against cached dependencies", flush=True)
    status = subprocess.run(common, cwd=workspace, env=env).returncode
    if status:
        raise SystemExit(status if status > 0 else 128 - status)


def rebuild(checksum):
    crate, path, fingerprint = artifacts[checksum]
    if not needs_rebuild(checksum):
        return links / path.name
    if checksum not in replaced:
        dep_info = path.with_name(path.name.removeprefix("lib")).with_suffix(".d")
        first = dep_info.read_text().splitlines()[0].split(": ", 1)[1].split()[0]
        output = links / f"lib{crate}-{checksum}-law.rlib"
        if not (full_history and output.is_file()):
            compile_source(crate, fingerprint, (workspace / first).resolve(), output, test=False)
        replaced[checksum] = output
    return replaced[checksum]


crate = "semio_s_artifact_puzzle_5d_v1_unit_tests"
fingerprint = cache / "semio-s-artifact-puzzle-5d-v1-unit-tests" / "d78d73d3f1b96766" / "fingerprint" / f"test-lib-{crate}.json"
source = next(workspace.glob("🏅*/🔖*/🪆*/✳*/✏️*/🧪*/🔬️unit-2/📦*/🦀*/🦀️.rs"))
executable = scratch / ("history-full" if full_history else "history")
if option_case:
    def reachable(fingerprint, seen):
        for checksum in dependencies(fingerprint):
            if checksum in seen or checksum not in artifacts:
                continue
            seen.add(checksum)
            record = artifacts[checksum]
            yield record
            yield from reachable(record[2], seen)
    acceptance = "semio_plugin_testing_acceptance"
    _, dependency, acceptance_fingerprint = next(
        record for record in reachable(fingerprint, set())
        if record[0] == acceptance and "artifact-app-testing" in json.loads(json.loads(record[2].read_text())["features"])
    )
    dep_info = dependency.with_name(dependency.name.removeprefix("lib")).with_suffix(".d")
    first = dep_info.read_text().splitlines()[0].split(": ", 1)[1].split()[0]
    library = links / f"lib{acceptance}-law-option-case.rlib"
    if not (full_history and library.is_file()):
        compile_source(acceptance, acceptance_fingerprint, (workspace / first).resolve(), library, test=False)
    if option_fixture:
        module = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🧪️tests/🔎️array-controls/🦀️.rs"
        test_source = scratch / "options-fixture.rs"
        test_source.write_text(
            'extern crate semio_os_kernel as pack;\nextern crate semio_os_kernel as protocol;\nextern crate semio_os_kernel as store;\n'
            'pub use semio_plugin_testing_acceptance::acceptance_change_buckets;\n'
            f'#[path = "{module}"] mod controls;\n'
        )
        compile_source("law_history_option_fixture", acceptance_fingerprint, test_source, executable, test=True, extra_extern={acceptance: library})
        raise SystemExit(subprocess.run([str(executable), "--exact", "controls::history_edit_acceptance_option_uses_the_default_only_when_omitted", "--nocapture"],
                                        cwd=source.parent, env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode)
    compile_source(crate, fingerprint, ticket / "law-5d-create-part.rs", executable, test=True, extra_extern={acceptance: library})
    test_case = "history_edits_end_to_end" if full_history else "create_part_anchor_option_history"
    raise SystemExit(subprocess.run([str(executable), "--exact", test_case, "--nocapture"],
                                    cwd=source.parent, env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode)
compile_source(crate, fingerprint, source, executable, test=True)
if symbols_only:
    print(executable, flush=True)
    raise SystemExit(0)
sys.exit(subprocess.run([str(executable), "mirror::editor::puzzle5d::unit_tests::history_edits_end_to_end",
                         "--exact", "--nocapture"], cwd=source.parent,
                        env={**os.environ, "RUST_MIN_STACK": "134217728"}, check=False).returncode)
