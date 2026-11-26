from pathlib import Path
import sys

original = Path(__file__).with_name("ui-transport-native.py").resolve()
sys.argv[1:] = ["--preview", "--preview-api", "--reuse"]
source = original.read_text()
source = source.replace('generated / "ui-preview-api"', 'generated / "ui-actor-native"')
source = source.replace('target = "lib-semio_os_store_engine" if api_only else "lib-semio_os_store_engine_derived"', 'target = "lib-semio_os_store_backbone"')
source = source.replace('    compile_crate(engine, engine_fingerprint, dependencies)\n    raise SystemExit(0)', '''
    engine = compile_crate(engine, engine_fingerprint, dependencies)
    exe = scratch / "ordinary-law"
    json_lib = max(shared.glob("libserde_json-*.rlib"), key=lambda library: library.stat().st_mtime)
    test = ticket / "ui-ordinary-native.rs"
    fixture = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/📤️original-outbox/transfer.json"
    env = {**os.environ, "SEMIO_UI_ACTOR_TRANSFER_FIXTURE": str(fixture),
           "ZIG_GLOBAL_CACHE_DIR": str(generated / "zig"), "ZIG_LOCAL_CACHE_DIR": str(generated / "zig-local")}
    args = [rustc, test, "--test", "--edition=2021", "-Z", "unstable-options",
            "-C", "linker-features=+lld", "-C", "link-self-contained=+linker",
            "-L", f"dependency={scratch}", "-L", f"dependency={shared}", "-o", exe]
    for path in native:
        args.extend(["-L", f"native={path}"])
    externs = {"semio_os_store_backbone": engine, "serde_json": json_lib}
    externs.update({name: dependencies[name] for name in ("semio_framework_value", "semio_os_spr")})
    for name, archive in externs.items():
        args.extend(["--extern", f"{name}={archive}"])
        metadata = archive.with_suffix(".rmeta")
        if metadata.is_file():
            args.extend(["--extern", f"{name}={metadata}"])
    subprocess.run(args, env=env, check=True)
    raise SystemExit(subprocess.run([exe, "--nocapture"]).returncode)''')
exec(compile(source, str(original), "exec"), {"__file__": str(original)})
