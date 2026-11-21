"""📦️ Inventory a CDN site and publish its GNU-style SHA-256 manifest atomically."""
import hashlib
import os
from pathlib import Path
import sys
import tempfile


def write_manifest(site, workspace, destination):
    rows = []
    for path in site.rglob("*"):
        name = path.relative_to(site).as_posix()
        relative = path.relative_to(workspace) if path.is_relative_to(workspace) else Path(name)
        if len(relative.as_posix().encode("utf-16-le")) // 2 > 256:
            raise ValueError(f"path longer than 256 characters: {relative}")
        if path.is_symlink():
            raise ValueError(f"release symlink {name}")
        if path.is_dir() or name == "SHA256SUMS":
            continue
        if not path.is_file() or any(char in name for char in "\n\r\\"):
            raise ValueError(f"unsupported release entry {name!r}")
        digest = hashlib.sha256()
        size = 0
        with path.open("rb") as source:
            for block in iter(lambda: source.read(1024 * 1024), b""):
                digest.update(block)
                size += len(block)
        rows.append((name, digest.hexdigest(), size))
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(dir=destination.parent, delete=False) as output:
            temporary = Path(output.name)
            for name, digest, _ in sorted(rows):
                output.write(f"{digest}  {name}\n".encode("utf-8"))
        temporary.chmod(0o644)
        os.replace(temporary, destination)
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)
    total = sum(size for _, _, size in rows)
    print(f"{len(rows)} files, {total} bytes. Largest files:")
    for name, _, size in sorted(rows, key=lambda row: row[2], reverse=True)[:10]:
        print(size, name)
    return len(rows), total


if __name__ == "__main__":
    write_manifest(*(Path(value).resolve() for value in sys.argv[1:]))
