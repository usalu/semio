# WP-R3: semio-base JSON Bridge, Third-Party Oracle, Contract Debt

Slice: R3 (session 10). Captures: `.tmp-ticket/wp-r3/generated/`. Private cargo: `.tmp-ticket/wp-r3/target`.
Inherits: R2 §2 (semio-base 43/43 with `parity=0/0` by recorded no-oracle decision; 26 contract rows).

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. Schema-first JSON bridge `SemioSnapshot`/`SemioMutation` | DONE | subject uses it for every scenario; unit vectors below |
| 1. Third-party oracle (Rust): `json-rust-semio-envelope-carrier-reader` (json-rust 0.12) | DONE, runs in the parity oracle phase | `parity-2.txt`, `parity-2-roles.txt` |
| 1. Second implementation (TS): platform `JSON.parse` + Ajv 8.20.0 probe `semio-base-carrier-reproduce` | DONE: 4/4 carrier-expressible vectors reproduced, 22/22 routed, 16/22 schema-conforming (6 arm-schema drifts, §4) | `ajv-probe-1.json`, `ajv-probe-nx.txt` (via nx) |
| 2. 26 semio-base contract rows | **0 remain** (26 → 0) | `contract-0.txt` → `contract-2.txt` |
| 3. `✉️mutate-semio-base` parity exhaustive | **43/43**, executed 86 = 43 oracle + 43 subject, 86 passed | `parity-2.txt`, `parity-2-roles.txt` |
| 3. Oracle discriminates | tampered one committed arm result → **42/43**, `mutate-apply-text` diff; restored byte-identical | `parity-tamper.txt` |
| base unit tests `cargo test -p semio-s-artifact-stdio-semio --lib -- subsets::base::` | **70/70** (incl. 3 new vector cases + `kinds_match_the_enum_and_the_catalog`) | `base-unit-1.txt` |
| runtime inventory `test inventory --artifact s.stdio.semio --standard v1 --subset base` | 19 runtime, 19 declared, **0 differences** | `inventory-1.txt` |
| repo-wide contract totals | 3335 → **2900** high rows (net −435; +10 rows are older debt newly visible, §3) | `contract-delta.txt` |

## 1. Bridge, oracle, second implementation

- **Bridge (schema-first).** `encode_semio_snapshot_json`/`decode_semio_snapshot_json` (`✉️base/🧬️schema/📸️snapshot/🦀️.rs`) and
  `encode_semio_mutation_json`/`decode_semio_mutation_json` (`…/🧬️mutations/🦀️.rs`): first-party `pack` JSON over the schema types'
  own `ToValue`/`FromValue`, the same pattern `📑️document` exports. The adapter's hand-written value-subset decoder is gone.
- **Published carrier fixed to match the wire** (it could not validate a single committed vector before):
  - `"type": "value"` → `"object"` (codemod corruption) in 5 base + 5 value schema files.
  - base `📸️snapshot/🔣️.json`: 18 arms (was 13) as `allOf` arm `$ref` + `subset` const, i.e. the real internally tagged wire.
  - base `🧬️mutations/🔣️.json` + `🟦️.ts` mirrors (the TS mirrors imported all 18 types from brep and had 13 arms): rewritten.
  - Vocabulary unified on the leaf kinds: `#[value(rename = "<noun>")]` ×18 removed (a compatibility shim that kept the old
    noun wire tag). Wire tag is now `applyBrep` as the schema already published; `KINDS`, catalog `kinds`, manifest ids,
    fixture `mutation` fields and scenario ids are `apply-<arm>`. This is what makes the runtime inventory agree (DESCRIPTORS
    report leaf kinds).
- **Rust oracle.** `✉️base/🔮️oracles/🦀️.rs`, linked into `semio-s-plugin-stdio-test-oracle` (`artifacts::semio::…::base`) under
  the existing `oracles` feature. json-rust reads every committed carrier (no json-rust type in the public API) and routes it:
  `setSnapshot` → payload; `apply<Arm>` on a matching arm → the arm's committed result (produced by that arm's registered
  independent implementation); mismatched arm → refused `mutation.target-missing`; inverse → before.
  - **Deviation from the brief, deliberate:** json-rust instead of `serde_json`. `serde_json` is a production dependency of the
    subject crate itself; `🧾️json`'s oracle already records why that is no independent evidence. json-rust is already linked
    by the stdio oracle crate, so no dependency was added.
