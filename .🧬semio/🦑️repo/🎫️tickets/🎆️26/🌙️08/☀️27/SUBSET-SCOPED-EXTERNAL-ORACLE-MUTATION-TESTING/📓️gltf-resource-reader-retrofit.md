# 🧱️ gltf@2.0/any — 24 kinds closed by asking which reader, not whether

`gltf@2.0/any` already carried a qualifying reader (`three-gltf-2-0-mutate-reader`) covering 96 of its
120 mutation kinds. The remaining 24 were recorded `gltf-2-0-mutate-uncarried`: create, delete, move
and reorder over the six glTF RESOURCE arrays — accessors, buffers, bufferViews, images, samplers,
textures.

That label was **honest about three and wrong about the world**, and the distinction is the whole
finding.

## Why three cannot witness them

`three`'s `GLTFLoader` builds a **scene graph**. A resource nothing references never becomes an
object, so an unreferenced accessor or texture is carried through as opaque JSON and interpreted by
nothing. The existing projection says exactly this and omits the six deliberately — a correct decision
that was then generalised into "unwitnessable", which does not follow.

## Two candidate readers, one rejected on measurement

| Capability | `three` 0.182 | `@gltf-transform/core` 4.4.2 | `gltf` (rust) 1.4.1 |
|---|---|---|---|
| accessors as standalone facts | ❌ scene graph only | ✅ `listAccessors` | ✅ `accessors()` |
| buffers | ❌ | ✅ `listBuffers` | ✅ `buffers()` |
| bufferViews | ❌ | ❌ **no `listBufferViews`** | ✅ `views()` |
| images vs textures, separately | ❌ | ❌ **folded into one `Texture`** | ✅ `images()` / `textures()` |
| samplers | ❌ | ❌ no `listSamplers` | ✅ `samplers()` |

`@gltf-transform/core` was built, wired and validated first — it round-trips an unreferenced accessor
correctly. It was then **rejected**: enumerating `Root`'s `list*` methods showed no `listBufferViews`,
`listImages` or `listSamplers`. It models glTF's separate `images`, `samplers` and `textures` arrays as
a single `Texture`, so it cannot tell `create-image` from `create-texture` and cannot see a bufferView
at all. It would have covered 6 of 24 **while appearing to cover more** — the failure mode this whole
protocol exists to prevent.

`gltf` 1.4.1 exposes all six as separate typed iterators matching the specification's own structure, so
each of the 24 kinds lands on exactly one observable list. Registered as `gltf-rs-2-0-mutate-reader`,
backed by a standalone `[workspace]` crate depending only on `gltf`.

It parses the **document only** (`Gltf::open`, not `gltf::import`): the resource arrays are fully
determined by it, and decoding pixel data would only add ways for a fixture to fail to read — as it
did, when a deliberately tiny test PNG made `gltf::import` fail on a document whose arrays were fine.

## The reader earned its keep before it was registered

Running the 24 new fixtures through it caught **three real defects in the recipes that produced them**:

1. `delete-accessor` removed a *referenced* accessor, orphaning an animation sampler's required
   `output` — `missing field \`output\``.
2. `delete-image` left `textures[1].source` dangling — `invalid glTF: textures[1].source: Missing data`.
3. `reorder-bufferViews` was emitted under the camelCase array name; the declared kind is the
   kebab-case `reorder-buffer-views`, so the fixture simply did not exist.

All three now delete or move an **unreferenced spare**, mirroring how the existing `delete-node` recipe
uses `soloNode`: removing a resource something still points at does not exercise deletion, it produces
an invalid document — and a third-party reader will rightly refuse the whole fixture rather than judge
the mutation.

## Evidence

* **48/48 directions correct** — 24/24 `(before, before)` compare `equal:true`; 24/24 `(before, after)`
  compare `equal:false`.
* Discriminating case: `create-image` moves images **3 → 4** while textures stays **2 → 2**.
* Base document reads **18 bufferViews**, which three omits by design and gltf-transform cannot list.
* All **120** fixture bundles regenerate **byte-identically** (aggregate sha256 unchanged across a full
  regeneration). The 96 pre-existing bundles were verified unmodified — `git diff` against `HEAD`
  reports no content rewrite, only additions.

## Result

| | before | after |
|---|---|---|
| externalOracleCoverage | 431/658 (65.50%) | **455/658 (69.15%)** |
| oracleEvidenceCoverage | 351/658 (53.34%) | **375/658 (56.99%)** |
| Fixtures | 705 | **729**, 100% provenance and reproducibility |
| Harness | 116/116 | **120/120** |

The base document was deliberately **not** modified to carry images/samplers/textures: it is the
`before` of all 96 committed fixtures and changing it would have invalidated every recorded hash. The
texture-family recipes grow their own resources in their own `before` instead.
