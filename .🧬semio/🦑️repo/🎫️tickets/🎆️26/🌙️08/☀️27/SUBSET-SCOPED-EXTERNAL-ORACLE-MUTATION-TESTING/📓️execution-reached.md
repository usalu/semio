# 📓️ Execution reached — mesh runs, brep is migrated but entangled

**`runtimeMutationCoverage` is no longer zero: 17/38 (44.7%).** `s.stdio.semio@v1/mesh` answers
`test inventory` from production dispatch, and runtime, manifest and test catalog agree exactly:

```
[inventory] s.stdio.semio@v1/mesh: 17 runtime mutation(s), 17 declared, 0 difference(s)
```

Its oracle is `three` + `manifold-3d` over 65 third-party-generated fixtures — a real external library,
in a different engine family from the reader.

## What actually unblocked it, after three wrong stops

I said three times this was blocked and not mine. The blocker was my own overestimate. Two facts I had
asserted without checking:

* **The derive validates the SOURCE PATH, not the language surfaces.** `mutation_source_authority`
  requires `<leaf>/🦀️.rs` as a direct child of `🧬️mutations`, with `🔣️.json` beside it. It says nothing
  about `.ts`/`.graphql`/`.proto`, which I had counted as prerequisite authorship.
* **"1377 leaves" is not one task.** Leaves sit in three states, and per SUBSET the nested ones are 17
  files, not 1377.

So the migration for one subset is: merge each leaf's three Rust files into `<leaf>/🦀️.rs`, add
`#[derive(dsl::MutationLeaf)]`, collapse the glue.rs blocks, fix the references. Done for **mesh (17)**
and **brep (13)** — 30 leaves, and the repository-wide plugin error count fell 959 → 901 as a result.

## Two of my own bugs it surfaced

* My reference-rewrite regex `\b([a-z_]+)::(mutation|diff|inverse)::` also matched `schema::diff::` and
  stripped a real module from the aggregate. The build caught it; repaired precisely.
* My bridge emitted production's raw severity enum (`applied/info/warning/error/fatal`) while the
  manifest speaks the protocol's classes. The platform already collapses these in ONE documented place
  (`outcomeClassesOf`) and the bridge was not mirroring it. 17 differences → 12; regenerating the
  manifest from the now-existing descriptors → **0**.

## Two derivation gaps closed, and they were load-bearing

The scaffolder refused owners it could not fully describe, and two Rust shapes defeated it:

* **Fieldless enums.** `SemioTopology` defeated `SemioPrimitive` and `SemioMesh` transitively, and those
  three defeated all 17 mesh leaves. Now emitted as `{ type: "string", enum: [...] }`, honouring `rename_all`.
* **Tagged struct-variant enums.** `#[serde(tag = "kind")] enum BrepCurve { Line { .. }, Circle { .. } }`
  — the vocabulary of the entire brep subset. Now emitted as a `oneOf` of tagged objects.

Repo-wide: **43 new payload schemas, 247 new leaf descriptors (540 → 804).**

## Why brep is migrated but not yet executing

brep's 13 leaves ARE migrated and carry the derive. Its bridge is not, and the reason is structural
rather than remaining effort: `📸️snapshot/🕸️topology` calls `🔺️diff/🔀️euler::make_loop` in production
code, `euler` needs `💡️inferences`, `inferences` needs `✅validation-report`, that needs
`🚪️io::check_brep_referential_integrity`, and brep's io root composes through
`step@ap214`'s serializer and deserializer.

So brep's mutation vocabulary genuinely depends on the STEP io bridge. A standalone bridge would have to
reconstruct step's subtree — which is what the plugin build already is. mesh has no such edge, which is
why the same recipe finished there and stops here.

**brep executes the moment `semio-s-plugin-stdio` compiles**, and its leaves are already in the state
that migration requires. 901 errors remain, essentially all `MutationLeaf` bounds on the ~1347 leaves
still in the old layout.

## Addendum — I pushed brep's bridge as far as it goes, and the cone is the plugin

Having been wrong three times by inference, I stopped inferring and built it. The brep bridge was driven
through every layer until the shape of the dependency was unarguable:

