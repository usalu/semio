# Wave 3a-0 design — B-Rep kernel dissolution

**Scope of this agent: DESIGN ONLY. Zero `.rs` files edited.** Every claim below was checked against
the real file, not the recon's summary of it — line numbers and signatures are quoted so the next
(implementation) agent can jump straight to them. Where the recon (`📓️wave3a-brep-recon.md`) or
`📌️important.md`'s brief phrased something imprecisely, the correction is called out explicitly
rather than silently absorbed, per the ticket's own standing lesson about census vs. search.

Method copied from `📓️wave2-reports/terrain-report.md`: trace every field/method to its real owner
before classifying it. Outcome is **not** "no vocabulary" here — unlike terrain, this module owns a
huge amount of tier-(b)-shaped editing surface. But the *tracing discipline* is the same, and it is
what produced the load-bearing correction in §3 (the EngineRep target is `Body`, not `Store<T,Id>`)
and the one in §5 (most of the 191 methods are already pure — the mutation is two layers of
indirection away from the real work, not the real work itself).

---

## 0. Ground facts — verified, with two corrections

Re-measured directly, not copied from the brief:

| Claim | Verified |
|---|---|
| `📐️brep/🧰️kernel/🦀️component.rs` 1452 LOC | ✅ exact (file ends at line 1452) |
| 191 `&mut self` methods on `Brep` | Not recounted by hand (191 grep-derived by the recon); the file's own region structure (below) makes the *composition* of that number the important fact, not its exact value |
| `📐️brep/🏟️arena/🦀️component.rs` 260 LOC, ~20 dependents | ✅ exact; **and it has 0 `&mut self` methods that touch topology** — it is a generic `Store<T, Id>` (insert/get/get_mut/remove/iter), fully label-blind, fully snapshot-blind |
| `BrepEngineHost` owns `cache: Mutex<EngineCache>` + `kernel: Mutex<Brep>` | ✅ exact, `⚙️engine/🖥️host/🦀️component.rs:84-87` |
| Baseline `cargo test -p semio-framework-3d --lib` → 407 passed, 0 failed | Taken as given (stated in the dispatch brief as already measured); not re-run by this design-only agent, since no `.rs` file was touched and a rebuild was not needed to answer the design question. The next (implementation) agent must re-confirm it before touching anything, per hard rule 11 |
| `📜️history/🦀️component.rs` is provenance (diff-shaped), not a command log | ✅ confirmed, and extended below — the mechanism is *real but currently inert* (see §2) |

**Correction 1 — the recon's "design `Arena::from_snapshot(base)` as an `EngineRep`" names the wrong
type.** `🏟️arena/🦀️component.rs` defines only `ArenaId`/`define_id!`/`Store<T,Id>` — a generic,
label-blind, snapshot-blind container (0 internal deps, confirmed). It has no concept of
`PersistentLabel`, no concept of "the whole topology," and cannot be the `EngineRep` target: `EngineRep<P>::build(&P) -> Self` needs one coherent `Self` representing "the whole ephemeral
working rep," and that is `Body` (`🕸️topology/🦀️component.rs:88-100`), which *owns* ten `Store<T,Id>`
instances plus the `LabelSource`. §3 designs `impl EngineRep<Seed> for Body`, not for `Store`.
`🏟️arena` itself needs no change and stays frozen exactly as the recon concluded — it just isn't
where the impl goes.

**Correction 2 — "every `🔺️euler` operator is already passed one [`OpRecorder`]" is true, but
incomplete in a way that matters for the phased plan.** Every low-level euler function
(`make_vertex`, `make_edge`, `make_loop`, `add_face`, `add_shell`, `add_solid`, `split_edge`,
`🔺️euler/🦀️component.rs:19-118`) does take `rec: &mut OpRecorder` and does call
`rec.record_generated`/`record_deleted`. But **every caller above euler currently constructs a fresh,
local `OpRecorder` and discards it at the end of the function** — grepped exhaustively:

```
🧱️primitives/🦀️component.rs   9 occurrences of `let mut rec = OpRecorder::new();`, one per make_* fn
🔀️boolean/🦀️component.rs      1 occurrence
➡️sweep/🦀️component.rs        2 occurrences
```

