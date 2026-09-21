# TC3 — catalog-carried genesis: can any plugin's document kind be CREATED on a hub?

Slice TC3 of ticket 26/09/18. Owner brief: land TC2 §9 (genesis becomes catalog data), generalise the
bootstrap verb to N plugins, bootstrap a fresh root on port 7651 and create the first `s.note.note`
document on a hub.

## 0. HUB HANDOFF (top of report)

**No hub source was changed by this slice. No coordinator hub rerun is needed.**
Baseline `cargo check -p semio-hub --all-targets` is **green** on the unmodified tree at 10:40 on
2026-09-21 (§5.2).
Nothing was started, nothing was killed, no peer file was reverted, no `🗑️generated` file I did not
create was touched, and the live hubs / surviving data roots (`gm1-boot`, `jc1-boot`, …) were never
opened.

**Why nothing landed is the finding**, not an excuse: TC2 §9's design is *necessary but not
sufficient*, and landing it in this fleet session would have broken every peer's live hub. §1 is the
measurement, §2 the corrected design, §3 the landing order that is actually safe. Read §1.5 first if
you only read one thing.

## 1. What I measured (all line numbers are as of 2026-09-21 ~04:00)

### 1.1 The genesis pair is NOT one pair per kind — the pack is, the spr is not

TC2 §9 assumes "the canonical empty document `(pack, spr)` per artifact kind" can be staged once, in
the cold build, and reused for every document of that kind. Half of that is true and the half that is
false is the half that decides the design.

`native_artifact_genesis_for_editor`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32763`) takes `(document_id, dialect)`,
refuses any id that is not a server-minted `artifact-<32 hex>`
(`is_server_minted_artifact_document_id_v1`, `:32757`), builds the envelope and ends in
`store::print_document_pack`.

`print_document_pack` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:12135`) is two
independent halves:

```
let pack = envelope.vcs.initial_snapshot.encode_pack();   // ← E::initial_snapshot() only
let spr  = print_document_spr(envelope).await?;            // ← carries doc_id
```

* **`pack` is document-id independent.** It is exactly `E::initial_snapshot().encode_pack()`. One
  staged blob per artifact kind is correct, and TC2 §9's `sourceComponentSha256` pinning works on it
  unchanged.
* **`spr` is not.** `print_document_spr` (`:11964`) ends in
  `HistoryLog { doc_id: envelope.id, schema, edits: [], transitions: [], composition, conflicts: [] }`
  → `os_spr::encode_history`. For a genesis document every list is empty, so the spr is a pure
  function of `(doc_id, schema, composition)` — but `doc_id` is minted per creation, so a staged spr
  belongs to exactly one document and to no other.

Consequence for the design: the catalog can carry the **pack** verbatim, but the **spr** has to be
either (a) re-encoded hub-side with the real id (`os_spr::decode_history` → set `doc_id` →
`encode_history`; the hub already links `os_spr`, it calls `encode_ops_vec` in
`🌎️hub/🗿️artifact-authority/🔌️adapters/🦀️.rs:70`), or (b) synthesised hub-side from a carried
`(schema, composition)` row. Byte-substituting the id is tempting because both ids are exactly 41
bytes — do not: the id also appears in the composition records, and nothing proves the encoder has no
length-dependent framing elsewhere.

**This is a correction to TC2 §9, not a blocker.** Option (a) is small and exact, and
`materialize_selected_genesis` already runs `codec.validate_pair(Input)` **and** `(Output)` over the
result (`🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:80-84`), so a bad re-encode fails closed.

### 1.2 Genesis is only ONE of four places the creation path is welded to a linked native codec

TC2 §9 says the native table "loses only `genesis`". Measured, `POST /spaces/{s}/artifact-creations`
(`🌎️hub/🏗️bootstrap/🦀️.rs:9736`) reaches a linked Rust codec four separate times:

