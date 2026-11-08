# 1107 Audit: Original Work Migration (lane aud-migration)

Read-only audit. No repo runs, no edits outside C, no sub-agents. Working tree inspected with `git grep` (includes uncommitted edits).

STATUS: UNVERIFIED. Nothing compiled. Consumer resolution is inferred from source text only.

## 1. Tickets and commits

- `🌙️11/☀️06` and `🌙️11/☀️07` do not exist. `🌙️11/☀️05` has four tickets (CONSOLIDATE-NEIGHBORING-UI-BORDERS, GET-STAFFAGE-WEBSITE-RUNNING-END-TO-END, REPAIR-MICRO-COMMIT-PREPARATION, SOLVE-ALL-MERGE-CONFLICTS). None mentions OriginalWork, NativeDecodeWork or NativeEncodeWork.
- Migration design lives in `🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🔍️audits/native-value-resumption.md` (line 61). Intended end state, quoted: "live once in control/contract/work-action".
- Related but not end-state: `🌙️11/☀️01/.../🔍️audits/member-native-work-binding.md`, `member-history-work-repair.md`, `io-native-receiving-implementation.md`; `🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITOR-COMPLETION/🧪️native-edit-helper-reuse.md`; `🌙️08/☀️29/AI-MCP-END-TO-END/native-work-view-2026-11-06.md` (test snippets only).
- Commits: 4f4fbeea720 (892 files, 11-06 17:56), fbf4da6a5aa (2137 files, 11-06 20:18), 0b2d54d02c4 (536 files, 11-07 02:20). Their stat does not name the migration; it lists the affected value, job, io, dsl and plugin files.
- Plan and open items: no ticket text states the plan. The inferred plan is the 11/01 end state above. The 11-06 and 11-07 tickets the brief expected are absent, so intended end state is inferred, not sourced.

## 2. Current state of decode and encode (value)

- HEAD `🛬️decode/🎟️work/🦀️.rs` defines `NativeDecodeWorkIdentity`, `NativeDecodeWorkIssuer`, `NativeDecodeWorkLoan`, `NativeDecodeWorkRecipient` and a local `Work<'a>` with u64 fields.
- Working tree (uncommitted, `M`) removes those definitions and aliases `OriginalWork as Work` and `OriginalWorkState as WorkState` from the control contract. `🛬️decode/🦀️.rs` (also `M`) wraps `work::Work<'a>`.
- `🛫️encode/🎟️action/🦀️.rs` is unchanged and already aliases `OriginalWork as Action`, `OriginalWorkState as WorkState`. It is the committed model.
- No definition or `pub use` of `NativeDecodeWork*` or `NativeEncodeWork*` remains in code outside tests. Decode and encode tests and every consumer below still import the removed names.

## 3. Census (code only: .rs and .ts, tickets excluded)

Symbols (matching lines via `git grep -c`): NativeDecodeWorkRecipient 127, NativeDecodeWorkIssuer 56, NativeDecodeWorkIdentity 1, NativeEncodeWorkRecipient 63, NativeEncodeWorkIssuer 61, NativeDecodeWorkLoan 0, NativeEncodeWorkLoan 0.

Totals: 88 files, 291 lines. Production 33 files, 96 lines. Test 55 files, 195 lines.

Production, ranked by lines (module root, dependency layer approximated from path, not Cargo metadata):

| Rank | Module | Prod lines | Prod files | Note |
|---|---|---|---|---|
| 1 | dsl/schema encoding borrowed + decoding + producer | 41 | 7 | record.rs 19, tree.rs 9, token.rs 5, text-source.rs 3 (all encode-side, no new names) |
| 2 | job (receiving state, context, receiving work) | 16 | 6 | receiving/state 10 old only; context mixed |
| 3 | spr history | 10 | 2 | old only |
| 4 | io sqlite-snapshot + io testing | 9 | 4 | old only |
| 5 | plugin (os/plugin document, archive-load, maintenance) | 7 | 3 | plus energy 2 and fem 2 plugin snapshot files |
| 6 | store (composition, codec, space-history validation) | 3 | 3 | old only |
| 7 | ui presentation receive | 2 | 1 | old only |
| 8 | replication causal birth/original | 2 | 1 | old only |
| 9 | pack record-decode | 1 | 1 | mixed |
| 10 | value (decode slots, testing admission) | 5 | 2 | value itself: 5 of 96 |

Dependency order, lowest first: value (owner) → dsl schema, pack, replication, ui → io sqlite-snapshot → job → store → spr → plugin, energy, fem.

Tests carry 195 of 291 lines. Largest test users: value encode action (14), decode construction (12), decode work (11), encode effect (7), decode backing and retirement (5 each).

Reach above value: 91 of 96 production lines (95%) sit outside value. The migration is incomplete for every consumer above, and the working tree has already removed the names they import.

## 4. Half-landed on the same pattern

- Mixed old and new names in one file (half-landed): `job/⏱️context/🦀️.rs` (old 1, new 5), `job/⏱️context/🛬️decode/🦀️.rs` (1 vs 4), `job/⏱️context/🛫️encoding/🦀️.rs` (1 vs 3), `pack/📖️record-decode/🦀️.rs` (1 vs 2), `store/🧩️composition/🚪️open/🏭️operation/🛂️receiving/🦀️.rs` (1 vs 1).
- Old names only, not started in production: job `receiving/🎟️state` (10), spr `history` (9), io `sqlite-snapshot` paged-text and blob (4 and 3), dsl `borrowed` (encode side, 36), plugin `document` (1 and 1), energy and fem snapshot bridges.
- Of the 33 production files still using old names, 25 were touched by commits 4f4fbeea720, fbf4da6a5aa or 0b2d54d02c4. The commits moved the migration partway, not across.
- Store and plugin: both still import `semio_framework_value_native_owned::native_decoding::NativeDecodeWorkIssuer` or `...NativeDecodeWorkRecipient` (spr, store composition, plugin document). That path has no definition in the working tree.

## Open items

1. Value working tree is uncommitted: decode definitions removed, tests not updated.
2. Consumers (job, spr, store, plugin, io, dsl, ui, replication, pack, energy, fem) still import removed names.
3. Old encode-side names in dsl borrowed (36 lines) are not covered by any migration ticket I found.
4. Compile check not run (no repo runs in this lane); the claim that consumers fail to resolve is from source text.

## Files

- Census script: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/aud-migration-census3.py`
- Census JSON: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/aud-migration-census.json`
- Production old-name file list: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/aud-migration-prod-old-names.txt`