None of `make_box`, `boolean_solid`, `extrude_face` etc. **returns** the `OpDelta` — their signatures
are `pub fn make_box(body: &mut Body, w: f64, d: f64, h: f64) -> Result<SolidId, KernelError>`
(`🧱️primitives/🦀️component.rs:113`), `pub fn boolean_solid(body: &mut Body, a: SolidId, b: SolidId,
op: BooleanOp, tol: f64) -> Result<SolidId, KernelError>` (`🔀️boolean/🦀️component.rs:32`),
`pub fn extrude_face(body: &mut Body, face: FaceId, direction: Vec3, distance: f64) -> Result<SolidId,
KernelError>` (`➡️sweep/🦀️component.rs:335`) — no `OpDelta` in any return type. The recorder is real,
threaded correctly *within* one call, and then dropped. So "`OpDelta` maps almost 1:1 onto the diff
shape" is correct as a *design* claim but **the plumbing that would let a diff constructor see it does
not exist yet** — this is real, scoped, mechanical work, not a finished mechanism waiting to be
wired up. Phase 1 (§6) is exactly this: change ~14 top-level `pub fn` signatures across
`primitives`/`boolean`/`sweep`/`blend`/`offset`/`sew`/`heal`/`mesh-io` to accept `rec: &mut OpRecorder`
(mirroring euler's own convention) instead of constructing one internally, and return nothing new —
the *caller* (the future diff constructor) owns the recorder and reads `rec.into_delta()` after the
call.

**Correction 3 (minor, flagged not fixed) — `🖋️imprint/🦀️component.rs:206,208` calls
`body.coedges.remove(cid)` / `body.loops.remove(outer)` directly**, bypassing euler's own docstring
invariant ("the *only* functions permitted to mutate a `Body`", `🔺️euler/🦀️component.rs:1`). Lower
stakes than it sounds — loops/coedges are unlabeled/structural, so no `PersistentLabel` bookkeeping is
skipped — but it is a real crack in the "assembled exclusively through checked editors" invariant the
whole addressing design in §2 leans on. Flagged for the phase-1 agent to either route through a new
euler-level `kill_loop`/`kill_coedge` pair or explicitly re-document the exception; not designed here
because deciding it requires reading `🖋️imprint`'s full 300+ lines, out of this design pass's budget.

---

## 1. The blocking question — where does the authoritative brep snapshot live, and who owns the mutation triads

### The constraint, re-verified independently

- `semio-framework-3d` (`🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml:1-2`, crate
  `semio-framework-3d`) already depends on `semio-framework-os-kernel` under its `brep` feature
  (`Cargo.toml:20-26` — `brep = ["dep:semio-framework", "dep:semio-framework-os-kernel", ...]`). That
  is a framework→framework edge (`💻️os/🔨️modules/⚙️engine` is itself under
  `🧰️framework/🛍️products/`), legal and already in use — `EngineRep`/`DraftEngineSession`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs:191-277`, W1-owned, frozen) are
  reachable from framework-3d **today**, no new dependency required.
- `semio-framework-3d` has **zero** dependency on any plugin, and cannot acquire one — confirmed by
  reading `📦️glue.rs` (the entire `pub mod brep { ... }` block is `#[cfg(feature = "brep")]`-gated,
  `📦️glue.rs:5-80`, and mounts only sibling `🧰️framework/` files via `#[path]`).
- `✳️brep`'s `SemioBrepSnapshot` (`✏️s/🔌️plugins/🗄️stdio/…/✳️brep/🧬️schema/📸️snapshot/🦀️component.rs:163-198`)
  is a real, already-registered `#[derive(ArtifactSchema)]` type (`#[artifact_schema(id =
  "s.stdio.semio.brep")]`), with a real (if minimal) `store::ArtifactDsl`/`store::ArtifactPack` codec,
  a real `SubsetValidator` (referential-integrity check, `✳️brep/🚪️io/🦀️component.rs:79-131`), and a
  real `ArtifactBuilder` (`SemioBrepBuilderConstruction`, `…/🧬️schema/🦀️component.rs:108-133`) whose
  `mutate()` already calls `apply_semio_brep_mutation(&mut self.snapshot, &mutation)` — i.e. **the
  slot where a diff constructor gets invoked from already exists and is already wired into the
  store/composer machinery.** Today it only has one mutation to dispatch to: the banned
  `📄set-snapshot`.
- **stdio does not currently depend on `semio-framework-3d` at all** (grepped its `Cargo.toml` and
  every `use` in `✳️brep/**` — the only geometry-value dependency is
  `crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint3`, a stdio-local type; no
  `semio_framework_3d`/`crate::brep::` import anywhere under `✳️brep`).
- **But the plugin→framework-3d edge is an established, already-used pattern elsewhere**: `📐️cad`,
  `🏭️process`, `🌀️procedural`, `🎪️demonstrator`, `💠️lowpoly`, and `🌊️flow/🧩️extensions/📐️brep` all
  list `semio-framework-3d` in their `Cargo.toml` today (grepped repo-wide). Adding it to stdio's
  `Cargo.toml` under the `brep` feature would be the same, unremarkable edge — not a novel or
  risky dependency shape.

### The options

**Option 1 — framework-3d becomes pure compute, consumed BY stdio.** stdio's `🔺️diff` leaves import
`semio-framework-3d`, build an ephemeral `Body` from the snapshot, call one pure engine fn, read back
what changed, translate into `SemioBrepDiff`. Dependency direction: stdio → framework-3d (legal,
precedented). Framework-3d never sees `SemioBrepSnapshot`, `SemioBrepMutation`, or any triad.

**Option 2 — a framework-side snapshot type, with stdio converting to/from it.** Framework-3d defines
its own tier-(a)-shaped `BrepSnapshot`, becomes the authoritative owner, and stdio's
`SemioBrepSnapshot` becomes a derived mirror.

