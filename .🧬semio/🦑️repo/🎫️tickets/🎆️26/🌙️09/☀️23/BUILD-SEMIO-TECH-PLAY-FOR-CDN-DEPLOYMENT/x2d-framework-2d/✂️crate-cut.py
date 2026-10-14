#!/usr/bin/env python3
"""✂️ Cuts `semio-framework-2d` into one crate per taxonomy facet of `◻️2d` (lane X2D).

Stages (each idempotent, `plan` prints, `apply` writes):
- `data`: splits the Unicode shaping record `🌐️unicode/📦️data/🔢️properties.bin` at its first table boundary into the
  `🏷️properties` (record head + property ranges) and `🔁️mappings` (decompositions, compositions, scalars, mirrors,
  brackets) asset files and writes their asset crate roots; the concatenation is verified against the original digest.
- `sources`: rewrites crate-relative paths in the facet roots to the owning facet crates, widens the font items the
  shape, unicode and run crates name, and drops the mounts that become crate roots.
- `tests`: distributes the monolithic font unit-test file to the crate that owns what each test exercises and moves its
  shared helpers to the dev-only `semio-framework-2d-text-font-testing` crate.
- `manifests`: writes every facet crate manifest and deletes the old crate root and manifest.

usage: ✂️crate-cut.py data|sources|tests|manifests plan|apply
"""
import hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
FW = os.path.join(REPO, "🧰️framework")
D2 = os.path.join(FW, "🔨️modules/◻️2d")
FONT = os.path.join(D2, "📝️text/🔤️font")
UNICODE = os.path.join(FONT, "🧵️shape/🌐️unicode")
stage, mode = sys.argv[1], sys.argv[2]
CRATES = json.load(open(os.path.join(HERE, "🧩️crates.json"), encoding="utf-8"))
written = []


def write(path, text, binary=False):
    """💾️ Writes (apply) or reports (plan) one output."""
    written.append(os.path.relpath(path, REPO))
    if mode == "apply":
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "wb" if binary else "w", **({} if binary else {"encoding": "utf-8"})).write(text)


def remove(path):
    """🗑️ Removes (apply) or reports (plan) one input."""
    if os.path.exists(path):
        written.append("- " + os.path.relpath(path, REPO))
        if mode == "apply":
            os.remove(path)


def edit(path, pairs):
    """✏️ Applies exact ordered replacements; every pair must hit at least once."""
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if isinstance(old, re.Pattern):
            text, count = old.subn(new, text)
        else:
            count = text.count(old)
            text = text.replace(old, new)
        if not count:
            raise SystemExit(f"no hit in {os.path.relpath(path, REPO)}: {old!r}")
    write(path, text)
    return text


def data():
    source = os.path.join(UNICODE, "📦️data/🔢️properties.bin")
    if not os.path.exists(source):
        print("already split")
        return
    blob = open(source, "rb").read()
    boundary = int.from_bytes(blob[20:24], "big")
    if int.from_bytes(blob[12:16], "big") != 64 or boundary != 64 + int.from_bytes(blob[8:12], "big") * 24:
        raise SystemExit("unexpected Unicode record layout")
    head, tail = blob[:boundary], blob[boundary:]
    assert hashlib.sha256(head + tail).digest() == hashlib.sha256(blob).digest()
    for folder, name, const, part, doc in [
        ("🏷️properties", "🔢️properties.bin", "PROPERTIES", head, "🏷️ Original Unicode shaping record head and per-scalar property ranges"),
        ("🔁️mappings", "🔢️mappings.bin", "MAPPINGS", tail, "🔁️ Original Unicode decomposition, composition, scalar, mirroring and bracket tables"),
    ]:
        base = os.path.join(UNICODE, "📦️data", folder)
        write(os.path.join(base, name), part, binary=True)
        write(os.path.join(base, "🦀️.rs"), f'//! {doc}, written by `provisionUnicodeShapeTables` as one part of the immutable record that\n//! `semio-framework-2d-text-font-shape-unicode` composes (`🌐️unicode/📦️data/🔣️provenance.json`).\npub const {const}:&[u8;include_bytes!("{name}").len()]=include_bytes!("{name}");\n')
    remove(source)
    print(f"boundary {boundary}: properties {len(head)} B, mappings {len(tail)} B, sha256 {hashlib.sha256(blob).hexdigest()}")