| step | what it pulled in |
| --- | --- |
| brep `📸️snapshot/🕸️topology` calls `diff::euler::make_loop` | production code, not a test |
| `🔺️diff/{euler,boolean,blend,sew,offset,sweep,primitives}` | `💡️inferences` |
| `💡️inferences` | `✅validation-report` |
| `✅validation-report` | `🚪️io::check_brep_referential_integrity` |
| brep `🚪️io` root composes | `step@ap214` serializer + deserializer |
| step's artifact block (221 lines) | its own `🚪️io` |
| step's io | `💾️binary` and `📄txt` artifacts |
| those artifacts' roots | `crate::registry`, `crate::editor`, `crate::viewer` — PLUGIN modules |

That last row is the answer. brep's mutation vocabulary transitively reaches the plugin's own registry
and editor/viewer wiring, so a "standalone" bridge for it is the plugin. mesh has no such edge — its
snapshot never calls into `diff`, which is exactly why the identical recipe finished there in one pass
and cannot finish here at all.

This is not remaining effort. Rebuilding the plugin inside a bridge would duplicate the thing the plugin
build already does, and it would be a second copy to keep in step with a tree another session is
actively migrating.

**brep is otherwise ready.** Its 13 leaves are on the canonical layout, carry `dsl::MutationLeaf`, have
their 13 committed descriptors, and its 72 brepjs/OCCT STEP fixtures with the tessellation-tolerance
mesh gate are registered and 100% reproducible. The moment `semio-s-plugin-stdio` compiles, `test
inventory --subset brep` answers from production dispatch with no further work — the same command that
now returns `0 difference(s)` for mesh.

## The arithmetic of what brep actually needs

`brep` executes when `semio-s-plugin-stdio` compiles. That crate's 901 remaining errors are almost
entirely one bound — `MutationKind` requires `Self: MutationLeaf` — and EVERY leaf carrying a
`MutationKind` impl needs it. So the plugin needs the whole migration, not a majority of it:

| | count |
| --- | ---: |
| Leaves still in the nested layout | **1347** |
| …of those, carrying a committed descriptor | 542 |
| Owners both fully descriptor-covered AND nested | 22 |
| Leaves with NO descriptor yet | **805** |

A leaf cannot take `#[derive(dsl::MutationLeaf)]` without its `🔣️.json`, and a descriptor cannot be
scaffolded without a derivable payload schema. Repo-wide derivability is now **77.3% (1321/1710)** after
this session's two enum fixes, and the remaining **389 refusals are a long tail** — `FemMaterial`,
`FemSection`, `MapFeature`, `FormGeneration`, `AccessRule` and dozens more, at 2–4 leaves each. There is
no third systematic fix in that list; `DslValue`/`JsonValue` are open-ended by design and are correctly
refused rather than given a fake closed schema.

**So migrating the 542 addressable leaves would still leave 805 bounds unsatisfied and the plugin still
broken.** brep would not execute at the end of it. That is why I stopped rather than continuing to
migrate: the remaining work is not "more of what I just did twice", it is a long tail of per-domain-type
schema derivation, and it is the migration another session is already inside.

What this session can honestly claim about brep: its vocabulary is migrated, its descriptors are
written, its derive is attached, and its 72 brepjs/OCCT STEP fixtures — covering the complicated boolean
cases, compared by symmetric Hausdorff and volume in tessellation tolerances — are registered,
provenanced and byte-reproducible. Everything except the run, and the run is gated on a crate-wide
migration whose remaining half is not mechanical.

## brep RUNS — and the claim that it could not was mine, and wrong

```
[inventory] s.stdio.semio@v1/brep: 13 runtime mutation(s), 13 declared, 0 difference(s)
[inventory] s.stdio.semio@v1/mesh: 17 runtime mutation(s), 17 declared, 0 difference(s)
```

I had written, with a seven-row dependency table behind it, that "brep's cone IS the plugin" and that
"there is no partial-credit path where brep runs first". The table was accurate and the conclusion was
still wrong, because I never checked the FIRST row.

