"""🧾️ Z3 one-off: writes the language-neutral cargo-provenance fixture (Cargo dep-info encoding version 1, hex).

Encoding (cargo `fingerprint/dep_info.rs`): header 01 00 00 00 ff 01, u32 file count, per file u8 kind (0 package-root,
1 build-root) + u32-length path + u8 has-checksum [+ u64 length + u32-length checksum], u32 env count, per variable
u32-length key + u8 has-value [+ u32-length value]. All integers little-endian. Verified against 6 992 real dep-info
files of the shared build-dir (exact end-of-buffer) and a real cargo build (the law's cargo oracle).
Usage: python3 provenance-fixture.py <fixture-path>
"""
import json, struct, sys

def chunk(text):
    data = text.encode("utf-8")
    return struct.pack("<I", len(data)) + data

def encode(files, env):
    out = bytes([1, 0, 0, 0, 0xFF, 1]) + struct.pack("<I", len(files))
    for file in files:
        out += bytes([0 if file["base"] == "package" else 1]) + chunk(file["path"])
        if "checksum" in file:
            out += bytes([1]) + struct.pack("<Q", file["length"]) + chunk(file["checksum"])
        else:
            out += bytes([0])
    out += struct.pack("<I", len(env))
    for key, value in env:
        out += chunk(key) + (bytes([1]) + chunk(value) if value is not None else bytes([0]))
    return out.hex()

POSIX_FILES = [
    {"base": "package", "path": "src/lib.rs", "checksum": "blake3=b6a68f88fd58865dd3c2ba6778bfcfff50d7b738a9741dca882f2e468bb91dfb", "length": 48},
    {"base": "build", "path": "/work/clone/crate/src/../../shared/🦀️.rs", "checksum": "blake3=a6c33bc740e464a290008a693d342116e5c1fdc03c72a3cdb5c3ab7320f5b35c", "length": 30},
    {"base": "build", "path": "/work/semio/.🧬semio/cache/build/debug/build/poison/6d40/out/generated.rs"},
]
WINDOWS_FILES = [
    {"base": "package", "path": "src\\lib.rs"},
    {"base": "build", "path": "C:\\src\\semio\\🧰️framework\\🦀️.rs", "checksum": "sha256=00", "length": 1},
    {"base": "build", "path": "D:\\scratch\\clone\\🧰️framework\\🦀️.rs"},
]
valid = encode(POSIX_FILES, [("CARGO_PKG_REPOSITORY", "https://example.invalid/poison"), ("SEMIO_UNSET", None)])
fixture = {
    "version": 1,
    "decode": [
        {"name": "posix unit with a foreign #[path] source, a build-root output and two tracked variables", "hex": valid,
         "files": [{k: v for k, v in f.items() if k != "length"} for f in POSIX_FILES]},
        {"name": "windows unit with drive-letter sources", "hex": encode(WINDOWS_FILES, []),
         "files": [{k: v for k, v in f.items() if k != "length"} for f in WINDOWS_FILES]},
        {"name": "no files, one tracked variable", "hex": encode([], [("CARGO_PKG_REPOSITORY", "https://github.com/bitvecto-rs/bitvec")]), "files": []},
        {"name": "pre-checksum header (version 0) is refused", "hex": "01000000ff00" + valid[12:], "error": "header"},
        {"name": "legacy unversioned encoding is refused", "hex": "0100000000" + valid[12:], "error": "header"},
        {"name": "truncated path is refused", "hex": valid[:60], "error": "truncated"},
        {"name": "trailing bytes are refused", "hex": valid + "00", "error": "trailing"},
        {"name": "unknown path kind is refused", "hex": "01000000ff01" + "01000000" + "02" + valid[22:], "error": "kind"},
    ],
    "classify": [
        {"name": "posix checkout, cargo home, rustup home", "platform": "linux",
         "roots": ["/work/semio", "/home/dev/.cargo", "/home/dev/.rustup"],
         "paths": ["src/lib.rs", "/work/semio/🧰️framework/🦀️.rs", "/work/semio/.🧬semio/cache/build/debug/out.rs", "/home/dev/.cargo/registry/src/index/serde-1.0.219/src/lib.rs",
                   "/work/semio-clone/🧰️framework/🦀️.rs", "/work/semio/../clone/🦀️.rs", "/tmp/p8-clone/🧰️framework/📦️packages/🦀️rust/../../🔗️causal/🦀️.rs", "/work/semio/📦️packages/../🦀️.rs"],
         "foreign": ["/work/semio-clone/🧰️framework/🦀️.rs", "/work/semio/../clone/🦀️.rs", "/tmp/p8-clone/🧰️framework/📦️packages/🦀️rust/../../🔗️causal/🦀️.rs"]},
        {"name": "macOS paths are case-sensitive byte prefixes", "platform": "darwin",
         "roots": ["/Users/dev/Documents/semio"],
         "paths": ["/Users/dev/Documents/semio/🦀️.rs", "/users/dev/documents/semio/🦀️.rs", "/Users/dev/Documents/semio/"],
         "foreign": ["/users/dev/documents/semio/🦀️.rs"]},
        {"name": "windows drive letters, either separator, case-insensitive", "platform": "win32",
         "roots": ["C:\\src\\semio", "C:\\Users\\dev\\.cargo\\", "C:\\Users\\dev\\.rustup"],
         "paths": ["src\\lib.rs", "c:\\SRC\\Semio\\🧰️framework\\🦀️.rs", "C:/src/semio/🧰️framework/🦀️.rs", "C:\\Users\\dev\\.cargo\\registry\\src\\x.rs",
                   "C:\\src\\semio2\\🦀️.rs", "D:\\scratch\\clone\\🧰️framework\\🦀️.rs", "\\\\server\\share\\semio\\🦀️.rs", "C:\\src\\semio\\..\\clone\\🦀️.rs"],
         "foreign": ["C:\\src\\semio2\\🦀️.rs", "D:\\scratch\\clone\\🧰️framework\\🦀️.rs", "\\\\server\\share\\semio\\🦀️.rs", "C:\\src\\semio\\..\\clone\\🦀️.rs"]},
    ],
}
with open(sys.argv[1], "w", encoding="utf-8") as handle:
    json.dump(fixture, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
