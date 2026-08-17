# 📓️ Recipe — migrating one subset onto the new declaration tree

Written by the W2-P pilot agent (`💾️binary`/`📄txt`, the two stdio carrier artifacts), the FIRST
real subset migration on this ticket. Every code excerpt below is from the real pilot, not
invented. Read `📓️design.md` first — this recipe assumes it.

## 0. Before you touch anything

1. Read the executable spec: `crate::app::declarations::fixture` in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (~line 14145). It is the
   reference shape for `build_declaration()` — copy its wiring pattern, not its literal types.
2. Check whether your target artifact's files changed since the ticket start commit
   (`git log --date=iso -1 -- <path>`, compare against `📓️status.md`'s recorded start commit). If
   a peer session touched them TODAY, stop and report `blocked-peer` with the commit proof — do
   not fight a moving target.
3. **Run the crate's baseline BEFORE editing anything, and wait for it to finish before your first
   edit.** Do not start a background baseline run and then immediately start editing — the
   baseline becomes contaminated (this pilot made this mistake; recovered only because the
   resulting errors were all provably outside its own files, confirmed by path-grepping the
   output — do not rely on getting that lucky).
4. **Check whether your crate currently builds at all**, independent of your own artifact:
   `cargo check -p <crate> --all-targets`. Stdio's crate does NOT build clean today — 433
   pre-existing errors at this pilot's start, all in artifacts nobody on this ticket owns
   (`🧿️semio`, `🖊️dwg`, `🎞️gif`, `🧊️gltf`, `🎥️mp4` — a peer session's in-progress rewrite). This is
   real and matches `📌️important.md`'s own warning. If this is your situation too:
   - `cargo check -p <crate> --lib` (no `--tests`, no `--all-targets`) is a MUCH narrower target
     and may compile clean even when `--all-targets` doesn't — the broken files are often only
     reached through `#[cfg(test)]` code or separate integration-test crates. Use `--lib` as your
     fast iteration loop.
   - Grep the `--all-targets` error output for YOUR artifact's own path fragment
     (`grep "🗄️stdio.*💾️binary\|📄txt"` style) to prove your changes add zero NEW errors, even
     though you cannot get the whole crate green. Report the total error count before/after (it
     may even improve between your before/after snapshots if a peer session is fixing things
     concurrently — this happened here, 433 → 268, entirely not this pilot's doing) alongside the
     explicit zero-attributable-to-me grep proof.

## 1. The hard wall: your artifact may not be free to leave its OLD registration channel

**This is the single most important discovery of this pilot.** Before assuming you can delete the
old `derive_artifact_facets!`/`ArtifactComposition`/`register()` cluster (mission step 6), check
whether your artifact is enumerated through a RIGID, count-checked catalog outside your boundary.
For stdio, that catalog is `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️component.rs`:

```rust
const SOURCES: [&str; 36] = [ /* one include_str! per artifact's 📜️artifact-definition.json */ ];
fn artifact_factories() -> BTreeMap<&'static str, fn(ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError>> {
    BTreeMap::from([("binary", crate::artifacts::binary::assembly as fn(...)), ("txt", ...), /* all 36 */])
}
```

cross-checked by a test (`schema_keys_and_runtime_factories_are_exact`) that the map's keys exactly
match the 36 `Source.artifact` values, and `validate_catalog` hard-asserts `values.len() != 36`.
**Your artifact's `assembly()` function (old channel) is referenced by name from this map — it is
a real, external, outside-your-boundary dependency, not dead code, even if nothing else about the
old machinery is live.** Grep before deleting:

```
grep -rn "artifacts::<name>::assembly\b" --include="*.rs" .        # MUST still exist somewhere outside your dir if you keep the old catalog
grep -rn "artifacts::<name>::artifact_kind\b" --include="*.rs" .   # check separately — may be genuinely dead
```

If your target crate has an equivalent rigid catalog, you have two choices, same as this pilot's
`🔧️patches/w2p-stdio-plugin-root.txt`:
- **Option A (do this first, always):** keep `assembly()` unchanged, ADD
  `builder.declare_artifact(your_artifact())` alongside the old loop in the plugin root. Both
  channels coexist; the old one is `ArtifactAssembly::Definition` (definition-only, registers no
  schema/io rows), so there is nothing to conflict with the new tree's own registration.
- **Option B (full cutover):** requires editing the catalog file itself (shrink the count, drop
  your artifact's `SOURCES`/`artifact_factories()` entries) — bigger, needs the WHOLE crate
  building clean to verify safely. Don't attempt this if your crate doesn't currently build (see
  §0.4).

**Either way, this catalog file is very likely NOT on any shard's boundary list — write a patch,
do not touch it directly**, exactly like the plugin root itself.

## 2. Grep for genuinely dead code before assuming anything is safe to delete

Comments in this codebase drift (they describe intent, not always current reality — the
`derive_artifact_facets!`-generated `register()`/imperative-plugin-root-call comments in this
pilot's own files claimed a call site in the plugin root that no longer exists). Trust the
compiler and grep, not doc comments:

```
grep -rn "artifacts::<name>::engine::register\|artifacts::<name>::io::register" --include="*.rs" .
grep -rn "<name>::<GeneratedBuilder>\b\|<name>::<GeneratedAnalyzer>\b\|<name>::<GeneratedComposer>\b" --include="*.rs" . | grep -v "your own artifact dir"
```

For this pilot: `register()`, `register_pilot_languages()`, `register_schema_specs()`,
`register_artifact_schema()`, `register_artifact_inferences()`, `io_registry`, and the
`derive_artifact_facets!`-generated `Builder`/`Analyzer`/`Composer` types were ALL confirmed
genuinely dead (zero call sites repo-wide) — except `BinaryBuilder`/`TxtBuilder`, still referenced
by their own `impl ArtifactInferrer for …Builder` one directory over. If your artifact's
`ArtifactInferrer` impl targets the generated builder type, retarget it onto
`semio_framework_plugin::app::SnapshotBuilder<Snapshot, Mutation>` (W1-C's generic replacement)
when you delete the macro, not before.

## 3. The carrier law (if your artifact is one of the two carriers — most won't be)

Only `s.stdio.binary@raw/*` and `s.stdio.txt@utf-8/*` are carriers. If yours isn't, skip to §4.
Look for a `SemioEnvelope`/`wrap_binary`/`wrap_text`/`unwrap_binary`/`split_text_preamble` call
inside the snapshot type's `ArtifactPack`/`ArtifactDsl` impl — that is the bug. Real before/after:

```rust
// BEFORE (💾️binary's ArtifactPack — wraps a `.semio` pack container around the file's own bytes):
fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
    let envelope = store::semio_format::SemioEnvelope::from_envelope_id(..., Component::Pack, 1)?;
    Ok(store::semio_format::wrap_binary(&envelope, &self.bytes))
}
// AFTER — the identity function on `bytes`, nothing else:
fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
    let _ = options;
    Ok(self.bytes.clone())
}
```

Same shape for `📄txt`'s `ArtifactDsl::parse_dsl`/`print_dsl` (raw text, no `wrap_text` preamble).
**Only the ONE payload variant the carrier constant actually governs needs fixing** —
`CARRIER_BINARY` only constrains the `Binary` `IoPayload` of `s.stdio.binary@raw/*`; that
artifact's OTHER native form (its hex-text `ArtifactDsl`) is a legitimate, separate, non-carrier
encoding and does NOT need to change. Same the other way for `📄txt`: only `ArtifactDsl`
(`Text`) is law-bound; its `ArtifactPack` (binary/pack form) stays wrapped, untouched.

**Do NOT touch the test, touch the codec** — but you WILL need to update every test that baked in
the old wrapped shape as an assumption:
- Regenerate committed fixture assets (`🎒️example.pack.semio`, `🗣️example.dsl.semio`) to the new
  raw bytes/text — for this pilot, `🎒️example.pack.semio` became literally `hello` (5 bytes, no
  header) and `🗣️example.dsl.semio` became byte-identical to the already-existing raw
  `📄example.txt` fixture (the carrier fixture the artifact ALREADY shipped for exactly this
  purpose — check whether yours has one too before hand-authoring bytes).
- Any test that manually unwraps the envelope before comparing (`unwrap_binary(&packed)`) no
  longer needs to — the payload is already unwrapped. Any test that reconstructs a "preamble +
  body" string from `split_text_preamble` for a grammar-conformance check needs to build that
  string directly instead (the grammar's synthetic `"<envelope-id>\n<body>"` convention is a
  TESTING convention, unrelated to the real, now-raw codec — see `grammar_conformance_law` in
  `📄txt/…/🧬️schema/🦀️component.rs` for the real fix).
- `fixture_honesty_law`-style tests that just compare codec output to the (now regenerated)
  fixture usually need NO code change — only the fixture bytes/text change.

**Write the law test** (`carrier_native_is_raw`) proving round-trip + non-pack-container, e.g.:

```rust
#[test]
fn carrier_native_is_raw() {
    for bytes in [Vec::<u8>::new(), vec![0x00, 0x01, 0xFF], b"hello".to_vec(), (0u8..=255).collect::<Vec<u8>>()] {
        let decoded = BinarySnapshot::decode_pack(&bytes).expect("decode");
        let encoded = decoded.encode_pack();
        assert_eq!(encoded, bytes, "carrier round trip must be byte-identical for {bytes:?}");
        assert!(!encoded.starts_with(&store::semio_format::BINARY_MAGIC), "carrier payload must not be a pack container: {encoded:?}");
    }
}
```

`store::semio_format::BINARY_MAGIC` (`[0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A]`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`) is the exact magic
`ArtifactPack::encode_pack` used to emit before this fix — assert against it explicitly, don't
just eyeball the bytes.

**A carrier artifact needs ZERO foreign `IoEntry` rows.** `io_route`'s registry entries are keyed
`(from, into)`; opening a raw file is `io_identify(bytes)` then `io_route(carrier → target)` —
that hop is registered on the TARGET artifact's side (`Deserializer<TargetSnapshot>` with
`FROM: CARRIER_BINARY`), never on the carrier's own side, because the carrier dialect and the
carrier constant are the SAME coordinate (`CARRIER_BINARY = "s.stdio.binary@raw/*"` IS
`s.stdio.binary`'s own dialect). This also means the old self-referential identity
`ArtifactDeserializer`/`ArtifactSerializer` leaves some stdio artifacts ship at
`🚪️io/📥️import/…/🗿️artifacts/<own-kind>/<own-standard>/<own-subset>/` (importing/exporting the
artifact to/from ITSELF) are provably redundant under the new mechanism for a carrier artifact —
confirmed present in both `💾️binary` and `📄txt` this pass, left in place only because deleting
them needs a clean-building crate to verify against (see §0.4). If your artifact is NOT a carrier,
its own genuine cross-dialect foreign leaves are NOT redundant — only the self-identity case is.

## 4. The declaration tree files — real shapes from this pilot

### 4a. Subset root (`🪆️subsets/✳️<x>/🦀️component.rs`)

```rust
use crate::artifacts::binary::standards::v_raw::subsets::any::{io, schema};
use crate::editor::binary as editor;   // ⚠️ top-level `crate::editor::<artifact>`, NOT
use crate::viewer::binary as viewer;   //    `crate::artifacts::<artifact>::…::editor` — see §5.
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::{Dialect, ExampleSource, StandardId, SubsetId};
use std::sync::OnceLock;

pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::binary::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::binary_artifact_inference_descriptor()]).as_slice()
}

pub fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: DIALECT,
        schema: SchemaDeclaration { descriptor: schema::binary_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::BinaryViewer>(viewer::create_binary_viewer()),
        editor: editor_surface::<editor::BinaryEditor>(editor::create_binary_editor()),
        examples: examples(),
    }
}
```

`&'static [T]` fields (`inferences`, `examples`) that don't have an existing `'static` source need
a `OnceLock<Vec<T>>` — there is no way to build a `&'static` slice from an owned-value-returning
function otherwise. This is boilerplate you'll repeat per subset; do not fight it.

### 4b. Standard root (`🏅️standards/🔖️<s>/🦀️component.rs`) — NEW file, did not exist before

```rust
use crate::artifacts::binary::standards::v_raw::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard() -> StandardDeclaration {
    StandardDeclaration {
        id: StandardId("raw"),
        media: MediaDeclaration { mimes: &["application/octet-stream"], extensions: &["bin"] },
        subsets: vec![subsets::any::subset()],
    }
}
```

**Take mimes/extensions from the REAL registration, don't invent them.** For stdio, the source of
truth is each artifact's `🧬️schema/📜️artifact-definition.json` → `representations[*].{mimes,
extensions}` (read by `📇️registry`'s `source_format_descriptors`) — NOT the old
`artifact_kind()`'s `MediaType`/`OsMediaCapability` fields (those are a coarser, unrelated
classification). Verify with a one-liner:
`python3 -c "import json; print(json.load(open('.../📜️artifact-definition.json'))['representations'])"`.

### 4c. Artifact root — ADD to the EXISTING file, do not replace it

```rust
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;   // ⚠️ NOT `semio_framework_plugin::ArtifactKindId` — see §5.
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.stdio.binary").expect("canonical stdio.binary kind"), localization: &[], standards: vec![crate::artifacts::binary::standards::v_raw::standard()] }
}
```

`localization: &[]` is a documented shortfall, not an oversight: the real en/de localized
descriptors live in the OLD `📜️artifact-definition.json` channel (kept — see §1); wiring them into
this field too is real follow-up work, not required for the tree to register or for any law to
hold.

### 4d. `io()` — for a carrier, this is nearly boilerplate

```rust
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::binary::{BinaryMutation, BinarySnapshot};   // ⚠️ artifact-level re-export path — see §5.
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::of::<BinarySnapshot, BinaryMutation>(crate::artifacts::binary::STDIO_BINARY_DOCUMENT_SCHEMA.to_string()),
        },
        entries: &[],   // carrier law: nothing to route from/to a carrier's own side, see §3
    }
}
```

`LanguagePair { text: None, binary: None }` for every facet is a REAL scope-narrowing this pilot
made deliberately, not an oversight: wiring real `&'static dsl::LanguageSpec` values into these
slots (so DSL tooling/grammar registration flows through the new tree too) is legal to defer — the
type's own doc calls plain-codec `LanguagePair{None,None}` subsets a supported shape — and the
underlying `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` codecs these WOULD point at are
unchanged, independently implemented, and independently tested either way. Flag it, don't silently
drop it, and don't block your own migration on solving it.

