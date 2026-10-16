"""Shared helpers of the pqs (PACK-QUIZ-FILE-SPLITS) tools: paths, coverage-checked planning, staging, multiset proofs."""
import collections, hashlib, json, os, re, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
W = F + "/🗑️generated/pqs-work"

TARGETS = {
    "value": dict(
        file="🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs",
        tree="🧰️framework/🔨️modules/🎒️pack/🌱️value",
        lib="🧰️framework/🔨️modules/🎒️pack/🌱️value/🦀️.rs",
        crate="semio_framework_pack_record", pkg="semio-framework-pack-record",
        cfgs=["lib", "test", "wasm", "p2", "wasmtest", "p2test"], features=[],
        ext=["semio_framework_async", "semio_framework_pack_format", "semio_framework_value", "semio_framework_dsl_record", "semio_framework_replication_codec", "semio_framework_pack_record_encode", "semio_framework_dsl", "serde_json", "semio_framework_pack_error", "semio_framework_pack_record_schema", "semio_framework_diagnostic", "serde"]),
    "catalog": dict(
        file="🧰️framework/🔨️modules/🎒️pack/📐️format/🗂️catalog/🦀️.rs",
        tree="🧰️framework/🔨️modules/🎒️pack/📐️format",
        lib="🧰️framework/🔨️modules/🎒️pack/📐️format/🗂️catalog/🦀️.rs",
        crate="semio_framework_pack_format_catalog", pkg="semio-framework-pack-format-catalog",
        cfgs=["lib", "test", "wasm", "p2", "wasmtest", "p2test"], features=[],
        ext=["semio_framework_replication_codec", "semio_framework_value", "serde_json", "semio_framework_pack_format_retained", "semio_framework_pack_error", "semio_framework_async_macros", "semio_framework_pack_format", "semio_framework_hash", "semio_framework_pack_source", "semio_framework_trace"]),
    "quizschema": dict(
        file="🧰️framework/🛍️products/❓️quiz/🧬️schema/🦀️.rs",
        tree="🧰️framework/🛍️products/❓️quiz/🧬️schema",
        lib="🧰️framework/🛍️products/❓️quiz/🧬️schema/🦀️.rs",
        crate="quiz_schema", pkg="semio-framework-quiz-schema",
        cfgs=["lib", "test", "wasm", "p2", "wasmtest", "p2test"], features=["testkit"],
        ext=["serde", "serde_json"]),
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
