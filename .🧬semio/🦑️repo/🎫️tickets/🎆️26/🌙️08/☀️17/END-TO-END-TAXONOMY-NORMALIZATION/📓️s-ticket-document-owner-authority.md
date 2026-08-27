# Ticket Document Owner Authority

## Exact physical cohort

The owner-directory probe for ticket-important convergence exposed a separate raw `ticket.md` leaf. A read-only physical census found 907 such files, all directly beneath a canonical year/month/day/ticket-slug owner. No nested fixture or arbitrary basename lookalike is included.

All 907 sources are regular files. Their exact owner-local destination is:

```text
<ticket-owner>/ticket.md -> <ticket-owner>/📝️.md
```

The parent already supplies the ticket semantics, so no `ticket` child directory should be invented. The source name is a repository convention, not an unconfigurable external-tool filename. The `🎫️ticket.json` manifest is a separate fixed contract and is unchanged.

| Property | Result |
|---|---:|
| Physical source leaves | 907 |
| Sibling manifests present | 853 |
| Closed manifests | 852 |
| Open manifests | 1 |
| Missing sibling manifests | 54 |
| Invalid sibling statuses | 0 |
| Occupied destinations | 0 |
| Exact/NFC/case/VS16-fold collisions | 0 |
| Maximum destination UTF-8 bytes | 150 |

The source/destination/mode/size/content/status ledger SHA-256 is:

```text
64731aebba268aa438438c3530ce992645ee3dafcd28f9c13260be0396309a73
```

The 54 missing-manifest cases are historical ticket-owned Markdown evidence. Their bytes and owner identities must be preserved; no manifest or status should be fabricated to permit the projection.

## Consumer census

A NUL-delimited Git-index-plus-untracked admission census excluded `compose`, `temp/compose`, and `.🧬semio` before any physical read, then used no-follow regular-file checks. It scanned 45,632 admitted physical leaves. Only four files outside historical ticket storage contain `ticket.md`; all four are retained Cursor plan documents:

- `.cursor/plans/live_subscription_field_tree_3bc375ea.plan.md:27`;
- `.cursor/plans/single_subscription_endpoint_9d8dfb86.plan.md:27` and `:185`;
- `.cursor/plans/strict-read-write-hooks_b877f378.plan.md:18`;
- `.cursor/plans/subscription_tree_mirrors_mutation_270e731b.plan.md:21` and `:145`.

Three occurrences are generic historical instructions; the other three name an obsolete `.repo/🎫️/...` path. None resolves to one of the exact current 907 source paths, and none is a live runtime/generator consumer. The repo product and root router contain no `ticket.md` token. Preserve these historical plans rather than rewriting prior execution evidence.

## Required integration

Add a distinct exact ticket-owner physical-leaf projection contract, not a global basename exception or a Markdown suffix allowlist. It must require the existing `ticket-slug` owner authority and the exact source filename `ticket.md`, render the primary Markdown kind-only destination, and work with or without an optional sibling manifest. Lifecycle status is evidence only; this document is not the active-important marker and must not inherit its closed-empty deletion rule.

The integration must include discovery/normalizer schema validation, deterministic plan rationale and preimages, isolated rollback/commit/empty-replan tests, counterfeit-owner rejection, destination-collision rejection, and source drift rejection. Only after that may a fresh production plan include these leaves.

This packet is read-only. No production file, schema, script, Git state, actual Compose tree, or ticket manifest was changed. No physical convergence is claimed.

## TDD Integration

The authority packet above was read-only. The subsequent implementation lane added a seven-vector language-neutral fixture and an independent Ajv claim oracle, then ran the new ticket-local test before implementation. It first failed because the pure resolver export was absent. After the resolver/interface was added in the discovery owner region, the test reached its intended schema red state: 0 passed, 2 failed, 3 assertions; the exact `ticket-document-primary-markdown-v1` contract was still absent. No source-leaf move or manifest change occurred.

After the README/LICENSE owner released the distinct parser branches, the schema row, discovery validator, normalizer parser, physical projection, and plan rationale were integrated. The contract is exactly `ticket-document-primary-markdown-v1` with `contractKind: owner-primary-file`, existing `ticket-slug` ownership, exact `ticket.md` source, Markdown file kind, and the primary `📝️.md` destination. No lifecycle condition or deletion rule is introduced.

The first transaction test run exposed that an exact-leaf scope does not inventory an occupied destination sibling. The owner projection now checks no-follow ancestry and direct physical destination siblings, including NFC/case/VS16 folds, before permitting a clean plan. It does not widen a same-parent leaf projection into unrelated ancestor siblings. A production probe found an existing `🎆️️26` year alias alongside canonical `🎆️26`; the exact leaf operation neither creates nor changes either ancestor. A regression preserves this scoped boundary while still rejecting folded destination leaves.

The coherent root rerun was:

```text
bun test --timeout 120000 './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️ticket-document-owner-authority.test.ts' './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️ticket-important-owner-authority.test.ts' './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️ticket-important-history-owner-authority.test.ts' './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️ticket-important-exact-mutations.test.ts'
15 pass, 0 fail, 176 assertions, 14.79 s
```

The new five-test packet proves the seven language-neutral claim vectors against Ajv, strict schema rejection, closed-empty/open/missing-manifest byte preservation, injected rollback, same-ticket retry, empty replan, exact/folded occupancy rejection, source drift rejection, and physical counterfeit-owner rejection. All temporary debug logging was removed. The existing active/history/exact ticket-important tests remain green with the new four-contract registry.

After the direct-sibling scope refinement, the new packet was rerun independently: 5 passed, 0 failed, 89 assertions, 21.75 seconds. A fresh production owner-scope plan for `FIX-CODEBASE-SECTIONS-AND-TICKETS` then exited zero with one document move, one closed-empty important removal, no other operations, no unresolved decisions, and digest `22acd5429721c8f785fee55069c90cd9d78d6bd9274641c6d1260b08360ff7a0`. This was read-only and no apply was run.

Changed files are the taxonomy, discovery, normalization, the permanent language-neutral `🧪️ticket-document-owner-authority/🔣️.json` fixture, the new ticket-local test, and the two existing ticket-important exact-registry assertions. No production ticket document or manifest has yet been moved.

## Broader Date-Scope Probe

A subsequent read-only plan for `🎆️26/🌙️01/☀️29` admitted 107 entries and proposed 29 moves and 10 removals, with no authored edits or regenerations. It remained blocked by nine unresolved decisions; digest `957de017251cae71ff428f575f5a3785418a9b4f633bb43d29d48bd30c0fbe18`, source-tree digest `24a50450d2658b4a7f4415f950ceef987c63a0d8b733f589643267eb6544bbf6`.

The residuals are an unresolved `test-fix.js` semantic stem under `FIX-VSCODE-EXTENSION-DIAGNOSTICS-DISPLAY`, an unresolved `migrate_tickets.py` stem under `MIGRATE-TICKETS-TO-NEW-FORMAT`, and seven unsupported historical `ticket.md` references in that migration ticket's manifest, Python script, and document. These are retained historical evidence, not authority to rewrite or delete the old migration script. The earlier consumer census intentionally excluded ticket storage and therefore did not cover these self-references. The entire date scope is not clean and must not be applied. A clean exact owner scope may proceed independently only after a fresh ticket-local plan and transaction gate.
