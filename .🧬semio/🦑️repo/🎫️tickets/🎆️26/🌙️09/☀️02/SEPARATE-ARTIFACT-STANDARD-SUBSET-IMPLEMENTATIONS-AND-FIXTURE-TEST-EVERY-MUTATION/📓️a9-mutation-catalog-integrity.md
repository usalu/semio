# A9 — Mutation catalog integrity

Shard A9 of `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
Scope: `unknown-mutation-catalog`, `mutation-catalog-unclaimed`, `mutation-kind-undeclared`,
`mutation-kind-uncovered`, `mutation-inverse-uncovered`, `unregistered-mutation-vocabulary`.

## Before / after (my six ids)

Judge: `bun ./📜️script.ts test contract`, counted from
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`.

| id | before | after (mine) | after (live gate, incl. concurrent churn) |
| --- | --- | --- | --- |
| `unknown-mutation-catalog` | 7 | **0** | 1 (new, unrelated — see below) |
| `mutation-catalog-unclaimed` | 7 | **0** | 0 |
| `mutation-kind-undeclared` | 12 | **0** | 0 |
| `mutation-kind-uncovered` | 1 | **0** | 0 |
| `mutation-inverse-uncovered` | 1 | **0** | 0 |
| `unregistered-mutation-vocabulary` | 37 | 33 (documented, see below) | 46 (33 mine + 13 unrelated) |

Did **not** raise `mutation-catalog-capability-mismatch` (0), `missing-capability` (0) or
`no-scenarios` (0) — confirmed by direct count after the fix, both classes are still zero.