E, R, FL, B = "semio_framework_2d_engine", "semio_framework_2d_retire", "semio_framework_2d_path_flatten", "semio_framework_2d_booleans"
F, U, S = "semio_framework_2d_text_font", "semio_framework_2d_text_font_shape_unicode", "semio_framework_2d_text_font_shape"
GEOMETRY = [
    ("crate::engine::", E + "::"), ("crate::retirement::", R + "::"), ("crate::physical_work_retirement", R + "::physical_work_retirement"),
    ("crate::flatten::", FL + "::"), ("crate::booleans::", B + "::"),
]


def geometry(path, extra=()):
    text = open(path, encoding="utf-8").read()
    pairs = [(old, new) for old, new in GEOMETRY if old in text] + list(extra)
    pairs += [(re.compile(r"crate::(\{\s*PathSegment,\s*Vec2\s*\}|PathSegment\b|Vec2\b)"), E + r"::\1")] if re.search(r"crate::(\{\s*PathSegment|PathSegment|Vec2)", text) else []
    edit(path, pairs)


def sources():
    edit(os.path.join(D2, "⚙️engine/🦀️.rs"), [
        ("//! by the [`crate::booleans`]/[`crate::trace`] pure-function kernels and by any external caller", "//! by the `semio-framework-2d-booleans`/`semio-framework-2d-trace` pure-function kernels and by any external caller"),
    ])
    edit(os.path.join(D2, "🧹️retire/🦀️.rs"), [("$crate::retirement::WorkRetirementProgress", "$crate::WorkRetirementProgress")])
    for facet in ["🛤️path/📏️flatten/🦀️.rs", "🛤️path/🖊️stroke/🦀️.rs", "🔀️booleans/🦀️.rs", "🔀️booleans/🛤️paths/🦀️.rs", "🔀️booleans/🛤️paths/🧪️tests/🦀️.rs", "🧱️regions/🦀️.rs", "🔍️trace/🦀️.rs", "📝️text/🔤️font/🧵️outline/🦀️.rs"]:
        geometry(os.path.join(D2, facet))
    edit(os.path.join(D2, "📝️text/🦀️.rs"), [('\n#[path="🔤️font/🦀️.rs"] pub mod font;', "")])
    edit(os.path.join(FONT, "🦀️.rs"), [
        ("pub(crate) struct FontReader<'a>", "pub struct FontReader<'a>"),
        ("pub(crate) fn reserve_slot<T>", "pub fn reserve_slot<T>"),
        ('\n#[path="🧵️run/🦀️.rs"]\npub mod run;', ""), ('\n#[path="📇️catalog/🦀️.rs"]\npub mod catalog;', ""), ('\n#[path="🧵️shape/🦀️.rs"]\npub mod shape;', ""),
    ])
    edit(os.path.join(FONT, "🤝️kerning/🦀️.rs"), [("pub(crate) struct Index", "pub struct Index"), (re.compile(r"pub\(crate\) fn (new|step|result)\b"), r"pub fn \1")])
    edit(os.path.join(FONT, "📇️catalog/🦀️.rs"), [("use super::{AdmittedFont,", f"use {F}::{{AdmittedFont,")])
    run = os.path.join(FONT, "🧵️run/🦀️.rs")
    edit(run, [
        ("use super::{AdmittedFont,FontSource,", f"use {F}::{{AdmittedFont,FontSource,"),
        ("use super::shape::{FontLookup,FontLookupPlanCursor,mark::FontMarkCursor,face::{FontClusterFaceCursor,FontClusterMapping,FontMappedScalar},layout::FontClusterLayoutCursor,unicode::{UnicodeLayout,normalize::{UnicodeNormalizationCursor,NormalizedScalar},cluster::{UnicodeGraphemeCursor,UnicodeCluster}}};",
         f"use {S}::{{FontLookup,FontLookupPlanCursor,mark::FontMarkCursor,face::{{FontClusterFaceCursor,FontClusterMapping,FontMappedScalar}},layout::FontClusterLayoutCursor}};\nuse {U}::{{UnicodeLayout,normalize::{{UnicodeNormalizationCursor,NormalizedScalar}},cluster::{{UnicodeGraphemeCursor,UnicodeCluster}}}};"),
        ("use super::shape::class::", f"use {S}::class::"), ("use super::shape::unicode::", f"use {U}::"),
        ("use crate::PathSegment;", f"use {E}::PathSegment;"),
    ])
    shape = os.path.join(FONT, "🧵️shape")
    edit(os.path.join(shape, "🦀️.rs"), [("use super::{FontFace,FontProgress,FontReader,reserve_slot};", f"use {F}::{{FontFace,FontProgress,FontReader,reserve_slot}};"), ('\n#[path="🌐️unicode/🦀️.rs"]\npub mod unicode;', "")])
    for facet in ["🔗️ligature", "🪄️substitute", "🧩️context"]:
        edit(os.path.join(shape, facet, "🦀️.rs"), [("use super::{FontLookup,super::{FontFace,FontProgress,FontReader,kerning::Index}};", f"use super::FontLookup;\nuse {F}::{{FontFace,FontProgress,FontReader,kerning::Index}};")])
    edit(os.path.join(shape, "📍️mark/🦀️.rs"), [("use super::super::{FontFace,FontProgress,FontReader,kerning::Index};", f"use {F}::{{FontFace,FontProgress,FontReader,kerning::Index}};")])
    edit(os.path.join(shape, "🏷️class/🦀️.rs"), [("use super::super::{FontReader,FontFace,kerning::Index};", f"use {F}::{{FontReader,FontFace,kerning::Index}};")])
    edit(os.path.join(shape, "📇️face/🦀️.rs"), [
        ("use super::{super::{AdmittedFont,FontSource,FontFace,FontOutlineLimits,FontProgress},unicode::{UnicodeLayout,UnicodeProperty,normalize::NormalizedScalar,cluster::UnicodeCluster}};",
         f"use {F}::{{AdmittedFont,FontSource,FontFace,FontOutlineLimits,FontProgress}};\nuse {U}::{{UnicodeLayout,UnicodeProperty,normalize::NormalizedScalar,cluster::UnicodeCluster}};"),
        ("super::super::reserve_slot", f"{F}::reserve_slot"),
    ])
    edit(os.path.join(shape, "📇️face/🧵️scalar/🦀️.rs"), [
        ("use super::{FontMappedScalar,super::{super::{FontFace,FontCmapCursor,FontReader,FontProgress},unicode::{UnicodeLayout,UnicodeMapping,UnicodeMappingCursor,UnicodePropertyCursor,normalize::NormalizedScalar}}};",
         f"use super::FontMappedScalar;\nuse {F}::{{FontFace,FontCmapCursor,FontReader,FontProgress}};\nuse {U}::{{UnicodeLayout,UnicodeMapping,UnicodeMappingCursor,UnicodePropertyCursor,normalize::NormalizedScalar}};"),
    ])
    edit(os.path.join(shape, "🧵️layout/🦀️.rs"), [
        ("face::{FontClusterMapping,FontMappedScalar},super::{FontFace,FontOutlineLimits,FontProgress,reserve_slot}};", f"face::{{FontClusterMapping,FontMappedScalar}}}};\nuse {F}::{{FontFace,FontOutlineLimits,FontProgress,reserve_slot}};"),
    ])
    edit(os.path.join(shape, "🧩️context/🪄️apply/🦀️.rs"), [
        ("face::{FontClusterMapping,FontMappedScalar},super::{FontFace,FontOutlineLimits,FontProgress,FontReader,reserve_slot}};", f"face::{{FontClusterMapping,FontMappedScalar}}}};\nuse {F}::{{FontFace,FontOutlineLimits,FontProgress,FontReader,reserve_slot}};"),
    ])
    edit(os.path.join(shape, "🧩️context/🪄️apply/🏪️workspace/🦀️.rs"), [("super::super::super::super::unicode::UnicodeProperty", f"{U}::UnicodeProperty")])
    edit(os.path.join(UNICODE, "🦀️.rs"), [
        ("use super::super::{FontReader,FontProgress};", f"use {F}::{{FontReader,FontProgress}};\nuse semio_framework_unicode_properties::PROPERTIES;\nuse semio_framework_unicode_mappings::MAPPINGS;"),
        ('pub const UNICODE_BYTES:&[u8]=include_bytes!("📦️data/🔢️properties.bin");',
         "static UNICODE_RECORD:[u8;PROPERTIES.len()+MAPPINGS.len()]=unicode_record();\n"
         "/// 🧷️ Composes the immutable record from its two asset parts at compile time, so the shipped record stays one contiguous static.\n"
         "const fn unicode_record()->[u8;PROPERTIES.len()+MAPPINGS.len()]{let mut record=[0;PROPERTIES.len()+MAPPINGS.len()];let(head,tail)=record.split_at_mut(PROPERTIES.len());head.copy_from_slice(PROPERTIES);tail.copy_from_slice(MAPPINGS);record}\n"
         "/// 📜️ The complete pinned Unicode shaping record that `UnicodeDataAdmissionJob` admits.\n"
         "pub static UNICODE_BYTES:&[u8]=&UNICODE_RECORD;"),
    ])
    for facet in ["🧵️normalize", "🧵️cluster", "↔️bidi"]:
        path = os.path.join(UNICODE, facet, "🦀️.rs")
        text = open(path, encoding="utf-8").read()
        old = re.search(r"use super::super::super::\{([^}]*)\};", text).group(0)
        edit(path, [(old, old.replace("super::super::super::", F + "::"))])


