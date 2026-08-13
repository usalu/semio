# 🧭️ Wave A0 — State Vocabulary SSOT

Ticket `26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION`, workstream A, wave A0.
Two tasks: collapse `StateClass` onto exactly four lanes, and give the taxonomy the transient lane
plus declared mode children.

---

## 1. `StateClass` is now strictly four

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧾️wire/🦀️component.rs`

```rust
pub enum StateClass { Artifact, Config, Presence, Transient }
```

Migration applied everywhere, with no aliases, no deprecations and no compatibility layer.

| retired | becomes | rationale |
| --- | --- | --- |
| `Persistent` | `Artifact` | persisted shared state IS the artifact lane |
| `LocalUi` | `Config` | persisted local-only |
| `SharedUi` | `Presence` | ephemeral shared |
| `Preview` | `Artifact` | draftness is a LANE property (which store the record lives in), never a field annotation |
| `Effect` | DELETED | effects are `Emit.effects`, not state |
| `Inferred` | leaves `StateClass` | derivation is its own axis: `#[derived]` / `x-semio-derived: true` / GraphQL `@derived` |

### The single `effect` field

`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/` —
`ImperativeArtifact::run_output_json`. Reclassified as `transient`: it is the output of running the
imperative program, held for the local UI only, never persisted and never shared. It is correctly
absent from `XDiff`, which the diff-coverage rule still enforces (see §1.3).

### The derived axis

`#[derived]` is a new field attribute understood by `#[derive(ArtifactSchema)]`. A field is on
exactly one axis — `#[state(…)]` OR `#[derived]` — and carrying both, or neither, is a compile error
(`parse_field_axis` in `🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️component.rs`, mirrored byte-for-byte
into its `📦️packages/🦀️rust/📦️glue.rs` twin, which is the file Cargo actually compiles).

The trait gained a sibling table:

```rust
pub trait ArtifactSchemaFields {
    fn artifact_schema_id() -> &'static str;
    fn field_states() -> &'static [(&'static str, StateClass)];
    fn derived_fields() -> &'static [&'static str] { &[] }
}
pub const JSON_SCHEMA_DERIVED_KEY: &str = "x-semio-derived";
```

Derived fields are absent from `field_states()` entirely — a derived field is not state.

### The five-format sweep

Every shipped facet leaf was rewritten in lock-step. Counts are the actual replacement totals from
`scratch-a0-sweep.py` (kept in this ticket folder; it is a one-off, not a permanent script):

| format | annotation | artifact | config | presence | transient | derived |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| Rust | `#[state(…)]` / `#[derived]` | 3 066 | 907 | 409 | 1 | 123 |
| JSON Schema | `"x-semio-state"` / `"x-semio-derived"` | 3 372 | 874 | 398 | 1 | 119 |
| GraphQL | `@state(class: …)` / `@derived` | 3 137 | 866 | 396 | 1 | 119 |
| Protobuf | `// @state …` / `// @derived` | 2 599 | 840 | 387 | 1 | 119 |
| TypeScript | `@state …` / `@derived` | 2 576 | 869 | 399 | 1 | 119 |

2 396 files touched. The artifact column folds `persistent` + `preview` together, which is why it
exceeds the old `persistent` count.

One stray `"x-semio-state": "identity"` in
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/…/🧬️schema/📸️snapshot/🔣️component.json` was never a legal state
class at all (it did not parse); it is now `artifact`, matching the snapshot facet it sits in.

A follow-up prose sweep retired the old vocabulary from 246 docstrings ("persistent fields only" →
"artifact-lane fields only", "across persistent, shared-ui and local-ui classes" → "across the
artifact, presence and config lanes", …).

### 1.3 Lock-step updates

- `parse_state_class_kebab` / `state_class_kebab` — `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`.
  The retired spellings now return `None`, pinned by a new test
  `retired_state_vocabulary_no_longer_parses`.
- `GRAPHQL_STATE_PREAMBLE` — Rust and its TypeScript twin `🟦️component.ts`:
  `enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }` plus a new
  `directive @derived on FIELD_DEFINITION`. The TS twin also gained `STATE_CLASSES` /
  `JSON_SCHEMA_DERIVED_KEY`.
- `validate_registered_app_descriptor` — config fields must be `StateClass::Config`, presence fields
  `StateClass::Presence`.
- Root `📜️script.ts`:
  - `policyAppSchemaStatePurityBreaches` expects `config` / `presence`.
  - `policyArtifactSchemaStateParityBreaches` filters on `artifact` (was `persistent`).
  - `policyArtifactSchemaDiffCoverageBreaches` excludes `transient` from `XDiff` (was `effect`) —
    transient is ephemeral local-only, so it is never diffed, which is exactly the property that had
    justified the `effect` exclusion.
  - `policyInferenceStateLeakBreaches` → `policyDerivedMarkerLeakBreaches`, watching
    `POLICY_DERIVED_MARKER = "#[derived]"` instead of `#[state(inferred)]`. Same rule ("never in a
    📸️snapshot facet"), same `inference-migration/state-leak` kind, retargeted onto the new axis.

