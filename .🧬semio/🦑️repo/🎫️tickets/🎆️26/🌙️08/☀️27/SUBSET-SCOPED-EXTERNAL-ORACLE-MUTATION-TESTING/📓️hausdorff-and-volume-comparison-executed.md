# 📐️ The Hausdorff/volume comparator EXECUTED — and shown to discriminate

The mesh comparison had only ever been established by reading the probe. It was run.

```
bun ✳️mesh/🔬️probes/📜️script.ts mesh-compare --input <a> --input <b>
probeVersion: three@0.182.0 + manifold-3d@3.5.1 + three-mesh-bvh@0.9.14
engine: manifold-3d wasm 3.5.1
```

## Identical geometry through two different carriers → zero

`degenerate-sliver-outline/expected.stl` vs the same fixture's `expected.obj` — the same solid written
by two different exporters, parsed by two different `three` loaders:

```json
"symmetricHausdorff": 0,
"hausdorffExpectedToActual": 0,
"hausdorffActualToExpected": 0,
"hausdorffSamples": 72,
"hausdorffSampling": "mesh vertices of both sides; exact at vertices, a lower bound between them",
"symmetricDifferenceVolume": 0,
"relativeVolumeError": 0,
"relativeAreaError": 0,
"tessellationDiffers": false,
"expected": { "volume": 0.004999999031424507, "area": 1.1019999936521052, "genus": 0, "triangles": 12 },
"actual":   { "volume": 0.004999999031424507, "area": 1.1019999936521052, "genus": 0, "triangles": 12 }
```

status `ok`, 22 ms.

## Different geometry → large, and past every threshold

A comparator that always returns zero proves nothing, so the second half matters more than the first.
`degenerate-sliver-outline` vs `rect-thin-plate`:

```json
"symmetricHausdorff": 7.802570091001569,
"normalizedSymmetricHausdorff": 1.5601988898413195,
"hausdorffExpectedToActual": 0.08000000193715096,
"hausdorffActualToExpected": 7.802570091001569,
"symmetricDifferenceVolume": 1.2029999726712703,
"relativeVolumeError": 239.00004112721362,
"relativeAreaError": 108.47368482815092
```

Against the brep pipeline's own assertions — `hausdorffInTessellationTolerancesMax: 3` and
`normalizedSymmetricDifferenceVolumeMax: 0.01` — that second pair fails both by orders of magnitude,
which is exactly what a working gate must do.

Note the ASYMMETRY in the second run: expected→actual is 0.08 while actual→expected is 7.80. That is
why the probe reports both directions and takes the max; a one-directional distance would have called
these two shapes nearly identical.

## What this settles, and what it does not

It settles that the measurement half of the goal's brep requirement — "a similar mesh… similar hausdorf
distance, volume" — is implemented, executes, produces real numbers from third-party engines
(`three-mesh-bvh` for closest-point queries, `manifold-3d` for volume/area/genus), and discriminates.

It does NOT settle the subject side. Both inputs here are fixture meshes. Feeding OUR kernel's exported
mesh in as one side needs `semio-s-plugin-stdio` to build, which is the 54-aggregate blocker.