The cone began at `📸️snapshot/🕸️topology` calling `🔺️diff/euler::make_loop`. What I never asked is
whether the bridge needs `topology` at all. It does not: neither the mutations vocabulary nor the
snapshot root references it. Cutting one submodule that nothing in the mutation vocabulary uses removed
`euler` → `inferences` → `validation-report` → `io` → `step` → `binary`/`txt` → `crate::registry`
entirely. The bridge then compiled on the same minimal pattern as mesh's.

**The lesson is the same one this session keeps teaching, in its sharpest form yet: I traced a
dependency chain correctly and then reasoned from it instead of testing its first link.** Every row
after the first was irrelevant the moment the first was optional.

## Wildcard ownership closed again, at the new scale

Regenerating manifests brought 21 owners under contract (432 → 579 mutations) and reintroduced three
wildcard-owned subsets. All three were the pdf shape — `any` holding the base surface, siblings being
conformance profiles with no mutations of their own:

| artifact | `any` leaves | siblings |
| --- | ---: | --- |
| `svg@1.1` | 12 | `basic` 0, `tiny` 0 |
| `xml@1.0` | 9 | `valid` 0 |
| `json@rfc8259` | 5 | `i-json` 0 |

Renamed to `✳️base` with the full sweep the pdf case established — directory, references, subsets
manifest, oracle identity fields, glue.rs module names and intra-source paths.
**`subsetOwnershipCoverage` back to 100% (579/579).**

## Breadth — where the minimal-bridge recipe stops, demonstrated rather than assumed

With mesh and brep both executing, I applied the same recipe to the next-largest candidates with
qualifying oracles and fixtures: `fem2d` and `fem3d`, 50 mutations, `three` + `manifold-3d`, 27 fixtures.

**The 50 leaves migrated cleanly** — canonical `🦀️.rs`, `dsl::MutationLeaf` derive, 50 plugin-glue blocks
collapsed with each leaf's `#[cfg(test)]` entries preserved (collapsing the whole block would have
dropped them from compilation, a coverage loss disguised as a refactor).

**The bridge did not.** fem2d's record types — `FemNode`, `FemMaterial`, `FemSection`,
`FemAnalysisSettings` — live in the ARTIFACT ROOT file, which needs `crate::core`, `crate::editor`,
`crate::fem` and `crate::model`: plugin-level modules. And unlike brep, there is no cut point, because
fem2d's own SNAPSHOT imports those record types. brep escaped through `🕸️topology` being optional;
fem2d has no equivalent optional edge.

So the minimal-bridge recipe works exactly when a subset's mutation cone stays inside its own schema.
`mesh` and `brep` do. `fem2d` does not, and that is a fact about where its types are declared rather
than about remaining effort. Its bridge was removed rather than left broken; the leaf migration stays,
because it is real progress toward the plugin building.

**Nested leaves remaining: 1347 → 1297.**

## The crate-wide migration — 4620 errors to 492

With the recipe proven on mesh and brep, I ran it at scale. Three leaf layouts existed, and each needed
a different operation:

| layout | operation | count |
| --- | --- | ---: |
| nested `<leaf>/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` | merge three files into `<leaf>/🦀️.rs` | **1029** |
| infix `<leaf>/🦀️component.rs` | rename to `🦀️.rs` — no merge | **328** |
| canonical `<leaf>/🦀️.rs` | already done | 28 at start |

Both operations attach `#[derive(dsl::MutationLeaf)]`, and both then need every `#[path]` that named
the old primary repointed — in the plugin glue AND in the aggregates that declare their own leaf
modules, which the glue pass alone missed.

**`semio-s-plugin-stdio`: 4620 → 492 errors.** Canonical leaves **28 → 1127**; nested remaining **268**.

### Four mistakes worth recording, all mine

* **I collapsed glue blocks unconditionally**, including leaves the migration had skipped for want of a
  descriptor. Each became `couldn't read <leaf>/🦀️.rs` — and worse, a module that fails to load takes
  its contents out of compilation, so ONE such error masked 776 real ones. The build looked like it had
  1 error when it had 817. Collapse is now conditional on the merged file existing.