| # | site | what it needs | survives genesis-as-data? |
|---|---|---|---|
| 1 | `artifact_creation_selection` (`🔏️trusted-catalog/🦀️.rs:352-368`) | a `VerifiedNativeArtifactCodec` whose `genesis.is_some()` **and** whose package/kind/schema/`pack_schema_hash` match the selection | yes — this is the one TC2 §9 fixes |
| 2 | `materialize_selected_genesis` → `catalog.resolve(&identity)` (`🌱️creation/🦀️.rs:76`) | a `VerifiedNativeArtifactCodec` for the exact identity to exist **at all** | **no** |
| 3 | the same function, `codec.validate_pair(Input)` and `(Output)` (`:81`, `:83`) | `ArtifactCodec::print_mirror` — a linked `fn` pointer | **no** |
| 4 | every later accepted edit, `TrustedArtifactCodec::apply_operation` (`🔏️trusted-catalog/🦀️.rs:275`) | `ArtifactCodec::apply_ops_binary` — a linked `fn` pointer | **no** |

`VerifiedNativeArtifactCodec`'s codecs come only from `NativeCodecProviderSetV1::linked()`
(`📇️native-openable-provider/🦀️.rs:28`), a `const` table of stdio + gis + vcs. The only other
`TrustedArtifactCatalog` in the tree, `PluginHostTrustedArtifactCatalog`
(`🔌️adapters/🦀️.rs:154`), resolves through `os_store::document_codec(&kind.schema)` — the
process-wide **native** registry — so it is not an escape either. Grepped: there is no
wasm-backed `impl TrustedArtifactCodec` anywhere in `🌎️hub`.

So even with a perfect catalog-carried genesis pair, `note` creation stops one line later, at
`catalog.resolve`. **Catalog-carried genesis does not, by itself, make any new kind creatable.**

### 1.3 A profile admits exactly ONE open target, so a generation carries at most one creatable kind

`TrustedCatalogLoader::verify_selected` ends with
`if open_targets.len() != 1 { return Err(catalog("selected profile must resolve exactly one document-open target")) }`
(`🔏️trusted-catalog/🦀️.rs:715`), and the loop that fills `open_targets` `continue`s over every
target that is not the profile's single `profile.open_target` (`:679`).
`artifact_creation_selection` and `artifact_creation_catalog` (`:352`, `:372`) both iterate
`self.open_targets`.

Therefore the brief's "for ANY kind the generation carries" is, today, "for the one kind the profile
names". A three-package `stdio + gis + note` bundle would still expose exactly one creatable kind.
Lifting this is a separate, independent change (profile → *set* of open targets, with the generation
id framing the set — `trusted_profile_generation` already hard-codes `1u32` for the open-target count
at `:889`).

### 1.4 `note` cannot be the third package, and `vcs` is still one string away

Re-verified TC2 §4.1/§4.2 against today's tree, unchanged:

* `note` has no `📇️native-codecs` module (only `gis` and `vcs` have one in the whole repo) and
  `semio-hub`'s `native-artifact-execution` feature does not depend on `semio-s-plugin-note`
  (`🌎️hub/📦️packages/🦀️rust/Cargo.toml:30`). Adding one is not a shortcut — it is exactly the
  linked-codec coupling this slice exists to delete.
