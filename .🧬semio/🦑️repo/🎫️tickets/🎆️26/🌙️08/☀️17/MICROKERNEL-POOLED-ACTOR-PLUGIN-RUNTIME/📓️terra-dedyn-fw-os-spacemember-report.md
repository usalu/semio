# 📓️ terra — dedyn-fw-os-spacemember report

Packet: `dedyn-fw-os-spacemember`, owned family `SpaceMember` (~15 uses), owned path
`🧰️framework/🛍️products/💻️os/**` scoped to that family only. Single file touched:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (20,878 lines).

## 1. Verified counts (python3 over the absolute path, two differently-implemented queries)

**Before**: 15 `dyn SpaceMember` occurrences, all in this one file (confirmed by a `os.walk` scan of
every `.rs` file under `🧰️framework/🛍️products/💻️os` excluding the completed-packet paths named in
the brief). 13 in code, 2 in doc comments (`ChildContentView`'s and `dispatch_emit_group`'s own doc
comments).

**After**: **0** code occurrences. Verified twice — a python3 line-scan classifying comment vs code
lines, and a `grep -n "dyn SpaceMember" | grep -v -E '^\s*[0-9]+:\s*(///|//)'` filter — both report
zero. Two doc-comment mentions remain (historical, describing the old shape); I updated the wording
of 3 others that would otherwise now read as false ("Box<dyn SpaceMember>", "no object-safe … getter"
where object-safety was never the actual constraint).

## 2. Mechanism chosen per family shape (R11)

All 15 sites are `SpaceMember`, but they split into three structurally different sub-cases:

### 2a. `VcsArtifactApp`'s own children storage (9 of 15) — generics, closed to this file

`children: HashMap<(String,String), (ArtifactDialect, Box<dyn SpaceMember>)>` and its direct
consumers (`child_store`, `register_child`, `absorb_created_children`, the `child_ptrs`/`member_ptr`/
`dispatches` cluster in `dispatch_emit_group`) → `VcsArtifactApp<A: ArtifactApp, M: SpaceMember +
MemberFactory = store::NoMembers>`, `children: HashMap<_, (ArtifactDialect, M)>`. Exactly the shape
`📓️terra-store-dedyn-report.md` established (`SpaceHost<M>`, `NoMembers` default) — R11's "exactly
one impl" doesn't apply (every app has different child kinds), "open set" generics is the right case,
and the default type param keeps every existing `VcsArtifactApp<SomeApp>` (no composed children)
compiling unchanged. `open_child` also got its companion fix: it called the DELETED
`store::child_store_factory` global registry (store's own report's lease-request item 2, filed against
this exact file/line) — now `M::open(&dialect.artifact_kind, envelope_pack).await`, compile-time
dispatch through the composition's own `store::MemberFactory` impl, no registry.

### 2b. `ChildContentView`/`ArtifactView` (2 of 15) — function-generic, NOT struct-generic (the real find)

This was the hard one. `ChildContentView<'a>` held `Box<dyn SpaceMember>` specifically so it could
stay a **single, non-generic type** — because it is the `children` field of `ArtifactView<'a, P>`,
and `ArtifactView<'_, Self::Snapshot>` is the FIXED parameter type on every method of the
`ArtifactApp` trait (`render`, `handle`, `interaction_topology`, `copy_fragment`, `cut_operations`,
`pending_effects`, `window_engagements`, `window_measures`, `tool_measures`, `context_menu`,
`import_media`, `media_fingerprint`, `ephemeral`, `paste_operations`) — a trait EVERY plugin in the
fleet implements with that literal signature.

