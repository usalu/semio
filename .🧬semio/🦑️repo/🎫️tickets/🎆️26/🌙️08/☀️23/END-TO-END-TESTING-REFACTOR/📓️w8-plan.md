# Wave 8 plan — the remaining artifacts

Wave 7 covered the twelve artifacts whose reference library was already linked. The remaining
artifacts split three ways by what a credible third-party reference can actually do, and the split
matters: a library that cannot WRITE cannot serve as a differential oracle, and pretending otherwise
would produce a test that compares an implementation with itself.

## Group A — a reference that reads AND writes → full differential, exactly as wave 7

| Artifact | Standard | Crate | Notes |
|---|---|---|---|
| 📰xml | 1.0 | `quick-xml` 0.42 | MIT, pure Rust, pull reader + writer |
| 🎨️svg | 1.1 | `quick-xml` 0.42 | SVG's snapshot is an `XmlDocument`; same reference |
| 🔣️json | rfc8259 | `serde_json` 1 | our JSON is hand-rolled, so this is a genuine second implementation |
| 🌐️html | 5 | `html5ever` 0.39 | WHATWG-compliant, has a serializer |
| 📝️md | commonmark | `comrak` 0.54 | 100% CommonMark/GFM, read + write, actively released |
| 📑️tsv | iana | `csv` 1 **(already linked)** | same crate, tab delimiter — no new dependency |
| 🖊️dxf | r12 | `dxf` 0.6 | MIT, reads and writes DXF |
| ☁️las | 1.0 | `las` 0.11 | MIT, ASPRS LiDAR, actively released |
| 🎥️mp4 | isobmff | `mp4` 0.14 | ISOBMFF reader + writer |
| 📐️step | ap214 | `ruststep` | Apache-2.0, ISO 10303 |

## Group B — a reference that only READS → independent validator, not a producer

These get `@mode-round-trip` and `@mode-property` scenarios in which the third-party library is the
**independent reader** that projects both the real input and our re-serialized output. That is still
external validation of our output — it is simply not a differential, and the feature must not claim
to be one.

| Artifact | Crate | What is missing |
|---|---|---|
| 🎵️mp3 | `symphonia` (bundle-mp3) 0.5 | decoder only; no pure-Rust MP3 encoder exists |
| 📕️xlsx | `calamine` 0.36 read + `rust_xlsxwriter` 0.96 write | two crates; the writer cannot modify an existing workbook, only create one |
| 🌦️epw | `epw-rs` 0.1 | alpha, no write path exposed |

## Group C — no credible reference exists → recorded no-oracle decision

The platform already supports and gates this: a `noOracleDecision` naming its substitutes, with
specification vectors carried in the feature and the inverse law as the metamorphic property. The
contract fails a `@mode-differential` scenario that has neither an oracle nor a second
implementation, so these cases must be honestly typed.

| Artifact | Why |
|---|---|
| 💬️bcf 2.1 | no standalone crate; only bundled inside larger IFC toolkits |
| 🎞️pptx | `pptx` 0.1.0 is a first release, not a credible reference |
| ☁️ply | `ply` 0.1.0 dates from 2017; `ply-rs` 0.1.2 is barely maintained |
| 📼️avi | `avirus` handles frame metadata only, obscure and unvetted |
| 🖊️dwg | `acadrust` is MPL-2.0 and unproven for AC1018; the format is proprietary |
| 🏗️ifc 2x3 / 4 | `ifc-lite-core` is MPL-2.0; worth a deliberate licence decision before adopting |
| 📄txt, 💾️binary | nothing meaningful for a third party to be authoritative about |

A crate that is a first release, unmaintained for years, or obscure is **not** made acceptable by the
fact that it is the only candidate. Registering it would put a weak implementation in the position
of deciding whether ours is correct. Group C says so out loud instead.

## Licence flag for the user

`acadrust`, `ifc-lite-core` and `symphonia` are **MPL-2.0**, not MIT/Apache like every currently
registered oracle. They are test-only and never linked into a production target, but adopting them
is a licence decision, not a technical one — flagged rather than taken unilaterally.

## Sequencing

1. Coordinator adds every Group A + Group B dependency to the oracle crate's `Cargo.toml` in ONE
   edit (it is a shared file and a lease), and creates the per-subset `🧪️oracle/` module stubs plus
   the `📦️lib.rs` module-tree entries, exactly as in wave 7.
2. Group A agents launch first — same brief, same shape, one subset each.
3. Group B agents launch alongside, with the reader-only constraint stated in their prompt so they
   type their scenarios correctly.
4. Group C agents last, since a recorded no-oracle decision needs the vectors written by hand.
5. 🧊️gltf (120 descriptor-table leaves, 7 compiled) and the 🧿️semio Pattern-B subsets (no `apply_*`
   entry point) remain as agreed: included, but as their own wave once the artifact families above
   are green, because both need production wiring rather than test authorship.