* `vcs` is linked and has a genesis factory, but its codec identity says `artifact_kind: "s.vcs.vcs"`
  (asserted literally at `📇️native-openable-provider/🦀️.rs:105`) while its manifest
  `ArtifactKindSpec::id` is `vcs.vcs`. `artifact_creation_selection` needs
  `codec.identity.artifact_kind == selection.artifact.kind` and `validate_descriptor_open_target`
  (`🔏️trusted-catalog/🦀️.rs:768`) needs that same value to be a **manifest** kind
  (`descriptor.manifest.artifact_kinds.iter().any(|kind| kind.id == target.artifact_kind …)`). Both
  cannot hold.

  One measurement TC2 did not have: the canonical `s.<plugin>[.<artifact>]` grammar in
  `DocumentIndexEntryV1::validate`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📇️document-index-v1/🦀️.rs:29`,
  `canonical_dialect_artifact_kind`) bounds **`dialect.artifact_kind`**, and its own `🪢` docstring
  says explicitly that this is *not* the manifest `ArtifactKindSpec::id` (`2d.note`, `stdio.json`).
  So the index grammar does **not** force `vcs.vcs → s.vcs.vcs`; it leaves TC2 §5 step 2's decision
  exactly where TC2 left it. I did not land it: it is not my slice, it changes a persisted envelope
  id or a manifest kind, and it needs the hub suite plus a fresh binary to be worth anything.

### 1.5 Why landing TC2 §9 *now* would have broken the fleet — the decisive constraint

Any genesis row added to `TrustedBundlePackageV1` must be framed into `trusted_profile_generation`
(`🔏️trusted-catalog/🦀️.rs:855-900`) — that is the whole point of "inside the generation id and
cannot be swapped without rotating the generation" (TC2 §9 step 2). But
`verify_selected` refuses any bundle whose recomputed generation differs from
`profile.generation_id` (`:719`). So **the first `os-hub` binary built from a tree carrying the new
framing refuses every catalog published before it** — `gm1-boot`, `jc1-boot`, `hs1-boot`, C5's
published-after-jco-1.34 root on 7621, and every data root the live-collaboration and presence
slices are standing on right now.

Making the field optional-and-unframed would be exactly the compat branch the ticket forbids, and
making it optional-but-framed changes the id anyway. There is no landing that is both correct and
non-breaking; the change is atomic with a re-bootstrap of every root still in use.

The fleet is mid-flight on outcome 3 (two users collaborating over a hub) with those roots, and the
coordinator owns the `os-hub` binary (preamble rule 26) — so a source change of mine would reach them
through the coordinator's next rebuild without their consent. Rule 21 makes outcome 3 observed-at-
runtime the bar. I therefore did not land it, and §3 says what has to be true first.

## 2. The corrected design

Unchanged from TC2 §9: genesis provenance as catalog data, hash-pinned by `sourceComponentSha256`,
framed into the generation id, no wire change to creation / open-plan / checkpoint. Corrections:

1. **Carry the pack, derive the spr.** `packages/<plugin>/genesis/<kindId>.pack` is the staged blob
   (`{ kindId, pack: {path, byteLength, sha256}, schema, sourceComponentSha256,
   sourceDescriptorByteSha256 }`). The hub mints the spr at creation from the retained
   `(schema, composition)` plus the real `document_id`, through `os_spr::encode_history`, and
   `materialize_selected_genesis`'s existing `validate_pair(Input)`/`(Output)` is the fence. Staging
   an spr for a placeholder id and rewriting bytes is refused (§1.1).
2. **Genesis-as-data is step 1 of 3, not the whole move.** The blocker that actually gates "any
   plugin's kind" is §1.2 rows 2–4: the hub has no executable codec for a package it did not link.
   The move that dissolves all three at once is a **wasm-hosted `TrustedArtifactCodec`** — the hub
   already links `semio-framework-plugin-host` (wasmtime 47, `component-model-async`;
   `🌎️hub/📦️packages/🦀️rust/Cargo.toml:58`) and already retains the verified component bytes per
   package (`VerifiedTrustedPackage::component_bytes`, handed to the browser by
   `assets_for_current_selection`). `print_mirror` / `apply_ops_binary` / genesis are all
   guest-callable through the same actor turn the browser drives.
   Once that exists, genesis needs no staged blob at all: the hub asks the component, exactly as
   TC2 §9 wanted, but at creation time and with the real document id — which also makes §1.1 moot.
   Staged genesis blobs are then a cold-start optimisation, not a mechanism.

   **Do not mistake the existing adapter for a head start.** `PluginHostTrustedArtifactCatalog`
   (`🔌️adapters/🦀️.rs:79-158`) is reachable *only* from
   `🗿️artifact-authority/🧪️tests/🔬️unit/🦀️.rs:385-387` — grepped repo-wide, nothing in
   `🏗️bootstrap/🦀️.rs` or anywhere else in production constructs it, and no wasmtime engine is
   booted in the `os-hub` process at all. It also resolves through `os_store::document_codec`, the
   process-global **native** registry, which `📓️terra-hub-headless-catalog-audit.md:139`
   (2026-09-03) already measured as starting empty with no registration call at `os-hub` startup —
   0/48 declared schemas covered. So the dependency edge is free, and everything behind it is new
   work.
3. **The producer TC2 §9 step 1 assumes does not exist, and adding it rebuilds the whole fleet.**
   §9 says the cold build instantiates the freshly built component "and asks, for each creatable
   artifact kind it declares, for its empty document". Measured against the ABI, it cannot ask.
   `world actor` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1388`) exports
   exactly four interfaces — `reactor`, `jobs`, `checkpoint`, `describe`:
   * `describe: async func() -> list<u8>` (`:1369`) returns a packed `PackageDescriptor` and nothing
     else; it is the build-time call the cold bootstrap already makes.
   * `checkpoint: async func() -> result<list<u8>, …>` is the *actor's* state, not a document pair.
   * a document pair only ever leaves a guest as the `load-document(load-document-effect)` **effect**
     (`:463`, `:590`), i.e. guest→host and only when the guest chooses to emit it. The SDK's
     `plugin_document_pack` (`🔌️plugin/🦀️.rs:36740`) is guest-side; it is not a WIT export and the
     host cannot call it.

   So step 1 needs a **new export** on `world actor` — something like
   `describe.genesis: async func(kind: string, document-id: string) -> result<tuple<list<u8>, list<u8>>, plugin-error>` —
   plus host support in `semio-framework-plugin-host` and a `--genesis` mode in the
   `semio-framework-plugin-describe` binary. A `world actor` change is not additive in practice: it
   invalidates every already-built component, so **all ~58 plugin components must be rebuilt cold**
   before any catalog can be published. That cost belongs in the estimate and was not in §9's.

   Note this export also dissolves §1.1: asked at creation time with the real `document-id`, the
   guest returns a correct pair and nothing needs to be staged or re-encoded. Taken with §2.2, the
   same export is most of the wasm-hosted codec — which is the argument for doing them together
   rather than staging blobs first.

