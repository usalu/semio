"""📎 Links native Open host law against the pinned repository SPR, Value, and folder event log."""
from pathlib import Path
from runpy import run_path
import json
from hashlib import sha256
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated/native-wgpu-save"
STAGE = OUT / "open"
RUSTC = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
if "--live" not in sys.argv and "--wire-live" not in sys.argv:
    run_path(str(TICKET / "📜️native-wgpu-open-stage.py"), run_name="__main__")
entries = json.loads((STAGE / "receipt.json").read_bytes())
if "--live" in sys.argv:
    for entry in entries:
        active = TICKET.parents[6] / entry["file"]
        staged = STAGE / entry["source"]
        assert sha256(active.read_bytes()).hexdigest() == sha256(staged.read_bytes()).hexdigest() == entry["after"]
for item in entries:
    subprocess.run([*RUSTC, "-Zparse-crate-root-only", "--edition=2021", str(STAGE / item["source"])], check=True)

def module(suffix):
    """🔍 Selects a reviewed source by live destination."""
    [item] = [row for row in entries if row["file"].endswith(suffix)]
    if "--wire-live" in sys.argv:
        return (TICKET.parents[6] / item["file"]).read_text(encoding="utf8")
    return (STAGE / item["source"]).read_text(encoding="utf8")

command = "impl NativeMediaDeclaration {" + module("💾️save/📂️open.rs").split("impl NativeMediaDeclaration {", 1)[1]
shell = module("🎨️theme-file-import-and-accessibility/📂️open.rs")
reader = shell.split("async fn read_selected_file", 1)[1].split("pub fn native_local_documents", 1)[0]
cancel_import = "async fn wait_native_file_import" + shell.split("async fn wait_native_file_import", 1)[1].split("async fn read_selected_file", 1)[0]
contract = json.loads((TICKET.parents[6] / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json").read_bytes())["wireIngress"]
fixture = (TICKET / "📜️native-wgpu-open-law.rs").read_text(encoding="utf8").replace("usize::MAX /* wire fixture */", str(contract["nativePayloadBytes"]))
source = OUT / "open-law.rs"
source.write_text(fixture.replace("/* import command */", command)
    .replace("/* bounded reader */", "async fn read_selected_file" + reader)
    .replace("/* cancellable import */", cancel_import), encoding="utf8")

if "--reader-only" in sys.argv:
    abi = (TICKET.parents[6] / "🧰️framework/🔨️modules/⏳️async/🔀️combinators/🦀️.rs").read_text(encoding="utf8")
    yield_body = abi.split("pub async fn yield_once()", 1)[1].split("\n}\n", 1)[0]
    fixture = fixture.split("fn wait", 1)[1].split("#[test]", 1)
    law = fixture[1].split("#[test]", 1)[1].split("/* cancellable import */", 1)[0]
    source.write_text(
        "use std::{future::Future, io::Read, path::{Path, PathBuf}, task::{Context, Poll, Waker}};\n"
        f"const EXPECTED_NATIVE_FILE_BYTES: usize = {contract['nativePayloadBytes']};\n"
        "mod semio_framework_async { use std::task::Poll; pub async fn yield_once()" + yield_body + "\n}}\n"
        "async fn read_selected_file" + reader + "fn wait" + fixture[0] + "#[test]" + law,
        encoding="utf8",
    )
    binary = OUT / ("open-reader.exe" if sys.platform == "win32" else "open-reader")
    subprocess.run([*RUSTC, "--edition=2021", "--test", str(source), "-o", str(binary)], check=True)
    subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(binary), "--nocapture"], check=True)
    readback = (OUT / "open-input/readback.pdf").read_bytes()
    expected = bytes.fromhex("255044462d312e37") + bytes((n % 257) % 256 for n in range(9700))
    assert readback == expected
    print("[DEBUG] independent Python binary readback SHA-256 " + sha256(readback).hexdigest())
    raise SystemExit(0)

if "--folder-provider" in sys.argv:
    cache = Path(sys.argv[sys.argv.index("--folder-provider") + 1]).resolve()
    all_artifacts = list(cache.glob("*/*/out/*"))
    [storage] = [file for file in all_artifacts if file.name.startswith("libsemio_os_store_sync_native-") and file.suffix == ".rmeta"]
    known = dict(json.loads(storage.with_suffix(".rmeta.semio-deps").read_bytes())["dependencies"])
    [spr] = [file for file in all_artifacts if file.name.startswith("libsemio_os_spr-") and file.suffix == ".rmeta"
             and file.name[3:-6] in known]
    parts = fixture.split("fn wait", 1)[1].split("#[test]", 1)
    folder = fixture.split("fn admits_the_first_archive_and_preserves_existing_durable_bytes", 1)[1]
    source = OUT / "folder-law.rs"
    source.write_text("use std::{future::Future,path::PathBuf,task::{Context,Poll,Waker}};\nfn wait" +
        parts[0].split("#[cfg(", 1)[0] + "#[test]\nfn admits_the_first_archive_and_preserves_existing_durable_bytes" +
        folder.replace("semio_os_kernel::sync::", "semio_os_store_sync_native::"), encoding="utf8")
    metadata = OUT / "open-folder-deps"
    metadata.mkdir(exist_ok=True)
    from shutil import copyfile
    for file in all_artifacts:
        if file.suffix not in (".rmeta", ".so", ".dll", ".dylib"):
            continue
        link = metadata / file.name
        if not link.exists():
            try:
                link.symlink_to(file.resolve())
            except OSError:
                copyfile(file, link)
    binary = OUT / ("folder-law.exe" if sys.platform == "win32" else "folder-law")
    args = ["--edition=2021", "--test", "-Awarnings", str(source), "-o", str(binary), f"-Ldependency={metadata}"]
    for file in (storage, spr):
        name = file.name[3:-6].rsplit("-", 1)[0]
        args.extend(("--extern", f"{name}={file}", "--extern", f"{name}={file.with_suffix('.rlib')}"))
    args.extend(f"-Ldependency={directory}" for directory in sorted({file.parent for file in all_artifacts if file.suffix == ".rlib"}))
    rsp = OUT / "open-folder.rsp"
    rsp.write_text("\n".join(args) + "\n", encoding="utf8")
    subprocess.run([*RUSTC, f"@{rsp}"], check=True)
    subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(binary), "--nocapture"], check=True)
    raise SystemExit(0)

