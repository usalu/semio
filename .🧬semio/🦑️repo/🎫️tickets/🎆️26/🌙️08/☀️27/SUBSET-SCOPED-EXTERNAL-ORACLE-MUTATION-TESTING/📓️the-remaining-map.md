# 📓️ The whole goal, in one table

Measured from the live registry, over every subset that declares mutations. This is the map I should
have drawn at the start rather than after chasing the build for most of a session.

| what the subset still needs | subsets | kinds | share |
| --- | ---: | ---: | ---: |
| **Oracle AND fixtures** — the achieved state | 13 | **214** | 10% |
| Oracle chosen, **no fixtures** — mechanical | 55 | **523** | 24% |
| **No qualifying oracle at all** | 77 | **1410** | 66% |
| total | 145 | 2147 | |

## What each band actually requires

**The 523.** These subsets already name a qualifying third-party library in their own
`🧪️oracle/🔣️.json` — `tobj`, `lopdf`, `dxf`, `gif`, `zip`, `riff`, `quick-xml`, `csv`. Nothing is
blocked: what is missing is a generated corpus and a probe. This is the same work the `mesh`, `brep`
and `bcf` pilots completed, and it is where a fleet can make direct progress without touching a file
another session is editing. **This is the largest tractable band and I spent most of the session
elsewhere.**

**The 1410.** These need an oracle before a fixture can mean anything, and the earlier research
established why most of them cannot have one yet: **163 serializers do not write the format they
declare** — 97 emit `print_dsl` text under a standard extension, 33 reinterpret their own pack bytes,
19 coerce through serde into an empty document, 14 never read their input. A third-party reader handed
those bytes parses this repository's own DSL, not the format. So the honest order for this band is
EXPORT CORRECTNESS FIRST, oracle second — it is not oracle research that is missing.

**The 214.** Oracle and fixtures both present. Of these, 30 additionally EXECUTE against production
dispatch (`mesh` 17, `brep` 13) with runtime, manifest and catalog in exact agreement.

## The honest reading

"Every mutation for all artifacts tested by an external library" decomposes into three very different
jobs, and only the middle one is a matter of effort:

1. 523 kinds — generate corpora against already-chosen libraries. **Tractable now.**
2. 1410 kinds — write real exporters, THEN oracle them. A large engineering programme, not a testing one.
3. Executing what is oracled — gated on `semio-s-plugin-stdio` compiling, now ~97 errors from green,
   the remainder in files a peer session is actively rewriting.

## Redrawn after the fixture wave and the audit that followed

Seven subsets gained corpora — `avi` 22, `docx` 25, `pdf@1.7` ×4 = 58, `note` 16, plus `obj`/`dxf`/`gif`
one apiece. **Fixtures 362 → 486, all at 100% provenance and 100% byte-reproducibility.**

Auditing those same subsets is what mattered more. **18 owners were counted as externally oracled while
their oracle was this repository's own second implementation** — `gltf`, `png`, `jpg`, `bmp`, `tiff`,
`avi`, `obj`, `dxf`, `gif`×2, `svg`, `xml`, `bcf`, `docx` and five `pdf` profiles. Each names a real
crate (`tobj`, `lopdf`, `riff`, `gif`, `zip`) that genuinely parses and writes the format — and each
carries a `🧪️oracle/🦀️component.rs`, 139 to 1069 lines, that computes what the mutation should produce.
Several say so themselves: *"a fresh, independent implementation"*, *"read out of the BYTES by the
independent implementation"*, *"deliberately mirrors"* the production diff.

| | before the audit | after |
| --- | ---: | ---: |
| External-oracle coverage | 64.3% | **31.6%** (208/658) |
| Oracle AND evidence | 37.8% | **20.2%** (133/658) |

The lower figures are the true ones. The crate discharges *"the result is a well-formed file of this
format"*; it does not discharge *"the mutation computed the right answer"*, and only the second is what a
mutation oracle is for.

`note` is the counter-example worth keeping in view: its three oracles are genuine third-party READERS
(`dxf`, `quick-xml`, `lopdf`), its own oracle file is 139 lines of projection helpers with no predict
path, and the gate does not flag it. That is the shape the other seventeen need.

## The retrofit pattern, and why it recovers coverage honestly

Reclassifying the 18 predicting oracles left three subsets — `avi`, `bcf`, `docx` — with a complete
reader-based judge and nothing registered to name it. Registering that judge took external-oracle
coverage 31.6% → 37.7% and evidence 20.2% → 26.3%, and the recovery is legitimate because the two
mechanisms are genuinely different:

| | predicting oracle | reader oracle |
| --- | --- | --- |
| where the expected state comes from | COMPUTED at test time by our own `🧪️oracle/🦀️component.rs` | COMMITTED as the `after` half of a byte-reproducible fixture |
| what the third-party crate does | acts as a codec for our own answer | parses BOTH sides and is the judge |
| what a shared misreading of the spec produces | two agreeing wrong answers | a visible disagreement |
| qualifies? | no — `cross-semio-implementation` | **yes** |

The probes carry the rule in their own headers: *"Everything here MARSHALS and READS; nothing here
applies a mutation or predicts what one should."* That sentence is the whole difference.

**This is the retrofit the other 15 reclassified subsets need**, and it is ordinary work rather than
research: commit after-state fixtures, write reader-only probes, register the reader. `gltf` alone is
120 kinds. It does not require the stub serializers to be fixed first, because these subsets already
write their own real format — it is only their ORACLE that was pointing inward.
