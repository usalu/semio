# 🧊️ The brep oracle EXECUTED — 72/72 fixtures reproduced byte-for-byte by brepjs

Everything about brep in this ticket had been established by READING the catalog. That is a weaker
claim than it sounds, so the generator was actually run and its output compared against what is
committed.

## What was run

```
bun ✳️brep/🏭️generator/📜️script.ts generate --out <scratch>
```

The console filled with OpenCASCADE's own STEP writer banners
(`Transferring Shape, ShapeType = 2`, `Transfer Mode = 0 I.E. As Is`) — brepjs/OCCT doing the work, not
a description of it doing the work.

```
[generator] reproducibility: 72/72 fixture(s) byte-identical across two generation passes
[generator] 72/72 bundle(s) generated
```

## Compared against the repository

Every regenerated file hashed against its committed counterpart:

| | |
|---|---|
| files compared | **292** |
| byte-identical | **292** |
| differing | **0** |
| missing from the repo | **0** |
| STEP files regenerated | **155** |

So the committed expectations for all 13 brep mutation kinds — `expected.step`, `expected.mesh.json`,
`expected.metrics.json`, `operand-a.step`, `operand-b.step` — are exactly what brepjs produces today,
and produces twice in a row. The corpus is not a snapshot someone took once; it is reproducible from
the third-party kernel on demand.

## Why this is the load-bearing check

`fixtureProvenanceCoverage` and the `reproducible: true` flag are DECLARATIONS in a manifest. This is
the thing they declare, performed. A corpus that cannot be regenerated is a corpus nobody can audit,
and OCCT's STEP export is famously not byte-deterministic across configurations — which is exactly why
this was worth executing rather than assuming.

## The mesh half, confirmed by reading the implementation

`✳️mesh/🔬️probes/📜️script.ts` computes the goal's Hausdorff requirement through **`three-mesh-bvh`**
closest-point queries (`symmetricHausdorff`, `normalizedSymmetricHausdorff` against the bounding-box
diagonal), with `three` parsing the carrier and `manifold-3d` measuring the solid. Its own header states
the rule this ticket exists to enforce:

> Everything here MARSHALS and INVOKES; nothing here computes geometry. […] The pipeline compares the
> emitted `measurements` against declared assertions and performs no arithmetic of its own, which is
> what keeps the reference external.

That is the "never reimplement the oracle" constraint written into the probe itself, and the brep
pipeline's `manifold-mesh-compare` stage consumes it with
`hausdorffInTessellationTolerancesMax: 3`.