I first assumed "just add `M` to `ArtifactView`/`ChildContentView` with a default, same as
`VcsArtifactApp`" — but traced it through and it does not work: Rust's associated-type DEFAULTS
(`type Members: SpaceMember = NoMembers;` on the trait itself) are unstable
(`#![feature(associated_type_defaults)]`), so making `ArtifactApp::render`'s signature generic in a
per-app member type would force **every** `impl ArtifactApp for X` fleet-wide (in `✏️s/🔌️plugins/**`,
11 siblings' territory, explicitly not mine) to add a mandatory `type Members = …;` line — the same
shape of change as the already-completed 2,671-file `async_test` codemod, but NOT something a single
family-scoped packet should trigger by editing one trait declaration. I confirmed via a repo-wide
grep that `ChildContentView`/`ArtifactView::with_children` construction is 100% confined to this one
file (24 hits, all here or doc-comment mentions in two unrelated plugins) — so the fleet-wide blast
radius would come entirely from the TRAIT signature, not from any actual fleet caller.

**Fix, without touching the trait**: made `M` generic on the **function** `ChildContentView::new`,
not on the struct. `new<M: SpaceMember>(children: &HashMap<_, (ArtifactDialect, M)>) -> Self` now
eagerly awaits `document_pack_bytes()` for every live child ONCE at construction and stores the
resulting `HashMap<_, (ArtifactDialect, Vec<u8>)>` — `ChildContentView` itself carries no type
parameter, no `dyn`, no lifetime (it now owns its bytes instead of borrowing), and slots perfectly
into `ArtifactApp`'s fixed, unmodified signature. `ArtifactView<'a, P>.children: ChildContentView`
(lost its `<'a>`, same reason). Behavior change, documented in the struct's own doc comment: reads are
now "as of `new()` time" rather than "live through every individual `.pack()` call" — every one of
this file's own call sites constructs `ChildContentView::new(...)` fresh immediately before use (per
the pre-existing "built by VcsArtifactApp before each dispatch" convention), so this is not observable
in practice; a caller that constructs one view and mutates children mid-lifetime before reading would
see stale data, which no code in this file does. Also fixed the 13 call sites' pre-existing
missing-`.await` (both `ChildContentView::new(...)` and the enclosing `ArtifactView::with_children(...)`
were already `async fn` but never awaited).

### 2c. Parent/children unification in `dispatch_group`/`undo_group`/`redo_group` (4 of 15) — BLOCKED, reported not improvised

