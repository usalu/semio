""" 🔎 Verify the captured archive's exact native mutation bytes."""

import hashlib
import re
import subprocess
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ARTIFACTS = TICKET / "🗑️generated"
LIBS = ARTIFACTS / "qr"
PROBE = ARTIFACTS / "archive-mutation-witness"
PROBE.mkdir(exist_ok=True)
BINARY = PROBE / "witness.exe"
TOOLCHAIN = re.search(r'^channel = "([^"]+)"$', (TICKET.parents[6] / "rust-toolchain.toml").read_text(), re.M)[1]
COMMAND = ["repo", "tools", "exec", "rustup", "--", "run", TOOLCHAIN, "rustc",
           "--edition=2024", "-L", f"dependency={LIBS}", "-L", f"dependency={ARTIFACTS / 'aw'}",
           "-o", str(BINARY), str(TICKET / "native-archive-mutation-witness.rs")]
LIBRARIES = {
    "semio_os_spr": "c78228f82adacf8a",
    "semio_os_dsl": "c18af18348f9b839",
    "semio_os_pack": "5090cefc2d4d397c",
    "semio_framework_value": "05431580f111efc8",
    "semio_framework_value_native_owned": "996df75d9ed9fa1b",
}
for crate, fingerprint in LIBRARIES.items():
    artifact = LIBS / f"lib{crate}-{fingerprint}.rlib"
    assert artifact.is_file(), artifact
    COMMAND += ["--extern", f"{crate}={artifact}"]
    if artifact.with_suffix(".rmeta").is_file():
        COMMAND += ["--extern", f"{crate}={artifact.with_suffix('.rmeta')}"]
subprocess.run(COMMAND, check=True)


def crc32c(data):
    checksum = 0xFFFFFFFF
    for byte in data:
        checksum ^= byte
        for _ in range(8):
            checksum = (checksum >> 1) ^ (0x82F63B78 if checksum & 1 else 0)
    return checksum ^ 0xFFFFFFFF


assert crc32c(b"123456789") == 0xE3069283
archive = ARTIFACTS / "late-profile-terrain.semio-archive"
assert hashlib.sha256(archive.read_bytes()).hexdigest() == "35c7af2c336e32530bb0375d17d437d91584976a900283959a9d5252b2a85c81"
subprocess.run([str(BINARY), str(archive), str(PROBE)], check=True)
for raw in sorted(PROBE.glob("*.op*")) + [PROBE / "parent.spk", PROBE / "parent.spr"]:
    data = raw.read_bytes()
    print(raw.name, len(data), data[:32].hex(), hashlib.sha256(data).hexdigest())
    for offset in range(len(data) - 31):
        if data[offset:offset + 8] not in (b"\x89SPK\r\n\x1a\n", b"\x89SPR\r\n\x1a\n"):
            continue
        header = data[offset:offset + 32]
        computed = crc32c(header[:20])
        stored = int.from_bytes(header[20:24], "little")
        print(f"[DEBUG] {raw.name}+{offset}: CRC-32C stored={stored:08x} computed={computed:08x}")
        assert computed == stored
