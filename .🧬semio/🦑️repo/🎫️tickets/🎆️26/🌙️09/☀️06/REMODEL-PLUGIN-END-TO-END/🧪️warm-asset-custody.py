"""🧪️ Links the asset custody law to the exact native crates already built by Cargo."""

import json
import os
from pathlib import Path
import subprocess
import sys
import tomllib


def dependency_receipts(build, fingerprint):
    """🧾 Resolves every alias to the artifact in the selected Cargo dependency graph."""
    receipts = {}
    for _, alias, _, value in fingerprint["deps"]:
        expected = value.to_bytes(8, "little").hex()
        for dep in (build / alias.replace("_", "-")).glob(f"*/fingerprint/lib-{alias}"):
            if dep.read_text() == expected:
                receipts[alias] = dep.with_suffix(".json")
                break
        assert alias in receipts, alias
    return receipts


def append_extern(command, alias, path):
    """📦️ Supplies the warmed library and its separately recorded metadata."""
    for artifact in (path.with_suffix(".rmeta"), path) if path.suffix == ".rlib" else (path,):
        if artifact.exists():
            command.extend(("--extern", f"{alias}={artifact}"))


def warmed_library(receipt, alias):
    """🔍️ Finds either the native Rust library or the compiled derive macro."""
    unit = receipt.parent.parent
    return next(path for path in (unit / "out").glob(f"lib{alias}-{unit.name}.*") if path.suffix in (".rlib", ".so"))


def main():
    """🔗️ Uses Cargo's recorded dependency hashes to avoid recompiling the native graph."""
    ticket = Path(__file__).resolve().parent
    root = ticket.parents[6]
    output = ticket / "🗑️generated"
    build = output / "rust-build/debug/build"
    aggregate = "semio_s_artifact_remodel_remodeling_mutations"
    receipt = build / aggregate.replace("_", "-") / "f61153ffc197bb8c" / "fingerprint" / f"lib-{aggregate}.json"
    fingerprint = json.loads(receipt.read_text())
    deps = dependency_receipts(build, fingerprint)
    externs = {aggregate: warmed_library(receipt, aggregate)}
    for alias in ("semio_os_kernel", "semio_framework_pack_json", "semio_s_artifact_remodel_remodeling_model"):
        externs[alias] = warmed_library(deps[alias], alias)
    assert len(externs) == 4 and all(path.exists() for path in externs.values()), externs
    manifest = root / "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️media-2/📦️packages/🦀️rust"
    source = manifest / "🧪️tests/🔬️assets/🧪️.rs"
    executable = output / ("asset-custody-native-before" if "--before" in sys.argv else "asset-custody-native")
    compiler = Path.home() / ".rustup/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc"
    common = [compiler, "--edition=2021", "-C", "debuginfo=0", *fingerprint["rustflags"], "-L", f"dependency={output / 'dsl-probe-dependencies'}", "-L", f"dependency={output}"]
    if "--before" not in sys.argv:
        media = "semio_s_artifact_remodel_remodeling_mut_media"
        leaf = media + "_2"
        media_receipt = deps[media]
        leaf_receipt = dependency_receipts(build, json.loads(media_receipt.read_text()))[leaf]
        mutations = manifest.parents[2]
        compiled = {}
        for alias, current, path in ((leaf, leaf_receipt, mutations / "🪜️media-2"), (media, media_receipt, mutations / "🎞️media"), (aggregate, receipt, mutations)):
            library = output / f"lib{alias}-custody.rlib"
            crate_manifest = path / "📦️packages/🦀️rust"
            crate_config = tomllib.loads((crate_manifest / "Cargo.toml").read_text())
            crate_source = (crate_manifest / crate_config["lib"]["path"]).resolve()
            flags = [*common, "--crate-name", alias, "--crate-type", "lib", "--emit=metadata,link", "-C", "metadata=custody", "-C", "extra-filename=-custody", "--out-dir", output, crate_source]
            for dependency, dep in dependency_receipts(build, json.loads(current.read_text())).items():
                append_extern(flags, dependency, compiled.get(dependency) or warmed_library(dep, dependency))
            subprocess.run(flags, check=True, env={**os.environ, "CARGO_MANIFEST_DIR": str(crate_manifest), "CARGO_PKG_VERSION": "0.1.0"})
            compiled[alias] = library
        externs[aggregate] = compiled[aggregate]
    command = [*common, "--test", "--crate-name", "asset_custody", source, "-o", executable]
    for alias, path in externs.items():
        append_extern(command, alias, path)
    subprocess.run(command, check=True)
    return subprocess.run([executable]).returncode


if __name__ == "__main__":
    sys.exit(main())