---

## 2. Taxonomy: the transient lane and mode children

SSOT `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. The transient directory is
`🫧️transient`.

- `appComponentDirs`, `appChildDirs`, `taxonomyLeafParentDirs` gain `🫧️transient`.
- **NEW** `modeChildDirs = ["🪟️windows", "🎚️config", "👥️presence", "🫧️transient"]`. Modes previously
  declared no children at all, so a mode-scoped state facet had nowhere legal to live.
- `windowChildDirs` gains `🎚️config` and `🫧️transient`;
  `windowRequiredChildDirs = ["🎬️actions", "🪛️utilities", "🎚️options", "🎚️config", "👥️presence", "🫧️transient"]`.
- **NEW** `transientChildDirs = ["🧬️schema"]`, mirroring `configChildDirs` / `presenceChildDirs`.
- `appSchemaSpecFilenames` gains `"🫧️transient/🧬️schema": "🔣️component.json"`.
- New `_stateLaneComment` records the four-lane doctrine in the SSOT itself.

### Scaffolding (`scratch-a0-scaffold.py`, also one-off and ticket-local)

Reality on disk was **119 windows** and **59 modes** (the brief estimated 120 / 66). The 120th
`🪟️windows` child is `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/` — a packaging dir,
not a window, and outside every `🎛️apps/<app>` root the validators walk.

| scope | dirs created | markers created |
| --- | ---: | ---: |
| windows (`🎚️config`, `🫧️transient`) | 238 | 238 |
| modes (`🪟️windows`, `🎚️config`, `👥️presence`, `🫧️transient`) | 179 | 179 |

Each genuinely-empty capability dir carries exactly one `📌️empty.md`
(`taxonomy.windowEmptyFacetFilename`), per the repo's existing convention. Two architect modes
(`📊️report`, `🔍️review`) had no `🪟️windows` at all and now declare an explicitly empty one — "an empty
capability is valid, an absent capability is not". Repo-wide marker count went 460 → 877.

### Validators

- **`policyWindowCompletenessBreaches`** (root `📜️script.ts`) already read
  `windowRequiredChildDirs`, so it picked up both new lanes with no code change.
- **NEW `policyModeCompletenessBreaches`** — the twin one level up, same three findings
  (`taxonomy/mode-completeness`, `taxonomy/mode-empty-child` for missing marker and for
  marker-beside-members). Wired into both the `verify` gate and the aggregate policy sweep.
- **`validateTaxonomyTree`** (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts`)
  gained a per-mode completeness loop over `modeChildDirs`. `TAXONOMY_WINDOW_CHILDREN` picks up the
  two new window lanes straight from the taxonomy.