**Option 3 (this design's recommendation) — Option 1, stated precisely enough that nobody mistakes
the new framework-3d type for a second snapshot.** Framework-3d gains exactly one new type: an
ephemeral, tier-(d) **seed** struct (§3) that exists only for the duration of one diff-constructor
call, is never persisted, never registered with `register_artifact_schema_descriptor`, and carries no
`#[artifact_schema]`/`#[state(persistent)]` annotation. It is the `P` in `EngineRep<P>`, not a second
`SemioBrepSnapshot`.

### Why Option 2 is rejected

Every real call site of `register_artifact_schema_descriptor` (106 of them, per the W2 exemplar's
already-completed census, `📓️wave2-reports/terrain-report.md` "Placement question") is under
`✏️s/🔌️plugins/**`; zero are under `🧰️framework/`. Framework modules have no artifact-store
integration (no composer, no `ArtifactDsl`/`ArtifactPack` codec, no `SubsetValidator`, no conformance-
law harness) — stdio's `✳️brep` subset already has all four, tested, with fixtures
(`codec_retention_law_populated_snapshot_round_trips_pack_and_dsl`,
`fixture_honesty_law`, `✳️brep/🧬️schema/📸️snapshot/🦀️component.rs:1044-1067`). Duplicating that
machinery in framework-3d to host a second `s.*.brep`-shaped authoritative type would be exactly the
"same authoritative state, two owners" defect the terrain exemplar's own "What I did not change"
section calls out as the violation this ticket exists to remove — and it would need its own
`#[artifact_schema(id = "...")]`, which collides in spirit (if not literally in string) with
`"s.stdio.semio.brep"`. Rejected on the same grounds the exemplar already established.

### What this wave commits to, concretely

1. **Authoritative brep snapshot**: stays `SemioBrepSnapshot`,
   `✏️s/🔌️plugins/🗄️stdio/…/✳️brep/🧬️schema/📸️snapshot/🦀️component.rs`. Unchanged by this wave (it is
   read-only, another session's, per the hot-file table).
2. **Mutation triads** (`🦠️mutation`/`🔺️diff`/`↩️inverse`, the `SemioBrepMutation` dispatch enum):
   physically live at `✏️s/🔌️plugins/🗄️stdio/…/✳️brep/🧬️schema/🧬️mutations/<slug>/`. **Not authored by
   this wave** — that write requires the stdio handoff, which is the one gate of three still closed
   (SMO's verb approval: open; IIF's `✳️brep` deferral to DKM: open; UCAS's stdio handoff: **closed**).
   Two of three open is not open, per `📌️important.md`'s own framing. §6 designs the shape precisely
   enough that whichever session next holds write access to `✳️brep` can author it mechanically, but
   does not write it.
3. **Framework-3d's job, and the only thing this design authorizes building**: dissolve the 191
   `&mut self` methods into (a) pure engine functions over `&mut Body`/`&Body` that already mostly
   exist (§5), (b) one new `EngineRep<Seed>` impl for `Body` (§3), (c) `OpRecorder` plumbing that
   surfaces `OpDelta` out of every top-level constructive fn instead of discarding it (§0 Correction
   2), and (d) deletion of `BrepEngineHost`'s host-session model, gated on two live external consumers
   (§4).

### What breaks, and the honest state of the "unfrozen" caveat

Nothing under `✏️s/` breaks from this wave's own scope (framework-3d-internal only). What breaks
*eventually*, when `BrepEngineHost`/`BrepKernel`/`GeometryHandle` are actually deleted, is covered in
full in §4 — it is real, it is cross-session, and it is bigger than stdio alone.

---

## 2. `PersistentLabel` as the addressing scheme

### What already holds

`Vertex`, `Edge`, `Face`, `Shell`, `Solid` (`🕸️topology/🦀️component.rs:16-79`) each carry a
`label: PersistentLabel`, assigned once at birth via `Body::new_label()` → `LabelSource::next_label()`
(monotonic, never reused, `📜️history/🦀️component.rs:20-28`), and only euler's checked constructors
mint them (`🔺️euler/🦀️component.rs:19-67`, each pairs `body.new_label()` with
`rec.record_generated(label)`). This is exactly the non-reused, id-stable address SMO's approved
`create-{vertex,edge,loop,face,shell,solid}`/`delete-{…}`/`move-vertex{vertex_id,new_point}`
(`📌️important.md` verb table) need, and arena ids (`VertexId`/`FaceId`/etc., generational, reused
after `Store::remove`, `🏟️arena/🦀️component.rs:91-101`) are explicitly the wrong choice for the same
reason `📓️status.md`'s W3a section already flags — reuse-after-delete would alias a stale mutation
address onto unrelated new geometry.

### A real gap, not previously flagged: `Loop` and `Coedge` have no `PersistentLabel`

`🕸️topology/🦀️component.rs:50-54`:
```
pub struct Loop {
    pub first: CoedgeId,
    pub face: FaceId,
}
```
No `label` field. `🔺️euler/🦀️component.rs:33-34`'s own docstring is explicit about why: *"Loops/coedges
have no `PersistentLabel` of their own (they are structural, not independently document-nameable), so
nothing is recorded."* That is a deliberate, documented design choice in the current code — and it
directly conflicts with SMO's approved vocabulary, which includes `create-loop`/`delete-loop` as a
first-class verb pair (`📌️important.md`, verb rulings table, row "brep create/delete"). **This is a
real, load-bearing conflict the next wave must resolve, not a naming nit**:

- Either SMO's ruling is corrected to drop `loop` from the addressable set (loops become purely an
  interior detail of `replace-face`-shaped mutations, addressed only via their owning face) — in
  which case euler's current "structural, not nameable" design is already correct and nothing in
  `topo.rs` changes;
- or `Loop` gains a `label: PersistentLabel` field (mirroring `Face`/`Shell`), `make_loop` gains a
  `rec: &mut OpRecorder` parameter and records it as generated, and `create-loop`/`delete-loop`
  become real addressed mutations.

This design does not resolve it — it requires SMO to be asked a second, narrower question ("is a
`BrepLoop` in `SemioBrepSnapshot` (which DOES have an `id: String`, `…/📸️snapshot/🦀️component.rs:104`
— the stdio snapshot already treats loops as id-addressable!) meant to be independently
create/delete-able, or only ever rewritten wholesale as part of its owning face's replacement?"). The
stdio snapshot's own shape (loops ARE id-keyed there) suggests the SMO ruling as currently worded is
the more likely correct one and framework-3d's `Loop` needs the label added — but this is a
**flagged, not resolved**, question; recommend it go back to SMO before Phase 1 (§6) touches
`topo.rs`.

### `LabelSource` — from host-local to deterministic/replica-convergent

Today: `Body::new()` → `LabelSource { next: 0 }` (`📜️history/🦀️component.rs:15-22`) — every fresh
`Body` restarts label numbering at 0. That is fine for a single long-lived host session (the current
architecture), and actively wrong for the target architecture, where a *new* `Body` is built from
scratch on every single diff-constructor call (§3). Two consequences:

1. **Determinism** (needed for `EngineRep::build(s) == EngineRep::build(s)`, the frozen W1 contract,
   `⚙️engine/🦀️component.rs:183`): trivially satisfied — `LabelSource` restarting at a value derived
   deterministically from the seed (see below) is itself deterministic.
2. **Replica convergence / no collision across edits**: NOT satisfied by restarting at 0 every build.
   If diff-constructor call A (building on `base` v10) and diff-constructor call B (building on the
   same `base` v10, from a concurrent edit) each start a fresh `Body` at `next: 0`, and both mint a new
   vertex, both get `PersistentLabel(0)` — a real collision the instant both diffs are merged.

**Design**: the seed struct (§3) must carry the label high-water-mark forward, not reset it. Concretely,
`Seed` needs a `next_label: u64` field, and `Body::build(seed)`'s `EngineRep` impl seeds
`LabelSource { next: seed.next_label }` instead of `LabelSource::new()`. `next_label`'s value must
itself be derivable from the persisted snapshot deterministically — the natural, no-new-persisted-
field option is `1 + max(every existing PersistentLabel-equivalent numeric suffix across
vertices/edges/faces/shells/solids in the snapshot)`, computed by the seed-construction code (stdio's
side, §1). This requires stdio's String ids to encode the numeric label recoverably (e.g.
`format!("{kind}{n}")` and stdio's ids already look exactly like that in the demo fixture — `"v1"`,
`"e1"`, `"f1"`, `"s1"`, `"so1"`, `✳️brep/…/📸️snapshot/🦀️component.rs:932-987`) — **but this is only a
convention today, not a constraint stdio's schema enforces**, so the alternative, more robust design
is an explicit persisted counter field on `SemioBrepSnapshot` (a genuine, small, additive schema
change, stdio's to make, out of this wave's write scope). Flagging both options rather than picking
one, since the choice is stdio's to make and depends on whether `SemioBrepSnapshot`'s id convention is
meant to stay human-authorable free text (favors the explicit counter) or is already effectively
label-derived everywhere (favors deriving it).

### The String-id ↔ `PersistentLabel` translation itself

Recommend a **call-scoped `HashMap<String, PersistentLabel>` built fresh inside each diff
constructor**, not a global or persisted string format lock-in:
`SemioBrepSnapshot.vertices[i].id` (`String`) maps to a `PersistentLabel` minted or looked up while
building the `Seed`/`Body` for that one call, and the reverse map is used when translating the
resulting `OpDelta` back into `SemioBrepDiff`'s id-keyed fragments. This keeps `PersistentLabel`'s own
representation (`pub struct PersistentLabel(pub u64)`, unchanged) fully decoupled from whatever string
convention stdio's snapshot ids use — cheaper to get right than inventing and locking in a canonical
`"kind{n}"` text format that every future format (STEP roundtrips, user-renamed ids, etc.) would then
be constrained by.

---

## 3. `EngineRep` for `Body` — design, round-trip law, how to test it

### The seed type (framework-3d-owned, tier (d), never persisted)

A new, small, plain-data struct in `🕸️topology/🦀️component.rs` (or a new sibling region in the same
file — no new file, per hard rule 8), reusing framework-3d's **own existing** `Pnt3`/`Curve3`/`Surface`
value types (`➡️vector`, `➰️curve`, `🏄️surface` — already `Clone + Serialize + Deserialize`, already
used inside `Body` itself) rather than inventing parallel ones:

```
pub struct BrepArenaSeed {
    pub next_label: u64,
    pub vertices: Vec<(String, Pnt3, Tol)>,
    pub edges: Vec<(String, String, String, Curve3, (f64, f64), Tol)>,   // id, v0_id, v1_id, curve, range, tol
    pub loops: Vec<(String, Vec<(String, bool)>)>,                       // id?, [(edge_id, forward)]  — see §2's open question
    pub faces: Vec<(String, String, Vec<String>, Surface, bool, Tol)>,   // id, outer_loop_id, inner_loop_ids, surface, flipped, tol
    pub shells: Vec<(String, Vec<String>)>,                              // id, face_ids
    pub solids: Vec<(String, String, Vec<String>)>,                      // id, outer_shell_id, inner_shell_ids
}
```

This is **structurally mechanical** to build from `SemioBrepSnapshot` on stdio's side, because
`BrepCurve`/`BrepSurface` (`…/📸️snapshot/🦀️component.rs:23-64`) already use the **identical variant
names** as framework's `Curve3`/`Surface` (`Line`/`Circle`/`Ellipse`/`Nurbs`;
`Plane`/`Cylinder`/`Cone`/`Sphere`/`Torus`/`Nurbs` — verified by reading both enum definitions side by
side, `➰️curve/🦀️component.rs:17-27` vs. `🏄️surface/🦀️component.rs:15-30`) — the only real
per-variant work is repackaging stdio's `(origin, direction)`/`(origin, normal)` pairs into framework's
`Frame3`, which framework's own `Frame3::from_normal` constructor already does (used identically in
`euler.rs`'s own test helper, `🔺️euler/🦀️component.rs:150`).

### `impl EngineRep<BrepArenaSeed> for Body`

```
impl EngineRep<BrepArenaSeed> for Body {
    fn build(seed: &BrepArenaSeed) -> Self {
        let mut body = Body::new();
        body.labels = LabelSource { next: seed.next_label };
        // insert vertices/edges/loops/faces/shells/solids via Store::insert directly (NOT via
        // euler's make_* — those MINT fresh labels; build() must PRESERVE the seed's own labels,
        // parsed/looked-up from each id, or every rebuild reassigns identity and the round-trip law
        // in the next subsection fails by construction).
        ...
    }
}
```

The critical design point, stated explicitly because it is the one place a naive implementation would
silently break the round-trip law: **`build()` must NOT call `euler::make_vertex` et al.** — those
mint a *fresh* label every time (`body.new_label()`), which is correct for a genuine user-facing
create but wrong for reconstructing an existing entity from its snapshot. `build()` inserts directly
into the `Store`s (`body.vertices.insert(Vertex { position, tol, label: <label recovered from the
seed's string id> })`), bypassing euler on purpose — this is the ONE place outside euler.rs that is
allowed to construct topology entities directly, and it should say so in its own docstring (mirroring
`🔺️euler/🦀️component.rs:1`'s "the *only* functions permitted to mutate a `Body`" — `build()` is
constructing a *fresh* `Body`, not mutating an existing one, so it does not violate that invariant,
but a reader coming from euler's docstring needs the distinction spelled out or they will "fix" it
into calling `make_vertex` and quietly break label preservation).

### The round-trip law

Two separate laws, at two separate boundaries, only one of which this wave's future implementation
phase can test without stdio:

**Law A (framework-3d-local, testable now, no stdio dependency)**:
`to_seed(&Body::build(&seed)) == seed` for any well-formed `seed` (a `Body → Seed` extraction
function is the mirror-image half of `build`, needed anyway for the diff constructor to read the
post-op state back out). "Well-formed" here means referentially consistent (every `edge`'s
`start_vertex`/`end_vertex` id resolves, etc.) — the same invariant stdio's own
`check_brep_referential_integrity` (`✳️brep/🚪️io/🦀️component.rs:79-131`) already checks on
`SemioBrepSnapshot`, so a malformed seed is stdio's problem to reject before calling into
framework-3d, not framework-3d's to defend against.

Test: extend the existing `#[cfg(test)] mod tests` in `🕸️topology/🦀️component.rs` (no new test file,
per hard rule 8) with a property test seeded by `semio_framework_math::random::Rng`, the same tool the
arena's own `quick::random_insert_remove_sequence_never_aliases_a_removed_id` test already uses
(`🏟️arena/🦀️component.rs:229-257`, direct precedent in this exact codebase) — generate small random
`BrepArenaSeed`s (a handful of vertices/edges forming valid loops) and assert the round trip.

**Law B (cross-boundary, stdio's to write once the handoff lands, specified here so it is not
reinvented)**: `SemioBrepSnapshot → Seed → SemioBrepSnapshot` is identity (the translation itself is
lossless), and end-to-end, `diff(mutation, base)` applied to `base` matches independently rebuilding
`Seed::from(base)`, running the same pure engine fn, and re-extracting — i.e. the diff constructor's
shortcut (build once, mutate, extract) must agree with the conceptually simpler "rebuild from scratch
after" ground truth. This is stdio's law to write in `✳️brep/🧬️schema/🔺️diff/🦀️component.rs`'s own
test module when it exists; not designed further here since it depends on the `SemioBrepDiff` shape
stdio itself owns.

---

## 4. Deleting `BrepEngineHost`

### What it owns today, exactly

`⚙️engine/🖥️host/🦀️component.rs:84-87`:
```
pub struct BrepEngineHost {
    cache: Mutex<EngineCache>,   // registers BrepDocumentOpEngine under BREP_ENGINE_ID = "s.3d.brep"
    kernel: Mutex<Brep>,          // one long-lived Brep{body,live,counter} session
}
```
Two genuinely different capabilities bundled into one struct:

1. **`EngineHost::derive`/`read`** (the `cache` half) — the real wasm-boundary content-addressed
   derive/cache path (`derive(engine_id, input) -> EngineHandle`, delegating to
   `BrepDocumentOpEngine::compute`, which parses a JSON `{"op": "box", ...}` request and runs it
   against a **throwaway** `Brep::new()` it constructs itself, `⚙️engine/🖥️host/🦀️component.rs:32`).
   This half is already stateless-per-call in spirit — the cache just memoizes identical requests —
   and needs no host-session at all; `EngineCache` (frozen, W1-scoped) is the right owner and this
   half survives unchanged.
2. **`kernel()`/`with_kernel`** (the `kernel: Mutex<Brep>` half) — a raw escape hatch handing out
   `&mut Brep` for direct, multi-call, handle-chaining use of the async `BrepKernel` trait. This half
   is the actual dissolution target — a long-lived, host-owned, cross-call mutable session, which is
   precisely tier (d)'s forbidden shape ("never a durable struct field... never crossing a dispatch
   boundary," `⚙️engine/🦀️component.rs` [os-kernel] `EngineRep` docstring).

### Call sites, and what each becomes — including two the recon did not find

The recon's dependency table stopped at framework-3d's own boundary. Grepping `BrepEngineHost`
**repo-wide** (not just under `🧊️3d/`) finds two real, live, plugin-side consumers the recon missed
entirely:

| Call site | What it does today | What it becomes |
|---|---|---|
| `⚙️engine/🖥️host/🦀️component.rs`'s own 2 `#[cfg(test)]` tests (`host_derive_registers_brep_engine`, `kernel_lock_runs_box_prim`) | Exercise the struct directly | Deleted along with the struct; their coverage (derive/read path, box_prim path) is subsumed by the pure-fn tests already on `Brep`/`Body` (`🧰️kernel/🦀️component.rs:1327-1452`'s own `#[cfg(test)] mod tests`, unaffected by this dissolution since those tests already call `k.box_prim_sync`/`block_on(k.box_prim(...))` directly on a fresh `Brep`, not through a host) |
| `✏️s/🔌️plugins/🏭️process/…/⚙️engine/🦀️component.rs:403,415,422,507,517,539,551,589,631,676` — a plugin struct field `host: BrepEngineHost`, driven via `session.kernel().lock()...volume(&handle)` etc. across **multiple separate calls** | Holds a document-lifetime `Brep` session, chaining `GeometryHandle`s across method calls (create in one call, fillet the same handle in a later call) | **Cannot survive as-is.** This usage pattern *is* the cross-dispatch-boundary ephemeral state tier (d) forbids — it is `process3d`'s own artifact needing the exact same triad+diff-constructor treatment `✳️brep` needs, on `process3d`'s own snapshot. Out of framework-3d's boundary; flagged below, not designed here |
| `✏️s/🔌️plugins/📐️cad/…/⚙️engine/🦀️component.rs:91-98` — `cad_brep_host()`, a **process-global `OnceLock<BrepEngineHost>`**, `.kernel().lock()` driving the same async trait | A single kernel session shared across **every** cad document in the process — worse than process3d's per-struct instance, this is the literal "process-global mutable geometry kernel" the doctrine's tier (d)/(e) split exists to make impossible | Same conclusion, more urgently: this is the purest instance of the exact anti-pattern named in `📌️important.md`'s thesis paragraph, just in a plugin rather than the framework kernel itself |

