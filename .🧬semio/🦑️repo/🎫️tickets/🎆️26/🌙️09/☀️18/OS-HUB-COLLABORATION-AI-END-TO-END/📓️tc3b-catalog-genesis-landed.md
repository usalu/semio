# TC3b — catalog-carried genesis LANDED: any plugin's document kind creatable on a hub

Slice TC3b of ticket 26/09/18. Brief: land TC3's corrected design
(`📓️tc3-catalog-carried-genesis.md` §2) in TC3's own order (§3), then bootstrap a fresh root on
port 7651 and create the first `s.note.note` document on a hub.

## 0. HUB HANDOFF (top of report)

**Hub source IS changed by this slice — the coordinator needs a `semio-hub` rebuild + suite rerun.**

* `cargo check -p semio-hub --lib` — **exit 0**, `Finished dev profile in 2m 58s`, 31 lib warnings
  (`🗑️generated/tc3b-check2.txt`).
* `cargo check -p semio-hub --all-targets --keep-going` — **exit 0**, 338 warnings across lib + bin +
  both test targets (`🗑️generated/tc3b-check4.txt`). Warnings on every target are the proof the whole
  tree really type-checked rather than short-circuiting a cached success.
* `cargo test/nextest -p semio-hub` is the coordinator's (preamble rule 26) and has NOT been run.

**Accepted break, per the brief and `📓️jc1-jco-task-return-and-republish.md` §6.** This tree changes
`world actor` (a new `codec` interface) and the owned Semio actor ABI (nine core exports →
thirteen). Every component built before it is refused by `OwnedSemioArtifact::from_component`, and
every catalog published before it carries a profile shape (`openTarget`) this binary no longer
reads. Live roots `gm1-boot` / `hs1-boot` / `jc1-boot` were never opened and keep serving from their
already-built binaries (`⚡️cache/hs1/os-hub-7611`, `⚡️cache/cargo/target-jc1/debug/os-hub`); nothing
was started, nothing was killed, no peer file was reverted, and no `🗑️generated` file I did not
create was touched.

## 1. Landing order and state

| step | what | state |
|---|---|---|
| 1 | `genesis` (+ the rest of the creation-path codec surface) exported from `world actor` WIT; one SDK implementation for every plugin | **landed**, native checks green, owned-ABI law + two genesis laws RUN green; the `cfg(wasm32-p2)` guest impl is **compiled only natively** (§6a) |
| 2 | wasm-hosted `TrustedArtifactCodec` in the hub artifact authority | **landed**, `cargo check -p semio-hub --all-targets` green |
| 2b | gis byte-for-byte oracle law | **landed and RUN green** (§3) — as a native-producer-vs-linked-codec equality, not guest-vs-native (§6b) |
| 3 | generation admits a SET of open targets; `trusted_profile_generation` frames the set | **landed** in Rust + JSON schema + fixtures + the TS bundle writer |
| 4 | `NativeCodecBinding::genesis` + the stdio/gis/vcs const genesis table deleted | **landed** |
| 5 | bootstrap verb takes an N-plugin list, default `stdio,gis,note` | **NOT landed** (§6c) |
| 6 | 7651 proof: bootstrap → sign in → space → creation → open-plan → execution target → socket | **NOT run** (§6d) — no http codes to record, because no request was made |

