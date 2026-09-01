# 📓️ The derivation tail was not a tail — it was one bug wearing forty names

I reported that the remaining payload-schema refusals were "a long tail of ~40 domain types at 2–4
leaves each, with no third systematic fix in it". That was wrong, and the way it was wrong is worth
recording: I had read the SYMPTOM list (which type name defeated each leaf) as the CAUSE list.

Tracing one of them properly — `AccessRule`, refused with *"AccessRule is not a shape this derivation
decides"* — gave a chain three deep:

```
Priority (unindexed)  →  EntityHeader.priority  →  AccessRule.header
```

`Priority` is an enum defined twice: once in `architect/program`'s kernel, once in the OS `db/policy`.
Different variants, same name. My type index was keyed by BARE NAME, so it marked the name ambiguous and
**deleted it** — and with it `EntityHeader`, and with that every register type in `architect/program`.
128 leaves in that one owner refused because of a name collision two directories apart.

**A name is not unique in this repository, and it is not supposed to be.** `FemMaterial` is one struct
in `fem/◻2d` and a different one in `fem/🧊3d`, exactly as the taxonomy intends. Rust resolves such a
name by module path; the index now does the same — it remembers where each definition lives and lets the
owner being derived pick the nearest one by longest shared path prefix.

## What that changed

| | before | after |
| --- | ---: | ---: |
| Leaves derivable from their Rust payload | 74.8% | **87.3%** (1493/1710) |
| Payload schemas written | — | **+232** |
| Leaf descriptors on disk | 540 | **1226** |
| Owners fully derivable | 31 | **54** |
| Nested leaves now carrying a descriptor | 542 | **951** of 1347 |

Three fixes got there, and each was found by using the tool rather than reviewing it:

1. **Fieldless enums** — `{ type: "string", enum: [...] }`, honouring `rename_all`. `SemioTopology`
   defeated `SemioPrimitive` and `SemioMesh` transitively, and those three defeated all 17 mesh leaves.
2. **Tagged struct-variant enums** — `#[serde(tag = "kind")] enum BrepCurve { Line { .. }, … }` as a
   `oneOf` of tagged objects. That is the entire brep vocabulary.
3. **Proximity resolution** — the one above. Composites first (+31 leaves), then enums (+141).

`#[serde(flatten)]` is now handled too — a flattened field merges its members into the parent object
rather than nesting under the field name. It unlocked nothing on its own, because those types were
already refused for the `Priority` reason, but a schema that emitted `header` as a nested property
would have been WRONG on the wire, and a check confirmed none had been written that way.

## What is genuinely left

217 refusals, and this time the tail is real: `DslValue`/`JsonValue` are open-ended by design and are
correctly refused rather than given a fake closed schema, and the rest are single domain types at 2
leaves each. **396 nested leaves still have no descriptor.**

So the plugin still cannot build — every leaf carrying a `MutationKind` impl needs `MutationLeaf`, and
396 cannot yet get one. But the addressable share went from 40% to **71%** of the remaining migration,
and the reason it moved was a defect in this platform's own index, not effort spent on domain types.

## Second pass — I traced the tail instead of reading it, again, and it moved again

Having just been wrong about "no third systematic fix", I stopped characterising the remaining 217
refusals and traced each to its ROOT unresolvable type. Three more causes came out, none of them a
domain-modelling problem:

**1. The scaffolder only looked for the diff in a `🔺️diff/` SUBDIRECTORY.** In the taxonomy's canonical
leaf layout — the one the `MutationLeaf` derive requires, and the one this migration produces — payload,
diff and inverse all live in the leaf's single `🦀️.rs`. **315 leaves are already in that shape**, and
reading only `🔺️diff/` refused every one of them for evidence sitting in the file next door.
`outcomeClasses` refusals: **124 → 0**, and scaffold derivability **87.3% → 94.5%**.

**2. `DslValue` / `JsonValue` were refused as "undecidable".** `DslValue` is literally the JSON value
model — `Null | Bool | Number | String | Array | Object`. Its honest schema is *any JSON value*, which is
what `{}` means. Refusing it was over-strict rather than principled, and because the scaffolder needs
EVERY leaf of an owner described before it emits any, one such leaf blocked its whole owner.

