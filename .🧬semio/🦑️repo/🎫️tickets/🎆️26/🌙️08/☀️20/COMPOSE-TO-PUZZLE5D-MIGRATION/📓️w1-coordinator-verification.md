# Wave 1 — Coordinator verification of census claims

The census fleet's highest-stakes claims were independently re-verified by the coordinator before
being allowed into the frozen contract. Three census claims were **wrong** and are corrected here.
Do not act on the raw census reports for these items — act on this file.

---

## ✅ CONFIRMED — 28 atomic mutations, exactly as planned
`…/🧬️mutations/🦀️component.rs` `enum Puzzle5dMutation` — 28 variants, matching the plan's list and order.

## ✅ CONFIRMED — Nakagin 180 pieces / 179 connections
`compose/client/lib/rs/lib.rs:21601-21607`
```rust
assert_eq!(wip_design.has_pieces().await.len(), 180, "wip nakagin piece count");
assert_eq!(wip_design.has_connections().await.len(), 179, "wip nakagin connection count");
assert_eq!(design.has_pieces().await.len(), 180, "nakagin piece count");
assert_eq!(design.has_connections().await.len(), 179, "nakagin connection count");
```
Design external id `9a890dd4-0a9c-48ac-920a-9e62666465ef`.

## ✅ CONFIRMED — `quality_sum` oracle conflict
`compose/client/lib/rs/lib.rs:5279-5281` is a GraphQL resolver returning constant `0.0`:
```rust
#[graphql(name = "qualitySum")]
pub async fn quality_sum(&self, _quality_id: Id) -> f64 { 0.0 }
```
`compose/fixture/quality-sum.cases.compose.json` expects `2349.53` ±`0.01`.
Go `SumQualityInDesign` (`compose/client/lib/go/main.go:5639-5678`) implements the real behavior.
**Ruling: fixture + Go win. The Rust stub is NOT the specification. Migration implements real summation.**

## ✅ CONFIRMED — flatten removes all fasteners
`compose/client/lib/go/main.go:14209` `FlattenDesignDiff` opens by building
`removedConnList` over **every** connection in the design. Flatten materializes absolute poses and
drops the entire connection set.

**Refinement the census missed:** Rust and Go implement *different halves* of flatten.
- Rust `flatten_design_positions(kit, design) -> HashMap<Id, PositionInput>` (`lib.rs:1393`) is a
  **pure position solver** — it returns poses and touches no connections.
- Go `FlattenDesignDiff` (`main.go:14209`) is the **full semantic operation** — solve poses,
  remove all connections, promote pieces to fixed.

This maps exactly onto the plan's §5.4 split: Rust's function is the `GeometrySolver` mechanism;
Go's function is the `flatten` semantic mutation. Port both, at their correct layers.

---

## ❌ CORRECTION 1 — `flatten.cases.compose.json` contains NO expected output
The plan (§6.1, §11) states *"The committed `flatten.cases.compose.json` output is authoritative."*
**It has no output.** Every case carries only `{name, kit, designPath}`:
```json
{"name":"nakagin_capsule_tower","kit":"kit/dev/metabolism/wip/initialKit/kit.compose.json",
 "designPath":["Nakagin Capsule Tower"]}
```
It is a **case catalog, not an oracle.**

Oracle strength varies sharply across the fixture set — surveyed exhaustively:

| fixture | carries expected output? | oracle |
|---|---|---|
| `flatten.cases` | **NO** — `{name,kit,designPath}` only | ⚠️ none |
| `export-design-representation.cases` | **NO** — `{name,kit,designName}` only | ⚠️ none (the `.gltf`/`.ifc` are de-facto but unreferenced) |
| `flatten-merkle.cases` | YES — `expect` + `mutations` | strong |
| `design-with-diff.cases` | YES — `expected`, `expectedPieceCounts`, `expectedConnectionCounts` | strong |
| `copy-paste.cases` | YES — `expectedCopyAsset`, `expectedPasteDiffAsset`, `expectedPasteWithCoordinateDiffAsset` | strong |
| `delete.cases` | YES — `expectedDiffAsset` | strong |
| `filter-kit.cases` | YES — `expectedKit` | strong |
| `find-replaceable-types.cases` | YES — 8 distinct assertion keys | strong |
| `quality-sum.cases` | YES — `expected` + `tolerance` | strong (conflicts with Rust) |
| `architect.cases` | YES — `expect` + `graphqlResponses` | strong |
| `hash.cases` | YES — pinned digests | strong |

