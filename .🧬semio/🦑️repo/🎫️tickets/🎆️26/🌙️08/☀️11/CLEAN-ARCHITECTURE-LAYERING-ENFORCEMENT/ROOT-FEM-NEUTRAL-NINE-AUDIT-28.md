# Root FEM Neutral Nine Audit 28

Source-only audit; no runtime acceptance, live native output read, source edit, Git mutation or lifecycle claim. Full relevant current text and identities plus complete before/proposed/current corpus journal are retained in `🗑️generated/root-fem-neutral-nine-audit-28/full-source-receipt.json`. Bounded read: 609769 bytes, 8 work units, 0.020 seconds under64MiB/65536/60sec.

| Source | Bytes | SHA-256 |
| --- | ---: | --- |
| 🕸️mesh/🦀️.rs | 132108 | `14caf48bc56f5a0c45cf67d4c8b75c7221668f6a7f58b9ca36dbc639a492b42d` |
| 🧬️schema/🦀️.rs | 1256 | `b8fb452de3617a5013775f760d5f3f05834c2156f14bdfa729412ec57780d063` |
| 🧬️schema/🔣️.json | 4487 | `992a5f413d65eae2c21c1f465454ea9f8ea5c329b2e4f3b72d050898fbbbf9f5` |
| 🧫️fixtures/🔣️.json | 12142 | `033599e7ba3c32caaf14042f47d536ec8b5328d7101ab306b1e40b8b572917e3` |
| 🧪️tests/🦀️.rs | 3997 | `d2501172750571d9690267c01c8fde29052d4e112789eee872ede90f82f5b2c8` |
| 🧪️tests/🟦️.ts | 10536 | `f9ebf0d592e35e769c1f27affe68d516a7a529125ee37f6d4b2db3334518f193` |
| 🔬️neutral/🟦️.ts | 4142 | `49f59f536a43db91368b26476edf6c0a2948953bd80f5ae52b1c9fc6bc8936a1` |
| fem2d-surface-inputs/neutral-nine-corpus-joins.jsonl | 441101 | `b8cb7d8146839b5ccb972db8248c91f8d1ca3fe92df93944c09be587ea7be463` |

The Rust complete_portable_region_surface_corpus constructs all admitted region geometry through actual triangulate → extrude_tri_mesh → split_to_tets → boundary_faces algorithms. It iterates exactly nine fixture vectors: rectangle, hole, reversed-winding, multiple-regions, bars-beams-empty, failed-then-valid, zero-thickness, negative-thickness, binary64-sensitive. Empty vector intentionally constructs no region; failed input intentionally skips on algorithm refusal, with expected admitted region identity order checked afterwards. Bounds, finite coordinates, original outer points, minimum triangles and valid volume/surface indices are asserted. Serialization uses actual algorithm results, with no referenceSurface fallback or synthetic native output. Deadline/input/output ceilings constrain the test; they do not prove allocation grants.

The neutral API and schema import only MeshOpts/PlanarDomain from their neutral parent; no Specific Semio snapshot or provider import appears in these defining files. The full mesh source retains original algorithms. Bun neutral receiver reads the declared actual result path, checks all nine identities and schemas, indexes actual geometry, and verifies closed edge incidence, Euler characteristic, oriented surface volume, tetrahedron volume, and independent Earcut area/extrusion volume. No supplied native output was inspected and no passing result is claimed.

Concrete oracle qualification: both Bun receivers instantiate Ajv with strict:false. They do not support a strictAJV claim. Catalogue was informed. First-party schema validation plus independent Ajv is present, and the original broad receiver retains its refusal tests and installed OBJ/STL receiving assertions, but the neutral-only receiver does not execute the broad installed foreign output route. Foreign reference output is explicitly derived from referenceSurface for independent Three parser checks; it is not substituted for actual native results.

Numeric policy declares binary64 native/JavaScript Earcut and Float32-only Three OBJ/STL projection. The neutral receiver uses ordinary determinant summation with absolute1e-10 volume and1e-12 area tolerances, not compensated summation. This is bounded corpus evidence; large-coordinate cancellation or arbitrary topology accuracy is not established. Float32 projection is checked in the original broad receiver, not the neutral Rust output path.

MeshJob physical grant chain remains outside this synchronous corpus. Preparation and mounted initialization contain try_reserve_exact allocations (insertion order, triangulation points/faces, edge authorities); InteractiveJob preparation stages use own fixed capacities and StepContext rather than a caller-supplied independent retained-capacity grant/receipt at each reserve. Do not certify production preparation grant admission/cancellation from this nine-case geometry runner. Catalogue explicitly confirms no Surface operation controller runtime exists; preserve that limitation and all original assertions until actual admission/retirement receivers are implemented and exercised.
