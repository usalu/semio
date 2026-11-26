from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
base = ticket / "ui-transport-native.py"
code = base.read_text().split("\nif is_history:")[0]
current = "--current-mounted" in sys.argv
law = "--law" in sys.argv
view = "--view" in sys.argv
drained = "--drained" in sys.argv
command_only = "--command" in sys.argv
mode = "latency-command" if command_only else "latency-drained" if drained else "latency-view" if view else "latency-law" if law else "latency-current" if current else "latency-mounted" if "--mounted" in sys.argv else "latency-page"
code = code.replace('scratch = generated / "ui-preview-api" if api_only else common', f'scratch = generated / "ui-{mode}"')
code = code.replace('crate = fingerprint.name.removeprefix', 'crate = "editor_surface" if not path else fingerprint.name.removeprefix')
code = code.replace('source = (config.parent / specification["lib"]["path"]).resolve()', 'source = test_source if not path else (config.parent / specification["lib"]["path"]).resolve()')
exec(compile(code, str(base), "exec"))
if current:
    shared = generated / "ui-latency-current/deps"
    shared.mkdir(exist_ok=True)
if current and not view:
    for fp in cache.glob("*/*/fingerprint/lib-*"):
        if fp.suffix or not (key := fp.read_text().strip()):
            continue
        for library in (fp.parent.parent / "out").glob(f"lib{fp.name.removeprefix('lib-')}-*"):
            if library.suffix not in {".rlib", ".rmeta", ".so"}:
                continue
            link = shared / library.name
            if not link.exists() and not link.is_symlink():
                link.symlink_to(library)
            if library.suffix in {".rlib", ".so"}:
                artifacts[key] = library, fp
baseline = cache / "semio-s-artifact-puzzle-3d-editor/575626de412aa72e/fingerprint/test-integration-test-editor-action-producer.json"
if command_only:
    changed_packages = set()
    directory = cache / "semio-plugin-command/ac6c929c3d408750"
    library = directory / "out/libsemio_plugin_command-ac6c929c3d408750.rlib"
    fingerprint = directory / "fingerprint/lib-semio_plugin_command.json"
    _, dependencies = closure(fingerprint)
    compile_crate(library, fingerprint, dependencies)
    raise SystemExit(0)
elif drained:
    changed_packages = set()
    directory = cache / "semio-plugin-decl-media/bd1db4f92a534b27"
    library = directory / "out/libsemio_plugin_decl_media-bd1db4f92a534b27.rlib"
    fingerprint = directory / "fingerprint/lib-semio_plugin_decl_media.json"
    _, dependencies = closure(fingerprint)
    media = compile_crate(library, fingerprint, dependencies)
    dependencies = {key: dependencies[key] for key in ("semio_framework_job", "semio_framework_value", "serde_json")}
    dependencies["semio_plugin_decl_media"] = media
    test_source = next(root.glob("🧰️framework/*/*/*/🔌️plugin/📜️declaration/🎞️media/🏭️construction/🧪️drained/🦀️.rs"))
    tests = ["original_drained_output_returns_the_exact_admitted_page_table"]
elif view:
    baseline = cache / "semio-s-artifact-puzzle-3d-editor/94dd611b159cff27/fingerprint/test-integration-test-editor-panels.json"
    view_retirement = next(root.glob("🧰️framework/*/*/*/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️mounted/🖼️view-retirement/🦀️.rs"))
    test_source = scratch / "view-retirement.rs"
    test_source.write_text(f'#[path="{view_retirement}"] mod view_retirement;\n')
    view_dependencies = {"semio_framework_job": "8538151342c510a6", "semio_framework_value": "13be4d38b95f6cb8", "semio_framework_trace": "dcf6b74f34baa54a", "serde_json": "c97f218b5bd58b39"}
    dependencies = {key: (shared / f"lib{key}-{identity}.rlib").resolve() for key, identity in view_dependencies.items()}
    dependencies["semio_framework"] = (shared / "libsemio_framework_manifest_view_model-4c41f4619dd98e6f.rlib").resolve()
    tests = ["plain_action_view_returns_its_exact_original_allocation_in_one_item"]
elif mode == "latency-page":
    changed_packages = set()
    baseline = cache / "semio-s-artifact-puzzle-2d-v1/7e55bd335d639832/fingerprint/lib-semio_s_artifact_puzzle_2d_v1.json"
    _, surface = closure(baseline)
    candidate = next((artifacts[key] for key in compiled if artifacts[key][1].name == "lib-semio_plugin_result_page"), None)
    if candidate is None:
        directory = cache / "semio-plugin-result-page/30b73108cf01f126"
        candidate = (next((directory / "out").glob("*.rlib")), directory / "fingerprint/lib-semio_plugin_result_page")
    library, fingerprint = candidate
    fingerprint = fingerprint.with_suffix(".json")
    _, dependencies = closure(fingerprint)
    page = compile_crate(library, fingerprint, dependencies)
    requested = {"semio_framework_job", "serde_json"}
    dependencies.update({alias: surface[alias] for alias in requested})
    dependencies["semio_plugin_result_page"] = page
    test_source = scratch / "result-page.rs"
    tests = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️pending/🧪️compact/🦀️.rs"
    test_source.write_text(f'#[path = "{tests}"] mod pending_result_page;\n')
    tests = ["pending_result_page::"]