`unknown-mutation-catalog` and `unregistered-mutation-vocabulary` show extra live-gate entries
that are **not mine**: a fourth gif89a occurrence (`✏️s/…/🎞️gif/🧪️tests/mutate-gif-89a/🥒️.feature`,
plus its now-broken `✳️base/🧪️oracle/🔣️.json` catalog, plus new `✳️application/✳️graphic-control/
✳️comment` mutation trees) and thirteen new `las`/`gltf` entries appeared between the ticket's
baseline dump and my gate run. Ticket-folder evidence (`🔨️a6-move-gltf-mutations.py`,
`🔨️a6-split-gltf-aggregators.py`, `🔨️a6-tag-gltf-manifest-subsets.py`) confirms shard A6 is
mid-flight restructuring `gltf`, and the same subset-splitting pattern is visible on `gif89a` and
`las` (new `✳️application`/`✳️graphic-control`/`✳️comment` and `✳️points`/`✳️vlr` subset
directories that did not exist in the ticket's baseline). Per the brief ("if a file changed under
you… do not chase unrelated breakage"), I left these three artifacts untouched.

## Part 1 — the seven mistargeted `-any` catalogs (`unknown-mutation-catalog` + `mutation-catalog-unclaimed`)

Each artifact-level feature claimed an invented `@mutations-<artifact>-any` catalog that was never
declared anywhere, while the real subset-owned catalog sat one directory below, unclaimed. In every
case the feature's `@capability-` tag already matched the real catalog's `capability` field and its
Examples-table kind list already matched the real catalog's `kinds` list exactly — so the fix was a
one-line retarget of the `@mutations-` tag, nothing else:

| artifact | feature | was claiming | now claims | kinds |
| --- | --- | --- | --- | --- |
| `📄️pdf` | `🧪️tests/mutate-pdf-1-4` | `pdf-1-4-any` (nonexistent) | `pdf-1-4-base` | 5 |
| `🎨️svg` | `🧪️tests/mutate-svg-1-1` | `svg-1-1-any` (nonexistent) | `svg-1-1-base` | 9 |
| `📜️docx` | `🧪️tests/mutate-docx-ecma-376` | `docx-ecma-376-any` (nonexistent) | `docx-ecma-376-base` | 13 |
| `📰xml` | `🧪️tests/mutate-xml-1-0` | `xml-1-0-any` (nonexistent) | `xml-1-0-base` | 6 |
| `📷️jpg` | `🧪️tests/mutate-jpg-jfif-1-01` | `jpg-jfif-1-01-any` (nonexistent) | `jpg-jfif-1-01-document` | 10 |
| `🔣️json` | `🧪️tests/mutate-json-rfc8259` | `json-rfc8259-any` (nonexistent) | `json-rfc8259-base` | 5 |
| `🖼️tiff` | `🧪️tests/mutate-tiff-6-0` | `tiff-6-0-any` (nonexistent) | `tiff-6-0-document` | 6 |

No `standardDirectoryName`/`subsetDirectoryName`/capability edits were needed anywhere — the
manifests were already correct; only the artifact-level feature file's `@mutations-` tag was wrong.

(A concurrent `gif89a` feature carries the exact same `-any` pattern — `@mutations-gif-89a-any` —
but its subset itself is being split into `✳️base/✳️application/✳️graphic-control/✳️comment` right
now by another session, its catalog is currently profile-invalid, and its oracle/manifest are also
mid-flight-broken. Left alone; not one of my seven, and touching it would race that shard.)

## Part 2 — the six catalogs missing `no-mutation` (`mutation-kind-undeclared`)

Twelve `mutation-kind-undeclared` breaches reduced to two real causes:

1. **`no-mutation` is a standard, widely-used control kind** (57 other catalogs across the repo
   already declare it first in `kinds`) meaning "apply these params and expect no projection
   change" — every flagged feature already had both a `mutate-no-mutation` and an
   `inverse-no-mutation` Examples row; the *catalog* just never declared the kind. Added
   `"no-mutation"` as the first `kinds` entry (matching repo convention) to:
   - `epw-energyplus-any` (`🌦️epw/…/✳️any/🧪️oracle/🔣️.json`) — 12→13 kinds
   - `zip-2-0-any` (`🎒️zip/…/✳️any/🧪️oracle/🔣️.json`) — 6→7 kinds
   - `zip-2-0-iso21320` (`🎒️zip/…/✳️iso21320/🧪️oracle/🔣️.json`) — 7→8 kinds
   - `step-ap214-cc1` … `step-ap214-cc6` (six files) — 4or5→5or6 kinds each
   - `semio-v1-flow` (`🧿️semio/…/✳️flow/🧪️oracle/🔣️.json`) — 12→13 kinds
   - `semio-v1-presentation` (`🧿️semio/…/✳️presentation/🧪️oracle/🔣️.json`) — 14→15 kinds

   This is the one place I noticed a **pre-existing, unrelated** gap while in these files: none of
   these catalogs have any `mutationManifests` (v2) entries at all, so `capability-without-manifest`
   and `test-only-mutation` already fired for every one of their kinds before my change and continue
   to after it (now covering one more name, `no-mutation`, same as every other kind). That id is not
   mine and predates this shard; noted for whichever shard owns v2 manifests.

2. **`semio-v1-drawing`'s six differently-named kinds** — see Part 3.

## Part 3 — `semio-v1-drawing`: the missing six kinds (`mutation-kind-uncovered` + `mutation-inverse-uncovered` + 6 of the 12 `mutation-kind-undeclared`)

`✏️s/…/🧿️semio/🧪️tests/mutate-semio-drawing/🥒️.feature` was flagged from both directions at once
because it was one naming bug: the catalog's real names are `rotate-node`, `scale-node`,
`group-nodes`, `ungroup-node`, `flatten-node`, `unflatten-node` (visible in the catalog's own
`vectors[].mutationId` and matching the `spec-vector` Examples table's `dir` column,
e.g. `🔄rotate-node`), but the `@id-mutate`/`@id-inverse` Examples tables used the short verb alone
(`rotate`, `scale`, `group`, `ungroup`, `flatten`, `unflatten`). So `mutate-rotate` was exercised
but undeclared, while `mutate-rotate-node` was declared but never exercised — one bug, seen from
both sides of the same check, exactly as the brief predicted.

Fix: renamed the `id` column in the `@id-mutate` and `@id-inverse` Examples tables (only — the
`@id-spec-vector` table already used the full names) from the six short verbs to the catalog's
six real kind names, and renamed the matching case-owned fixture directories under `🧫️fixtures/`
so `local://🧫️<id>/🦠️mutation/🔣️.json` keeps resolving:
`🧫️rotate→🧫️rotate-node`, `🧫️scale→🧫️scale-node`, `🧫️group→🧫️group-nodes`,
`🧫️ungroup→🧫️ungroup-node`, `🧫️flatten→🧫️flatten-node`, `🧫️unflatten→🧫️unflatten-node`
(each still holds its original `🦠️mutation/🔣️.json` payload — content untouched, only the
directory name changed to match its new scenario id). Also updated the two prose references to
`inverse-unflatten` → `inverse-unflatten-node` (a pre-existing, deliberately-red scenario —
`Unflatten`'s computed inverse cannot restore an arbitrary replaced node; the red status is
unchanged by the rename, only the name it's referred to by).

No brand-new scenarios or brand-new fixture vectors were needed: the six kinds already had real,
fixture-backed mutate/inverse Examples rows and real committed mutation payloads under
`🧫️fixtures/`; they were simply addressed by the wrong id.

## Part 4 — `unregistered-mutation-vocabulary` (37 → 33 resolved-by-documentation, 4 fixed)

Investigated all 37 directories individually (`🖥️ls`/`find` on each). None were framework
scaffolding placeholders in the sense the brief's checklist implies except one; the rest split into
two real categories, neither of which the two prescribed dispositions ("register" or
"remove-if-empty") cleanly cover without side effects — documented below rather than forced.

### Fixed (4)

| path | finding | action |
| --- | --- | --- |
| `🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations` (repo root, **not** under `✏️s/…`) | An orphaned duplicate of part of the real `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/…` tree, sitting directly at the repo root; its one file (`💬set-archive-comment/🦀️.rs`) contains the literal text `placeholder`. Tracked in git since 2026-08-29, untouched since. | Deleted the whole stray `🎒️zip/` tree at repo root. |
| `🏔️gisterrain/…/✳️any/✏️editor/🎚️config/🧪️tests/🧬️mutations` | Genuinely empty (0 entries). | Removed the empty directory. |
| `🗺️gismap/…/✳️any/✏️editor/👥️presence/🧪️tests/🧬️mutations` | Real content, but it's a plain Rust unit-test module (`mod direct_mutation_tests`), wired in by one `#[path = "🧪️tests/🧬️mutations/🦀️.rs"]` attribute in the sibling `🦀️.rs` — not a declared mutation vocabulary at all, just an unlucky directory name collision with the taxonomy's reserved `🧬️mutations` name. | Renamed to `🧪️tests/🧬️direct-leaves` (gisterrain's own sibling config module already uses this exact name for the identical pattern — `🧪️tests/🧬️direct-leaves/🦀️.rs` / `mod direct_leaf_contracts`), updated the one `#[path]` string. No content moved. |
| `🗺️gismap/…/✳️any/✏️editor/🎚️config/🧪️tests/🧬️mutations` | Same pattern as above. | Same fix: renamed to `🧪️tests/🧬️direct-leaves`, updated the `#[path]` string. |

### Documented, left unregistered (33) — with reasoning

**Three gis editor-state catalogs are structurally impossible to register as written**
(`🏔️gisterrain/…/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations` [2 kinds: `set-camera`,
`set-locale`], `🗺️gismap/…/✳️any/✏️editor/👥️presence/🧬️schema/🧬️mutations` [1 kind:
`set-camera`], `🗺️gismap/…/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations` [6 kinds:
`set-layer-visibility`, `set-vector-style`, `set-camera`, `set-lod-mode`, `set-render-mode`,
`set-layer-stroke-scale`]). All three are real, live, tested production Rust (DSL/pack codecs,
protocol text/binary op codecs, their own `#[cfg(test)]` suites) — genuine editor/presence
view-state mutation vocabularies, not placeholders.

The blocker is `mutationCatalogProblems` in the framework's own `🟦️.ts`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`): the
`unregistered-mutation-vocabulary` walker computes the required contribution owner as exactly
`dirname(dirname(<mutations-dir>))` — here `…/✏️editor/🎚️config` or `…/✏️editor/👥️presence` — but
because that owner string contains `/🏅️standards/` anywhere, `mutationCatalogProblems` forces it
into "profiled" mode, which additionally requires
`owner.endsWith(\`/🏅️standards/${standardDirectoryName}/🪆️subsets/${subsetDirectoryName}\`)`. A
nested owner with trailing `/✏️editor/🎚️config` past the subset root can never satisfy an
`endsWith` check anchored at that same subset root — so **no compliant catalog can be registered at
the walker-computed owner**, full stop. The only way to physically satisfy the check would be
relocating the mutation-kind directories up into the subset root's own `🧬️schema/🧬️mutations/`
(merging with `gisterrain-1-any`/`gismap-1-any`, which today mean *document-content* mutations, a
different vocabulary) and rewriting every `#[path]`/`include_str!` reference the move would break —
a real, live production-code refactor across two plugin subsets I judged out of proportion and too
risky to make blind in a shared tree without the plugin owner's review. Flagged as a follow-up
(framework rule gap or a dedicated relocation ticket); not fixed here.

**Twenty-nine `🧰️framework/🛍️products/💻️os/🔨️modules/{🔌️plugin,🏪️store,📡️spr,🔁️workflow,
♾️infinite,🌊️flow}` entries and one `🧰️framework/🔨️modules/📡️replication` entry (30 total)** are
all real, populated Rust unit-test fixture trees for the framework's own generic CQRS
mutation/diff/inverse mechanism (e.g. `store`'s `⏱️timestamped`/`🚦️severity`/`🛂️validated`/
`🪤️lossy` demo-aggregate variants, `spr`'s `add-counter`/`add-counter-twice`/…, `plugin`'s
`set-transaction-count`/`set-surface-count`/…) — none are empty, none are dead scaffolding.
None of them sit under any `/🏅️standards/` path, so a plain non-profiled catalog (`id`,
`capability`, `kinds`, `vectors: []`) *would* pass `mutationCatalogProblems` cleanly — I confirmed
none of their physical mutation-kind directories carry a nested `🧪️tests/<scenario>` subdirectory,
so an empty `vectors: []` would not additionally trigger `mutation-vector-unregistered`.

But registering any of them would immediately trade this breach for a **new**
`mutation-catalog-unclaimed` — a class I *am* accountable for and had just brought to zero in Part
1: `mutation-catalog-unclaimed`'s "claimed" set is every `.feature` file's `@mutations-<id>` tag,
and there is not a single `🥒️.feature` file anywhere under any of these 30 module trees (confirmed
by search — the whole `{plugin,store,spr,workflow,infinite,flow,replication}` tree carries exactly
three `.feature` files total, all for an unrelated `os-config` catalog). Registering a catalog here
without also writing a matching Gherkin feature + test case + adapters — real, substantial,
per-directory scaffolding, times 30 — is out of this shard's proportion; doing it partially (a
catalog with no claim) is strictly worse than the status quo on my own metric. Flagged as a
follow-up: either design real cross-language coverage for these, or (my recommendation) recognize
them as native-only framework-internal test fixtures the artifact-mutation-catalog gate was never
meant to reach, and exempt directories with no `🏅️standards/` ancestor and no sibling `.feature`
surface from `unregistered-mutation-vocabulary`.

## Verification

`bun ./📜️script.ts test contract` was run in the foreground before touching anything (matching the
ticket's `🗑️generated/breach-*.json` baseline dumps, confirmed line-for-line for all six of my ids)
and again after all edits above, reading the fresh
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`. Also explicitly re-checked
`mutation-catalog-capability-mismatch`, `missing-capability` and `no-scenarios` counts (0/0/0,
unchanged) to confirm none of the retargets or `no-mutation` additions introduced a mismatch.

## Files touched

Retargeted `@mutations-` tags (7):
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/🥒️.feature`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧪️tests/mutate-svg-1-1/🥒️.feature`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🧪️tests/mutate-docx-ecma-376/🥒️.feature`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🧪️tests/mutate-xml-1-0/🥒️.feature`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🧪️tests/mutate-jpg-jfif-1-01/🥒️.feature`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🧪️tests/mutate-json-rfc8259/🥒️.feature`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🧪️tests/mutate-tiff-6-0/🥒️.feature`

Added `"no-mutation"` to `kinds` (11):
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc1/🧪️oracle/🔣️.json` … `✳️cc6/🧪️oracle/🔣️.json` (6 files)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧪️oracle/🔣️.json`

`semio-v1-drawing` coverage fix:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-drawing/🥒️.feature` (edited)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-drawing/🧫️fixtures/{🧫️rotate-node,🧫️scale-node,🧫️group-nodes,🧫️ungroup-node,🧫️flatten-node,🧫️unflatten-node}` (renamed from `{🧫️rotate,🧫️scale,🧫️group,🧫️ungroup,🧫️flatten,🧫️unflatten}`)

Vocabulary registration cleanup:
- Deleted `🎒️zip/` (stray tree at repo root)
- Deleted `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️tests/🧬️mutations` (empty)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/{🧪️tests/🧬️mutations→🧪️tests/🧬️direct-leaves, 🦀️.rs}` (renamed + `#[path]` updated)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/{🧪️tests/🧬️mutations→🧪️tests/🧬️direct-leaves, 🦀️.rs}` (renamed + `#[path]` updated)

No `🪆️subsets/🔣️.json` files were edited and no Gherkin test-case directories were moved.
