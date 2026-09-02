# Shard A10 — Oracle Honesty And The Residual Stragglers

## Before / after (measured via `bun ./📜️script.ts test contract`, filtered to shard A10's ids)

| id | before | after |
| --- | --- | --- |
| `reimplementation-registered-as-third-party` | 20 | 2 (documented false positives, see below) |
| `missing-external-oracle` | 13 | 13 (genuine, now-honest gap — see below) |
| `no-oracle-covers-mutation` | 3 | 0 |
| `oracle-capability-mismatch` | 23 | 0 |
| `unknown-oracle` | 2 | 0 |
| `claimed-implementations-missing` | 1 | 0 |
| `missing-fixture` (mathematical, note only) | 2 | 0 |
| `orphan-fixture` | 1 | 0 |
| `fixture-file-missing` | 1 | 0 |
| `case-slug` | 2 | 0 |

Total repo breach count moved 1453 → 1296 over the course of this shard (other shards' concurrent
work moved it independently in between; both counts were read from a fresh `test contract` run, not
assumed). None of `oracle-in-production` (316, pre-existing, unrelated files), `oracle-profile-mismatch`
(0), `fixture-generated-by-non-qualifying-oracle` (0) or `fixture-generator-unregistered` (0) were
raised by this shard's edits — checked explicitly after every batch of changes, including two
regressions caught and reverted mid-session (see "Mistakes made and corrected" below).

## The 20 reimplementation-registered-as-third-party cases

Read every one of the 20 flagged `🦀️oracle.rs` files directly. All 20 share the exact same shape: a
hand-written `apply`/`apply_kind`/`forward` dispatch that computes each mutation's expected RESULT
itself, operating on an owned struct/tree — the registered crate (`html5ever`, `zip`, `quick-xml`,
`ruststep`, `calamine`, …) is used only to parse and re-serialize bytes, never to decide what a
mutation should produce. The catch-all arm of every one of these dispatches is the literal
`"mutation kind {other:?} has no oracle implementation"` — the exact string the breach detector's
`predicts` regex looks for — confirming this is a systemic pattern, not 20 independent judgment
calls.

**Verdict: all 20 are genuine re-implementations, not third-party wrappers.** None of the 20 crates
independently computes mutation semantics; all merely codec the artifact.

Fix applied to all 20: reclassified `kind` from `third-party-library` to `cross-semio-implementation`
in each owner's `🧪️oracle/🔣️.json` (`html`, `zip`×2, `pptx`×3, `svg`×2, `ifc`×5, `step`, `xlsx`×3,
`docx`×2, `xml`), with a rationale note explaining the reclassification and pointing at this file.
`capabilities` were left untouched (see "Mistake #1" below for why).

Verified this creates **zero** new `missing-external-oracle` breaches: none of these 20 owners
populate `mutationManifests` (only `mutationCatalogs`, the older v1 vocabulary), so no
`oracleRequirements` entry anywhere references these capabilities — the false qualifying claim was
purely a registry-metadata lie with no downstream mutation actually gated on it.

### The 2 that remain flagged (ifc/2x3/✳️any, ifc/4/✳️any)

Both of these owners genuinely ALSO carry `ifcopenshell-ifc-2x3-any-differential` /
`ifcopenshell-ifc-4-any-differential` — a real, independently-produced second producer: a Python
script (`🧪️tests/differential-ifc-2x3/🐍️.py`, `differential-ifc-4/🐍️.py`) that shells out to
IfcOpenShell (a genuine open-source C++/Python IFC engine, 0.8.4.post1) to read AND write IFC2X3/IFC4
from its own independent Part-21 parser — verified by reading the Python script and the oracle's own
rationale, which documents a real measured byte-level round-trip (193 915 → 188 288 bytes) and one
honestly-excluded kind (`remove-instance`, which IfcOpenShell repairs references on and the subject
deliberately does not).

The detector still flags this contribution because `reimplementationOracleBreaches` operates at
**file granularity** (`join(contribution.owner, "🧪️oracle", "🦀️.rs")`, one path per owner) rather
than per-oracle-entry: since the SAME `🦀️oracle.rs` also hosts the (now correctly reclassified)
`ruststep-*` predicting dispatch, the whole contribution is flagged, sweeping the legitimately
independent `ifcopenshell-*` entry in with it. The sanctioned escape hatch
(`judgedByProbes`: registered `probes` + `comparisonPipelines` with a `qualified` status) is a
larger, separately-verifiable schema retrofit — the same shape as the PNG precedent
(`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/`) — that
would need its own measured qualification evidence, not a copy-paste. Left open and documented here
rather than force-fit with an unverified claim.

