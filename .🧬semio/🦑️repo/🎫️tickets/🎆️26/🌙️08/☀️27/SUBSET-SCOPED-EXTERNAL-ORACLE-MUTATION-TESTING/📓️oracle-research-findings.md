# 📓️ Can a third-party library predict these mutations? — research findings

Four owners investigated in depth (`en1998`, `block-5d`, `block-3d`, `block-2d`), each by reading the
artifact's schema, its mutation leaves, its io module and its current registration — and by checking
every candidate library actually exists on npm / PyPI / crates.io.

**All four verdicts: NO QUALIFYING ORACLE POSSIBLE TODAY.** Two independent root causes, both
actionable, and neither of them a labelling problem.

## Root cause 1 — 45% of serializers emit the wrong bytes

A carrier oracle works by having a third-party reader of a STANDARD FORMAT verify what a mutation
produced: `brepjs` reads the STEP a Boolean should have written. That is impossible when the exporter
writes the artifact's own internal DSL text under the standard format's extension.

**80 serializers do exactly that** — `serialize_bytes` is `print_dsl(snapshot).into_bytes()` for a
format the DSL is not. Repo-wide, `stubSerializerBreaches` counts them:

| Format | Stub serializers | Format | Stub serializers |
| --- | --- | --- | --- |
| `png` | 12 | `csv` | 6 |
| `obj` | 7 | `md` | 6 (**and zero real ones**) |
| `stl` | 7 | `svg` | 5 |
| `dwg` | 7 | `zip` | 5 |
| `pdf` | 6 | `dxf` | 3 (**and zero real ones**) |

53 of the 80 are BINARY formats, where DSL text is unambiguously not that format. Verified independently
by two agents on the `block` plugin: `stdio.obj`, `stdio.stl`, `stdio.png` and `stdio.zip` all emit
`print_dsl(...)`, so `tobj 4.0.5`, `stl_io 0.11.0`, `png 0.18.1` and `zip 9.0.0` — all confirmed real and
suitably licensed — would fail to parse the output outright.

**The artifact declares an export capability it does not have.** That is a correctness bug in its own
right, and it is the reason a whole class of artifacts cannot have a carrier oracle. It is now a gate:
`stub-serializer`, high priority for binary formats.

## Root cause 2 — some domains genuinely have no third-party implementation

`en1998` (Eurocode 8, seismic design) was checked against every plausible source rather than assumed:

```
npm    eurocode8, eurocode, seismic-design, response-spectrum,
       structural-dynamics, openseespy, en1998      → all 404
PyPI   structuralcodes 0.7.1     Eurocode 2 concrete, not EC8
       concreteproperties 0.8.0  section analysis, not EC8
       anastruct 1.7.0           2D frame FEA, not EC8
       openquake.engine 3.26.2   hazard/GMPE modelling, not EC8 code checks
       osmg 1.0.13               OpenSees wrapper, not EC8 formulas
       pyeurocode, eurocode8, pyresponse-spectrum   → 404
```

Nothing published implements EN 1998's elastic/design response-spectrum formulas. And it would not help
if it did: **all 49 of `en1998`'s mutations are plain scalar field-sets** with no computed output for a
domain library to re-derive. Its io module declares zero import and zero export bridges, so there is no
carrier either.

The same holds for `block-2d`/`block-3d`/`block-5d`: `GripKind`, `VortexKind`, "kind-level compatibility
rule", "rim angle in radians" are a bespoke connector-kit vocabulary no published standard models, and
every one of their 26/37/41 mutations is a literal field write with nothing to compute.

## What this reframes

The goal — *every mutation of every artifact predicted by a third-party library* — is achievable exactly
where the artifact speaks a STANDARD FORMAT, and that is where this session's coverage went:

| Owner | Mutations manifested | Third-party oracle |
| --- | --- | --- |
| `gltf@2.0/any` | 120 | glTF readers |
| `png@1.2/any` | 15 | `png` crate |
| `jpg@jfif-1.01/any` | 10 | `image` crate |
| `tiff@6.0/any` | 6 | `image` crate |
| `pdf@1.4/any` | 5 | `lopdf` |
| `bmp@v3/any` | 5 | `image` crate |
| `step@ap214/cc6` | 5 | `brepjs` + `ruststep` + `manifold-3d` |

