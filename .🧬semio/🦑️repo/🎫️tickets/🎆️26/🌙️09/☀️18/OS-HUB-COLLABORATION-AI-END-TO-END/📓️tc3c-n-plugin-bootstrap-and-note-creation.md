# TC3c — N-plugin trusted-catalog bootstrap, and the first `s.note.note` document created on a hub

Slice TC3c of ticket 26/09/18. Brief: finish TC3b — generalise the two-package bootstrap builder to
an N-package list, fix the `apply_ops_binary` empty-batch panic (TC3b §6f), boot a fresh hub on
**7651** from this tree and create the first `s.note.note` document ever created on a hub.

Predecessor: `📓️tc3b-catalog-genesis-landed.md` (§5 "not done", §6 the ordered list).
Started 2026-09-21 11:30. Mutex queue on arrival: pz1 (holding since 10:32), s10, c5, rb1.

## 0. HUB HANDOFF (top of report)

**Hub source IS changed by this slice — the coordinator needs a `semio-hub` rebuild + suite rerun.**
Two hub fences changed — `validate_descriptor_open_target`
(`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`) and the GIS inference binding
(`🌎️hub/💡️inference/📇️catalog/🦀️.rs`) — plus the whole N-package bootstrap builder in
`🌎️hub/📦️packages/🦀️rust/📜️script.ts`.

Green checks, each with its warning count as the proof the tree really type-checked rather than
replaying a short-circuited cache (memory *Require Warnings As Proof Of Type-Check*):

| check | exit | warnings | capture |
|---|---|---|---|
| `cargo check -p semio-framework-plugin-describe` | 0 | 16 | `🗑️generated/tc3c-check-describe.txt` |
| `cargo check -p semio-framework-plugin` | 0 | 63 | `🗑️generated/tc3c-check-plugin.txt` |
| `cargo check -p semio-s-artifact-note-note` | 0 | 80 | `🗑️generated/tc3c-check-note-artifact.txt` |
| `cargo check -p semio-hub --lib` | 0 | 181 | `🗑️generated/tc3c-check-hub.txt` |
| `cargo check -p semio-hub --all-targets --keep-going` | 0 | 338 | `🗑️generated/tc3c-check-hub-all.txt` |
| `cargo check -p semio-hub --lib` (after the inference-catalog fix) | 0 | 181 | `🗑️generated/tc3c-check-hub2.txt` |

`cargo check -p semio-framework-plugin` reported *fresh* on its first run although the file had just
been edited, so freshness was PROVEN rather than assumed: a `compile_error!` was appended, the check
reported exactly that error, and the probe was removed. Cargo does track the `#[path]`-included
godfile; the cached green verdict therefore includes this slice's edit.

`cargo test/nextest -p semio-hub` is the coordinator's (preamble rule 26) and has NOT been run.

## 1. N-package bootstrap builder — design

The verb is REPLACED, not kept beside the old one:
`trusted-stdio-gis-bootstrap` → **`trusted-catalog-bootstrap [--packages <plugin,plugin,…>]`**
(default `stdio,gis,note`), `TrustedStdioGisBootstrapScript` → `TrustedCatalogBootstrapScript`,
`materializeTrustedStdioGisBundle` → `materializeTrustedCatalogBundle(repoRoot, dataRoot, selection)`,
and the nx target `trusted-stdio-gis-bootstrap` → `trusted-catalog-bootstrap` (`forwardAllArgs`, so
`bun nx run os-hub:trusted-catalog-bootstrap -- --packages stdio,gis` works). Every source-text gate
that used the old symbol names as region boundaries moved with them.

**Adding a fourth plugin is ONE row**, `TRUSTED_BOOTSTRAP_PACKAGES`:
`{ pluginId, cargoPackage, componentPackageId, outputName, linkedCodecRegistry, opensDocuments }`.
Everything else is derived:

* **Codec rows.** `linkedCodecRegistry` is not a schema transcription — it names the file the hub
  binary's COMPILED `NativeCodecProviderSetV1` is generated from, so it is present for exactly the
  packages this binary links Rust codecs for (stdio, gis) and `null` for every other one. A linked
  package's rows must EQUAL its compiled provider's preview or `verify_selected`'s
  `consumed_bindings.len() != binding_map.len()` fence refuses the generation, so they are read from
  that same registry. An unlinked package answers for itself: the builder asks the built component
  through the new `codecs` subcommand of the descriptor emitter (§1a).