## 2. WIT diff summary

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit`, +46 lines: one new
`interface codec`, and `export codec;` added to `world actor` (which now exports
`reactor`, `jobs`, `checkpoint`, `describe`, `codec`).

```wit
interface codec {
  use types.{plugin-error};
  record document-pair { pack: list<u8>, spr: list<u8> }
  pack-schema-hash: async func(artifact-kind: string) -> result<list<u8>, plugin-error>;
  genesis:          async func(artifact-kind: string, document-id: string) -> result<document-pair, plugin-error>;
  print-mirror:     async func(artifact-kind: string, pair: document-pair) -> result<tuple<string, string>, plugin-error>;
  apply-ops:        async func(artifact-kind: string, pair: document-pair, ops: list<u8>) -> result<document-pair, plugin-error>;
}
```

Four design decisions, each answering a specific TC3 measurement:

1. **`genesis` takes the real `document-id`.** TC3 §1.1 measured that a genesis pair's `pack` is
   `E::initial_snapshot().encode_pack()` (id-independent) but its `spr` is
   `HistoryLog { doc_id, .. }` (belongs to exactly one document). Asking the guest at creation time,
   with the server-minted id in hand, is exact; staging an id-independent template and re-encoding
   its id host-side is not (the id also appears in composition records). So the catalog carries the
   *provenance* — the hash-pinned component — and the pair is minted per creation.
2. **The selector is the document SCHEMA**, not the manifest kind id and not the app id. `schema` is
   what `store::ArtifactCodec` is keyed by, what `TrustedArtifactIdentity::artifact_schema` carries
   and what `PluginApp::artifact_schema()` already answers, so there is exactly one unambiguous
   mapping and no new identity space. (The WIT parameter is spelled `artifact-kind` for continuity
   with the ABI's other identity parameters; the value passed and matched is the schema.)
3. **Four functions, not one.** TC3 §1.2 measured that creation reaches a linked Rust codec FOUR
   times, and genesis is only the first: `catalog.resolve` needs the identity to exist,
   `validate_pair(Input)` and `(Output)` need `print_mirror`, and every later accepted edit needs
   `apply_ops_binary`. A `genesis`-only export would have moved the wall one line, exactly as TC3
   predicted. `pack-schema-hash` is the fourth: it is how a catalog row's `packSchemaHash` gets
   pinned to the component for a package the hub links no codec for.
4. **The owned ABI gets the same four.** The production `describe` path is the repository-owned
   interpreter (`execute_describe_owned`), not wasmtime — `execute_describe_wasmtime` is
   `#[cfg(test)]`. So the same single SDK implementation is exported twice: through the WIT for
   wasmtime and jco/browser, and through `semio_owned_{pack_schema_hash,genesis,print_mirror,
   apply_ops}_v1` for the owned interpreter, which is the path the hub actually calls. This is the
   established two-surface shape of the actor ABI, not a second channel: `__semio_owned_core_exports!`
   and `__semio_actor_exports!` already coexist by construction, and the law below holds them equal.

**One producer, every plugin.** `app::artifact_app_genesis_pair::<A: ArtifactApp>` and
`app::artifact_app_apply_ops::<A>` replace `native_artifact_genesis_for_editor::<E: ArtifactEditor>`
outright (the old one, `NativeArtifactGenesisFactoryV1` and `NativeArtifactGenesisFutureV1` are
deleted, not kept alongside). They are reached through four new `PluginApp` methods
(`artifact_pack_schema_hash`, `artifact_genesis_pair`, `artifact_print_mirror`,
`artifact_apply_ops`), implemented once on `VcsArtifactApp<A, M>` — the SDK's only `PluginApp`
implementor — and forwarded to every plugin's closed app enum by `#[dyn_enum]` with no per-plugin
edit. `plugin_runtime::plugin_artifact_codec_app` resolves the owning app from the installed
bundle's own manifest, preferring the editor and refusing an ambiguous schema rather than picking by
order.

## 3. Oracle law result

`gis_guest_genesis_is_byte_for_byte_the_linked_codec_pair_on_one_generation`
(`✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧪️tests/📇️native-codecs/🦀️.rs`). GIS is the one package where
both halves exist in one process, so it is the only place the equality can be asserted without a
built component. For each of the two GIS codec receipts, on one generation and one server-minted
document id, the law asserts:

* the guest producer emits a complete nonempty pair;
* the **linked** `ArtifactCodec::print_mirror` accepts it (the exact fence
  `materialize_selected_genesis` applies twice before publication);
* decoding that pair with the linked package's own concrete `Snapshot`/`Mutation` types and
  reprinting it returns the **identical** `pack`, `spr` and `ops`.

```
running 3 tests
test gis_native_controlled_inference_executes_literal_progress_cancel_and_deadline_trace ... ok
test gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution ... ok
test gis_guest_genesis_is_byte_for_byte_the_linked_codec_pair_on_one_generation ... ok
test result: ok. 3 passed; 0 failed
```
Capture: `🗑️generated/tc3b-gis-oracle-law.txt` (exit 0).

