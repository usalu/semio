# Ticket Important Blocker Closure

## Decision

The 34 fail-closed `📌️important.md` residuals are preserved without coercing ticket lifecycle state.

The active compulsory-action document remains exclusive to an exact ticket owner with an explicit `status: "open"` manifest:

```text
<ticket>/📌️important/📝️.md
```

Historical or owner-incomplete ticket-root notes instead project byte-for-byte to:

```text
<ticket>/📓️important/📝️.md
```

`📓️important` is a distinct, ticket-parent-scoped semantic directory kind. It records preserved historical information and is never consulted by create, finish, or reopen lifecycle code. No ticket status is inferred or rewritten by this projection.

## Exact Dispositions

| Cohort | Count | Disposition |
| --- | ---: | --- |
| Explicit `closed`, nonzero source | 21 | Project byte-for-byte to `📓️important/📝️.md` |
| Adjacent manifest with invalid/missing status, nonzero source | 1 | Project byte-for-byte to `📓️important/📝️.md`; retain the manifest unchanged |
| Ticket-root-shaped owner without adjacent manifest | 9 | Project byte-for-byte, including three zero-byte leaves, to `📓️important/📝️.md` |
| Empty nested phase residue beneath `INTERACTIVE-JOB-RUNTIME-REFACTOR` | 2 | Remove only through two exact zero-byte path-mutation authorities |
| Deep `presence` fixture placeholder | 1 | Parent-owned move to `👥️presence/📝️.md` |
| **Total** | **34** | No content loss and no status coercion |

The nine owner-root paths without a manifest are the paths whose immediate parent has the exact `ticket-slug` directory contract. The two phase residues and the deep fixture do not have that immediate owner and cannot inherit the historical-ticket projection.

A fresh immediate-owner census independently reproduces exactly 31 history projections, finds zero occupied `📓️important/📝️.md` destinations, and measures a maximum destination length of 152 UTF-8 bytes, below the 240-byte policy ceiling.

The first zsh-form census returned no output and is not evidence. The recorded result comes from the explicit Bash rerun with an `NF==8` immediate-owner filter.

## Authority Order

Inventory applies the following mutually exclusive order:

1. Resolve embedded ticket-root relocation.
2. Apply the strict ticket-manifest lifecycle authority: open projects to `📌️important`; closed-zero removes; closed-nonzero and invalid remain unavailable to the active lifecycle projection.
3. Apply `ticket-important-history-markdown-v1` only to an exact ticket-slug immediate parent and exact raw basename. It accepts only the residual states `closed-nonzero`, `invalid-manifest`, or `missing-manifest`; it never accepts `open` or `closed-zero`.
4. Apply the two exact zero-byte nested-phase removals.
5. Apply the exact presence-fixture parent-owned move.

The history projection records the source leaf preimage and, when present, the sibling manifest preimage and parsed state. Planning rejects an occupied destination, incoming reference that is not covered by a structured edit, changed source or manifest bytes, a symlink/nonregular source, or a source outside the exact immediate owner.

The closed FEM ticket currently has six governed old-name occurrences that must converge to its history destination: two structured `join(ticket, "📌️important.md")` calls, three prose markers across the two owning `.mjs` files, and the generated `📋️registrar-handoff.json` marker. The two script preimage hashes remain `e18760…` and `5dab8f…`. Edit the two sources first and regenerate/verify the registrar output; do not hand-author the generated JSON independently.

## Schema Shape

Add the parent-scoped semantic kind:

```json
"ticket-important-history": {
  "emoji": "📓️",
  "slugPattern": "^important$",
  "allowEmojiOnly": false,
  "parentKindIds": ["ticket-slug"],
  "inferWithoutEmoji": false,
  "projectionOnly": true
}
```

Add a closed tagged owner projection contract `ticket-important-history-markdown-v1`. Its source is exact `📌️important.md`, destination is exact `📓️important/📝️.md`, owner is exact `ticket-slug`, and its only admitted dispositions are the three historical residual states above. Generic semantic-stem inference must not use the kind.

The language-neutral seven-case authority matrix is frozen at:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️ticket-important-history-owner-authority/🔣️.json
```

`jq` independently validates schema version 1, seven unique cases, four projections, and three unclaimed boundary cases.

The portable ticket-local test `🧪️ticket-important-history-owner-authority.test.ts` validates the same matrix and rejects a forged active-document destination through independent Ajv JSON Schema parsing. Current result: `2 pass`, `0 fail`, `14 expect()` calls in 174 ms.

The two nested phase removals remain exact mutation-catalog cases with complete zero-byte regular-file preimages. The presence fixture is an exact mutation-catalog move. These three paths do not justify any broad rule.

## Transaction Acceptance

- Every projection is a normal move with byte/mode/size preimage and structured reference edits.
- The two removals require exact catalog authority and retained rollback backups.
- The sibling ticket manifest, when present, is included in affected pre-state but remains byte/mode identical.
- A failed or cancelled apply restores all 34 sources and every edited reference exactly.
- Post-apply inventory contains zero raw `📌️important.md` leaves in these cohorts.
- Open tickets contain only the active `📌️important/📝️.md` shape; historical notes contain only `📓️important/📝️.md`.
- A second plan is empty, and the temporary two-path removal catalog entries are deleted after convergence.

## Evidence Boundary

The live no-follow census is recorded in `📓️s-ticket-important-live-census.md`. This decision read only the named repository-ticket leaves and their immediate sibling manifests. It did not traverse or read actual `compose/**` or `temp/compose/**`, and it did not mutate Git state.
