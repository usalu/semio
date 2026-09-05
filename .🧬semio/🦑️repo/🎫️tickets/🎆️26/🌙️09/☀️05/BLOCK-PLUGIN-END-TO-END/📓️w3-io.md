# W3 — `✏️s/🔌️plugins/🧱️block` io layer: made real and registered

Scope: `🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/**` plus each subset root
`✳️any/🦀️.rs` and each artifact root `🗿️artifacts/<dim>/🦀️.rs`. All paths relative to
`/Users/ueli/Documents/semio`.

## 1. The registration mechanism (what a healthy plugin actually does)

The task brief pointed at `🧩️puzzle` and `🌀️procedural` as the healthy reference. **They are not.**
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs` carries the *same*
`entries: &[]` deviation block block did, self-documented as a lease-request for a later ticket. The
plugins that already run the new channel are `🗒️note`, `🖍️draw`, `🕸️dag`, `✒️writer`, `🎬️sequence`,
`💡️reasoning`, `📋️forms`, `➗️mathematical`, `🌿️vcs`, `🪵️sourcing`, `🎞️animate`.

The reference implementation followed here is
`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:466-572`.

**The channel, end to end:**

1. `🧰️framework/🔨️modules/🚪️io/🦀️.rs:2364` `pub mod io_mechanism` declares two typed traits —
   `Serializer<S> { const INTO: Dialect; const FIDELITY: IoFidelity; async fn serialize(&S) -> IoResult<IoPayload> }`
   and `Deserializer<S> { const FROM; const FIDELITY; const CONFORMANCE; async fn sniff(..); async fn deserialize(..) }`.
   Both method signatures are RPITIT (`-> impl Future<…> + Send`), so an impl **must** spell them
   `async fn` (see §6 — most existing plugins currently do not).
2. `serializer_entry::<S: store::ArtifactPack, T: Serializer<S>>(own: Dialect) -> IoEntry` and
   `deserializer_entry::<S, T>` (`🚪️io/🦀️.rs:2669`/`:2710`) erase one typed impl into an
   `IoEntry { from, into, fidelity, sniff, run }` vtable row. They decode/encode the *native* side
   through `S::decode_pack`/`S::encode_pack` and drive the async codec body with `resolve_ready`.
3. A subset publishes its rows as `IoDeclaration { native: NativeCodecs {…}, entries: &'static [IoEntry] }`
   returned by a `pub fn io()` in its own `🚪️io/🦀️.rs`.
4. The subset root `🪆️subsets/✳️any/🦀️.rs` binds `io: io::io()` into its `SubsetDeclaration`.
5. `Plugin::builder(…).declare_artifact(artifact())` → `commit_artifact_declarations`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28128`) preflights every entry
   (`preflight_io_entries`, `:28086` — the only law is "no two entries claim the same
   `(from, into)` at different fidelity") and then registers them on the global `io_mechanism`
   registry, one `io_register` per subset.

**The OLD channel that block was still on** (`ArtifactComposition` + `io_registry::entries()` +
free-function `serialize_bytes`/`deserialize_bytes`) is a different mechanism entirely. Its
`io_registry::entries()` in all three block subsets had **zero callers repo-wide** (verified by
`grep -rn "block2d::io::\|block3d::io::\|block5d::io::"` outside the io roots themselves) — it was
dead code that nevertheless *looked* like registration. It is deleted, not shimmed.

`derived_composition` is kept because it is a *different* facet:
`semio_framework_plugin::derive_artifact_facets!` in each subset's `🧬️schema/🦀️.rs` binds
`composition: super::super::io::derived_composition::Block<N>dComposerComposition`. It is now
native-dialect-only; its six foreign-format branches moved into the typed leaves.

## 2. Per-format decision table (identical in all three subsets)

| foreign dialect | direction | fidelity | decision | why |
|---|---|---|---|---|
| `s.stdio.txt@utf-8/*` | export + import | `Exact` | **real** — this subset's own `.semio` DSL snapshot text via `store::ArtifactDsl::print_dsl`/`parse_dsl` | the txt rendition of a block document IS its DSL text: the exact bytes `📚️examples/**/🖼️assets/**/🗣️.dsl.semio` carry |
| `s.stdio.json@rfc8259/*` | export + import | `Exact` | **real** — `dsl::ToValue` record tree → `dsl::json::from_dsl_value` → stdio's `write_json_text` | the snapshot is a pure record tree; every field survives both ways |
| `s.stdio.zip@2.0/*` | export + import | `Exact` | **real** — a genuine zip 2.0 container (stdio's `encode_zip`/`decode_zip`) with two members: `snapshot.<ext>.semio` (authoritative DSL text) and `snapshot.json` | the examples' own `🎒️.pack.semio` shape generalised; import prefers the DSL member and falls back to json |
| `s.stdio.stl@ascii/*` | export + import | `Lossy` | **typed `Err`** | a block document is a KIND DEFINITION. Its only geometry-bearing field is `representations[].mesh_url` — a URL to an external mesh asset, never vertex data (`✏️s/🔌️plugins/🧱️block/🦀️.rs`, `BlockRepresentation`). `handles`/`vortices`/`grips` are anchor frames (angle/radius/position/direction), not a surface. Conversely an STL solid carries no identity, catalog or compatibility rows. |
| `s.stdio.obj@3.0/*` | export + import | `Lossy` | **typed `Err`** | identical reasoning to `stl` |
| `s.stdio.png@1.2/*` | export + import | `Lossy` | **typed `Err`** | the schema has no raster field at all, and the plugin ships no rasterizer (`👁️viewer` renders through the framework's window kits, not into a buffer a leaf can reach). Painting a blank canvas — what `🗒️note`'s own png leaf settles for — would silently claim an export that did not happen. |

**Why the three refusing hops are still REGISTERED.** An unregistered `(from, into)` pair yields a
bare "no route" at the router; a registered one at the weakest fidelity (`Lossy`, rank 0, so the
router never prefers it over a real hop) hands the caller the actual sentence explaining why the
conversion cannot exist. The message shape is
`"<fmt> {import,export} not supported for a <noun> kind definition: <reason>"`, asserted by the Rust
test `geometry_and_raster_hops_refuse_with_a_reason`.

**What was there before (all three subsets, verified before the rewrite):**

- `🎒️zip`, `📷️png`, `🔺️stl`, `🧊️obj` **export** returned `print_dsl(snapshot).into_bytes()` — plain DSL
  text mislabelled as those four binary formats. Worse than a stub: no zip/png/stl/obj reader could
  open it and nothing said so.
- `🎒️zip`, `📷️png`, `🔺️stl`, `🧊️obj` **import** ignored `bytes` and returned
  `Ok(Block<N>dSnapshot::default())` — silent total data loss.
- `🔤️txt` both directions returned `Err("txt … not yet implemented")`, and the **export** leaf carried
  a stray `deserialize_bytes` (an import-direction function inside the export tree, from a
  copy-paste of stdio's json↔txt bridge). Deleted.
- `🔣️json` was the only genuinely implemented pair; its logic is preserved verbatim inside the new
  typed impls.

## 3. Files changed

**Rust — 36 io leaves rewritten as typed `Serializer`/`Deserializer` impls** (12 per subset):
`✏️s/🔌️plugins/🧱️block/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}/🗿️artifacts/{🔣️json/🔖️rfc8259,🔤️txt/🔖️utf-8,🎒️zip/🔖️2.0,🔺️stl/🔖️ascii,🧊️obj/🔖️3.0,📷️png/🔖️1.2}/✳️any/🦀️.rs`

**Rust — 3 io roots rewritten** (truthful, structurally identical headers; `derived_composition`
trimmed to native-only; old `io_registry`/`import_stdio_kinds`/`export_stdio_kinds` deleted; new
`pub fn io() -> IoDeclaration`; 5 unit tests each, 6 for `◻️2d`/`🖐️5d`):
`…/{◻️2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`

**Rust — 3 subset roots rewritten** (`io: io::io()` replaces the local `io_declaration()` and its
DEVIATION note): `…/{◻️2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`

**Rust — 3 artifact roots, minimal edits** (`"stdio.txt"` added to `artifact_kind()`'s
`export_stdio_kinds`/`import_stdio_kinds`, and a `composer.format-6` capability row for
`s.stdio.txt@utf-8/*` in `definition()`):
`✏️s/🔌️plugins/🧱️block/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/🦀️.rs`

**TypeScript — 6 leaves + 2 test files + 4 fixtures:**
- `…/{◻️2d,🖐️5d}/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts` — field
  tables, float formatter, string escaper, `block<N>dToJsonText`
- `…/{◻️2d,🖐️5d}/…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts` —
  `block<N>dFromJsonText`, `block<N>dCanonicalJsonText`
- `…/{◻️2d,🖐️5d}/…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️.ts` — a real
  `.semio` DSL **reader** (`block<N>dFromDslText`)
- `…/{◻️2d,🖐️5d}/…/🚪️io/🧪️tests/🟦️.ts` — the bun parity tests
- `…/{◻️2d,🖐️5d}/…/🚪️io/🧪️tests/🧫️fixtures/*.json` — the shared oracle, asserted from BOTH sides

**Ticket generators** (kept, they are the single source for the 39 structurally identical files):
`🐍️w3-io-leaves.py`, `🐍️w3-io-roots.py`, `🐍️w3-io-typescript.py`, `🐍️w3-io-typescript-txt.py`,
`🟦️w3-fixture.ts`.

## 4. Cross-language parity design

`🚪️io/🧪️tests/🧫️fixtures/<asset>.json` is one file asserted from both languages:

- **TypeScript** (`bun test …/🚪️io/🧪️tests/🟦️.ts`): `block<N>dToJsonText(block<N>dFromDslText(dsl))`
  must equal the fixture byte-for-byte, and `block<N>dCanonicalJsonText(fixture)` must be a fixed
  point.
- **Rust** (`🚪️io/🦀️.rs::json_matches_the_typescript_parity_fixture`):
  `json_text(&from_dsl_text(EXAMPLE))` must equal the same `include_str!`-ed bytes.

The TS writer is a genuine second implementation: it re-derives member order from its own field
tables, re-implements `serde_json`'s escaping rules, and re-implements `pack::json::write_float`'s
lexeme rule (shortest round-trip digits; fixed notation for decimal exponent in `-5..=15` with an
explicit `.0` on whole numbers; `e±` exponential otherwise). Nothing is echoed from the Rust output.

The TS DSL reader independently discovered and implements the grammar's positional `None` marker
(an unquoted `_`, `TokenKind::Placeholder`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔍️lexer/🦀️.rs:560`) — it appears in
`🖐️5d/📚️examples/🎬️hexagonal-cut-concrete-forest-left`'s `lod` column.

## 5. Verification (real outputs)

### 5.1 bun — PASSING

```
$ cd /Users/ueli/Documents/semio && bun test \
    "./✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts" \
    "./✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts"
bun test v1.3.14 (0d9b296a)

 8 pass
 0 fail
 8 expect() calls
Ran 8 tests across 2 files. [98.00ms]
```

(Note: `bun test <glob>` does not discover `🧪️tests/🟦️.ts` — bun requires `.test`/`.spec` in the
filename for glob discovery. The explicit `./`-prefixed path works and is what the command above
uses. Wiring this into an nx target belongs to `📦️packages/🟦️typescript/📜️script.ts`, which W1 owns.)

### 5.2 cargo — BLOCKED by another session's in-flight breakage

`cargo check -p semio-s-plugin-block --lib` was run three times with
`RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-block-io CARGO_BUILD_JOBS=4 RUSTFLAGS=-Awarnings`.
It never reached `semio-s-plugin-block`: a dependency failed each time, in a different crate each
time, in files this packet did not touch.

Run 1 (baseline, before any edit) and run 2 — `semio-framework-os-kernel`:

```
🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/📇️directory/🔌️client/🦀️.rs:22:61: error[E0432]: unresolved import `super::schema::DirectorySpaceDetailV1`: no `DirectorySpaceDetailV1` in `os_directory::schema`
error: could not compile `semio-framework-os-kernel` (lib) due to 1 previous error
EXIT=101
```

(`git status` shows `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/{🧬️schema/🦀️.rs,🔌️client/🦀️.rs}`
staged-modified by a peer session; that peer fixed it between runs.)

Run 3 — `semio-s-plugin-stdio`, mid-rename (`🔣️json` → `🧾️json`, `🪆️subsets/✳️base` → `🧱️base`,
`🧊️obj` subset renamed) by the `ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY` refactor:

```
✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/../../📇️registry/🦀️.rs:257:5: error: couldn't read `…/../🗿️artifacts/🧊️obj/🧬️schema/📜️artifact-definition.json`: No such file or directory (os error 2)
✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/../../📇️registry/🦀️.rs:273:5: error: couldn't read `…/../🗿️artifacts/🎞️pptx/🧬️schema/📜️artifact-definition.json`: No such file or directory (os error 2)
✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/../../📇️registry/🦀️.rs:897:75: error: couldn't read `…/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio`: No such file or directory (os error 2)
✏️s/🔌️plugins/🗄️stdio/…/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️replace-pixel-data/🦀️.rs:8:1: error: MutationLeaf source authority failed: descriptor owner does not exactly match source owner
```

…and, deeper in the same run, the `🎞️pptx` subset mid-rename:

```
✏️s/🔌️plugins/🗄️stdio/…/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🧬️schema/🦀️.rs:43:25: error[E0277]: the trait bound `PptxMutation: Mutation<PptxSnapshot>` is not satisfied: unsatisfied trait bound
```

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/` no longer exists on disk at
all — that artifact's subset tree is being moved right now. Per the packet's own instruction, this
is quoted and left alone rather than fixed. Full logs:
`🗑️generated/w3-cargo-check-run{1-baseline,2,3}.txt`.

### 5.3 rustfmt — PASSING (syntax + repo formatting)

The only Rust verification that could run. `rustfmt` fully parses a file, so a clean run proves
every one of the 39 generated files is syntactically valid Rust; it does not prove type-correctness.

```
$ cd /Users/ueli/Documents/semio
$ for f in <36 io leaves + 3 io roots + 3 subset roots>; do rustfmt --emit stdout "$f" >/dev/null; done
# (no output — every file parsed)
$ for f in <same>; do rustfmt --check "$f" >/dev/null || echo "STILL: $f"; done
remaining: 0
```

`rustfmt` was then applied in place with the repo's own `rustfmt.toml`
(`edition 2021`, `max_width 250`, `use_small_heuristics = "Max"`), so the committed files match the
repo formatting gate. **Re-running the ticket generators re-emits the pre-`rustfmt` layout** — run
`rustfmt` over the 42 files again afterwards.

## 6. Unverified items and known pre-existing blockers

1. **The whole Rust half of this packet is written but not compiled.** No `cargo check` run reached
   `semio-s-plugin-block`. Everything under §2/§3 that is Rust is "written to the declared trait
   signatures", not "verified to compile".

2. **Pre-existing async-convention breakage inside `🧱️block` itself, independent of this packet.**
   `store::ArtifactDsl`/`store::ArtifactPack` declare **synchronous** methods
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4891` and `:9326` —
   `fn parse_dsl(&str) -> Result<Self, TextError>`, `fn print_dsl(&self) -> String`,
   `fn encode_pack_with(…)`, …), but block's own impls spell them `async fn`, e.g.
   `…/◻️2d/…/🧬️schema/📸️snapshot/🦀️.rs:66-104`. That is `error[E0053]` (confirmed with a minimal
   `rustc --edition 2021` reproduction). Repo-wide there are 55 `async fn print_dsl` impls against
   146 sync ones — a half-applied codemod. **This packet did not touch those files and did not
   "fix" them**, because doing so would fight whichever session is mid-migration. Until they are
   reconciled, `semio-s-plugin-block` cannot compile regardless of the io layer.

   The inverse mismatch exists on the `io_mechanism` side: the traits are RPITIT
   (`-> impl Future + Send`) but all 36 existing `fn serialize(from: &…) -> IoResult<IoPayload>`
   impls across `🗒️note`/`🕸️dag`/`✒️writer`/… are synchronous. **This packet's new leaves follow the
   trait as declared** (`async fn`), which is the side the framework's own
   `resolve_ready(T::serialize(&value))` call sites require.

3. **TypeScript gaps (deliberate, precisely scoped):**
   - **`🧊️3d` has no TS json mirror.** `Block3dSnapshot.catalog` is
     `store::ArtifactChild<SemioKitSnapshot>` (`🧰️framework/…/🏪️store/🦀️.rs:2688`,
     `{ child_id: String, target: ArtifactRef }`), a framework-internal child-reference encoding that
     the `🟦️.ts` typed twins do not declare and that no TS-visible schema pins down. Mirroring it
     would mean hand-guessing a framework type's wire shape — exactly the kind of assumption
     CLAUDE.md forbids. `◻️2d` and `🖐️5d` are pure record trees and are fully mirrored.
   - **No TS DSL *printer*.** The reader is a schema-driven parser for the document shape the
     snapshot grammar actually emits; the printer additionally needs the grammar's layout rules
     (block/table selection, column-header synthesis, `@`/`^`/`rad` re-suffixing, per-type number
     lexemes, 2-space indentation, the `_` placeholder policy) which live in
     `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/` and have no TS twin anywhere in the repo. Porting
     `dsl::print` is its own packet. Consequence: the TS `🔤️txt` **export** leaf stays `export {};`.
   - The other 24 TS io leaves (`🎒️zip`/`📷️png`/`🔺️stl`/`🧊️obj` for all three subsets, plus every
     `🧊️3d` leaf and both `🔤️txt` export leaves) remain `export {};`.

4. **`print_dsl` fixed-point assertion is a hypothesis.** `txt_round_trips_every_example` asserts
   `print_dsl(parse_dsl(asset)) == asset` (modulo a trailing newline). The example assets are
   documented as having been generated by `print_dsl`
   (`…/🧬️schema/📸️snapshot/📝️text/🦀️.rs`'s own test comment), so this should hold — but it has not
   been run. If it fails it is a real finding about fixture drift, not a bug in this leaf.

5. **`zip` fidelity is declared `Exact`** on the argument that the container's DSL member is
   lossless. If the framework later treats `Exact` as "byte-identical re-encode", zip should drop to
   `Canonical`; `preflight_io_entries` does not care either way today.

6. **`artifact_kind().{export,import}_stdio_kinds` still list `stdio.obj`/`stdio.png`/`stdio.stl`.**
   Those hops now refuse with a reason rather than silently lying, so the lists are no longer
   *false*, but a UI driven by them will offer three exports that always fail. Trimming them is a
   UI-lane decision (the editor packets own those menus) and was left alone.
