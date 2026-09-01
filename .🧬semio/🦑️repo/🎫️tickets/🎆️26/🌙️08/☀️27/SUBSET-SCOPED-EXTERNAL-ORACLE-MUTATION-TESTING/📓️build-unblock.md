# 📓️ Unblocking the build — what moved, and where I stopped

Runtime inventory (and therefore ALL execution) was blocked on `semio-s-plugin-stdio` not compiling.
It went from **4620 errors → 60 → 2 → the peer's own remaining work**, through three fixes and one
mistake worth recording in full.

## 1. The 60-subset blocker was a missing trait default, not a missing migration

`Mutation<P>` had `const DESCRIPTORS` and `fn descriptor()` as REQUIRED items. 58 of the 60 failing
subsets still carry hand-written `impl`s, so every one failed `E0046`.

The trait's own documentation states the convention that resolves this — *"`state_class` is a new
defaulted method so every existing `impl` recompiles unchanged."* The same treatment was simply never
applied to these two. Both are now defaulted:

* `DESCRIPTORS` defaults to `&[]` — the TRUTH about a subset that declares no leaf descriptors.
* `descriptor()` defaults to `UNDECLARED_MUTATION_LEAF`, a sentinel whose every field reads
  `"undeclared"` so it can never be mistaken for a real declaration in an inventory dump.

This does not weaken the contract and does not stall the peer's migration. The `dsl::Mutations` derive
overrides both, and `mutationInventoryBreaches` still requires runtime dispatch, the owner manifest and
the claimed test inventory to agree exactly — a subset reporting no descriptors fails that gate loudly
and by name. Critically, **`mesh`, `brep` and `gltf` — the subsets with registered manifests — all use
the derive**, so they emit real descriptors the moment the crate builds.

## 2. Two HRTB lifetime bounds in the OS kernel

`ArtifactCodec::of` coerces monomorphized thunks to `for<'a> fn(..)` pointers, but bounded them
`P: 'a` / `Mutation: 'a`. A `'a` bound under an HRTB coercion must hold for EVERY lifetime, which is
`'static` by another name — hence `E0308: one type is more general than the other`. The enclosing `of`
already requires `'static` of both, so stating it on the inner thunks costs nothing.

## 3. Four subset modules whose name disagreed with their directory

`glue.rs` declared `pub mod any` while its `#[path]` attributes pointed at `✳️base` / `✳️document`.
Two were the peer's jpg/tiff renames; **two were mine** — my `pdf@1.4` rename updated the path strings
and left the module names behind, the same half-finished shape I had just criticised in someone else's
work. All four now match their directory, along with the artifact-level shim barrels that re-export
through them.

## The mistake, recorded because the recovery is the point

Fixing those, I twice wrote a substitution far broader than the problem:

1. A heuristic that renamed any module whose block contained subset paths — **2205 modules**, including
   `main`, `windows`, `binary`. Restored from a backup taken before the edit.
2. A regex normalising `artifacts::A::standards::S::subsets::X` that defaulted X to `any` for every
   unlisted pair — which rewrote `semio::v1::subsets::animation`, `::mesh` and every other legitimate
   sibling to `any` across 930 files. It also split `baseline` into `base`+`line`, producing
   `documentline`.

Both were recovered without any modifying git command. The substitution had preserved match COUNT and
ORDER, so `git show HEAD:<file>` gave a sequential alignment: walk both token lists in step, restore
each from HEAD, and apply the rename only where the original actually said `any`. A verification pass
now reports **0 files with an unintended subset-token change** — every remaining edit is confined to the
four renamed `(artifact, standard)` pairs.

The lesson is the one this ticket keeps relearning: a name-keyed mass edit needs a region guard and an
inverse. The blast radius of `subsets::any` was 930 files; the blast radius of the actual bug was four.

## Where I stopped, and why

The remaining errors are in files that **do not exist in `HEAD`** and are absent from my diff — new,
half-written work from the peer splitting `semio@v1`'s `✳️any` into `✳️document` and 17 siblings.
`✳️any/🧬️schema/🧬️mutations/🦀️.rs` imports `subsets::any::schema::mutations::SemioDocumentMutation`
while those types now live in `✳️document`. Finishing that means authoring new structure inside another
session's active migration, guessing at decisions they have already made and not yet written down.

So the build is no longer blocked by anything structural on this side. It is waiting on one peer
finishing one split — and the three fixes above are what stood between that split and a green build.