- **Projection** both roles report: `{schema, subset, diagnostics, matchesReference, envelopeDigest}`, the digest over the whole
  resulting envelope with ordered keys, so every scenario compares complete documents.
- **Vectors.** 18 delegated arms now run on committed before/mutation/after triples (`🧫️fixtures/*-applied/`, new
  `🦠️mutation.json` each) instead of empty arms + in-code verbs. 3 new catalog vectors: `📸️set-snapshot/🪞️reasserts-the-value-envelope-unchanged`
  (the identity; the vocabulary has no `no-mutation` verb, so the old `noMutation` docstring scenario became this),
  `📸️set-snapshot/🔁️retypes-a-value-envelope-to-an-empty-image`, `🖼️apply-image/🚫️refuses-a-value-envelope` (rejected,
  `🔺️diff/🚫️.absent`), each with its canonical Rust test wired in `🧬️mutations/🦀️.rs` FixtureCases.
- **Four arm fixtures were not in the carrier's wire spelling** (decode failed): document (`styleId`/`imageId` vs the Rust
  `style_id`/`image_id`), video (`kind: "V"`, hex `data`), brep (vertices without `tol`), object (`childId` ≠ `artifactId`,
  violating the child-identity law). Spelling corrected by hand, then verified lossless against the bridge's own re-encoding
  (only default-valued members differed; `compare-debug-dumps.py`). Their manifests are now `handcrafted` with attribution to
  the generating arm oracle (the ticket generator scripts could not reproduce the corrected files).
- **TS second implementation.** `✉️base/🔬️probes/📜️script.ts carrier-reproduce` (+ `📋️project.json`, launch entry
  `⚖️gate🧿️semio✉️base🔬️carrier-reproduce`), registered as qualified probe `semio-base-carrier-reproduce` with pipeline
  `semio-v1-base-carrier-reproduce-v1` on the 4 envelope-owned vectors.

## 2. The 26 contract rows, by root cause

| Rows | Root cause | Fix |
|------|-----------|-----|
| 1 `subsetDirectoryName must start with ✳️` (+ `Unknown mutation catalog`, 2× `vocabulary … no catalog registers it`) | catalog validator hard-coded `✳️`/`🔖️` prefixes; subsets use owner-chosen emojis (104 + 18 rows repo-wide) | `profileDirectoryIsCanonical` (canonical emoji + slug, path-emoji statute) in `🧪️test/🟦️.ts`; test added to `🧪️test-platform/🟦️.ts` |
| 19 `requires a third-party-library … none is registered` | no oracle | json-rust oracle, requirement names it |
| 1 `19 kinds have no wire record` | `💾️binary/📡️.protocol.semio` had only the frame header | 19 `record <kind> tag=<n>` matching `mutation_tag`; `.ksy` doc tag fixed (setSnapshot is 0) |
| 1 `No runtime inventory` | no production bridge for base | `✉️base/🏭️bridge/` (`📜️script.ts` + `🦀️.rs` reading `SemioMutation::DESCRIPTORS`); leaf descriptors declare `error` (refusal on mismatch) |
| 1 `set-snapshot … no fixture-backed vector` | vector registered in the catalog only | fixture manifests for all 4 catalog vectors |
| surfaced afterwards: `case-above-subset` | `subsetCoordinatesOfOwner` had the same `✳️`/`🔖️` bug | emoji-agnostic, same canonical check |
| surfaced afterwards: `reimplementation-registered-as-third-party` | heuristic needs a qualified probe pipeline as judge | pipeline over the Ajv probe |

## 3. Newly visible debt (not base; not fixed, owners' files)

- 7× `mutationManifests[0] subsetDirectoryName does not match the owner path`: equation (`➗️equation`, `📐️geometry`, `🕸️graph`),
  fem 2d/3d `🌐️any`, xml `✅️valid`/`🧱️base` still declare `✳️<id>`. Fix: set `subsetDirectoryName`/catalog profile to the real dir.
- 3× gltf catalogs `gltf-2-0-{scene,buffer,mesh}` claimed by no feature (were hidden behind the rejected catalogs).

## 4. Findings handed on