4. **Profile → open-target set** (§1.3) is an independent prerequisite for "N creatable kinds in one
   generation", and is cheap next to (2) and (3): lift the `!= 1` fence, frame the sorted set instead of the
   hard-coded `1u32` at `:889`, and keep `resolve_document_open`'s unambiguity rule.

## 3. Landing order that is actually safe

1. **Fleet quiescence for the hub binary.** Nothing here can land while peers' live proofs depend on
   pre-existing published generations (§1.5). Land it in a window where the coordinator re-bootstraps
   every root still in use, or after the collaboration slices have their receipts.
2. **The `world actor` genesis export + wasm-hosted `TrustedArtifactCodec`** (§2.3 then §2.2) behind the existing `NativeCodecProviderSourceV1`
   port, proven first on `gis` (whose native codec gives a byte-for-byte oracle: same pack, same spr,
   same `pack_schema_hash`). This is the largest and the only load-bearing piece.
3. **Profile open-target set** (§2.4), with `trusted_profile_generation` framing the set.
4. **Genesis via the guest** — delete `genesis` from `NativeCodecBinding` (`:128`, `:137-145`,
   `:165`), `VerifiedNativeArtifactCodec` (`:257`), the `gis`/`vcs` `into_codec_and_genesis` calls
   (`📇️native-openable-provider/🦀️.rs:87`, `:119`), and turn `artifact_creation_selection`'s
   `codec.genesis.is_some()` into "the selection's package can be asked for this kind's genesis".
