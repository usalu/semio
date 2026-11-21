"""🧪️ Exercise the CDN archive integrity checks on GNU-style manifests."""
import hashlib
import io
import os
import subprocess
import sys
from pathlib import Path
import runpy
import tarfile
import tempfile

ticket = Path(__file__).resolve().parent
verify = runpy.run_path(str(ticket / "verify-release-archive.py"))["verify"]
page = b"<title>Demonstrator</title>"
digest = hashlib.sha256(page).hexdigest()
with tempfile.TemporaryDirectory(dir=ticket / "🗑️generated") as temp:
    archive = Path(temp) / "site.tar.gz"
    def package(entries):
        with tarfile.open(archive, "w:gz") as output:
            for name, data in entries:
                member = tarfile.TarInfo(name)
                member.size = len(data)
                output.addfile(member, io.BytesIO(data))
    site = [("./index.html", page), ("./SHA256SUMS", f"{digest}  index.html\n".encode())]
    package(site)
    assert verify(archive) == (1, len(page))
    accepted = Path(temp) / "accepted.sha256"
    accepted.write_bytes(site[1][1])
    assert verify(archive, accepted) == (1, len(page))
    changed = b"changed"
    package([("./index.html", changed), ("./SHA256SUMS", f"{hashlib.sha256(changed).hexdigest()}  index.html\n".encode())])
    try:
        verify(archive, accepted)
    except ValueError as error:
        assert "differs from the accepted site" in str(error)
    else:
        raise AssertionError("self-consistent archive of an unaccepted revision")

    for entries in (site[:1], site[1:], [("index.html", b"wrong"), site[1]],
                    site + [("./index.html", page)], site + [("../escape", b"x")],
                    site + [("extra", b"x")]):
        package(entries)
        try:
            verify(archive)
        except ValueError:
            continue
        raise AssertionError(entries)
    package(site)
    archive.write_bytes(archive.read_bytes()[:-3])
    try:
        verify(archive)
    except EOFError:
        pass
    else:
        raise AssertionError("truncated gzip trailer")
    source = Path(temp) / "site"
    source.mkdir()
    files = {"index.html": page, "CNAME": b"example.test", ".nojekyll": b"", "SHA256SUMS": b"previous publisher"}
    for name, body in files.items():
        (source / name).write_bytes(body)
    destination = Path(temp) / "packages"
    accepted.write_bytes(b"".join(f"{hashlib.sha256(files[name]).hexdigest()}  {name}\n".encode() for name in sorted(files) if name != "SHA256SUMS"))
    command = ["bash", str(ticket / "package-site.sh"), str(source), str(destination), str(accepted)]
    environment = {**os.environ, "PACKAGE_PYTHON": sys.executable}
    subprocess.run(command, env=environment, check=True, capture_output=True)
    packaged, = destination.glob("site-*.tar.gz")
    assert verify(packaged, accepted) == (3, sum(map(len, (page, b"example.test"))))
    assert {p.name: p.read_bytes() for p in source.iterdir()} == files, "packaging changed its captured input"
    accepted.write_bytes(b"wrong acceptance")
    assert subprocess.run(command, env=environment, capture_output=True).returncode != 0
    assert list(destination.iterdir()) == [packaged]
    assert {p.name: p.read_bytes() for p in source.iterdir()} == files
print("release archive fixture passed")
