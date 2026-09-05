# Fable — VCS Native Openable Provider

Lane `fable-vcs-native-provider`, 2026-09-05. This report records only what was changed and what was
actually executed. Every terminal below was run by this lane; nothing is inferred from a peer report.

## Boundary

VCS becomes the second first-party **native openable provider** beside stdio and GIS: a
package-owned closed receipt, a neutral corpus with an independent AJV/Node/WebCrypto oracle, one
Rust receipt law pair, one linked hub provider entry with exact identity fences, and hub/registry
gate extensions. No immutable bundle, no producer profile, no hub process, no client mount and no
all-plugin activation is claimed anywhere.

## 1. Package identity — already canonical, descriptor still stale

Verified in current source, not changed by this lane:

- `✏️s/🔌️plugins/🌿️vcs/🦀️.rs:29-32` — `Plugin::<VcsApps>::builder("vcs").label("VCS").version("0.1.0").package_id("semio:vcs")`,
  i.e. `package_id` after the label/version typestate stages, as the builder requires.
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🦀️.rs:5,22-37` — `VCS_DOCUMENT_SCHEMA = "vcs.vcs"` and
  `ArtifactKindSpec { id: VCS_DOCUMENT_SCHEMA.into(), … }`. No `vcs.document` remains in VCS Rust source.

Still stale, **not** hand-edited (per the no-hand-edit rule for generated descriptors):

- `✏️s/🔌️plugins/🌿️vcs/🔣️.json` (mtime 2026-09-04 11:17) still contains 2 × `"vcs.document"` and 0 ×
  `"packageId"`; `✏️s/🔌️plugins/🌿️vcs/🛂️.descriptor.semio` is from 2026-08-18.
- The only legitimate regenerator is the registered `@semio-tech/vcs-plugin:describe` target
  (`bun ./📜️script.ts describe` → `describePluginComponent(repoRoot, "semio-s-plugin-vcs", …)`), which
  builds this crate's own `wasm32-wasip2` cdylib. **Blocker (recorded, see §7):** it was not run in this
  lane's budget — the shared `target/debug` lock was held for 5h15m by a peer
  `cargo check --workspace --all-targets --keep-going`, and the box was carrying ~50 concurrent rustc
  processes. The lane's own cold private-target build was already the single cargo process allowed.
  **Nonclaim:** the checked-in VCS descriptor pair is *not* fresh and is not authority for anything below.

## 2. VCS `📇️native-codecs` receipt module (new)

New files, modelled 1:1 on `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/`:

- `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🦀️.rs` — closed one-variant `enum VcsCodecV1 { Vcs }`,
  `NativeVcsCodecIdentityV1` (plugin/package/version/factory/kind/schema/extension/capability +
  `pack_schema_hash: [u8;32]`), inert `NativeVcsCodecReceiptV1`, private `validate()` and
  `into_codec()`. `pack_schema_hash` is `Sha256::digest` over `include_bytes!` of
  `🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/📡️.protocol.semio`.
  `validate()` re-derives the declaration through `crate::artifacts::vcs::definition()` and requires the
  artifact identity `s.vcs.vcs`, the single codec capability `s.vcs.vcs.codec.document`, descriptor bytes
  `vcs.vcs:vcs`, exactly two claims (`codec: vcs.vcs`, `codec-extension: 7:vcs.vcs:vcs`), no second codec
  and a nonzero pack hash — **before** any typed codec exists. `into_codec()` validates first, then
  builds `store::ArtifactCodec::of::<VcsSnapshot, VcsDemoMutation>("vcs.vcs")` and rejects a result whose
  schema/extension differ from the receipt.
- `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🔣️.json` + `🧬️.schema.json` — language-neutral corpus:
  one receipt row (`vcs.vcs.v1` / `s.vcs.vcs` / `vcs.vcs` / `vcs` / `s.vcs.vcs.codec.document`,
  331 protocol bytes, `e16083d1…05d3`) and nine hostile projections
  (`missing`, `duplicate`, `foreign-package`, `wrong-version`, `bare-kind`, `legacy-kind`,
  `legacy-schema`, `wrong-extension`, `zero-hash`). `legacy-kind`/`legacy-schema` are the explicit
  anti-alias cases: `vcs.document` must be denied in both positions.
- `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🧪️tests/🦀️.rs` — two laws (see §5).

Wiring:

- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/🦀️.rs` — `#[path = "../../📇️native-codecs/🦀️.rs"] pub mod native_codecs;`.
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/Cargo.toml` — new `semio-framework-hash` dependency and
  `[[test]] name = "native_codecs"` (ASCII target name, emoji path).

## 3. Neutral receipt module — condition not satisfiable, duplication recorded

The packet allowed moving the shared receipt types into a neutral module **only if both current
owners can be pointed at it without breaking their tests**. Verified against current source, they
share nothing to move:

- stdio owns `NativeCodecFactoryReceipt` (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs`) — a struct with
  `descriptor_codec_id`, `runtime_capability_id` and a **live `factory: fn() -> store::ArtifactCodec`
  pointer**, produced by a macro-generated 26-row bijection table.