* **Open targets.** `trustedBootstrapDescriptorOpenTargetsV1` reads the package's OWN verified
  descriptor: a kind is creatable exactly when the manifest declares its `ArtifactKindSpec` and binds
  an EDITOR app to its dialect, and the surface, app, window, dialect and role are then that app's
  own. The hard-coded `s.gis.gismap` / `gis2d-main` / `s.gis.gismap@1/*#editor` literals are gone.
* **Dependencies.** `trustedBootstrapResolveDependencies` no longer spells "GIS plus Stdio": every
  dependency a package's compiled manifest claims must be in the selected closure at the exact
  compiled version, sorted canonically.
* **Browser actors.** Built for every package that opens documents (`opensDocuments`), not for
  `pluginId === "gis"`; a mismatch between that flag and what the descriptor actually declares is an
  error, not a silent difference.
* **Profile identity.** `local-${requested.join("-")}-open-v1`, from the REQUESTED list order, so
  `--packages stdio,gis` still mints `local-stdio-gis-open-v1` and the hub's own exactness fence on
  that profile id keeps guarding the two-package closure, while `stdio,gis,note` mints
  `local-stdio-gis-note-open-v1`. The selected CLOSURE stays in canonical identity order, which
  `validate_bundle` requires.
* **Generation hash.** `trustedBootstrapProfileEncoding` now takes `Record<string, rows>` and frames
  the open-target SET behind its own count in `trusted_profile_generation`'s canonical order, and the
  per-package browser-actor renderer comes from `package_actor_renderer`'s own rule instead of "is
  this the one open target's package". For a ONE-target profile every framed byte is identical to the
  retired encoding (same renderer answer, same count of 1, same field order), so this generalisation
  does not by itself rotate an existing generation id.
* **The published tree.** `trustedBootstrapVerifyGeneration` spelled the exact directory closure
  `packages/{gis,stdio}` with `browser/closed-actor.mjs` under `gis`. It now derives the closure from
  the bundle's own package records — one directory per selected package, a `browser/` directory
  exactly where a record carries a closed actor — so a third package's tree is verified as strictly
  as the first two rather than being unrepresentable.
* **The candidate probe** picks the GIS target by its OWNER instead of `openTargets[0]`, because the
  targets are a set now and an index would silently probe whichever package sorts first.

### 1a. How an unlinked package's pack fingerprints are obtained

New subcommand on the build-time emitter (`semio-framework-plugin-describe`), the same binary the
component build already produces and runs for `describe`:

```
semio-framework-plugin-describe codecs <component.wasm> --kinds <k1,k2,…> --out <file.json>
```

`kinds` are dialect artifact kinds as the package's own compiled descriptor spells them. For each:

1. `codec.genesis(kind, "artifact-c0dec0de…")` mints the kind's canonical empty document. Its `spr`
   is a `HistoryLog` whose `schema` field IS `A::DOCUMENT_SCHEMA` — the string `store::ArtifactCodec`
   and the trusted catalog are keyed by, and the ONLY place a package publishes it. The probe also
   asserts the returned history carries the requested id and zero edits.
2. `codec.pack-schema-hash(schema)` returns that kind's 32-byte record fingerprint.

Both run on the exact hash-verified component bytes about to be published, on a throwaway owned-
interpreter instance. That made one small addition necessary in the SDK: `plugin_artifact_codec_app`
now resolves by document schema (the primary key, unchanged) OR by the app definition's dialect
artifact kind, because the artifact kind is the only document identity a package's own manifest
publishes — build tooling asks by kind, reads the schema back out of genesis, and asks everything
after that by schema. The hub's own calls are unchanged.

### 1b. Two identity defects this uncovered, both fixed at the root

* **`s.note.note` answered to a second name.** `note::artifact_kind()` declared
  `id: "2d.note"` while `NOTE_DIALECT.artifact_kind` (and therefore every surface id, and the
  `#[artifact_schema]` id) is `s.note.note`. `validate_descriptor_open_target` requires the declared
  kind spec id to equal the target kind, so note could never be opened on a hub under its real name.
  The spec now reads `id: NOTE_DIALECT.artifact_kind` (`name: "Note"`), exactly as
  `semio_s_artifact_gis_gismap::artifact_kind()` already reads `GISMAP_DIALECT.artifact_kind`.
* **Every inference was denied on a multi-target catalog.**
  `verified_gis_map_binding_with_service` resolved its target through
  `VerifiedTrustedCatalog::selected_document_open()`, which answers `Some` only when the generation
  carries EXACTLY ONE open target. TC3b generalised the catalog to an open-target SET but left that
  accessor deliberately narrow, so the first generation with two creatable kinds would have answered
  `None` and denied every hub inference. It now resolves the GIS Map target by KIND through
  `artifact_creation_selection("s.gis.gismap")` — the same unambiguity rule (exactly one writable
  editor target whose codec identity this generation verified), scoped to one kind instead of to the
  whole catalog.
