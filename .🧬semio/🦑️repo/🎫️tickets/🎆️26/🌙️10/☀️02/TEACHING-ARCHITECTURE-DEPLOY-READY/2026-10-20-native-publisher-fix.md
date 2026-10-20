# Native Frontend Publisher Ownership Fix

Root19034 establishes genuine physical RED: managed Go compilation completed, then actual stageArtifacts refused `Unowned artifact directory` for fresh n2. Log is generated/2026-10-20-native-linux-frontend-build2.log. This agent ran no command/test.

## Cause And Exact Fix

repo-mcp runBuildScript resolved binary then called mkdirSync(dirname(binary),recursive:true), creating the FINAL artifact publication directory without its marker. runCanonicalGoBuild captures output privately; buildGoArtifactsV1 eventually calls stageArtifacts(dirname(output),owner,files). Actual executeArtifactPublicationV1:79 correctly refuses any existing final directory without the marker/owner. At:81 the publisher itself creates dirname(staging); its source-capture port also owns parent/sibling-stage creation. No replacement mkdir is necessary in the native frontend builder.

Removed exactly that premature mkdir and unused fs/dirname imports from the builder. Publisher/capture guards, marker format, cleanup and all owner validation are unchanged. No empty-directory adoption, fake marker, fabricated receipt, Source copy, new runtime interface/dependency or n2modification was introduced.

## Schema-First Regression

Extended the EXISTING installation schema/fixture/test owner.8language-neutral rows cover Linux/macOS/Windows spelling, fresh nested/default destinations, same-owner intent and unowned/foreign/symlink publication refusal propagation. The test compiles the ACTUAL runBuildScript declaration using existing independent TypeScript and esbuild paths, twice for incoming versus newly issued control (32preparedvariants). Its canonical-Go handoff port asserts an empty filesystem-effect transcript, exact module/output args, Original/control/env/budget, and exact identity of any publication refusal. SQLite independently constructs the expected output argument JSON; Ajv validates each closed fixture row. The test would expose the old mkdir through the filesystem-effect transcript before handoff.

This is owner-boundary neutral verification, not an actual physical publication run for all destination states. The destination labels describe the canonical port result supplied by each vector; the test does not implement or weaken the publisher. Existing primitive publisher tests cover physical owner/idempotence/foreign behavior separately. Actual fresh n3is the next physical GREEN Root owns. No neutral test pass or n3result is claimed here.

## Changed Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📥️installation/🧬️schema/🔣️.json`: 4130B / SHA256 `5cd3488e7a281508f7b53e17ced596a24dd4d50cdf28b97ded5ea392d2ab547f` (preimage `125df624f9aad091a917beb5058bfcfe02400814b70b6c31032184565ce9f7de`).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📥️installation/🧫️fixtures/🔣️.json`: 5069B / SHA256 `faa27bddcb87f1530b861c607ab4af03347b45f6854564357b5886433e347e26` (preimage `fa275bfb730f9d7670577e5ba1ffc76f017fc7d011e4c22063daa76cb44967f3`).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📥️installation/🧪️tests/🟦️.ts`: 16310B / SHA256 `71d864748af202b6a534fed8a6c6e82415b8f22f0cb55375daa8a0519d204723` (preimage `1ab09da8e9ae367958fab4cc096c3492535e3472970341e031c3163c6045cc14`).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🟦️.ts`: 2743B / SHA256 `d8f982cba9dceadf5b3e002acd3b25291e770ca6040e827879f9ffb25d98b522` (preimage `8a84dc5229f8e69b8ff3d483c130fc2fc33d0ab905123a7080cdf8cbf3a624dd`).

Owning native source-test target: `repo run repo-mcp:test-install` (existing source execution metadata; no new target). Source test additions were statically reviewed here only. CI workflow remains unchanged pending Root9854actualRED. Root can run the already-reviewed fast native frontend recovery with a fresh absent n3destination; do not re-use/adopt n2.