## 5. Gotchas that will cost you a debug cycle if you don't know them up front

1. **`editor`/`viewer` are NOT nested under `artifacts::<name>::…` in `📦️glue.rs`.** They are
   mounted at the TOP level as sibling `pub mod editor { pub mod <name> { … } }` / `pub mod viewer
   { … }` blocks. From inside your new subset-root file, reach them as `crate::editor::<name>`/
   `crate::viewer::<name>`, not `crate::artifacts::<name>::…::editor`. This pilot got it wrong on
   the first pass (a plausible-looking path that simply doesn't exist) — verify with
   `grep -n "pub mod editor\b\|pub mod viewer\b" 📦️glue.rs` before writing the import.
2. **`ArtifactKindId::parse` is `store::os_io::ArtifactKindId`, not
   `semio_framework_plugin::ArtifactKindId`.** The plugin crate itself reaches it via
   `use store::os_io::{ArtifactKindId, ArtifactRef};` (its own `component.rs`, ~line 348) — it is
   NOT re-exported at the plugin crate's own root. `store` here is the `semio_framework_os_kernel`
   crate alias every stdio file already has (`store::ArtifactPack`, `store::PackError`, etc. —
   same alias, already in scope everywhere).
3. **Your snapshot/mutation types live under `schema::snapshot::`/`schema::mutations::`
   submodules, not directly on `schema::`.** The artifact root's own `pub use
   crate::artifacts::<name>::schema::snapshot::<Name>Snapshot;` (and the mutations/diff twins) is
   what makes `crate::artifacts::<name>::<Name>Snapshot` resolve — that is the path to import from
   elsewhere, NOT `…::subsets::any::schema::<Name>Snapshot` (which doesn't re-export the leaf
   types at that level).
4. **The plugin-crate's own `ArtifactDeclaration` (old, debt D1) and the new
   `app::declarations::ArtifactDeclaration` are two DIFFERENT types with the SAME bare name in the
   SAME crate** (W1-C's own documented deviation). If a file already imports the old one bare
   (most stdio artifact roots do, via `ArtifactKindSpec`-adjacent imports — check first), import
   the new one fully qualified (`semio_framework_plugin::app::declarations::ArtifactDeclaration`)
   rather than adding a second bare `use`, which is an immediate `E0659` ambiguity error.
5. **`&'static dsl::LanguageSpec`/`&'static [ArtifactInferenceDescriptor]`/`&'static
   [ExampleSource]` fields need a `OnceLock`, not a `const fn`** — the source functions
   (`*_inference_descriptor()`, `ExampleSource::new(...)`) are not `const`-callable. Don't spend
   time trying to make them one; the `OnceLock` pattern above is the accepted shape (mirrors how
   the fixture itself does it for its own `std1_strict_entries()`).
6. **Check for a rigid, count-checked artifact catalog outside your boundary BEFORE planning to
   delete old registration machinery** — see §1. This is the single biggest time sink risk in this
   recipe; check it in your first 30 minutes, not your last.
7. **If your crate doesn't currently build clean (`cargo check -p <crate> --all-targets`), do not
   attempt "job step 6" style deletions this pass** — you cannot safely verify them. Do the
   carrier-law-equivalent fix (if applicable) and the additive declaration-tree build (§4), which
   are both independently verifiable via `--lib` even when `--all-targets` is red elsewhere, and
   report the blocker plainly rather than guessing.

## 6. Verification command block (copy-paste, adjust crate/artifact names)

```bash
cd /Users/ueli/Documents/semio
TICKET=".🧬semio/🦑️repo/🎫️tickets/<your-ticket-path>"
CARGO_TARGET_DIR="$PWD/$TICKET/🎯️target" cargo check -p <crate> --lib                       # fast loop
CARGO_TARGET_DIR="$PWD/$TICKET/🎯️target" cargo check -p <crate> --all-targets                # full picture (may be pre-broken — see §0.4)
CARGO_TARGET_DIR="$PWD/$TICKET/🎯️target" cargo nextest run -p <crate> --lib --no-fail-fast    # numbers for the report
CARGO_TARGET_DIR="$PWD/$TICKET/🎯️target" cargo check -p <crate> --target wasm32-wasip2 --lib  # W1-C's ~7min cold-build caution applies
bun ./📜️script.ts policy                                                                       # per-policy report-mode breach counts
bun ./📜️script.ts verify taxonomy enforce                                                      # only meaningful if you touched taxonomy-relevant paths
```

## 7. Do-not-do-this list (mistakes made and undone this pass)

- Do NOT start a background "baseline before I touch anything" cargo run and then immediately
  start editing files in the same turn — the baseline is contaminated the moment your first Edit
  lands. Wait for it (or accept the honest caveat and prove your files aren't implicated by
  grepping the error output for your own paths, as this pilot had to do after the mistake).
- Do NOT assume `register()`/`io_registry`/generated `Builder`/`Analyzer`/`Composer` types are
  dead just because a comment says the plugin root "reaches" them — comments drift; grep the
  actual call site.
- Do NOT import the new `app::declarations::ArtifactDeclaration` bare if the file already has a
  bare `use` of the old `app::ArtifactDeclaration` in scope (or vice versa) — always qualify one
  of them.
- Do NOT try to delete a carrier artifact's self-referential identity `ArtifactSerializer`/
  `ArtifactDeserializer` leaf without first confirming (§3) it really is self-referential
  (`FROM == INTO == this artifact's own dialect`) — a genuine cross-dialect foreign leaf on a
  carrier artifact (rare, but possible if a carrier ever gains a second standard) is NOT
  redundant.
- Do NOT physically relocate `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}/{📝️text,
  💾️binary}` into `🚪️io/…` for every facet in one pass just because design.md's tree diagram shows
  the end state — a single trait (`ArtifactDsl`/`ArtifactPack`) can only be implemented ONCE for a
  type, so "one copy per direction" is not literally possible for the CODE (only the duplicated
  spec ASSET file is exact-mirror-able); pick one direction to own the real impl (this pilot did
  not attempt the move at all for this reason, plus the wide test blast radius one relocation was
  observed to have — see `📓️w2-p-report.md` `## openQuestions` for the concrete numbers). Whoever
  does this for real should scope it artifact-by-artifact, not blanket, and budget for updating
  every `grammar_conformance_law`/`protocol_walk_law`/`fixture_honesty_law`-style test that
  references the moved facet's old path.