TESTS = {
    "font": {"dir": FONT, "tests": [25, 27, 34, 36, 38], "helpers": [28, 31, 32]},
    "shape": {"dir": os.path.join(FONT, "🧵️shape"), "tests": [6, 10, 14, 16, 18, 20, 53, 58, 60, 62, 69, 72], "helpers": [4]},
    "unicode": {"dir": UNICODE, "tests": [30, 63, 65, 67], "helpers": []},
    "run": {"dir": os.path.join(FONT, "🧵️run"), "tests": [12, 46, 48], "helpers": [39, 40, 41, 42, 43, 44]},
    "catalog": {"dir": os.path.join(FONT, "📇️catalog"), "tests": [50], "helpers": []},
}
CAT, TESTING = "semio_framework_2d_text_font_catalog", "semio_framework_2d_text_font_testing"
LOOK = r"(?<![A-Za-z0-9_:])"
REWRITE = {
    "font": [(LOOK + r"crate::PathSegment", E + "::PathSegment")],
    "shape": [("use shape::{face::*,unicode::*};", f"use crate::face::*;use {U}::*;"), ("use shape::{face::*,unicode::{*,normalize::NormalizedScalar,cluster::UnicodeCluster}};", f"use crate::face::*;use {U}::{{*,normalize::NormalizedScalar,cluster::UnicodeCluster}};"),
              (LOOK + r"shape::unicode::", U + "::"), (LOOK + r"shape::", "crate::"), (LOOK + r"catalog::", CAT + "::")],
    "unicode": [(LOOK + r"shape::unicode::", "crate::")],
    "run": [(LOOK + r"crate::PathSegment", E + "::PathSegment"), (LOOK + r"shape::unicode::", U + "::"), (LOOK + r"shape::", S + "::"), (LOOK + r"run::", "crate::"), (LOOK + r"catalog::", CAT + "::")],
    "catalog": [(LOOK + r"catalog::", "crate::")],
}
DOCS = {
    "font": "//! 🧫️ Shared authored SFNT admission, outlines and kerning agree with ttf-parser, rustybuzz and pixels and retain original backing through funded close.",
    "shape": "//! 🧶️ Shared OpenType lookup selection, substitution, marks, ligatures and cluster mapping agree with ttf-parser and rustybuzz through funded close.",
    "unicode": "//! 🔡️ Admitted Unicode records, bidi levels, canonical normalization and grapheme clusters agree with the original corpus through funded close.",
    "run": "//! 🏃️ Shaped multiline outline runs match rustybuzz and ttf-parser placement and pixels and retain completed children through funded close.",
    "catalog": "//! 🗂️ Shipped family requests retain their actual immutable sources through funded close.",
}


