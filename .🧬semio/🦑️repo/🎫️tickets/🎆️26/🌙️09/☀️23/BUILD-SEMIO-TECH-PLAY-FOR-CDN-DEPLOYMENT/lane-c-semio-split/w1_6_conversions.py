#!/usr/bin/env python3
"""🔗️ W1 step 6 (split plan 1f + 1g): every subset's conversions move out of its `🚪️io/🦀️.rs` into the conversion roots
`🚪️io/📥️import/🦀️.rs` and `🚪️io/📤️export/🦀️.rs`, each exporting its `entries()` and `CONVERSION`; the subset keeps
its own composition rows only. Conversion-free composition tests become plain `#[cfg(test)]`; round-trip tests that need
both directions move to the export root."""
import sys, json, textwrap
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from use_rewrite import _parse_tree

CFG_RX = re.compile(r'^\s*#\[cfg\((?:all\(test, )?feature = "conversion-[a-z0-9-]+"\)?\)\]\s*$')
ROUND_TRIPS = {"document": ["docx_round_trip_is_stable", "md_round_trip_is_stable", "txt_round_trip_is_stable", "pdf_round_trip_is_stable"], "presentation": ["pptx_round_trip_is_stable"], "flow": ["json_round_trip_is_stable"]}
FLOW_HELPERS = ["node", "edge"]
report = {}
pending = {}
_real_write = write


def write(p, s):
    pending[p] = s


def read_pending(p):
    return pending[p] if p in pending else read(p)


def version_ident(ver):
    v = re.sub(r"[.\-]", "_", ver)
    return ("v" if v[:1].isdigit() else "v_") + v


def leaf_tree(rows, kind):
    """🌳️ Renders `<kind>::artifacts::<fmt>::<ver>::any` module mounts for (fmt_ident, ver_ident, path) rows."""
    out = ['#[path = "."]', "pub mod %s {" % kind, '    #[path = "."]', "    pub mod artifacts {"]
    by_fmt = {}
    for fmt, ver, path in rows:
        by_fmt.setdefault(fmt, []).append((ver, path))
    for fmt, vers in by_fmt.items():
        out += ['        #[path = "."]', "        pub mod %s {" % fmt]
        for ver, path in vers:
            out += ['            #[path = "."]', "            pub mod %s {" % ver, '                #[path = "."]', "                pub mod any {", '                    #[path = "%s"]' % path, "                    mod component;", "                    pub use component::*;", "                }", "            }"]
        out += ["        }"]
    out += ["    }", "}"]
    return "\n".join(out)


def extract_root_block(lines, name):
    for i, l in enumerate(lines):
        if l == "    pub mod %s {" % name and lines[i - 1].strip().startswith('#[cfg(feature = "conversion-') and lines[i - 2].strip() == '#[path = "."]':
            e = block_end(lines, i)
            body = [x[4:] if x.startswith("    ") else x for x in lines[i + 1:e]]
            return i - 2, e, body
    return None


def comment_start(lines, i):
    k = i
    while k > 0 and re.match(r"^\s*(///|// )", lines[k - 1]) and not re.match(r"^\s*//#", lines[k - 1]):
        k -= 1
    return k


def collect(lines):
    items = []
    i = 0
    keep = []
    while i < len(lines):
        if CFG_RX.match(lines[i]):
            start = comment_start(lines, i)
            j = i + 1
            while lines[j].strip().startswith("#["):
                j += 1
            head = lines[j].strip()
            if head.startswith("include!("):
                items.append(("test_include", lines[i:j + 1]))
                keep.append(re.sub(r"#\[cfg\(all\(test, feature = \"conversion-[a-z0-9-]+\"\)\)\]", "#[cfg(test)]", lines[i]))
                keep.append(lines[j])
                i = j + 1
                continue
            if head.endswith(";") and "{" not in head:
                end = j
            else:
                end = block_end(lines, j)
            text = lines[start:end + 1]
            if i > start:
                del keep[len(keep) - (i - start):]
            items.append((classify(head), text))
            i = end + 1
            continue
        keep.append(lines[i])
        i += 1
    return items, keep


def classify(head):
    if head.startswith("use semio_framework_plugin::{deserializer_entry_of"):
        return "use_plugin"
    if head.startswith("use "):
        return "use_leaf"
    if head.startswith("static IO_ENTRIES") or re.match(r"fn (io_entries|io_bridge_entries|bridge_entries)\(", head):
        return "entries"
    if head.startswith("register_composer_entries("):
        return "register_call"
    if head.startswith("let builder = {"):
        return "declare_block"
    if re.match(r"pub mod \w+_(de)?serializer;", head):
        return "path_mod"
    if re.match(r"pub enum Semio\w+Format", head):
        return "format_enum"
    if head.startswith("pub fn encode_"):
        return "encode_fn"
    if head.startswith("pub fn decode_"):
        return "decode_fn"
    if head.startswith("pub mod dwg {"):
        return "dwg_mod"
    raise SystemExit("unclassified conversion item: " + head)


