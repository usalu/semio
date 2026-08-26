#!/usr/bin/env python3
"""Rewrites the four templated paragraphs of the 15 📕️norm feature descriptions so each states its
own subset's real vocabulary shape, real committed input and real evidence limits."""
import glob
import os
import textwrap

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"

# artifact-dir -> (reading, inverse, carrier)
#   reading  replaces the "Both implementations read the SAME committed bytes …" paragraph
#   inverse  replaces the "`inverse-` projects BOTH …" paragraph
#   carrier  replaces the "⚠️ Honest boundary — the CARRIER. …" paragraph
FACTS = {
    "📕️din4108": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. What the second "
            "reading has to get right HERE is that nineteen of the twenty-two kinds are flat scalar edits on the "
            "envelope record — `airtightness-n50`, `rh-int`, `psi-times-l-sum`, `solar-absorptance` — while "
            "`insert-layer`, `remove-layer` and `reorder-layers` address the `layers` build-up BY POSITION. A layer "
            "list that shifted the wrong way is invisible to any scalar reading, so those three rows are the only ones "
            "in this subset where the two implementations can disagree about structure rather than about a number."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document, because for nineteen scalar kinds the "
            "restored document is the before-document and projecting only it would make every row of the table report "
            "the same value. On `insert-layer`/`remove-layer`/`reorder-layers` the mutated projection is the only "
            "place the layer ORDER is observable at all."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads "
            "`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, a 455-byte hand-authored single-build-up demo: one "
            "`category=residential` envelope over a two-entry `layers [thickness-m:QTY lambda-w-mk:NUM]` build-up. It is the smallest committed document in "
            "this plugin and it is NOT a real DIN 4108 verification — no measured n50 report, no real material "
            "catalogue. It exercises the grammar, not the domain, and that is the ceiling on what this case's identity "
            "evidence proves. The carrier itself has no published grammar either: this subset's committed "
            "`🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` is the repository-wide `payload = OCTET+` "
            "placeholder, so the two implementations are compared on the envelope preamble, the ordered `key=value` "
            "fields and the `layers` block as written, plus the digest and length of what each side re-emitted — never "
            "on a mapping from carrier tokens to the JSON snapshot's enum spellings, which is stated nowhere."
        ),
    ),
    "📗️din16798": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. This subset is "
            "the widest FLAT vocabulary in the plugin — all sixty-two kinds are `change-<field>` on one indoor-climate "
            "record, with no collection, no composed child and no positional addressing anywhere. That makes it the "
            "purest test of the naming mechanic itself: sixty-two `new_<field>` arguments must each resolve to exactly "
            "one of the document's own keys by normalised spelling, and near-collisions the domain really contains "
            "(the five-way `change-theta-rm-c` / `change-theta-set-c` / `change-theta-st-c` / `change-theta-ec` / "
            "`change-theta-amb-c` family, `change-heat-recovery-eta` versus `change-heat-recovery-eta-min`, "
            "`change-hr-th` versus `change-storage-th`) are where a second reading "
            "written from the spelling rule alone can genuinely land on the wrong field."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. With sixty-two scalar kinds the restored "
            "document is always the before-document, so projecting only it would make every one of the sixty-two rows "
            "report the identical value and the differential would be vacuous — the mutated projection is the only "
            "half of the pair that distinguishes one row from the next."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads "
            "`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, 1,224 bytes of hand-authored residential demo data "
            "(`annex=de occupancy=residential comfort-category=II …`). Every one of the sixty-two fields is present, "
            "which is exactly what this case needs, but the values are illustrative rather than a measured building, "
            "so this is grammar and field-coverage evidence, not evidence about DIN EN 16798 conformity. The carrier "
            "has no published grammar: the committed `📖️component.grammar.semio` is the repository-wide "
            "`payload = OCTET+` placeholder, so identity is compared at the envelope preamble, the ordered "
            "`key=value` fields and the digest and length of the re-emitted bytes — deliberately not at a "
            "carrier-token-to-enum-spelling mapping, which no document in this repository states."
        ),
    ),
    "📙️din18599": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. Twelve of the "
            "thirteen kinds are scalar edits on the primary-energy balance (`change-ht`, `change-hv`, "
            "`change-system-losses-kwh`, `change-annual-limit-kwh`); the thirteenth, `update-climate`, is the only "
            "kind in the subset that does not address a value in the document at all — it addresses a HANDLE."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document: for the twelve scalar kinds the restored "
            "document is the before-document, so projecting only it would make all twelve rows report the same value. "
            "For `update-climate` the mutated projection is where the replacement handle becomes visible."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads "
            "`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, a 430-byte hand-authored residential demo. Note what "
            "is actually IN those bytes: `climate=[64696e...,64696e...]` is a pair of hex-encoded handles, not climate "
            "data — the child's own rows never appear in the carrier, so a byte-exact re-emission proves the handle "
            "was preserved and proves nothing whatever about the referenced table. That is this case's real evidence "
            "ceiling and it is narrower than the other fourteen norm subsets'. The carrier also has no published "
            "grammar: the committed `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` "
            "placeholder, so the two sides are compared on the envelope preamble, the ordered `key=value` fields and "
            "the digest and length of what each re-emitted, never on an inferred token-to-enum mapping."
        ),
    ),
    "📓️iso16757": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. Unlike the "
            "twelve Eurocode subsets, this vocabulary is almost entirely COLLECTION work: `create`/`delete`/`rename` "
            "triples over `products`, `product-groups`, `property-definitions` and `subjects`, plus "
            "`add`/`remove-selection-constraint` and `replace-part-number-rule`. Nine of the twenty-one kinds mint or "
            "destroy an id-keyed entity, so a second implementation has to agree on identity and on membership order, "
            "not merely on a number — which is why this subset, and not its scalar siblings, is where the carrier "
            "divergence below actually shows up."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. For the `create`/`delete` pairs the "
            "mutated projection is where a wrongly-placed re-insertion becomes visible; the restored projection alone "
            "would report the before-document for every row and say nothing about WHERE the entity came back."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads "
            "`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, 4,128 bytes of hand-authored demo catalogue — the "
            "largest committed document in this plugin, and the only one that exercises NESTED record blocks "
            "(`alternatives [locale:TEXT text:TEXT] { … }` inside `catalogue`, `manufacturer` and each product). It is "
            "a demo, not a shipped manufacturer catalogue, so it evidences the grammar and the nesting, not ISO 16757 "
            "conformity. The carrier has no published grammar: the committed `📖️component.grammar.semio` is the "
            "repository-wide `payload = OCTET+` placeholder, so identity is compared at the envelope preamble, the "
            "ordered `key=value` fields and the nested blocks as written, plus the digest and length of the re-emitted "
            "bytes — and the nesting is precisely what the recorded divergence above is about."
        ),
    ),
    "📔️vdi3805": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. This "
            "vocabulary is split three ways and no sibling subset is: `create`/`delete`/`rename-product` and "
            "`replace-product-configuration` work an id-keyed catalogue; `create`/`delete-geometry`, "
            "`resize-geometry`, `replace-geometry-parameters` and `add`/`remove-geometry-connection` work a per-product "
            "geometry graph; `create`/`delete-curve` and `replace-curve-points` work ordered point lists. Only "
            "`change-strict-mode`, `change-correction-as-of`, `change-edition-profile` and `update-limits` are flat "
            "scalars. A second implementation therefore has to reproduce three different addressing conventions here, "
            "not one."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. For the geometry and curve kinds the "
            "restored document is the before-document on every row, so the mutated projection is the only place a "
            "connection re-attached to the wrong endpoint or a point list restored in the wrong order can be seen."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads "
            "`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, 1,961 bytes of hand-authored demo (`manufacturer=DEMO "
            "record-count=3 building-system-number=system-code=\"420\" …`). Three records is enough to exercise the "
            "nesting and the derived index, and it is not a real VDI 3805 manufacturer file, so nothing here evidences "
            "conformity to the published data-exchange format. The carrier has no published grammar either: the "
            "committed `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so the two "
            "sides are compared at the envelope preamble, the ordered `key=value` fields and the nested blocks as "
            "written plus the digest and length of what each re-emitted — which is where the recorded divergence above "
            "is observed."
        ),
    ),
    "📘️en1990": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. This is the "
            "SMALLEST vocabulary in the plugin — ten kinds — and the only one whose collection half is entirely "
            "REFUSALS: `change-annex`, `change-permanent-action`, `change-resistance`, `change-consequence-class` and "
            "`change-seismic-action` apply, while `remove`/`change-category`/`change-value`/`reorder-variable-actions` "
            "are committed as `rejected` against an unseeded child slot and `insert-variable-action` is the recorded "
            "failure above. Four of the ten rows are therefore evidence that both sides REFUSE the same thing, which is "
            "weaker than four rows moving a document, and it is stated here rather than counted as if it were the same."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. Five rows are `rejected`, so their "
            "mutated and restored projections are identical by construction; projecting only the restored document "
            "would collapse the other five rows onto the same value too and leave the table saying nothing at all."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed "
            "`📚️examples/📕️high-consequence-office/🖼️assets/🗣️high-consequence-office.dsl.semio` — a named CC3 "
            "office case, not a generic demo, which is a stronger input than the four norm subsets that read "
            "`🎬️demo/🗣️example.dsl.semio`. What it still cannot carry is the `q_k` child: the carrier holds the "
            "handle, so a byte-exact re-emission says nothing about the variable actions themselves. The carrier has "
            "no published grammar either — the committed `📖️component.grammar.semio` is the repository-wide "
            "`payload = OCTET+` placeholder — so identity is compared at the envelope preamble, the ordered "
            "`key=value` fields and the digest and length of the re-emitted bytes, never at an inferred mapping from "
            "carrier tokens onto the JSON snapshot's enum spellings."
        ),
    ),
    "📘️en1991": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. All thirty-two "
            "kinds are flat `change-<field>` edits, but the field set is the most HETEROGENEOUS in the plugin: one "
            "document carries snow (`change-snow-zone`, `change-snow-altitude-m`), wind (`change-wind-zone`, "
            "`change-en-vbms`, `change-cd`, `change-cs`), thermal (`change-delta-tk`), crane (`change-crane-class`, "
            "`change-hoist-class`, `change-hoisting-speed-ms`), silo (`change-silo-k`, `change-silo-mu`, "
            "`change-silo-bulk-density-kn-m3`), bridge, accidental-impact and fire families side by side. The reading "
            "risk here is not spelling but SCOPE — an argument resolved into the wrong action family still names a "
            "real key."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the restored "
            "document is the before-document on all thirty-two rows; the mutated projection is the only half that "
            "tells the snow row from the silo row."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed "
            "`📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🗣️retail-hydrocarbon-fire.dsl.semio` — a named "
            "hydrocarbon-fire retail case rather than a generic demo, so the fire family (`change-fire-curve`, "
            "`change-fire-resistance-min`, `change-fire-member-capacity-c`) is exercised against a document that "
            "actually motivates it. It is still an authored case, not a submitted design. The carrier has no published "
            "grammar: the committed `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` "
            "placeholder, so the two implementations are compared at the envelope preamble, the ordered `key=value` "
            "fields and the digest and length of what each re-emitted — never at a carrier-token-to-enum mapping this "
            "repository nowhere states."
        ),
    ),
    "📘️en1992": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. All thirty-five "
            "kinds are flat `change-<field>` edits, and this subset's own difficulty is PREFIX FAMILIES: ten "
            "`change-anchor-*` keys, five `change-liquid-*` keys and two `change-bridge-*` keys sit in the same "
            "document as the bare section keys they shadow (`change-anchor-as-mm2` beside `change-as-mm2`, "
            "`change-anchor-d-mm` beside `change-d-mm`). Resolving `new_as_mm2` by normalised spelling has to land on "
            "the bare key and not on the anchor one, and that is a genuine way for an independent reading to go wrong."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the restored "
            "document repeats the before-document on all thirty-five rows; only the mutated projection distinguishes "
            "`change-as-mm2` from `change-anchor-as-mm2`, which is exactly the confusion this case exists to catch."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed "
            "`📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🗣️liquid-retaining-fem-anchor.dsl.semio` — a named "
            "case that carries the liquid-retaining, FEM and anchor field families at once, which is why every prefix "
            "family above is present in one document instead of being split across fixtures. It is an authored case, "
            "not a submitted design. The carrier has no published grammar: the committed "
            "`📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so identity is "
            "compared at the envelope preamble, the ordered `key=value` fields and the digest and length of the "
            "re-emitted bytes, never at an inferred token-to-enum mapping."
        ),
    ),
    "📘️en1993": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. This subset has "
            "a SHAPE no other norm subset has: sixteen of its seventeen kinds are `update-<group>-inputs` — "
            "`update-bolt-inputs`, `update-weld-inputs`, `update-plated-inputs`, `update-silo-shell-inputs`, "
            "`update-tower-inputs` and eleven more — each replacing a whole nested input RECORD, not a scalar. Only "
            "`change-annex` is a flat field. A second implementation therefore has to agree on record-level "
            "replacement semantics (which sub-fields the payload overwrites and which it leaves standing), a question "
            "the twelve scalar-only subsets never ask."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. For a record-replacing `update-` kind the "
            "restored document is the before-document, so only the mutated projection shows whether the replacement "
            "overwrote the whole record or merged into it — the single most likely place these two implementations "
            "could differ."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed "
            "`📚️examples/📕️high-strength-connection/🖼️assets/🗣️high-strength-connection.dsl.semio` — a named "
            "high-strength bolted-connection case, so the bolt, weld and through-thickness input records are populated "
            "rather than defaulted. It is an authored case, not a fabricator's submission. The carrier has no "
            "published grammar: the committed `📖️component.grammar.semio` is the repository-wide "
            "`payload = OCTET+` placeholder, so the two sides are compared at the envelope preamble, the ordered "
            "`key=value` fields and the nested input blocks as written, plus the digest and length of what each "
            "re-emitted — and for this subset the nested blocks are the whole point."
        ),
    ),
    "📘️en1994": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. All twenty-two "
            "kinds are flat `change-<field>` edits over a composite steel-concrete section, and the specific reading "
            "hazard here is the STUD family: `change-delta-tau-stud-mpa`, `change-n-cycles-stud` and "
            "`change-v-ed-per-stud-kn` sit beside the member-level `change-delta-sigma-mpa`, `change-v-ed-kn` and "
            "`change-m-ed-knm` they resemble. Resolving `new_v_ed_kn` must land on the member key, not the per-stud "
            "one."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the restored "
            "document repeats the before-document on all twenty-two rows and only the mutated projection separates the "
            "member-level row from the per-stud row that shadows it."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed "
            "`📚️examples/📕️composite-bridge-girder/🖼️assets/🗣️composite-bridge-girder.dsl.semio` — a named "
            "composite bridge-girder case, which is why the fatigue keys (`change-fatigue-detail`, "
            "`change-n-cycles-stud`, `change-delta-sigma-mpa`) carry real values instead of defaults. It is an "
            "authored case, not a designed bridge. The carrier has no published grammar: the committed "
            "`📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so identity is "
            "compared at the envelope preamble, the ordered `key=value` fields and the digest and length of the "
            "re-emitted bytes, never at an inferred token-to-enum mapping."
        ),
    ),
    "📘️en1995": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. All twenty "
            "kinds are flat `change-<field>` edits, and this subset is the one whose fields are mostly SHORT, "
            "unpunctuated symbols out of the timber code — `fmk`, `fvk`, `fc0-k`, `chi`-free but `w-mm3`, `a-mm2`, "
            "`a-ef-mm2`, `h-mm`, `b-mm`. Normalised spelling has less to work with here than anywhere else in the "
            "plugin (`a-mm2` versus `a-ef-mm2` differ by two characters), which is exactly the case a second reading "
            "written from the naming mechanic alone has to survive."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the restored "
            "document repeats the before-document on all twenty rows; the mutated projection is what tells `a-mm2` "
            "from `a-ef-mm2`."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed "
            "`📚️examples/📕️glulam-footbridge/🖼️assets/🗣️glulam-footbridge.dsl.semio` — a named glulam footbridge, "
            "so the vibration and fatigue keys (`change-a-vert-ms2`, `change-n-cycles-bridge`) and the "
            "`change-service-class`/`change-load-duration` pair are populated by a document that motivates them. It is "
            "an authored case, not a built bridge. The carrier has no published grammar: the committed "
            "`📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so the two sides are "
            "compared at the envelope preamble, the ordered `key=value` fields and the digest and length of what each "
            "re-emitted, never at an inferred token-to-enum mapping."
        ),
    ),
    "📘️en1996": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. All twenty-two "
            "kinds are flat `change-<field>` edits, and half of this subset's fields are ENUM-valued rather than "
            "numeric — `change-unit`, `change-mortar`, `change-masonry-class`, `change-exposure`, "
            "`change-design-situation`, `change-annex`. That matters because an independent implementation resolves "
            "the FIELD by normalised spelling but must reproduce the VALUE's spelling exactly, and an enum is the one "
            "place where a plausible-looking near-miss survives a numeric comparison."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the restored "
            "document repeats the before-document on all twenty-two rows; for the six enum-valued kinds the mutated "
            "projection is the only place the written spelling of the new value is observable."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed "
            "`📚️examples/📕️loadbearing-wall/🖼️assets/🗣️loadbearing-wall.dsl.semio` — a named load-bearing masonry "
            "wall, so the enum fields above carry real spellings rather than defaults. It is an authored case, not a "
            "surveyed wall. The carrier has no published grammar: the committed `📖️component.grammar.semio` is the "
            "repository-wide `payload = OCTET+` placeholder, so identity is compared at the envelope preamble, the "
            "ordered `key=value` fields and the digest and length of the re-emitted bytes — and for an enum-heavy "
            "subset that byte-level comparison is doing more work than it does elsewhere in this plugin."
        ),
    ),
    "📘️en1997": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. All twenty-two "
            "kinds are flat `change-<field>` edits over one geotechnical record, and the reading hazard is the PILE "
            "family: `change-pile-base-area-m2`, `change-pile-dm`, `change-pile-lm` and `change-pile-n-profiles` sit "
            "beside the spread-footing keys (`change-footing-area-m2`, `change-b-m`, `change-dfm`) they mirror. "
            "`change-design-approach` is the one enum, and it selects which of the two families a verification even "
            "reads."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the restored "
            "document repeats the before-document on all twenty-two rows; the mutated projection is what separates the "
            "pile row from the footing row that mirrors it."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads "
            "`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, 330 bytes — the smallest committed document in the "
            "plugin, a single line of demo values (`v-ed-kn=500 h-ed-kn=80 footing-area-m2=2 phi-deg=30 …`). It "
            "touches every field once and nothing more: no ground-investigation record, no real design approach "
            "comparison, no second soil layer. Unlike its nine Eurocode siblings this subset has NO named example "
            "case, and that is a real gap in its evidence rather than a stylistic difference. The carrier has no "
            "published grammar either: the committed `📖️component.grammar.semio` is the repository-wide "
            "`payload = OCTET+` placeholder, so the two sides are compared at the envelope preamble, the ordered "
            "`key=value` fields and the digest and length of what each re-emitted."
        ),
    ),
    "📘️en1998": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. Forty-nine "
            "flat `change-<field>` kinds make this the widest Eurocode vocabulary here, and they are grouped by "
            "STRUCTURE TYPE rather than by quantity: `change-silo-*` (six), `change-tank-*` (four), `change-tower-*` "
            "(six), `change-wall-*` (five), `change-bearing-*` (two), `change-foundation-*` (four) and "
            "`change-retrofit-*` (five) all coexist with the bare frame keys. Six of those groups carry a `v-rd-kn` or "
            "`m-rd-knm` resistance of their own, so an argument resolved into the wrong group still names a key of the "
            "right shape — the sharpest version of the scope hazard in this plugin."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the restored "
            "document repeats the before-document on all forty-nine rows; the mutated projection is the only half that "
            "distinguishes `change-silo-v-rd-kn` from `change-bridge-v-rd-kn` from the bare `change-v-rd-kn`."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed "
            "`📚️examples/📕️seismic-rc-frame/🖼️assets/🗣️seismic-rc-frame.dsl.semio` — a named reinforced-concrete "
            "frame. Be precise about what that means for the forty-nine kinds above: the frame case populates the "
            "spectrum, ground-type, mass and drift keys, while the silo, tank, tower and retrofit groups are carried "
            "at their committed defaults, so identity evidence for those groups is thinner than the mutate/inverse "
            "evidence is. The carrier has no published grammar: the committed `📖️component.grammar.semio` is the "
            "repository-wide `payload = OCTET+` placeholder, so the two sides are compared at the envelope preamble, "
            "the ordered `key=value` fields and the digest and length of what each re-emitted."
        ),
    ),
    "📘️en1999": dict(
        reading=(
            "Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below "
            "is a declared `asset://` fixture, so neither side holds a transcription that could drift. All twenty-six "
            "kinds are flat `change-<field>` edits, and this subset is the one where the SAME quantity appears three "
            "times under different qualifiers: `change-m-ed-knm` (member) beside `change-sheet-m-ed-knm` (sheeting), "
            "`change-w-el-mm3` beside `change-sheet-w-el-mm3`, `change-delta-sigma-ed` beside "
            "`change-sigma-ed-shell-mpa`, plus a weld group (`change-weld-throat-mm`, `change-weld-length-mm`, "
            "`change-v-weld-ed-kn`) and a shell group (`change-shell-r-mm`, `change-shell-t-mm`). Aluminium's "
            "`change-alloy` is the one enum and it is what makes the rest of the record mean anything."
        ),
        inverse=(
            "`inverse-` projects BOTH the mutated and the restored document. Every kind is scalar, so the restored "
            "document repeats the before-document on all twenty-six rows; the mutated projection is the only half that "
            "tells the member `m-ed-knm` from the sheeting one."
        ),
        carrier=(
            "⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed "
            "`📚️examples/📕️aluminium-roof-purlin/🖼️assets/🗣️aluminium-roof-purlin.dsl.semio` — a named aluminium "
            "roof purlin, so `change-alloy`, the section keys and the sheeting group are populated by a document that "
            "motivates them; the shell and weld groups ride at their committed defaults, which is a real limit on what "
            "this file's identity evidence covers. The carrier has no published grammar: the committed "
            "`📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so identity is "
            "compared at the envelope preamble, the ordered `key=value` fields and the digest and length of the "
            "re-emitted bytes, never at an inferred token-to-enum mapping."
        ),
    ),
}

