#!/usr/bin/env python3
"""🪓️ gran-value staged carves: anchored, all-or-nothing, dry run by default.

groups:
  dead-edges       12 dead manifest edges around the value hubs (facade, derive, edit, persistent)
  ordered-wire     ordered leaves the intrinsic closure: its ToValue/FromValue impls are mounted by the ordered crate
  derive-resources the owned_json_file proc macro and the compiler-resources edge leave value-derive

usage:
  apply.py [--root DIR] [--group NAME[,NAME]|all] [--diff] [--write] [--status] [--ledger DIR]

Without --write nothing is touched. With --write every anchor of every selected group is validated first; any refusal aborts before the first byte is written, and a failed write rolls all files back from the ledger backup.
"""
import argparse, difflib, hashlib, json, os, re, sys, time

DEFAULT_ROOT = "/Users/ueli/Documents/semio"
V = "🧰️framework/🔨️modules/🌱️value"
FW = "🧰️framework"
DERIVE = f"{V}/✨️derive"
SNAP = f"{DERIVE}/🚪️io/📝️text/📸️snapshot"
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"


class Refusal(Exception):
    """🚫️ An anchor, count or checksum did not match the tree; nothing is written."""


class Plan:
    """🗂️ In-memory edit set: original texts, planned texts, created files."""

    def __init__(self, root):
        self.root = root
        self.orig = {}
        self.new = {}

    def abs(self, rel):
        return os.path.join(self.root, rel)

    def text(self, rel):
        if rel not in self.new:
            p = self.abs(rel)
            if not os.path.isfile(p):
                raise Refusal(f"missing file {rel}")
            with open(p, encoding="utf-8") as f:
                self.orig[rel] = self.new[rel] = f.read()
        return self.new[rel]

    def put(self, rel, text):
        self.text(rel)
        self.new[rel] = text

    def create(self, rel, text):
        if os.path.exists(self.abs(rel)) or rel in self.new:
            raise Refusal(f"file already exists {rel}")
        self.orig[rel] = None
        self.new[rel] = text

    def changed(self):
        return {r: t for r, t in self.new.items() if t != self.orig.get(r)}


def replace_once(plan, rel, old, new):
    """✂️ Exact single replacement; the anchor must occur exactly once."""
    text = plan.text(rel)
    n = text.count(old)
    if n != 1:
        raise Refusal(f"{rel}: anchor occurs {n} times, expected 1: {old[:90]!r}")
    plan.put(rel, text.replace(old, new, 1))


def replace_all(plan, rel, old, new, count):
    """✂️ Exact replacement with a required occurrence count."""
    text = plan.text(rel)
    n = text.count(old)
    if n != count:
        raise Refusal(f"{rel}: anchor occurs {n} times, expected {count}: {old[:90]!r}")
    plan.put(rel, text.replace(old, new))


def cut_between(plan, rel, start, end, sha):
    """✂️ Removes text from the start anchor up to the end anchor after checking its checksum; returns the removed text."""
    text = plan.text(rel)
    if text.count(start) != 1 or text.count(end) != 1:
        raise Refusal(f"{rel}: cut anchors not unique ({text.count(start)}, {text.count(end)})")
    a, b = text.index(start), text.index(end)
    if b < a:
        raise Refusal(f"{rel}: cut anchors out of order")
    got = hashlib.sha256(text[a:b].encode()).hexdigest()
    if got != sha:
        raise Refusal(f"{rel}: cut block drifted (sha256 {got[:16]} != {sha[:16]})")
    plan.put(rel, text[:a] + text[b:])
    return text[a:b]


def section(lines, header):
    """📑️ Index range of a TOML table body, header line excluded."""
    hits = [i for i, l in enumerate(lines) if l.strip() == header]
    if len(hits) != 1:
        raise Refusal(f"table {header} occurs {len(hits)} times")
    a = hits[0] + 1
    b = next((i for i in range(a, len(lines)) if lines[i].lstrip().startswith("[")), len(lines))
    return a, b


def dep_index(lines, header, name):
    """🔎️ Index of the single-line dependency entry `name = ...` inside a table."""
    a, b = section(lines, header)
    rx = re.compile(r"^" + re.escape(name) + r"\s*=\s*(\{.*\}|\".*\")\s*$")
    hits = [i for i in range(a, b) if rx.match(lines[i])]
    if len(hits) != 1:
        raise Refusal(f"{header} {name}: {len(hits)} single-line entries, expected 1")
    return hits[0]


def dep_remove(plan, rel, header, name):
    """➖ Removes one dependency entry."""
    lines = plan.text(rel).split("\n")
    del lines[dep_index(lines, header, name)]
    plan.put(rel, "\n".join(lines))