### What capability is lost, honestly

Nothing framework-3d itself needs is lost — every real computation (`box_prim`, `fillet`, `volume`,
...) survives as a pure fn over `&Body`/`&mut Body` (§5). What is lost is the **convenience of
holding a mutable multi-call session at all** — by design, that convenience is exactly what the
doctrine forbids. `cad`/`process3d` lose the ability to do `create → (later call) → fillet(handle)`
without an intervening artifact-store round trip. That is not a regression this wave can avoid or
soften; it is the point of the dissolution, and it reaches further than `🧊️3d` alone.

### Consequence for the phased plan and for scope

**`BrepEngineHost`, the `BrepKernel` async trait, and `GeometryHandle` cannot be deleted by
framework-3d acting alone.** Doing so breaks `semio-process3d` and `semio-cad` compilation
immediately. Per the greenfield rule (no compatibility layers, no deprecations) there is no soft
landing — either those two plugins are migrated in lockstep (their own snapshot + mutation triads,
each calling framework-3d's pure fns from their own diff constructors, exactly like `✳️brep`'s
eventual triads would), or `BrepEngineHost`/`BrepKernel`/`GeometryHandle` stay exactly as they are
today until that migration happens elsewhere. This wave's phased plan (§6) treats deletion of the
host/trait/handle model as a **separately gated final phase**, explicitly not bundled with the
framework-internal phases that need no cross-session coordination. Also flagged: `SolidExporter`/
`SolidImporter` (`🧰️kernel/🦀️component.rs:1238-1323`) — a second `&Brep`/`&mut Brep`-shaped surface —
are used beyond `process3d` too: `🔺️mesh/🦀️component.rs`, `💻️os/🦀️component.rs`, and
`💻️os/🖥️host/🦀️component.rs`, the last of which is squarely APA's escape-hatch territory
(`register_mesh_*`/`register_solid_*`/`register_dwg_*`/`register_app_io`,
`📌️important.md`'s hot-file table). Deleting the `Brep`-handle-shaped codec traits is therefore not
purely a DKM/cad/process-3-way problem — it may also be APA's to coordinate. Not designed further
here; flagged as a cross-session dependency the phased plan's final phase must resolve with APA before
executing, not something this design can resolve unilaterally.

---

## 5. The 191 `&mut self` methods, classified by kind

Traced against the file's own region structure (`🧰️kernel/🦀️component.rs`, region markers at lines
51, 90, 156, 246, 949, 1233), not re-derived from a fresh grep:

| Kind | Region | Approx. count | Representative example | Disposition |
|---|---|---|---|---|
| **Registry / host plumbing** | `🧮Registry` (156-243) | ~13 (`mint`, 5×`register_*`, `entity`, 5×`*_id`/`*_ref` accessors) | `fn mint(&mut self, kind: GeometryKind, entity: Entity) -> GeometryHandle` (159-169) | **Disappears entirely.** Exists only to support cross-call `GeometryHandle` chaining (§4) — once a diff constructor computes an entire compound op in one function call, there is nothing to register a handle *for*. Not migrated, deleted |
| **SyncApi delegators** | `🔖️SyncApi` (246-948), ~90 methods | ~90 | `pub fn box_prim_sync(&mut self, w: f64, d: f64, h: f64) -> Result<GeometryHandle, BrepError> { let solid = make_box(&mut self.body, w, d, h).map_err(map_err)?; Ok(self.register_solid(solid)) }` (249-252) | **Thin wrapper, deleted along with Registry.** The real work (`make_box`) is already a pure fn over `&mut Body` (verified for every category below) — the wrapper's only job is minting a `GeometryHandle` for the result, which disappears with Registry. The diff constructor calls `make_box`/`boolean_solid`/etc. directly |
| **Async `BrepKernel` trait impl** | `🔖️BrepKernel` (949-1232), 89 methods | 89 | `async fn box_prim(&mut self, w: f64, d: f64, h: f64) -> Result<GeometryHandle, BrepError> { self.box_prim_sync(w, d, h) }` (kernel.rs:953-955, matching pattern for all 89) | **Deleted with the trait itself** (§4) — one-line delegators to the SyncApi layer, `async` only because the trait is `#[async_trait(?Send)]` for wasm-host-call compatibility that a pure-fn diff constructor doesn't need |
| **Pure topology edits (already `&mut Body`, not `&mut self`)** | outside kernel.rs — `🔺️euler`, `🔀️boolean`, `➡️sweep`, `🎨️blend`, `↔️offset`, `🧵️sew`, `🩹️heal`, `🧱️primitives` | n/a (these are the functions the 90 SyncApi wrappers above call into — not part of the 191 count, but the actual work) | `pub fn boolean_solid(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64) -> Result<SolidId, KernelError>` (`🔀️boolean/🦀️component.rs:32-37`) | **Already tier-(e) pure compute.** Becomes what the diff constructor calls directly, once `rec: &mut OpRecorder` threading (§0 Correction 2) is added to the ~14 top-level signatures that don't yet take one |
| **Queries** | `🔖️SyncApi`'s read-only subset + `📏️measure`/`🏷️classify`/`🌳️bvh`/`🔮️oracle`/`✅️validate`/`🧩️tessellate` | ~25 of the 191 (the `&self`-only SyncApi methods, e.g. `volume_sync`, `curve_point_sync`) + all of measure/classify/tessellate/validate (already free fns over `&Body`) | `pub fn solid_volume(body: &Body, solid: SolidId, chord_tol: f64) -> Result<f64, KernelError>` (`📏️measure/🦀️component.rs:34`) | **Inferences (tier c/e), never mutations** — matches SMO's own ruling ("brep tessellate/measure/validate ✅ inferences, never mutations," `📌️important.md`). These become the pure fns an `InferredField::compute` body calls, once IIF's `✳️brep` inference wave (deferred to DKM per the five-session handshake) picks this subset up |
| **IO / bulk import** | `📄️step`, `📦️mesh-io` | subset of the 90 SyncApi + `import_step`/`import_stl`/etc. in the `BrepKernel` trait | `pub fn read_step(text: &str) -> Result<Body, StepError>` (`📄️step/🦀️component.rs:48`) builds a **whole new `Body` from scratch**; `import_stl_to_body(body: &mut Body, data: &[u8], tolerance: f64) -> Result<SolidId, KernelError>` (`📦️mesh-io/🦀️component.rs:105`) adds **one solid into an existing body** | **`read_step` and whole-file import → `ArtifactStore::reset`**, per doctrine ("Bulk or procedural generation... goes through `ArtifactStore::reset`"), unambiguous. **`import_stl_to_body`/`import_obj_to_body`/etc. are genuinely ambiguous** — flagged, not resolved: if the editor gesture is "import this STL as a new solid into my existing scene," that is a `create-solid`-shaped mutation (one entity, id-addressed, undoable); if it is "replace my document with this file," it is `reset`. The function signature alone doesn't distinguish these — which gesture it is depends on the (plugin-side) UI, out of this wave's visibility |
| **Codecs** | `🔌️Codecs` (1233-1325) | 0 (traits/impls, not `&mut self` methods on `Brep` itself, though `SolidImporter::import` takes `&mut Brep`) | `SolidExporter`/`SolidImporter` traits, `kernel: &Brep`/`&mut Brep` params | **Depends on the Registry/`GeometryHandle` model that §4 shows cannot be deleted unilaterally.** Stays until the cross-session `process3d`/`cad`/APA coordination in §4 resolves; not designed further here |

