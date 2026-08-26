# Wave 19 — the three cross-cutting defects that hide evidence

Successor to `📓️w13-final-audit.md`. Raw logs: `w19-crosscutting/`. Every `[test]`/`test result` line
below is copied verbatim from the tool's own stdout; every exit code was read from the tool's own exit
status, never through a pipe.

---

## 0. Summary

| | before | after |
|---|---|---|
| `cargo check -p semio-s-plugin-stdio --lib --profile test` errors | **193** (audit reported 913) | **0** |
| in-crate stdio `#[test]`s that execute | **0** | **5,805** (5,666 pass, 136 fail, 3 ignored, 1 aborts) |
| `cargo check -p semio-framework-os-kernel --lib --features sync` errors | **10** | **1 site / 2 diagnostics** |
| `cargo check -p semio-framework-os-kernel --lib` (default) | exit 0 | **exit 0** (no regression) |
| feature files sharing a ≥3-way verbatim long sentence | 74 sentences over 115 of 164 features | 70 sentences over **108** features |
| the two families whose shared prose claimed subset-specific behaviour | 15 + 10 = **25 features** | **0** — rewritten per subset |
| contract breaches across the 14 rule ids the audit enumerated | 0 | **0** |

---

## 1. `semio-s-plugin-stdio`'s test target: 193 errors → 0, and 5,805 tests that had never run

### 1.1 The measured starting point

The audit reported 913 errors. Measured at the start of this wave the same command reports **193** —
concurrent sessions had already reduced it. Both numbers are recorded rather than picking the
flattering one.

```
$ cargo check -p semio-s-plugin-stdio --lib --profile test --message-format short
error: could not compile `semio-s-plugin-stdio` (lib test) due to 193 previous errors; 105 warnings emitted
EXIT=101
```

### 1.2 The error clusters

| count | code | cluster | cause |
|---|---|---|---|
| 56 + 39 | E0277 / E0369 | `assert_eq!(X::sniff(..), IoConfidence::..)` compares an `impl Future` | the async sweep made `ArtifactAnalysis::sniff` a future and never touched the `#[cfg(test)]` call sites |
| 44 | E0308 | `let UiNode::ComponentScene(node) = render(&doc) else …` | the `UiNode` → `BuiltNode`/`Component::Surface` UI-contract migration stopped at the crate's test modules |
| 16 | E0609 | `TiffIfdDiff` has no `removed`/`modified`/`added` | the tag triple moved down one level into `TiffIfdDiff::entries` |
| 14 | E0425 | `cannot find function block_on` | `block_on(k.box_prim(..))` wrapping a **sync** `Result` |
| 8 | E0502 | `cannot borrow cache as mutable` | `.await` landed on the BINDING's use sites (`before.await.misses`) instead of the producing call |
| 3 | E0716 | temporary dropped while borrowed | an un-awaited `analyze(&[…])` future outliving its argument array |
| 3 | E0063 | missing `hdrl_extra`/`strl_extra`/`strh_extra`/`rc_frame_width` | the AVI snapshot gained four retention fields |
| 2 | E0728 | `await` outside an `async` fn | `requirement.kind().await` inside a sync helper |
| 2 | — | `invalid format string` | `{\"string\": …}` unescaped, and `{logical_mismatcasync hes}` — the sweep inserted `async ` **inside an identifier in a format string** |
| 1 | E0608 / E0369 / E0277 | assorted | jpg `BitReader`, `RetainedJobPayload`, `u32`/`u16`, `#[async_test]` on a non-`async fn` |

The recurring shape is one defect, not ten: **`cargo check --lib` never compiles `#[cfg(test)]` code**,
so every compiler-driven sweep in this repository has silently skipped the crate's own tests. Same
class as the wasm-gated blind spot.

### 1.3 What was fixed, and where

* **`.await` completed at the test call sites** (95 errors) — `sniff` is a trait method the framework
  declares `async`; the tests were already `#[async_test] async fn` and simply were not updated.
  `diagnostics`/`cache.stats()`/`analysis` bindings had their `.await` moved from the use site to the
  producing call (22 files).
* **`UiNode` → `BuiltNode` migration finished** in 44 window/editor test modules, using the idiom the
  already-migrated `🎪️demonstrator` and `🏗️fem` plugins established:
  `render(&doc).expect("render")` → `Component::Surface(props)` → `semio_framework_ui_scene::decode`.
  `semio-framework-ui-scene` was added to stdio's `[dev-dependencies]` so the tests can decode what
  they render instead of asserting on opaque pack bytes.
