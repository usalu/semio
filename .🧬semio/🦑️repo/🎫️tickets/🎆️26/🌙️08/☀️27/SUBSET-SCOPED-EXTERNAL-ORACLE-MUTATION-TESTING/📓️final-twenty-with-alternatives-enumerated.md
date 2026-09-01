# 🔒️ The last 20, each with its alternatives enumerated

Coverage **638/658 (96.96%)**. Four negatives in this ticket were overturned by asking *"what else in
this library could do it, and what other instance of this mutation would look different"* — gif
interlace, pdf encryption, obj vertices, tiff byte order. So the remaining twenty were put through the
same question deliberately, rather than being left on their first recorded reason.

Each row below states what was CHECKED, not what was assumed.

## `mathematical` 9 + `sequence` 4 — the scene reaches no third-party-readable carrier

Their mutated state lives behind `ArtifactChild` handles whose `local_owner` is `#[serde(skip)]`, so
`serde_json::to_value(snapshot)` emits only `{childId, target}`.

**Alternative checked:** these subsets also have a `🚪️io/📸️snapshot/📝️text` codec, and it *does*
materialise the scene — `mathematical`'s text codec handles `MathematicalGraph`, `MathematicalNode`
and `MathematicalEdge` directly. So a carrier carrying the graph exists.

**Why it still does not qualify:** that carrier is the **semio DSL**, this repository's own grammar.
No third party parses it, so it cannot be the judged side of a comparison. A reader we wrote would be
the predicting-oracle mistake this whole ticket exists to prevent.

The CSV export carries graph nodes only and honestly declares `IoFidelity::Lossy` — which is exactly
why the sibling csv oracle covers exactly the five node kinds and no more.

→ Genuinely needs the exporters, and those live in `semio-s-plugin-stdio`, which does not compile.

## `jpg` 4 — no reader exposes the marker, or no writer can produce it

| Kind | Checked |
|---|---|
| `change-restart-interval` | Pillow WRITES it (`restart_marker_blocks`) but does not read it back. `zune-jpeg` 0.5 parses the DRI marker into `restart_interval` — but the field is **`pub(crate)`**, with no public accessor. `image`-rs decodes to pixels. No reader available. |
| `replace-huffman-table`, `remove-huffman-table` | Pillow's `huffman_ac`/`huffman_dc` return **empty** and are deprecated for removal in Pillow 12. No other available library exposes the tables. |
| `remove-quant-table` | No writer: a JPEG without a quantisation table is not a decodable JPEG. |

## `obj` 1 — `set-unknown-statements`

three's `OBJLoader` skips an unrecognised line with a warning; `tobj` is a mesh reader. A
document-preserving OBJ parser would witness it — **npm, PyPI and the vendored cargo registry were all
searched and none is available offline**.

## GIF `set-pixel-aspect-ratio` ×2 — both halves blocked, and this one is definitive

* **Reader:** the `gif` crate never reads byte 12 of the logical screen descriptor. Its only mention of
  the field is `encoder.rs:345`, `tmp.write_le(0u8)?; // aspect ratio` — it writes a hardcoded zero and
  has no parse path at all. Pillow's `info` after a GIF round trip carries only `background` and
  `version`.
* **Writer:** nothing surveyed emits a non-zero value.

Neither side exists, in either GIF version.

## What would close each

| Group | Kinds | Needs |
|---|---|---|
| `mathematical`, `sequence` | 13 | the composed scene exported to a carrier a third party can read — blocked on `semio-s-plugin-stdio` compiling |
| `jpg` | 4 | a JPEG library exposing DRI and Huffman tables (zune-jpeg would need `restart_interval` made public) |
| `obj` | 1 | a document-preserving OBJ parser |
| GIF aspect ratio | 2 | a library that both writes and reads the aspect-ratio byte; none found |

None of the twenty is closable by trying harder with the libraries present. That is a different
statement from the one this ticket kept making too early, and it is the one the evidence now supports.

---

## Inventory check — searched the whole set, not the libraries that came to mind

The alternatives above were found by asking "what else could do this". That question is only as good as
the inventory it is asked against, so the inventory itself was then enumerated: **1876 vendored cargo
crates**, all of `node_modules`, and the installed Python environment.

### What the sweep turned up

**`gif` 0.14.2 is vendored alongside 0.13.3** — a second version I had not been working with. Checked
directly: `0.14.2/src/encoder.rs:401` is `tmp.write_le(0u8)?; // aspect ratio`, the same hardcoded zero,
and it has no parse path for byte 12 either. So the GIF aspect-ratio negative now holds across **both
vendored versions**, not one.

### What it did not turn up