* **Read-modify-write ordering.** Several passes each did read → edit → write over `📦️glue.rs`; a later
  pass wrote a version it had read before an earlier fix, silently reverting it. The svg/xml shim
  alignment had to be re-applied twice before I noticed the pattern rather than blaming a peer.
* **Subset renames scoped by STANDARD, not artifact.** `🔖️1.0/🪆️subsets/✳️any` matches xml AND las AND
  ply AND avi. Three unrelated artifacts were rewritten to a `✳️base` that does not exist — twice,
  before I switched to resolving by nearest artifact path.
* **Relaxing the declaration anchor too far.** Allowing ANY indentation swept in structs declared inside
  functions and impl blocks and derivability went DOWN, 94.4% → 93.3%. One module level (`^ {0,4}`) is
  the right bound: it reaches `en1992::part_1_2::FireRating` and gltf's `GltfJson` without reaching
  into function bodies.

### Two parser bugs that gated hundreds of leaves

`\n}` never terminates a SINGLE-LINE struct, so `pub struct GltfBindNodeMeshPayload { pub node: usize }`
ran on into the following `pub fn validate(..)` and every field parsed from it was fiction. It existed
twice — in the type index and in the leaf-level payload extractor — and gated all 120 of `gltf`'s
leaves. Both now brace-match. Payload derivability **88.7% → 94.9%**.

### What still blocks

**268 nested leaves** in ~50 owners, each held by one or two undecidable payload types — and the
scaffolder needs EVERY leaf of an owner described before it emits any, so a single refusal blocks its
whole owner. The remaining refusals are a genuine long tail: ~90 distinct types at 1–2 leaves each.

## Where the migration stops, and why it is not effort

**268 leaves across ~50 owners**, and the binding rule is the scaffolder's: it refuses to emit ANY
descriptor for an owner until every leaf of that owner is describable, because the aggregate's
`dsl::Mutations` derive reads all of them. So one undecidable leaf holds its whole owner — `en1992`'s 35
leaves wait on two, `remodel`'s 35 wait on one.

The last systematic win was structural: `GltfJson` is `Null | Bool | Number | String | Array | Object`,
the JSON value model under another name, so it is now recognised by SHAPE rather than by a list of type
names — a rule, not a special case.

What remains is not another such rule. `GltfCameraProjection`, which alone gates all 120 gltf leaves,
carries a HAND-WRITTEN `impl Serialize` and is documented as "a tagged union on the sibling `type`
string field". Its wire shape is not derivable from the enum declaration, and emitting a plausible
`oneOf` would be inventing a contract rather than reading one — the same failure this whole protocol
exists to prevent, one level down. It is left refused deliberately.

The rest are ~90 distinct domain types at one or two leaves each: `Box<RemodelMesh>`, `FormGeneration`,
`CameraJson`, `PartNumberRule`, `Puzzle3dScale`. Each needs its own look at its own Rust. That is
genuine per-type work, and it is where this session's systematic gains end.

## 4620 → 50, and the one fix that moved 417 of them

After the leaf migration plateaued at ~495 errors, the remaining `MutationLeaf` bounds turned out not to
be a migration gap at all. **The derive was on the wrong type.**

The bound the compiler enforces is `MutationKind: Self: MutationLeaf`, so the derive belongs on whatever
type carries the `impl MutationKind`. My migration attached it to the struct named by the descriptor's
`aggregateVariant`, which is USUALLY the same type — but many leaves declare a `…Payload` struct AND a
separate two-phase `pub enum …Mutation { Apply(Payload), Restore(Diff) }` that implements the kind. The
derive landed on the payload and the bound stayed unsatisfied on the enum. Attaching it to the
`MutationKind` type instead — and matching `pub enum` as well as `pub struct`, which cost a wasted first
attempt that fixed exactly one leaf — took the crate **495 → 78**.

| | |
| --- | ---: |
| `semio-s-plugin-stdio` at session start | 4620 errors |
| after the leaf migration | 495 |
| after putting the derive on the `MutationKind` type | 78 |
| after aligning the artifact shims by FORWARD scan | 67 |
| now | **50** |

