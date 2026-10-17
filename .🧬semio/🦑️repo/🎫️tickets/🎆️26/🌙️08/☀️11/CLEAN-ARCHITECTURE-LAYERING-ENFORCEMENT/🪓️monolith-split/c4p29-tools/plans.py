"""🗺️ c4p29 split plans for Package 29: the stdio pdf 1.7 base lower, lift, export, graph-projection and diff modules, the pdf crate lib root (three mount trees) and the docx base test oracle.
`mode` "mod": children are `#[path]` modules (`use super::*;`, root `<vis> use <mod>::*;` when the child exports a name another file uses). `ranges` are 1-based inclusive line ranges of the pre-edit file.
Boundaries are computed from item names and region names of the pre-edit blob, never typed as raw line numbers: `T(key, text)` is the first line of the doc/attribute block of the first line containing `text`, `R(key, region)` the span of a region.
`pre` / `post` lines wrap a child body (a region that spans two children is repeated, balanced, in each, as in `RESULT-REGION-RESTORE.md`); `tree: True` children are inline mount trees moved to files; `container` is the inline module whose depth-1 items are carved."""
import os, re
from c4plib import *
import splitter as S
from rslex import outline


def _ls(key):
    return lines_of(read(f"{ORIG}/{key}.rs"))


def S(key, name, nth=1):
    ls = _ls(key)
    its = [it for it in outline("\n".join(ls) + "\n") if it["name"] == name and it["kind"] not in ("use", "other")]
    assert len(its) >= nth, (key, name)
    return _block_start(ls, its[nth - 1]["start"] + 1)


def T(key, text, nth=1):
    ls = _ls(key)
    hits = [i + 1 for i, l in enumerate(ls) if text in l]
    assert len(hits) >= nth, (key, text)
    return _block_start(ls, hits[nth - 1])


def _is_c(l):
    s = l.strip()
    return (s.startswith(("///", "//", "#[")) and not s.startswith(("//#region", "//#endregion", "// #region", "// #endregion")))


def _block_start(ls, line):
    i = line - 1
    while i - 1 >= 0:
        prev = ls[i - 1]
        if _is_c(prev):
            i -= 1
            continue
        if prev.strip() == "":
            j = i - 2
            while j >= 0 and ls[j].strip() == "":
                j -= 1
            if j >= 0 and _is_c(ls[j]) and ls[j].strip().startswith("///"):
                i = j + 1
                continue
        break
    return i + 1


def R(key, name, which=1):
    ls = _ls(key)
    stack, out = [], []
    for i, l in enumerate(ls, 1):
        m = re.match(r"^\s*//\s?#region\s*(.*)$", l)
        if m:
            stack.append((i, m.group(1).strip()))
            continue
        m = re.match(r"^\s*//\s?#endregion\b", l)
        if m and stack:
            s, n = stack.pop()
            if n == name:
                out.append((s, i))
    out.sort()
    return out[which - 1]


def OPEN(key, name):
    for i, l in enumerate(_ls(key), 1):
        m = re.match(r"^\s*//\s?#region\s*(.*)$", l)
        if m and m.group(1).strip() == name:
            return i
    raise KeyError((key, name))


def END(key):
    return len(_ls(key))


def _kids(specs):
    out = []
    for folder, mod, doc, ranges, extra in specs:
        d = dict(folder=folder, mod=mod, ranges=ranges, doc=doc)
        d.update(extra)
        out.append(d)
    return out


def _plan(rel, lib, specs, mode="mod", **more):
    d = dict(rel=rel, mode=mode, lib=lib, children=_kids(specs))
    d.update(more)
    return d




def TREE(key, name):
    its = [it for it in outline("\n".join(_ls(key)) + "\n") if it["name"] == name and it["kind"] == "mod"]
    assert len(its) == 1, (key, name, len(its))
    return (its[0]["start"] + 1, its[0]["end"] + 1)


def MODOPEN(key, name):
    for i, l in enumerate(_ls(key), 1):
        if re.match(rf"^mod {name} \{{$", l):
            return i
    raise KeyError((key, name))


def CLOSE(key, open_line):
    ls = _ls(key)
    for i in range(open_line, len(ls) + 1):
        if ls[i - 1] == "}":
            return i
    raise KeyError((key, open_line))