`store::CompositionCoordinator::dispatch_group<M: SpaceMember + MemberFactory>(parent: &mut M,
children: &mut [(&mut M, ChildDispatch)], …)` and `undo_group`/`redo_group<M: SpaceMember>(members:
&mut [(&ArtifactRef, &mut M)], …)` all require **one** `M` for parent AND every child. In
`VcsArtifactApp`, the parent is always `self.store: ArtifactStore<A::Snapshot, A::Mutation>` (fixed
per `A`); the children are `M` (this composition's own child-kind type, from §2a) — a *different* type
in the general (heterogeneous-children) case. I checked whether the degenerate "M happens to equal the
parent's own type" case saves it: production `ArtifactStore<P, Mutation>` implements `SpaceMember`
(real, crate-root blanket impl) but **not** `MemberFactory` — only a `#[cfg(test)]`-private fixture
wrapper *inside `🏪️store`'s own test module* implements `MemberFactory` for it, which is not visible
from my crate. So even the same-type case fails `dispatch_group`'s bound; `undo_group`/`redo_group`
(no `MemberFactory` half) would degenerate-case-compile only for a composition whose child type is
*literally* `ArtifactStore<A::Snapshot, A::Mutation>`, which is not the general shape `space_members!`
exists for.

This is exactly the "architectural mismatch in `dispatch_group`'s signature" the brief told me to
report rather than improvise. Per that instruction I did **not** invent a workaround (no fleet-wide
`ArtifactStore: MemberFactory` impl, no unsafe transmute, no reintroduced `dyn`). What I did: removed
the `dyn` cast at all 4 sites (`&mut self.store as &mut dyn SpaceMember` → `&mut self.store`; the two
`Vec<(_, &mut dyn SpaceMember)>` annotations → `Vec<(_, &mut M)>`), which is O1-compliant (zero `dyn
<first-party trait>`, verified above) and turns the residual problem into an ordinary, precisely
located `E0308` type-mismatch at exactly those 4 lines — documented inline at each site with a
`🚧️ BLOCKED` comment — rather than leaving a hidden object-safety violation. **Needs, on the
`🏪️store` side (not my owned path)**: either split `dispatch_group`/`undo_group`/`redo_group` into
two type params (`Mp: SpaceMember` for the parent, `Mc: SpaceMember [+ MemberFactory]` for children),
or give production `ArtifactStore<P, Mutation>` a real (non-test) `MemberFactory` impl for the
same-type composition case. I did not choose between these — that's a design call above this packet's
scope, and either resolves both the two `dispatch_group` sites and the two `undo_group`/`redo_group`
sites identically.

## 3. Macro friction

`store::space_members!` worked exactly as documented on the first try for a genuinely new use (this
file's own test fixture, `TestMembers`, one variant) — no friction. Did not need `dyn_enum_close!`
here at all (this file doesn't own any trait declaration to `#[dyn_enum]`).

## 4. Test-module fallout (in-family only, not chased further)

`register_child`'s signature changing from `Box<dyn SpaceMember>` to `M` is a hard type change, so I
adapted the composition test fixtures to keep them type-consistent with the new signature: added
`store::space_members! { pub enum TestMembers { Child("s.test.child", "semio.test/v1") =>
store::ArtifactStore<TestSnapshot, TestMutation> } }`; `new_test_child` now returns
`Result<TestMembers, VcsError>` instead of boxing; the two `member.as_any_mut().downcast_mut::<...>()`
call sites (only needed to escape a type-erased `dyn`) became a plain `let TestMembers::Child(x) = y;`
destructure; deleted `register_test_child_factory` (called the deleted
`store::register_typed_child_store_factory` — the composition now dispatches by kind at compile time
through `TestMembers::open`, no runtime registry needed); added an explicit
`VcsArtifactApp<TestApp, TestMembers>` annotation on the one `reloaded` binding where nothing else in
that test pins `M` for inference, and on `reads_child_count`'s parameter type.

**What I deliberately did NOT do**: this test module (like evidently the rest of this 20K-line file)
has a *separate*, pre-existing, pervasive missing-`.await` problem — `app.dispatch_typed(...)
.expect(...)`, `app.dispatch_action(...).expect(...)`, `VcsArtifactApp::new(...)` itself, etc., called
without `.await` throughout, on functions that were already `async fn` before I touched anything.
That is the general "asyncify signatures first, then insert awaits" fixpoint-loop fallout named in the
brief (R9/rule 8's "counted, not chased"), not unique to `SpaceMember`, and touches dozens of
unrelated command names in this same test module. I fixed `.await` only on the specific
sub-expressions I was already rewriting for type reasons (`test_child_dialect()`, `new_test_child()`,
`ChildContentView::new(...)`, `ArtifactView::with_children(...)`, `absorb_created_children(...)`) —
not on the surrounding `.expect(...)` calls on unrelated commands. This file's test module will not
compile end-to-end until that separate pass runs; that pass is not scoped to my family.

## 5. Acceptance

**`cargo check -p semio-framework-plugin --lib`**, `CARGO_TARGET_DIR=.../scratchpad/target-dedyn-os-spacemember`,
foreground, ~590s budget — run once, near the end, per the brief's own instruction. Full log:
`terra-dedyn-os-spacemember-cargo-check1.txt` in this folder (359KB). **Exit code 101.** Never reaches
`semio-framework-plugin`: blocked upstream in `semio-framework-ui`
(`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` +
`🦀️label.rs`), 169 errors, all universal-async-codemod fallout (missing `.await` producing
`bool`/`String`-vs-`Future` mismatches inside `#[derive(Serialize)]`d structs and a `BTreeMap::collect`)
— entirely unrelated to `SpaceMember`, not in my owned path, not touched. This matches the ticket's
own "COMPILE REALITY" warning: the guest SDK (`semio-framework-plugin`) sits behind
`semio-framework-os-kernel`, which is itself behind this UI crate in the dependency graph.

**Structural verification (trustworthy, done in place of the blocked compile)**:
- Zero `dyn SpaceMember` in code, two independently-implemented queries (§1), both zero.
- A syntax-only parse of the edited file (`rustc --edition 2021 --crate-type lib -Z
  parse-crate-root-only 🦀️component.rs`) exits 0 with no diagnostics — confirmed the flag actually
  catches real syntax errors first (tested against a deliberately unclosed-delimiter file, which it
  correctly rejected). This does not check types/resolution (blocked upstream, see above), only that
  every edit landed as well-formed Rust.
- `#![allow(async_fn_in_trait)]` was already present at this crate's root from an earlier packet — did
  not need to add it.
- No `+ Send` bound added anywhere (R3) — `M: SpaceMember + MemberFactory` throughout, Send comes
  structurally from whatever concrete `M` a plugin picks.

## 6. Lease-request

```lease-request
Owner: whichever packet owns 🏪️store/** (or the coordinator, if store-dedyn is closed)
File: 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
Needed changes (either one resolves all 4 residual sites in my file, §2c above):
  1. Split `CompositionCoordinator::dispatch_group`/`dispatch_peer_group`/`dispatch_relation_group`/
     `compensate` into `<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>` (parent vs. children), and
     `undo_group`/`redo_group` into `<Mp: SpaceMember, Mc: SpaceMember>` — OR
  2. Give production `ArtifactStore<P, Mutation>` a real (non-test-only) `MemberFactory` impl, so the
     "composition where every member — parent included — is the same concrete type" case at least
     compiles even though the general heterogeneous case would still need (1).
Why I can't do this myself: 🏪️store/🦀️component.rs is not my owned path (a completed sibling
packet's file — "read it for its shape, do not re-edit it" per the brief).
Where it bites: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs, the 4 sites tagged
`🚧️ BLOCKED` inline — `dispatch_emit_group`'s `self.composition.dispatch_group(...)` call, and
`dispatch_group_history_action`'s `members` vec + `undo_group`/`redo_group` calls.
```

## 7. Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (only file touched — everything
  above: imports, `VcsArtifactApp<A, M>`, `ChildContentView`/`ArtifactView`, `open_child`,
  `register_child`, `child_store`, `absorb_created_children`, `dispatch_emit_group`,
  `dispatch_group_history_action`, and the composition test fixtures).
- Ticket-folder log: `terra-dedyn-os-spacemember-cargo-check1.txt` (this folder).

## 8. Summary for the coordinator

- Starting: 15 `dyn SpaceMember` (13 code, 2 comment). Ending: **0 code occurrences**, verified two
  ways. 4 sites converted to a precisely-located, documented, non-`dyn` compile blocker in `🏪️store`
  (§2c, lease-request filed) rather than worked around.
- Mechanism: generics on `VcsArtifactApp<A, M = NoMembers>` for the closed per-composition storage
  shape (matches `📓️terra-store-dedyn-report.md`'s own precedent exactly); a function-generic (not
  struct-generic) `ChildContentView::new<M>` to avoid a fleet-wide `ArtifactApp::Members`
  associated-type rollout that would have reached 11 siblings' owned paths — the one finding here that
  matters beyond this packet, since any other family hitting a trait with a FIXED method signature
  (not just `SpaceMember`/`ArtifactApp`) will hit the same associated-type-defaults-are-unstable wall.
- `cargo check` UNRUN in the sense of never reaching my crate — blocked at `semio-framework-ui`, named
  and logged, not touched (out of scope).