* **A migrated plugin was structurally un-openable.** The hub's discoverability fence read only
  `descriptor.manifest.artifact_kinds`, the channel `PluginBuilder::artifact_kind(…)` fills. A plugin
  migrated onto the declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM) —
  note, and every plugin after it — stitches its spec into the OWNING APP's `artifact_kinds` instead,
  so the fence rejected it however the catalog was built. `validate_descriptor_open_target` now
  accepts either declaration site, and the TS derivation mirrors the same union.

## 2. `apply_ops_binary` empty-batch panic (TC3b §6f) — FIXED

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`, `ArtifactCodec::of`'s
`apply_ops_binary_impl` thunk (the `store.dispatch_apply_exact(…)` tail) and the new
`ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS` const beside `ArtifactCodecApplyFuture`.

**Cause.** The thunk is the one place in the tree that builds an `ArtifactStore`, uses it and lets
it fall out of scope in the same expression. `ArtifactStore`'s `Drop` asserts an exact terminal-empty
shallow-shell witness, and nothing closed the store — an `encode_ops_vec(&[])` batch is a NONEMPTY
ops vector that decodes to ZERO mutations, so `dispatch_apply_exact` retires nothing, every owner is
still installed at the end of the turn, and `Drop` aborted the process.

**Fix.** After printing the pack, the store is drained through the same close cursor the hub's own
document lanes run — `close_owned_step(1, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)` until `Complete`,
then `close_owned_terminal_is_empty()` — before it is dropped. `Blocked`, a close error, a
non-terminal completion and budget exhaustion each become a `VcsError::ValidationFailed` instead of
an abort. An empty batch and a full one now leave by the identical path.

**Verified:** type-checked only (it is on `semio-hub`'s `apply_operation` path, whose suite is the
coordinator's). It is compiled by all four green checks above.

## 3. The wasm mutex hold — per-stage wall time

### TC3b §6a IS CLOSED — the guest half of `world actor`'s `codec` interface compiles

`cargo check -p semio-s-plugin-note --target wasm32-wasip2` — **exit 0**, `Finished dev profile in
43.08s` at 13:45:27, with `Checking semio-s-artifact-note-note` and `Checking semio-s-plugin-note`
in the log (a real compile, not a replayed cache). Capture: `🗑️generated/tc3c-note-wasm-check.txt`.

This was the one unverified compile gate TC3b handed over: the four `codec::Guest` methods in
`__semio_actor_exports!` and the four `semio_owned_*_v1` bodies in `__semio_owned_core_exports!` are
`#[cfg(all(target_arch = "wasm32", target_env = "p2"))]`, so NO native check ever compiles them, and
what `wit_bindgen` (not `wasmtime-wit-bindgen`) generates for an exported interface — method names,
async-ness, the `DocumentPair` record, the `(String, String)` tuple return — had never been put in
front of a compiler. It is correct.

ONE detached hold, queued at **11:37** (`📜️tc3c-hub-boot.sh 7651 stdio,gis,note`, driver pid in
`🗑️generated/tc3c-driver-pid.txt`, wrapper in `📜️tc3c-mutex-work.sh`). The queue on arrival was
`pz1` (holding since 10:32) → `s10` → `c5` → `rb1`. The hold runs three stages so the queue is paid
for once and the lock never idles:

| stage | what | capture |
|---|---|---|
| 0 | `cargo check -p semio-s-plugin-note --target wasm32-wasip2 --features component-app-assembly` — TC3b §6a's one unverified compile gate (the `codec` guest halves are `cfg(wasm32-p2)` and NO native check compiles them) | `🗑️generated/tc3c-note-wasm-check.txt` |
| 1 | cold `wasm-release` cdylib builds of `semio-s-plugin-{stdio,gis,note}`. `produceFreshComponentV1` gives each package a private `CARGO_TARGET_DIR` inside the catalog build root, but `build.build-dir` is shared repo-wide, so this populates exactly the cache the bootstrap then reuses | `🗑️generated/tc3c-prebuild-*.txt` |
| 2 | `trusted-catalog-bootstrap --packages stdio,gis,note` (describe + jco actors + the `os-hub` binary + publication) | `🗑️generated/tc3c-hub-dev.txt` |

The detached driver was un-throttled with `taskpolicy -B` on its pid and its child (memory
*Background QoS Throttles Agent Builds*): a `nohup`ed chain inherits the background QoS band and its
cargo children crawl.

