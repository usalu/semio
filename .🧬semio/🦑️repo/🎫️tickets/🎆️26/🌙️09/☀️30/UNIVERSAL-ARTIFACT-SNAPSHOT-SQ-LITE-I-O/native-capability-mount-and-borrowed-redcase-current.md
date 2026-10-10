# Native Capability Mount and Borrowed Redcase Audit

Read-only current source audit, no builds.

## Closed original capability

Actual contract/reader lives at Repo caching/📦️artifacts/📋️native-orchestration/📥️invocation/{🟦️.ts,🧬️schema/🔣️.json}. Required closed fields: version1, repositoryRoot, artifactDirectory, command `{kind:"native-owner-command",manifest,workingDirectory,program,arguments}`, transport `{maximumBytes,maximumLines}`, child `{maximumElapsedMilliseconds}`, network `{offline}`. Extra owner and string-command fields are invalid. Root/artifact/manifest/cwd/program paths have256 limit; child finite integer1..86400000, transport positive safe integers.

Both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` selected Actor receiving Source/Native, Value literal Source/Native, CSV subset validation Source/Native and TSV subset validation Source/Native currently declare only owner plus string command. These eight capability envelopes are not valid closed NativeOwnerCapabilities. Needed values must come from actual original caller declaration: repository root, selected artifact directory exactly matching transported CARGO_TARGET_DIR, exact owner-command selected manifest/cwd/program/argument vector after Nx transformation, original output transport ceilings, original child ceiling and original offline scope. Do not merely reuse top-level bun nx string as program/arguments: owner-command reader compares its actual child request. Do not mint default artifact/output/time/network authority inside a downstream parser.

## Lower SQL redcase

Receiving same_token64 preserves exact bytes only for single quotes. Current fixture contains original DEFAULT doublequoted alpha;beta and changed doublequoted Alpha;beta expected false. Native comparison is case insensitive for these doublequoted tokens and therefore admits changed literal. Independent Source compares sqlite_master actual/authored declarations and correctly sees mismatch. Fix literal-context comparison or declare conservative quoted-token exactness with authored identifier-case tests; do not change oracle to permit literal mutation.

## Original scalar relocation law

Store native-retirement test OriginalRelocation holds both original and replacement vectors under one admitted frame. Input.take follows body.admit_frontier and native checkpoint; partial scalar checks verify exact authored values and no retired scalar in both fields before actual recipient close. Zero release forces pending frame preservation; heap births/releases compared with original receipt. Cases include complete original order, refusal after first move, before reversal, and during reversal. Teardown uses unbounded while has_retirement_owner; add authored close turn bound/progress law to avoid hanging on a retained zero-progress defect. This is source qualification, not evidence of execution or native compile.