- GIS owns `NativeGisCodecIdentityV1` / `NativeGisCodecReceiptV1` — no factory pointer, no
  `descriptor_codec_id`/`runtime_capability_id`, a single `capability` field instead, and identity
  derived from a `match` on a private enum.

There is no common supertype to extract: a neutral struct that keeps stdio's fields would force GIS
and VCS to carry two dead fields plus a function pointer they deliberately do not have, and a neutral
struct that drops them would delete stdio's descriptor-codec/runtime-capability bijection, which the
hub's `NativeOpenableCatalogProviderV1::from_receipts` checks for uniqueness. So the escape hatch was
taken: VCS implements the GIS-shaped contract in its own module, and the duplication is recorded here
as follow-up.

**Follow-up (not done, deliberately):** a neutral `NativeCodecReceiptIdentityV1` +
`trait NativeCodecReceiptV1` for the *GIS/VCS* shape only, homed beside `PluginAssemblyError` in
`semio-framework-plugin` (both crates already depend on it, and it owns the error type and
`ArtifactDefinition`). stdio would stay on its own factory-pointer receipt. Cost is a framework-plugin
edit, which recompiles every plugin crate and the hub, and touches GIS's module while the Sol lane is
still collecting GIS evidence — out of this lane's budget, not a design objection.

## 4. Hub link (`NativeCodecProviderSetV1::linked`)

`🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs`:

- `NATIVE_OPENABLE_PROVIDER_SET_V1_ID` `"stdio+gis/native-codecs/v1"` → `"stdio+gis+vcs/native-codecs/v1"`;
  `NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS` `28` → `29`; new `NATIVE_VCS_PROVIDER_RECEIPTS = 1`.
- Third `NativeCodecProviderEntryV1 { plugin_id: "vcs", package_id: "semio:vcs", preview: preview_vcs_bindings }`
  in `linked()`; its doc-comment no longer says VCS is absent.
- `preview_vcs_bindings` checks, per receipt and before any binding is pushed: `plugin_id == "vcs"`,
  `package_id == "semio:vcs"`, `package_version == version` (the caller's compiled version),
  `factory_id == "vcs.vcs.v1"`, `artifact_kind == "s.vcs.vcs"`, `schema == "vcs.vcs"`,
  `extension == "vcs"`, `capability == "s.vcs.vcs.codec.document"`, nonzero pack hash, factory-id and
  `(kind, schema)` uniqueness; then it consumes `into_codec()` and requires the typed codec's
  schema/extension/pack hash to equal the receipt's. Closure cardinality is checked twice (input length
  and post-loop `factories`/`artifacts` set sizes), which stdio does and GIS does not.
  `context.checkpoint()` runs before the preview, per receipt, and before each push.
- The loader is untouched: `TrustedCatalogLoader::load` still requests a binding **per selected
  package**, so a `stdio+gis` profile never calls the VCS entry and the unconsumed-binding law is
  unchanged. `🔏️trusted-catalog/🦀️.rs` was **not** edited by this lane, including its
  `local-stdio-gis-open-v1` block.

`🌎️hub/📦️packages/🦀️rust/Cargo.toml`: `semio-s-plugin-vcs` added as an optional, no-default path
dependency and `"dep:semio-s-plugin-vcs"` added to the `native-artifact-execution` feature.

## 5. Fixtures, oracles, laws and gates

**New neutral fixtures**

| Fixture | Contents |
| --- | --- |
| `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/{🔣️.json,🧬️.schema.json}` | 1 receipt row + 9 hostile closures, strict `additionalProperties: false`, `const`-pinned identity |
| `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🌿️vcs-v1/{🔣️.json,🧬️.schema.json}` | 8 selection cases (exact, cross-package, cross-plugin, wrong version, unknown provider, cancelled, deadline-at, deadline-before) — exactly 1 accepted — plus 2 `unconsumedProfiles` (`stdio+gis` and `stdio+gis+vcs`) |