def dep_add_after(plan, rel, header, after, line):
    """➕ Inserts a dependency entry below another one; the key must be absent."""
    lines = plan.text(rel).split("\n")
    key = line.split("=")[0].strip()
    a, b = section(lines, header)
    if any(re.match(r"^" + re.escape(key) + r"\s*=", lines[i]) for i in range(a, b)):
        raise Refusal(f"{rel}: {key} already declared")
    lines.insert(dep_index(lines, header, after) + 1, line)
    plan.put(rel, "\n".join(lines))


def member_add_after(plan, rel, after, member):
    """➕ Inserts a workspace member below another one."""
    line = f'    "{after}",'
    replace_once(plan, rel, line + "\n", line + "\n" + f'    "{member}",\n')


def pkg(rel_dir):
    return f"{rel_dir}/📦️packages/🦀️rust/Cargo.toml"


FACADE = pkg(V)
DERIVE_PKG = pkg(DERIVE)
EDIT_PKG = pkg(f"{V}/📋️list/🔺️edit")
PERSISTENT_PKG = pkg(f"{V}/📋️list/🔗️persistent")
INTRINSIC_PKG = pkg(f"{V}/🌳️intrinsic")
ORDERED_PKG = pkg(f"{V}/🗂️ordered")
PRINT_PKG = pkg(f"{FW}/🛍️products/📓️print")
RESOURCES_PKG = pkg(SNAP)
DEPS = "[dependencies]"
DEV = "[dev-dependencies]"


def dead_edges(plan):
    """🕳️ Twelve manifest edges with no textual reference in the module tree of their crate, tests and bin roots included."""
    for name in ["semio-framework-value-clone-source", "semio-framework-value-native-decoding", "semio-framework-value-paged-list", "semio-framework-value-allocation-return", "semio-framework-hash", "semio-framework-value-edit", "semio-framework-value-schema", "semio-framework-value-registry", "semio-framework-schema-registry"]:
        dep_remove(plan, FACADE, DEPS, name)
    dep_remove(plan, DERIVE_PKG, DEPS, "semio-framework-hash")
    dep_remove(plan, EDIT_PKG, DEPS, "semio-framework-value-native-encoding")
    dep_remove(plan, PERSISTENT_PKG, DEPS, "semio-framework-value-original-aliases")


ORDERED_LIB = f"{V}/🗂️ordered/📦️packages/🦀️rust/🦀️.rs"
INTRINSIC_LIB = f"{V}/🌳️intrinsic/📦️packages/🦀️rust/🦀️.rs"
ORDERED_WIRE = f"{V}/🗂️ordered/🌐️wire/🦀️.rs"
SET_WIRE = f"{V}/🗂️ordered/🧺️set/🌐️wire/🦀️.rs"


def ordered_wire(plan):
    """🗂️ The ordered crate owns its wire impls: intrinsic drops the ordered edge, ordered gains the intrinsic edge."""
    replace_once(plan, INTRINSIC_LIB, '#[path="../../../🗂️ordered/🌐️wire/🦀️.rs"]\nmod ordered_value_wire;\n', "")
    dep_remove(plan, INTRINSIC_PKG, DEPS, "semio-framework-value-ordered")
    dep_add_after(plan, ORDERED_PKG, DEPS, "semio-framework-value-native-decoding", 'semio-framework-value-intrinsic = { path="../../../🌳️intrinsic/📦️packages/🦀️rust" }')
    replace_once(plan, ORDERED_LIB, "pub use ordered::*;\n", "pub use ordered::*;\nuse semio_framework_value_intrinsic::{DecodedValue,DslValue,FromValue,NativeEncodeControl,ToValue};\n#[path=\"../../🌐️wire/🦀️.rs\"]\nmod ordered_value_wire;\n")
    replace_once(plan, ORDERED_WIRE, "use semio_framework_value_ordered::ordered::*;\n", "use crate::ordered::*;\n")
    replace_once(plan, SET_WIRE, "use semio_framework_value_ordered::ordered::{OrderedMap,OrderedSet};\n", "use crate::ordered::{OrderedMap,OrderedSet};\n")


DERIVE_LIB = f"{DERIVE}/🦀️.rs"
SNAP_LIB = f"{SNAP}/🦀️.rs"
OWNED_JSON_TEST = f"{DERIVE}/🧪️tests/📚️owned-json/🦀️.rs"
FW_MANIFEST = f"{FW}/Cargo.toml"
PRINT_FILES = [f"{FW}/🛍️products/📓️print/🚪️io/📝️text/💡️inferences/🦀️.rs", f"{FW}/🛍️products/📓️print/🧬️schema/💡️inferences/🦀️.rs", f"{FW}/🛍️products/📓️print/🧬️schema/💡️inferences/🎨theme/🦀️.rs"]

