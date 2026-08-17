# 📓️ Design contract — Clean Artifact → Standard → Subset Mechanism

Binding for every agent. Deviations must be reported, not improvised.

## 0. The four rules this ticket exists to make true

1. **Hierarchy is literal**: artifact → standard → subset, each level one root `🦀️component.rs` that mounts its
   own children and exports one declaration function.
2. **Every subset is a complete standalone implementation**: own snapshot/diff/mutations/inferences types, own
   io, own viewer, own editor, own examples. A subset never `use`s a sibling subset or another standard.
3. **All IO goes exclusively over the io system** as serializers and deserializers — *including* the native DSL
   text grammar and pack binary protocol.
4. **All shared code lives in `🔨️modules`** at the lowest common owner (standard → artifact → plugin → `✏️s/🔨️modules`
   → framework). ≥2 consumers or it is inlined. No subset-level modules.

## 1. Tree shape (taxonomy v6)

```
✏️s/🔌️plugins/<p>/
  🦀️component.rs                         plugin() -> Plugin
  📦️packages/🦀️rust/📦️glue.rs           prelude + mounts ONLY artifact roots, plugin modules, commands, plugin root
  📦️packages/🟦️typescript/📦️index.ts    mirrors the same tree
  🔨️modules/<m>/                          plugin-level shared code (≥2 artifacts consume)
  🎮️commands/
  🗿️artifacts/<a>/
    🦀️component.rs                       artifact() -> ArtifactDeclaration; mounts standards + artifact modules
    🔨️modules/<m>/                        artifact-level shared code (≥2 standards or ≥2 subsets consume)
    🏅️standards/🔖️<s>/
      🦀️component.rs                     standard() -> StandardDeclaration; mounts subsets + standard modules
      🔨️modules/<m>/                      standard-level shared code (≥2 subsets consume)
      🪆️subsets/🔣️component.json          subset vocabulary manifest (unchanged, must match disk)
      🪆️subsets/✳️<x>/
        🦀️component.rs  🟦️component.ts   subset() -> SubsetDeclaration; mounts schema/io/viewer/editor/examples
        🧬️schema/                         TYPES + pure transforms ONLY (no codecs)
          🦀️component.rs 🟦️component.ts 🔣️component.json 🛰️component.proto 🔗️component.graphql
          📸️snapshot/ 🔺️diff/ 🧬️mutations/<verb-noun>/{🦠️mutation,🔺️diff,↩️inverse} 💡️inferences/<name>/
        🚪️io/                             ALL codecs
          🦀️component.rs 🟦️component.ts   io() -> IoDeclaration  /  IoEntryDescriptor[] mirror
          📸️snapshot/{📝️text,💾️binary}     native codec, BOTH directions + 📖️component.grammar.semio / 📡️component.protocol.semio
          🔺️diff/{📝️text,💾️binary}
          🧬️mutations/{📝️text,💾️binary}
          💡️inferences/{📝️text,💾️binary}
          📥️import/🧩️deserializers/🗿️artifacts/<kind>/🔖️<std>/✳️<sub>/   foreign-dialect Deserializer leaves
          📤️export/🧵️serializers/🗿️artifacts/<kind>/🔖️<std>/✳️<sub>/     foreign-dialect Serializer leaves
        👁️viewer/  ✏️editor/                unchanged surface shape (🎭️modes/🪟️windows/…)
        📚️examples/<name>/{🖼️assets,🧪️tests}
```

