"""🧪️ Compare the release inventory with the system SHA-256 implementation."""
import json
from pathlib import Path
import runpy
import shutil
import subprocess
import tempfile
from unittest.mock import patch

ticket = Path(__file__).resolve().parent
write = runpy.run_path(str(ticket / "site-manifest.py"))["write_manifest"]
tool = shutil.which("sha256sum")
shasum = shutil.which("shasum")
with tempfile.TemporaryDirectory(dir=ticket / "🗑️generated") as directory:
    root = Path(directory)
    site = root / "site"
    site.mkdir()
    case = json.loads((ticket / "🔣️site-manifest.json").read_bytes())
    files = {name: bytes.fromhex(value) for name, value in case["files"].items()}
    for name, content in files.items():
        (site / name).write_bytes(content)
    if tool or shasum:
        command = [tool] if tool else [shasum, "-a", "256"]
        expected = subprocess.check_output([*command, *sorted(files)], cwd=site)
    else:
        expected = b""
        for name in sorted(files):
            digest = subprocess.check_output(["powershell", "-NoProfile", "-Command",
                "(Get-FileHash -Algorithm SHA256 -LiteralPath '" + name + "').Hash"], cwd=site)
            expected += digest.strip().lower() + b"  " + name.encode() + b"\n"
    assert write(site, root, site / "SHA256SUMS") == (case["expectedCount"], case["expectedBytes"])
    assert (site / "SHA256SUMS").read_bytes() == expected
    write(site, root, root / "before")
    assert (root / "before").read_bytes() == expected
    (site / "index.html").write_bytes(bytes.fromhex(case["mutatedIndex"]))
    write(site, root, root / "after")
    assert (root / "after").read_bytes() != expected
    for directory, name in (("d" * 140, "n" * 120), ("🧱" * 60, "n" * 132)):
        candidate = site / directory / name
        portable_name = candidate.relative_to(root).as_posix()
        windows_length = int(subprocess.check_output(["node", "-p", "process.argv[1].length", portable_name]))
        assert windows_length > 256
        with patch.object(Path, "rglob", return_value=[candidate]):
            try:
                write(site, root, site / "SHA256SUMS")
            except ValueError as error:
                assert "longer than 256" in str(error), error
            else:
                raise AssertionError("over-length release path accepted")
    assert (site / "SHA256SUMS").read_bytes() == expected
print("Release manifest matched the system SHA-256 tool and detected changed content.")
