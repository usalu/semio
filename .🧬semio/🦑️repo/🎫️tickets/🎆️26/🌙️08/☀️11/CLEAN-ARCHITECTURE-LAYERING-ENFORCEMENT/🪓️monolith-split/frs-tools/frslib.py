"""Shared helpers of the frs (FRAMEWORK-RUST-FILE-SPLITS) tools: paths, coverage-checked planning, staging, multiset proofs."""
import collections, hashlib, json, os, re, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
W = F + "/🗑️generated/frs-work"

TARGETS = {
    "number": dict(
        file="🧰️framework/🔨️modules/🔢️number/🦀️.rs",
        tree="🧰️framework/🔨️modules/🔢️number",
        lib="🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust/🦀️.rs",
        crate="semio_framework_number", pkg="semio-framework-number",
        cfgs=["lib", "test", "wasm", "p2", "wasmtest", "p2test"], features=[]),
    "async": dict(
        file="🧰️framework/🔨️modules/⏳️async/🦀️.rs",
        tree="🧰️framework/🔨️modules/⏳️async",
        lib="🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/🦀️.rs",
        crate="semio_framework_async", pkg="semio-framework-async",
        cfgs=["lib", "test", "wasm", "p2", "wasmtest", "p2test"], features=["testkit", "entrypoint"],
        ext=["semio_framework_value", "semio_framework_trace", "serde", "serde_json", "parking_lot", "semio_framework_async_macros", "js_sys", "wasm_bindgen"]),
    "grammar": dict(
        file="🧰️framework/🔨️modules/🗣️dsl/📖️grammar/🦀️.rs",
        tree="🧰️framework/🔨️modules/🗣️dsl",
        lib="🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust/🦀️.rs",
        crate="semio_framework_dsl", pkg="semio-framework-dsl",
        cfgs=["lib", "test", "wasm", "p2", "wasmtest", "p2test"], features=[],
        ext=["semio_framework_diagnostic", "semio_framework_value", "semio_framework_dsl_lexical", "semio_framework_dsl_token", "semio_framework_async", "semio_framework_async_macros", "serde_json"],
        extra_stub="pub fn lex() {}\npub fn lex_with() {}\npub struct CommentDialect;\npub struct LexOptions;\npub struct StringEscape;\npub struct StringMode;\npub struct SpannedToken;\npub enum TokenKind { Lt, DotEnum, __Stub }\n"),
    "channel": dict(
        file="🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs",
        tree="🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel",
        lib="🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs",
        crate="semio_framework_os_channel", pkg="semio-framework-os-channel",
        cfgs=["lib", "test", "wasm", "p2", "wasmtest", "p2test"], features=[],
        ext=["semio_framework_value", "protocol", "semio_framework_diagnostic", "semio_framework_job", "semio_framework_io_sqlite_snapshot", "semio_framework_io_control", "serde", "serde_json", "semio_framework_trace_memory_testing", "semio_framework_async_macros", "semio_framework_trace"]),
}


def read(path):
    with open(path, encoding="utf-8") as f:
        return f.read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def lines_of(text):
    """Lines without terminators; a trailing newline yields no extra empty element."""
    ls = text.split("\n")
    if ls and ls[-1] == "":
        ls.pop()
    return ls


def dedent(ls, n=4):
    out = []
    for l in ls:
        k = 0
        while k < n and k < len(l) and l[k] == " ":
            k += 1
        out.append(l[k:])
    return out


class Coverage:
    """Every original line (1-based) must be claimed exactly once."""

    def __init__(self, total):
        self.total = total
        self.owner = [None] * (total + 1)

    def claim(self, who, a, b):
        for i in range(a, b + 1):
            assert self.owner[i] is None, f"line {i} claimed by {self.owner[i]} and {who}"
            self.owner[i] = who

    def check(self):
        missing = [i for i in range(1, self.total + 1) if self.owner[i] is None]
        assert not missing, f"unclaimed lines {missing[:20]} ({len(missing)})"


def multiset(ls):
    return collections.Counter(l.strip() for l in ls if l.strip() != "")


def diff_multiset(orig_ls, new_ls):
    a, b = multiset(orig_ls), multiset(new_ls)
    return a - b, b - a


def stage_dir(name):
    return f"{W}/{name}/stage"


def snap_dir(name):
    return f"{W}/{name}/snap"
