# Norm composed-child persistence and member wiring — current-source audit

**Verdict: RED — EN 1990 `q_k` and DIN 18599 `climate` are local-process caches, not durably materialized composed children.** This is a source audit only; no build or runtime gate was run.

## Decisive current boundary

`ArtifactChild<S>` deliberately serializes only `child_id` and `target`; `local_owner` is skipped from serde, DSL, pack, equality, and debug ([store `🦀️.rs:2626-2628`, `2649-2657`, `2749-2774`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs)). That is correct framework behaviour: a child is its own envelope/store, and a parent diff must not embed the child payload ([`🦀️.rs:2599-2604`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs)). It means a caller must create/load an actual child store before reading it.

Norm does the opposite today:

- EN 1990 makes a `DefaultHasher`-named handle with a process-only `En1990QkWorkingTable`; its target id is the unrelated constant `en1990-qk` ([`📘️en1990/🦀️.rs:63-94`](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🦀️.rs)). On a reloaded handle, `en1990_qk` returns an empty table, and inferences, outline, inverses, and all five variable-action mutations consume that fall-through (for example [`💡️inferences/🦀️.rs:100-104`](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs)).
- DIN 18599 does the same with `Din18599ClimateWorkingData`, a `DefaultHasher` id, and a constant target id `din18599-climate`; reloaded climate silently becomes two all-zero 12-month arrays ([`📙️din18599/🦀️.rs:95-126`](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🦀️.rs)). Its table converter additionally maps missing/invalid cells to `0.0` ([`📙️din18599/🦀️.rs:72-92`](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🦀️.rs)).
- Both snapshot codecs intentionally round-trip only a `[child_id,target]` handle ([EN `📸️snapshot/🦀️.rs:43-76`](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs), [DIN `📸️snapshot/🦀️.rs:49-82`](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs)). Their existing `child-owner-isolation` fixtures explicitly expect `wireHasPayload: false`. Those are cache-isolation tests, not persistence tests.

The silent defaults make this correctness-critical, not merely a missing restore enhancement: a malformed/missing composition must reject or remain non-ready; it must never create a valid-looking empty action table or zero climate calculation.

## Missing materialization and app ownership

The schema derives correctly declare slots (`qK` from `q_k`, `climate`), but Norm has **no** `ArtifactRefs` implementation and no call to `sync_composition_graph`; the generic app explicitly says it is not automatic ([plugin `🦀️.rs:20909-20927`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs)). Slot metadata alone neither creates a child envelope nor populates `VcsArtifactApp.children`.

The actual transport is separate and already exposes the necessary boundary:

1. `LoadDocument` loads only the parent.
2. `LoadChildren` parses each external `ChildPackEntry` and calls `load_child_pack` ([plugin `🦀️.rs:31263-31295`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs)).
3. `child_packs` emits only entries in the app's live `children` map ([`🦀️.rs:24487-24504`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs)).

Norm registers every one of its 30 apps with the default `NoMembers` builder, including the two composed families ([Norm root `🦀️.rs:11-42`, `101-108`](../../../../../../../../✏️s/🔌️plugins/📕️norm/🦀️.rs)). Thus no current Norm factory can open, retain, read, or return a child `SemioMembers` store. `editor_with_members` exists ([builder `🦀️.rs:484-512`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs)); there is no corresponding `viewer_with_members`, so an editor-only conversion would still leave read-only restore broken. The enum has to carry the concrete member fleet for all four EN/DIN editor/viewer variants.

## Exact factory-coordinate defect

Even after adding member-aware factories, current generic loading cannot open a stdio table:

- `VcsArtifactApp::open_child` calls `M::open(&dialect.artifact_kind, envelope_pack)` ([plugin `🦀️.rs:19858-19895`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs)).
- `MemberFactory::open` takes only an arbitrary `kind` string, not the requested full dialect ([store `🦀️.rs:17802-17810`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs)).
- `SemioMembers` is intentionally discriminated by the **subset** (`"table"`), because all 18 variants share kind `s.stdio.semio` ([stdio `🦀️.rs:1332-1361`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs)). Its own helper recovers the subset from the persisted envelope before calling `open` ([`🦀️.rs:1370-1386`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs)).

Generic `LoadChildren` therefore supplies `"s.stdio.semio"` to a factory that has `"table"`, not a table discriminator. It must fail, and the generic call does not compare supplied parent handle, supplied coordinate, and persisted envelope coordinate. The same defect exists on genesis: the coordinator calls `Mc::create(&spec.dialect.artifact_kind, ...)` ([store `🦀️.rs:19160-19172`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs)).

## Why a Norm-only cache patch cannot close this

The framework already has the right atomic primitive: `ChildGenesis` + `dispatch_group` creates the child first, applies the parent second, stamps ownership, and returns the new member ([store `🦀️.rs:18562-18594`, `19055-19231`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs)). It derives one child id from `parent_id + slot + parent-op fingerprint + ordinal`, and that exact id must serve both the child envelope and the parent's handle ([`🦀️.rs:18834-18849`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs)).

But current `dispatch_emit_group` hard-codes `genesis: Vec::new()` and documents that `ChildEmit` targets an already-live child ([plugin `🦀️.rs:20749-20793`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs)). Existing Norm diffs merely construct a new local-only handle. `register_child` can adopt a separately-built member, but it cannot atomically create that member and commit a parent snapshot containing its coordinator-minted handle ([`🦀️.rs:19904-19942`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs)).

