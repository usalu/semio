"""📦️ Stream the release archive and check its exact contents against its manifest."""
import gzip
import hashlib
from pathlib import Path, PurePosixPath
import re
import sys
import tarfile


def relative_name(name):
    name = name.removeprefix("./")
    path = PurePosixPath(name)
    if not name or path.is_absolute() or ".." in path.parts or str(path) != name:
        raise ValueError(f"unexpected archive path {name!r}")
    return name


def verify(path):
    actual = {}
    expected = None
    total = 0
    with gzip.open(path, "rb") as inflated, tarfile.open(fileobj=inflated, mode="r|") as archive:
        for member in archive:
            if member.name in (".", "./") and member.isdir():
                continue
            name = relative_name(member.name)
            if member.isdir():
                continue
            if not member.isfile() or name in actual:
                raise ValueError(f"unexpected or duplicate archive entry {name}")
            stream = archive.extractfile(member)
            if name == "SHA256SUMS":
                if expected is not None:
                    raise ValueError("duplicate checksum manifest")
                expected = {}
                for line in stream:
                    match = re.fullmatch(rb"([0-9a-f]{64}) [ *](.+)\n", line)
                    if not match:
                        raise ValueError("invalid checksum manifest entry")
                    relative = relative_name(match[2].decode("utf-8"))
                    if relative in expected:
                        raise ValueError(f"duplicate checksum for {relative}")
                    expected[relative] = match[1].decode("ascii")
            else:
                digest = hashlib.sha256()
                for block in iter(lambda: stream.read(1024 * 1024), b""):
                    digest.update(block)
                actual[name] = digest.hexdigest()
                total += member.size
        while inflated.read(1024 * 1024):
            pass
    if expected is None:
        raise ValueError("missing checksum manifest")
    for name in sorted(actual.keys() | expected.keys()):
        if actual.get(name) != expected.get(name):
            raise ValueError(f"archive checksum or membership mismatch for {name}")
    return len(actual), total


if __name__ == "__main__":
    count, size = verify(Path(sys.argv[1]))
    print(f"Verified {count} files and {size} bytes in the release archive")