Two more laws were rewired onto the new producer and RUN green:

* `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🧪️tests/…` — zero history at a server-minted id, every hostile
  id in `🌱️artifact-document-id-v1` refused. `2 passed` (`🗑️generated/tc3b-vcs-genesis-law.txt`).
  The old "a substituted dialect is refused" case is **deleted as structurally impossible**, not
  dropped: the new producer stamps `A::DIALECT` itself and takes no dialect argument.
* `owned_core_exports_are_defined_once_and_invoked_by_both_owners`
  (`🖨️describe/🧪️tests/🔬️unit/🦀️.rs`) now asserts **thirteen** exports, that they are defined in ONE
  shared macro body, and that `plugin_exports!` and both `extension_exports!` arms reach exactly
  `OwnedSemioExport::ALL`. `1 passed` (`🗑️generated/tc3b-owned-abi-law.txt`).

## 4. Hub changes, and the bootstrap/rebuild cost

**Wasm-hosted codec.** `verify_selected` now compiles each verified package's component once
(`OwnedRuntime::compile_component`, keyed by the already-hash-verified component bytes) and every
`VerifiedNativeArtifactCodec` carries a `GuestArtifactCodecBinding`. `codec: Option<ArtifactCodec>`
is `Some` only when this binary links one (stdio import/export and GIS inference stay native, TC2
§9); `validate_pair`/`apply_operation` take the native path when it exists and the guest path
otherwise, and `initial_pair` is **always** the guest's. The hub re-validates the guest's genesis
itself — `os_spr::decode_history` must show the requested `doc_id` and zero edits, transitions and
conflicts — before the existing `validate_pair(Input)`/`(Output)` fences run.

**Hash pinning without a new bundle field.** For a package with no linked provider the carried
`packSchemaHash` is checked against `codec.pack-schema-hash` asked of the component itself; for a
linked package the native codec's own hash still pins it. `NativeCodecProviderSetV1::preview` now
returns an empty binding vector for a package outside its compiled table instead of erroring —
refusing there is precisely what made a fourth package structurally unloadable.

**Open-target set.** `TrustedBundleProfileV1.open_target` → `open_targets: Vec<…>`;
`verify_selected`'s `open_targets.len() != 1` fence becomes "as many as the profile declares, and at
least one"; `trusted_profile_generation` frames the sorted set behind its own count in place of the
hard-coded `1u32`. For a one-target profile the framed bytes are **identical** to the old encoding,
so the existing stdio+gis generation id does not rotate on this change alone — only the profile's
JSON shape does. `selected_document_open()` deliberately still answers only when the generation
carries exactly one target, so `💡️inference/📇️catalog` keeps its unambiguous behaviour.

**Deletions (step 4).** `NativeCodecBinding::{genesis, with_genesis, has_genesis}`,
`VerifiedNativeArtifactCodec::genesis`, the `into_codec_and_genesis`/`genesis_factory` pair in both
`✏️s/🔌️plugins/🌍️gis/📇️native-codecs` and `…/🌿️vcs/📇️native-codecs`, and
`artifact_creation_selection`'s `codec.genesis.is_some()` predicate (every codec now has creation
authority, so the predicate collapses to identity matching).

**Bootstrap wall time: not measured — no bootstrap was run.** The `world actor` + owned-ABI change
invalidates every already-built component, so a bootstrap from this tree needs its components
rebuilt cold first. The wasm build mutex was held by **pz1 continuously from 10:32 to at least
11:19** with s10 queued behind it, and the machine sat at load 33–55 with 6–25 concurrent cargos for
the whole slice. A ~58-component cold sweep under that is hours, which preamble rule 17 forbids
spending here. **I am therefore handing the named list to the coordinator** (§6e).

## 5. 7651 proof — every http code

**None. No request was made and no hub was started**, because no component from this tree exists yet
(§4). `📜️tc3b-hub-boot.sh` is written and ready: port 7651, data root `.🧬semio/🌐hub/tc3b-boot`,
private `CARGO_TARGET_DIR=…/target-tc3b`, the whole bootstrap inside the wasm mutex,
`OS_HUB_CREDENTIAL_SIGN_IN=true` on the hold, and `OS_HUB_TRUSTED_PLUGINS` passed to the bootstrap
verb (which does not read it yet — §6c).