Do not replace this with content hashes, `DefaultHasher`, a target constant distinct from `child_id`, an inline parent table, a post-commit `register_child`, or a local-owner cache. Each breaks either deterministic identity, atomic publication, reload, or failure semantics.

## Smallest dependency-ordered repair

### P0 — close factory dialect authority in the framework

Change `MemberFactory::{create,open}` and the generated `space_members!` implementation to receive/validate a complete `ArtifactDialect`, not a caller-selected kind string. `SemioMembers` must close over exactly `{artifact_kind: "s.stdio.semio", standard: "v1", subset: "table"}` for this path and verify the persisted envelope's coordinate is identical. The generic VCS `open_child` and coordinator genesis call must pass the full coordinate. No fallback from kind, subset, or old coordinate is permitted.

Focused framework/stdio laws:

- generic `VcsArtifactApp<_, SemioMembers>::LoadChildren` opens a genuine table child;
- wrong kind, standard, subset, malformed/missing persisted composition dialect, and supplied-vs-envelope coordinate mismatch all fail before map/graph/publication admission;
- the same closed coordinate is used for genesis creation.

### P1 — first-class owned-child preparation and member-aware roles

Expose one framework operation that prepares an owned child before parent-op serialization, returning the coordinator-minted id/handle for the parent mutation and carrying `ChildGenesis {slot,dialect,initial_pack}` through the existing atomic group. It must call `absorb_created_children` only after group success and then sync the parent `ArtifactRefs` graph. This resolves the present circularity: the parent op needs the minted handle, while the mint currently hashes the parent op.

Add `viewer_with_members` as the exact viewer twin of `editor_with_members`, then make the four `NormApps` EN/DIN role variants `VcsArtifactApp<..., semio_s_plugin_stdio::...::SemioMembers>` and use those two member builders in `plugin()`. Add `ArtifactRefs` implementations for the two snapshots returning the exact `qK`/`climate` handles. Parent activation/readiness must wait until the loaded child set equals those declared refs—no duplicates, missing entries, extra entries, wrong slot/id, wrong coordinate, or envelope mismatch—and no calculate/render/mutate path may substitute data while that boundary is incomplete.

### P2 — deterministic table schema bindings and strict reads

Move the EN/DIN converters behind an exact binding owned by each child slot:

- EN has exactly columns `category: Str`, `value: Float`, each row exactly two cells, finite numeric value, and an explicit bounded row limit.
- DIN has exactly columns `thetaEC: Float`, `gHWM2: Float`, exactly 12 rows, each row exactly two finite floats, and explicit physical-range validation owned by the standard schema.

Both readers return a typed materialization/validation error rather than empty/default output. Every variable-action and climate mutation must create/update the real child through the prepared/group route, never use `with_local_owner`. Existing working-scene fixtures must be replaced by restore fixtures; their asserted `wireHasPayload: false` remains true only for a bare handle, never as a successful document restore claim.

## Neutral oracle and acceptance

Add one language-neutral fixture for each parent with its parent pack plus a separate `ChildPackEntry`. It must assert relationship invariants rather than a hard-coded dynamic id: parent handle id == entry id == child envelope id; slot is `qK`/`climate`; exact stdio table coordinate; child owner points to the parent/slot/id; table decodes to the chosen values. A Node/Bun oracle independently rejects extra/missing/duplicate entries, wrong slot/id/coordinate, wrong envelope id/owner, truncated/unknown/malformed table, non-finite values, EN column order, and DIN non-12 row count.

The Rust process law for each EN/DIN role must:

1. invoke real `plugin()` and create its real app;
2. create parent plus exact child, persist the two envelopes separately, and `ReadChildren` exactly once;
3. create a fresh app, load parent then child entries, prove table values survive and a computation/mutation sees them;
4. prove a changed child is committed atomically with its parent handle and is visible after a second fresh reload;
5. prove any hostile case reaches a fault/non-ready boundary with no child map, graph edge, or public content publication.

The independent fixture oracle and exact Rust cases belong in a new permanent Norm `composed-child-source` / `composed-child-test` script route in [`📦️packages/🦀️rust/📜️script.ts:95-273`](../../../../../../../../✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts), with matching targets in `📦️packages/🦀️rust/📋️project.json`. Register their developer commands in the authored [`.vscode/🧩️launch.seed.jsonc:1916-1962`](../../../../../../../../.vscode/🧩️launch.seed.jsonc) beside the existing Norm gates, then run `bun nx run @semio-tech/plugin-registry:generate` and `...:check-generated`; `.vscode/launch.json` is generated and must never be hand-edited. The existing `surface-render-*`, `config-mutation-*`, and generic framework genesis tests do not traverse this EN/DIN restore path.

## Acceptance and nonclaims

Acceptance of this packet is bounded to two Norm composed slots and the necessary generic full-dialect/member/genesis plumbing. It establishes durable EN/DIN table child reload and fail-closed use. It does **not** claim every `#[child]` field in the repository is wired, generic child garbage collection is complete, network/hub document transport is proven, or a production WGPU render has been run.
