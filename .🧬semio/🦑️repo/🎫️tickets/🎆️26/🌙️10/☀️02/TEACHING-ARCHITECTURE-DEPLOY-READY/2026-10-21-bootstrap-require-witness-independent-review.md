# Bootstrap Require Witness Independent Review

## Actual Cause

Actual80593 exited1/signalnull without cancellation at18:50:40Z; frozen467304/6f3be80b… log has no selected test records. Complete stack is capturedCreateRequire eval11:236→bootstrap witness115:23→bootstrap151:17. All12 focused Source pins were reported unchanged by High.

Actual bootstrap Source73 creates require anchored at join(root,package.json), then uses only require.resolve for each parser package manifest and entry; it never executes the parser through that require. The package manifest is DATA, not the declaring implementation module. Actual capturedCreateRequire checks images.has(path.resolve(base)) before constructing the underlying require. Thus the uncaptured DATA anchor fails the declaration guard before parser resolution. This is not a virtual semio-source URL conversion error.

Changing only the anchor to import.meta.url would not repair the next contract: captured resolve accepts builtins or the declaring module explicit immutable import map, not arbitrary installed parser package resolution. Broadening the loader guard or falling back to live require would weaken implementation authority.

## Relevant Existing Interface

The existing parser receipt interface exposes buildParserReceiptMatchesV1 and readBuildParserInspectionV1, not an installed-package resolver. The witness already owns a bound dependency store/generation target and verifies binding receipt, native store confinement, exact locked versions and complete bounded package digest. Installed parser path/entry observation belongs to this material DATA inspection context, distinct from implementation-module execution authority. A finite schema-owned parser material projection can derive known package directory/entry under that binding and retain physical validation without executing external Source. High owns the repair.

Initial bounded nofollow identity pins are in `🗑️generated/2026-10-21-bootstrap-require-witness-initial-pins.json`. No Source evaluation, CLI/Native run, boundary fallback or retry was performed. Candidate review pending schema/corpus and genuine capability witness.

## Installed Manifest DATA Observation 2026-10-21T19:00:11.491Z

Bounded current package manifests show TypeScript5.9.3 main `./lib/typescript.js` and @iarna/toml2.2.5 main `toml.js`; neither declares exports/type. These are current physical DATA observations under generation22d1461ddc6b5f4d1305, not authority to execute installed Source. Shared exact conventions with High for a finite schema-owned path projection retaining existing custody/version/fullhash checks.

## Current Candidate Evidence Gap

Current peer runtime uses readInstalledBuildParserEntryV1 under the bound target and existing physical checks; createRequire removed from bootstrap. Two peer resolution laws exist: finite Ajv/SQLite helper projection plus actual compiler-captured helper against native require.resolve fixture oracle. The latter initially authored only runCapturedBuildSourceV1 and launched parent Original environment, without a received child Original/bare birth-owning program/projected control environment. High is adding that minimal genuine capability context and exact structural bootstrap confinement edges while retaining both laws. Current first-law implicit ceiling and physical URL locator also require explicit bounded selection/Source ownership alignment. These are regression-evidence issues, not a new runtime diagnosis.

## Final Parser Candidate Static Review 2026-10-21T19:22:37.886Z

The parser-specific repair and genuine capability regression are CLEAR for integration subject to separately reviewed current loader context. Nine direct/context files have bounded nofollow ordinary singlelink stable named/fd pins in `🗑️generated/bootstrap-require-witness-independent-current.json`; all pinned TypeScript and reconstructed actual generated entry AST have0 syntax diagnostics. Strict Ajv accepts18 neutral entry/physical rows; closed birth witness accepts genuine-shaped finite DATA and refuses extra fields, zero budget and wrong ingress.

Finite parser manifest DATA projection now feeds a physical entry-path checker that refuses missing/directory/symlink/changed/outside paths. Bootstrap derives package directory from held target/name, bounds actual installed manifest, invokes exact entry checker and native store confinement, retains generation binding, locked version, finite package walk/digest and existing package containment. No bootstrap createRequire remains. Structural law now asserts exact directory/manifest/entry initializers and both stage/package containment predicates, not just helper-name presence. Native require.resolve is used only as an independent fixture oracle.

The preserved actual capture law receives one Original, captures actual helper Source, then projects the same60000ms issued control environment into the owned child. The actual streamloader keeps import.meta.mainfalse and invokes exported runCapturedBuildSourceV1; this proof-rechecks and delegates once to local runScriptProgram(args), which receives the actual Original and reads its issued envelope. Bare guard is ordinary launch form, not claimed to fire under captured loader. Child emits one closed birth record; parent compares exact owner/maximumelapsed/deadline to the actual projected environment and separately compares one material record. Cleanup preserves primary failure. Two selected titles have explicit25000/75000ms ceilings, CLI75000ms, sum100000ms plus60000ms margin=160000msphase. Authored execution remains unrun.

## Separate Loader Context Blocker

Current loader context changed from27258/c796b921… to27974/c319d100…. Captured require declaration guard remains strict and now emits diagnostic DATA. However global import onResolve changed its admission predicate from `!candidate||!images.has(candidate)||!declared&&candidate!==entry` to `!candidate||!images.has(candidate)`. Thus an unmapped absolute/file-URL import of a captured sibling would be admitted where the previous explicit import-edge guard rejected it. This is a concrete authority-boundary semantic change outside the parser DATA repair. Root was notified; a full candidate cannot be cleared on the claim of unchanged loader until this separate delta is resolved or explicitly reviewed.