def tests():
    source = os.path.join(FONT, "🧪️tests/🦀️.rs")
    lines = open(source, encoding="utf-8").read().split("\n")
    if not lines[0].startswith("//! 🧫️ Shared authored SFNT sources"):
        print("already distributed")
        return
    lines[61] = lines[61].removesuffix("#[test]")
    helpers = {21: "bytes", 22: "close", 23: "close_with_items"}
    testing = os.path.join(FONT, "🧪️testing")
    body = lines[20].replace('join("../../..")', 'join("../../../../../../..")')
    write(os.path.join(testing, "🦀️.rs"), "//! 🧰️ Dev-only support of the font crate family tests: authored font bytes and funded physical close with heap witnesses.\n"
          "use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneStep}};\n"
          "/// 🔤️ Reads one shipped font asset outline by its `🖼️assets/🔤️fonts` family path.\npub " + body + "\n"
          "/// 🧹️ Closes one owner with single-item grants and returns the physically released bytes.\npub " + lines[21] + "\n"
          "/// 🧮️ Closes one owner with `items`-item grants, proving every step against the heap witness, and returns the released bytes.\npub " + lines[22] + "\n")
    for crate, spec in TESTS.items():
        items = []
        for number in spec["helpers"]:
            items.append(lines[number - 1])
        for number in spec["tests"]:
            block = [lines[number - 1]]
            if number == 53:
                block = lines[52:56]
            items.append("#[test]\n" + "\n".join(block))
        text = "\n".join(items)
        for old, new in REWRITE[crate]:
            text = re.sub(old, new.replace("\\", "\\\\"), text) if old.startswith("(?<!") else text.replace(old, new)
        used = [name for name in ["bytes", "close", "close_with_items"] if re.search(r"(?<![A-Za-z0-9_.])" + name + r"\(", text)]
        head = [DOCS[crate], "use super::*;"]
        if crate in ("shape", "unicode", "run", "catalog"):
            head.append(f"use {F}::*;")
        if used:
            head.append(f"use {TESTING}::{{{','.join(used)}}};")
        value = [name for name in ["ControlledRetirement", "RetainedCloneGrant", "RetainedCloneStep"] if name in text]
        if value:
            head.append("use semio_framework_value::{" + ",".join(f"retirement::controlled::{n}" if n == "ControlledRetirement" else f"retained_clone::{n}" for n in value) + "};")
        head += ["#[global_allocator]", "static FONT_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;"]
        old_dir = os.path.join(FONT, "🧪️tests")
        new_dir = os.path.join(spec["dir"], "🧪️tests")
        text = re.sub(r'include_str!\("([^"]+)"\)', lambda m: 'include_str!("' + os.path.relpath(os.path.normpath(os.path.join(old_dir, m.group(1))), new_dir) + '")', text)
        write(os.path.join(new_dir, "🦀️.rs"), "\n".join(head) + "\n" + text + "\n")
    for crate in ("shape", "unicode", "run", "catalog"):
        root = os.path.join(TESTS[crate]["dir"], "🦀️.rs")
        text = open(root, encoding="utf-8").read()
        if '#[path="🧪️tests/🦀️.rs"]' not in text:
            write(root, text.rstrip("\n") + '\n#[cfg(test)]\n#[path="🧪️tests/🦀️.rs"]\nmod tests;\n')