| Need | Searched | Result |
|---|---|---|
| A JPEG marker/structure reader (DRI, Huffman) | cargo (1876 crates), npm, Python | nothing beyond Pillow (`Skip` handler for DRI; empty deprecated Huffman accessors), `image`-rs and `zune-jpeg` (`restart_interval` is `pub(crate)`) |
| A document-preserving OBJ parser | cargo, npm, PyPI | only `tobj` (mesh) and three's `OBJLoader` + `MTLLoader`; neither retains unknown statements |
| Anything reading the GIF aspect-ratio byte | cargo (both gif versions), Pillow | nothing reads it; both gif versions only write a hardcoded `0` |

`kamadak-exif` is vendored but reads EXIF only — not the DRI or DHT segments these kinds touch.

## Standing conclusion

Twenty kinds remain, and for each the blocker is now stated at the level of a **specific symbol in a
specific version**, not as a general impression:

* `mathematical` 9 + `sequence` 4 — the only carrier holding their scene is the **semio DSL**, this
  repository's own grammar. Needs the exporters, which live in a crate that does not compile.
* `jpg` 4 — `zune-jpeg` 0.5.15 `decoder.rs:144`, `restart_interval` is `pub(crate)`; Pillow's DRI
  handler is `Skip`; its Huffman accessors return empty and are deprecated for removal in Pillow 12.
* `obj` 1 — no document-preserving parser in any of the three ecosystems.
* GIF `set-pixel-aspect-ratio` ×2 — `gif` 0.13.3 `encoder.rs:345` and 0.14.2 `encoder.rs:401`, both
  writing a hardcoded zero, neither reading.

Closing any of them requires a library that is not present, or the blocked crate to build. That is a
claim about the environment, made after enumerating it.

---

## Both guards applied to the last 18

Two failure modes have now been named in this ticket, and each overturned negatives:

* **(a) an entry point was tested, not a capability** — 4 kinds recovered
* **(b) the KIND was misread, and the negative followed** — 2 kinds recovered

Both were then applied deliberately to everything still open. What each produced:

### `mathematical` 9 + `sequence` 4 — checked at a second level

Guard (b) asks: the mutation lands in a composed CHILD, so could the child's own carrier be the
judged one? The children are real subsets — `s.stdio.semio.{text,table,value,flow}` — so the question
is fair.

**Checked directly:** all four exist and all four carry **0 mutation kinds, 0 oracles and 0 fixtures**.
They are carrier artifacts without vocabularies of their own. And materialising a child's CONTENT still
runs through the parent's `local_owner`, which is `#[serde(skip)]` — so the child cannot be exported
without the plugin, which does not compile.

Blocked at both levels, not one.

### `jpg` 3

Guard (b) recovered `remove-quant-table` by re-reading it as a change to the table LIST. The same
reading was applied to the rest:

* `replace-huffman-table` / `remove-huffman-table` — the list reading holds (a JPEG may share Huffman
  tables between components), but Pillow exposes **no write control** over Huffman tables and its read
  accessors return empty and are deprecated for removal in Pillow 12. Neither half exists.
* `change-restart-interval` — Pillow writes it (`restart_marker_blocks`) and cannot read it back;
  `zune-jpeg` parses the DRI marker into `restart_interval` but the field is `pub(crate)`
  (`decoder.rs:144`). Writer yes, reader no.

### GIF `set-pixel-aspect-ratio` ×2 — the most thoroughly closed of the set

Neither guard opens it. Both vendored versions of the crate were read: `gif` 0.13.3 `encoder.rs:345`
and 0.14.2 `encoder.rs:401` each write a hardcoded `0u8` and neither has any parse path for byte 12 of
the logical screen descriptor. Pillow's `info` after a GIF round trip carries only `background` and
`version`. There is no reader and no writer, in any available version.

## Standing position

Eighteen kinds remain. Every one has now been checked against **both** named failure modes, and each
blocker is pinned to a named symbol in a named version. Closing any of them needs a library that is not
present, or `semio-s-plugin-stdio` to compile.

---

## Closing state — 14 remain, and what was searched for each

Coverage **644/658 (97.87%)**. `pdf`, `obj` and `gif` are complete artifacts; `jpg` has one kind left.

Three failure modes were named over this ticket and all three were applied to what remains:

* **(a)** an entry point was tested, not a capability
* **(b)** the KIND was misread, and the negative followed
* **(c)** the INVENTORY was scoped, and the scope was never stated

### `jpg::remove-huffman-table` — 1

Guard (b) first: the schema says `RemoveHuffmanTableMutation { key: JpgHuffmanTableKey }` — it removes
**exactly one** DHT entry, identified by `(class, id)`, from `huffman_tables: Vec<JpgHuffmanTable>`.

Guard (c) next — searched, and stated: Pillow (no Huffman write control), `cjpeg` **every switch**,
`jpegtran` **every switch**, `zune-jpeg`, `image`-rs.

What exists produces the wrong shape of change:

| Candidate | Table count | Why it is not this kind |
|---|---|---|
| `cjpeg -grayscale` | 4 → 2 | removes two tables and changes the component count |
| `jpegtran -progressive` | 4 → 10 | adds tables and restructures the scan |
| `jpegtran -arithmetic` | 4 → 0 | removes all four; arithmetic coding has no DHT at all |
| `jpegtran -scans FILE` | unchanged | scan scripts control spectral selection, not table assignment |

