# Lowpoly `testing/contract` Breaches — 9 → 8

BEFORE: 9 (1 oracle, 1 protocol, 7 io). AFTER: 8 (1 oracle, 7 io). Verified by re-running
`bun ./📜️script.ts test 2>&1 | grep 💠️lowpoly | grep testing/contract`.

## CLOSED — Breach 2, wire records (`🧬️mutations/💾️binary/📡️.protocol.semio`)

Root cause: `binaryProtocolDriftBreaches` (framework `🟦️.ts:3621`) requires each `record` line to
match `/^record\s+([a-z][a-z0-9-]*)\s+tag=(\d+)/m` — kebab-case kind name, `tag=N` (no space). The
file had PascalCase names (`CreateObject`) and `tag 1` (space), so the regex matched zero records —
100% miss, hence "17 have no wire record". Rewrote all 17 as kebab-case `tag=N`, matching lowpoly's
`LowpolyMutation::KINDS` exactly (`🧬️mutations/🦀️.rs:66-83`), keeping the original tag numbers. Added
`field` lines per record (name, order and type from each mutation's own struct — e.g.
`🌱️create-object/🦀️.rs`, `↗️move-object/🦀️.rs`) modeled on the one sibling protocol that already
uses this exact grammar correctly, `📐️cad/…/🧬️mutations/💾️binary/📡️.protocol.semio` (verified against
the framework grammar parser, `🧰️framework/…/🗣️dsl/📖️grammar/🦀️.rs:750-800`, which is where `f32`,
`varint`, `utf8`, `bytes`, `array <prim>` are defined as valid primitives — `f32` is real, not
over-widened to `f64`). String → `utf8`, `usize` → `varint`, `bool` → `u8`, `[f32;3]` → `array f32`,
nested structs (`LowpolyObject`, `LowpolyPaintLayer`, `Vec<PixelRun>`) → `bytes`, matching cad's own
`bytes`-for-nested-struct precedent. Confirmed against the one committed mutation fixture with JSON
(`🌱️create-object/🧪️tests/…/🦠️mutation/🔣️.json`) that field names/order agree with the Rust struct
(no binary-encoded fixtures exist for mutations — only JSON — so JSON field order was the strongest
available evidence, as directed).

## NOT CLOSED — Breach 1, runtime inventory (`🧪️oracle/🔣️.json`)

`test inventory` (framework `📜️script.ts:1011`, `InventoryScript`) requires an owner-published
`🏭️bridge/📜️script.ts` beside the subset that shells to a Rust binary reading
`<Mutation>::DESCRIPTORS` and prints a `runtime-inventory/v2` JSON. Exactly 3 plugins have this today
(stdio/semio@v1 mesh/brep, step/cc6) — all built the same way: a **standalone** bridge `Cargo.toml`
with its own `[workspace]` override, so the bridge crate never joins the contended root
`Cargo.toml`'s `members` list (confirmed: root `Cargo.toml:4` is a hand-written list, no globs — a
non-overlaid bridge crate would have to be added there directly). This ticket's hard rules forbid
adding a `[workspace]` overlay to any Cargo.toml, and the alternative — adding a `[[bin]]` to
lowpoly's existing shared, multi-agent, root-workspace-member crate
(`✏️s/…/💠lowpoly/📦️packages/🦀️rust/Cargo.toml`) — is outside my 3 exclusively-owned paths and risks
a full-crate rebuild/contention for concurrent devs on a file nobody handed me. `LowpolyMutation`
already derives `dsl::Mutations` (so `DESCRIPTORS` exists and the bridge logic itself would be a
direct copy of the mesh bridge's `main()`), so the ONLY blocker is where the bridge crate's build
config can legally live under this ticket's constraints. Left honestly unclosed rather than violate
either rule.

## TRUE FINDING, NOT WEAKENED — Breaches 3-9, io serializers (7 files)

Confirmed by reading `🚪️io/📤️export/🧵️serializers/…` for all 7 formats plus the framework's
`stubSerializerBreaches` detector (`🟦️.ts:5241`): `LowpolyObject.mesh` is
`Option<store::ArtifactChild<SemioMeshSnapshot>>`, a content-addressed handle; `ComposeSource`/
`ErasedComposeSource` (`🧰️framework/…/🚪️io/🦀️component.rs:768-802`) give the io layer only
`{dialect, payload}`, no store/session resolver — a synchronous `&LowpolySnapshot -> Bytes` function
cannot reach real mesh vertices. This is unchanged from and matches this ticket's own prior
`📓️research/📝️io-format-truth.md` (same conclusion, independently re-derived here). obj/ply/png
legitimately smuggle the DSL text through each format's own real per-line retention slot
(`ObjUnknownStatement`, `PlySnapshot.comments`, PNG `tEXt`) — honest carrier tricks, not lies, but
still not obj/ply/png geometry, so the gate is right to flag them. stl/gltf/dwg/las have no such
retention slot and are explicit typed `Err(...)` stubs — never a silent empty/wrong success. The
gate's own suggested remediation is "implement the serializer, or remove `<format>` from
`exportDialects`"; removing formats would understate real capability (these ARE registered,
DSL-carrying, round-trippable exports) and dropping the 4 pure stubs would just delete an honest
"not implemented" for a format still worth claiming once geometry is reachable — so no in-scope
change makes this pass honestly. **Framework capability needed to close for real**: give the io
serializer signature (`serialize`/`serialize_bytes`) access to a store/session resolver so
`ArtifactChild<S>` handles can be followed synchronously or the call path threaded async — out of
scope for `🚪️io/**` alone, this is a `🧰️framework/🔨️modules/🚪️io` signature change affecting every
owner with a child-handle field, not just lowpoly.
