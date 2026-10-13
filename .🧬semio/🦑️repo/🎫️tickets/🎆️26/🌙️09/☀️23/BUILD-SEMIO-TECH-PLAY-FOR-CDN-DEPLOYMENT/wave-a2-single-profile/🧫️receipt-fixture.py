#!/usr/bin/env python3
"""🧫️ Independent (Python) oracle that writes the language-agnostic `ComponentReceiptV1` fixture: receipt admission vectors,
deliverable layout vectors, toolchain projection vectors and cargo-provenance digests computed here without the TypeScript
implementation. Usage: `🧫️receipt-fixture.py <fixture path>`."""
import hashlib
import json
import posixpath
import sys

SHA = "a" * 64
BASE = {
    "schema": "semio.plugin.component-receipt/v1",
    "pluginId": "note",
    "packageId": "semio:note",
    "profile": "wasm-release",
    "target": "wasm32-wasip2",
    "wasmSha256": "1" * 64,
    "wasmBytes": 14056513,
    "semanticDescriptorSha256": "2" * 64,
    "toolchain": "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)",
    "sourceTreeDigest": "3" * 64,
    "provenanceSha256": "4" * 64,
}


def case(identifier, valid, **changes):
    value = {**BASE, **changes}
    for key, replacement in list(changes.items()):
        if replacement is None:
            del value[key]
    return {"id": identifier, "valid": valid, "value": value}


RECEIPTS = [
    case("shipped-release", True),
    case("local-iterate", True, profile="wasm-iterate", wasmBytes=314572800),
    case("release-at-shipped-bound", True, wasmBytes=268435456),
    case("release-over-shipped-bound", False, wasmBytes=268435457),
    case("iterate-at-local-cap", True, profile="wasm-iterate", wasmBytes=536870912),
    case("iterate-over-local-cap", False, profile="wasm-iterate", wasmBytes=536870913),
    case("too-small", False, wasmBytes=7),
    case("fractional-bytes", False, wasmBytes=1.5),
    case("unknown-profile", False, profile="wasm-dev"),
    case("foreign-target", False, target="wasm32-unknown-unknown"),
    case("foreign-schema", False, schema="semio.plugin.component-receipt/v2"),
    case("uppercase-digest", False, wasmSha256="A" * 64),
    case("short-digest", False, semanticDescriptorSha256="5" * 63),
    case("toolchain-without-commit", False, toolchain="rustc 1.99.0-nightly"),
    case("malformed-plugin-id", False, pluginId="Note"),
    case("malformed-package-id", False, packageId="note"),
    case("missing-provenance", False, provenanceSha256=None),
    {"id": "unknown-member", "valid": False, "value": {**BASE, "hashes": {"wasmSha256": SHA}}},
]

DELIVERABLES = [
    {"crateRoot": "plugins/🗒️note/📦️packages/🦀️rust", "packageName": "semio-hub-note", "profile": "wasm-release", "directory": "plugins/🗒️note/📦️packages/🦀️rust/dist/components", "wasm": "plugins/🗒️note/📦️packages/🦀️rust/dist/components/semio_hub_note.wasm", "receipt": "plugins/🗒️note/📦️packages/🦀️rust/dist/components/🧾️receipt.json"},
    {"crateRoot": "plugins/🗒️note/📦️packages/🦀️rust", "packageName": "semio-hub-note", "profile": "wasm-iterate", "directory": "plugins/🗒️note/📦️packages/🦀️rust/dist/components-iterate", "wasm": "plugins/🗒️note/📦️packages/🦀️rust/dist/components-iterate/semio_hub_note.wasm", "receipt": "plugins/🗒️note/📦️packages/🦀️rust/dist/components-iterate/🧾️receipt.json"},
]

TOOLCHAIN = [
    {"verbose": "rustc 1.99.0-nightly (9f36de775 2026-07-19)\nbinary: rustc\ncommit-hash: 9f36de775bc636c8e88c31a173c2bcb6995956a0\ncommit-date: 2026-07-19\nhost: aarch64-apple-darwin\nrelease: 1.99.0-nightly\nLLVM version: 22.1.8\n", "identity": "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)"},
    {"verbose": "rustc 1.90.0 (1159e78c4747b02ef996e55082b704c09b970588 2025-09-14)\nbinary: rustc\ncommit-hash: 1159e78c4747b02ef996e55082b704c09b970588\ncommit-date: 2025-09-14\nhost: x86_64-unknown-linux-gnu\nrelease: 1.90.0\nLLVM version: 20.1.8\n", "identity": "rustc 1.90.0 (1159e78c4747b02ef996e55082b704c09b970588)"},
    {"verbose": "rustc 1.99.0-nightly\nbinary: rustc\nrelease: 1.99.0-nightly\n", "identity": None},
]

