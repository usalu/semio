"""Shared helpers of the frs (FRAMEWORK-RUST-FILE-SPLITS) tools: paths, coverage-checked planning, staging, multiset proofs."""
import collections, hashlib, json, os, re, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
W = F + "/🗑️generated/pfs-work"

TARGETS = {
    "clearance": dict(
        file="🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🧪️tests/🔬️unit/🦀️.rs",
        tree="🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance",
        lib="🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🦀️.rs",
        crate="semio_framework_pets_clearance", pkg="semio-framework-pets-clearance",
        cfgs=["lib", "test"], features=[],
        ext=["semio_framework_pets_contract", "semio_framework_pets_kinematics", "serde", "serde_json"]),
    "schema": dict(
        file="🧰️framework/🛍️products/🐾️pets/🧬️schema/🦀️.rs",
        tree="🧰️framework/🛍️products/🐾️pets",
        lib="🧰️framework/🛍️products/🐾️pets/🔨️modules/📜️contract/🦀️.rs",
        crate="semio_framework_pets_contract", pkg="semio-framework-pets-contract",
        cfgs=["lib", "test"], features=["testkit"],
        ext=["serde", "serde_json"]),
    "e2e": dict(
        file="🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🦀️.rs",
        tree="🎓️teaching/🛂️proctor",
        lib="🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🦀️.rs",
        crate="end_to_end", pkg="teaching-proctor",
        cfgs=["test"], features=[],
        ext=["proctor", "semio_framework_async", "serde_json", "server", "tungstenite", "ureq", "sha2", "tokio"]),
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