Removed shapes (W6 forbids them): `🧬️schema/*/{📝️text,💾️binary}` (moved to io), `🧬️migrations`, artifact-level
`🧬️schema`/`📚️examples`/`📦️opc`/`🎬️interaction-spec`, plugin-level `⚙️engine`/`🔄️fsm`/`📇️registry`/`🎟️capabilities`/`🔧️setup`/`🛂️manifest`,
subset-level `🔨️modules`, any `🚪️io` child other than `📥️import`/`📤️export`/the four native facet dirs + the two
component leaves, `✏️editor/⚙️engine` stays legal (app engine, owned by ticket #2553).

> ### ⚠️ CORRECTION (2026-08-17, forced by the W2-P pilot — supersedes the original import/export mirror)
> The original tree put the native codec under BOTH `📥️import/🧩️deserializers/📸️snapshot/📝️text` and
> `📤️export/🧵️serializers/📸️snapshot/📝️text`. **That is not implementable.** `ArtifactDsl` (parse + print) and
> `ArtifactPack` (decode + encode) are single, inherently bidirectional traits, and Rust allows exactly one impl of a
> trait per type — so an "exact mirror" would force either duplicated codec logic (two sources of truth, the precise
> thing this ticket exists to remove) or one side being a hollow re-export.
>
> **Corrected rule:** the import/export split expresses *direction*, and direction only exists for **foreign**
> dialects, where each way genuinely is a separate function (`Serializer::serialize` vs `Deserializer::deserialize`).
> The **native** codec is one bidirectional thing and therefore sits directly under `🚪️io/<facet>/<representation>/`,
> unsplit. User decision #2 is unchanged — `🚪️io` is still where every byte crossing lives; only its internal shape
> changes. The taxonomy vocabulary patch (`🔧️patches/w1b-discovery-io-native-codec-vocabulary.txt`) must be
> re-cut against this corrected shape before W2 proper.

### Module path slugs (policy `policyOwnerMountsChildrenBreaches`)

Strip the emoji prefix, kebab→snake, leading digit gets `_`:
`🔖️1`→`v1`, `🔖️1.4`→`v1_4`, `🔖️89a`→`v89a`, `🔖️ap214`→`v_ap214`, `🔖️ecma-376`→`v_ecma_376`, `🔖️riff-pcm`→`v_riff_pcm`;
`✳️any`→`any`, `✳️i-json`→`i_json`, `✳️cc6`→`cc6`; `📄txt`→`txt`, `◻2d`→`_2d`, `🧊️3d`→`_3d`, `🖐️5d`→`_5d`.
Canonical paths: `crate::artifacts::<a>::standards::<s>::subsets::<x>::{schema,io,viewer,editor,examples}`,
`crate::modules::<m>`, `crate::artifacts::<a>::modules::<m>`, `crate::artifacts::<a>::standards::<s>::modules::<m>`.
**Owner mounts children; nobody else mounts anything. No `pub use` shims of sibling paths.**

## 2. SDK declarations (framework `🔌️plugin`)

```rust
pub struct ArtifactDeclaration { pub kind: ArtifactKindId, pub localization: &'static [(ArtifactLocale, &'static str)], pub standards: Vec<StandardDeclaration> }
pub struct StandardDeclaration  { pub id: StandardId, pub media: MediaDeclaration, pub subsets: Vec<SubsetDeclaration> }
pub struct MediaDeclaration     { pub mimes: &'static [&'static str], pub extensions: &'static [&'static str] }
pub struct SubsetDeclaration    { pub dialect: Dialect, pub schema: SchemaDeclaration, pub io: IoDeclaration,
                                  pub viewer: SurfaceDeclaration, pub editor: SurfaceDeclaration, pub examples: &'static [ExampleSource] }
pub struct SchemaDeclaration    { pub descriptor: ArtifactSchemaDescriptor, pub inferences: &'static [ArtifactInferenceDescriptor], pub inference_services: Vec<ArtifactInferenceService> }
pub struct IoDeclaration        { pub native: NativeCodecs, pub conformance: Option<fn(&IoPayload) -> Vec<Diagnostic>>, pub entries: &'static [IoEntry] }
pub struct NativeCodecs         { pub snapshot: LanguagePair, pub diff: LanguagePair, pub mutations: LanguagePair, pub inferences: Option<LanguagePair>, pub codec: store::ArtifactCodec }
pub struct SurfaceDeclaration   { pub definition: AppDefinition, pub factory: fn() -> Box<dyn PluginApp>,
                                  pub app_schema: fn() -> Option<AppSchemaDescriptor>,
                                  pub mutation_roster: Option<fn() -> (&'static str, &'static [SemanticDescriptor])>, pub rights: Rights }
pub fn editor_surface<E: ArtifactEditor>(def: AppDefinition) -> SurfaceDeclaration;
pub fn viewer_surface<V: ArtifactViewer>(def: AppDefinition) -> SurfaceDeclaration;
impl PluginBuilder<Ready> { pub fn artifact(self, a: ArtifactDeclaration) -> Self; }   // ONLY registration channel
```

`try_build` walks artifact→standard→subset and registers schema descriptor + inferences, the document codec,
io entries, viewer/editor (ids from `surface_app_id(&dialect, role)`), examples, media.

Kept: `ArtifactEditor`/`ArtifactViewer`/`ViewEmit`, `AppRouter`/`OpeningResolver`, `ArtifactInferrer`,
`SemanticMutation`/`#[derive(Mutations)]`, composition (`ArtifactRef`/`ArtifactChild`/`ArtifactLink`/coordinator),
`ArtifactBuilder` shrunk to `empty/from_snapshot/mutate/absorb/build` (+ new generic `SnapshotBuilder<S>`).

Deleted in W6: `ArtifactDefinition`+capability rows, `ArtifactKindSpec`/`OsMediaCapability`/`MediaType`/`ArtifactIo`/`AppIo`/`ArtifactPresentation`,
`.editor()/.viewer()/.document_app()/.artifact_kind()/.artifact_definition()`, `derive_artifact_facets!`, `subset!`, `SubsetKind`,
`DerivedArtifactSpec`/`DerivedArtifactBuilder`/`DerivedArtifactAnalyzer`/`DerivedArtifactComposer`,
`ArtifactAnalysis`/`ArtifactAnalyzer`/`ArtifactComposition`/`ArtifactComposer`/`ArtifactDecomposer`/`ArtifactChildren`/`NoChildren`,
`HostMediaHandlerDeclaration`, `ArtifactBuilder::from_text/from_binary`, stdio `📇️registry`.

## 3. IO system (framework `🚪️io`)

Split the file that is compiled twice today (`semio_framework::io` **and** `semio_framework_os_kernel::os_io`):
- `🚪️io/🧬️schema/🦀️component.rs` — vocabulary, mounted ONCE (os-kernel), re-exported by `semio_framework`:
  `StandardId, SubsetId, Dialect, ArtifactDialect, ArtifactKindId, ArtifactRef, IoPayload{Text,Binary},
   Confidence, IoFidelity{Exact,Canonical,Semantic,Lossy}, IoError, IoResult, IoEntryDescriptor, IoRoute` (ts-rs typegen).
- `🚪️io/🦀️component.rs` — registry, mounted ONCE (`semio_framework`):

```rust
pub trait Serializer<S>   { const INTO: Dialect; const FIDELITY: IoFidelity; fn serialize(from: &S) -> IoResult<IoPayload>; }
pub trait Deserializer<S> { const FROM: Dialect; const FIDELITY: IoFidelity;
                            fn sniff(_p: &IoPayload) -> Confidence { Confidence::None }
                            fn deserialize(p: &IoPayload) -> IoResult<S>; }
pub struct IoEntry { pub from: Dialect, pub into: Dialect, pub fidelity: IoFidelity,
                     pub sniff: Option<fn(&IoPayload) -> Confidence>, pub run: fn(&IoPayload) -> IoResult<IoPayload> }
pub fn serializer_entry<S, T: Serializer<S>>(own: Dialect) -> IoEntry;      // from = own
pub fn deserializer_entry<S, T: Deserializer<S>>(own: Dialect, conformance: Option<fn(&S)->Vec<Diagnostic>>) -> IoEntry; // into = own
pub fn io_register(entries: &'static [IoEntry]) -> Result<(), IoRegistryError>;
pub fn io_route(from: &ArtifactDialect, into: &ArtifactDialect, max_hops: u8 /* ≤3 */) -> IoResult<IoRoute>;
pub fn io_run(route: &IoRoute, payload: IoPayload) -> IoResult<IoPayload>;
pub fn io_identify(payload: &IoPayload) -> Vec<(ArtifactDialect, Confidence)>;
pub fn io_entries() -> Vec<IoEntryDescriptor>;
```

**Payload law.** The `IoPayload` of dialect D is D's *native* encoding: `Binary` = its pack, `Text` = its DSL.
The only exceptions are the two **carrier dialects**, whose native encoding IS the raw external content:
- `s.stdio.binary@raw/*` — raw file bytes
- `s.stdio.txt@utf-8/*` — raw file text

So: **open a file** = `io_identify(bytes)` → `io_run(io_route(carrier → D))`. **Save a file** = `io_run(io_route(D → carrier))`.
This is what stops exports writing pack bytes into `.gif`/`.png` files (today's `registry_export_media` bug).
Conformance (`pdf@1.4/*`→`pdf@1.4/a`) and standard migration (`gif@87a/*`→`gif@89a/*`) are ordinary `IoEntry` rows.
`IoDeclaration.conformance` runs after every hop INTO that dialect (this replaces `SubsetValidator`).

`io_route` is deterministic: highest minimum fidelity, then fewest hops, then lexicographic coordinate order;
cycle-free; `max_hops ≤ 3`.

WIT: guest `list-io-entries`, `io-run(from,into,payload)`, `io-sniff(from,into,payload)`;
host `io-routes(from,into)`, `io-run(from,into,payload)` (host executes the whole cross-plugin route — guests never
chain), `io-identify(payload)`; keep `resolve-artifact-link`. `from`/`into` are `ArtifactDialect::to_coordinate` strings.

**Exclusivity.** Outside `🚪️io` (and `#[cfg(test)]`), these are policy breaches:
`parse_dsl` / `print_dsl` / `encode_pack` / `decode_pack` / `ArtifactDsl::` / `ArtifactPack::` /
`include_bytes!` / `include_str!` of an artifact payload / `std::fs::` / `semio_s_plugin_<other>::…::io::`.
Viewers, editors and commands convert through the host imports (`host_io_run` in Rust, `ioRun` in TS).
`serde_json` is **not** banned (it is the UI/command protocol).

## 4. Modules

A `🔨️modules/<m>` needs **≥2 distinct consumer roots** (subset/standard/artifact dirs importing its path) at the
level it sits. One consumer ⇒ inline it. Subset-level modules are forbidden. Planned extractions:

| from | to |
|---|---|
| pdf object model shared by 1.4/1.7 and 7 profiles | `📄️pdf/🔨️modules/🧱️object-model` |
| step `🚪️io/{📐️part21,🧱️brep,🪜️ladder}` | `📐️step/🔨️modules/{📐️part21,🧱️brep,🪜️ladder}` |
| ifc artifact `🧬️schema` (`IfcEntity`,`IfcHeader`) + `🚪️io/🏛️spatial` | `🏗️ifc/🔨️modules/{🧱️entity,🏛️spatial}` |
| gif 87a LZW/colour-table used by 89a | `🎞️gif/🔨️modules/🧮️lzw` |
| mp4 `🚪️io/{🎥️h264,📦️boxes}` | `🎥️mp4/🔨️modules/{🎥️h264,📦️boxes}` |
| zip `📦️opc` (docx/xlsx/pptx consume) | `🗄️stdio/🔨️modules/📦️opc` |
| dwg ac1018/ac1024 shared decode | `🖊️dwg/🔨️modules/🧱️decode` |
| ooxml strict/transitional shared | `🗄️stdio/🔨️modules/📰ooxml` |
| `🧿️semio` cross-subset types (brep/mesh/drawing/table/value/…) | `🧿️semio/🏅️standards/🔖️v1/🔨️modules/<m>` |
| `✳️brep/🧬️schema/⚙️engine` | inline, or `🧿️semio/…/🔨️modules/🧱️brep-kernel` if ≥2 consumers |
| norm `🎚️config,👥️presence,📄️artifact,🖥️app-surface` | `📕️norm/🔨️modules/…` |
| fem/space plugin `⚙️engine` | `🏗️fem/🔨️modules/⚙️engine`, `🪐️space/🔨️modules/⚙️engine` |
| draw `🔄️fsm` | `🖍️draw/🔨️modules/🔄️fsm` |
| cad `🎬️interaction-spec`, artifact-level `📚️examples` (cad/layout/draw) | cad module / subset `📚️examples` |
| stdio `📇️registry` | deleted (declaration tree replaces it) |
| gltf subset `🔨️modules/*` | inline (single consumer) or lift to `🧊️gltf/🔨️modules` |

## 5. Recipe per subset (refined in `📓️recipe-subset.md` after the first shard)

1. Create/rewrite `✳️<x>/🦀️component.rs`: `#[path]` mounts of `🧬️schema`, `🚪️io`, `👁️viewer`, `✏️editor`, `📚️examples`
   + `pub fn subset() -> SubsetDeclaration`.
2. Move native codec leaves out of `🧬️schema/*/{📝️text,💾️binary}` into `🚪️io/📥️import/🧩️deserializers/*` and
   `🚪️io/📤️export/🧵️serializers/*`, taking `📖️component.grammar.semio` / `📡️component.protocol.semio` with them.
3. Rewrite foreign leaves as `impl Serializer<Snapshot>` / `impl Deserializer<Snapshot>`; delete the hand-rolled
   `compose()` dispatch chain, `ArtifactComposition`, `ArtifactAnalyzer`, `SubsetValidator`, `derive_artifact_facets!`,
   `io_registry::entries()`, `register()`.
4. Derived subsets: give them their own `Snapshot`/`Diff`/`Mutation` types (built from module types where shared) and
   a `Deserializer FROM` the base dialect carrying the conformance check + a `Serializer INTO` the base.
5. Extract shared code to `🔨️modules` at the lowest common owner; delete the sibling `use`.
6. Standard root + artifact root components with their declaration fns; plugin root `.artifact(…)`.
7. TS twins: `🚪️io/🟦️component.ts` exports the `IoEntryDescriptor[]` mirror; `📦️index.ts` mirrors the tree.
8. Examples as DSL text assets (`🗣️*.semio`) loaded through the native text deserializer.
9. Delete every `📦️glue.rs` shim that re-exported a sibling subset.
10. Verify: `CARGO_TARGET_DIR=<ticket>/🎯️target cargo nextest run -p <crate> --all-targets` (exact numbers in the report).