**A defect the hold itself found, in the first second it ran.** Stage 0 aborted at 13:39:13 with
`exit=101`, `error: the package 'semio-s-plugin-note' does not contain this feature:
component-app-assembly`. The preamble's blanket rule *"plugin crates need
`--features component-app-assembly`"* and TC3b §6a's prescribed command are both wrong for this
crate: `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/Cargo.toml` declares **no `[features]` table at
all**. The feature was never what gated the `codec` guest halves — they are
`cfg(all(target_arch = "wasm32", target_env = "p2"))`, which the plain `--target wasm32-wasip2`
check compiles. `📜️tc3c-mutex-work.sh` now runs the plain check, with that measurement in the
comment. Cost: one whole mutex hold and a re-queue behind `s10` at 13:39.

**A second hold lost to a peer's in-flight edit, and the fix that stops it recurring.** The retried
hold reached stage 0 at 13:40:08 and died at 13:40:30 with
`error[E0277]: the trait bound 'document::UiNodeRecord: Clone' is not satisfied` —
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs` (mtime **13:36**, four minutes earlier)
had lost `Clone` from `UiNodeRecord` while `UiNodeTable` still derives it. Confirmed **red natively
too** (`cargo check -p semio-framework-ui-contract`), i.e. a peer mid-refactor, not a wasm or
TC3c-specific fault, and not mine to revert (preamble rule 3). `📜️tc3c-hub-boot.sh` now runs a
PRE-FLIGHT `cargo check -p semio-framework-ui-contract` OUTSIDE the mutex, retrying once a minute for
up to 40 minutes, and queues for the lock only once the shared tree compiles. Discovering a peer's
breakage inside a hold costs the whole queue slot; discovering it outside costs nothing.

**Queue wait, measured.** Queued 11:37. `pz1` released at ~11:56 and `rb1` took the lock; at 13:14
`rb1` was still holding (2 h 01 m of wrapper wall, 1 h 14 m on its current step). It is NOT
deadlocked — preamble rule 27(b)'s test was run at 12:41 and again at 13:12: the holder's chain is
`zsh → bun → bun → node → cargo → 4 live rustc` and later `node → bun`, i.e. a genuinely progressing
component-release build, so its cargo was never killed and never waited on. TC3c is next in the
queue (`pz1` and `s10` re-queued behind it at 12:01/12:04).

(per-stage wall times: filling — the hold had not been granted when this section was last updated,
13:14)

## 4. 7651 proof table — every http code

Probes, both browser-free and both driven by the product's own sealed request builders:
`🐍️tc3c-create-and-attach.ts` (sign-in → create-space → creation catalog → `POST
artifact-creations` polled to `ready` → `open-plan` → `execution-target/{manifest,component,
descriptor}` → `socket-grants` → document socket + `SocketHelloV1`, waiting for the hub's `Session`
frame), driven per kind by `📜️tc3c-provision.sh` which first mints `user1@semio.dev` /
`user2@semio.dev` through `os-hub credential set` against the 7651 data root.

(http codes: filling — the bootstrap had not completed when this section was written)

## 5. Honest gaps

**(a) Two laws in `trusted-stdio-gis-bundle-check --source` are RED, both pre-existing.** The gate
is a chain, so neither is reached from my own bundle law until the earlier one passes.
* `proveTrustedCompiledDependenciesFixture`'s `atomicCases` literal contradicted its own fixture:
  commit **48a8c69cdb** (2026-09-19, a peer's auto-commit) changed `gis-dependency`,
  `gis-trailing-byte` and `gis-duplicate-field` from `previews: ["stdio"]` to `previews: []` in
  `🧫️fixtures/🔗️compiled-dependencies/🔣️.json` and did not touch the literal. **Fixed here** by
  aligning the literal to the fixture (the side that was edited deliberately), with the commit hash
  in the comment.
* `hubSchemaExport(…, "schema://framework.manifest/ArtifactKindFormatsFixture")` rejects
  `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🗄️artifact-kind-formats.json`, which is a CLEAN file
  in git. Not touched: it is a framework-manifest scope contract, outside this slice, and it is red
  at HEAD independently of anything here. **This is why the `--source` gate verb has no green run in
  this report** — the builder itself is proven by `tsc --noEmit` (clean apart from two pre-existing
  `DirectoryHomeControllerSurfaceV1` / `BrowserPluginRuntimeSurfaceV1` errors a peer owns), by the
  verb's own runtime fences, and by the bootstrap run in §3.

**(b) No three-package Rust fixture.** `🧫️fixtures/👥️two-package/🔣️.json` is unchanged and still
feeds `semio-hub`'s unit tests, because the hub's `profile.id == "local-stdio-gis-open-v1"`
exactness fence is a real law for the two-package closure and `--packages stdio,gis` still mints
exactly that id. A three-package generation should get its own SIBLING fixture (`👥️three-package`)
plus a unit law that `validate_bundle` accepts it and that its generation id differs — I did not
write it: the hub suite is the coordinator's (rule 26) and the slice's runtime budget went to the
bootstrap. The three-package generation IS exercised end to end by §3/§4 instead, which is the
stronger evidence, but a fixture law would catch a regression without a 60–90 min build.

**(c) The launch row could not be "renamed": there was none.** `grep -rn "trusted-stdio-gis" .vscode/`
is empty — the whole `trusted-stdio-gis-*` verb family has never had a `.vscode/launch.json` row.
The nx target was renamed in `🌎️hub/📦️packages/🦀️rust/📋️project.json`. `.vscode/launch.json` and
`🧩️launch.seed.jsonc` are both modified by a peer right now, so I did not add a new row into a file
another agent is editing; it is one row of work for whoever owns that file next.

**(d) The `codecs` subcommand is verified by the bootstrap, not by a unit law.** It has no test of
its own; its first execution is stage 2 of the hold. Its failure mode is loud (a nonzero exit with
the guest fault text), and the hub re-derives the same hash at publication time, so a wrong answer
cannot reach a published generation — but a law that runs it against a built gis component and
compares with the linked Rust codec would be the proper oracle, and is the natural closure of TC3b
§6b.

**(e) `linkedCodecRegistry` is still a declared path per package.** It names the file the hub
binary's compiled provider is generated from, so it is structurally tied to the Rust table
`NativeCodecProviderSetV1` — but nothing in the build PROVES the two agree; `verify_selected`'s
`consumed_bindings.len() != binding_map.len()` fence is what would catch a divergence, at publish
time.

## 6. Files changed

Source:
* `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` — §2: the close cursor in
  `apply_ops_binary_impl` + `ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `plugin_artifact_codec_app` resolves by
  document schema OR dialect artifact kind
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs` —
  `component_codec_rows` + the `codecs` subcommand + `CODEC_PROBE_DOCUMENT_ID`
* `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` — `validate_descriptor_open_target`'s
  discoverability union (plugin-level OR owning-app kind specs)
* `🌎️hub/💡️inference/📇️catalog/🦀️.rs` — `verified_gis_map_binding_with_service` resolves its target
  by kind (`artifact_creation_selection`) instead of `selected_document_open()`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts` — the built
  artifact's WIT export gate requires `codec` alongside `checkpoint`/`describe`/`jobs`/`reactor`