- **Discovery contract** (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`)
  gained `transientChildDirs` / `modeChildDirs` on the `Taxonomy` type,
  `appFacetChildDirs("🫧️transient")`, and a new `StateLaneContract` region asserting the three
  non-artifact lanes appear in `appChildDirs`, `modeChildDirs`, `windowChildDirs` and
  `windowRequiredChildDirs`. `validateTaxonomy()` reports **0 problems**.

### The `⚙️engine` / `⚙️engine/📚️examples` contradiction

Resolved as instructed, in favour of the taxonomy SSOT (`⚙️engine` is in `appComponentDirs`):

- TS `validateTaxonomyTree` now **requires** `🎛️apps/<app>/⚙️engine/`, and **keeps** its separate,
  correct finding that `⚙️engine/📚️examples/` must move to the app root. Both coexist; they are not
  the same rule. On disk today: 0 apps carry `⚙️engine/📚️examples`, so that finding stays at zero.
- Rust `testkit::assert_taxonomy_components` required `app/⚙️engine/📚️examples`, which exists nowhere
  in the repo — a stale assertion, safe to correct because `assert_constitutional_crates` currently
  has **no callers** (the macro that used to invoke it survives only in an older ticket's
  pre-patch snapshot). It now requires `⚙️engine/` alone. This was the only edit made to the hot
  `🔌️plugin/🦀️component.rs`, re-read immediately beforehand.

**Consequence to hand on:** 2 apps genuinely lack `⚙️engine/` — `🧩️puzzle/🧊️3d` and `🧩️puzzle/🖐️5d`.
They will now surface as registry findings. This is a true breach of the ticket's own target
architecture ("Every app has a headless engine, modes, config, presence, transient") and belongs to
workstream A's "app headless ⚙️engine" item, not to A0.

### Emoji-prefix collision, and why the rule was the thing that changed

Adding `🎚️config` beside the pre-existing `🎚️options` inside every window produced **123 new
high-priority `taxonomy/emoji-prefix-uniqueness` breaches**. Neither name could move: `🎚️config` is
the state-lane token keyed on by `configChildDirs` / `appSchemaSpecFilenames` and by 38 apps, and
`🎚️options` is a window capability wired into 119 windows' `#[path]` graphs.

The rule was the stale party. `policyEmojiSiblingIdentityIsStructural` already exempts families
whose shared leading emoji is a structural kind marker rather than a local identity; a window's and
a mode's children are **entirely taxonomy vocabulary**, authored nowhere locally, so there is no
local visual identity to keep unique — and renaming either member at a site would break the
vocabulary it belongs to. The exemption now covers `windowChildDirs` members under a `🪟️windows`
parent and `modeChildDirs` members under a `🎭️modes` parent. Count returned to its pre-existing 3.

---

## 3. Verification — what was actually run

### Cargo

| command | result |
| --- | --- |
| `RUSTC_WRAPPER="" cargo check -p semio-framework-schema --all-targets` | **0 errors** |
| `RUSTC_WRAPPER="" cargo check -p semio-framework-os-kernel --all-targets` | **0 errors** |
| `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-imperative -p semio-s-plugin-flow --all-targets` | **0 errors** |
| `RUSTC_WRAPPER="" cargo check --workspace --all-targets` | 188 errors, **none mine** — see below |
| `RUSTC_WRAPPER="" cargo test -p semio-framework-os-kernel --lib` | **833 passed / 3 failed** |

Logs: `scratch-a0-check-schema.txt`, `scratch-a0-check-kernel3.txt`, `scratch-a0-check-final.txt`,
`scratch-a0-check-plugins.txt`, `scratch-a0-check-workspace.txt`, `scratch-a0-kernel-test-final.txt`.

**Workspace failures** are confined to `semio-compose-rs` (92) and `semio-framework-ui` (89), on
`UiTreeActionPlacement`, `label_impl::Label: From<&str>`, `kernel_3d_scene`, `dsl::DslRecord`. Zero
of the 188 errors mention `StateClass`, `#[state(…)]`, `#[derived]` or any retired token; **zero**
files under `🧰️framework/🔨️modules/🖱️ui` or `compose/` were touched by this wave, and neither tree
contains a single state annotation. A transient 190th error
(`ArtifactDsl for DemoSnapshot missing EXTENSION`) appeared mid-run from a concurrent session and had
already resolved by the next kernel check.

**Kernel test suite vs the recorded 834 / 1 baseline:**

- `os_dsl::fixture_sweep::m5_cross_artifact_rejection::all_non_stdio_grammars_reject_each_others_shipped_fixtures`
  — the known pre-existing failure.
- `os_spr::command::tests::operation_descriptor_fingerprint_is_golden_pinned` — **mine, fixed.** The
  golden blake3 pin hashes the serde variant name, which went `Persistent` → `Artifact`. Re-pinned to
  `2fe60b82…7818890` with a comment recording why.
- `os_spr::channel::tests::app_command_fixture_corpus_matches_golden_hex_and_round_trips` and
  `…app_frame_fixture_corpus…` — **not mine, left alone.** A concurrent session bumped
  `CHANNEL_VERSION: u32 = 5 → 6` in `📡️spr/🧵️channel/🦀️component.rs` without re-pinning its golden
  corpus; the drift is literally the version byte (`0006…` actual vs `0005…` golden). That file has
  zero state annotations, `git diff` against the index shows no edit of mine, and the bump is visible
  in `git diff HEAD` as another session's staged work. Re-pinning their goldens would collide with
  their in-flight change.

Net against baseline: **833 passed (+ the 1 I fixed, − the 2 they broke), 1 pre-existing failure,
2 concurrent-session failures, 0 introduced by this wave.**

### Taxonomy / TypeScript

- `validateTaxonomy(loadTaxonomy())` → **0 problems**.
- `bun test 🧪️index.test.ts -t "completeness policy"` → **2 pass / 0 fail** (the window test, now
  taxonomy-driven instead of hardcoding four facets, plus a new mode-completeness test).
- The registry `📜️script.ts` imports and parses cleanly. Its `check` command exits early on a stale
  `.vscode/launch.json` (another session's churn) before reaching `validateTaxonomyTree`, so the two
  new checks were verified by direct on-disk simulation instead: **0** modes missing a declared
  child, **0** apps with `⚙️engine/📚️examples`, **2** apps missing `⚙️engine/` (the puzzle pair above).
- Pre-existing failures elsewhere in `🧪️index.test.ts` (29 total) were confirmed stale against
  `git show HEAD:🔣️taxonomy.json` — e.g. assertions on `📡️spr` in `artifactChildDirs` and on an older
  `exampleAssetKindPrefixes` shape, both already wrong before this wave.

### Policy sweep

`bun ./📜️script.ts policy`, per-kind counts from `.🦑️repo/⚡️cache/breaches/compose.json`:

```
TOTAL before = 29 456   after = 29 461   delta = +5
high  before = 23 858   after = 23 859   delta = +1
```

Per-kind delta (`scratch-a0-kinddiff-final.txt`):

| delta | before | after | kind \| priority |
| ---: | ---: | ---: | --- |
| +2 | 704 | 706 | `taxonomy/plugin-registration-violation` \| medium |
| +2 | 24 | 26 | `taxonomy/plugin-registration-setup-callback` \| medium |
| +2 | 308 | 310 | `os-state-authority/item-scope-global` \| high |
| −1 | 1 | 0 | `taxonomy/plugin-closed-shape` \| high |

Every one of those five sits in `🌍️gis`, `📐️cad`, `🎪️demonstrator` or `🗄️stdio` files this wave never
opened (two of them inside the *other* ticket's `🎯️target/debug/build/libsqlite3-sys/out/bindgen.rs`
build artifact) — concurrent-session churn. The record-level added/removed listing is in
`scratch-a0-added-removed.txt`; the only entries touching my files are 3 `authority-struct-map` and
3 `budget/no-budget-null` records that appear on **both** sides — pure line-number shifts, net zero.

Every kind this wave could plausibly have moved stayed exactly flat:

| kind | before | after |
| --- | ---: | ---: |
| `taxonomy/window-completeness` | 0 | 0 |
| `taxonomy/window-empty-facet` | 0 | 0 |
| `taxonomy/mode-completeness` | 0 | 0 |
| `taxonomy/mode-empty-child` | 0 | 0 |
| `app-schema/state-purity` | 0 | 0 |
| `artifact-schema/state-parity` | 0 | 0 |
| `artifact-schema/diff-coverage` | 0 | 0 |
| `inference-migration/state-leak` | 0 | 0 |
| `taxonomy/emoji-prefix-uniqueness` | 3 | 3 |
| `taxonomy/dirs` | 94 | 94 |

Adding two required window lanes across 119 windows, three required mode children across 59 modes,
and a brand-new mode-completeness rule cost **zero** new breaches.

---

## 4. Handed on

1. `🧩️puzzle/🧊️3d` and `🧩️puzzle/🖐️5d` need a headless `⚙️engine/` — workstream A's own item, now
   surfaced by the registry validator rather than hidden behind a stale assertion.
2. `os_spr::channel::tests::app_{command,frame}_fixture_corpus_matches_golden_hex_and_round_trips`
   need their golden hex re-pinned by whoever bumped `CHANNEL_VERSION` to 6.
3. `testkit::assert_constitutional_crates` has no callers. Its taxonomy half is correct again, but
   nothing runs it — the macro that used to invoke it is gone. Worth re-wiring or deleting.
4. `🫧️transient/🧬️schema` is declared in `appSchemaSpecFilenames` but no app ships one yet.
   Deliberate: the app/mode/window transient facets get real schema leaves in the later A-waves that
   build `TransientStore` and `OsTransient`. The Rust and TS app-schema-owner validators were left
   requiring only `🎚️config` + `👥️presence`, so nothing red-flags in the meantime.

## 5. Ticket-local scratch

`scratch-a0-sweep.py`, `scratch-a0-scaffold.py` (both one-off, neither a permanent script),
`scratch-a0-sweep-result.txt`, `scratch-a0-scaffold-result.txt`, `scratch-a0-check-*.txt`,
`scratch-a0-kernel-test*.txt`, `scratch-a0-taxonomy-tests*.txt`, `scratch-a0-registry-check.txt`,
`scratch-a0-policy-{before,after,after2,final}.txt`,
`scratch-a0-breaches-{before,after,after2,final}.json`,
`scratch-a0-kindcounts-{before,after}.txt`, `scratch-a0-kinddiff{,2,-final}.txt`,
`scratch-a0-added-removed.txt`.
