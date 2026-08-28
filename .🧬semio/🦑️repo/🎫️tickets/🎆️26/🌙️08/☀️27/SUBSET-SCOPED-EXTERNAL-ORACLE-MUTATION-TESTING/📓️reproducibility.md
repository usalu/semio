# 📓️ Reproducibility — three bugs, and why "regenerate the corpus twice" was the wrong test

Repository-wide fixture reproducibility went **1.65% → 100% (258/258)**, verified by the platform's own
`test fixture reproduce`, which regenerates each fixture and compares hashes. Getting there needed three
separate fixes, and the order they surfaced in matters.

## 1. The checker itself had never worked

`fixture reproduce` passes `SEMIO_FIXTURE_OUT` and then looked for the produced file at
`join(outDir, basename(file.path))`. Every generator in the repository treats that variable as a fixtures
ROOT and writes `<root>/<recipe>/<file>`. So the checker looked one directory too shallow and reported
`generator produced no <file>` for **every file it had ever been asked to check** — 560 problems on a
corpus that was largely fine. The generators agree with each other; the harness was the odd one out, so
the fix is there: resolve with the manifest's own path, which already carries the recipe segment.

Until this was fixed there was no working way to *check* reproducibility, only ways to assert it.

## 2. OCCT stamps three process-global counters, and one of them hides

OpenCASCADE writes into every STEP export a wall-clock `FILE_NAME` timestamp, a translator counter on
`PRODUCT`, and an occurrence counter on `NEXT_ASSEMBLY_USAGE_OCCURRENCE`. None depends on the shape; all
three depend on how many exports ran earlier **in the same process**.

The first two were found and normalized by regenerating the whole corpus twice and diffing. That test
passed — and it was insufficient, because both passes export in the same order, so the counters agree
and the corpus looks stable. Only regenerating ONE fixture on its own starts the counters somewhere else.
Running the now-working checker against the "100% reproducible" corpus returned **23 of 119 still
differing**, every difference confined to `NEXT_ASSEMBLY_USAGE_OCCURRENCE` and none to geometry:

```
< #376 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('5','','',#5,#31,$);
> #376 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('1','','',#5,#31,$);
```

Same byte length, four differing lines, identical solids. Normalizing the third counter closed it:
**119/119, 0 problems**, and the same canonicalizer applied to the new BRep corpus gave 72/72.

The lesson is about the test, not the counter: *a reproducibility check that regenerates everything in
one batch cannot see order-dependent state.* Per-item regeneration can.

## 3. Recording reproducibility destroyed the digest it was recorded next to

The BRep generator hashed `expected.metrics.json`, then **rewrote that file** to add `reproducible` and
`reproducibilityDiffs`. Every one of the 72 recorded `expected-measurements` digests therefore described
content that no longer existed on disk — 72 mismatches whose cause was the reproducibility feature
itself. It also made the file self-referential: a metrics file whose content states whether that same
file reproduces.

The fix is not to re-hash after the rewrite but to stop the rewrite. The metrics describe the geometry;
whether the bundle regenerates byte-identically is a fact about the bundle and belongs on the manifest.

## Also corrected while measuring

`dependencyIsolationCoverage` reached **100% (157/157)** after two accuracy fixes, both in the direction
of the registry telling the truth rather than of a better number:

* The `manifold-mesh-compare` probe declared `productionReachable: true` for `manifold-3d`. Grepping
  every non-test path shows manifold-3d appears only in `package.json`, oracle registrations, probes and
  generators — it is genuinely test-only, and the claim was simply false. `manifold-3d` was also missing
  from `🔒️dependencies.json` entirely and is now registered `test-oracle`.
* The reverse case: I had registered the `three-carrier-reader` oracle as `productionReachable: false`.
  `three` IS a production-runtime dependency here (cad, puzzle, the os renderer). Corrected to `true`
  with the debt recorded, and stating the consequence — the mesh pipeline's independence rests on
  `manifold-3d`, which is test-only and a different engine family. three parses; manifold judges.