- Arm schema ≠ arm wire (Ajv probe warnings): audio/presentation mutation payload schemas lack `insertChannel`/`insertSlide`;
  brep payload schema internally tagged, wire externally tagged; image `rgba8`, video `data` typed as strings (wire: byte arrays);
  document nullable options typed non-null and `styleId`/`imageId` vs wire `style_id`/`image_id`. Fixed only the two
  dangling refs that stopped the envelope schema compiling: brep `#/$defs/semioPoint3` ×18, and cad's `#/definitions/{layer,block,entityRecord,point2,entity}`, all of which resolve to its `$defs`.
- object/kit snapshot schemas: top-level `additionalProperties: false` removed (the envelope embeds the arm beside `subset`;
  the other 16 arms were already open).
- `🔺️mutate-semio-mesh`: its TS adapter registers oracles only, but the artifact ships TS, so `subject --implementation typescript`
  errors 52/52 (measured). Same trap would hit any TS oracle adapter under `🧿️semio`.
- 11 base leaf dirs lack FE0F (`🎬apply-video`, `🧱apply-brep`, …) against `mutationDirectoryPattern`; untouched.
- `🧪️test-platform/🟦️.ts` (live-repo suite): 93 pass / 19 fail in both runs (`test-platform-1.txt`, `test-platform-2.txt`, same 19 names). No failure names `✉️base` after the pipeline fix. The ones I read are other owners' debt: 13 oracles without `comparisonProfiles`, the deleted `🗽️obj` case, block-5d fixture debt. There is no pre-change baseline, so I do not claim all 19 predate R3.
- `verify dependencies literal-external` fails as before (target 0, current 239, 15 oracle conflicts); R3 added no third-party dependency.

## Processes (pids)

All foreground or finished: parity/contract/inventory/cargo runs 14197, 22849, 28398, 28690 (exited). test-platform runs, incl. 47665 (exited).

## Files changed (R3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts`: `profileDirectoryIsCanonical`, catalog validator, `subsetCoordinatesOfOwner`, `caseAboveSubsetBreaches`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts`: profile-directory test
- `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🦀️.rs`: `artifacts::semio::…::base` module
- `…/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/`:
  - `🔮️oracles/🦀️.rs` (new), `🔮️oracles/🔣️.json` (oracle, probe, pipeline, catalog, manifests, fixtures; no-oracle decision removed)
  - `🔬️probes/📜️script.ts`, `🔬️probes/📋️project.json` (new); `🏭️bridge/{📜️script.ts,🦀️.rs,Cargo.toml,Cargo.lock}` (new)
  - `🧪️tests/✉️mutate-semio-base/{🥒️.feature,🦀️.rs}` (rewritten)
  - `🧬️schema/📸️snapshot/{🦀️.rs,🔣️.json,🟦️.ts,📝️text/🔣️.json}`, `🧬️schema/🔣️.json`, `🧬️schema/🔺️diff/{🔣️.json,📝️text/🔣️.json}`
  - `🧬️schema/🧬️mutations/{🦀️.rs,🔣️.json,🟦️.ts,🧪️tests/🔬️unit/🦀️.rs,💾️binary/📡️.protocol.semio,💾️binary/🥋️.ksy}`, 18× `*apply-*/🔣️.json`
  - new tests: `🧬️mutations/📸️set-snapshot/🧪️tests/{🪞️reasserts-…,🔁️retypes-…}/🦀️.rs`, `🧬️mutations/🖼️apply-image/🧪️tests/🚫️refuses-a-value-envelope/🦀️.rs`
  - fixtures: 18× `🧫️fixtures/*-applied/🦠️mutation.json` (new); document/video/brep/object before/after; `🧫️fixtures/🧬️mutations/📸️set-snapshot/{🪞️…,🔁️…}/`, `🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses-a-value-envelope/` (new)
- other subsets: `🔢️value/🧬️schema/{🔣️.json,📸️snapshot/🔣️.json,📸️snapshot/📝️text/🔣️.json,🔺️diff/🔣️.json,🔺️diff/📝️text/🔣️.json}` (`type`),
  `🧊️brep/🧬️schema/📸️snapshot/🔣️.json`, `📐️cad/🧬️schema/📸️snapshot/🔣️.json` (dangling refs), `📦️object`/`🧰️kit` `🧬️schema/📸️snapshot/🔣️.json`
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`: one gate entry
- Ticket inputs kept: `.tmp-ticket/wp-r3/*.py`, `*.ts` (generators/experiments used above)