def manifests():
    for name, spec in CRATES["crates"].items():
        package = os.path.join(D2, spec["folder"], "📦️packages/🦀️rust")
        rel = lambda target: os.path.relpath(os.path.join(REPO, target), package)
        out = ["[package]", f'workspace = "{os.path.relpath(FW, package)}"', f'name = "{name}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true", f'description = "{spec["description"]}"', "",
               "[package.metadata.semio]", 'role = "framework"', f'id = "{name.removeprefix("semio-framework-")}"', "",
               "[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", "",
               "[lints]", "workspace = true", "", "[lib]", f'name = "{name.replace("-", "_")}"', 'path = "../../🦀️.rs"', ""]
        for table, key in (("dependencies", "deps"), ("dev-dependencies", "dev")):
            rows = []
            for dep in spec.get(key, []):
                if dep in CRATES["crates"]:
                    rows.append(f'{dep} = {{ path = "{os.path.relpath(os.path.join(D2, CRATES["crates"][dep]["folder"], "📦️packages/🦀️rust"), package)}" }}')
                elif dep in CRATES["external"]:
                    rows.append(f'{dep} = {{ path = "{rel(CRATES["external"][dep])}" }}')
                else:
                    rows.append(f"{dep} = {CRATES['thirdParty'][dep]}")
            if rows:
                out += [f"[{table}]"] + rows + [""]
        write(os.path.join(package, "Cargo.toml"), "\n".join(out))
    remove(os.path.join(D2, "📦️packages/🦀️rust/Cargo.toml"))
    remove(os.path.join(D2, "📦️packages/🦀️rust/🦀️.rs"))


{"data": data, "sources": sources, "tests": tests, "manifests": manifests}[stage]()
print("\n".join(written))