**Extended neutral fixtures**

- `🌎️hub/🧪️fixtures/🧭️native-artifact-provider-frontier-v1/{🔣️.json,🧬️.schema.json}`: provider id,
  receipt count `28 → 29`, `pluginDependencies` gains `"vcs"` (schema `const`s updated in lockstep).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧫️fixtures/📦️native-catalog-selection/{🔣️.json,🧬️.schema.json}`:
  a third package `gis` (2 receipts, depends on stdio), profiles `native-stdio-gis-v1` and
  **`native-stdio-gis-vcs-v1`**, a third available provider, and four new cases — `stdio+gis` planning
  without ever calling VCS (28 receipts), `stdio+gis+vcs` dependency-first (29 receipts,
  `["semio:stdio","semio:gis","semio:vcs"]`), a selected-provider failure at VCS, and a missing VCS
  provider. 19 → 23 cases.

**New oracles (Bun + AJV 2020 strict + Node `createHash` + WebCrypto)**

- `proveVcsNativeCodecReceipts` in `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📜️script.ts` — validates the
  corpus, joins it against the real Cargo component id and workspace version, agrees Node and WebCrypto
  SHA-256 on the pinned 331 protocol bytes, admits the literal closure, denies all nine hostile
  closures, and requires the Rust module to `include_bytes!` that exact path with no `vcs.document`.
- `proveVcsNativeProviderSelectionFixture` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — validates the
  selection corpus, joins it to the package-owned closure, requires exactly one accepted selection,
  requires each `unconsumedProfiles` row to preview only its selected packages, and pins the linked
  entry, its exact identity fences and the `= 29;` closure constant in the production Rust source.

**New Rust laws**

- `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🧪️tests/🦀️.rs`
  - `vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution`
  - `vcs_native_receipt_closure_denies_every_hostile_row_including_the_retired_document_kind`
- `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs` `mod tests`
  - `vcs_native_provider_selection_binds_literal_owner_version_and_cancellation_without_publication`
  - `linked_provider_set_previews_only_the_selected_packages_of_a_stdio_gis_or_stdio_gis_vcs_profile`

**Registered gates (extended, not duplicated)**

| Gate | Change |
| --- | --- |
| `@semio-tech/vcs-plugin:native-codec-check` (new target in the plugin's existing `📋️project.json`, `📜️script.ts` router entry `native-codec-check`) | Runs the VCS oracle, then the two VCS Rust receipt laws through the shared `runExactCargoLaws` |
| `os-hub:native-openable-catalog-provider-check` | Now also runs `proveVcsNativeCodecReceipts` + `proveVcsNativeProviderSelectionFixture`, and adds the two hub VCS laws to the existing `semio-hub` lib law group |
| `os-hub:native-catalog-selection-check` / `@semio-tech/plugin-registry:native-catalog-selection-check` | Now plans four profiles including `native-stdio-gis-vcs-v1`; the summary line derives its positive/denied counts from the corpus instead of hard-coding `2`/`17` |
| `os-hub:trusted-stdio-gis-bundle-check --source` (Sol-owned) | One literal updated: its provider-source assertion now expects `NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 29`. **Sol must re-run this gate**; the lane's own frontier check at `📜️script.ts:2603` is fixture-driven and needed no literal change |

`.vscode/🧩️launch.seed.jsonc` gained `⚖️gate🪢️native-codecs🌿️vcs` at order `411.10865` (immediately
after the VCS identity gate), with lane-owned `SEMIO_TEST_ARTIFACT_DIR`/`CARGO_TARGET_DIR`.

## 6. `native-stdio-gis-vcs-v1` in the producer — exact seam, deliberately not crossed

The packet allowed adding the profile **only** to the producer's profile table. Current source has no
such table. In `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `materializeTrustedStdioGisBundle` inlines the
profile id `"local-stdio-gis-open-v1"` in a single expression that is inseparable from the producer:

- the two-element `requests` array of `{pluginId, cargoPackage, componentPackageId, outputName, …}`;
- `if (codecs.stdio.length !== 26 || codecs.gis.length !== 2)`;
- `trustedBootstrapSourceCodecs` returning `Record<"gis" | "stdio", …>` — a TypeScript literal union;
- `const file = (plugin: "gis" | "stdio", receipt) => …` and the literal `packages: [file("gis", gis), file("stdio", stdio)]`;
- `for (const [plugin, receipt] of [["gis", gis], ["stdio", stdio]] as const)` in the generation reader;
- `profile.packages.length !== 2` / `bundle.packages?.length !== 2` cardinality assertions in two places;
- `profileId !== "local-stdio-gis-open-v1"` in the current-pointer reader.