LIB = f"{PDF}/🦀️.rs"
MODS = f"{PB}/🔨️modules"


def lower():
    k = "lower"
    return _plan(f"{MODS}/⬆️lower/🦀️.rs", LIB, [
        ("🎛️options-and-lowering", "options_and_lowering", "🎛️ The lowering options, the `Lowering` pass state with its object table and reference maps, `LoweredDocument` and the document-level lowering entry points.", [(R(k, "🔖️Options")[0], R(k, "🔖️Lowering")[1])], dict(replace=[("pub use super::lift::Category;", "pub use crate::standards::v1_7::subsets::base::modules::lift::Category;")])),
        ("🔤️fonts-and-resources", "fonts_and_resources", "🔤️ Lowering of fonts and CID system info and of the shared resource dictionaries (images, forms, graphics states, shadings, patterns, colour spaces, properties).", [(R(k, "🔖️Fonts")[0], R(k, "🔖️Resources")[1])], {}),
        ("🧭️pages-and-navigation", "pages_and_navigation", "🧭️ Lowering of the page tree and pages and of the navigation structures (outlines, named destinations, page labels, annotations).", [(R(k, "🔖️Pages")[0], R(k, "🔖️Navigation")[1])], {}),
        ("📚️catalog-and-info", "catalog_and_info", "📚️ Lowering of the document catalog, the information dictionary and the page mode names.", [R(k, "🔖️Catalog")], {}),
    ])


def lift_plan(k, rel):
    first = R(k, "🔖️Lifter")[0]
    return _plan(rel, LIB, [
        ("🧩️lifter-and-document", "lifter_and_document", "🧩️ The lifter state, the resource categories, the document entry points and the document-level lanes of the lift.", [(first, R(k, "🔖️Document")[1])], {}),
        ("🪟️resources-and-fonts", "resources_and_fonts", "🪟️ Lifting of the resource dictionaries (images, forms, graphics states, shadings, patterns, colour spaces, properties) and of fonts.", [(R(k, "🔖️Resources")[0], R(k, "🔖️Fonts")[1])], {}),
        ("🧭️navigation-and-annotations", "navigation_and_annotations", "🧭️ Lifting of the page tree, outlines, named destinations, page labels and annotations.", [(R(k, "🔖️Navigation")[0], R(k, "🔖️Annotations")[1])], {}),
        ("📚️catalog-and-page-mode", "catalog_and_page_mode", "📚️ Lifting of the document catalog (interactive form, optional content, viewer preferences, metadata) and the page mode names.", [R(k, "🔖️Catalog")], {}),
    ])


def lift():
    return lift_plan("lift", f"{MODS}/⬇️lift/🦀️.rs")


def gp():
    return lift_plan("gp", f"{PB}/🧬️schema/🔎️graph-projection/🦀️.rs")


def exp():
    k = "exp"
    graft = T(k, "/// 🪡 A graft in progress")
    return _plan(f"{PB}/🚪️io/📤️export/🦀️.rs", LIB, [
        ("🧬️encode-and-reconcile", "encode_and_reconcile", "🧬️ The write options, `encode_pdf` and its recording, reading and catalog-lane helpers, and the reconcile and regenerate passes.", [(R(k, "🔖️Encode")[0], graft - 1)], dict(post=["//#endregion 🔖️Encode"])),
        ("🪡️graft", "graft", "🪡️ The `Graft` that patches the retained object graph in place, with the resource section helpers.", [(graft, R(k, "🔖️Encode")[1])], dict(pre=["//#region 🔖️Encode"])),
        ("🌊️streaming-and-builders", "streaming_and_builders", "🌊️ The page-at-a-time `DocumentStream`, the text layout and the typed document builders.", [(R(k, "🔖️Streaming")[0], R(k, "🔖️Builders")[1])], {}),
    ])