mode = next((flag for flag in ("--wasm-metadata", "--wasm-run", "--native-root") if flag in sys.argv), None)
if mode:
    cache = Path(sys.argv[sys.argv.index(mode) + 1]).resolve()
    native = mode == "--native-root"
    artifacts = list(cache.glob("*/*/out/*")) + list(cache.parent.parent.parent.glob("prototype/build/*/*/out/*.dylib"))
    kernels = [file for file in artifacts if file.name.startswith("libsemio_os_kernel-") and file.suffix == ".rmeta"]
    if "--kernel" in sys.argv:
        selected = Path(sys.argv[sys.argv.index("--kernel") + 1]).resolve()
        kernels = [file for file in kernels if file == selected]
    [kernel] = kernels
    keys = dict(json.loads(kernel.with_suffix(".rmeta.semio-deps").read_bytes())["dependencies"])
    metadata = OUT / ("open-native" if native else "open-wasi")
    metadata.mkdir(exist_ok=True)
    for file in artifacts:
        if file.suffix not in (".rmeta", ".so", ".dll", ".dylib"):
            continue
        link = metadata / file.name
        if not link.exists():
            from shutil import copyfile
            try:
                link.symlink_to(file.resolve())
            except OSError:
                copyfile(file, link)
    args = ["--edition=2021", "--target=wasm32-wasip2", "--test", "--emit=metadata", "-Awarnings",
            "-Ldependency=" + str(metadata), str(source), "-o", str(OUT / "open-law.rmeta")]
    binary = OUT / (("open-law.exe" if sys.platform == "win32" else "open-law") if native else "open-spr.wasm")
    if native:
        args = [arg for arg in args if arg not in ("--target=wasm32-wasip2", "--emit=metadata")]
        args[args.index("-o") + 1] = str(binary)
        if "--native-folder" in sys.argv:
            args.extend(("--cfg", "native_open_folder"))
    if mode == "--wasm-run":
        law = source.read_text(encoding="utf8")
        parts = law.split("#[test]")
        assert len(parts) == 4
        source.write_text(parts[0] + parts[1] + "\nfn main() { natural_open_addresses_exact_owned_bytes_and_uses_the_registered_binary_descriptor(); }\n", encoding="utf8")
        args = [arg for arg in args if arg not in ("--test", "--emit=metadata")]
        args[args.index("-o") + 1] = str(binary)
    crates = ("semio_os_kernel", "semio_os_spr", "semio_plugin_decl_media", "semio_plugin_decl_app",
              "semio_framework", "semio_framework_value", "semio_framework_async")
    for name in crates:
        candidates = [file for file in artifacts if file.name.startswith(f"lib{name}-") and file.suffix == ".rmeta"]
        if name == "semio_os_kernel":
            candidates = [kernel]
        elif len(candidates) > 1:
            candidates = [file for file in candidates if file.name[3:-6] in keys]
        [path] = candidates
        args.extend(("--extern", f"{name}={path}"))
        if mode != "--wasm-metadata":
            args.extend(("--extern", f"{name}={path.with_suffix('.rlib')}"))
    if mode != "--wasm-metadata":
        args.extend(f"-Ldependency={directory}" for directory in sorted({file.parent for file in artifacts if file.suffix == ".rlib"}))
    if native:
        args.extend(("--cfg", "native_open_import"))
    rsp = OUT / ("open-native.rsp" if native else "open-wasi.rsp")
    rsp.write_text("\n".join(args) + "\n", encoding="utf8")
    subprocess.run([*RUSTC, f"@{rsp}"], check=True)
    if native:
        subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(binary), "--nocapture"], check=True)
    elif mode == "--wasm-run":
        subprocess.run(["repo", "tools", "exec", "jco", "--", "run", str(binary)], check=True)
        print("[DEBUG] Native Open SPR and descriptor command law passed under WASI")
    else:
        print("[DEBUG] Native Open SPR and descriptor command passed actual WASI typecheck")
    raise SystemExit(0)

raise SystemExit("Choose --reader-only, --native-root <cache>, --wasm-metadata <cache> or --wasm-run <cache>")
