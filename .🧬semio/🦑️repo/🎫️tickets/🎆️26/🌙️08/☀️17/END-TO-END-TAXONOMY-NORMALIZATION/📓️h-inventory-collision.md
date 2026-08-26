# H-INVENTORY-COLLISION Report

## Method

The bounded census used `git ls-files -z -- ':!compose/**'`. The `compose/**` pathspec was removed before path classification or filesystem reads. Directories were derived from admitted tracked paths. Unicode checks used NFC, VS16 removal and locale-independent lowercase comparison. A `sha256-merkle-v1` filesystem pass was run only on the opaque root to establish its before digest; symlinks were hashed without following them.

## Counts

| Measure | Count |
| --- | ---: |
| Tracked non-`compose` files | 64,707 |
| Derived non-`compose` directories | 37,971 |
| Directories without an emoji-leading identity under the current coarse classifier | 5,928 |
| Files whose stem is not already a coarse kind-only basename | 49,958 |
| Current NFC/case/VS16 collision groups | 1 |
| Naive same-parent, same-kind destination collision groups | 1,254 |
| Paths longer than 240 UTF-8 bytes | 8,212 |
| Windows reserved/trailing-space hazards | 1 |

The existing normalized collision is the pair `🧪test-final2.txt` and `🧪️test-final2.txt` in ticket `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`. The Windows hazard is a tracked trailing-space path in the same historical ticket tree. The large path-length count proves semantic child-directory creation must budget complete destination lengths before any mutation.

## Collision Shape

The 1,254 naive same-kind groups are not errors by themselves; they are the exact evidence that semantic stems cannot simply be dropped. Representative groups are `.claude/{launch,settings}.json`, `.cursor/{hooks,mcp}.json`, every `.cursor/plans/*.plan.md`, `.vscode/{extensions,launch,mcp,settings}.json`, root JSON contracts, Storybook specifications/stories, and historical ticket reports. Each concern needs a semantic child directory or an exact fixed/configurable entry contract.

## Owner Shards

Tracked files are concentrated in `✏️s` (41,711), `.🧬semio` (18,818), `🧰️framework` (3,318), `.cursor` (433), `♻️mit-bestand` (220), `.storybook` (71) and `🌎️hub` (38). Safe write shards are therefore one plugin/extension owner below `✏️s`, one framework module/product owner, one historical ticket, and one dot-tool family. Root manifests and shared registries require sole writers.

## Required Controls

- Plan every admitted file to its final path before mutation; directory moves are expanded to file moves.
- Reject current and destination collisions after byte, NFC, case-folded and VS16-folded comparison.
- Reject any destination exceeding the configured cross-platform byte budget or carrying a Windows reserved/trailing-dot/space segment.
- Never use a decorative fallback emoji. Unknown semantic stems remain unresolved.
- Treat dot-tool directories and their fixed child names through exact contracts, not broad dot-path exemptions.
- Inventory the active ticket explicitly because its new reports are untracked until integration; historical tracked tickets remain in ordinary scope.

## Acceptance Checks

Two inventories must be byte-identical; every admitted source appears exactly once; the plan contains no unresolved semantic or collision; the second plan is empty; and the final opaque digest equals `a312d352730435c1c2053e7a82545fce53f3d6a00a32d84863f945555717e9dc`.
