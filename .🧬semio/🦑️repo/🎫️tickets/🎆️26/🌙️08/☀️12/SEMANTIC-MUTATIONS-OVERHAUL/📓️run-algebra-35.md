# Run Algebra and Remaining Inverse Boundary

This packet examines the actual Run diff composition and replacement inverse, separately from the current strict-schema and checked-admission repairs. The owning sources are the Workflow component and its five direct Run mutation leaves. No production mutation source is changed by this packet.

Source inspection found that `RunDiff::absorb` replaces the previous nonempty diff with the incoming diff, while the protocol requires sequential composition. It also found that finishing a previously recorded node removes that node and appends its replacement; the existing inverse chooses the prior record but does not restore its original position. The existing replacement-inverse test checks the returned mutation value, not the complete restored snapshot. These are hypotheses until the genuine compiled client below executes.

The neutral vectors in `🧪️run-algebra-35/🔣️vectors.json` specify two-operation log preservation, start-plus-log field preservation, and replacement-inverse order restoration. The independent TypeScript model and Ajv schema oracle are separate from the Rust client. The Rust client calls the real public mutation/diff/inverse APIs and consumes inverse plans in the repository's reverse-order convention. It does not copy the Run mutation implementation.

The prior seven-check review also proved that first insertion cannot satisfy the current `explicit-mutation` descriptor. Run history must not acquire a fabricated removal operation just to make an inverse test green. A correct resolution requires an explicit domain decision and a typed non-invertibility result through the actual generic undo contract, or a genuinely valid inverse leaf. The present `Vec<Op>` contract cannot distinguish a successful empty inverse from a non-invertibility reason. No replacement-only restriction or hidden compatibility inverse is approved here.

The Run owner remains incomplete while application branches live centrally, composition laws fail, or inverse metadata exceeds actual behavior. Application-time clock generation is another source observation requiring deterministic replay review. Results below will distinguish prepared tests, independent model/oracle runs and the exact compiled artifact checkpoint.

## Neutral Oracle Execution

The actual command `bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️run-algebra-35/📜️script.ts` exited0. Ajv accepted the three-case language-neutral contract and the independent TypeScript reference matched all three expected results. The public Rust client is prepared but has not executed at this checkpoint; these three oracle passes are not Run implementation acceptance.

## Actual Public Rust Execution

After framework build35, root compiled and ran the unchanged three-case public Rust probe through the source/artifact fingerprinting controller. The retained run is `🧪️workflow-actual-source-34/🧫️run-1v2Tai`: compiler exit0, runtime exit101, unchanged full input hashes. All three implementation checks failed against the accepted neutral expectations:

- Two appended log diffs retained only `second`, losing `first`.
- A composed Start plus Append retained the log but lost workflow selection and left status `pending`.
- Finishing and then inversely replacing middle node `b` restored values in order `[a,c,b]`, not original `[a,b,c]`.

These are now real runtime failures, not inferred source defects. The inverse-order probe used the repository's reverse replay convention. This is separate from the already reproduced first-insertion non-invertibility issue; the current API needs both a lawful composition repair and an honest inverse-availability contract.

## Corrected Actual Public Checkpoint 36

After the source-owned sequential composition/in-place replacement repair and fresh framework build36, the unchanged independent client passed all3 vectors in `🧪️workflow-actual-source-34/🧫️run-ILbi4I`. Compiler and runtime exited0 and complete source/artifact/vector hashes remained stable. Logs retain both appended messages, Start selection/status survive composition, and reversed replacement inverse restores `[a,b,c]` with complete snapshot equality.

The mounted actual-source suite separately ran54 tests successfully, including five new permanent algebra/rejection laws. The seven-check contract probe still fails first-insertion total inverse (6pass/1fail); the bounded algebra fix does not close that generic API defect or the application-time clock and central ownership gaps.
