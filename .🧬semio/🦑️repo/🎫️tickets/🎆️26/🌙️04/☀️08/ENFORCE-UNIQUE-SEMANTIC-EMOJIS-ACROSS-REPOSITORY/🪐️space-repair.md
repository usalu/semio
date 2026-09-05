# Space Artifact Emoji Repair

## Scope

Owned tree: `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space`.

All seven initial duplicate findings were repaired explicitly:

- Editor and viewer `🎚️options` → `☑️options`; configuration remains `🎚️config`.
- Subset `🧪️oracle` → `🔮️oracle`; tests remain `🧪️tests`.
- The four create, rename, touch, and delete mutation payload schemas use `🧬️.schema.json`, distinct from their `🔣️.json` descriptors.

Rust, Python, aggregate-schema, descriptor, and oracle-manifest references were updated only where they address this artifact. Stdio oracle references and the unrelated editor-config schema reference were preserved.

## Verification

The final scoped statute audit covers 126 files, 128 directories, and 254 governed entries. Missing, generic, presentation, spacing, duplicate, multiple, reserved-emoji, and independent-oracle findings are all zero.

The exact central oracle override is:

```json
"✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any": "🔮️oracle"
```

`bun nx run @semio-tech/space-plugin:test-quick` completed unsuccessfully in the shared Stdio dependency: 156 existing `MutationLeaf` source-authority/trait errors. The output contained no missing Space path introduced by this repair.

## 2026-09-05 Full Space Pass

The later full-plugin census covered the engine and Home launcher, not only the Space artifact. Every rename below was selected from the command's actual behavior and checked against its own sibling set before the move.

- Engine engagement: `⌨️compiled-dag-engagement-input`, `📨️compiled-dag-engagement-submit`, retained `💬️workflow-engagement-input`, and `✅️workflow-engagement-submit`.
- Engine persistence and exchange: `📜️export-studio-dsl`, `📦️export-studio-pack`, `📥️import-space-pack`, `🧳️import-space-pack-payload`, `🚪️open-space`, `🎬️set-active-example`, `📤️export-media`, retained `🖼️import-media`, and `🧾️import-media-payload`.
- Engine instance and graph actions: `❎️close-focused-instance`, retained `🔍️open-instance`, `🔌️connect-media-ports`, `✂️disconnect-media-edge`, `📋️copy-app-instance`, `🗑️delete-selection`, `👯️duplicate-app-instance`, `🚚️move-media-node`, `📌️paste-app-instance`, `🩺️patch-app-instances`, `🔧️patch-media-nodes`, `🚮️remove-app-instance`, `🏷️rename-app-instance`, `🗂️reorganize-workflow`, and `🚀️spawn-app`.
- Engine parameters and navigation: `➕️add-parameter`, `🔗️bind-parameter-field`, `🩹️patch-parameter`, `➖️remove-parameter`, `🔓️unbind-parameter-field`, retained `🧭️go-home`, `🗺️navigate-virtual-file-system-node`, `⚙️set-active-panel-tab`, and `📇️set-app-registrations`.
- Home launcher: `📎️bind-space-file`, `🏗️create-studio`, `📥️import-space`, `🌌️open-space`, `🏠️go-home`, and `🧭️navigate-virtual-file-system-node`; the already-distinct command identities were retained.
- The three engine window configuration owners and the two Home window configuration owners are `⚙️config`; their option siblings remain `🎚️options`. The Home subset independent implementation is `🔮️oracle`.
- The retained-command-limits, projection-persistence, and direct change-catalog-generation JSON Schema files are `🧬️.schema.json`; adjacent JSON descriptors remain `🔣️.json`. Only descriptor/path values resolving to those four physical files changed. Logical payload-schema values without a physical sidecar were intentionally preserved.

All exact Rust mounts, TypeScript imports, schema references, comments, oracle paths, and command taxonomy identities were updated. The Home test contribution override is `🔮️oracle`; no stale removed Space command identity remains in the command registry unless the exact identity is still physically owned by another plugin.

The final full Space audit covers 439 files, 363 directories, and 796 governed entries. Missing, generic, presentation, spacing, duplicate, multiple, reserved-emoji, and oracle findings are all zero. Every physical Engine and Home command has an exact central membership, and `validateTaxonomy(loadCatalogTaxonomy())` returns `[]`.

`bun nx run @semio-tech/space-js:test` exits 0. `bun nx run @semio-tech/space-plugin:home-directory-projection-persistence-check` exits 0 with 11 checks, including the committed schema through Ajv and hostile projection cases.
# Final command correction (2026-09-05)

- Handpicked `📬️apply-directory-event-page` for the Home command previously carrying generic `📄️`: the mailbox glyph expresses accepting a delivered directory-event page into the local projection and is distinct from sibling `📇️fold-directory-events`.
- Updated both live Rust mount/script references and registered the exact command member in the central taxonomy.
- Space audit: 440 files, 364 directories, 798 governed paths; all eight breach counts are zero.
- `bun nx run @semio-tech/space-plugin:home-directory-projection-persistence-check`: passed, 11 checks clean.
