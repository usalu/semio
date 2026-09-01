# 🔍️ The remaining 60, and why each group is a different problem

External-oracle coverage is **598/658 (90.88%)**. The 60 that remain are not one backlog. They fall
into three groups with genuinely different causes, and only one of them is more of the same work.

## Group 1 — 34 kinds: our own EXPORT discards the field

| Subset | Kinds | What the reader would need that our encoder never writes |
|---|---|---|
| obj@3.0 | 10 | tobj is a mesh reader; the kinds touch document-only structure it does not model |
| pdf (4 subsets) | 8 | `insert/remove-encryption-dictionary` — lopdf's writer needs real encryption state for a `/Encrypt` trailer entry |
| jpg@jfif-1.01 | 6 | our encoder discards quantisation/Huffman tables and the restart interval |
| gif@89a | 5 | pixel-aspect-ratio byte, comment and application extensions — outside `gif::Encoder`'s public write surface |
| png@1.2 | 3 | `tIME`, and unknown ancillary chunks the `png` crate skips |
| tiff@6.0 | 1 | `change-byte-order` — our encoder hardcodes native endianness |
| bmp@v3 | 1 | `change-header-fields` — private struct fields |

These are **export-correctness work, not oracle research**. In each case a qualifying third-party
reader exists and would witness the kind the day the writer emits it. Every one is recorded
`-uncarried` with a source-verified reason rather than routed around.

## Group 2 — 14 kinds: the mutated state is not in the subset's own carrier

`mathematical` (10) and `sequence` (4). Both look like the `fem`/`draw`/`document` case that was just
solved by reading the JSON carrier, and both are **not**, for a reason worth stating exactly:

```rust
// 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
pub struct ArtifactChild<S> {
    pub child_id: String,
    pub target: crate::os_io::ArtifactRef,
    #[serde(skip)]
    local_owner: Option<Arc<dyn std::any::Any + Send + Sync>>,
    ...
}
```

`MathematicalSnapshot`'s `notation`/`results`/`computed` and `SequenceSnapshot`'s `content` are
`ArtifactChild` handles. `mathematical_children_from_state(&graph, &geometry)` builds them with
`.with_local_owner(...)`, and `mathematical_scene` reads the graph and points back **out of that
owner**. Because `local_owner` is `#[serde(skip)]`, `serde_json::to_value(snapshot)` emits only
`{childId, target}` — the graph, the points, the steps and the edges **never reach the subset's own
JSON at all**.

So these 14 are not witnessable in their own carrier by any reader, however good. Closing them means
reading the composed CHILD artifact's carrier (`s.stdio.semio.text` / `.table` / `.value` / `.flow`)
and relating it back — a composed-artifact testing story this ticket has not built.

**This was checked, not assumed**, and it corrects an earlier estimate in this ticket that counted all
34 non-`-uncarried` kinds as "addressable".

## Group 3 — 12 kinds: `gif@87a`, writer-blocked

Fully documented in `📓️gif-87a-conformance-and-writer-limits.md`. `gif` 0.13 cannot write a GIF87a at
all (hardcoded signature, unconditional Graphic Control Extension); Pillow writes conformant
single-image 87a but sets no interlace bit and reintroduces GCE for multi-frame. A partial retrofit of
the seven single-image kinds is sound and unblocked; interlace and the three multi-image kinds need a
writer neither library provides.

## The discriminator that decided groups 2 and 3

The same question closed 71 kinds and blocked 14: **is the state this mutation changes a fact about
the subset's own carrier, or about something else?**

* `fem2d`/`fem3d` — nine collections, all INLINE in the snapshot → 44 closed.
* `draw` — `layers: Vec<DrawLayerNode>`, INLINE → 3 closed.
* `semio@v1/document` — `images: Vec<DocImage>` with raw bytes, INLINE → 3 closed.
* `semio@v1/cad` — already had a qualifying dxf reader AND fixtures; the `-uncarried` marker was
  simply stale, and the reader was measured naming each change (`entity[3] layer differs: "0" vs
  "ANNOTATIONS"`, `block[0].entity[0] layer differs: "0" vs "LEAF"`) → 2 corrected.
* `mathematical`/`sequence` — COMPOSED behind `#[serde(skip)]` handles → not closable this way.

A reader is only as good as the carrier it reads. Most of this ticket's remaining work is about
carriers, not about libraries.

---

## Update — the three groups collapse into ONE cause

Coverage is now **609/658 (92.55%)**. `gif@87a` was retrofitted (group 3 closed to 10 of 12), and
`mathematical`'s `change-coefficient` was closed. Following those through changed the diagnosis of what
is left: **groups 1 and 2 are the same problem wearing different clothes.**