## missing-external-oracle (13) and no-oracle-covers-mutation (3): one problem, now honestly separated

Read `mathematical-mutation-semantics`, `sequence-step-graph-mutation-semantics` and
`draw-mutation-semantics` in full, and cross-referenced every mutation's real `oracleRequirements`
across every subset of `mathematical`, `sequence` and `draw` (a concurrent shard split these three
artifacts into subsets mid-session — re-read fresh each time rather than trusting an earlier
snapshot).

Findings, per owner:

- **`draw`**: every one of its 14 kinds is now genuinely discharged — 11 by `quick-xml-draw-1-mutate`
  (a real SVG bridge, `draw_document_to_svg`) and 3 (`rename-layer`, `set-layer-locked`,
  `set-layer-blend-mode`) by `serde-json-draw-carrier-reader`. `draw-1-mutate-uncarried` is not
  required by any real mutation at all — vestigial. The no-oracle decision was claiming capabilities
  that are either fully discharged elsewhere or entirely unused.
- **`mathematical`**: `mathematical-1-mutate` (create-node, delete-node, delete-nodes,
  change-node-label, move-node) is genuinely discharged by `csv-rfc4180-mathematical-1-mutate` (a real
  RFC 4180 CSV export/reader). `mathematical-1-mutate-carrier` (change-coefficient) is genuinely
  discharged by `serde-json-mathematical-carrier-reader`. `mathematical-1-mutate-uncarried` is
  **genuinely required** by 9 real mutations (4 in `✳️geometry`: `replace-points`, `insert-point`,
  `remove-point`, `move-point`; 5 in `✳️graph`: `change-graph-directed`, `update-graph-algorithm`,
  `replace-graph`, `connect-nodes`, `disconnect-nodes`) and **nothing discharges it**.
- **`sequence`**: `sequence-1-mutate` is genuinely discharged by `csv-rfc4180-reader`.
  `sequence-1-mutate-uncarried` is genuinely required by 4 real mutations (2 in `✳️dependency`:
  `connect-steps`, `disconnect-steps`; 2 in `✳️step`: `change-step-collapsed`, `move-step`) and
  nothing discharges it.

**The `no-oracle-covers-mutation` rule's own point is exactly this: a no-oracle decision may never
claim a capability a real mutation requires, because that makes an open gap look handled.** Fix
applied: narrowed all three decisions' `capabilities` to `[]` — every capability that WAS discharged
elsewhere no longer needs an excuse, and every capability that IS genuinely gapped is no longer
smoothed over by a decision; the rationale text (a real, substantial research trail — prior surveys
of `petgraph` and external CAS candidates, both declined; what a second implementation would need;
what currently blocks one) was kept and appended with a note explaining the narrowing, so the history
isn't lost.

This is a **disposition, not a fix**: the 13 `missing-external-oracle` mutations (graph-attribute
edits — `directed`/`algorithm`/`seed`, arbitrary edge lists, point-cloud geometry, an equation AST —
that no exported carrier encodes) remain genuinely un-oracled. I did not find, in the time available,
a real third-party library beyond what the prior survey already declined, and did not attempt to
fabricate a qualifying registration to make the number move — that would recreate exactly the
`reimplementation-registered-as-third-party` problem this shard exists to remove. What changed is
that the gap is now the ONLY thing claiming these 13 mutations — no no-oracle decision papers over it
— which is the second law's whole point: a real, visible gap instead of a hidden one.

## unknown-oracle (2)

