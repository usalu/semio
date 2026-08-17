# stdio g6 — semio artifact — declarative conversion: NOT converted, reasoned STOP

## Assignment
Group g6, single artifact: `stdio.semio` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs`),
registered from the plugin root via `crate::artifacts::semio::standards::v1::engine::register();`.

## Finding: structural gap, not an oversight — left fully imperative

`semio`'s v1 `engine::register()`
(`🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:9-29`) is a fan-out over **19
independent per-subset `register()` calls** (brep, mesh, model, value, document, cad, drawing,
image, video, audio, animation, presentation, flow, text, table, graph, object, kit, any). I read
one representative fully (`✳️animation/🚪️io/🦀️component.rs:127-133`, doc-commented "Registers this
subset's schema descriptor, document codec, and SubsetValidator"): each subset independently calls
`register_artifact_schema_descriptor(...)` with its **own** id (e.g. animation's own descriptor,
keyed off `STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA`) and `register_document_codec(ArtifactCodec::of::<
ThatSubset'sSnapshot, ThatSubset'sMutation>(...))` with its own schema string, plus its own subset
validator and composer bridge entries.

Verified against the framework: `ArtifactSchemaRegistry::register` (`🧰️framework/🔨️modules/🧬️schema/
🦀️component.rs:182-184`) is a genuine `HashMap<id, descriptor>` — many descriptors are meant to
coexist under one artifact. But `ArtifactDeclaration`'s `schema` field
(`🔌️plugin/🦀️component.rs:953`) and `document_codec` field (:959) are both **singular** —
`Option<ArtifactSchemaDescriptor>` / `Option<DocumentCodecSpec>`, and `.schema(...)` is the
**mandatory, typestate-gating** first call (`NeedsSchema → DeclarationReady`, :1019-1039) — every
other builder method (`.composers`, `.subset_validators`, `.inferences`, `.document_codec_bare`,
etc.) is unreachable until exactly one schema is supplied. `semio` genuinely needs 19 schema
descriptors and 19 document codecs under one `artifact_kind`; the builder has room for one of each.
Picking one subset's pair arbitrarily would silently drop registration for the other 18 — exactly
the kind of "invent a field or drop a call" the task instructs against. `composers`,
`subset_validators`, and `inferences` ARE plural fields and could in principle carry all 19 subsets'
worth, but the mandatory `.schema()` gate blocks reaching them at all without first resolving the
singular-schema problem, so nothing about this artifact can move to the declarative side today. This
needs a new plural mechanism (e.g. `.schemas()`/`.document_codecs()`, keyed per subset) that does not
exist and is out of this task's scope to invent.

## Corroborating: the group note's in-flight-migration caution is real, independently confirmed

Per `git log`/`stat` (not assumed): `✳️mesh/🧬️schema/🧬️mutations` mtime **Aug 13 00:24:18**,
`✳️brep/…/🧬️mutations` **Aug 12 21:48:51**, `✳️drawing/…/🧬️mutations` **Aug 12 22:03:27** — all far
more recent than this ticket's stdio baseline measurement (00:41) and all inside the "another session
is actively migrating brep/drawing/mesh mutations" window the group note warned about. This is a
second, independent reason not to touch anything under those subsets' `🧬️mutations/**`, consistent
with the hard rule. I made no edits there.

## Action taken

**Zero code changes.** No `declaration()` added at `🧿️semio/🦀️component.rs`. Plugin root line
`crate::artifacts::semio::standards::v1::engine::register();`
(`✏️s/🔌️plugins/🗄️stdio/🦀️component.rs:37`) left untouched — still imperative, still present.

## Verification

- `grep -rn "fn declaration" .../🗿️artifacts/🧿️semio` → **no match** (expected — none added, per
  above).
- `grep -n "register();" .../🦀️component.rs | grep semio` → line 37 present, unchanged.
- `grep -rn "io_registry::entries" .../🗿️artifacts/🧿️semio` → **zero bare calls** (none found at all
  within semio's tree; the artifact-root `io_registry::entries()` at `🧿️semio/🦀️component.rs:45-46`
  already fully qualifies as `v1::entries()`).
- `RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-stdio --all-targets`
  → **`Finished \`dev\` profile [unoptimized] target(s) in 2m 06s`, `EXIT=0`, 0 lines matching
  `^error`.** Full log: `scratch-g6-semio-check-final.txt` (also `scratch-g6-semio-check.txt`, an
  earlier identical run). Warnings only (unused fns/imports, pre-existing, not from any edit here —
  I made none).

## Summary (10 lines)

1. Assigned artifact: `stdio.semio` (g6, single artifact).
2. Converted: none. Left fully imperative — plugin root's `engine::register()` call untouched.
3. Reason: `semio`'s v1 standard fans out to 19 subset `register()` calls, each with its own
   `ArtifactSchemaDescriptor` id and document codec.
4. `ArtifactDeclaration.schema`/`.document_codec` are singular `Option`s; `.schema()` is the
   mandatory gate before any other field is reachable — 19-vs-1 is unrepresentable without a new
   plural field, which is out of scope to invent.
5. `composers`/`subset_validators`/`inferences` are plural and could hold all 19 subsets' data in
   principle, but are unreachable without first resolving the schema singularity, so nothing moved.
6. Corroborating, independently verified via `git log`/`stat`: `brep`/`drawing`/`mesh` mutations
   dirs all mtime within the last ~3h (00:24, 21:48, 22:03) — active peer churn, untouched by me.
7. No files edited; no `declaration()` added; no `.artifact(...)` added to the plugin root.
8. `grep "fn declaration"` under `🧿️semio` → no match (expected, none added).
9. `grep "io_registry::entries"` under `🧿️semio` → zero bare calls (pre-existing, already qualified).
10. `cargo check -p semio-s-plugin-stdio --all-targets` → green, `Finished`/`EXIT=0`, 0 errors — logs
    in `scratch-g6-semio-check-final.txt`.