### Ruling
`flatten`'s only committed oracle is **indirect**, via `flatten-merkle.cases.compose.json`, which
pins `planeHash`/`centerHash` for two Nakagin pieces plus an 11-row sensitivity matrix.

But per H35, entity-type LABELS feed the hash (`merkle_node_str(&["Position", id, flat], …)`), so
renaming Piece→Part changes every digest by construction. **Hash equality across the terminology
change is impossible.** Therefore:

- Flatten parity is established **numerically**, not by hash: run the legacy solver at the pin, run
  the new solver, compare poses under the legacy tolerance.
- The flatten-merkle sensitivity matrix is retained as a **behavioral** oracle (which input channel
  moves which output channel), which survives renaming.
- The flatten after-snapshot fixture must be **generated and reviewed**, not lifted from compose.
  Its migration-manifest status is `covered-by-inference-test`, not `migrated`.

This does not change the plan's choice of `flatten` as the Wave-3 proof fixture — it still exercises
the full codec matrix. It changes only where its expected after-state comes from.

---

## ❌ CORRECTION 2 — the binary diff codec is NOT "already complete"
Census H44 reported the binary diff protocol needs no new files, only wiring. **Half right.**

`🧬️schema/🔺️diff/💾️binary/` exists with the same six-file shape as `📸️snapshot/💾️binary/`
(`📡️component.protocol.semio`, `🌶️component.spicy`, `🔠️component.abnf`, `🥋️component.ksy`,
`🟦️component.ts`, `🦀️component.rs`). But the Rust file is a **5-line stub**:

```rust
//! binary rep for stdio.json 🔺️diff
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
```

Compare the wired snapshot equivalent (60 lines, real `encode`/`decode` + tests):
```rust
pub async fn encode(document: &Puzzle5dSnapshot) -> Vec<u8> { store::ArtifactPack::encode_pack(document) }
pub async fn decode(bytes: &[u8]) -> Result<Puzzle5dSnapshot, PackError> { … }
```

Codec Rust line counts: snapshot/binary 60 · snapshot/text 120 · **diff/binary 5** · diff/text 193 ·
mutations/binary 99 · mutations/text 47. The diff/binary stub is the clear outlier.

**S03's actual task:** author `encode`/`decode` in `🔺️diff/💾️binary/🦀️component.rs` mirroring the
snapshot codec, confirm `📡️component.protocol.semio` describes that encoding, then flip
`diff: LanguagePair { text: …, binary: None }` → `Some(&langs[N])` in the subset root's
`io_declaration()`. Small and well-scoped — but it is real code, not a one-line wiring flip.

---

## ❌ CORRECTION 3 — compose has ZERO Rust consumers outside `compose/`; `erased_compose` is a false positive
Census H52 reported `🧰️framework/…/🔌️plugin/🦀️component.rs` as a compose consumer via
`erased_compose`. **That is a name collision.** `erased_compose` is a generic framework helper for
*composing artifact sources* (`ArtifactComposer`/`ArtifactDeserializer`/`ArtifactSerializer`
fn-pointer thunks at `:643/:684/:725`). It has nothing to do with the `compose/` tree.

Exhaustive re-grep for `semio-compose`, `@semio-tech/compose`, `compose-fixture` outside `compose/`
returns **no source-code hits at all** — only build orchestration:

| file | coupling |
|---|---|
| `Cargo.toml:115-117` | 3 workspace members: `compose/client/lib/rs`, `compose/client/lib/query/rs`, `compose/client/bin/gql/rs` |
| `go.work:8` | `./compose/client/lib/go` |
| `pyproject.toml:12,37` | members + testpaths `compose/py`, `compose/engine` |
| `Monorepo.sln` | 9 compose project entries |
| `package.json:164,165,244` | `query:build`, `query:test`, dep `@semio-tech/compose-js` |
| `📜️script.ts` | 13 orchestration targets (desktop, js, react, engine, 3dm-ui, sketchpad-play, sketchpad-docs, query) |
| `Cargo.lock`, `bun.lock` | generated |

All `@semio-tech/compose-*` packages resolve to paths under `compose/`. **Compose is fully
self-contained as code.** Wave 9 deletion is a clean subtree removal plus seven build-manifest edits.
This makes cleanup substantially cheaper than the plan assumed.

---

