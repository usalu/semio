# 🚧️ The last 49 need export changes, and the crate they live in does not compile

Coverage is **609/658 (92.55%)**. The remaining 49 were traced to a single cause: they mutate state
that **no carrier records**. Closing them therefore means changing EXPORTERS, not adding oracles.

That work was attempted and is **blocked**, for a reason outside this ticket.

## The two exporters that would close 13 of the 49

`mathematical` 9 and `sequence` 4 need their composed scene to reach a carrier. Both exporters are
one small change away in principle — and both currently **misdeclare their own fidelity**:

| Exporter | Declares | Actually writes |
|---|---|---|
| `MathematicalIntoJson` | `IoFidelity::Exact` | `serde_json::to_value(snapshot)` — the composed children are `ArtifactChild` handles whose `local_owner` is `#[serde(skip)]`, so graph and geometry are dropped |
| `SequenceIntoJson` | `IoFidelity::Exact` | same shape — steps and edges are behind the `content` handle and never written |
| `MathematicalIntoCsv` | `IoFidelity::Lossy` | graph nodes only (`id,label,x,y`) — **honest**, and exactly why its csv oracle covers exactly the five node kinds |

The fix is to emit the materialised scene (`mathematical_graph` / `mathematical_geometry`, which read
the live owner) rather than the opaque handles. Two of these three fidelity labels are wrong, and no
gate currently checks fidelity labels at all.

## Why it could not be done

```
$ cargo build -p semio-s-plugin-mathematical --offline
error: could not compile `semio-s-plugin-stdio` (lib) due to 124 previous errors
```

`semio-s-plugin-stdio` is the dependency every one of these exporters sits behind. Measured:

| Error | Count |
|---|---|
| `E0277` (trait bound) | 54 |
| `E0433` (`cannot find 'any' in 'subsets'`) | 37 |
| `E0432` (unresolved import) | 14 |
| `E0308` (mismatched types) | 1 |

Concentrated in `🗿️artifacts/🧿️semio` (70) and `🗿️artifacts/📜️docx` (30) — module-tree resolution in the
glue, which is the shape of the in-flight aggregate rename this ticket recorded on 2026-08-27
(commit `d394744295` added an `aggregate source is not the taxonomy canonical mutation primary` check
whose demanded renames had not landed).

### Established as NOT this ticket's doing, rather than assumed

* **No error is in a `🏭️generator` or `🔬️probes` tree** — the only Rust this ticket wrote.
* The erroring files (`🎨️svg`, `🎞️pptx` mutation leaves) carry git status **`A`** — staged additions
  from the repository's auto-commit tooling — not `M`. Nothing here rewrote them.
* Every crate this ticket added still builds clean on its own: the gif@87a reader, the fem json
  engines, the pdf lopdf engines. All are standalone `[workspace]` roots and depend on their one
  third-party library, so none of them is affected by the plugin's state.

Per CLAUDE.md a peer's in-flight refactor is not chased.

## Consequence

An exporter change made now could not be compiled, let alone verified, and this ticket does not claim
work it has not run. The 49 stay uncovered and honestly recorded.

**What is ready the moment `semio-s-plugin-stdio` builds:**

1. Emit the materialised scene from `MathematicalIntoJson` and `SequenceIntoJson`, and correct their
   `IoFidelity` declarations. The carrier readers for both patterns already exist in this ticket —
   `🦀️json-engine` under `fem2d`, `fem3d`, `draw`, `semio/document` and `mathematical` — so once the
   fields are exported, registering them is the mechanical part.
2. The 36 encoder-side kinds each need their own writer fix (tiff endianness, bmp private header
   fields, jpg quantisation/Huffman, png `tIME`, pdf `/Encrypt`, gif aspect-ratio and interlace).
3. A `fidelity-declaration` gate would have caught the two false `Exact` labels; nothing checks them
   today.