def diff():
    k = "diff"
    return _plan(f"{PB}/🧬️schema/🔺️diff/🦀️.rs", LIB, [
        ("🔀️tri-state-indexed-and-keyed", "tri_state_indexed_and_keyed", "🔀️ The tri-state patch value and the index-keyed and id-keyed triple diffs every collection lane rides on.", [(R(k, "🔖️TriState")[0], R(k, "🔖️KeyedTriple")[1])], {}),
        ("📄️page-info-and-pages", "page_info_and_pages", "📄️ The per-page patch, the document information patch and the pages triple with its apply, absorb and inverse laws.", [(R(k, "🔖️PageDiff")[0], R(k, "🔖️PagesTriple")[1])], {}),
        ("🌳️value-trees", "value_trees", "🌳️ The recursive COS value patches: dictionaries, arrays, values and the object triple.", [(R(k, "🔖️DictDiff (reused for nested Dict/Stream.dict AND top-level trailer)")[0], R(k, "🔖️ObjectsTriple")[1])], {}),
        ("🧭️path-addressing-and-validation", "path_addressing_and_validation", "🧭️ Object path addressing and the validators of every diff lane.", [(R(k, "🔖️PathAddressing")[0], R(k, "🔖️Diff")[0] - 1)], {}),
        ("🧾️pdf-diff-aggregate", "pdf_diff_aggregate", "🧾️ The `PdfDiff` aggregate: validation, unchecked application, absorption and inversion across every document lane.", [R(k, "🔖️Diff")], {}),
        ("🛠️mutation-diff-builders", "mutation_diff_builders", "🛠️ The per-mutation diff builders and the page box vocabulary.", [R(k, "🔖️MutationDiffBuilders")], {}),
    ])


def root():
    k = "root"
    return _plan(f"{PDF}/🦀️.rs", LIB, [
        ("🧾️artifact-declaration", "artifact_declaration", "🧾️ The packaged artifact definition and declaration: formats, native codecs, contribution, assembly, subset validators, pilot languages, the artifact kind and the derived io registry.", [(T(k, "/// 📜 Schema-owned package definition"), R(k, "🚪️DerivedIoRegistry")[1])], dict(replace=[('include_str!("📜️artifact-definition.json")', 'include_str!("../📜️artifact-definition.json")')])),
        ("🌳️standards-tree", "standards", "🌳️ The `standards` mount tree: the 1.4 and 1.7 standards with their subsets, schemas, io and codec modules.", [TREE(k, "standards")], dict(tree=True)),
        ("✏️editor-tree", "editor", "✏️ The `editor` mount tree: the editor components of every PDF subset with their presence, config and window modules.", [TREE(k, "editor")], dict(tree=True)),
        ("👁️viewer-tree", "viewer", "👁️ The `viewer` mount tree: the viewer components of every PDF subset with their presence, config and window modules.", [TREE(k, "viewer")], dict(tree=True)),
    ])


def _root_variant(plan):
    only = os.environ.get("C4P29_ROOT_ONLY")
    if only == "trees":
        plan["children"] = [c for c in plan["children"] if c.get("tree")]
    elif only == "decl":
        plan["children"] = [c for c in plan["children"] if not c.get("tree")]
    return plan


def ora():
    k = "ora"
    co = MODOPEN(k, "oracles")
    cc = CLOSE(k, co)
    return _plan(f"{DB}/🔮️oracles/🦀️.rs", f"{ART}/📜️docx/🔮️oracles/🦀️.rs", [
        ("🌳️xml-tree-and-opc", "xml_tree_and_opc", "🌳️ The generic XML node tree with its reader and writer, and the OPC container types, relationships and zip read and write.", [(R(k, "🔖️GenericXmlTree")[0], R(k, "🔖️OpcTypes")[1])], {}),
        ("📝️wordprocessing-and-package", "wordprocessing_and_package", "📝️ The WordprocessingML block model, its parse and serialize halves and the package read and write.", [(R(k, "🔖️WordprocessingModel")[0], R(k, "🔖️Package")[1])], {}),
        ("🧮️json-values-and-addressing", "json_values_and_addressing", "🧮️ The JSON parameter readers and writers and the run and block path addressing.", [(R(k, "🔖️JsonValue")[0], R(k, "🔖️PathAddressing")[1])], {}),
        ("🔁️forward-inverse-and-routing", "forward_inverse_and_routing", "🔁️ The forward mutation kinds, their computed inverses and the routing and projection entry points.", [(R(k, "🔖️Forward")[0], R(k, "🔖️Routing")[1])], {}),
    ], container=(co, cc))


ORDER = ["lower", "lift", "exp", "gp", "diff", "root", "ora"]
PLANS = {key: globals()[key]() if key not in ("lift", "gp") else (lift() if key == "lift" else gp()) for key in ORDER}
PLANS["root"] = _root_variant(PLANS["root"])
