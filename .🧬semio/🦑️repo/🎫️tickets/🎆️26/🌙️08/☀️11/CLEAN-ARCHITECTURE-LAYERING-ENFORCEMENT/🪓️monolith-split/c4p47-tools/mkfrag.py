"""📐 C4P47 fragment spec generator for the three flat test files (dsu, neu, hu): chunk start tables -> specs-frag/<tag>.json for fragsplit.py.
A start is an item line of the ORIGINAL snapshot; with back=True it is backed up over contiguous doc/attribute/comment lines (indentation aware, never over a region end).
A chunk runs to the line before the next start (trailing blanks trimmed) unless `ranges` gives explicit lines. usage: mkfrag.py"""
import json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
STAGE = "/Users/ueli/.cache/c4p47/stage-frag"
M = "🧰️framework/🛍️products/💻️os"
FILES = {
    "dsu": (M + "/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs", 0),
    "neu": (M + "/🔨️modules/🧠️neural/⚙️engine/🧪️tests/🔬️unit/🦀️.rs", 0),
    "hu": (M + "/🖥️host/🧪️tests/🔬️host-unit/🦀️.rs", 1),
}
V = "️"


def D(emoji, kebab, doc):
    return emoji + V + kebab, "%s%s %s" % (emoji, V, doc)


T = {
    "dsu": [
        (13, False, None, D("🧱", "dsl-field-and-derived-document-laws", "DSL field and derived document laws: the primitive and wire field impls, the toy idiom with the language registry, the derived scalar and record document with its handcrafted codecs and the derived operation enum with its text and binary laws.")),
        (293, False, None, D("🌳", "scene-and-shape-laws", "Scene and shape laws: the recursive block tree, the nested collections, the newtype tuple variants and their handcrafted document codecs.")),
        (485, False, [[485, 888]], D("🖌", "paint-self-ref-and-table-laws", "Paint, self-reference and table laws: the optional statements and block record fields, the paint operations, the boxed required field, the self-referential record struct and the compact structure-of-arrays table field.")),
        (896, False, None, D("📦", "paged-operation-laws", "Paged operation laws: the paged DSL operation settle and take helpers and the 8194-byte header, frame policy, intrinsic bridge and metadata admission laws.")),
    ],
    "neu": [
        (4, False, None, D("🔁", "dictionary-and-registry-laws", "Dictionary and registry laws: the observed system, the dictionary cursor, the echo and double operators, schema round trips, registry dispatch and the custom dispatch pipelines.")),
        (163, False, None, D("🔀", "wire-channel-and-collect-laws", "Wire, channel and collect laws: wire precedence over recorded literals, resolved channels per neuron and fixed and variadic port routing.")),
        (260, False, None, D("🧵", "cluster-and-top-level-laws", "Cluster and top-level laws: the add operator, boundary neurons, cluster contracts and inner trees, top-level evaluation and shakability.")),
        (391, False, [[391, 499]], D("🗂", "cached-evaluation-laws", "Cached evaluation laws: declared defaults, stable node hashes, cache hits, per-node errors and branch recomputation.")),
        (501, False, [[501, 780]], D("⏱", "budgeted-walk-laws", "Budgeted walk laws: remaining and probe walks, resumption across calls, failed dispatch service from the cache, the wall clock and node count caps and the cache epoch refresh before sweep.")),
        (782, False, None, D("📚", "cardinality-and-schema-component-laws", "Cardinality and schema component laws: cardinality symbols and counts, heterogeneous list refusal, null atoms and the tri-modal schema component.")),
        (914, False, None, D("🌊", "dirty-set-and-wave-laws", "Dirty set and wave laws: the dirty set for unchanged snapshots, descendants, removals and seeds, and the budgeted wave walks over contributed graphs.")),
        (1094, False, None, D("🚦", "owned-control-laws", "Owned control laws: the compact pending extension and the dictionary writer source retirement and original text laws.")),
    ],
    "hu": [
        (11, False, None, D("🔌", "plugin-registry-and-hot-swap", "Plugin registry and hot swap: loading plugin apps into the registry and the generation bump and app tracking of a hot swap.")),
        (227, False, None, D("🛟", "rollback-contributions-and-recovery", "Rollback, contributions and recovery: the invalid manifest rollback, the contribution tracking and the recovery UI of a quarantined plugin.")),
        (392, False, None, D("🧰", "host-fixtures", "Host fixtures: the minimal app definition, the seeded draw and sink plugins and the space and workflow store fixtures.")),
        (485, False, None, D("🎭", "actor-context-and-store-laws", "Actor context and store laws: the retained actor context, the workflow store catalog wire and the backbone round trips with their cursor.")),
        (609, False, None, D("🌱", "instances-parameters-and-catalog", "Instances, parameters and catalog: spawning and removing app instances, studio parameters, space catalog entries, cycles and the concurrent delete and wire reconcile.")),
        (709, False, None, D("📝", "dsl-and-op-text-laws", "DSL and operation text laws: the sample workflow snapshot, its DSL round trips and the operation text and document text round trips.")),
        (925, True, [[916, 1039]], D("🧩", "extension-install-laws", "Extension install laws: the test executor bridge, the sample extension package, install, mismatch refusal, capability scoping and uninstall.")),
    ],
}


def back_up(ls, s):
    k = s
    while k > 1:
        prev = ls[k - 2].lstrip()
        if prev and not prev.startswith("//#endregion") and not prev.startswith("// #endregion") and (prev.startswith("///") or prev.startswith("#[") or prev.startswith("//")):
            k -= 1
        else:
            break
    return k


os.makedirs(HERE + "/specs-frag", exist_ok=True)
for tag, rows in T.items():
    orig = f"{HERE}/orig/{tag}.rs"
    ls = open(orig, encoding="utf-8").read().split("\n")
    if ls[-1] == "":
        ls.pop()
    file_, base_depth = FILES[tag]
    starts = []
    for item, back, rng, (folder, doc) in rows:
        starts.append(rng[0][0] if rng else (back_up(ls, item) if back else item))
    assert starts == sorted(starts) and len(set(starts)) == len(starts), (tag, starts)
    chunks = []
    for i, (item, back, rng, (folder, doc)) in enumerate(rows):
        if rng:
            ranges = rng
        else:
            a = starts[i]
            b = (starts[i + 1] - 1) if i + 1 < len(rows) else len(ls)
            while not ls[b - 1].strip():
                b -= 1
            ranges = [[a, b]]
        chunks.append(dict(name=folder, folder=folder, doc=doc, ranges=ranges, **({"marker": "//" if ls[ranges[0][0] - 1].lstrip().startswith("//") and not ls[ranges[0][0] - 1].lstrip().startswith("///") else "///"} if tag == "hu" else {})))
        print(tag, folder, ranges, "first:", ls[ranges[0][0] - 1].strip()[:60])
    spec = dict(file=file_, original=orig, stage=STAGE, base_depth=base_depth, chunks=chunks)
    json.dump(spec, open(f"{HERE}/specs-frag/{tag}.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)
