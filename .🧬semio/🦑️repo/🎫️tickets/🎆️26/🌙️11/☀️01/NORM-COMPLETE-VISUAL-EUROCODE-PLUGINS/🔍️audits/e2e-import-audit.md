# Norm Import and Aggregate Audit

Date: 2026-11-04. Read-only static audit. Paths were checked against the live workspace; no execution or type-resolution success is implied.

Scanned 37819 files beneath norm and dev/norm; 3374 relative TypeScript import/export/require references, 3440 Cargo path assignments. Missing imports: 12. Missing Cargo paths: 0.

## Missing TypeScript Targets

- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🟦️.ts:9 → ./🎭️modes/✏️edit/🪟️windows/📥️inputs/🟦️
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🟦️.ts:10 → ./🎭️modes/✏️edit/🪟️windows/📊️results/🟦️
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🟦️.ts:9 → ./🎭️modes/👁️view/🪟️windows/📊️report/🟦️
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts:496 → ./🧱️change-plated-lateral-pressure-coefficient/🧬️schema/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts:516 → ./🧱️change-plated-stiffener-flange-thickness/🧬️schema/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts:527 → ./🧱️change-plated-transverse-stiffener-inertia/🧬️schema/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🟦️.ts:9 → ./🎭️modes/✏️edit/🪟️windows/📥️inputs/🟦️
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🟦️.ts:10 → ./🎭️modes/✏️edit/🪟️windows/📊️results/🟦️
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🟦️.ts:9 → ./🎭️modes/👁️view/🪟️windows/📊️report/🟦️
- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🪧marker-grips/🟦️.ts:4 → ../../../../../🎬️structural-scene/🧬️schema/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🪟️window/🟦️.ts:4 → ../../../../../🎬️structural-scene/🧬️schema/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🫧️live-lanes/🧬️schema/🟦️.ts:2 → ../../../../../../🎬️structural-scene/🧬️schema/🟦️.ts

## Missing Cargo Paths

None.

## Artifact Aggregate Targets

Expected 15 artifacts; found 15 TypeScript aggregate projects. Projects with a test target: 15. Projects with an existing command script: 15.

- 🏭️vdi3805 (@semio-tech/norm-vdi3805): build, check, test; script=true; test=repo ./📜️script.ts test
- 🧩️en1994 (@semio-tech/norm-en1994): build, check, test; script=true; test=repo ./📜️script.ts test
- 🪨️en1996 (@semio-tech/norm-en1996): build, check, test; script=true; test=repo ./📜️script.ts test
- 🏋️en1991 (@semio-tech/norm-en1991): build, check, test; script=true; test=repo ./📜️script.ts test
- ⚡️din18599 (@semio-tech/norm-din18599): build, check, test; script=true; test=repo ./📜️script.ts test
- 🔩️en1993 (@semio-tech/norm-en1993): build, check, test; script=true; test=repo ./📜️script.ts test
- 🌍️en1997 (@semio-tech/norm-en1997): build, check, test; script=true; test=repo ./📜️script.ts test
- 📇️iso16757 (@semio-tech/norm-iso16757): build, check, test; script=true; test=repo ./📜️script.ts test
- 🏛️en1992 (@semio-tech/norm-en1992): build, check, test; script=true; test=repo ./📜️script.ts test
- 🌬️din16798 (@semio-tech/norm-din16798): build, check, test; script=true; test=repo ./📜️script.ts test
- ⚖️en1990 (@semio-tech/norm-en1990): build, check, test; script=true; test=repo ./📜️script.ts test
- 🧱️din4108 (@semio-tech/norm-din4108): build, check, test; script=true; test=repo ./📜️script.ts test
- 🪵️en1995 (@semio-tech/norm-en1995): build, check, test; script=true; test=repo ./📜️script.ts test
- 🪶️en1999 (@semio-tech/norm-en1999): build, check, test; script=true; test=repo ./📜️script.ts test
- 🫨️en1998 (@semio-tech/norm-en1998): build, check, test; script=true; test=repo ./📜️script.ts test

## Method Limits

The scan excludes dependency, build, store and cache trees. Relative path existence covers static and dynamic import/export/require quoted references, not alias resolution or generated runtime specifiers. Cargo path assignments include library and binary source paths as well as dependency directories; paths are checked for existence only. Aggregate target presence is configuration coverage, not evidence that its internal suite reaches every feature.