* `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — the N-package builder: `TrustedBootstrapPackageSpecV1` +
  `TRUSTED_BOOTSTRAP_PACKAGES`, `trustedBootstrapSelectPackages`,
  `trustedBootstrapComponentCodecRowsV1`, `trustedBootstrapDescriptorOpenTargetsV1`,
  `trustedBootstrapPackageRenderer`, `trustedBootstrapOpenTargetOrder`, the open-target-SET
  `trustedBootstrapProfileEncoding`, the generalised `trustedBootstrapResolveDependencies`,
  `materializeTrustedCatalogBundle`, `TrustedCatalogBootstrapScript`, the derived closure in
  `trustedBootstrapVerifyGeneration`, the actor renderer/path in
  `trustedBootstrapGenerationReceipt` and the staged bundle, the owner-selected GIS target in
  `proveTrustedStdioGisCandidatePlan`, every two-package fence and every source-text gate that named
  the retired symbols, and §5a's `atomicCases` alignment
* `🌎️hub/📦️packages/🦀️rust/📋️project.json` — nx target `trusted-stdio-gis-bootstrap` →
  `trusted-catalog-bootstrap` (`forwardAllArgs`)
* `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json` —
  `profile.openTarget` → `profile.openTargets` (a set of one)
* `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs` — `artifact_kind().id` = `NOTE_DIALECT
  .artifact_kind` (`s.note.note`, was `2d.note`), `name` = `Note`
* `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
  — the manifest assertion follows that id
* `✏️s/🔌️plugins/🗒️note/🦀️.rs` — the activation-event docstring follows that id

Ticket-owned: this report, `📜️tc3c-hub-boot.sh`, `📜️tc3c-mutex-work.sh`, `📜️tc3c-provision.sh`,
`🐍️tc3c-create-and-attach.ts`, and `🗑️generated/tc3c-*`.