TAIL_OLD_START = "It imports nothing from the Rust it judges"
TAIL_NEW = (
    "It imports nothing from the Rust it judges and transliterates none of it: the document field a `new*` argument "
    "names is resolved by normalised spelling against the document's own keys, which is what the naming mechanic "
    "states, never from a table copied out of `🧬️mutations/**` — and the paragraph below names the spellings in THIS "
    "subset where that resolution can genuinely go wrong. The recorded no-oracle decision it replaces is gone from "
    "`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, because there is now a reference to "
    "compare against."
)

LAWS = (
    "Each side then asserts the same three laws in role — the applied document must BE the committed after-snapshot; "
    "an `applied` vector must move the document and a `rejected` one must leave it bit-identical; and the mutation "
    "followed by its OWN computed inverse must restore the before-snapshot exactly. What `parity` adds on top is the "
    "only thing a single implementation can never provide: that two implementations, in two languages, written from "
    "one written specification, reach the same document."
)


def wrap(text: str) -> str:
    return "\n".join("  " + line for line in textwrap.wrap(text, width=98, break_long_words=False, break_on_hyphens=False))


def paragraphs(body: str):
    return body.split("\n\n")


def rewrite(path: str, facts: dict) -> bool:
    original = open(path, encoding="utf-8").read()
    lines = original.split("\n")
    start = next(i for i, l in enumerate(lines) if l.startswith("Feature:"))
    end = next(i for i in range(start + 1, len(lines)) if lines[i].lstrip().startswith("@"))
    head, body, tail = lines[: start + 1], "\n".join(lines[start + 1 : end]).rstrip("\n"), lines[end:]

    out = []
    for para in paragraphs(body):
        flat = " ".join(para.split())
        if TAIL_OLD_START in flat:
            keep = flat.split(TAIL_OLD_START)[0].strip()
            out.append(wrap(keep + " " + TAIL_NEW))
        elif flat.startswith("Both implementations read the SAME committed bytes"):
            out.append(wrap(facts["reading"] + " " + LAWS))
        elif flat.startswith("`inverse-` projects BOTH"):
            out.append(wrap(facts["inverse"]))
        elif flat.startswith("⚠️ Honest boundary — the CARRIER"):
            out.append(wrap(facts["carrier"]))
        else:
            out.append(para)

    rewritten = "\n".join(head + ["\n\n".join(out), ""] + tail)
    if rewritten != original:
        open(path, "w", encoding="utf-8").write(rewritten)
        return True
    return False


changed = 0
for artifact, facts in FACTS.items():
    matches = glob.glob(os.path.join(ROOT, artifact, "🧪️tests", "*", "component.feature"))
    assert len(matches) == 1, (artifact, matches)
    if rewrite(matches[0], facts):
        changed += 1
print(f"rewrote {changed} of {len(FACTS)} norm features")
