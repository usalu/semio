"""🧪 Check the first SPR and mounted owner against native or WASI production dependencies."""
import os
from pathlib import Path
import re
import struct
import subprocess
import sys
import tempfile

import google_crc32c

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
cache = Path(sys.argv[1]).resolve()
wasm = "wasm32-wasip2" in cache.parts
prefix = ["repo", "tools", "exec", "--light"]
out = ticket / "🗑️generated/native-first-archive-exchange"
out.mkdir(parents=True, exist_ok=True)
pack = ticket / "🗑️generated/paged-jpeg-assets/large.dct-placement.pack"
toolchain = re.search(r'^channel = "([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.MULTILINE).group(1)
libraries = {
    "cold_spr": "semio_os_spr", "protocol": "protocol", "semio_os_pack": "semio_os_pack",
    "semio_framework_value": "semio_framework_value", "semio_framework_pack_source": "semio_framework_pack_source",
    "semio_framework_hash": "semio_framework_hash", "semio_framework_job": "semio_framework_job",
    "semio_os_vcs": "semio_os_vcs",
}

with tempfile.TemporaryDirectory(dir=out) as directory:
    directory = Path(directory)
    binary = directory / ("proof.wasm" if wasm else "proof.exe" if os.name == "nt" else "proof")
    capture = directory / "spr.bin"
    wire = (ticket / "natural-pdf-wire-records.rs").read_text()
    wire = re.sub(r"^//!.*$|^#\[(?:serde|value)\(.*\)\]$", "", wire, flags=re.MULTILINE)
    wire = re.sub(r",?\s*(?:serde|semio_framework_value(?:_derive)?)::\w+", "", wire)
    exchange = (ticket / "native-first-archive-exchange.rs").read_text().split("\n", 1)[1]
    exchange = exchange.replace("use protocol::wire::command_ingress::", "use crate::wire_fixture::")
    env = os.environ.copy()
    for kind, source in [("WIRE", wire), ("EXCHANGE", exchange)]:
        path = directory / f"{kind}.rs"
        path.write_text(source)
        env["SEMIO_FIRST_ARCHIVE_" + kind] = str(path)
    args = ["--edition=2024", "-O", str(ticket / "native-first-archive-mounted-law.rs"), "-o", str(binary)]
    if wasm:
        args += ["--target", "wasm32-wasip2"]
    cold_spr = list(cache.glob("*/*/out/libsemio_os_spr-*.rlib"))
    if not cold_spr:
        args += ["--cfg", "first_archive_without_spr_cache"]
        libraries.pop("cold_spr")
    rustc = prefix + ["rustup", "--", "run", toolchain, "rustc"]

    def metadata(library):
        output = subprocess.run(rustc + ["-Z", "ls=root", str(library.with_suffix(".rmeta"))],
                                cwd=root, check=True, capture_output=True, text=True).stdout
        return dict(re.findall(r"^\d+ (\w+)-([a-f0-9]+) hash ", output, re.MULTILINE))

    vcs, = cache.glob("*/*/out/libsemio_os_vcs-*.rlib")
    dependencies_of_vcs = metadata(vcs)
    for alias, library in libraries.items():
        paths = sorted(cache.glob(f"*/*/out/lib{library}-*.rlib"))
        if len(paths) > 1:
            unit = dependencies_of_vcs.get(library)
            if library == "semio_framework_job":
                expected = dependencies_of_vcs["semio_framework_job_context"]
                paths = [path for path in paths if metadata(path).get("semio_framework_job_context") == expected]
            elif unit:
                paths = [path for path in paths if f"-{unit}." in path.name]
        assert len(paths) == 1, (library, paths)
        for path in (paths[0].with_suffix(".rmeta"), paths[0]):
            args += ["--extern", f"{alias}={path}"]
    discovery = [cache]
    if wasm:
        discovery.append(cache.parents[2] / "prototype/build")
    dependencies = {artifact.parent for cache_root in discovery for artifact in cache_root.glob("*/*/out/*")
                    if artifact.suffix in (".rlib", ".rmeta", ".dylib", ".so", ".dll")}
    args += [f"-Ldependency={path}" for path in sorted(dependencies)]
    response = directory / "rustc.args"
    response.write_text("\n".join(args) + "\n")
    subprocess.run(rustc + ["@" + str(response)],
                   cwd=root, env=env, check=True)
    if wasm:
        command = ["repo", "tools", "exec", "jco", "--", "run", str(binary), "--"]
    else:
        command = [str(binary)]
    subprocess.run(command + [str(capture), str(pack)], cwd=root, check=True)
    encoded = capture.read_bytes()


def varint(body, offset):
    result = 0
    for shift in range(0, 28, 7):
        byte = body[offset]
        offset += 1
        result |= (byte & 127) << shift
        if byte < 128:
            return result, offset
    raise AssertionError("invalid integer")


position = 0
records = []
for record_count in [5, 4, 3]:
    position += 32
    kinds = []
    for _ in range(record_count):
        length, start = varint(encoded, position)
        end = start + length
        expected_crc, backward = struct.unpack_from("<II", encoded, end)
        assert expected_crc == google_crc32c.value(encoded[start:end])
        assert backward == end + 8 - position
        kinds.append(encoded[start])
        position = end + 8
    records.append(kinds)
assert position == len(encoded)
assert records[0][3] == records[1][2] == 0x41
assert records[0][-1] == records[1][-1] == records[2][-1]
print("Independent Python framing and Google CRC32C verified all first-SPR records")