RESOURCES_MANIFEST = '''[package]
workspace = "../../../../../../../.."
name = "semio-framework-value-derive-resources"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Proc-macro compiling source-authored JSON assets into owned intrinsic values, with compiler resource capture and tracked paths."

[package.metadata.semio]
role = "framework"
id = "value-derive-resources"

[lints]
workspace = true

[lib]
name = "semio_framework_value_derive_resources"
proc-macro = true
path = "../../🦀️.rs"

[dependencies]
syn = { version = "2.0", features = ["full", "extra-traits", "visit", "visit-mut"] }
quote = "1.0"
proc-macro2 = "1.0"
semio-framework-native-build-compiler-resources = { path = "../../../../../../../🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/📦️packages/🦀️rust" }
'''

RESOURCES_ENTRY = '''/// 📚️ Compiles a package-relative source JSON asset into an owned intrinsic value.
#[proc_macro]
pub fn owned_json_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    observe_compiler_resources(|| {
        let path = syn::parse_macro_input!(input as syn::LitStr);
        expand(&path).unwrap_or_else(|error| error.to_compile_error()).into()
    })
}

'''


def derive_resources(plan, shas):
    """📚️ owned_json_file becomes its own proc-macro crate; value-derive no longer needs nightly tracked-path features or the compiler-resources crate."""
    head = cut_between(plan, DERIVE_LIB, "#![feature(proc_macro_tracked_path, proc_macro_tracked_env)]\n", "use semio_framework_value_derive_expansion as component;\n", shas["head"])
    replace_once(plan, DERIVE_LIB, '#[path = "🚪️io/📝️text/📸️snapshot/🦀️.rs"]\nmod owned_json;\n\n', "")
    entry = cut_between(plan, DERIVE_LIB, "/// 📚️ Compiles a package-relative source JSON asset into an owned intrinsic value.\n", "/// 🗃️ Implements `value::ToValue` for a `#[value(...)]`-annotated struct or enum.\n", shas["entry"])
    feature, rest = head.split("\n", 1)
    use_line, fn_block = rest.lstrip("\n").split("\n\n", 1)
    first, body = plan.text(SNAP_LIB).split("\n", 1)
    plan.put(SNAP_LIB, first + "\n" + feature + "\n" + body)
    replace_once(plan, SNAP_LIB, "pub(crate) fn expand(path: &LitStr) -> syn::Result<TokenStream> {\n", use_line + "\n\n" + RESOURCES_ENTRY + fn_block.rstrip("\n") + "\n\npub(crate) fn expand(path: &LitStr) -> syn::Result<TokenStream> {\n")
    if "pub fn owned_json_file(input: TokenStream)" not in entry:
        raise Refusal("derive entry block is not the owned_json_file macro")
    plan.create(RESOURCES_PKG, RESOURCES_MANIFEST)
    dep_remove(plan, DERIVE_PKG, DEPS, "semio-framework-native-build-compiler-resources")
    dep_add_after(plan, DERIVE_PKG, DEV, "semio-framework-value", 'semio-framework-value-derive-resources = { path = "../../🚪️io/📝️text/📸️snapshot/📦️packages/🦀️rust" }')
    replace_once(plan, OWNED_JSON_TEST, 'semio_framework_value_derive::owned_json_file!("../../🧫️fixtures/📚️owned-json/🔣️.json")', 'semio_framework_value_derive_resources::owned_json_file!("../../🧪️tests/🧫️fixtures/📚️owned-json/🔣️.json")')
    member_add_after(plan, FW_MANIFEST, "🔨️modules/🌱️value/✨️derive/⚙️expansion/📦️packages/🦀️rust", "🔨️modules/🌱️value/✨️derive/🚪️io/📝️text/📸️snapshot/📦️packages/🦀️rust")
    dep_add_after(plan, PRINT_PKG, DEPS, "semio-framework-value-derive", 'semio-framework-value-derive-resources = { path = "../../../../🔨️modules/🌱️value/✨️derive/🚪️io/📝️text/📸️snapshot/📦️packages/🦀️rust" }')
    for f in PRINT_FILES:
        text = plan.text(f)
        n = text.count("semio_framework_value_derive::owned_json_file!")
        if n < 1:
            raise Refusal(f"{f}: no owned_json_file call")
        plan.put(f, text.replace("semio_framework_value_derive::owned_json_file!", "semio_framework_value_derive_resources::owned_json_file!"))
    total = sum(plan.orig[f].count("semio_framework_value_derive::owned_json_file!") for f in PRINT_FILES)
    if total != 6:
        raise Refusal(f"print owned_json_file call sites: {total}, expected 6")


