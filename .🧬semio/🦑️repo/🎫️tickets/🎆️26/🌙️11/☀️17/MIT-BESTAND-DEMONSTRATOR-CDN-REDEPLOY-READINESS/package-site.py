"""📦️ Package and verify a site that passed CDN acceptance."""
import argparse
from datetime import datetime, timezone
from pathlib import Path
import runpy
import tarfile
from tempfile import NamedTemporaryFile, TemporaryDirectory


TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
ROOT = next(parent for parent in TICKET.parents if (parent / "♻️mit-bestand/🧺️demonstrator/📋️project.json").is_file())
PAGES_MAX_PUBLISHED_BYTES = 1_000_000_000


def check_pages_bytes(size):
    if size > PAGES_MAX_PUBLISHED_BYTES:
        raise ValueError(f"Published site is {size:,} bytes. GitHub Pages allows {PAGES_MAX_PUBLISHED_BYTES:,} bytes.")


def package(site, output, accepted):
    if not accepted.is_file():
        raise ValueError(f"No accepted manifest at {accepted}. Run strict acceptance with accept-site.py.")
    if output.is_relative_to(site):
        raise ValueError("Archive output must be outside the accepted site")
    for name in ("index.html", "CNAME", ".nojekyll"):
        if not (site / name).is_file():
            raise FileNotFoundError(site / name)
    scan = runpy.run_path(str(TICKET / "validate-local-references.py"))["scan"]
    failures = scan(site)
    if failures:
        raise ValueError("\n".join(failures))
    make_manifest = runpy.run_path(str(TICKET / "site-manifest.py"))["write_manifest"]
    verify_archive = runpy.run_path(str(TICKET / "verify-release-archive.py"))["verify"]
    GENERATED.mkdir(exist_ok=True)
    with TemporaryDirectory(dir=GENERATED, prefix="pkg-") as scratch:
        manifest = Path(scratch) / "SHA256SUMS"
        _, size = make_manifest(site, ROOT, manifest)
        if accepted.read_bytes() != manifest.read_bytes():
            raise ValueError("Site does not match the accepted manifest")
        check_pages_bytes(size + manifest.stat().st_size)
        output.mkdir(parents=True, exist_ok=True)
        with NamedTemporaryFile(dir=output, prefix=".site-", delete=False) as file:
            temporary = Path(file.name)
        try:
            with tarfile.open(temporary, "w:gz", compresslevel=6) as archive:
                for path in sorted(site.iterdir()):
                    if path.name != "SHA256SUMS":
                        archive.add(path, arcname=f"./{path.name}")
                archive.add(manifest, arcname="./SHA256SUMS")
            count, size = verify_archive(temporary, accepted)
            print(f"Verified {count} files and {size} bytes in the release archive")
            timestamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
            packaged = output / f"site-{timestamp}.tar.gz"
            temporary.replace(packaged)
        finally:
            temporary.unlink(missing_ok=True)
    return packaged


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("site", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("accepted", type=Path)
    args = parser.parse_args()
    try:
        print(f"archive: {package(args.site.resolve(), args.output.resolve(), args.accepted.resolve())}")
    except (OSError, ValueError, tarfile.TarError) as error:
        raise SystemExit(str(error))
