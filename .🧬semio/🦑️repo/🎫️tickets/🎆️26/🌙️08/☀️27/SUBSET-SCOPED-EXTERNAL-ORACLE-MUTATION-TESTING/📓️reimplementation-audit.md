# 📓️ The correction that matters most: a reimplementation wearing a library's name

**External-oracle coverage was 238/286 (83.2%). It is 82/286 (28.7%).** The difference is not new work
undone — it is a claim that was never true, in work that predates this ticket and that I had been
counting without checking.

## What I found

Auditing my own headline number, I opened `gltf`, the single largest manifested block at 120 mutations.
Its registered oracle is the Rust `json` crate, `kind: third-party-library`. Its own rationale says why
that does not hold:

> `json` is domain-BLIND — it knows nothing about glTF's own schema — so every one of the registered
> kinds' actual semantics **is reimplemented from scratch** in `../🧪️oracle/🦀️component.rs`

The third-party component is a JSON tokenizer. What a glTF mutation *should produce* is computed by 601
lines of this repository's own Rust. Both halves of the comparison then read the same specification, so
a misreading of it produces two agreeing wrong answers — the one failure a differential test exists to
prevent, and precisely what the goal means by *never reimplement the same code*.

**It also overclaimed its scope by a factor of seventeen.** Its dispatch implements SEVEN kinds and
returns `mutation kind {other} has no oracle implementation` for the rest. The manifest named it against
all 120.

## It was not one owner

| owner | mutations | evidence |
| --- | ---: | --- |
| `gltf@2.0` | 120 | "reimplemented from scratch"; 7 kinds implemented, 113 refused |
| `png@1.2` | 15 | doc comment: "deliberately mirrors" the production `diff` |
| `jpg@jfif-1.01` | 10 | same `apply_kind` + "has no oracle implementation" shape |
| `bmp@v3` | 5 | same |
| `tiff@6.0` | 6 | self-described independent implementation |
| **total** | **156** | |

`pdf@1.4` (lopdf) and `step@ap214/cc6` (ruststep + brepjs) were checked and are CLEAN — they marshal to
the library rather than predicting output. So are the four pilots built in this ticket (`mesh`, `brep`,
`fem2d`, `fem3d`, `sequence`), which carry zero lines of own oracle Rust: they marshal to `three`,
`manifold-3d`, `brepjs` and `csv` through probes.

## The honest split these registrations blurred

A crate that decodes the artifact discharges **"the result is a well-formed file of this format."**
It does not discharge **"the mutation computed the right answer."** Only the second is what a mutation
oracle is for. All five are now `cross-semio-implementation` — a required supplement, never a
substitute — and their mutations report as owed a qualifying third-party reference.

## The backlog this exposed

A new gate, `reimplementation-registered-as-third-party`, finds **38 owners** repo-wide with this shape
— `html`, `zip`, `gif`, `pptx`, `svg`, `ifc`, `bcf`, `pdf`, `step`, `xlsx`, `docx`, `xml`, `avi`, `dxf`,
`obj` among them. Most are not manifested yet, so they were not inflating any number; they would have
inflated it the moment someone registered them. The harness pins that the gate detects the shape and
that the five corrected owners stay corrected — it deliberately does NOT assert the repo is clean,
because it is not, and a green check over 38 real breaches would be the same kind of lie.

## What would actually discharge glTF's 120

`three` 0.182.0's `GLTFLoader` — already vendored, already registered and qualified for the sibling
`semio@v1/mesh` subset, where it reads real glTF geometry and PBR material state. Pointing it at this
subset is the work. Naming the `json` crate was not.