else:
    changed_packages = {
        "semio-plugin-result-page", "semio-plugin-retained-command", "semio-plugin-time-travel",
        "semio-plugin-tool-run", "semio-plugin-tool-run-operations", "semio-plugin-command", "semio-plugin-tool-machine",
    }
    if current:
        changed_packages = set()
        identity, target = os.environ.get("PUZZLE_LATENCY_FINGERPRINT", "94dd611b159cff27:editor-panels").split(":", 1)
        baseline = cache / "semio-s-artifact-puzzle-3d-editor" / identity / "fingerprint" / f"test-integration-test-{target}.json"
    from collections import defaultdict
    root_lib = cache / "semio-s-artifact-puzzle-3d-editor/e6c19ebd210932dd/fingerprint/lib-semio_s_artifact_puzzle_3d_editor.json"
    package_keys = defaultdict(set)
    visited = set()

    def visit(path):
        if path in visited:
            return
        visited.add(path)
        for package, _, _, checksum in json.loads(path.read_text())["deps"]:
            key = checksum.to_bytes(8, "little").hex()
            if key in artifacts:
                package_keys[package].add(key)
                visit(artifacts[key][1].with_suffix(".json"))

    visit(root_lib)
    root_key = root_lib.with_suffix("").read_text().strip()
    surface = json.loads(baseline.read_text())
    for row in surface["deps"]:
        package, alias, _, checksum = row
        prior = checksum.to_bytes(8, "little").hex()
        candidates = package_keys[package]
        if alias == "semio_s_artifact_puzzle_3d_editor":
            row[3] = int.from_bytes(bytes.fromhex(root_key), "little")
        elif candidates and prior not in candidates:
            actual = max(candidates, key=lambda key: artifacts[key][0].stat().st_mtime)
            row[3] = int.from_bytes(bytes.fromhex(actual), "little")
    graph = scratch / "matched-3d-test.json"
    graph.write_text(json.dumps(surface))
    _, dependencies = closure(baseline if current else graph)
    if recorded_command := os.environ.get("PUZZLE_LATENCY_COMMAND_LOG"):
        import shlex
        command_line = next(line for line in reversed((generated / recorded_command).read_text().splitlines()) if "process didn't exit successfully:" in line and "`" in line)
        command = shlex.split(command_line.split("`", 2)[1])
        dependencies = {}
        for index, arg in enumerate(command[:-1]):
            if arg != "--extern":
                continue
            alias, path = command[index + 1].split("=", 1)
            path = Path(path)
            archive = path.with_suffix(".rlib")
            dependencies[alias] = archive if archive.exists() else path
    probe = (generated / "native-ui-3d-latency-trace.rs").read_text()
    original_dispatch = '    dispatch_unsettled(&mut app,"translateSelection",Some(&json!({"ids":[victim],"dx":3.5,"dy":0.0,"dz":0.0})),None).await.unwrap();'
    probe = probe.replace(original_dispatch, f'''    if std::env::var_os("PUZZLE_LATENCY_DELETE").is_some() {{
        dispatch_unsettled(&mut app,"deleteSelection",None,None).await.unwrap();
    }} else {{
    {original_dispatch}
    }}''')
    marker = "            let owner = unit.page_publication.owner.as_ref();"
    probe = probe.replace(marker, '''            if format!("{:?}", unit.stage) == "Retiring" {
                eprintln!("[DEBUG] turn={turn} close {:?}: meta={} session={} request={} reserved={} raw={} emit={} emit-close={} completion={} completion-close={} chunks={} chunks-close={} window-config={} window-transient={} lease={} lease-close={} direct={}",
                    unit.retirement_demands(524288), unit.meta.view_state.is_some(), unit.session.is_some(), unit.producer_request.is_some(), unit.reserved_producer.is_some(), unit.raw_input.is_some(), unit.publication.is_some(), unit.publication_retirement.is_some(), unit.completion.is_some(), unit.completion_retirement.is_some(), unit.output_chunks.is_some(), unit.output_retirement.is_some(), unit.window_config_authority.is_some(), unit.window_transient_authority.is_some(), unit.cancellation_lease.is_some(), unit.cancellation_retirement.is_some(), !unit.granted_retirement_ready());
            }
''' + marker)
    test_source = scratch / "mounted.rs"
    if law:
        latency = next(root.glob("✏️s/🔌*/🧩*/🗿*/🧊*/🏅*/🔖*/🪆*/✳*/✏*/🧪*/🔬️mutation-latency/🦀️.rs"))
        probe = probe.split("#[semio_framework_async_macros::async_test]", 1)[0]
        probe += f'#[path="{latency}"] mod mutation_latency;\n'
    test_source.write_text(probe)
    tests = ["one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns" if law else "small_translation_receipts"]
executable = compile_crate(None, baseline, dependencies)
raise SystemExit(subprocess.run([executable, "--nocapture", *tests], env={**os.environ, "RUST_MIN_STACK": "134217728"}).returncode)
