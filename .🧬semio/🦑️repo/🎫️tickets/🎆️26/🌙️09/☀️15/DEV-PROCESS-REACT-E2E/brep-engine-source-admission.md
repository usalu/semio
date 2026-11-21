# B-Rep Engine Source Admission

The Process3d native compile stopped at source admission with 4,208 active engine lines and a 4,000-line limit. The original configured test closure had 4,626 active lines. A concurrent extraction of topology reach and validation reporting brought the engine to 4,272 configured test lines and 272,546 bytes before the changes below.

## Changes

- Moved projection for analytic curves, trimmed edges, conic faces, analytic surfaces and solid primitives to the B-Rep analysis crate. It uses native points and vectors. The engine continues to resolve live registry handles, convert contract vectors and apply the existing refusals.
- Moved six existing fixture and oracle laws into the existing public engine test crate. They cover funded arc publication, analytic boundaries, selected solid operations, topology deconstruction, trimmed closest points and oversized patterns. Moved tests observe the registry count, arena and topology through existing public interfaces. Tests that mutate private arena state or inspect pending mutations remain with the engine.
- Promoted the existing analysis dependency on B-Rep construction to use the shared solid primitive descriptor. Added the existing job crate as a development dependency to the public test crate for the funded arc test.

## Verification

The repository source closure reader reported the following counts on macOS.

| Crate | Configuration | Files | Active lines | Bytes |
| --- | --- | ---: | ---: | ---: |
| B-Rep engine | Production | 20 | 3,644 | 218,661 |
| B-Rep engine | Test | 26 | 3,875 | 237,583 |
| B-Rep analysis | Production | 12 | 1,871 | 84,396 |
| B-Rep analysis | Test | 14 | 2,289 | 114,594 |
| B-Rep engine tests | Production | 1 | 13 | 623 |
| B-Rep engine tests | Test | 13 | 2,131 | 172,394 |

The engine test closure has 125 active lines, six files and 24,561 bytes of headroom. Scoped Git whitespace checks passed. The new paths are under 256 bytes and all relocated fixture references resolve. Process3d compilation is being verified by the parent task.

## Native registry follow-up

Editor run 4897 reported an unresolved `predicates::ray_event` import during compilation and a 4,058-line native registry closure during the subsequent source-admission diagnostic. In the current checkout the vector crate publicly exposes its `predicates` module and `ray_event` accepts the three points its analysis callers supply. The matching source files were updated nine milliseconds apart at 16:46:58 UTC during the prior build. Their current declarations resolve at the source level. This is consistent with a build having started from earlier vector inputs.

The production source-closure reader independently reproduced 4,058 active lines in the current native registry. The pure polygon and helix wire recipe now lives in the existing solid-topology crate. It keeps the original sampling, tolerance and admission computations. The registry retains the original recipe reexports and owns label reservations, staged topology and publication. The peer polygon and conic face publication changes were preserved.

The source-closure reader and scoped diff check passed. Native registry now has 24 files, 3,964 active lines and 218,766 bytes. Solid topology has five files, 940 active lines and 47,957 bytes. The already running editor Cargo check owns compilation verification.
