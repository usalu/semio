"""📦 Retains the exact dependency cohort of the staged native Save proof."""

from argparse import ArgumentParser
from hashlib import sha256
import json
from pathlib import Path
from shutil import copyfile

OUT = Path(__file__).resolve().parent / "🗑️generated/sl/cohort"
ROOTS = (
    "semio-plugin", "semio-s-artifact-stdio-pdf-schema-4", "semio-s-artifact-stdio-pdf-schema-5",
    "semio-s-artifact-stdio-pdf-conformance-support", "semio-s-artifact-stdio-pdf-lift",
    "semio-s-artifact-stdio-pdf-lower", "semio-plugin-testing-laws",
)


def retain(cache):
    keys = set()
    for name in ROOTS:
        [receipt] = (cache / name).glob("*/out/*.rmeta.semio-deps")
        keys.add(receipt.name.removeprefix("lib").removesuffix(".rmeta.semio-deps"))
        keys.update(key for key, _ in json.loads(receipt.read_bytes())["dependencies"])
    files = []
    for path in cache.glob("*/*/out/*"):
        key = path.name.removeprefix("lib").split(".", 1)[0]
        if key in keys and (path.name.endswith((".rmeta", ".rmeta.semio-deps", ".rlib", ".dylib", ".so", ".dll", ".d"))):
            files.append(path)
    bytes_retained = 0
    for group in (False, True):
        for path in files:
            if (path.suffix == ".rlib") != group:
                continue
            owner, *unit = path.relative_to(cache).parts
            destination = OUT.joinpath(sha256(owner.encode()).hexdigest()[:8], *unit)
            assert len(str(destination).encode("utf-16-le")) // 2 <= 256, destination
            destination.parent.mkdir(parents=True, exist_ok=True)
            initial = path.stat()
            copyfile(path, destination)
            assert destination.stat().st_size == initial.st_size, path
            final = path.stat()
            assert (initial.st_size, initial.st_mtime_ns, initial.st_ino) == (final.st_size, final.st_mtime_ns, final.st_ino), path
            bytes_retained += initial.st_size
        print(f"[DEBUG] Retained {'link' if group else 'metadata'} artifacts, {bytes_retained} bytes total", flush=True)
    print(f"[DEBUG] Retained {len(files)} exact cache files in the ticket")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("cache", type=Path)
    args = parser.parse_args()
    retain(args.cache.resolve())
