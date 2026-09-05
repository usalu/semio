# DWG Naming Repair

## Handpicked decisions

- `🔖️ac1018` → `4️⃣ac1018`: AC1018 is the R2004 DWG generation.
- `🔖️ac1024` → `🔟ac1024`: AC1024 is the 2010 DWG generation.
- Both real drawing assets are `🏛️architectural.dwg`, matching their architectural-building content instead of the generic document marker.
- Each local oracle is `🔮️oracle`; editor/viewer option facets are `☑️options` beside `🎚️config`.
- The version mutation and its corpus owner are `🏷️set-version-info`; its committed pair is `⬅️before.json` and `➡️after.json`.
- The structure inference is presentation-correct `🗂️structure`.
- AC1018's set-snapshot payload schema is `🧬️.schema.json`, distinct from its `🔣️.json` descriptor; the descriptor and matching oracle manifest now resolve that exact leaf.
- Existing wire identities such as `ac1018`, `ac1024`, `set-version-info`, Rust module identifiers, and serialized payload values were not renamed.

All physical moves were literal, non-overwriting moves. Incoming Stdio package, registry, oracle-package, fixture-sweep, allow-list, local include, asset, descriptor, and oracle references were updated to their exact current coordinates. The central taxonomy contains exact owner-scoped oracle overrides for both standards and validates with no problems.

## Verification

- Strict audit: 313 files, 189 directories, 502 governed paths; missing 0, generic 0, presentation 0, spacing 0, duplicate 0, multiple 0, reserved-emoji 0, oracle 0.
- `bun nx run @semio-tech/repo-test-domain:test-fixture-verify -- --artifact s.stdio.dwg`: 2 fixtures, 0 file problems.
- Every DWG JSON authority parses with `jq empty`.
- A current-source scan found no old canonical DWG standard, architectural-carrier, fixture-pair, oracle, option, mutation, or inference coordinate. Independent serializer trees in other plugins retain their separately owned standard-directory identities and are not incoming references to the moved DWG artifact roots.
- Full native Stdio qualification is deferred until the concurrently repaired LAS and STEP mounts are source-coherent; no native DWG pass is claimed here.
