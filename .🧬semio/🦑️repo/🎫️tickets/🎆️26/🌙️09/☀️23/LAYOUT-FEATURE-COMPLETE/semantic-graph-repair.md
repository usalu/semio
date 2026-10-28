# Semantic Graph Repair

Implemented all nine concrete witnesses from semantic-graph-review.md in the shared Layout model and semantic mutation boundaries. Payload schemas and wire types are unchanged. Work is scoped to Layout, not IFC.

## Behavior

- UpdateTextFrame walks the candidate target chain to reject transitive and cross-page cycles. It avoids rebuilding the chain index when the thread edge is unchanged.
- DeleteFrame and DeletePage refuse deletion when a surviving local text frame points into the removed frame set. Page-internal threads may be deleted together. Refused inverses are empty.
- CreateFrame checks global local-frame identity, inherited/local identity collisions, owning layer, referenced story/link, finite positive bounds, finite colors, positive columns, finite nonnegative inset sizes and the candidate outgoing thread chain. An explicit layerId is applied before validation.
- CreatePage checks its own page graph and candidate combined thread graph, requires its spread, and appends spread membership in the same diff as page insertion. DeletePage removes membership in the same diff as page removal. Semantic inverse restores both document page order and spread page order; an extra SetPageSpread step is retained only when restoring a non-final spread position.
- EditStory resolves Unicode scalar positions into UTF-8 bytes and publishes content plus adjusted style ranges atomically. Each run keeps surviving prefix/suffix and ordered overlaps. Inserted text inherits runs covering the right character at the splice boundary, with left inheritance at EOF. Zero-length anchors move with the edit. Fully removed spans without inherited text disappear. Semantic inverse retains original runs plus the reverse text edit, replayed in framework storage-reversed order. Unformatted text needs only the reverse edit.
- CreateStory and SetStoryRuns share UTF-8 boundary and style-reference validation with whole-document admission. CreateLink rejects empty/whitespace identity without imposing image metadata limits.
- Whole-document validation reuses the same story/page helpers; per-story reference sets remain shared during complete validation. Ordinary mutations do not run whole-document validation.

## Test-First Specification and Executed Evidence

The neutral corpus was created before the production helpers and guards. It first contained 44 cases; it now contains 49 cases (17 accepted, 32 refused), including all nine review witnesses, cross-page chains/inbound removal, page/frame references and collisions, geometry, missing styles and UTF-8 style splices with overlapping and empty spans. Rules are committed inside the corpus. Native test source consumes exactly those before/mutation/after documents and asserts actual central diff application, canonical graph validity, immutable inputs, inverse diff restoration, storage-reversed semantic inverse with valid intermediate graphs, mutation text/binary transport and sparse diff transport. Additional native sources cover NaN/infinities and overflowing character offsets.

Executed through Node on installed libraries, using the retained semantic-graph-check.mjs harness:

- Independent semantic oracle: 49 cases,17 accepted, 32 refused. fast-json-patch3.1.1 independently reproduces each expected after-document; Ajv8.20.0 validates graph projections, Kahn ordering validates thread acyclicity, and a scalar-token annotation oracle independently reproduces formatting outcomes without native interval arithmetic. All external imports are test-only behind the owned oracle interface.
- Existing canonical validation oracle: 61 document and 9 parent witnesses still pass.
- Golden corpus audit: all 102 existing before/after snapshots across 51 scenarios now pass the independent graph validator. They were previously missing every spread row despite page spread references. Authored spread rows and memberships were supplied consistently, preserving empty spread rows across mutations. The three affected create/delete page sparse diffs were updated. The Python test-only page projection now updates spread membership as well; it was source-reviewed but not executed here.
- web-tree-sitter0.20.8 with installed Rust grammar: 15 changed Rust source files parse with no grammar errors. This is syntax verification only. Maximum changed Rust path 146 characters; maximum existing repaired golden snapshot path 209 characters.

The harness writes no generated outputs. No native compiler, native test, or Python test pass is claimed. The cached repository executable is known stale/silent, and native execution remains unavailable in this session. No build/repository infrastructure or AGENTS files were edited; no git modification or shell fallback was used.

## Remaining Verification

Run the native Layout model/mutations tests through a functioning rebuilt repo runner, including the new semantic_graph_tests module, existing fixture codec matrix and protocol inverse laws. The registered Bun wrapper has not been run by Bun here; its owned JavaScript oracle was executed directly. Native type checking and actual mutation/diff/undo runtime behavior remain unverified until that runner works.

## Exact Files

Paths below are absolute. The102 golden snapshot JSON files and3 sparse page diff JSON files are intentional source fixture changes, not copied/generated ticket artifacts.

- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-page/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-page/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✍️edit-story/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-frame/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-frame/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️update-text-frame/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖create-story/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️create-link/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️set-story-runs/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✂️story-splice/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🕸️graph/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🕸️graph/oracle.mjs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🕸️graph/🟦️.ts
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📐️mutate-layout-1/🐍️.py
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧾change-data-fields/🧾️attaches/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧾change-data-fields/🧾️attaches/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🛤️change-link-path/🔗️relinks/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🛤️change-link-path/🔗️relinks/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/🫓️refuses/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/🫓️refuses/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/🚫️rejects/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/🚫️rejects/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/🗜️doubles/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/🗜️doubles/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/⚠️skips/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/⚠️skips/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/⏸️keeps-unit-factors/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/⏸️keeps-unit-factors/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/↔️stretches/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗜️scale-frames/↔️stretches/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-page/🚫️removes-page-2/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-page/🚫️removes-page-2/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-page/🚫️removes-page-2/🔺️diff/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-page/📍️removes-a-middle-row/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-page/📍️removes-a-middle-row/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-page/📍️removes-a-middle-row/🔺️diff/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-character-style/📍️removes-a-middle-row/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-character-style/📍️removes-a-middle-row/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖨️change-print-target/🖨️sets/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖨️change-print-target/🖨️sets/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖊️change-frame-stroke/🖊️adds/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖊️change-frame-stroke/🖊️adds/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖇️create-link/🔗️appends-link-3/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖇️create-link/🔗️appends-link-3/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🕹️move-frame/📍️moves-the-rect-frame/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🕹️move-frame/📍️moves-the-rect-frame/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔤change-frame-wrap-mode/🔤️switches/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔤change-frame-wrap-mode/🔤️switches/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔢change-frame-columns/🔤️splits/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔢change-frame-columns/🔤️splits/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔒set-frame-flags/🔒️locks-frame-1/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔒set-frame-flags/🔒️locks-frame-1/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️rotate-frame/🌀️rotates/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️rotate-frame/🌀️rotates/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/🚫️rejects/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/🚫️rejects/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/🔃️orbits/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/🔃️orbits/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/🔁️refuses/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/🔁️refuses/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/🌀️turns/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/🌀️turns/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/⚠️skips/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/⚠️skips/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/⏸️keeps-a-zero-angle/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔃️rotate-frames/⏸️keeps-a-zero-angle/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔀reorder-pages/🔀️moves/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔀reorder-pages/🔀️moves/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔀reorder-pages/📍️moves-a-middle-row/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔀reorder-pages/📍️moves-a-middle-row/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📖create-story/📖️appends-story-3/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📖create-story/📖️appends-story-3/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📕delete-story/🚫️removes-story-2/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📕delete-story/🚫️removes-story-2/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📕delete-story/📍️removes-a-middle-row/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📕delete-story/📍️removes-a-middle-row/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐update-page-margins/📐️sets/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐update-page-margins/📐️sets/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐update-grid/📐️sets/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐update-grid/📐️sets/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📏resize-frame/📐️resizes-the-rect-frame/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📏resize-frame/📐️resizes-the-rect-frame/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏷️rename-page/🏷️renames-page-1/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏷️rename-page/🏷️renames-page-1/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏛️update-page-columns/🏛️splits/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏛️update-page-columns/🏛️splits/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🎨change-frame-fill/🎨️repaints/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🎨change-frame-fill/🎨️repaints/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌱create-page/➕️appends-page-3/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌱create-page/➕️appends-page-3/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌱create-page/➕️appends-page-3/🔺️diff/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➖delete-frame/🚫️removes/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➖delete-frame/🚫️removes/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➖delete-frame/📍️removes-a-middle-row/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➖delete-frame/📍️removes-a-middle-row/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕create-frame/🔲️inserts/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕create-frame/🔲️inserts/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️rename-layout/🏷️renames/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️rename-layout/🏷️renames/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✍️edit-story/📝️rewrites-story-1-body/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✍️edit-story/📝️rewrites-story-1-body/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-frames/🚫️rejects/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-frames/🚫️rejects/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-frames/🔁️refuses/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-frames/🔁️refuses/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-frames/✋️drags-both-frames/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-frames/✋️drags-both-frames/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-frames/⚠️skips/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-frames/⚠️skips/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-frames/⏸️keeps-a-zero-offset/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✋️drag-frames/⏸️keeps-a-zero-offset/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✂️delete-link/🔗️removes-link-2/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✂️delete-link/🔗️removes-link-2/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✂️delete-link/📍️removes-a-middle-row/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✂️delete-link/📍️removes-a-middle-row/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/↕️change-page-height/↕️lengthens/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/↕️change-page-height/↕️lengthens/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/↔️change-page-width/↔️widens-page-1/📸️snapshot/⬅️before/🔣️.json
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/↔️change-page-width/↔️widens-page-1/📸️snapshot/➡️after/🔣️.json
- C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/semantic-graph-check.mjs


## Continuation Evidence

The current committed corpus is 57 semantic graph cases (17 accepted, 40 refused), plus three nonfinite gutter word cases. Additional column-count, page-parent assignment and selection-output overflow guards are mounted in the native mutations/shared targeted validator. Latest executed independent checker confirms 61 complete documents, nine parent cases and all 102 golden snapshots. The strict executable TypeScript counterpart passes 96 portable, 51 golden, all 58 variants, 40 word cases and two drawing cases. Rust grammar parsing now covers 42 affected files without errors; native compilation and tests remain unrun. See `ieee-parity.md` and `semantic-typescript.md` for exact continuation scope and limitations.
