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
  builds this crate's own `wasm32-wasip2` cdylib. **Blocker (recorded, see §8):** it was not run, and could not
  have succeeded — `describe` builds this crate's `wasm32-wasip2` component, and the VCS crate does not compile
  at all right now (18 errors across `E0046`/`E0053`/`E0277`/`E0308`/`E0432`/`E0599`/`E0631` from in-flight
  framework migrations, none of them this lane's; §8). Separately, the shared `target/debug` lock was held for
  5 h 15 m by a peer `cargo check --workspace --all-targets --keep-going` on a box carrying ~50 rustc processes.
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

All bun/oracle gates below were re-run at 23:2x after ~4 h of concurrent peer edits and are current.

| Command | Result |
| --- | --- |
| `bun ./📜️script.ts nx run @semio-tech/vcs-plugin:native-codec-check --skip-nx-cache -- --oracle-only` | **GREEN, exit 0.** `vcs-native-codec-oracle: receipts=1 hostile=9 ajv+node+webcrypto=1` |
| `bun ./📜️script.ts native-openable-catalog-provider-check --oracle-only` (hub cwd) | **GREEN, exit 0** (whole gate). `vcs-native-codec-oracle: receipts=1 hostile=9 …` · `vcs-native-provider-selection-oracle: cases=8 accepted=1 unconsumed-profiles=2 linked-receipts=29 …` · `native-openable-claim-oracle cases=8` · `native-openable-neutral-oracle: AJV=2 owner-receipts=26 protocol-webcrypto=26 targets=1 hostile-denied=13 no-partial=13` |
| `bun ./📜️script.ts nx run @semio-tech/plugin-registry:native-catalog-selection-check --skip-nx-cache` | **GREEN, exit 0.** `native-catalog-selection-oracle cases=23 positive=4 denied=19 authority=planning-only published=0` |
| `bun ./📜️script.ts native-catalog-selection-check --oracle-only` (hub cwd) | **GREEN, exit 0.** Same corpus terminal through the hub gate |
| `bun ./📜️script.ts nx run @semio-tech/plugin-registry:generate --skip-nx-cache` | **GREEN, exit 0.** 59 plugin crates, 60 playgrounds, 45 framework packages; `.vscode/launch.json` regenerated, `⚖️gate🪢️native-codecs🌿️vcs` at line 5881 |
| `bun ./📜️script.ts nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache` | **GREEN, exit 0.** `plugin registry generated catalog and launch bytes are fresh.` |
| `bun ./📜️script.ts trusted-stdio-gis-bundle-check --source` (hub cwd, Sol-owned) | **RED, exit 1** — external, see below |

### Earlier stdio blocker: resolved by its own lane, not by this one

At 19:30 this gate aborted with `ENOENT … 🗿️artifacts/🧊️obj/…/📡️.protocol.semio` because stdio's checked-in
projection still named `🧊️obj` after the emoji-uniqueness repair renamed the directory to `🗽️obj`. The stdio
lane rewrote `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/📜️native-codec-factories.json` at 20:59 (committed
`3a6a9d6bfc`, `git log --date=iso` → 2026-09-05 22:02:04); it now contains 0 × `🧊️obj` / 1 × `🗽️obj`, and the
gate is green end-to-end. The VCS oracle calls remain ordered before `proveNativeOpenableCatalogProviderFixture`
so package-owned evidence stays reachable independently of stdio's projection freshness.

### External blocker — Sol's bootstrap generation is stale after that same stdio repair

```
error: trusted bootstrap full generation mismatch: b96fb865e3d176c5038700db7ad47d2dd5624d3c2baa9b059c728cf548e1d696
      at proveTrustedStdioGisBootstrapFixture (🌎️hub/📦️packages/🦀️rust/📜️script.ts:4560)
```

Attribution is proved, not assumed. `trustedBootstrapProfileEncoding` hashes only the rows returned by
`trustedBootstrapSourceCodecs`, which reads exactly two files — stdio's `📜️native-codec-factories.json` and
GIS's `📇️native-codecs/🔣️.json` — and no file this lane created or edited. stdio's projection was last
committed 2026-09-05 22:02:04 (`3a6a9d6bfc`); Sol's fixture
`🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/🔣️.json` was last committed 2026-09-05 03:53:30
(`fe7c8a8f8b`) and still pins `generationId = 7cf0515d…`. The stdio repair therefore invalidated Sol's pinned
generation ~18 h after it was written. **Sol must recompute that `generationId` to `b96fb865…`**; this lane did
not touch their fixture. The gate aborts at line 4560, well before line 5386, so it never reaches this lane's
one-token change there (`NATIVE_OPENABLE_PROVIDER_SET_V1_RECEIPTS: usize = 28 → 29`), which consequently has
**no** verified terminal yet and must be re-run by Sol once their generation is repinned.

## 8. Cargo terminal

One cargo process at a time, foreground-launched, in the lane-private target dir
`…/scratchpad/fable-vcs-native-provider-target` (the shared `target/debug` lock was held for 5 h 15 m by a peer
`cargo check --workspace --all-targets --keep-going`).

```
CARGO_TARGET_DIR=…/fable-vcs-native-provider-target CARGO_BUILD_JOBS=4 RUSTC_WRAPPER=""   cargo check -p semio-s-plugin-vcs --lib --tests --message-format=short
```

**RED after 3 h 52 m** (cold build, box at load average 68 with ~29 competing rustc processes, this lane's rustc
pinned at nice 5 and ~2–3 % of one core; `taskpolicy -B`/`renice` could not lift it without privileges):

```
error: could not compile `semio-s-plugin-vcs` (lib) due to 18 previous errors; 42 warnings emitted
error: could not compile `semio-s-plugin-vcs` (lib test) due to 152 previous errors; 92 warnings emitted
```

**The VCS crate does not compile today, and none of it is this lane's.** Attribution, measured from the log
rather than asserted:

- This lane's `📇️native-codecs/🦀️.rs` produced **exactly one diagnostic in the whole build** — `warning: unused
  import: Hasher` — and **zero errors**. That warning is now fixed (`use semio_framework_hash::Sha256;`), which is
  precisely the change the compiler asked for.
- Every error is in files this lane never touched: `✏️editor` (124), `🧬️schema` (77), `🚪️io` (23), `👁️viewer` (8).
- The error classes are in-flight framework migrations, not identity or naming drift: `E0046` missing
  `DESCRIPTORS`/`descriptor` trait items (presence/config), `E0053` `render` expected `Result<ComponentTree, …>`
  found `UiNode`, `E0053` `command_from_action` expected `DslValue` found `JsonValue`, `E0277`
  `Result<IoOutcome<…>, IoError>` is not a future (io went async), `E0277` `Label: From<Label>` prelude collision,
  `E0432` `protocol::testkit::assert_mutation_{diff_absorb,inverse}_law` removed, plus 18 × `E0308` / 12 × `E0631`
  / 4 × `E0599` in the editor. The single `E0432` is a framework testkit removal, not a rename of anything here.

A confirmation re-check (`--lib` only, deps warm) was started after the import fix; a peer had meanwhile rebuilt
`semio-framework-plugin`, so it must re-check `semio-s-plugin-stdio` (the ~3 h unit) again and had not finished
within this lane's window. Retry command, unchanged:

```
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/c34c334c-fe3e-420d-a00b-7b4aa1238be5/scratchpad/fable-vcs-native-provider-target CARGO_BUILD_JOBS=4 RUSTC_WRAPPER="" cargo check -p semio-s-plugin-vcs --lib --message-format=short
```

### Consequences, stated plainly

- **The four new Rust laws were never executed.** The two VCS receipt laws need `semio-s-plugin-vcs` (lib test)
  to build; the two hub link laws need the hub, which now depends on that crate. Narrowest filters, ready to run
  the moment the VCS crate's migration errors are fixed:
  - `bun nx run @semio-tech/vcs-plugin:native-codec-check --skip-nx-cache`
  - `bun nx run os-hub:native-openable-catalog-provider-check --skip-nx-cache`
- **Blast radius of the hub dependency — the coordinator should decide.** `native-artifact-execution` is in the
  hub's `default` feature set, so adding `dep:semio-s-plugin-vcs` means a default-feature hub build now also
  requires the VCS crate to compile. stdio and GIS have already been migrated (stdio checked clean in this
  lane's build); VCS has not. Until the migration lanes repair VCS, this line makes a default hub build
  inherit VCS's 18 errors. Reverting only the hub `Cargo.toml` dependency line and the `linked()` entry would
  restore the previous hub build while leaving the receipt module, fixtures, oracles and laws intact — this lane
  did **not** do that, because linking VCS is the packet's deliverable and the correct end state, and no
  feature-gate or compatibility shim may be introduced to hide it. Flagging it rather than deciding unilaterally.

## 9. Explicit nonclaims

- **No Rust law in this packet has passed.** Nothing here claims the VCS receipt module, the hub provider entry
  or their four laws compile or execute. The only executed evidence is the bun/AJV/Node/WebCrypto oracles in §7.
- The VCS crate does not currently compile; the descriptor `describe` target was therefore never runnable either.
- No fresh VCS descriptor pair. `🔣️.json` (2 × `vcs.document`, 0 × `packageId`) and `🛂️.descriptor.semio` remain
  stale and were **not** hand-edited.
- No immutable bundle, no `native-stdio-gis-vcs-v1` bundle generation, no producer, candidate or rotation change,
  no `current.json`.
- No hub process was started; no readiness, open plan or catalog generation was observed. No codec was registered.
- No client mount, no browser/WGPU/MCP rendering, no all-plugin activation.
- The stdio+gis `--native` and `--process` gates remain unrun by anyone; nothing here upgrades them.
- `os-hub:trusted-stdio-gis-bundle-check --source` is RED for a reason outside this lane, so this lane's one-token
  change inside it is **unverified** and needs Sol's re-run.