## 6. Honest gaps

**(a) The `cfg(wasm32-p2)` guest halves are compiled only natively.** The four `codec::Guest` methods
in `__semio_actor_exports!` and the four `semio_owned_*_v1` bodies in `__semio_owned_core_exports!`
are behind `#[cfg(all(target_arch = "wasm32", target_env = "p2"))]`, so no native check compiles
them (memory: *Native Cargo Misses Wasm-Gated Code*). The law in §3 proves the macro bodies DEFINE
the right symbols, and the four `plugin_runtime::plugin_artifact_*` functions they call are compiled
natively. What IS verified: the new WIT compiles through wasmtime's `bindgen!` **twice** — the
plugin host's single `mod actor_bindings` (`🖥️host/🦀️.rs`, in every green check from
`tc3b-check1.txt` on) and the describe crate's `#[cfg(test)] mod actor_bindings`, which the green
`owned-abi-law` run compiled. So `world actor` with `export codec;` parses, its world is valid, and
its types are representable. What is NOT verified is the **guest** trait shape that `wit_bindgen`
(not `wasmtime-wit-bindgen`) generates for the exported interface — method names, async-ness, the
`DocumentPair` record and the `(String, String)` tuple return. The two generators share their
kebab→snake and record-naming rules, so the risk is low, but it is real. A
`cargo check -p semio-s-plugin-note --target wasm32-wasip2 --features component-app-assembly`
through the mutex is the one remaining compile gate before any component build is attempted. I did
not run it: at 11:2x the mutex queue was `pz1` (holding 52 min) → `s10` → `c5` → `rb1`, four ahead
of me, which is hours — preamble rule 17 forbids spending the slice there and the brief forbids
waiting. See §6e.

**(b) The oracle is producer-vs-linked-codec, not guest-vs-native.** The brief asked for "a law that
runs both on the same generation and compares". The law in §3 runs the exact producer the guest
runs, and compares it byte-for-byte with the linked GIS codec, on the same generation and the same
document id — but in one process, not across the component boundary. Closing that last gap needs a
built gis component, i.e. §6e.

**(c) The N-plugin bootstrap list is NOT landed.** `materializeTrustedStdioGisBundle`
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts`) is welded to two packages far more deeply than TC1 §5's ten
rows: `projectTrustedBootstrapCodecsV1` hand-transcribes every GIS pack-record field spec in
TypeScript to recompute `packSchemaHash`, `trustedBootstrapProfileEncoding` is typed
`Record<"gis" | "stdio", …>`, and the fixture asserts `packageCount: 2, codecCount: 28`. The clean
way in is the `codec.pack-schema-hash` export this slice just added — the builder should ASK each
built component for its codec rows instead of transcribing them — but that is a substantial
TypeScript rewrite plus a new fixture, and it cannot be verified without component builds anyway. I
landed only what the schema change forced: the bundle writer now emits `openTargets: [ … ]`, the
rotation path reads it, and `🌎️hub/🧪️tests/🤝️integration/🟦️.ts` iterates the set.

**(d) Nothing is verified at runtime.** Per rule 21 the bar is observed-at-runtime and this slice
does not clear it. Everything in §1 marked *landed* is landed-and-type-checked, and the three laws
in §3 are landed-and-run; no hub, no socket, no http code.

**(e) What the coordinator has to run, in order.** (1)
`cargo check -p semio-s-plugin-note --target wasm32-wasip2 --features component-app-assembly`
through `📜️wasm-build-mutex.sh` — the guest-side compile gate (§6a); fix any binding-shape error
before anything else. (2) A cold rebuild of the components the 7651 proof needs FIRST — `stdio`,
`gis`, `note` — then `📜️tc3b-hub-boot.sh 7651`. (3) The full ~58-component sweep, which is
unavoidable now: every component in the tree is refused by the thirteen-export owned ABI until it is
rebuilt, so `dev s` and every plugin-hosting surface is dark until the sweep finishes. (4)
`cargo test -p semio-hub` (rule 26).

**(f) A pre-existing bug found in passing, not fixed.** `ArtifactCodec::apply_ops_binary` drops its
`ArtifactStore` without the terminal-empty shallow-shell witness: calling it with an
`encode_ops_vec(&[])` batch panics with *"artifact store reached Drop without its exact
terminal-empty shallow-shell witness"* (`🏪️store/🦀️.rs:18569`). It is on the hub's own
`apply_operation` path for every natively linked codec. I reshaped my own law around it rather than
change a store the whole fleet is standing on; it wants its own slice.

**(g) `nativeCodecs` keeps its wire name** although a row may now be backed by the component rather
than a linked codec. Renaming it to `artifactCodecs` would churn three JSON fixtures and the TS
builder that peers are editing right now, for no behavioural gain; the Rust docstrings say plainly
what the field means.

**(h) `semio-framework-plugin --all-targets` is RED — in a peer's file, not mine.** 23 errors, all
`dsl::LocalizedLabel` (`no method as_str`, `no method contains`, `LocalizedLabel: From<&str>`) in
`🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` and
`🧪️tests/🛰️declaration-channels-unit/🦀️.rs` — U3's localisation migration mid-flight
(`🗑️generated/tc3b-check6.txt`). The two errors my own change had caused there (a removed
`use std::pin::Pin;` that `artifact_app_laws` needs under `--all-targets` although a plain `cargo
check` reports it unused) are fixed, with an `#[allow(unused_imports)]` and a note saying why the
warning must not be believed. `cargo check -p semio-framework-plugin` (lib) is green.