### The unified statement

> Every one of the remaining 49 kinds mutates state that **no carrier records**. A reader cannot
> witness what nothing writes down.

Two mechanisms produce that:

**(a) The encoder writes the carrier but omits the field — 36 kinds.** `tiff::change-byte-order`
hardcodes native endianness; `bmp::change-header-fields` hides them behind private fields; the `jpg`
quantisation/Huffman kinds are discarded by our own encoder; `png`'s `tIME` and unknown chunks are
skipped; lopdf's writer refuses a synthetic `/Encrypt`; `gif`'s aspect-ratio byte and interlace bit
have no writer that emits them with an 87a signature; tobj models no document-only structure.

**(b) The state sits behind a composed-child handle that never serialises — 13 kinds.**
`mathematical` 9, `sequence` 4. `ArtifactChild::local_owner` is `#[serde(skip)]`, so
`serde_json::to_value(snapshot)` emits only `{childId, target}`.

### What that reclassification is worth

`mathematical` was previously filed as "needs a composed-artifact testing story". Closing
`change-coefficient` forced a closer look, and the truth is more specific:

* `equation` is **inline** → reaches the JSON export → **closed**.
* graph **nodes** reach the CSV export (`id,label,x,y`) → which is precisely why the sibling csv oracle
  covers exactly the five node kinds, no more.
* **edges**, `directed`, `algorithm` and **points** reach **no carrier at all** —
  `MathematicalIntoCsv` is declared `IoFidelity::Lossy` and emits only nodes.

So those nine need `MathematicalIntoCsv` (or a JSON export of the materialised scene) to emit them.
That is export work with a clear shape, not an open research question.

### A discrepancy found on the way, worth its own fix

`SequenceIntoJson` serialises `SequenceSnapshot` — `{schema, content}` where `content` is a child
handle — and declares **`IoFidelity::Exact`**. It is not exact: the steps and edges behind that handle
are never written. `MathematicalIntoCsv`, which drops strictly less, honestly declares itself
`IoFidelity::Lossy`. The fidelity label on the sequence exporter is wrong and no gate currently checks
it.

### Consequence for the goal

Reaching 658/658 does not need more oracles or more libraries. Every remaining kind needs its **carrier
to record the field first** — and for 13 of them, that means composed children materialising into their
own carriers rather than vanishing at the serde boundary. The ordering this ticket recorded early
holds, and is now exact: *this is export correctness.*

---

## Update 2 — `obj` +3, and the last audit of the `-uncarried` labels

Coverage **612/658 (93.01%)**. `obj`'s ten `-uncarried` kinds were re-audited the way `cad`'s two were,
and the same lesson applied a fourth time: *"the reader we registered cannot see it"* had been recorded
as *"no reader can see it"*.

`tobj` is a MESH reader — it resolves faces into vertex buffers and discards `mtllib`, `usemtl` and
smoothing-group statements entirely. `three`'s OBJLoader parses and keeps all three.

**Measured against three's OBJLoader, one kind at a time:**

| Kind | Moves the projection? |
|---|---|
| `set-mtllib` | ✅ `materialLibraries` |
| `set-usemtl` | ✅ per-child material name |
| `set-smoothing-groups` | ✅ `flatShading` from `s off` |
| `insert-vertex` / `remove-vertex` | ❌ an unreferenced vertex is dropped by this loader too |
| `insert-texcoord` / `remove-texcoord` | ❌ same |
| `insert-normal` / `remove-normal` | ❌ same |
| `set-unknown-statements` | ❌ OBJLoader skips an unrecognised line with a warning |

Three claimed, seven left `-uncarried`. **The measurement set the scope, not an estimate of it** — and
it is recorded in the oracle's own qualification criteria so the next reader of that file can see which
kinds were tested and rejected rather than never considered.

## The audit is now complete

Every one of the 46 remaining kinds has been individually checked against the readers available to its
subset. Four rounds of this audit found stale labels — `cad` 2, `draw` 3, `semio/document` 3, `obj` 3,
plus `gltf` 24 and `fem` 44 earlier. There are no more.

| Cause | Kinds |
|---|---|
| Encoder omits the field from the carrier | `jpg` 6, `pdf` 8, `gif@89a` 5, `png` 3, `gif@87a` 2, `tiff` 1, `bmp` 1 = **26** |
| Reader drops unreferenced/unknown elements, and no surveyed reader preserves them | `obj` 7 |
| State behind a `#[serde(skip)]` child handle, never serialised | `mathematical` 9, `sequence` 4 = **13** |

All three need the **carrier** to record the field. None needs another oracle, and none is closable
while `semio-s-plugin-stdio` does not compile.