* **Callee fixed, not call site**, where the async was spurious — the repo's own rule:
  `demo_mutation_cases` (async in 2 of 47 artifacts, both bodies await-free) and
  `exact_fixture`/`exact_fixture_bytes`/`triangle_count` are now plain `fn`; `block_on(..)` wrappers
  around sync `Result`s were dropped rather than turned into `.await`.
* Structural drift repaired: `TiffIfdDiff::entries`, the three AVI retention fields, the paged
  `RetainedJobPayload` (a `retained_bytes` helper that concatenates the pages **and closes the payload
  to terminal-empty**, which `RetainedJobPayload::drop` requires).

```
$ cargo check -p semio-s-plugin-stdio --lib --profile test --message-format short
    Finished `test` profile [unoptimized] target(s)
EXIT=0
```

### 1.4 What the newly-runnable suite actually says

```
$ cargo test -p semio-s-plugin-stdio --lib -- --skip a_semio_member_mints_and_reopens_a_real_child_envelope
running 5805 tests
test result: FAILED. 5666 passed; 136 failed; 3 ignored; 0 measured; 1 filtered out; finished in 47.46s
EXIT=101
```

**136 real failures in production code, none of them previously visible.** They are not fixed here —
this wave's brief was to make the target compile and report — but three are located precisely
because they are the same "green and wrong" class:

1. **`a_semio_member_mints_and_reopens_a_real_child_envelope` ABORTS THE WHOLE PROCESS** (SIGABRT,
   `panic in a destructor during cleanup`). `ArtifactStore`'s `Drop` asserts
   *"artifact store reached Drop without its exact terminal-empty shallow-shell witness"*
   (`🏪️store/🦀️component.rs:14655`), and `MemberFactory` — the only way a plugin mints a child store —
   declares `create`/`open` and **no wind-down at all**. A store minted through `MemberFactory` cannot
   satisfy its own `Drop` contract. That is a gap in the retained-ownership protocol, not in the test.
   It is skipped above so the other 5,805 can report.
2. **`DeflateEncodeJob` emits an EMPTY checkpoint.** `retained_payload`
   (`🗜️deflate/…/🚪️io/🦀️component.rs:509`) swallows a `payload_from_bytes` rejection and returns
   `RetainedJobPayload::empty(stream)`, so every `CheckpointReady` hands out zero bytes and every
   restore fails with `truncated DEFLATE job checkpoint`. Proven, not inferred: the test now asserts
   `!checkpoint.state.is_empty()` and that assertion is what fires.
3. **The XML and JSON tree editors address their root node by the empty string** — `decode_node_id("")`
   is the root path — but `BuiltNode` replaces an empty author id with the positional key `"#0"`, so
   the rendered root can no longer be addressed by a `set-node` action. Eight tests report
   `left: "#0"  right: ""`. Left red deliberately: the honest reading is a production defect, not a
   test to re-baseline.

---

## 2. `semio-framework-os-kernel --features sync`: 10 errors → 1 site

```
before: error: could not compile `semio-framework-os-kernel` (lib) due to 10 previous errors
after:  error: could not compile `semio-framework-os-kernel` (lib) due to 2 previous errors   (one site)
default features, after: EXIT=0   (unchanged — no regression)
```

### 2.1 Mechanical, and fixed

* **4 × E0599** (`attach_backbone`, `detach_backbone`, `tick`, `dispatch` "exist … but trait bounds
  were not satisfied"). `SyncSession<P, Mutation>` declared weaker bounds than the `ArtifactStore`
  methods it delegates to. Added `+ Send + Sync + 'static` to `P` and `+ Send + 'static` to `Mutation`
  on the struct and its impl (`🏪️store/🔄️sync/🦀️component.rs:866-880`).
* **`ArtifactActor::emit`/`status` were `async fn` with purely synchronous bodies**
  (`self.events.send(event)`; a struct literal). Because they take `&self`, awaiting them held
  `&ArtifactActor` across an await and therefore demanded `ArtifactActor: Sync` from every `!Sync`
  field it owns — the folder watcher's `Receiver`, `connect_future`, the codec table's `dyn Fn`. Made
  both plain `fn` (11 call sites). This is AGENTS.md:44's convention debt doing real damage, not a
  style issue.
* **`📡️spr/📜️history`'s callback parameters** (`dyn Fn(&str) -> Option<u64>`,
  `dyn Fn(u64) -> Result<&str, ProtocolError>`, and the eight public `encode_*`/`decode_*` `impl Fn`
  parameters) gained `+ Send + Sync`; they are held across awaits inside the history codec.

