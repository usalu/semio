# Retained Exact STEP Cylinder Export

The retained `brep.io.exportStep` writer preserves one native full cylinder as STEP Part 21 analytic geometry. It reads the original outward cylindrical wall, two planar caps, full circular rims and one shared axial edge from the certified live Body. It verifies that wall and cap loops use the exact original topology and parameter directions. The source retains its native world frame, signed axial direction, cap placement, radius and height.

The writer emits one `CYLINDRICAL_SURFACE`, two native `CIRCLE` supporting curves and one axial `LINE`. The wall's shared axial edge uses one explicit `SEAM_CURVE` carrying the two original `PCURVE` branches at zero and one complete angular turn. Both rim `SURFACE_CURVE` records carry their original wall and disk parameter curves. Native circular adjacency and orientations are retained. Numeric records remain round-trippable at 17 significant digits.

The same source-revision guard, two-pass sizing, per-record depth quote, fixed scratch limit, 64-byte output chunks and paid retirement apply. The cylinder admission proof performs a fixed read of three native faces and six coedges before any document allocation. No Engine, Registry or Session adapter or new runtime dependency was needed. The planar writer domain remains available. The retained import accepts a narrower axis-aligned subset and can roundtrip the reversed cardinal placement. The emitted oblique cylinder was checked independently in Open CASCADE. General curved shells and circular holes in an otherwise planar solid remain outside this writer branch.

## Verification

- The language-neutral export fixture covers a translated cylinder on a negative axis and a separate translated cylinder on an oblique axis.
- The native Rust witness checked both authored seam branches, three native edge records, the two circular support wrappers, six parameter curves, one-event grants, partial document retirement and source identity guards. The negative-axis STEP file reimported into an exact native cylinder with its signed axis, radial frame, height, radius and world bounds preserved.
- A digest-pinned independent Open CASCADE WASM reader reopened both emitted production files. It matched three BRep faces, extents, surface area and volume within the reader's tessellation tolerance. The existing box, prism, orthogonal and incoming authored-cylinder examples also passed. Temporary binaries and STEP files were cleaned from ticket scratch.
- Rust parse checks and targeted diff whitespace checks passed. The registered `original_step_exchange_preserves_authored_full_cylinders` test exercises the original import/export jobs and exact native measurements. Linked registration verification awaits the official Flow runner.