## 7. Files changed

Source:
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit` (+46)
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (+556/−…): `owned_abi::{CodecInput,
  CodecPair, CodecMirror}`; four `PluginApp` methods; their `VcsArtifactApp` implementations;
  `app::artifact_app_genesis_pair` / `app::artifact_app_apply_ops` replacing
  `native_artifact_genesis_for_editor`; `plugin_runtime::plugin_artifact_{codec_app,
  pack_schema_hash,genesis,print_mirror,apply_ops}`; the `codec::Guest` impl in
  `__semio_actor_exports!`; four `semio_owned_*_v1` bodies in `__semio_owned_core_exports!`
* `…/🔌️plugin/🧠️interpreter/🦀️.rs`: `OwnedSemioExport::ALL` 9 → 13 + their names and core types
* `…/🔌️plugin/🖥️host/🦀️.rs`: `OwnedOperation` + `OwnedCodecInput`, `GuestDocumentPair`,
  `GuestDocumentMirror`, and `OwnedRuntime::{codec_call, codec_pack_schema_hash, codec_genesis,
  codec_print_mirror, codec_apply_ops}`
* `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` (+283/−…) and `…/🧬️schema/🦀️.rs`,
  `…/🧬️schema/🔣️.json`, `…/🧫️fixtures/👥️two-package/🔣️.json`
* `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs`
* `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (bundle writer + rotation read `openTargets`) — both this
  file and `🌎️hub/🧪️tests/🤝️integration/🟦️.ts` parse clean under `bun build --target node`
* `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs`, `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🦀️.rs`

Laws/tests:
* `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧪️tests/📇️native-codecs/🦀️.rs` (+ the oracle law)
* `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🧪️tests/📇️native-codecs/🦀️.rs`
* `🧰️framework/…/🔌️plugin/🖨️describe/🧪️tests/🔬️unit/🦀️.rs`
* `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs`,
  `…/📇️native-openable-provider/🧪️tests/🔬️unit/🦀️.rs`, `🌎️hub/🧪️tests/🔏️trusted-catalog-profile/🦀️.rs`,
  `🌎️hub/🧪️tests/🤝️integration/🟦️.ts`

Ticket-owned: this report, `📜️tc3b-hub-boot.sh`, and `🗑️generated/tc3b-*.txt`
(`check1`, `check2`, `check3`, `check4`, `check5`, `check6`, `vcs-genesis-law`, `gis-genesis-law`,
`gis-oracle-law`, `owned-abi-law`).

`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` and `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/
🌐️browser-actor/🔣️.json` also show in `git diff --stat` for this tree: those are **peers'** edits,
not mine.