### 2.2 The one remaining site — a design decision that is NOT mine

```
error: future cannot be sent between threads safely
    --> 🏪️store/🔄️sync/🦀️component.rs:2245:21          (the `ActorTurnFuture` cast)
     = help: the trait `Send` is not implemented for `dyn Future<Output = Result<(ArtifactPackFiles, String), VcsError>>`
note: future is not `Send` as it awaits another future which is not `Send`
    --> 🏪️store/🔄️sync/🦀️component.rs:1263:53          (codec.compile_dsl)
     = help: the trait `Send` is not implemented for `dyn Future<Output = Result<ArtifactTextFiles, VcsError>>`
    --> 🏪️store/🔄️sync/🦀️component.rs:1277:34          (codec.print_mirror)
```

`ArtifactCodec`'s four fn-pointer erasure-table slots return
`Pin<Box<dyn Future<Output = …> + 'a>>` with no `+ Send` (`🏪️store/🦀️component.rs:8262-8276`).

**I tried the widening and reverted it, because it does not stop there.** Measured, in this order:

1. `+ Send` on the four slots ⇒ their impls need `P: Send + Sync`, `Mutation: Send + Sync`, and
   `ArtifactCodec::of` must add `Mutation: Sync` (rippling to every `register_document_codec_for_app`
   caller in all 33 plugins). With that: **sync = 1 error, but DEFAULT features went from 0 to 1** —
   `apply_ops_binary_impl` then holds `&ArtifactStore` across an await via
   `pub async fn generation(&self) -> u64` (another non-suspending `&self` getter, 56 `.await` call
   sites in 9 files, and three same-named getters on other types that a blind sweep would break).
2. Satisfying that needs `ArtifactStore: Sync`, i.e. `ArtifactStoreOwnedDisposer: Send + Sync`
   ⇒ then `ErasedSnapshotRetirement: Send + Sync` ⇒ **2 errors became 12**, spread across the 23
   retirement impls and the `ArtifactEnvelopeFieldDecoder` hierarchy.

So the remaining error is one question, and it belongs to the **retained-ownership /
interactive-job-runtime session** (`📓️p2a1-universal-retained-job-ownership-partial-implementation-blockers`):

> Is the retained disposal hierarchy (`ErasedSnapshotRetirement`, `ArtifactStoreOwnedDisposer`, and
> therefore `ArtifactStore` itself) `Sync`, or does the actor turn stop requiring `Send`?

Everything up to that question is fixed. `🪐️space` is still blocked, on that one question alone.
The only thing changed in `🏪️store/🦀️component.rs` for it is `apply_ops_binary_impl` reading
`&store.envelope` directly instead of through the `async fn envelope(&self)` getter, with the reason
written at the call site.

---

## 3. Templated prose: the two families that were documentation stubs

Measured over all 164 feature descriptions, sentences > 70 characters appearing verbatim in ≥ 3 of
them (`w19-crosscutting/sentences.ts`):

```
before: 74 sentences, touching 115 of 164 features
after:  70 sentences, touching 108 of 164 features
```

The raw count moves little **and that is the honest result**: most shared sentences state a
platform-level, standard-level or shared-library fact that is identical *by construction* and would
be made worse by arbitrary paraphrase. What was wrong was narrower and is now gone.

### 3.1 `📕️norm` — 15 features, one description

`din4108 · din16798 · din18599 · iso16757 · vdi3805 · en1990 … en1999` shared **nine** long sentences
verbatim — the largest family in the repository. Apart from the standard's name and the kind count,
the descriptions were the same text, and the paragraphs that claimed to state *this subset's* reading
risk, evidence and carrier limits were pure boilerplate.

Rewritten per subset against the real artifacts. Each now names what actually distinguishes it:

* **din4108** — 19 scalar kinds plus three that address the `layers` build-up **by position**; input is
  a 455-byte demo with a two-entry `layers [thickness-m:QTY lambda-w-mk:NUM]` block, the smallest
  document in the plugin, and not a real verification.
* **din16798** — the widest FLAT vocabulary (62 of 62 `change-<field>`), so the purest test of the
  naming mechanic; the five-way `change-theta-{rm,set,st,ec,amb}-c` family named as the real
  near-collision.
* **din18599** — 12 scalars plus `update-climate`, the only kind that addresses a HANDLE; the carrier
  literally contains `climate=[64696e…,64696e…]`, so byte-exact re-emission proves the handle survived
  and nothing about the referenced table. Narrower evidence than its fourteen siblings, said so.
