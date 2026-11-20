"""🧪️ Exercise the CDN archive integrity checks on GNU-style manifests."""
import hashlib
import io
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
print("release archive fixture passed")
