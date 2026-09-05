# Remodel Plugin Emoji Repair

## Scope

Owned tree: `✏️s/🔌️plugins/📸️remodel`, including the Remodeling artifact, package mounts, live local references, its exact oracle registration, and its command member registrations.

The initial strict audit covered 676 files, 615 directories, and 1,285 governed entries. It found 82 breaches: 76 sibling-emoji collisions and 6 missing identities. Every physical move was selected and executed explicitly; no automatic emoji chooser, rename planner, or modifying Git operation was used.

The complete handpicked Remodel command roster is:

- Pipeline parameters: `🌁️set-dense-params`, `🌠️set-feature-params`, `🌐️set-geo-params`, `🥣️set-ingest-params`, `🪢️set-match-params`, `🕸️set-mesh-params`, `🏎️set-motion-params`, and `🧮️set-sfm-params`.
- Ground control and calibration: `🧿️add-gcp`, `🔭️calibrate-cameras`, `🛠️edit-calibration`, `🔎️place-gcp-observation`, and `🚮️remove-gcp`.
- Media and reports: `🧾️export-qc-report`, `🎞️import-frames`, `🎥️import-video`, `🖼️import-frame-payload`, `💽️import-video-bytes-payload`, `✅️import-video-done`, and `📼️import-video-frame-payload`.
- View state: `🪛️set-active-utility`, `📷️set-camera`, `⏱️set-frame-cursor`, `👓️set-layer-visibility`, `🗣️set-locale`, and `📊️set-report-table`.
- Streams and reconstruction: `🌱️add-stream`, `🪓️remove-stream`, `🔄️set-stream-sync`, `⏩️advance-reconstruction`, `🔁️retry-stage`, `🏗️run-reconstruction`, `▶️run-stage`, and the already-distinct `🛑️cancel-reconstruction`.
- Clearing: `☁️clear-dense`, `🗾️clear-geo-products`, `🧱️clear-mesh-result`, the retained semantic `🧹️clear-result`, `⭐️clear-sparse`, `🚂️clear-tracks`, and `♻️reset-placeholder-mesh`.

Four window option facets now use the repository's canonical `☑️options`; OBJ import/export authorities use `🗿️obj`; the subset-local oracle uses `🔮️oracle`; the editor fixture and all 35 mutation payload sidecars use `🧬️.schema.json`. The previously unprefixed reconstruction fixture assets are now `🏁️commit-reconstruction/{⬅️before.json,➡️after.json}` and the test-local inputs are `⬅️commit-reconstruction-before.json`, `🦠️commit-reconstruction-mutation.json`, and `➡️commit-reconstruction-after.json`.

All 34 mutation-vector `directoryName` values were reconciled individually with their already-handpicked physical scenario directory names. All 70 physical payload-schema references in the mutation and oracle manifests were updated from `🔣️.schema.json` to `🧬️.schema.json` while leaving each owner manifest `🔣️.json` unchanged.

## Verification

- Final strict scoped audit: 676 files, 615 directories, 1,285 governed entries; missing, generic, presentation, spacing, duplicate, multiple, reserved-emoji, and oracle findings are all zero.
- `validateTaxonomy(loadCatalogTaxonomy())` returns `[]` after the exact Remodel oracle override and 36 previously absent command registrations. Three command names already existed exactly and were not duplicated.
- All 259 non-dot Rust `#[path]` mounts in the Remodel package entry resolve.
- All 34 mutation-vector owner directories, payload schemas, and scenario directories resolve.
- Both files in the handcrafted reconstruction fixture manifest resolve.
- `bun nx run @semio-tech/remodel-plugin:test-quick` reached the unrelated shared Stdio compile failure `identifiers cannot contain emoji: 📸️jpg`; the contended `cargo nextest list` then exceeded its 1,200,000 ms budget. No missing Remodel path was reported.

## Exact Central Override

```json
"✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any": "🔮️oracle"
```