ROOTS = {"repository": "/work/semio", "build": "/work/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build", "cargoHome": "/home/dev/.cargo", "home": "/home/dev"}
STRINGS = [
    "/work/semio/🧰️framework/🔨️modules/🔏️hash/🦀️.rs",
    "/work/semio/🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust/Cargo.toml",
    "/home/dev/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/quote-1.0.45/src/lib.rs",
    "/work/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/wasm32-wasip2/wasm-release/build/semio-hub-note/out/generated.rs",
    "/home/dev/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/lib.rs",
    "/work/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/wasm32-wasip2/wasm-release/deps/libsemio_framework_hash-0f.rlib",
    "/work/semio/🌎️hub/🧩️compositions/🗒️note/📦️packages/🦀️rust/dist/components/semio_hub_note.wasm",
    "/work/semio/Cargo.lock",
    "/opt/toolchains/wasm-component-ld",
]
OBSERVED = {
    "units": [
        {"inputs": [{"path": STRINGS[0], "kind": "file", "sha256": "0" * 63 + "1"}, {"path": STRINGS[1], "kind": "file", "sha256": "0" * 63 + "2"}, {"path": STRINGS[2], "kind": "file", "sha256": "0" * 63 + "3"}], "artifacts": [{"path": STRINGS[5], "sha256": "f" * 64}]},
        {"inputs": [{"path": STRINGS[3], "kind": "file", "sha256": "0" * 63 + "4"}, {"path": STRINGS[4], "kind": "file", "sha256": "0" * 63 + "5"}, {"path": STRINGS[7], "kind": "file", "sha256": "0" * 63 + "6"}, {"path": STRINGS[0], "kind": "file", "sha256": "0" * 63 + "1"}], "artifacts": [{"path": STRINGS[6], "sha256": "e" * 64}]},
        {"inputs": [{"path": STRINGS[8], "kind": "file", "sha256": None}], "artifacts": []},
    ],
    "buildResources": [
        {"package_id": "path+file:///work/semio/🌎️hub/🧩️compositions/🗒️note/📦️packages/🦀️rust#semio-hub-note@0.1.0", "out_dir": "/work/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/wasm32-wasip2/wasm-release/build/semio-hub-note/out", "path": "/work/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/wasm32-wasip2/wasm-release/build/semio-hub-note/out/semio-runtime-resource-inputs.jsonl", "sha256": "7" * 64, "resources": [{"input": {"kind": "read", "path": STRINGS[0]}, "sha256": "0" * 63 + "1"}]},
    ],
}


def inside(root, path):
    return path != root and path.startswith(root.rstrip("/") + "/")


def normalize(path):
    for name, root in (("$BUILD", ROOTS["build"]), ("$CARGO_HOME", ROOTS["cargoHome"]), ("$REPO", ROOTS["repository"]), ("$HOME", ROOTS["home"])):
        if inside(root, path):
            return name + "/" + posixpath.relpath(path, root)
    return path


def digest(rows):
    ordered = sorted(rows, key=lambda row: "\0".join(row).encode("utf-8"))
    return hashlib.sha256(json.dumps(ordered, separators=(",", ":"), ensure_ascii=False).encode("utf-8")).hexdigest()


def merkle(leaves):
    nodes = [bytes.fromhex(leaf) for leaf in sorted(leaves)]

    def tree(items):
        if len(items) == 1:
            return hashlib.sha256(b"\x00" + items[0]).digest()
        split = 1
        while split * 2 < len(items):
            split *= 2
        return hashlib.sha256(b"\x01" + tree(items[:split]) + tree(items[split:])).digest()

    return (hashlib.sha256(b"").digest() if not nodes else tree(nodes)).hex()


def provenance():
    """Unit leaves (inputs and artifacts), build-script run leaves (capture and resources), the RFC 6962 root over them, and the in-repository sources."""
    leaves, sources = [], {}
    for unit in OBSERVED["units"]:
        rows = [["input", normalize(row["path"]), row["kind"] or "", row["sha256"] or ""] for row in unit["inputs"]] + [["artifact", normalize(row["path"]), row["sha256"] or ""] for row in unit["artifacts"]]
        leaves.append(digest(rows))
        for row in unit["inputs"]:
            if row["kind"] == "file" and row["sha256"] is not None and inside(ROOTS["repository"], row["path"]) and not inside(ROOTS["build"], row["path"]):
                sources[posixpath.relpath(row["path"], ROOTS["repository"])] = row["sha256"]
    for run in OBSERVED["buildResources"]:
        leaves.append(digest([["capture", normalize(run["path"]), run["sha256"]]] + [["resource", normalize(witness["input"]["path"]), witness["input"]["kind"], witness["sha256"] or ""] for witness in run["resources"]]))
    return {"roots": ROOTS, "observed": OBSERVED, "leaves": sorted(leaves), "sourceTreeDigest": digest([[path, sha] for path, sha in sources.items()]), "provenanceSha256": merkle(leaves)}


def main():
    fixture = {"receipts": RECEIPTS, "deliverables": DELIVERABLES, "toolchain": TOOLCHAIN, "provenance": provenance()}
    with open(sys.argv[1], "w", encoding="utf-8") as handle:
        handle.write(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")


if __name__ == "__main__":
    main()