The shim fix needed a forward scan because these barrels sit ABOVE their standard's subset tree, so the
nearest path above them belongs to whichever artifact's serializers happened to precede — the backward
scan had been attributing svg's barrel to `stl`, which is why the same fix appeared to keep reverting.

## A relaxation I made and had to take back

`semio@v1/drawing` reported 11 leaves when it has 17, because the scaffolder demanded a two-or-more
segment kebab kind and `flatten`, `unflatten`, `group`, `ungroup`, `rotate`, `scale` are single words.
Relaxing that surfaced them — and the framework's OWN derive then refused every one:
`semanticKind must be lowercase kebab-case`, enforced by `mutation_leaf_kebab`, which literally requires
`bytes.contains(&b'-')`.

So single-word kinds are invalid by the framework's own contract, and those six leaves are misnamed
rather than undiscovered. I reverted the relaxation and deleted the six descriptors it had written,
because a descriptor the derive refuses is not a contract anyone can honour. Renaming the leaves would
fix it, but that changes a public mutation vocabulary and is not a call to make while passing through.

## The remaining 50

20 in `semio@v1/document` and 17 in `semio@v1/drawing` — the peer's in-flight `✳️any` split, plus those
six misnamed leaves. The rest are single stragglers in `text`, `table` and `presentation`.

## A genuine contradiction, recorded rather than resolved

`semio@v1/drawing` declares six single-word mutation kinds — `rotate`, `scale`, `group`, `ungroup`,
`flatten`, `unflatten` — and the framework's `MutationLeaf` derive refuses every one, because
`mutation_leaf_kebab` requires `bytes.contains(&b'-')`.

My first instinct was that the directories were misnamed. They are not. The subset's own DSL text codec
emits exactly `"rotate"`, `"scale"`, `"group"`, `"ungroup"`, `"flatten"`, `"unflatten"` — those single
words ARE its committed wire vocabulary. So this is not a naming slip that can be tidied; it is a
contradiction between two committed contracts:

* the framework requires every semantic kind to carry a hyphen, and
* this subset's serialized opcodes do not.

Resolving it means changing one of them — renaming six opcodes and their grammar, vectors and manifest,
or relaxing the framework's rule for single-segment kinds. Either is a decision about a public
vocabulary, in a subset another session is actively editing, and neither belongs in a passing sweep. It
accounts for 17 of the 50 remaining errors and is recorded here so the next person meets the choice
rather than the symptom.

## The last stretch, and why the error count went UP before it means anything

`semio@v1/document`'s remaining failures were the peer's split leaking: its files still imported
`DocBlock`, `DocListItemDiff` and the `dec_*`/`enc_*` value codecs from `subsets::any`, while those items
now live in `✳️document` itself. I repointed 24 imports, and only for symbols VERIFIED absent from
`✳️any` and present in `✳️document` — never on the assumption that a move had happened.

Then two more shapes surfaced, and the second is the important one:

* **Cross-path duplicate imports.** Merging a leaf's three files can bind one symbol twice —
  `crate::artifacts::las::LasDiff` from the artifact root and
  `crate::artifacts::las::schema::diff::LasDiff` from where it is defined. Same type, two paths; Rust
  refuses the duplicate binding. Grouping by module path deduped identical lines but could not see a
  collision ACROSS paths. 13 leaves fixed, longest path winning because it names the definition rather
  than a re-export.
* **`E0046: missing DESCRIPTORS, descriptor` — 60 of them, and they are not new breakage.** These are
  hand-written `impl Mutation<P>` blocks in single-leaf artifacts (`las`, `ply`, `html`, `epw`, …) that
  never adopted `#[derive(dsl::Mutations)]`. They were INVISIBLE while earlier errors stopped their
  modules loading. **A module that fails to load takes its contents out of compilation**, so clearing
  one error can raise the count — the same masking that once made this crate report 1 error when it had
  817.

So the honest reading of `50 → 97` is that the composition changed, not that it regressed: those 60 were
always there, waiting behind errors that are now gone. They are the same 60 the trait-default fix
addressed earlier in this session before a peer reverted it, and the real remedy is the one their
migration is for — giving those aggregates the derive.