5. **Only then** the N-plugin bootstrap list, which at that point is a genuine list change
   (TC1 §5's eight welded rows + TC2 §3's two: the per-request codec *projector* and the
   `🧬️stdio-gis-bootstrap` fixture's `limits`/`sources` at `📜️script.ts:8462-8470`).

## 4. What of the brief was NOT done, and why (honest gaps)

| brief item | state |
|---|---|
| (1) land genesis as catalog data | **not landed** — §1.5 (breaks every live root) and §1.2 (insufficient on its own). Design corrected instead (§2). |
| (1) laws in Rust + TS twins + a stdio/gis/note fixture | **not written** — they pin a schema I deliberately did not change; a fixture asserting a note genesis row would assert a creation that still cannot resolve a codec (§1.2). |
| (2) generalise `trusted-stdio-gis-bootstrap` to an N-plugin list | **not landed.** Re-confirmed the ten welded rows are where TC1/TC2 left them (`materializeTrustedStdioGisBundle`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts:9416-9612`; the hard `codecs.stdio.length !== 26 \|\| codecs.gis.length !== 2` at `:9497`; `schemaVersion: 2` + `packages?.length !== 2` at `:9615`). With no admissible third package (§1.4) the parameter would have exactly two possible arguments. |
| (3) bootstrap `.🧬semio/🌐hub/tc3-boot`, hub on 7651, create `s.note.note` | **not attempted.** It cannot succeed: `note` has no native codec, so `catalog.resolve` fails (§1.2) and `validate_descriptor_open_target` has no note codec row to admit the target. A 3 × 45–60 min cold `wasm-release` chain through the shared mutex (C5 held it to ≈ 04:30, S10 and PZ1 queued behind) to reach a refusal already provable by reading is exactly the burn preamble rule 17 forbids. **No http codes to record — no request was made.** `📜️tc3-hub-boot.sh` was not written. |
| (4) `cargo check -p semio-hub --all-targets` + artifact-authority tests | check **run and red — in a peer's file, not mine** (§5): the shared tree does not currently compile `semio-framework-plugin`, so `semio-hub` never got a verdict. `cargo test -p semio-hub` is the coordinator's (rule 26) and the artifact-authority tests live inside `semio-hub`, so I could not run them; with no source change of mine, the coordinator's existing `🗑️generated/coordinator-hub-*.txt` remains the authority. |

Two claims in TC2 §9 are wrong and are corrected here rather than in that file (it is a peer's
report): "the canonical empty document `(pack, spr)` per artifact kind" (§1.1) and "the native codec
table … loses only `genesis`" (§1.2).

## 5. Verification actually run

### 5.1 Run 1 — 2026-09-21 ~04:2x — RED, in a peer's file

`cargo check -p semio-hub --all-targets`, private `CARGO_TARGET_DIR=…/⚡️cache/cargo/target-tc3`,
shared build-dir. **Exit 101 — 7 errors, none in `semio-hub`.** The run died in a *dependency*,
`semio-framework-plugin` (lib), on the U3 mutation-label localisation edit mid-flight in that file
(`git status --short …/🔌️plugin/🦀️.rs` → `MM`):

```
error[E0277]: `LocalizedLabel` doesn't implement `std::fmt::Display`
error[E0308]: …/🔌️plugin/🦀️.rs:11145  labels.push(protocol::SemanticMutation::label(op));  expected `String`, found `LocalizedLabel`
error[E0308]: …/🔌️plugin/🦀️.rs:24492  CommandLogAppend { … label … }                        expected `LocalizedLabel`, found `String`
```

`semio-hub` itself never got a verdict. I did not touch the peer's file and did not retry into their
edit window (preamble rule 3/13).

### 5.2 Run 2 — 2026-09-21 10:36–10:40, after the session-7 outage — GREEN

Same command, same private target dir. **Exit 0, `Finished dev profile in 3m 53s`, 335 warnings**
(warnings across `semio-hub` lib + `os-hub` bin + both test targets, so the whole tree really
type-checked rather than short-circuiting on a cached success). U3's `LocalizedLabel` conversion had
landed in the meantime. Capture: `🗑️generated/tc3-hub-check.txt`.

This is a **baseline on an unmodified tree** — I changed no source, so there is nothing of mine for
it to have caught. Its value is negative evidence: the hub tree is green, so nothing in §1 is an
artefact of a broken checkout.

`cargo test -p semio-hub` / the artifact-authority unit tests remain the coordinator's (rule 26).

## 6. Files changed

**None.** Ticket-owned additions only: this report and `🗑️generated/tc3-hub-check.txt`.