def load_shas():
    p = os.path.join(os.path.dirname(os.path.abspath(__file__)), "anchors.json")
    with open(p, encoding="utf-8") as f:
        return json.load(f)


GROUPS = {
    "dead-edges": lambda plan, shas: dead_edges(plan),
    "ordered-wire": lambda plan, shas: ordered_wire(plan),
    "derive-resources": lambda plan, shas: derive_resources(plan, shas),
}


def build(root, names):
    plan = Plan(root)
    shas = load_shas()
    for n in names:
        GROUPS[n](plan, shas)
    return plan


def done(root, name):
    """✅️ True when the group's end state is already on disk."""
    marks = {
        "dead-edges": lambda: "semio-framework-value-registry" not in open(os.path.join(root, FACADE), encoding="utf-8").read(),
        "ordered-wire": lambda: "semio-framework-value-intrinsic" in open(os.path.join(root, ORDERED_PKG), encoding="utf-8").read(),
        "derive-resources": lambda: os.path.isfile(os.path.join(root, RESOURCES_PKG)),
    }
    try:
        return marks[name]()
    except OSError:
        return False


def write(plan, ledger):
    """💾️ Atomic per-file write with first-touch backups and rollback on any failure."""
    os.makedirs(ledger, exist_ok=True)
    book = {}
    done_ = []
    try:
        for rel, text in plan.changed().items():
            p = plan.abs(rel)
            h = hashlib.sha1(rel.encode()).hexdigest()[:16]
            if plan.orig[rel] is not None:
                with open(os.path.join(ledger, h), "w", encoding="utf-8") as f:
                    f.write(plan.orig[rel])
            book[rel] = {"backup": h if plan.orig[rel] is not None else None, "created": plan.orig[rel] is None}
            os.makedirs(os.path.dirname(p), exist_ok=True)
            tmp = p + ".gran-value.tmp"
            with open(tmp, "w", encoding="utf-8") as f:
                f.write(text)
            os.replace(tmp, p)
            done_.append(rel)
        for rel, text in plan.changed().items():
            with open(plan.abs(rel), encoding="utf-8") as f:
                if f.read() != text:
                    raise OSError(f"read-back mismatch {rel}")
    except Exception:
        for rel in done_:
            p = plan.abs(rel)
            if book[rel]["created"]:
                os.remove(p)
            else:
                with open(os.path.join(ledger, book[rel]["backup"]), encoding="utf-8") as f:
                    with open(p, "w", encoding="utf-8") as g:
                        g.write(f.read())
        raise
    with open(os.path.join(ledger, "ledger.json"), "w", encoding="utf-8") as f:
        json.dump(book, f, ensure_ascii=False, indent=1)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=DEFAULT_ROOT)
    ap.add_argument("--group", default="all")
    ap.add_argument("--diff", action="store_true")
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--status", action="store_true")
    ap.add_argument("--ledger", default=None)
    a = ap.parse_args()
    names = list(GROUPS) if a.group == "all" else a.group.split(",")
    for n in names:
        if n not in GROUPS:
            sys.exit(f"unknown group {n}")
    if a.status:
        for n in names:
            print(f"{n}: {'applied' if done(a.root, n) else 'pending'}")
        return 0
    try:
        plan = build(a.root, names)
    except Refusal as e:
        print(f"REFUSED: {e}")
        return 2
    ch = plan.changed()
    for rel, text in sorted(ch.items()):
        old = plan.orig[rel]
        if old is None:
            print(f"  create {rel} ({len(text.splitlines())} lines)")
        else:
            d = list(difflib.unified_diff(old.splitlines(), text.splitlines(), lineterm="", n=0))
            plus = sum(1 for l in d if l.startswith("+") and not l.startswith("+++"))
            minus = sum(1 for l in d if l.startswith("-") and not l.startswith("---"))
            print(f"  edit   {rel} +{plus} -{minus}")
            if a.diff:
                print("\n".join(d))
    print(f"{len(ch)} files, groups {names}, {'WRITE' if a.write else 'dry run'}")
    if a.write:
        ledger = a.ledger or os.path.join(a.root, TICKET, "🗑️generated", "gran-value-apply", time.strftime("%Y%m%d-%H%M%S"))
        write(plan, ledger)
        print(f"written; ledger {ledger}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
