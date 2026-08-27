# 📓️ brepjs oracle qualification

**Spike:** `🔬️brepjs-spike/📜️script.ts` (re-runnable; writes `📤️out/`).
**Package:** `brepjs@18.119.8`, Apache-2.0, already resolved in `node_modules`.
**Kernel:** `brepjs-opencascade@0.15.6`, LGPL-2.1-only — OpenCASCADE 8.0 compiled to WASM.
**Engine family:** `opencascade`. Independence is accounted at this level: any other OCCT wrapper is
the SAME family and does not count as a second oracle.

**Result: 11 of 12 criteria met.**

| Criterion | Met | Evidence |
| --- | --- | --- |
| kernel-init | ✔ | `init()` resolves offline from the vendored WASM |
| kernel-capabilities | ✔ | `{exact:true, brepExport:true, exactMeasurement:true, tessellationModel:"extract-time"}` |
| boolean-cut | ✔ | `cut(box(20,20,20), translate(cylinder(5,40),[10,10,-10]))` → compound holding exactly 1 solid |
| measure-volume | ✔ | `6429.203673205102` vs the analytic `20³ − π·5²·20 = 6429.203673205104`; **relative error 2.83e-16** |
| measure-area | ✔ | `2871.238898038469` |
| valid-solid | ✔ | `isValidSolid` true |
| export-step | ✔ | 19 068 bytes, `ISO-10303-21;`, `AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF` |
| **step-self-determinism** | **✘** | see below |
| tessellate | ✔ | 918 vertices / 908 triangles at `tolerance 1e-3`, `angularTolerance 0.1` |
| step-round-trip | ✔ | reimport → volume agrees to **8.49e-15** relative |
| bounding-box | ✔ | `getBounds` returns the expected box, fuzzed by the kernel's own 1e-7 tolerance |
| topology-counts | ✔ | 1 solid, 7 faces, 15 edges — the exact bored-box answer |

## The one failure, and why it matters

Two `exportSTEP` calls **on the same shape in the same process** are NOT byte-identical:

```
#7 = PRODUCT('Open CASCADE STEP translator 8.0 1',   ← first  export
#7 = PRODUCT('Open CASCADE STEP translator 8.0 2',   ← second export
```

OCCT stamps a **monotonically incrementing translator counter** into `PRODUCT`, and `FILE_NAME`
additionally carries a wall-clock timestamp (`'2026-08-27T16:28:46'`), a producer string and an
originating-system string. None of these is semantic.

This is measured confirmation of the plan's §5.3 position, and it is stronger than the plan assumed:
it is not only *cross-writer* byte equality that is unreliable — a single writer is not even
self-deterministic at the byte level. So:

1. **Raw STEP byte equality may never be a gate**, in either the self-determinism (A) or the
   cross-writer (B) form, unless the compared bytes have first passed through a canonicalizer that
   normalises `FILE_NAME`, `FILE_DESCRIPTION` and the `PRODUCT` name strings.
2. **No external canonicalizer is qualified today.** STEPcode is the candidate named in the plan; it
   is not present in this environment and its AP242 coverage and deterministic entity ordering have
   not been demonstrated here. Claiming "canonical STEP byte equality" now would be claiming a
   guarantee nothing measures.
3. Therefore the BRep pipeline's canonical-bytes stage is registered `optional: true` behind a probe
   whose `qualification.status` is `provisional`, and the **operative gate is semantic STEP
   equivalence**: external reimport succeeds, BRep validity holds, topology invariants hold, and the
   CGAL-measured geometry agrees within the resolved tolerance. `evaluatePipeline` treats an
   `optional` stage as reportable-but-not-gating, and `isQualifiedProbe` is what any release gate
   must consult before claiming a probe's strongest guarantee.

## Registration consequences

| Field | Value | Why |
| --- | --- | --- |
| `kind` | `third-party-library` | Qualifying: it discharges an external-oracle requirement |
| `engine.family` | `opencascade` | Two OCCT wrappers are ONE family for independence accounting |
| `productionReachable` | `false` | Must stay test-only |
| `networkDuringExecution` | `false` | The WASM kernel is vendored; the spike ran offline |

**Standing risk (plan §13):** if Semio's own BRep subject ever adopts OpenCASCADE, `brepjs` stops
being a strongly independent exact-kernel oracle and becomes an interoperability/regression fixture
source. `engineIndependenceBreaches` in the v2 library reports exactly that case, keyed on the
subject's declared engine family, so the day it happens is a reported finding rather than a silent
weakening.