For a Semio-native vocabulary the honest order of work is the reverse of what it looks like: **implement
the real carrier export first**, then the third-party reader of that carrier becomes the oracle. Naming
a library before the bytes exist would register a requirement nothing can discharge — which is what
`missing-external-oracle` reports, and why these four owners are correctly left un-oracled rather than
given a plausible-sounding registration.

## The 57 cross-semio registrations are honest, and their rationales already say so

Every one investigated already carries, in its own `rationale`, the sentence that it is supplemental
under Protocol v2 and that a qualifying third-party reference is still owed — several also record that
a third-party library was *declined after being considered*, naming what it lacks. That is the right
record. What was missing was the machine-readable `kind` making it un-ignorable, which this session
added, and the per-owner worklist, which `test gap` now produces.

## Confirmed by source, not by directory listing — a real partial carrier

Four more owners (`present`, `semio-v1-object`, `sequence`, `jack`) were read the same way: not the
directory tree, but every `serialize_bytes` body. The directory tree lies in both directions.

`jack` declares csv/svg/png/md export. Its **svg** serializer returns `print_dsl(snapshot).into_bytes()`
— DSL text with no `<svg>` element at all. Its **csv** writes exactly two rows: a literal `payload`
header and the entire DSL text as one field. `present` declares pptx/pdf/svg/png; its pptx hop coerces
`{schema, source, tiles}` into a `PptxSnapshot` whose fields are all `#[serde(default)]`, so `source`
and `tiles` — the only fields its 9 mutations touch — are silently dropped. Its svg/png serializers say
"degenerate placeholder" in their own doc-comments. `semio-v1-object` is the honest extreme: no
`📥️import`/`📤️export` directories exist at all, and its own doc-comment says so.

But `sequence` is different, and this is the finding worth acting on: its **CSV export is real** — one
RFC4180 row per step with `[id, kind, JSON-encoded params]`, written by actual row logic, not a DSL dump.
That makes a genuine third-party CSV reader (`csv` 1.4.0 crates.io, Unlicense/MIT, 232M downloads;
`csv-parse` 7.0.2 npm, MIT) a **qualifying carrier oracle for 4 of its 8 mutation kinds** —
`create-step`, `delete-step`, `duplicate-step`, `edit-step-params` all change rows a foreign parser can
see. The other 4 (`move-step`, `change-step-collapsed`, `connect-steps`, `disconnect-steps`) touch x/y,
UI state and edges, none of which the carrier encodes — a flat grid has no edge concept.

That is the shape of the whole problem in miniature: **the carrier decides what is oracle-able.** Not
the domain, not the mutation. Where a real standard-format export exists, a third-party reader
discharges the requirement for exactly the mutations that format can represent, and no others.

## The consolidated verdict across eight owners investigated in depth

| Owner | Real carrier? | Verdict |
| --- | --- | --- |
| `sequence-1` | CSV, real | **partial oracle available** — 4/8 kinds |
| `jack-1` | csv + svg both fake | none |
| `present-1` | pptx/pdf/svg/png all fake or lossy-to-zero | none |
| `semio-v1-object` | no io at all | none |
| `en1998` | no io at all; also no library exists | none |
| `block-2d` / `block-3d` / `block-5d` | obj/stl/png/zip all `print_dsl` | none |

Seven of eight are blocked by the carrier, not by the absence of a library. In five of those the
library exists, is registered-quality, and would work the day the export writes real bytes:
`tobj 4.0.5`, `stl_io 0.11.0`, `png 0.18.1`, `zip 9.0.0`, `python-pptx 1.0.2`.

**So the ordering is fixed, and it is not the one the goal's phrasing implies.** "Every mutation tested
by an external library" is not primarily an oracle-research task. It is an export-correctness task:
80 serializers must write their format before any third-party reader of that format can judge them.
Registering a library against a stub export would create a green test that proves nothing — the reader
would parse DSL text and fail, or worse, a lenient reader would accept it. That is why these owners are
left un-oracled with a machine-readable reason rather than given a plausible registration.
