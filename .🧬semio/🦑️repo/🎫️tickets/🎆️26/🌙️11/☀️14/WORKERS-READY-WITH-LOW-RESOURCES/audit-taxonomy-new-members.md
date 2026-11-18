# Taxonomy Admission Audit: New Member Directories since 2026-11-17

Read-only audit. No source files edited, no builds, no git writes.

## Summary

- Directories born since 2026-11-17 00:00 under the two roots (excluding node_modules, .cache, dist, target): **305**.
- **Not admitted: 0.** Every directory resolves under the taxonomy rules as emulated below.
- Admitted by explicit registry (exact `semanticDirectoryKinds` emoji+slug, or exact `memberNames`): **65**.
- Admitted only by a generic `memberSlugPattern` (lowercase kebab slug plus an emoji, under an owner chain that includes `tests` or `crate-module-children`): **240**. These are admitted without being listed anywhere, so a directory can exist without an explicit registry entry.
- Caveat: the owner chain was emulated in Python from `resolveSemanticDirectoryKindId` (TypeScript, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📂️semantic-directory-kinds/🟦️.ts`). The emulation is approximate (ancestor-based owners, no RGI-emoji verification beyond the leading-grapheme check), so treat the 0-count as a strong indication, not a proof. The authoritative check is the taxonomy loader test, see the last section.

## Taxonomy source of truth and admission rule

- Source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (SSOT vocabulary; keys `semanticDirectoryKinds` 812 entries of emoji + `slugPattern`, `semanticDirectoryMemberKinds` 330 entries of `ownerKindIds` + `memberNames` and optional `memberSlugPattern`). Root `🧰️framework/🔨️modules` is covered by the same SSOT through `members-of-modules`.
- Resolution order (from the TS resolver): (1) a `semanticDirectoryKinds` entry whose emoji is the leading grapheme and whose `slugPattern` matches the slug, honoring `parentKindIds`; (2) exact `memberNames` of a member kind whose `ownerKindIds` contains the parent or an ancestor kind; (3) `memberSlugPattern` of a member kind owned by the parent or an ancestor, requiring an admitted RGI emoji. Anything else is rejected.
- Consequence: the emoji is not checked against a list for pattern admission, only the slug. A name such as `🏠️home` is explicit, while `🏞️task-view` is admitted by the slug rule under `crate-module-children`, which allows kebab-case and CamelCase slugs (the `[A-Z]` branch).

## Special names requested

| Name | Example path | Created | Admitted | Rule |
|---|---|---|---|---|
| 🏞️task-view | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view` | 2026-11-18 10:00:02 | yes | slug pattern: crate-module-children |
| 🏃️executor | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🏃️executor` | 2026-11-18 10:14:40 | yes | slug pattern: crate-module-children |
| 🏠️local-key | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🪪️identity/🧪️tests/🏠️local-key` | 2026-11-18 10:07:27 | yes | slug pattern: members-of-tests |
| 🆓️free-space | (not born in window) | - | - | - |
| 🖼️image-slack | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🖼️image-slack` | 2026-11-18 05:35:45 | yes | slug pattern: crate-module-children |
| 🔷️typescript-compiler | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🔷️typescript-compiler` | 2026-11-17 22:07:15 | yes | slug pattern: crate-module-children |
| 🔎️module-references | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧮️task-model/🔎️module-references` | 2026-11-17 21:43:25 | yes | slug pattern: crate-module-children |
| 🤝️nested-custody | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🤝️nested-custody` | 2026-11-18 05:33:58 | yes | slug pattern: crate-module-children |
| 🧯️release-failure | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧯️release-failure` | 2026-11-18 05:33:58 | yes | slug pattern: crate-module-children |
| 🐢️speed | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🐢️speed` | 2026-11-17 18:00:00 | yes | slug pattern: crate-module-children |
| 📏️workspace-scale | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️workspace-scale` | 2026-11-17 17:30:00 | yes | slug pattern: crate-module-children |
| 🪧️door-board | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🪧️door-board` | 2026-11-17 17:47:27 | yes | slug pattern: crate-module-children |
| 🔭️look-queue | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🔭️look-queue` | 2026-11-17 13:20:45 | yes | slug pattern: crate-module-children |
| 🧮️farm-width | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🧮️farm-width` | 2026-11-17 12:00:00 | yes | slug pattern: members-of-tests |
| 🏠️machine-state | `🧰️framework/🔨️modules/🏃️process/🏗️build/🏠️machine-state` | 2026-11-17 15:14:51 | yes | slug pattern: crate-module-children |
| 🏠️home | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🏠️home` | 2026-11-17 23:29:31 | yes | explicit: members-of-modules |

## Full table

| Path (repo-relative) | Created (local) | Admitted | Rule |
|---|---|---|---|
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🧮️read-plan` | 2026-11-17 00:00:00 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🧮️read-plan/🧪️tests` | 2026-11-17 00:00:00 | yes | explicit dir: tests |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🧮️read-plan/🧬️schema` | 2026-11-17 00:00:00 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/💰️funding/🧪️tests/🫀️heartbeat` | 2026-11-17 00:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛑️tool-boundary` | 2026-11-17 00:00:49 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛑️tool-boundary/🐍️oracle` | 2026-11-17 00:00:49 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🎚️lanes` | 2026-11-17 00:07:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🔎️lookup-holds` | 2026-11-17 00:30:41 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🌍️spread` | 2026-11-17 00:37:56 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🛣️lanes` | 2026-11-17 00:37:56 | yes | explicit member: members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🌊️early-wave` | 2026-11-17 00:48:43 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🪂️set-aside` | 2026-11-17 02:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🪶️rest-walk` | 2026-11-17 02:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧪️tests/🤝️pool-reader` | 2026-11-17 02:31:56 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🪦️reaper` | 2026-11-17 02:47:22 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🐕️watchdog` | 2026-11-17 02:47:51 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🪦️reaper` | 2026-11-17 02:48:40 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🪦️busy-orphan` | 2026-11-17 02:52:41 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🐕️watchdog` | 2026-11-17 02:53:16 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/📥️take/🧪️tests/🚦️no-progress` | 2026-11-17 03:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🧪️tests/⏳️lease-wait` | 2026-11-17 03:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🆔️record-revision` | 2026-11-17 03:07:03 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🔔️keep-alive` | 2026-11-17 03:07:03 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🏟️fleet-rehearsal` | 2026-11-17 04:30:00 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🏟️fleet-rehearsal/🐜️owner` | 2026-11-17 04:30:00 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🏟️fleet-rehearsal/👷️machine` | 2026-11-17 04:30:00 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🏟️fleet-rehearsal/🧪️tests` | 2026-11-17 04:30:00 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🏟️fleet-rehearsal/🧬️schema` | 2026-11-17 04:30:00 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧯️old-record-settle` | 2026-11-17 04:31:18 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🧮️known-unit` | 2026-11-17 05:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🪦️stale-take` | 2026-11-17 05:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🧪️tests/🧰️members-set` | 2026-11-17 05:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/⏳️stretch-verdicts` | 2026-11-17 06:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🚪️exits` | 2026-11-17 06:33:32 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🚪️exits` | 2026-11-17 06:33:32 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🧮️lifecycle` | 2026-11-17 06:33:32 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧮️lifecycle` | 2026-11-17 06:33:32 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🤝️adopted-verdict` | 2026-11-17 07:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧒️children` | 2026-11-17 07:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧒️children/👷️child` | 2026-11-17 07:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🧭️stage-names` | 2026-11-17 07:01:29 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/🧪️tests/🪦️busy-orphan` | 2026-11-17 07:01:29 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/⏳️verdict-wait` | 2026-11-17 07:04:21 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🥇️essential` | 2026-11-17 08:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🥇️first-child` | 2026-11-17 08:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧪️tests/🪶️owner-retained` | 2026-11-17 08:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧬️read-verdicts` | 2026-11-17 08:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧬️read-verdicts/👷️child` | 2026-11-17 08:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🪘️refs-file` | 2026-11-17 08:39:08 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🪘️refs-file/👷️child` | 2026-11-17 08:39:08 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📂️source-names` | 2026-11-17 09:00:49 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📂️source-names/🧪️tests` | 2026-11-17 09:00:49 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📂️source-names/🧬️schema` | 2026-11-17 09:00:49 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🚧️read-parent` | 2026-11-17 09:19:27 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🚧️read-parent/🧪️tests` | 2026-11-17 09:19:27 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🚧️read-parent/🧬️schema` | 2026-11-17 09:19:27 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🚧️final-verdict` | 2026-11-17 09:22:55 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🚧️no-inline` | 2026-11-17 09:22:55 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🚧️read-parent` | 2026-11-17 09:22:55 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧯️take-no-forced-collection` | 2026-11-17 09:30:29 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🪢️shared-intake` | 2026-11-17 09:30:29 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🔖️document-head` | 2026-11-17 09:30:29 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🪧️derive-claim` | 2026-11-17 09:56:38 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep` | 2026-11-17 10:37:03 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/🧪️tests` | 2026-11-17 10:37:03 | yes | explicit dir: tests |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/🧬️schema` | 2026-11-17 10:37:03 | yes | explicit dir: schema |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/💿️leaked-images` | 2026-11-17 10:40:26 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🗃️child-custody` | 2026-11-17 10:40:46 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🎛️one-budget` | 2026-11-17 11:00:39 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🧮️farm-width` | 2026-11-17 12:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🧮️measured-ask` | 2026-11-17 12:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧭️verdict-failure` | 2026-11-17 12:49:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🧭️parent-routes` | 2026-11-17 12:49:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🪪️claim-lease` | 2026-11-17 12:49:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/⏳️sealing-progress` | 2026-11-17 12:53:47 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🔭️look-queue` | 2026-11-17 13:20:45 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🔭️look-queue/🧪️tests` | 2026-11-17 13:20:45 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🔭️look-queue/🧬️schema` | 2026-11-17 13:20:45 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🌊️streamed-shape` | 2026-11-17 13:30:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🔁️inventory-changed` | 2026-11-17 13:33:37 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🔁️store-changed` | 2026-11-17 13:33:37 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🧱️layer-lost` | 2026-11-17 13:33:37 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔑️key-table` | 2026-11-17 14:00:01 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔤️text-arena` | 2026-11-17 14:00:01 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🔑️key-table` | 2026-11-17 14:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🔤️text-arena` | 2026-11-17 14:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🦶️footprint-recycle` | 2026-11-17 14:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🧽️retained` | 2026-11-17 14:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🚧️read-parent/🧪️tests/🔭️look-scope` | 2026-11-17 15:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🔭️look-job` | 2026-11-17 15:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🌊️flat-memory` | 2026-11-17 15:00:47 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🏠️machine-state` | 2026-11-17 15:14:51 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🏠️machine-state/🧪️tests` | 2026-11-17 15:14:51 | yes | explicit dir: tests |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🏠️machine-state/🧬️schema` | 2026-11-17 15:14:51 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab` | 2026-11-17 15:29:54 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/👷️machine` | 2026-11-17 15:29:54 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🧪️tests` | 2026-11-17 15:29:54 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🧬️schema` | 2026-11-17 15:29:54 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/⏫️child-priority` | 2026-11-17 15:30:11 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🏠️machine-state/🧪️tests/🔒️spelling` | 2026-11-17 15:35:19 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🪜️stage-plan` | 2026-11-17 15:35:19 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🔤️plan-keys` | 2026-11-17 15:35:19 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🩹️describe-guard` | 2026-11-17 15:54:25 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/📖️documents` | 2026-11-17 16:00:00 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔖️journal` | 2026-11-17 16:00:00 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔖️journal/🧬️schema` | 2026-11-17 16:00:00 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🗃️file-ports` | 2026-11-17 16:00:00 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🔖️journal` | 2026-11-17 16:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid` | 2026-11-17 16:06:59 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪢️SharedBorderRow` | 2026-11-17 16:21:58 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🖥️host-matrix` | 2026-11-17 16:30:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🔪️slice-read` | 2026-11-17 16:31:29 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🈯️LanguageChoice` | 2026-11-17 16:39:38 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️CardStack` | 2026-11-17 17:05:12 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🧩️component/🔹️shared-border-padding` | 2026-11-17 17:16:53 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️tests/🧩️component/🔹️shared-border-cells` | 2026-11-17 17:16:53 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️workspace-scale` | 2026-11-17 17:30:00 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️workspace-scale/🧬️schema` | 2026-11-17 17:30:00 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🆔️host-identity` | 2026-11-17 17:32:40 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🥞️failure-frames` | 2026-11-17 17:32:40 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🪪️owner-idle` | 2026-11-17 17:32:40 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🎯️probes` | 2026-11-17 17:35:48 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🔭️scope` | 2026-11-17 17:35:48 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📚️wide-failure` | 2026-11-17 17:44:18 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🔌️providers/🎯️requested-group` | 2026-11-17 17:44:18 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🪧️door-board` | 2026-11-17 17:47:27 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🪧️door-board/🧪️tests` | 2026-11-17 17:47:27 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🪧️door-board/🧪️tests/🐍️oracle` | 2026-11-17 17:47:27 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔐️custody/🧪️tests/🚪️bounded-door` | 2026-11-17 17:49:18 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🧪️tests/🪧️holder` | 2026-11-17 17:53:40 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🤝️handover` | 2026-11-17 17:54:20 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🤝️handover` | 2026-11-17 17:58:22 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🐢️speed` | 2026-11-17 18:00:00 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🐢️speed/🧪️tests` | 2026-11-17 18:00:00 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🐢️speed/🧬️schema` | 2026-11-17 18:00:00 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/⏳️progress-timer` | 2026-11-17 18:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/⏳️progress-timer/🛰️service` | 2026-11-17 18:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🪟️window-pane-exclusivity` | 2026-11-17 18:43:17 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🗣️restart-cause` | 2026-11-17 19:11:30 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧠️admission-memo` | 2026-11-17 19:11:30 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧠️admission-memo/🧪️tests` | 2026-11-17 19:11:30 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🧪️tests/❓️unknown-input` | 2026-11-17 19:11:30 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🧪️tests/🧭️source-owners` | 2026-11-17 19:11:30 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🪟️recheck-window` | 2026-11-17 19:26:57 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/🧫️fixtures/🔀️pane-fold-exclusive` | 2026-11-17 19:54:32 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/🪆️projected-parent` | 2026-11-17 20:19:13 | yes | explicit dir: mutation-test-profile |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🧪️tests/🔭️observed-files` | 2026-11-17 21:05:14 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/⏳️async/💤️lazy-slots` | 2026-11-17 21:13:01 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/📥️take/🧪️tests/🪧️tail-report` | 2026-11-17 21:14:12 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🛂️admission/🔭️observed-catalog` | 2026-11-17 21:15:46 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🩺️pool-status-read` | 2026-11-17 21:17:20 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧪️tests/🪟️windows-progress` | 2026-11-17 21:37:47 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🔁️resume` | 2026-11-17 21:37:47 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧹️staging-sweep` | 2026-11-17 21:37:47 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧹️staging-sweep/🧪️tests` | 2026-11-17 21:37:47 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧹️staging-sweep/🧬️schema` | 2026-11-17 21:37:47 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧮️task-model/🔎️module-references` | 2026-11-17 21:43:25 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧮️task-model/🔎️module-references/🧪️tests` | 2026-11-17 21:43:25 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧮️task-model/🔎️module-references/🧬️schema` | 2026-11-17 21:43:25 | yes | explicit dir: schema |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🙋️asks` | 2026-11-17 21:45:33 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🙋️asks/🧪️tests` | 2026-11-17 21:45:33 | yes | explicit dir: tests |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🙋️asks/🧬️schema` | 2026-11-17 21:45:33 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🧮️shared-budget` | 2026-11-17 21:45:33 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧪️tests/🙋️ask` | 2026-11-17 21:45:33 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🛂️admission/🧷️catalog-producer` | 2026-11-17 21:45:58 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🚫️doomed-handoff` | 2026-11-17 21:46:27 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🌊️owner-scale` | 2026-11-17 21:46:27 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🐘️never-here` | 2026-11-17 21:46:27 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧮️task-model/🔎️module-references/🧪️tests/🧫️corpus` | 2026-11-17 21:48:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🤝️selected-fleet` | 2026-11-17 21:50:51 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🧯️failure/🔁️same-cause` | 2026-11-17 21:54:14 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🧮️read-plan/🧪️tests/🪣️chunks` | 2026-11-17 21:54:50 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🔷️typescript-compiler` | 2026-11-17 22:07:15 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🔷️typescript-compiler/🧪️tests` | 2026-11-17 22:07:15 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🔷️typescript-compiler/🧬️schema` | 2026-11-17 22:07:15 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🩹️repair-rounds` | 2026-11-17 22:10:50 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧵️retain-rows` | 2026-11-17 22:11:42 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🔷️typescript-compiler/🧪️tests/🚫️bare-name` | 2026-11-17 22:15:51 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🕰️stale-stage` | 2026-11-17 22:18:18 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🔷️tooling-compiler` | 2026-11-17 22:19:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🫙️fresh-namespace` | 2026-11-17 22:34:46 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧊️stale-facts` | 2026-11-17 23:06:08 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🧯️failure/🏷️owner-named` | 2026-11-17 23:06:28 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🤝️publication-capture` | 2026-11-17 23:06:28 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🏷️owner-tag` | 2026-11-17 23:06:28 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/📋️slice-report/🧪️tests/🏁️final-line` | 2026-11-17 23:06:28 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🧊️frozen` | 2026-11-17 23:22:12 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🧪️tests/🧊️frozen` | 2026-11-17 23:22:12 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧪️tests/🔧️tooling-edit` | 2026-11-17 23:22:41 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🌊️seal-under-churn` | 2026-11-17 23:22:41 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🏠️home` | 2026-11-17 23:29:31 | yes | explicit member: members-of-modules |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🏠️home/🧪️tests` | 2026-11-17 23:29:31 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🏠️home/🧬️schema` | 2026-11-17 23:29:31 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🩻️heavy-refusal` | 2026-11-17 23:43:37 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🎯️roster-scope` | 2026-11-17 23:54:05 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/📏️plan-scale` | 2026-11-18 00:18:33 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🧪️tests/🧾️receipt` | 2026-11-18 00:35:53 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️CardStack/🎯️targets` | 2026-11-18 00:44:36 | yes | explicit dir: targets |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️CardStack/🎯️targets/🧊️wgpu` | 2026-11-18 00:44:36 | yes | explicit dir: wgpu-target |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🔎️installed-watcher` | 2026-11-18 01:04:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🤝️fleet` | 2026-11-18 01:04:50 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🤝️fleet/🧪️tests` | 2026-11-18 01:04:50 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🤝️fleet/🧬️schema` | 2026-11-18 01:04:50 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🧭️turn` | 2026-11-18 01:07:21 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🤝️pool-admission` | 2026-11-18 01:07:21 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🧊️alone-fresh` | 2026-11-18 01:17:09 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🏎️ports` | 2026-11-18 01:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📏️artefact-scale` | 2026-11-18 01:36:23 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/🎯️need-scope` | 2026-11-18 01:39:40 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/🧾️sealed-plan` | 2026-11-18 01:43:26 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🩹️evidence` | 2026-11-18 01:45:06 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧾️sealed-plan` | 2026-11-18 02:00:00 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🧪️tests/🧭️closure` | 2026-11-18 02:01:38 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧲️adopt-registry` | 2026-11-18 02:02:59 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange` | 2026-11-18 02:20:41 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/🧪️tests` | 2026-11-18 02:20:41 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/🧬️schema` | 2026-11-18 02:20:41 | yes | explicit dir: schema |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🌍️ground` | 2026-11-18 02:21:48 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🌍️world` | 2026-11-18 02:23:01 | yes | explicit member: members-of-members-of-modules |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/🔌️attempt` | 2026-11-18 02:25:49 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🧭️sites` | 2026-11-18 02:26:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🔌️attempt` | 2026-11-18 02:26:19 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🔗️wiring` | 2026-11-18 02:30:03 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🖥️real-server` | 2026-11-18 02:34:05 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/📏️revision` | 2026-11-18 02:34:34 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/📏️revision/🧪️tests` | 2026-11-18 02:34:34 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery` | 2026-11-18 03:00:01 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🔌️real` | 2026-11-18 03:00:01 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🧪️tests` | 2026-11-18 03:00:01 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🧬️schema` | 2026-11-18 03:00:01 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🧺️bounds` | 2026-11-18 03:00:01 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/📦️packages` | 2026-11-18 03:07:32 | yes | explicit dir: packages |
| `🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/📦️packages/🦀️rust` | 2026-11-18 03:07:32 | yes | explicit dir: rust-language |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📦️bundle-load` | 2026-11-18 03:12:09 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/📥️take/🧪️tests/🔒️decided` | 2026-11-18 04:30:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/🧪️tests/🫀️idle-claim` | 2026-11-18 04:30:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/📋️slice-report/🧪️tests/🪜️read-steps` | 2026-11-18 04:30:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim` | 2026-11-18 05:14:02 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🧪️tests` | 2026-11-18 05:14:21 | yes | explicit dir: tests |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🧬️schema` | 2026-11-18 05:14:21 | yes | explicit dir: schema |
| `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim` | 2026-11-18 05:19:30 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧯️release-failure` | 2026-11-18 05:33:58 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧯️release-failure/🧪️tests` | 2026-11-18 05:33:58 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧯️release-failure/🧬️schema` | 2026-11-18 05:33:58 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🤝️nested-custody` | 2026-11-18 05:33:58 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🤝️nested-custody/🧪️tests` | 2026-11-18 05:33:58 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🤝️nested-custody/🧪️tests/🔒️cross-process` | 2026-11-18 05:33:58 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🤝️nested-custody/🧪️tests/🔒️cross-process/🧱️holder` | 2026-11-18 05:33:58 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🤝️nested-custody/🧬️schema` | 2026-11-18 05:33:58 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🧶️nx/🚦️single-flight/🧪️tests/🕰️long-holder` | 2026-11-18 05:34:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/🌱️regrow` | 2026-11-18 05:35:45 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🖼️image-slack` | 2026-11-18 05:35:45 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🖼️image-slack/🧪️tests` | 2026-11-18 05:35:45 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🗃️replica-store` | 2026-11-18 05:35:45 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🗃️replica-store/🧪️tests` | 2026-11-18 05:35:45 | yes | explicit dir: tests |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧫️fixtures/🧪️conformance/🖥️composite/🗂️card-stack` | 2026-11-18 05:46:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧪️tests/🤝️holder-custody` | 2026-11-18 06:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🤝️holder-custody` | 2026-11-18 06:00:01 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests/🔖️task-semantics` | 2026-11-18 06:02:13 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests/🔖️task-semantics/🧬️schema` | 2026-11-18 06:02:13 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🔒️loaded-implementation` | 2026-11-18 06:02:13 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📂️source-names/🧪️tests/🕳️hollow` | 2026-11-18 06:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🌍️bigger-host` | 2026-11-18 06:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🔁️retry-alone` | 2026-11-18 06:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🧪️fixture` | 2026-11-18 06:52:23 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🧪️tests/🪟️wrapped-refusal` | 2026-11-18 07:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🪟️rename-settlement` | 2026-11-18 07:00:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/💽️disk-recovery` | 2026-11-18 07:00:07 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🖼️image-slack/🧪️tests/♻️reclaim` | 2026-11-18 07:00:08 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🧪️tests/📥️need` | 2026-11-18 07:00:08 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/🦀️cargo-workspace/🔒️lock/📐️closure` | 2026-11-18 07:02:21 | yes | slug pattern crate-module-children |
| `🧰️framework/🔨️modules/🏃️process/🦀️cargo-workspace/🔒️lock/📐️closure/🧪️tests` | 2026-11-18 07:02:21 | yes | explicit dir: tests |
| `🧰️framework/🔨️modules/🏃️process/🦀️cargo-workspace/🔒️lock/📐️closure/🧬️schema` | 2026-11-18 07:02:21 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🧷️failure-log` | 2026-11-18 07:02:38 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📂️source-names/🧪️tests/🤖️machine-local-dirs` | 2026-11-18 07:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🫥️self-progress` | 2026-11-18 07:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🔷️typescript-compiler/🧪️tests/🍞️captured` | 2026-11-18 07:46:16 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🔭️snapshot` | 2026-11-18 07:54:07 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🧮️aside-census` | 2026-11-18 07:55:54 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/🆓️free-space/🧪️tests` | 2026-11-18 08:00:09 | yes | explicit dir: tests |
| `🧰️framework/🔨️modules/🏃️process/🆓️free-space/🧪️tests/🪟️windows-shaped` | 2026-11-18 08:00:09 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/🆓️free-space/🧪️tests/🪟️windows-shaped/🧫️fixtures` | 2026-11-18 08:00:09 | yes | explicit dir: fixtures |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️agent/💽️disk/🧪️tests` | 2026-11-18 08:00:09 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️agent/💽️disk/🧪️tests/🪟️volume` | 2026-11-18 08:00:09 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🕳️disk-unknown` | 2026-11-18 08:00:09 | yes | slug pattern members-of-tests |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🪦️stale-queue` | 2026-11-18 08:30:16 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🪢️late-refusal` | 2026-11-18 08:44:41 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🧾️poll-ledger` | 2026-11-18 08:49:10 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🪜️step-overdue` | 2026-11-18 09:45:14 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/📥️take/🧪️tests/📣️published` | 2026-11-18 09:47:45 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🧪️tests/🗂️task-views` | 2026-11-18 10:00:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view` | 2026-11-18 10:00:02 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🧪️tests` | 2026-11-18 10:00:02 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🧪️tests/🐍️oracle` | 2026-11-18 10:00:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🧪️tests/🦀️cargo` | 2026-11-18 10:00:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🧪️tests/🧰️world` | 2026-11-18 10:00:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🧪️tests/🧹️collection` | 2026-11-18 10:00:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🧪️tests/🪂️fallback` | 2026-11-18 10:00:02 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🧬️schema` | 2026-11-18 10:00:02 | yes | explicit dir: schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🪪️identity/🧪️tests/🏠️local-key` | 2026-11-18 10:07:27 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🏃️executor` | 2026-11-18 10:14:40 | yes | slug pattern crate-module-children |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🏃️executor/🧪️tests` | 2026-11-18 10:14:40 | yes | explicit dir: tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🏠️state-directory` | 2026-11-18 10:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧪️tests/🏠️state-directory` | 2026-11-18 10:30:00 | yes | slug pattern members-of-tests |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🏃️executor/🧬️schema` | 2026-11-18 10:30:03 | yes | explicit dir: schema |

## Read-only verification available

- No command was run. Pure taxonomy tests exist under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/`, for example `🔏️path-emoji-statutes/🟦️.ts`, `🔬️workspace-contract/🧱️taxonomy-normalization/` and `⏱️taxonomy-scaling-laws/🟦️.ts`. The module under test is `🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts` (the resolver is in `🔍️discovery/📂️semantic-directory-kinds/🟦️.ts`).
- The repo's way to run them is the `repo` binary (per CLAUDE.md), which would be a build/test run, so it was not executed here. Run them with `repo test` on the library test project, scoped to these paths, to get the authoritative verdict.
- Note that the Go layout check (`🐹️go-layout`) is a separate gate from this taxonomy admission and is not covered by the verdict above.
