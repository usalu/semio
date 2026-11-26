#!/usr/bin/env python3
"""📦️ Pass a linked procedural core through the pinned WASI component encoder."""
import hashlib
import os
import pathlib
import shutil
import subprocess
import sys


def core_at(path):
    with path.open("rb") as stream:
        if stream.read(8) != b"\0asm\x01\0\0\0":
            raise ValueError(f"Expected a core WebAssembly module: {path}")
    return path


def deliver_core(arguments):
    source = core_at(pathlib.Path(os.environ["SEMIO_PROCEDURAL_LINKED_CORE"]))
    destination = pathlib.Path(arguments[arguments.index("-o") + 1])
    shutil.copyfile(source, destination)


def pack_core(arguments):
    source, output, compiler = map(lambda path: pathlib.Path(path).resolve(), arguments)
    core_at(source)
    sysroot = subprocess.check_output([compiler, "--print", "sysroot"], text=True).strip()
    host_record = subprocess.check_output([compiler, "-vV"], text=True).splitlines()
    host = next(line.split(" ", 1)[1] for line in host_record if line.startswith("host: "))
    name = "wasm-component-ld.exe" if os.name == "nt" else "wasm-component-ld"
    encoder = pathlib.Path(sysroot) / "lib" / "rustlib" / host / "bin" / name
    subprocess.run(
        [encoder, "--wasm-ld-path", pathlib.Path(__file__).resolve(), "--validate-component", "true", "-o", output, source],
        env=dict(os.environ, SEMIO_PROCEDURAL_LINKED_CORE=str(source)),
        check=True,
    )
    digest = hashlib.sha256()
    with output.open("rb") as stream:
        header = stream.read(8)
        if header != b"\0asm\r\0\x01\0":
            raise ValueError(f"Expected a WebAssembly component: {output}")
        digest.update(header)
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    print("[DEBUG] Encoded component", output.stat().st_size, digest.hexdigest(), flush=True)


if "SEMIO_PROCEDURAL_LINKED_CORE" in os.environ:
    deliver_core(sys.argv[1:])
else:
    pack_core(sys.argv[1:])