- **`@oracle-mercantile`** (`🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/web-mercator-tile-oracle`):
  `mercantile` (pure-Python, zero-runtime-dep, pinned `>=1.2.1` in `pyproject.toml`'s `test` group,
  genuinely invoked in `🐍️.py`) was never registered — this module had no `🧪️oracle/` contribution
  directory at all. Created
  `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️oracle/🔣️.json` registering it
  `third-party-library`, capability `web-mercator-tile-selection`, comparison profile
  `floating-point-v1` (the feature's own tag). Also found and fixed a real dead reference while here:
  the feature's prose and the `🐍️.py` adapter both pointed at `🔣️vectors.json`, but the committed
  fixture is `🔣️.json` (kind-only basename) — the Python script would have thrown `FileNotFoundError`
  at runtime. Fixed both, and added a `local://🔣️.json` reference so the same fixture is no longer
  `orphan-fixture` either (see below).
- **`@oracle-semio-mesh-typescript-three-independent`**
  (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-mesh`): the feature's own prose
  says this id replaced a recorded no-oracle decision that is "already gone" — confirmed: the mesh
  subset's `noOracleDecisions` array is empty, but nobody added the replacement oracle entry. The
  second producer is real: `🟦️.ts` beside the feature is a from-scratch TypeScript reimplementation of
  the carrier/pack-frame/17-verb vocabulary, pinned against the committed `🧊️cube` fixture,
  importing nothing from the Rust subject. Registered it in
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧪️oracle/🔣️.json`
  as `kind: cross-semio-implementation` (a required supplement, honestly — not `third-party-library`,
  since it is this repository's own second implementation of a semio-native format nobody else
  speaks). Did not touch this subset's `fixtureManifests`, per the brief's instruction to leave
  🧿️semio's fixtures to other shards.

## claimed-implementations-missing (1)

`os-config-identity-mutation-semantics`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/mutate-os-config-identity`) claimed
the `independent-implementations` substitute but the case had only the one `🦀️.rs` adapter. The claim
was true in spirit — `applyIdentityConfigMutation`/`inverseIdentityConfigMutation`
(`🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🟦️.ts`) is a real, independently-written
TypeScript leaf for the same `signIn`/`signOut` vocabulary — but no adapter in the CASE directory
exercised it. Added `🟦️.ts` beside the existing `🦀️.rs`: a second SUBJECT implementation that reads
the identical committed `(before, mutation, after, outcome)` specification vectors (mirroring the
Rust adapter's own `include_str!`-style direct read, never re-derived) and asserts the same laws —
applied-record-matches-after, account-claim, outcome-status, and the inverse-restores-exactly law —
through the TypeScript dispatch instead. Verified it bundles clean with `bun build`.

## Stragglers

- **`missing-fixture` (➗️mathematical, 🗒️note)**: both referenced
  `asset://…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`; the committed file is
  `🗣️.dsl.semio` (kind-only basename, same wave-0 migration pattern the earlier ticket wave fixed
  everywhere else — these two features were missed). Repointed both.
- **`orphan-fixture` (🗺️tiled-map)**: same case as the mercantile registration above — fixed by
  adding the `local://🔣️.json` reference once the filename itself was corrected.
- **`fixture-file-missing` (📐️step/✳️cc6)**: the manifest pointed at
  `…/🧫️fixtures/📐️hexagonal-cut-concrete-forest-left-ap214.stp` (flat file); the real committed
  file is at `…/🧫️fixtures/🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp` (moved into a
  per-fixture directory with a kind-only basename during a prior restructuring, reference never
  updated). Verified byte-identical (sha256 and size both match the manifest's pinned digest) before
  repointing — this is the same file, not a substitute.
- **`case-slug` (🎠️kernel, ×2)**: `🧪️reject-malformed-version-input` and
  `🧪️satisfy-version-requirements` were emoji-prefixed duplicates sitting alongside empty,
  correctly-named stub directories (`reject-malformed-version-input`,
  `satisfy-version-requirements`) — evidence of an incomplete prior rename. Removed the empty stubs
  and `git mv`'d the real content into the kebab-case names. Confirmed nothing else in the repo
  referenced the old emoji-prefixed paths.

## Mistakes made and corrected mid-session (kept here for the record)

1. **Renamed capabilities, not just kind, on the first pass of the 20 reimplementation fixes** —
   appended `-second-implementation` to match the PNG/GLTF precedent exactly. This broke 20 feature
   files' `@oracle-<id>` tags (they still asked for the plain capability), spiking
   `oracle-capability-mismatch` 23 → 43. Reverted the capability rename (kept only the `kind` change);
   confirmed via the actual `oracleRequirementBreaches` source that `oracle-capability-mismatch` only
   checks capability-name membership, not `kind`, so the rename was never necessary for the primary
   fix.
2. **Renamed `lopdf-pdf-1-7-base-mutate-reader`'s id while fixing its capability typo**, without
   checking whether anything else referenced the old id. It did: 17 `fixtureManifests` entries named
   it as their `generator.oracle`, so the rename spiked `fixture-generator-unregistered` 0 → 17.
   Reverted the id rename, kept the capability fix, retagged the feature to the (unrenamed) real id.
3. **When retagging the 22 other `oracle-capability-mismatch` features to their reader oracle**, 9 of
   those readers (`gif-87a-any-mutate-reader` and 8 `lopdf-pdf-1-*-mutate-reader` entries) turned out
   to have their capability typo'd with a spurious `-reader` suffix — and, discovered only after
   fixing the oracle's `capabilities` field, their `mutationManifests[].oracleRequirements[].capability`
   carried the SAME typo, so stripping only the oracle side broke the self-consistent (if
   feature-mismatched) pair, spiking `missing-external-oracle` 13 → 115 across those subsets (plus a
   concurrent shard's unrelated mathematical/sequence subset split, confirmed separately). Fixed the
   `oracleRequirements` capability strings to match, restoring `missing-external-oracle` to its
   correct 13.

All three were caught by re-running `test contract` after each batch rather than assuming success —
per the ticket's own rule, measured, not asserted.

## Files touched

Reclassified (kind only, `third-party-library` → `cross-semio-implementation`) — 20 files:
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🌐️html,🎒️zip×2,🎞️pptx×3,🎨️svg×2,🏗️ifc×5,📐️step,📕️xlsx×3,📜️docx×2,📰xml}/…/🧪️oracle/🔣️.json`

Capability/profile/retag fixes for `oracle-capability-mismatch` — 23 registry files + 23 `🥒️.feature`
files under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{📼️avi,💬️bcf,🖼️bmp,📜️docx,🖊️dxf,🎞️gif×2,🧊️gltf,📷️jpg,🧊️obj,📄️pdf×9,📷️png,🎨️svg,🖼️tiff,📰xml}`

No-oracle decision narrowing — 3 files:
`✏️s/🔌️plugins/{➗️mathematical/…/✳️graph,🎬️sequence/…/✳️any,🖍️draw/…/✳️any}/🧪️oracle/🔣️.json`

New registrations — 2 files:
`🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️oracle/🔣️.json` (new),
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧪️oracle/🔣️.json` (edited)

New adapter — 1 file:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/mutate-os-config-identity/🟦️.ts` (new)

Stray-reference / rename fixes — 6 items:
`✏️s/🔌️plugins/➗️mathematical/…/🧪️tests/mutate-mathematical-1/🥒️.feature`,
`✏️s/🔌️plugins/🗒️note/…/🧪️tests/mutate-note-1/🥒️.feature`,
`🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/web-mercator-tile-oracle/{🥒️.feature,🐍️.py}`,
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🧪️oracle/🔣️.json`,
`🧰️framework/🔨️modules/🎠️kernel/🧪️tests/{reject-malformed-version-input,satisfy-version-requirements}`
(renamed from emoji-prefixed duplicates)

Scratch scripts, kept in the ticket folder: `🩹️a10-reclassify-reimplementations.py`,
`🩹️a10-fix-capability-mismatch.py`.

## Final answer

- `reimplementation-registered-as-third-party`: 20 → 2. All 20 investigated were genuine
  re-implementations wearing a third-party label — **zero** turned out to be genuine third-party
  wrappers needing only "structural" correction. The 2 that remain flagged (`ifc/2x3/✳️any`,
  `ifc/4/✳️any`) are verified-legitimate third-party oracles (`ifcopenshell`) caught by the detector's
  file-level (not entry-level) granularity, documented above rather than force-qualified.
- `missing-external-oracle`: 13 → 13 (unchanged, now honestly the ONLY thing claiming those 13
  mutations — the 3 `no-oracle-covers-mutation` decisions that used to (partly) mask them are gone).
- `no-oracle-covers-mutation`: 3 → 0. `oracle-capability-mismatch`: 23 → 0. `unknown-oracle`: 2 → 0.
  `claimed-implementations-missing`: 1 → 0. `missing-fixture` (mine): 2 → 0. `orphan-fixture`: 1 → 0.
  `fixture-file-missing`: 1 → 0. `case-slug`: 2 → 0.
- Confirmed zero new `oracle-in-production`, `oracle-profile-mismatch`,
  `fixture-generated-by-non-qualifying-oracle` or `fixture-generator-unregistered` breaches in the
  final state (two of these were transiently raised mid-session by my own mistakes and reverted —
  see above).