for s in CONVERSION_SUBSETS:
    emoji = EMOJI[s]
    rp = SUBSETS + "/" + emoji + "/🦀️.rs"
    fp = SUBSETS + "/" + emoji + "/🚪️io/🦀️.rs"
    R = read(rp).split("\n")
    F = read(fp).split("\n")
    items, keep = collect(F)
    kinds = [k for k, _ in items]
    trees = {}
    names = {}
    imp = extract_root_block(R, "import")
    if imp:
        exp = extract_root_block(R, "export")
        for key, block, prefix in (("import", imp, "🚪️io/📥️import/"), ("export", exp, "🚪️io/📤️export/")):
            body = "\n".join(block[2]).replace('#[path = "' + prefix, '#[path = "')
            assert "🚪️io/" not in body, (s, key)
            trees[key] = body
        lo, hi = imp[0], exp[1]
        assert exp[0] > imp[1]
        R[lo:hi + 1] = ['    #[cfg(feature = "conversion-%s")]' % s, '    #[path = "🚪️io/📥️import/🦀️.rs"]', "    pub mod import;", '    #[cfg(feature = "conversion-%s")]' % s, '    #[path = "🚪️io/📤️export/🦀️.rs"]', "    pub mod export;"]
    else:
        rows = {"import": [], "export": []}
        for kind, text in items:
            if kind != "path_mod":
                continue
            path = re.search(r'#\[path = "([^"]+)"\]', "\n".join(text)).group(1)
            modname = re.search(r"pub mod (\w+);", "\n".join(text)).group(1)
            key = "import" if path.startswith("📥️import/") else "export"
            rel = path.split("/", 1)[1]
            parts = rel.split("/")
            fmt = re.sub(r"^[^a-z0-9]+", "", parts[2])
            ver = version_ident(re.sub(r"^🔖️", "", parts[3]))
            rows[key].append((fmt, ver, rel))
            names[modname] = (key, "%s::artifacts::%s::%s::any" % ("deserializers" if key == "import" else "serializers", fmt, ver))
        trees["import"] = leaf_tree(rows["import"], "deserializers")
        trees["export"] = leaf_tree(rows["export"], "serializers")
        anchor = keep.index("//#region 🎹️DerivedComposition")
        keep[anchor:anchor] = ['#[cfg(feature = "conversion-%s")]' % s, '#[path = "📥️import/🦀️.rs"]', "pub mod import;", '#[cfg(feature = "conversion-%s")]' % s, '#[path = "📤️export/🦀️.rs"]', "pub mod export;"]

    leaf = {}
    for kind, text in items:
        if kind != "use_leaf":
            continue
        src = "\n".join(t for t in text if not t.strip().startswith(("#", "//")))
        body = src.strip()[4:].rstrip(";")
        flat, _ = _parse_tree(body)
        for segs, name in flat:
            segs = list(segs)
            if segs[:2] == ["super", "super"]:
                segs = ["crate", "standards", "v1", "subsets", s, "io"] + segs[2:]
            joined = "::".join(segs)
            m = re.search(r"::io::(import|export)::(.*)$", joined)
            if m:
                leaf[name] = (m.group(1), m.group(2))
                continue
            m = re.search(r"::io::(\w+_(?:de)?serializer)$", joined)
            if m:
                key, rel = names[m.group(1)]
                leaf[name] = (key, rel)
                continue
            raise SystemExit("unmapped leaf use %s %s" % (s, joined))
    entries = {"import": [], "export": []}
    for kind, text in items:
        if kind != "entries" or not any(re.search(r"\bfn ", t) for t in text):
            continue
        for m in re.finditer(r"(deserializer|serializer)_entry_of::<\s*([^>]+?)\s*>\(\)", "\n".join(text)):
            ty = m.group(2)
            mm = re.search(r"::io::(import|export)::(.*)::(\w+)$", ty)
            if mm:
                key, rel, name = mm.group(1), mm.group(2), mm.group(3)
            else:
                key, rel = leaf[ty]
                name = ty
            assert (key == "import") == (m.group(1) == "deserializer"), (s, ty)
            entries[key].append("%s::%s" % (rel, name))

    moved = {"import": [], "export": []}
    uses_needed = {"import": set(), "export": set()}
    new_keep = keep
    for kind, text in items:
        if kind == "format_enum":
            body = [t for t in text if not CFG_RX.match(t)]
            idx = new_keep.index("//#region 🧾️Encoding")
            new_keep[idx + 1:idx + 1] = body
        elif kind in ("encode_fn", "decode_fn"):
            key = "export" if kind == "encode_fn" else "import"
            moved[key].append("\n".join(t for t in text if not CFG_RX.match(t)))
            uses_needed[key].add("crate::standards::v1::subsets::%s::io::Semio%sFormat" % (s, s.capitalize()))
        elif kind == "dwg_mod":
            t = "\n".join(text)
            moved["export"].append("/// 🖊️ DWG mesh export for the BREP artifact.\npub mod dwg {\n    use semio_framework_3d::brep::engine::{BrepError, BrepKernel, GeometryHandle};\n" + re.search(r"(    struct DwgExporter;.*?\n    }\n)", t, re.S).group(1) + "\n" + re.search(r"(    /// 📤️ Encodes.*?\n    }\n)", t, re.S).group(1) + "}")
            moved["import"].append("/// 🖊️ DWG mesh import for the BREP artifact.\npub mod dwg {\n    use semio_framework_3d::brep::engine::{BrepError, BrepKernel, GeometryHandle};\n" + re.search(r"(    struct DwgImporter;.*?\n    }\n)", t, re.S).group(1) + "\n" + re.search(r"(    /// 📥️ Imports.*?\n    }\n)", t, re.S).group(1) + "}")

    round_trip = None
    if s in ROUND_TRIPS:
        tp = SUBSETS + "/" + emoji + "/🚪️io/🧪️tests/🔬️derived-composition-unit/🦀️.rs"
        tsrc = read_pending(tp)
        tsrc, texts, _ = extract_functions(tsrc, ROUND_TRIPS[s])
        write(tp, tsrc)
        helpers = ""
        if s == "flow":
            hs = []
            for h in FLOW_HELPERS:
                a, b = fn_span(tsrc, h)
                hs.append(tsrc[a:b])
            helpers = "".join(hs)
        used = set(re.findall(r"\b(Semio\w+(?:From|To)\w+)\b", "".join(texts)))
        imports = sorted("crate::standards::v1::subsets::%s::io::%s::%s::%s" % (s, leaf[n][0], leaf[n][1], n) for n in used)
        round_trip = ("//! 🔁️ Round trips of the `%s` subset through each format: import, export, import again.\n\n" % s
                      + "".join("use %s;\n" % u for u in imports)
                      + "use crate::standards::v1::subsets::%s::schema::snapshot::*;\nuse semio_framework_plugin::{ArtifactDeserializer, ArtifactSerializer};\n\n" % s
                      + textwrap.dedent(helpers + "".join(texts)))

    for key in ("import", "export"):
        kind = "deserializers" if key == "import" else "serializers"
        arrow = "into" if key == "import" else "out of"
        fmts = sorted(set(re.match(r"\w+::artifacts::(\w+)::", e).group(1) for e in entries[key]))
        extra_uses = sorted(uses_needed[key])
        body = moved[key]
        txt = ("//! %s The `%s` subset's %s conversions: the %s leaves that carry `s.stdio.semio` `%s` documents %s %s, and the\n//! `CONVERSION` that registers and declares their composer rows.\n\n"
               % ("📥️" if key == "import" else "📤️", s, key, kind[:-1], s, arrow, ", ".join(fmts) if fmts else "their formats")
               + trees[key] + "\n\n"
               + "".join("use %s;\n" % u for u in extra_uses)
               + "use crate::base_foundation::SemioConversion;\nuse semio_framework_plugin::{%s_entry_of, io::{register_composer_entries, ComposerEntry}, app::{ArtifactDeclarationBuilder, DeclarationReady}};\n\n" % ("deserializer" if key == "import" else "serializer")
               + "/// 🧾️ This direction's composer rows, built once.\npub fn entries() -> &'static [ComposerEntry] {\n    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();\n    ENTRIES.get_or_init(|| vec![" + ", ".join("%s_entry_of::<%s>()" % ("deserializer" if key == "import" else "serializer", e) for e in entries[key]) + "]).as_slice()\n}\n\n"
               + "/// 🔌️ Registers and declares this direction's composer rows; consumers name it in `register` / `declaration`.\npub const CONVERSION: SemioConversion = SemioConversion { declare, register };\n\n"
               + "fn declare(builder: ArtifactDeclarationBuilder<DeclarationReady>) -> ArtifactDeclarationBuilder<DeclarationReady> {\n    static COMPOSERS: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();\n    builder.composers(crate::base_foundation::semio_written(entries(), &COMPOSERS))\n}\n\n"
               + "fn register() {\n    register_composer_entries(entries()).expect(\"static Stdio registration must be available and conflict-free\");\n}\n"
               + ("\n" + "\n\n".join(body) + "\n" if body else "")
               + ("\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit/🦀️.rs\"]\nmod tests;\n" if key == "export" and round_trip else ""))
        write(SUBSETS + "/" + emoji + "/🚪️io/%s/🦀️.rs" % ("📥️import" if key == "import" else "📤️export"), txt)
    if round_trip:
        write(SUBSETS + "/" + emoji + "/🚪️io/📤️export/🧪️tests/🔬️unit/🦀️.rs", round_trip)
    write(fp, "\n".join(new_keep))
    write(rp, "\n".join(R))
    report[s] = {"items": kinds, "import": len(entries["import"]), "export": len(entries["export"])}
for p, c in pending.items():
    _real_write(p, c)
print(json.dumps(report, indent=0))
