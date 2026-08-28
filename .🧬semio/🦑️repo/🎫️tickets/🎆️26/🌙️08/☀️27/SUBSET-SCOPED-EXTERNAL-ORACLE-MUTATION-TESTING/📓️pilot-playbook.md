# 📓️ Playbook — putting one subset under an external oracle

Distilled from the `mesh` (17/17) and `brep` (13/13) pilots. Every numbered lesson below cost a real bug.

## The five artefacts

```
<subset>/🧪️oracle/🔣️.json      oracles + probes + comparisonPipelines + mutationManifests + fixtureManifests
<subset>/🔬️probes/📜️script.ts   marshals to a third-party lib, emits ProbeReport JSON; computes NOTHING itself
<subset>/🏭️generator/📜️script.ts + 🧪️<family>/📜️script.ts   fixtures built BY a third-party library
<subset>/🧫️fixtures/<recipe>/  the bundles
```

## Step 0 — is this subset even reachable?

Read every `🚪️io/📤️export/🧵️serializers/🗿️artifacts/<fmt>/**/🦀️component.rs` BODY. The directory tree lies
in both directions. A carrier is REAL only if it writes the format; three stub shapes disqualify it:

1. `serialize_bytes` returns `print_dsl(..).into_bytes()`.
2. `encode_pack` the source snapshot then `decode_pack` those bytes AS THE TARGET type (envelope
   type-confusion — worse than unimplemented).
3. No `serialize_bytes` at all, only `serialize_text` returning `print_dsl`.

`bun 🧰️framework/…/🧪️test/📜️script.ts contract` reports these as `stub-serializer`. If every carrier is a
stub, STOP — registering an oracle against it produces a green result standing on bytes the reader never
understood. Say so and move on; that is a real finding, not a failure.

**JSON does not count.** A JSON export of our own schema is our own schema in JSON syntax; a validator
checks shape, not whether the mutation computed the right answer.

## Step 1 — which mutations can this carrier actually witness?

Per mutation, ask what the carrier ENCODES. A material-roughness change is invisible in STL by
construction; it is visible in glTF. Only register a mutation against a carrier that can see it. A probe
handed a carrier that cannot encode the property must return `unsupported`, **never an empty `ok`** — an
empty result read as ok lets a mutation pass against evidence that was never there.

Mutations the carrier cannot witness keep an `oracleRequirement` naming a capability no oracle provides,
so they report honestly as un-oracled rather than being absorbed into a capability-level pass.

## Step 2 — outcome classes come from the CODE

Grep each mutation's `🔺️diff/🦀️component.rs` for `MutationOutcome::{new,empty,error,fatal}`:
`new`→`applied`, `empty`→`no-op`, `error`/`fatal`→`rejected`. Do not trust doc comments — four mesh
leaves document "duplicate id is a no-op" while the code returns `fatal`.

## Step 3 — the gate, and the tolerance trap

Decide what tessellation freedom means HERE before choosing a threshold:

* **BRep** — a solid may legitimately be tessellated many ways, so gate in *tessellation tolerances*.
* **Mesh** — the artifact IS the vertex/index buffer and a mutation transforms it deterministically, so
  gate near-exact.

Measured proof that this matters: allowing re-tessellation on the mesh subset gave `relativeVolumeError`
8.43e-2 for a LEGITIMATE difference and 1.07e-1 for a genuinely wrong solid — overlapping, so the gate
proved nothing. Comparing at matched tessellation gave 0.000e+00 vs 1.07e-1 instead.

**Always validate the gate both ways**: it must ACCEPT a known-good pair and REJECT a known-bad one.
A gate only ever tested on good input is not a gate.

All tolerances scale-relative: `max(absoluteFloor, relative × boundingBoxDiagonal)`. A fixed 1e-3
tessellation tolerance made a 1e6-scale model mesh for 12 minutes into 2.4 GB.

## Step 4 — engine families

The reader and the judge must be DIFFERENT implementations, or the measurement confirms the parse
instead of checking it. mesh uses `three` to parse and `manifold-3d` to measure. Two OCCT wrappers are
ONE family. Declare `productionReachable` truthfully — `three` is production here, `manifold-3d` is not.

## Step 5 — fixtures

Class `third-party-generated`, built BY the library, never hand-rolled to match our output. Then:

1. **Measure the EXPORTED artifact, not the in-memory shape.** Export, re-import, then measure. A BRep
   recipe recorded 23 edges/14 vertices while its own STEP file contained 24/16.
2. **Paths are `../🧫️fixtures/<recipe>/<file>`** — resolved against the OWNER'S ORACLE directory. Bare
   paths silently resolve to nothing.
3. **`--only` MERGES** into the manifest index, never overwrites.
4. **Never rewrite a file after hashing it.** Recording reproducibility INTO the metrics file invalidated
   all 72 recorded digests.
5. **Prove reproducibility per-fixture**, via `test fixture reproduce`. Regenerating the whole corpus
   twice cannot see order-dependent state: it passed while 23 of 119 fixtures were still differing on an
   OCCT process-global counter.
6. Report a recipe the library refuses; never drop it to make the corpus look clean.

## Step 6 — verify, don't assert

`bun 🧰️framework/…/🧪️test/📜️script.ts contract` then `matrix` then `fixture reproduce --subset <id>`.
Quote real output. A claim that a test passes without having run it is the one unrecoverable error here.