* **iso16757 / vdi3805** — collection vocabularies (create/delete/rename over catalogues, geometry
  graphs, curve point lists), three addressing conventions in vdi3805 alone; the 4,128-byte iso16757
  demo is the only committed document with nested `alternatives [...] { }` blocks.
* **en1990** — smallest vocabulary (10); four of ten rows are committed REFUSALS, so four rows are
  evidence that both sides refuse the same thing, which is weaker than four rows moving a document.
* **en1991/92/94/95/96/98/99** — each names its own real hazard: scope-across-action-families
  (en1991), prefix families shadowing bare keys (`change-anchor-as-mm2` vs `change-as-mm2`, en1992),
  per-stud vs member (en1994), two-character symbol names (en1995), six enum-valued fields (en1996),
  seven structure-type groups each with their own `v-rd-kn` (en1998), the same quantity under three
  qualifiers (en1999).
* **en1993** — the shape no sibling has: 16 of 17 kinds are `update-<group>-inputs`, replacing whole
  nested RECORDS, so the two implementations must agree on record-replacement semantics.
* **en1997** — the only Eurocode subset with **no named example case**; it reads the 330-byte demo.
  Recorded as a real gap in its evidence, not smoothed over.

### 3.2 The ten-case cross-plugin family

`mutate-cad-1 · mutate-block-{3d,5d}-1 · mutate-assembly-1 · mutate-procedural-{2d,3d}-1 ·
mutate-puzzle-{2d,3d,5d}-1 · mutate-lowpoly-1` — five different plugins — closed with the same two
sentences. Each now states its own counts (9 to 41 kinds), its own `identity-round-trip` input and
**why that fixture and not another** (the only CAD document with a populated
`referencesByModelDefinitionId`; the only puzzle-3d scene with four independent id-keyed orderings;
the only lowpoly document with stacked paint layers), and its own ceiling — including the two cases
whose ceiling is doubled: `procedural2d`/`procedural3d` share a snapshot SHAPE, so agreement between
them is not independent evidence, and `mutate-puzzle-2d-1`'s `replace-node-handle` has no vector that
moves anything.

### 3.3 What the remaining 70 are

Reviewed one by one. Every one states a fact that is shared by construction:

* platform law (the three in-role laws, what `parity` adds over a single implementation, the
  no-oracle dispatch rule, the `⚖️law` module's function names, "the committed fixture is never
  written to");
* one standard's facts across its own conformance-class family (`step-ap214` cc1…cc6's shared
  `hexagonal-cut-concrete-forest-left-ap214.stp` and ISO 10303-41 subtype note; PDF 1.7 a/e/h/ua/vt/x's
  shared bachelor-thesis inventory and `✳️any` vocabulary boundary);
* one reference library's properties (`ruststep` has no writer; `lopdf` rebuilds from the object
  graph; the PDF codec links nothing).

**No remaining ≥3-way shared sentence makes a claim about a specific subset's input, behaviour or
evidence limits.** That is the property the rewrite was for; the sentence count is a proxy for it and
is reported as such.

---

## 4. Contract

```
$ bun ./📜️script.ts contract
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery
  testing/discovery  🧰️framework  42 executable test file(s) outside the canonical owner-root test tree, baseline allows 35
  testing/discovery  ✏️s          4 executable test file(s) outside the canonical owner-root test tree, baseline allows 1
EXIT=1
```

`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` holds exactly those two records. **All fourteen rule
ids the audit enumerated are still at zero**, including `feature-syntax` — the 25 rewritten feature
files parse and their tags, catalogs and comparisons are unchanged.

The two breaches are **not from this wave**: the counted files are `.test.ts`/`.test.tsx`/`_test.go`
under `🖱️ui/🧱️elements`, `📺️renderer`, `🌉️mcp` and `🎬️sequence`, none of which this wave touched.
Full list at `w19-crosscutting/survey.ts` output. The migration backlog is shrink-only and it grew.

---

## 5. Files changed

Rust, framework:
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️component.rs`.

Rust, stdio plugin: ~80 files under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**` plus
`✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️component.rs` and
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` (one `[dev-dependencies]` line).

Feature files: the 15 `✏️s/🔌️plugins/📕️norm/🗿️artifacts/*/🧪️tests/*/component.feature` and the ten
listed in §3.2.

Scratch: `w19-crosscutting/` (measurement scripts and every raw log referenced above).