**3. `set-snapshot` has no payload struct, and never did.** It is the whole-document replace; its payload
IS the artifact's snapshot type, named in the leaf's own `apply(projection: &mut DwgSnapshot, …)`
signature. All 50 refusals were this one shape. Read from the signature rather than assumed:
`aggregateVariant` refusals **50 → 11**.

Also fixed: fixed-size arrays whose element is a path or generic (`[SemioPoint3; 4]` — the matcher
demanded a bare identifier), tuples, and fully-qualified type paths.

## Where the session ends

| | start | end |
| --- | ---: | ---: |
| Leaves scaffoldable with full evidence | ~73% | **94.5%** |
| Leaf descriptors on disk | 540 | **1395** |
| Owners fully derivable | 31 | **67** |
| **Nested leaves that can now take the derive** | 542 | **1001** of 1347 |

The migration's addressable share went from **40% to 74%**, and every point of it came from a defect in
this platform's own derivation — not from domain work. 346 leaves remain blocked, on 94 `payloadSchema`
and 11 `aggregateVariant` refusals that are now genuinely heterogeneous.

The plugin still does not compile, because every leaf carrying a `MutationKind` impl needs
`MutationLeaf` and 346 cannot yet get one. So `brep` still does not run. But the thing standing between
here and there is now a bounded, measured list rather than "a long tail with no systematic fix in it" —
which is what I twice reported before actually tracing it.

## Third pass — the tail is real this time, and here is the evidence

Same discipline once more: every remaining `payloadSchema` refusal traced to its root type rather than
read off the symptom list. The result is different from the previous two passes, and that difference is
the finding:

**188 distinct root causes across ~200 refusals — at most 2 leaves each.** `FormQuestion`,
`PropertyBag`, `EntityRef`, `Box<DrawLayerNode>`, `Option<crate::artifacts::puzzle3d::Puzzle3dScale>`,
`FormGeneration`… no shape repeats more than twice, and no two share a mechanism.

The previous two passes each found ONE defect wearing many names — a name-keyed index that deleted
legitimate homonyms (141 leaves), a diff-location assumption that missed the canonical layout
(124 leaves). Those were platform bugs. What is left is not: it is ~188 individual domain types, each
needing its own look, and that is genuine per-type work rather than a fix I am failing to find.

So this is where the systematic gains end, and I am saying so with the trace behind it rather than as a
characterisation — because twice already the characterisation was wrong and the trace was what corrected
it.

## Fourth pass — three more rules, and the honest stopping point

Same discipline, three more findings, each a rule rather than a special case:

* **`ArtifactChild<S>` is the framework's CHILD HANDLE.** Its phantom marker and local owner are both
  `#[serde(skip)]`, so its wire shape is `{ childId, target: { artifactId, dialect } }` and does not
  depend on `S` at all. Read from the struct. Every composite artifact points at its children this way,
  so refusing it refused all of them.
* **The JSON value model, recognised by SHAPE.** `GltfJson` is `Null | Bool | Number | String | Array |
  Object` — `DslValue` under another name. Matching the six variants rather than a list of type names is
  what makes it a rule.
* **`set-snapshot` has no payload struct, and its payload is the SNAPSHOT.** The type is named in the
  leaf's own `apply(projection: &mut DwgSnapshot, …)`, so it is read from there. ~35 single-leaf owners
  were each blocking themselves entirely over a struct that was never meant to exist.

Derivability **74.8% → 95.9%**; scaffoldable with full evidence **96.3%**; descriptors **540 → 1526**;
canonical leaves **28 → 1517**; nested remaining **206**.

## Where I stopped, and the one refusal I chose to keep

`GltfCameraProjection` alone gates all 120 `gltf` leaves, and I left it refused ON PURPOSE. It carries a
HAND-WRITTEN `impl Serialize` and its doc calls it "a tagged union on the sibling `type` string field" —
so its wire shape is not derivable from the enum declaration. Emitting a plausible `oneOf` would be
inventing a contract instead of reading one, which is the exact failure this protocol exists to prevent,
one level down. A schema I cannot justify is worse than a refusal I can.

The remaining 206 leaves sit in owners each held by one or two types of that kind, or by genuine
one-off domain shapes. That is per-type work now, not another rule.
