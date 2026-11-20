# ISO 16757, VDI 3805, and DIN V 18599 example verification

## Typed Rust DSL

The audit links each family's production snapshot crate and framework `pack` from a matching WASI compiler cohort. For each checked-in asset it calls `ArtifactDsl::parse_dsl`, prints the typed snapshot, parses that output again, and compares the two canonical prints byte for byte.

| Family | Result | Examples |
| --- | --- | --- |
| ISO 16757 | 2 of 2 passed | Conforming and deliberately broken catalogues |
| VDI 3805 | 22 of 22 passed | Demo, nonconforming valve dataset, and both examples for each of sheets 3, 4, 5, 6, 7, 8, 16, 19, 53 and 60 |
| DIN V 18599 | 5 of 5 passed after repair | Demo, compliant detached, noncompliant detached, cooled office, and compliant two-zone |

The ISO and VDI runs used the base Norm component's `pe88539` lane and JCO 1.39.0. The typed decoder accepted the braced map records added before this audit. The intentionally defective examples remain parseable so the application can evaluate their normative defects. ISO and VDI needed no further asset edits.

The older DIN V 18599 model referenced a kernel metadata hash absent from the cache. The combined build's `p9171e3` lane supplied a matching model and framework. The original five files all failed on the required `thermal-mass-class` zone column. Each zone now explicitly stores `medium` and `90` in the `thermal-mass-class:ENUM` and `c-wirk-wh-m2k:NUM` columns. The model's code-built examples use those values, including both zones in the two-zone example. The same WASI decoder and printer passed all five updated files. The production examples use `include_str!`, so guest builds that pinned inputs earlier still contain the previous fixtures.

## Pack parity

The two ISO 16757 examples have checked-in Pack siblings. The VDI 3805 and DIN V 18599 examples have no Pack siblings.

Both previous ISO Packs failed the current production decoder with `schema error: expected Text, found Enum(4) at 1:1`. The audit linked the production ISO snapshot and Pack codec from the combined build's `p9171e3` WASI lane. The published format was regenerated from each already-verified DSL snapshot. The candidate decoded to the same typed snapshot and re-encoded byte for byte. Both newly checked-in Packs passed typed comparison with their DSL on a separate run. The broken example still includes its intended nonconforming data.

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🎒️.pack.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🚫️broken/🎒️.pack.semio`

## Standards Play

The catalogue picker probe loads a deliberately nonconforming example and restores the demo. It requires a change and restoration of the compliance summary. Earlier route logs showed both shells ready even when `setActiveExample` refused the initial fixture.

Chromium requested the newly linked core from component run `wjUjBhj` over the active Standards server at port 6034. Both preview core requests returned HTTP 200. Both routes reached `data-shell-error` before the picker became available. Document loading and the automatic `setActiveExample` command reported `original-turn.source-not-ready`. The DIN 4108 preview encountered the same source-readiness failure in its archive path. The preview core predates strict retained-total validation.

The bundle then staged with component hash `5a8d21919808dcd10964a88d42d0a6cf6ae3e6d20938746cd75fe7efc57a9a99`. The unmodified published-resource requests also returned HTTP 200 for both routes. The two browser shells stopped with the same source-readiness error before the picker. The stack led through the generated bridge from `poll` to `admitWire` to `OriginalOperationSource.issue`. A separate per-page diagnostic observed the earlier native `semantic` refusal from `reactor.poll` at epoch 18739 for ISO 16757 and epoch 18585 for VDI 3805. The diagnostic left the served artifact intact. The DIN 4108 archive trace also observed a semantic refusal at epoch 18585 and recovered its diagnostic as archive hydration replay. The ISO and VDI picker transitions remain unverified while the shell stops at the native refusal.

## Ticket files

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/audit-eurocode-assets.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/audit-iso16757-pack.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/link-norm-dsl-audit.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/probe-norm-catalogue-examples.mjs`

## DIN V 18599 asset files

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/✅️compliant-detached/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/❄️cooled-office/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/❌️noncompliant-detached/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢️compliant-two-zone/🗣️.dsl.semio`