And in a colour baseline **all four tables are referenced**, so a file with one removed by byte surgery
would not decode — the fixture would record a broken file rather than a mutation.

No writer produces a decodable JPEG differing by exactly one DHT entry.

### `mathematical` 9 + `sequence` 4

Checked at three levels, each recorded when it was checked:

1. Their own JSON carrier drops the scene — `ArtifactChild::local_owner` is `#[serde(skip)]`.
2. Their `📸️snapshot/📝️text` codec **does** materialise the graph — but it is the **semio DSL**, this
   repository's own grammar, with no third-party parser. A reader we wrote would be the
   predicting-oracle mistake this ticket exists to prevent.
3. Their composed children are real subsets (`s.stdio.semio.{text,table,value,flow}`) — all four exist
   and all four carry **0 mutation kinds, 0 oracles, 0 fixtures**, and materialising a child's content
   still runs through the parent's skipped owner.

They need the exporters, which live in `semio-s-plugin-stdio` — re-checked this session, still failing
with 124 errors from a peer's in-flight refactor.

### The standing guard

Every negative above states **what was searched**, at the level of a named switch, a named symbol, or a
named version. That convention is the lasting output of the three failure modes: a negative that does
not say what it covered cannot be trusted later, and this ticket produced six of them before the
convention existed.

---

## `mathematical` — all four carriers enumerated, and why each fails

Guard (c) says a negative must state what it searched. Earlier notes here said "the JSON export drops
the scene" and "the text codec is our own DSL" — true, but that was two carriers, not the set. The
subset exports to **four**, and all four were then read:

| Carrier | Fidelity | What it actually emits | Why it cannot judge the 9 |
|---|---|---|---|
| `🔣️json` | declares `Exact` — **and is not** | `serde_json::to_value(snapshot)`; the composed children are `ArtifactChild` handles whose `local_owner` is `#[serde(skip)]` | the scene never reaches it |
| `📊️csv` | `Lossy` (honest) | graph NODES only — `id,label,x,y` | exactly why the csv oracle covers exactly the five node kinds |
| `📝️md` | `Canonical` | `MdSnapshot::from_text(print_dsl(snapshot))` — the semio DSL wrapped in one markdown block | see below |
| `📄txt` | `Lossy` | DSL text blob | same |

### The markdown carrier is the interesting rejection

Markdown *is* third-party-readable, and any change to the graph or geometry does change the DSL text
inside it, so a markdown reader would report a difference. It would also be **worthless as evidence**:
the reader sees one opaque code block and compares strings. Every bit of discrimination lives in our own
`print_dsl`, and the judge contributes nothing but string equality.

That is precisely the shape this ticket already built a gate for — `stubSerializerBreaches`, the
single-blob-payload serializer. Registering the md carrier here would have raised coverage while
smuggling our own printer in as the oracle: the predicting-oracle mistake, wearing a markdown parser as
a costume.

**Rejected deliberately, and recorded, so a later reader does not "discover" it as an easy win.**

The four carriers are the whole set. Closing these nine needs one of them to emit edges, the `directed`
flag, the `algorithm` and the geometry points as structure a third party can interpret — which is
export work, in `semio-s-plugin-stdio`, which does not compile.

---

## `jpg::remove-huffman-table` — every writer option exhausted, listed

The kind removes ONE DHT entry by `(class, id)` from `huffman_tables: Vec<JpgHuffmanTable>`. A fixture
needs a decodable JPEG with exactly one fewer table than its pair, at the same image.

Every option of both installed CLI toolchains and all three libraries, tested rather than reasoned about:

| Writer | Option | Tables | Verdict |
|---|---|---|---|
| Pillow | — | 4 | no Huffman write control at all |
| cjpeg / jpegtran | *(default)* | 4 | baseline |
| cjpeg / jpegtran | `-optimize` | 4 | recomputes contents, same count — this is what closed `replace-huffman-table` |
| cjpeg / jpegtran | `-grayscale` | 2 | drops two AND changes the component count |
| cjpeg / jpegtran | `-progressive` | 10 | adds tables, restructures the scan |
| jpegtran | `-arithmetic` | 0 | arithmetic coding has no DHT at all |
| jpegtran | `-scans FILE` | 4 | **tested** — libjpeg assigns tables per component regardless of scan grouping |
| `zune-jpeg`, `image`-rs | — | — | decoders only |

And in a colour baseline **all four tables are referenced**, so removing one by byte surgery yields a
file that does not decode: the fixture would record a broken file rather than a mutation.

A grayscale JPEG's two tables (DC0, AC0) are likewise both referenced, so the count cannot be reduced
there either.

**No available writer produces a decodable JPEG differing from another by exactly one DHT entry.** That
is a statement about eight tested options across five tools, not an impression.