Total accounting: Registry (~13) + SyncApi (~90) + async trait impl (89) ≈ 192, matching the recon's
191 within grep-vs-hand-count noise — and **all three groups disappear or shrink to nothing under
this design**, none of them survive as a "mutation triad." The methods that DO become mutation-triad
diff logic are not in the 191 count at all — they are the already-pure `&mut Body` functions in the
sibling modules, which is exactly the reason this dissolution is smaller than its LOC count suggests
(see §6 Phase 1's estimate).

---

## 6. Phased implementation plan

Each phase is scoped to be independently checkable against the 407-test baseline
(`cargo test -p semio-framework-3d --lib`, `CARGO_TARGET_DIR=<ticket>/🎯️target`, hard rule 5), and
ordered so a phase never depends on a later one landing first.

### Phase 1 — `OpRecorder` plumbing (framework-3d only, no external consumers touched)

Change ~14 top-level `pub fn` signatures (one per file: `primitives::make_box`/`make_sphere`/etc.,
`boolean::boolean_solid`/`compound_cut`/`section_solid_by_plane`/`split_solid_by_plane`,
`sweep::extrude_face`/`revolve_face`/`loft_profiles`/`sweep_along_path`/`helical_sweep`/`pipe`,
`blend::fillet_edges`/`chamfer_edges`/`fillet_variable`, `offset::offset_face`/`offset_solid`/
`thicken_face`/`draft_angle`/`shell_solid_with_open_faces`, `sew::sew_faces`, `heal::heal_solid`/
`defeature`/`convert_to_nurbs`) to accept `rec: &mut OpRecorder` instead of constructing one
internally (mirroring euler's own convention exactly — no new pattern introduced). **No caller outside
these files changes** — `kernel.rs`'s SyncApi wrappers pass a throwaway `&mut OpRecorder::new()` at
each call site for now (their own deletion is Phase 2), so behavior is unchanged and every one of the
407 tests keeps passing untouched. Verify: `cargo test -p semio-framework-3d --lib` unchanged pass
count; `cargo bench -p semio-framework-3d --bench kernel` on the `booleans`/`sweeps`/`features` groups
(the ones §recon flagged as touching boolean/sweep/tessellate) to confirm zero perf regression from
the added parameter (should be a no-op — recorder plumbing is not on any hot inner loop, it is called
once per top-level operation).

### Phase 2 — `EngineRep<BrepArenaSeed> for Body` + `Loop` label decision (framework-3d only)

Land the seed struct and `build()` (§3) plus `Body → Seed` extraction, in `🕸️topology/🦀️component.rs`.
Blocked on the `Loop`/`Coedge` label question (§2) being answered by SMO first — if labels are added
to `Loop`, `make_loop`'s signature changes too (gains `rec: &mut OpRecorder`), which is a small,
mechanical, in-scope addition to Phase 1's list, not a separate wave. Verify: the new property test
(§3 Law A) plus unchanged 407-baseline; `Body`'s existing `#[derive(Serialize, Deserialize)]` round
trip (already covered by no test today — worth adding one law test here too, since `Body` serde is a
different round trip than the seed one and both matter for future undo/redo use).

### Phase 3 — delete the Registry region and the 90 SyncApi wrappers (framework-3d only)

Once Phase 1+2 land, `kernel.rs`'s `🧮Registry` region and `🔖️SyncApi` region become dead code — no
internal caller needs them (external callers are exactly `process3d`/`cad`, which are Phase 5's
problem, not this phase's). Delete both regions; `Brep` struct itself may shrink to nothing or be
deleted entirely if nothing inside framework-3d still needs a `Brep`-shaped bundle (the async
`BrepKernel` trait impl, Phase 5, is the last internal user). Verify: 407-baseline unaffected IF Phase
5 has not yet run (SyncApi's callers — the async trait impl — still exist and still compile against
whatever's left); if Phase 5 runs first this phase is unnecessary (the trait impl's deletion removes
the last caller of SyncApi automatically). **Recommend running Phase 5 before Phase 3** for exactly
this reason — merge Phase 3 into Phase 5's cleanup rather than doing it twice.

### Phase 4 — cross-session coordination gate (not framework-3d's to execute alone)

Before touching `BrepEngineHost`/`BrepKernel`/`GeometryHandle`, the coordinator opens a handshake with
whoever can act on `✏️s/🔌️plugins/🏭️process/**` and `✏️s/🔌️plugins/📐️cad/**` (unclaimed in the current
hot-file table — the coordinator's call whether that's APA, a new sub-wave of DKM, or a request to the
plugin owners directly) plus APA specifically for `SolidExporter`/`SolidImporter`'s reach into
`💻️os/🖥️host`. This phase produces no code; it produces an agreement on who migrates `process3d`/`cad`
to their own snapshot+mutation-triad shape (or an explicit decision to leave `BrepEngineHost` alive
indefinitely, which would be a real, stated exception to the greenfield rule and should be recorded as
such if chosen). **This design does not recommend a path here** — it is a scope/ownership decision for
the coordinator, not a technical one this agent can settle.

### Phase 5 — delete `BrepEngineHost`, `BrepKernel` trait, `GeometryHandle`, `Brep` struct, Codecs region (gated on Phase 4)

Only executable once Phase 4's migration (wherever it lands) is complete. Deletes: `⚙️engine/🖥️host/**`
entirely (149 LOC), the `BrepKernel` trait + `GeometryHandle`/`GeometryKind`/`BrepTopology` types in
`⚙️engine/🦀️component.rs` (the async trait's 89-method surface — its non-trait types like `Aabb`/
`ParamDomain`/`ClosestPoint`/`MeshTransfer`/`FaceGroup` likely survive as plain value types returned by
the new pure-fn call sites, need re-checking against whichever call sites replace `BrepKernel::volume`
etc.), and `🔌️Codecs` (1238-1323) once `SolidExporter`/`SolidImporter`'s replacement shape is settled
with APA. Verify: 407-baseline, full `cargo test -p semio-framework-3d --lib`, plus
`cargo bench -p semio-framework-3d --bench kernel` across all 9 groups (this phase touches every
category the benchmark covers) before/after.

### Phase 6 — stdio triad authoring (out of this ticket's write boundary until the handoff lands)

Not framework-3d's phase at all — recorded here only so the mechanical shape is written down once.
Once UCAS's stdio handoff lands (gate 3 of 3), whoever holds `✳️brep` writes the `SemioBrepMutation`
dispatch enum and the ~30 triads SMO's verb table already specifies (`create/delete-{vertex, edge,
face, shell, solid}` [+`loop` pending §2's SMO question], `move-vertex`, `replace-curve`,
`replace-surface`, and the `group_id`-batched booleans/Euler/sweep/offset/fillet compounds), each
`🔺️diff` leaf following exactly the shape §1/§3 describe: build `Seed` from `base`, `Body::build(&seed)`,
call one Phase-1-updated pure fn with a fresh `OpRecorder`, translate `OpDelta` + touched entities back
into `SemioBrepDiff` via the label↔id map (§2). This wave deletes `📄set-snapshot` (the banned
whole-document mutation, `✳️brep/🧬️schema/🧬️mutations/📄set-snapshot/**`) with no replacement, per the
locked decision (`📌️important.md`).

---

## 7. Explicitly not in scope, and why

- **Writing anything under `✏️s/`.** Zero gates fully open (§1); this design specifies the shape for
  whoever authors it later, but authors nothing.
- **Resolving the `Loop`/`Coedge` label question (§2).** Requires a narrower SMO ruling this design
  flags but cannot obtain.
- **Migrating `process3d`/`cad` off `BrepEngineHost`** (§4 Phase 4). Cross-session scope decision, not
  a framework-3d design question.
- **`SolidExporter`/`SolidImporter`'s replacement shape** (§4, §5 Phase 5). Reaches into APA's
  escape-hatch territory (`💻️os/🖥️host`); needs their sign-off, not designed here.
- **`🖋️imprint`'s euler-exclusivity violation** (§0 Correction 3). Flagged, not designed — deciding the
  right fix needs a full read of a 300+-line file outside this design pass's budget, and it's a
  pre-existing crack, not something this wave's dissolution introduces or worsens.
- **The `import_*_to_body` create-vs-reset ambiguity** (§5, "IO / bulk import" row). Depends on
  plugin-side UI gesture semantics this design has no visibility into.
- **Mesh (`🔺️mesh/🦀️component.rs`) and drawing (`◻2d`) dissolution.** Explicitly separate lanes per
  the hot-file table; this document is `📐️brep/**` only, per the dispatch brief's scope.
- **Rebuilding/rerunning the 407-test baseline or the benchmark suite.** This is a design-only agent
  with zero `.rs` edits; there was nothing to verify a diff against. The next (implementation) agent
  must re-confirm the baseline fresh before Phase 1, per hard rule 11 — do not inherit this document's
  citations of test names/line numbers as a substitute for that.
