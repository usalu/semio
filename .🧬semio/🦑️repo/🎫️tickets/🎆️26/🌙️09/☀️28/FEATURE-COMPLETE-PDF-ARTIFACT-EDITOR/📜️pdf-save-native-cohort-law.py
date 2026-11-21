"""🔗 Link visible-graph and page-box regressions against one exact native Save compiler cohort."""

import argparse
import hashlib
import json
import os
import re
from pathlib import Path
import shutil
import subprocess

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
SAVE = "semio_s_artifact_stdio_pdf_native_save"
CASES = {
    "visible": ("pdf-save-visible-graph", "visible-save", "proof.rs", "VISIBLE"),
    "boxes": ("pdf-save-box-graft", "box-graft", "box-graft.rs", "BOX"),
}


def dependencies(library):
    """🧾 Read the repository broker's actual Rust metadata witnesses."""
    metadata = library.with_suffix(".rmeta.semio-deps")
    return json.loads(metadata.read_text())


def cohort(cache, directories):
    """🪪 Match each direct consumer to the recorded identifier and exact metadata hash."""
    selected, = cache.glob(f"*/*/out/lib{SAVE}-*.rlib")
    identity = dependencies(selected)
    index = {}
    for directory in directories:
        for path in directory.iterdir():
            if path.name.startswith("lib") and path.suffix in (".rlib", ".dylib", ".so", ".dll"):
                index.setdefault(path.stem.removeprefix("lib"), []).append(path)
    result = {SAVE: selected}
    for name, svh in identity["dependencies"]:
        paths = index.get(name, [])
        if not paths:
            continue
        matched = []
        for path in paths:
            metadata = path.with_suffix(".rmeta.semio-deps")
            if not metadata.exists() or dependencies(path)["svh"] == svh:
                matched.append(path)
        assert len(matched) == 1, (name, svh, paths, matched)
        result.setdefault(name.rsplit("-", 1)[0], matched[0])
    if "serde_json" not in result:
        candidates = [path for name, paths in index.items() if name.startswith("serde_json-") for path in paths]
        result["serde_json"] = sorted(candidates)[-1]
    return result


def run(args):
    """🧪 Materialize guarded postimages and drive the original native graph and paid writer."""
    name, folder, candidate_file, environment = CASES[args.case]
    output = GENERATED / folder
    output.mkdir(parents=True, exist_ok=True)
    (output / "law.rs").write_bytes((TICKET / ("📜️" + name + "-law.rs")).read_bytes())
    (output / "law-output.rs").write_bytes((TICKET / "📜️pdf-save-law-output.rs").read_bytes())
    (output / "law.json").write_bytes((TICKET / ("🔣️" + name + ".json")).read_bytes())
    if args.case == "boxes":
        shutil.copyfile(GENERATED / "pdf-page-history-source.pdf", output / "in.pdf")
    cache = args.cache.resolve()
    directories = sorted(cache.glob("*/*/out"))
    libraries = cohort(cache, directories)
    suffix = ".wasm" if args.wasi else ".exe" if os.name == "nt" else ""
    binary = output / (args.mode + suffix)
    rust = ["--edition=2021", "--crate-name", "pdf_" + args.case + "_native_law", str(output / "law.rs"), "-o", str(binary)]
    if args.wasi:
        rust.extend(("--target=wasm32-wasip2", "-Clink-arg=-zstack-size=8388608"))
        metadata = output / ("md-" + cache.parents[2].name)
        metadata.mkdir(exist_ok=True)
        for directory in directories:
            for artifact in directory.glob("*.rmeta"):
                link = metadata / artifact.name
                if not link.exists():
                    try:
                        link.symlink_to(artifact)
                    except (NotImplementedError, OSError):
                        link.write_bytes(artifact.read_bytes())
        rust.extend(("-L", "dependency=" + str(metadata)))
        directories += sorted((cache.parents[2] / "prototype" / "build").glob("*/*/out"))
    else:
        rust.append("--test")
    direct = [SAVE, "serde_json"]
    if args.mode == "green":
        ((path, before, candidate),) = json.loads((GENERATED / (name + "-overlay.json")).read_text())
        assert hashlib.sha256(Path(path).read_bytes()).hexdigest() == before
        (output / candidate_file).write_text(candidate, encoding="utf8")
        rust.extend(("--cfg", "candidate"))
        if args.case == "boxes":
            direct.append("semio_framework_value")
    for name in direct:
        selected = libraries[name]
        rust.extend(("--extern", f"{name}={selected.with_suffix('.rmeta')}", "--extern", f"{name}={selected}"))
    for directory in directories:
        rust.extend(("-L", "dependency=" + str(directory)))
        for archive in directory.glob("*.a"):
            rust.extend(("-L", "native=" + str(archive.parent)))
            break
    response = output / ("native-" + args.mode + ".rsp")
    response.write_text("\n".join(rust) + "\n", encoding="utf8")
    print("[DEBUG] Linked Save cohort", libraries[SAVE], flush=True)
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
    env = {**os.environ, "SEMIO_PDF_" + environment + "_FIXTURE": str(output), "RUST_MIN_STACK": "134217728"}
    command = [str(binary), "--nocapture"]
    if args.wasi:
        command = ["repo", "tools", "exec", "--light", "jco", "--", "run", str(binary), "--jco-dir", str(output / (args.mode + "-jco"))]
    result = subprocess.run(command, cwd=output, env=env, check=False, capture_output=True)
    stdout = result.stdout
    if args.wasi and args.mode == "green" and result.returncode == 0:
        frame = re.search(rb"SEMIO_PDF_SAVE_BYTES (\d+)\n", stdout)
        assert frame is not None
        start = frame.end()
        expected_length = int(frame.group(1))
        end = start + 2 * expected_length
        data = bytes.fromhex(stdout[start:end].decode("ascii"))
        assert data.startswith(b"%PDF-") and len(data) == expected_length
        (output / "out.pdf").write_bytes(data)
        stdout = stdout[:frame.start()] + stdout[end:]
    stderr = result.stderr.decode(errors="replace")
    print(stdout.decode(errors="replace"), stderr, end="")
    if args.mode == "red":
        expected = "retained page geometry qualification" if args.case == "boxes" else "retained qualified visible graph"
        assert result.returncode != 0 and expected in stderr, result.returncode
    else:
        result.check_returncode()


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("cache", type=Path)
    parser.add_argument("case", choices=CASES)
    parser.add_argument("mode", choices=("red", "green"))
    parser.add_argument("--wasi", action="store_true")
    run(parser.parse_args())