## ❌ CORRECTION 4 — TypeScript parity is VACUOUS
Every one of the 28 mutations' `🦠️mutation/🟦️component.ts` is **exactly one line**:
```ts
export {};
```
Verified on `🌱create-part`, `🗑delete-part`, `📍move-part2d`, `🔗connect-grips` — all 1 line, all
empty modules. There is no TypeScript apply, diff, or inverse implementation anywhere.

Consequences, stated plainly rather than papered over:
- Plan §2's `component.ts` requirement: **all 28 mutations are formally Rust-only targets**, recorded
  as such in each test descriptor. This is the plan's own documented escape clause, applied
  universally rather than exceptionally.
- Plan §3's "Implementation parity: Rust and TypeScript…" row: **vacuous** — nothing to compare.
- Plan §13's CI shard 7 "Rust/TypeScript parity": **vacuous** — it would always pass trivially.

Writing 28 real TypeScript mutation implementations is new scope the plan does not ask for and does
not budget. **Not undertaken.** The migration records the gap honestly instead of shipping a green
gate that checks nothing. Flagged for the user as a scope decision.

---

## ⚠️ OPEN — Nakagin part-count mismatch between the two worlds
compose Nakagin = 180 pieces / 179 connections (asserted, verified above).
Census H43 reports the puzzle5d Nakagin example's DSL asset at **≈250 parts** — and flags it as
unverified. If real, the two "Nakagin" assets are **not the same design**, and the plan's §8.3
"same graph topology" asset-parity rule cannot hold for this example.

This must be settled before the Wave-3 proof fixture is built on `nakagin-capsule-tower`.
Assigned to Wave 2 as a blocking question — see the contract's open-questions section.

## ⚠️ OPEN — paste ID generation is nondeterministic in Go
`compose/client/lib/go/main.go:274-278`:
```go
func Id() string { bytes := make([]byte, 16); rand.Read(bytes); return hex.EncodeToString(bytes) }
```
Rust has **no** paste/copy function at all (`grep "fn paste\|fn copy_"` → no hits); copy/paste is
`CopyDesign` `main.go:12451` and `PasteDesign` `main.go:12617`, Go-only. This confirms the plan's
§1.1 requirement: paste must serialize its ID map rather than regenerate ids at apply time.

---

## ✅ RESOLVED — Nakagin part count MATCHES. Census H43 was wrong.
`…/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio` (394 lines, 168 KB) is a
table-format DSL. Block structure:

| line | block |
|---|---|
| 7, 9, 11, 13 | `parts`/`grips`/`fasteners`/`ropes` **kind-catalog** headers — all EMPTY (catalogs are referenced by handle `kind-catalogs-7904dd65836c8ff4`, not embedded) |
| 16 | `kind-compatibility` rows |
| 32 | `parts [id part-kind anchor part-2d part-3d grips]` — **instance** rows |
| 214 | `fasteners [id source target fastener-kind gap shift rise rotation turn tilt x y]` rows |

Row counts measured directly:
- parts (lines 33-212): **180**
- fasteners (lines 215-393): **179**

**180 / 179 — exactly equal to compose's 180 pieces / 179 connections.**

H43's "≈250 parts" counted every indented line in the file, including the `kind-compatibility`
block. The flagship asset is already migrated with identical topology; asset parity for Nakagin is
established, not outstanding. The Wave-3 proof fixture may safely build on this example.

Note the empty embedded catalogs + `kind-catalogs=child_id=… target="…!s.stdio.semio@v1/kit"`
handle on line 6: this example is ALREADY on the handle shape that
`normalize_kind_catalogs_for_snapshot_value` exists to convert legacy embedded catalogs into. It
will not exercise that legacy path — a separate fixture must, or the path is simply dead and can be
deleted outright.

## ⚠️ OPEN — the other two examples' asset coverage
`🏗️nakagin-capsule-tower/🖼️assets/` carries `🗣️tower.dsl.semio` (168 KB, the real content) plus
`🔧️tower.op.semio` (285 B), `🎒️tower.pack.semio` (270 B), `📡️tower.spr.semio` (267 B).
**The three non-DSL assets are ~270 bytes each — they cannot contain a 180-part design.** They are
placeholders or single-op samples, not full-snapshot encodings. There is **no `tower.json`**.

Consequence for the plan's §2.2 shared-snapshot reference: a `component.ref.json` pointing at this
example currently resolves to only ONE real encoding (DSL). The harness requirement that "all three
encodings decode to the same typed snapshot" cannot be met until `tower.pack.semio` and
`tower.json` are generated from the DSL. That generation is S06's Wave-3 task and is a
**prerequisite** of the proof fixture, not a parallel activity.