Adding a third package requires generalising all of the above to an arbitrary package list — a
rewrite of the exact function family the Sol lane is actively iterating on (rotation, candidate
publication, receipt-exchange ordering). **This lane stopped at the receipt/provider-set boundary**, as
instructed, and added the `native-stdio-gis-vcs-v1` profile only where a real profile table exists
today: the registry's planning-only selection corpus (§5). No producer, candidate or rotation code was
touched.

## 7. Verification — exact commands and results

| Command | Result |
| --- | --- |
| `bun ./📜️script.ts nx run @semio-tech/vcs-plugin:native-codec-check --skip-nx-cache -- --oracle-only` | **GREEN, exit 0.** `vcs-native-codec-oracle: receipts=1 hostile=9 ajv+node+webcrypto=1` |
| `bun ./📜️script.ts native-openable-catalog-provider-check --oracle-only` (hub cwd) | **VCS portion GREEN**, then RED on a pre-existing stdio blocker — see below. Printed: `vcs-native-codec-oracle: receipts=1 hostile=9 …` and `vcs-native-provider-selection-oracle: cases=8 accepted=1 unconsumed-profiles=2 linked-receipts=29 …` |
| `bun ./📜️script.ts nx run @semio-tech/plugin-registry:native-catalog-selection-check --skip-nx-cache` | **GREEN, exit 0.** `native-catalog-selection-oracle cases=23 positive=4 denied=19 authority=planning-only published=0` |
| `bun ./📜️script.ts native-catalog-selection-check --oracle-only` (hub cwd) | **GREEN, exit 0.** Same corpus terminal through the hub gate |
| `bun ./📜️script.ts nx run @semio-tech/plugin-registry:generate --skip-nx-cache` | **GREEN, exit 0.** 59 plugin crates, 60 playgrounds, 45 framework packages; `.vscode/launch.json` regenerated with `⚖️gate🪢️native-codecs🌿️vcs` at line 5881 |
| `bun ./📜️script.ts nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache` | **GREEN, exit 0.** `plugin registry generated catalog and launch bytes are fresh.` |

### External blocker — stdio projection paths are stale (pre-existing, not this lane)

`os-hub:native-openable-catalog-provider-check` cannot reach a green terminal for any lane right now.
Verbatim:

```
ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio'
      at proveNativeOpenableCatalogProviderFixture (🌎️hub/📦️packages/🦀️rust/📜️script.ts:3315)
```

The stdio artifact directory is now `🗽️obj` (`🧊️` was reassigned to `gltf` by the repo-wide
emoji-uniqueness repair), but the checked-in projection
`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/📜️native-codec-factories.json` (mtime 2026-09-05 06:06,
five hours before this lane started) still names the old path. This belongs to the stdio/emoji lane and
also blocks the Sol bundle producer, whose `trustedBootstrapSourceCodecs` reads the same file. This
lane's two VCS oracle calls were deliberately ordered *before* `proveNativeOpenableCatalogProviderFixture`
so package-owned evidence is reachable while that repair is pending; both printed green above.

### Cargo status

- `cargo check -p semio-s-plugin-vcs --lib` against the shared `target/` sat on the file lock for 13
  minutes and was killed. Lock holder: a peer `cargo check --workspace --all-targets --keep-going`
  with 5h15m elapsed, plus ~50 concurrent `rustc` processes on the box.
- Relaunched once, as the single cargo process, into the lane-private
  `…/scratchpad/fable-vcs-native-provider-target`. Outcome is recorded in §8.

## 8. Cargo terminal

<!-- filled in below -->

## 9. Explicit nonclaims

- No fresh VCS descriptor pair. `🔣️.json`/`🛂️.descriptor.semio` remain stale and were not hand-edited.
- No immutable bundle, no `native-stdio-gis-vcs-v1` bundle generation, no producer, candidate or
  rotation change, no `current.json`.
- No hub process was started; no readiness, no open plan, no catalog generation was observed.
- No client mount, no browser/WGPU/MCP rendering, no all-plugin activation.
- The stdio+gis process gates remain unrun by anyone; nothing here upgrades them.
- `os-hub:native-openable-catalog-provider-check` has no green terminal for its stdio portion; only the
  two VCS oracle lines inside it are this lane's evidence.
