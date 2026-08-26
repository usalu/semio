# Ticket Important Markdown Owner Implementation Packet

## Scope and evidence boundary

This is a read-only implementation packet for the retained ticket file named `📌️important.md`. No production, test, schema, Git, Compose, or temp-Compose state was changed. Actual `compose/**`, `temp/compose/**`, and `temp-compose/**` were neither traversed nor read.

The cohort and lifecycle evidence come from `📓️h-important-semantic-authority.md` and a narrow re-census of only its 460 named non-Compose paths. The retained pre-transaction-v2 inventory is evidence, not final v2 acceptance: 116,981,622 bytes, SHA-256 `f03a…`. A post-v2 inventory must confirm the same authority against the final inventory schema.

Stable cohort accounting:

| Cohort | Count | Disposition authority |
| --- | ---: | --- |
| Canonical ticket root, adjacent valid manifest, status `open` | 173 | Project to the canonical nested document |
| Canonical ticket root, adjacent valid manifest, status `closed`, zero-byte source | 243 | Remove as lifecycle evidence; do not create a destination |
| Canonical ticket root, adjacent valid manifest, status `closed`, nonempty source | 21 | Error; preserve source; do not apply any plan containing it |
| Canonical ticket root, adjacent manifest with missing/invalid status | 1 | Error; preserve source |
| Embedded one-level ticket root with adjacent valid manifest, status `open` | 10 | Block on owner relocation, then project |
| No adjacent ticket manifest | 12 | Unclaimed/unresolved; preserve source |
| **Total** | **460** | Zero double counting |

Content evidence is exact: 409 files are zero bytes, 51 are nonempty, and there are no whitespace-only files. The proposed rule is therefore byte-exact `zero-byte`, not `TrimSpace`. The one invalid manifest is:

` .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️05/FIX-ENGAGEMENT-SUGGESTION-CLICK/🎫️ticket.json `

It has no own `status` field. It must not acquire the Go reader's current implicit `open` default for projection authority.

## Frozen behavior

The canonical open-ticket document is:

```text
<ticket-root>/📌️important/📝️.md
```

The source-to-destination rule is exact:

```text
<ticket-root>/📌️important.md
  -> <ticket-root>/📌️important/📝️.md
```

Authority is conjunctive. A source is governed only when all of these are true:

1. Its parent resolves through the exact fixed-directory contract `ticket-slug`.
2. The same parent contains the exact fixed filename contract `ticket-manifest`.
3. The manifest is valid JSON and owns an explicit string `status` equal to `open` or `closed`.
4. The source basename is exactly `📌️important.md`, its file kind is `markdown`, and it is a regular file admitted by the inventory.
5. The destination is rendered only by the registered contract, never inferred from a free filename stem.

The lifecycle table is normative:

| Manifest state | Source bytes | Normalized outcome | Apply behavior |
| --- | --- | --- | --- |
| `open` | Any | `📌️important/📝️.md` | Move and rewrite every governed reference |
| `closed` | Exactly zero bytes | No destination | Remove with lifecycle authority and a frozen preimage |
| `closed` | Nonzero | Canonical destination may be reported for diagnostics/reference planning, but the inventory/plan has an error | Preserve everything; apply is forbidden |
| Missing or invalid status | Any | No destination | Preserve; emit an error |
| Missing/nonmatching owner or sibling manifest | Any | No owner projection | Preserve; retain ordinary unresolved classification |
| Embedded ticket owner | Any | No direct projection in the current planner | Preserve; emit `relocate-owner-first` until embedded-owner relocation can carry the same authority |

Whitespace-only content is nonzero and therefore blocks closing. This removes platform/runtime ambiguity from Unicode whitespace definitions.

## Go ticket owner changes

### Exact production owner

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go`

| Current region | Approximate lines | Required change |
| --- | ---: | --- |
| `TicketStatus`, `IsValid` | 9933–9955 | Keep the closed enum. Projection must reject missing/invalid status rather than use a default. |
| `Ticket.ImportantPath` | 11138–11159 | Retain the runtime field, but always populate the nested canonical path. |
| `Ticket.UnmarshalJSON` | 11186–11286 | Current legacy fallback at 11255–11268 converts a missing status to `open`. Remove that fallback if the Go owner becomes the strict manifest reader; at minimum, the projection resolver must parse the raw manifest independently and require an own valid status field. No compatibility form should be added. |
| Path helpers | 22316 onward | Change `GetImportantFilePath` at 22331–22335 to `filepath.Join(GetTicketPath(...), "📌️important", "📝️.md")`. |
| Ticket title update | around 22499 | Continue recomputing `ImportantPath` after owner-directory rename through the corrected helper. |
| `CreateTicket` | 22615–22652 | Create a zero-byte canonical leaf. Existing `WriteTextFile` at 17053–17060 already creates parent directories. Never create the old flat source. |
| `SaveTicket` / `ReadTicket` | 23320–23365 | Recompute the canonical path. Require an explicit valid status before lifecycle operations. |
| `FinishTicket` | 25311–25470 | Apply the same fail-closed important-file rule to bulk and non-bulk closes. Reject unreadable/nonzero content; remove the zero-byte leaf and then its now-empty `📌️important` directory; return removal failures rather than warning and continuing. |
| `ReopenTicket` | 25473–25577 | Recreate a missing zero-byte canonical leaf. Never overwrite an existing nonempty leaf. |
| Bulk close caller | 29200–29215 | Do not bypass important-file validation/removal when `bulk=true`. |
| Single close caller | 29233–29252 | Preserve the same owner behavior as bulk close. |

The owner must never delete a nonempty important file, and closing must remain atomic from the user's perspective: do not save `status: closed` if validation or removal fails. Reopening an already materialized nonempty document preserves it.

### Exact Go tests

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️component_test.go`

Add focused tests in the existing ticket lifecycle region:

- `TestTicketImportantPathCanonical`: helper output is exactly `📌️important/📝️.md` on the host separator, with no old flat leaf.
- `TestCreateTicketCreatesCanonicalImportantLeaf`: canonical leaf exists, is zero bytes, and the old path does not exist.
- `TestReadAndRenameTicketUseCanonicalImportantPath`: read and owner rename both recompute the nested path.
- `TestFinishTicketImportantLifecycle`: table cases for non-bulk and bulk zero-byte removal, nonempty preservation/error, unreadable preservation/error, and whitespace-only preservation/error. Assert no status transition on every error.
- `TestReopenTicketRestoresMissingCanonicalImportantLeaf`: reopen creates the missing zero-byte leaf and preserves an existing nonempty leaf.
- `TestTicketManifestRequiresExplicitValidStatus`: missing and unknown statuses fail instead of becoming `open`.

`setupTicketDir` at approximately 17595–17613 currently writes a manifest without `status`; update that fixture to carry `"status":"open"`. `TestFinishTicketPurgesOversizedArtifacts` around 17633 currently constructs a `Ticket` without `ImportantPath`; either populate it through the owner helper or separate unrelated artifact-purge behavior from the important lifecycle assertion.

Focused command owned by `@semio-tech/repo-client`:

```sh
bun nx run @semio-tech/repo-client:test -- -- -run=TestTicketImportant
```

The package script at `⌨️cli/📦️packages/🟦️typescript/📜️script.ts` already routes tests to `go test` with `-short`; no new permanent script is required.

## Schema-first projection authority

### Semantic directory kind

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, `semanticDirectoryKinds` near line 209.

Add one exact kind:

```json
"ticket-important": {
  "emoji": "📌️",
  "slugPattern": "^important$",
  "allowEmojiOnly": false,
  "inferWithoutEmoji": false,
  "projectionOnly": true
}
```

`projectionOnly` is required. The earlier census recommendation was incomplete without it: the normalization engine's current leading-emoji file-stem branch can match a semantic kind even when `inferWithoutEmoji` is false. Generic stem matching must exclude projection-only kinds, while canonical directory validation must still accept `📌️important` when rendered by the owner contract. This prevents the kind from authorizing unrelated `📌️*.md` leaves.

### Owner projection contract

Do not widen the current artifact-oriented `semanticPathProjectionContracts`. Add a separate strict registry, recommended name `semanticOwnedFileProjectionContracts`, with one closed tagged contract:

```json
"ticket-important-markdown-v1": {
  "contractKind": "owner-sibling-manifest-file",
  "ownerFixedDirectoryContractId": "ticket-slug",
  "requiredSiblingFixedFilenameContractId": "ticket-manifest",
  "manifestAdapter": "json",
  "manifestStatusLocation": "status",
  "allowedStatuses": ["closed", "open"],
  "sourceFileKindId": "markdown",
  "sourceFilename": "📌️important.md",
  "destinationDirectoryKindId": "ticket-important",
  "destinationDirectoryName": "📌️important",
  "destinationFilename": "📝️.md",
  "emptyContentRule": "zero-byte",
  "statusDispositions": {
    "open": "project",
    "closed-empty": "remove",
    "closed-nonempty": "problem",
    "invalid": "problem"
  },
  "rationaleRule": "ticket-important-markdown-projection-v1"
}
```

The JSON registry key, contract kind, adapter, empty-content rule, disposition keys/values, and rationale are closed enums. Validation must require NFC/nonempty strings, exact registry keys, resolvable fixed/directory/file-kind IDs, `sourceFilename`'s `.md` agreement with `sourceFileKindId`, exact destination `📝️.md`, and a destination directory name that resolves to `ticket-important`. Reject equal-specificity overlaps on the tuple `(owner contract, sibling manifest contract, source filename)`.

### Discovery types, validation, and pure authority

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`

Exact extension points:

- `SemanticDirectoryKindSpec` near lines 87–93: add required `projectionOnly: boolean` or, if migration of every kind is intentionally preferred, a required closed `stemInference` enum. The minimal packet uses `projectionOnly`, with explicit `false` on every existing kind.
- Projection contract types near lines 122–304: add the tagged owner-file contract separately from the artifact variants.
- `Taxonomy` near lines 447–471: require `semanticOwnedFileProjectionContracts`.
- `semanticDirectoryKindId` near lines 769–790: continue validating canonical directory names; do not use this resolver alone to authorize a projection-only file stem.
- Strict taxonomy validation near lines 1749–2063: validate the closed contract and projection-only relationships described above.

Expose a pure, filesystem-independent authority function, recommended name `semanticOwnedFileProjectionAuthority`. Its inputs are admitted source/owner/manifest facts plus source bytes or byte length; the filesystem adapter remains in normalization. Its result must include:

```ts
{
  contractId: "ticket-important-markdown-v1";
  ownerPath: string;
  manifestPath: string;
  manifestContentHash: string;
  sourcePath: string;
  destinationPath?: string;
  status?: "closed" | "open";
  contentState: "nonzero" | "zero-byte";
  disposition: "problem" | "project" | "remove" | "unclaimed";
  problems: readonly string[];
}
```

The function must require exact fixed-contract identities supplied by the caller, parse an own JSON `status` field with no default, and render only the registered destination. A counterfeit path that merely resembles a ticket or matches the basename must return `unclaimed`.

## Normalization parser and planner packet

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`

Exact extension points:

| Region | Approximate lines | Minimal change |
| --- | ---: | --- |
| Discovery imports | around 31 | Import the pure owner-file projection authority and its in-repo types. |
| `TaxonomyV7` and parser | 642–798 | Parse `projectionOnly` and the required owner-file projection registry with exact keys/types; no fallback for absent or legacy forms. |
| Projection parser | 1117–1174 | Keep artifact parsing intact; add a separate parser for the owner-file tagged contract. |
| Generic semantic stem resolution | 1789–1820 and `canonicalFile` 2263–2310 | Exclude projection-only kinds from generic file-stem inference. Accept `ticket-important` only through explicit authority. |
| Reference token adapters | 2626–2699 | Existing TypeScript/JavaScript string-token support handles the three `join(..., "📌️important.md")` forms. Add a closed prose-path form only if the two documented prose markers are meant to be rewritten; otherwise stale verification must intentionally block. |
| Reference edit construction | 3056–3164 | Build edits from authoritative old/new paths and require every governed incoming edge to be accounted. |
| Inventory construction | around 3765 onward | Run an owner projection prepass after admitted fixed owner/manifest facts exist and before generic leaf canonicalization. Set the authorized normalized path or lifecycle disposition without filesystem inference. |
| Embedded ticket roots | around 4233–4241 | Current relocation requires fixed leaves. Do not falsely label important Markdown fixed. Emit `relocate-owner-first` for the 10 embedded owners until embedded relocation carries semantic projection authority. |
| Move planning | 4364–4394 | Use rationale `ticket-important-markdown-projection-v1`; preserve collision/platform/path-budget checks. |
| Plan assembly | 4525–4582 | Materialize closed/zero-byte lifecycle removals before reference move authority and exclude removal sources from moves. |

For an open governed source, set `normalizedPath` to the canonical nested destination and suppress `semantic-stem-unresolved`. For closed/nonempty, expose the canonical destination for deterministic diagnostics and reference planning only if the plan remains unconditionally error-blocked; never allow apply. Missing/invalid/unclaimed inputs retain their source path and receive an exact error/unresolved code.

Recommended problem codes:

- `ticket-important-manifest-invalid`
- `ticket-important-closed-nonempty`
- `ticket-important-owner-relocation-required`
- `ticket-important-reference-rewrite-unresolved`

### Lifecycle removal authority

Reuse the existing staged evidence-removal transaction and rollback machinery rather than introduce a second deletion engine. Add one strict authority variant:

```ts
{
  kind: "ticket-lifecycle-empty-document";
  projectionContractId: "ticket-important-markdown-v1";
  ticketRoot: string;
  manifestPath: string;
  manifestContentHash: string;
  status: "closed";
  emptyContentRule: "zero-byte";
  authorityDigest: string;
}
```

The operation rationale is `closed-ticket-important-empty-v1`. The source path, content hash, mode, and size remain frozen in the operation preimage. Staging, apply, verification, journal resume, injected failure, and rollback must treat it exactly like other evidence removals. A changed manifest/status/source preimage makes the plan stale.

## The three structured live reference edits

All three sources are `.mjs` files and correctly use the existing `typescript` adapter. Offsets and hashes are for the current named source preimages and must be re-derived during inventory; they are golden evidence, not permission to skip preimage validation.

| Source | Structured location | Old value | New value | Current preimage SHA-256 |
| --- | --- | --- | --- | --- |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT/🔧️wave0-scaffold.mjs` | `typescript-path:256:17@9362` | `📌️important.md` | `📌️important/📝️.md` | `1408f1f061569e71372afeb5d72a2032e44cd336ba40aa7a66a340f75cc75b64` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🔧️write-handoff.mjs` | `typescript-path:84:17@3054` | `📌️important.md` | `📌️important/📝️.md` | `e18760f389273a8db1262c04e79a181e41a26adba2bbb753ac98c6a6d5c3c84b` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🔧️update-handoff-tests.mjs` | `typescript-path:37:17@1858` | `📌️important.md` | `📌️important/📝️.md` | `5dab8fd67e876c41b5ac411cf9b5f632bf301bef841a6f92f8e8eefddb971be5` |

The NATIVE-BREP target is open and nonempty, so its move/edit is executable when all other checks pass. The FEM target is closed and nonempty, so its two structured edits may be represented in a deterministic blocked plan, but apply must remain forbidden until the lifecycle contradiction is resolved.

There are also two raw prose occurrences that the current TypeScript tokenizer does not emit:

- `🔧️write-handoff.mjs`, line 114, column 58, byte offset 4402.
- `🔧️update-handoff-tests.mjs`, line 14, column 56, byte offset 963.

Therefore the repository contains three structured incoming edges but five raw old-name occurrences. This is the principal reference-coverage blocker. The implementation must choose a closed, structured prose-path grammar for these two exact contexts, or stale-output verification must block the projection. A blanket text replacement is not acceptable.

## Permanent language-neutral and TypeScript tests

Own a language-neutral golden fixture under the repo-lib fixture taxonomy, for example:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/
  🧫️fixtures/🧪️ticket-important-projection/🔣️.json
```

The JSON must encode exact source/owner/manifest/destination paths, all lifecycle dispositions, the three structured edits, and the two currently unsupported prose markers. It must not contain ticket-local absolute paths.

Add focused tests to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`:

1. Strict taxonomy accepts the exact semantic kind and contract.
2. Strict taxonomy rejects unknown keys, missing IDs, invalid NFC, filename/file-kind mismatch, non-projection-only destination kinds, and equal-specificity owner/source overlap.
3. Pure authority table covers open, closed/zero-byte, closed/nonempty, missing status, invalid status, missing manifest, counterfeit owner, and embedded owner.
4. An isolated committed Git fixture produces the exact open move and three structured reference locations where their targets are admitted.
5. Closed/zero-byte produces only the authorized removal; closed/nonempty blocks and preserves bytes.
6. Stale manifest/source/reference preimages fail before mutation.
7. Failure injection at every apply stage restores the old source/reference bytes; cancellation does likewise.
8. A successful second inventory/plan is empty.
9. Collision, platform name, and path-budget policy still apply to the rendered nested destination.
10. A negative proves arbitrary `📌️foo.md` cannot inherit `ticket-important` authority.
11. A negative proves the two prose occurrences are either governed by an exact prose adapter or reported stale/unresolved; they may not disappear from accounting.
12. Cohort accounting remains `460 = 438 + 10 + 12` and has no duplicate authority.

Use the existing `fast-glob` test dependency only to cross-check candidate enumeration. Glob parity is not owner authority: exact fixed-contract identity plus the sibling manifest remain required.

Focused command:

```sh
bun nx run @semio-tech/repo-lib:test -- --test-name-pattern=ticket-important
```

## Ownership and sequencing

| Packet | Exclusive production responsibility | Depends on |
| --- | --- | --- |
| Schema/discovery owner | `🔣️taxonomy.json`; discovery types, validation, pure authority; language-neutral authority fixture | Nothing beyond current v7 strict load |
| Go ticket owner | Canonical helper plus create/read/rename/close/reopen lifecycle and Go tests | Frozen canonical destination and zero-byte rule |
| Normalization owner | Strict parser, projection-only stem exclusion, prepass, move/removal/reference planning, apply/rollback verification, repo-lib normalization tests | Strict-green schema/discovery authority |
| Reference adapter owner, if separated | Exact prose-path token form and stale marker coverage | A frozen closed grammar for the two prose contexts |

Safe order:

1. Land schema/discovery and pure authority red-to-green.
2. Land Go owner canonical creation/lifecycle so new operations no longer recreate the old flat file.
3. Land normalization parsing/planning/apply with isolated fixtures.
4. Resolve or intentionally block the two prose references.
5. Resolve the 21 closed/nonempty owner contradictions and the 10 embedded-owner relocations transactionally; do not hand-move them.
6. Run post-v2 inventory and prove convergence.

## Acceptance checks and blockers

Acceptance requires all of the following:

- Open governed files normalize only to `📌️important/📝️.md`.
- Closed zero-byte files have an exact lifecycle removal with status/manifest/source preimages.
- Closed nonempty, invalid-status, missing-manifest, and embedded-owner inputs remain byte-preserved and error-blocked.
- Bulk close cannot bypass the lifecycle rule.
- Reopen restores the canonical zero-byte leaf.
- Generic semantic-stem inference cannot use `ticket-important`.
- All governed incoming references are structured and preimage-frozen; stale old-name markers after apply are zero.
- Apply failure/cancellation rolls back moves, removals, references, and owner directory creation.
- Second inventory and plan are empty for a completed fixture.
- Final v2 census has zero duplicate or unaccounted ticket-important entries.

Current blockers that must not be masked:

1. **Two prose markers are outside the current TypeScript token adapter.** They require an exact structured grammar or make stale verification fail.
2. **Twenty-one canonical tickets are closed with nonempty content.** These are lifecycle contradictions and cannot be automatically removed.
3. **Ten sources are owned by embedded one-level ticket roots.** Existing embedded relocation accepts fixed leaves only; important Markdown must remain a semantic projection and relocate-owner-first.
4. **One manifest lacks a status and twelve sources lack an adjacent manifest.** No owner authority exists for them.
5. **The Go manifest reader currently defaults missing status.** Projection authority must not consume that fallback; the clean owner implementation should remove it rather than preserve legacy behavior.
6. **The final transaction-v2 inventory has not yet revalidated the 460-entry cohort.** Retained pre-v2 counts are exact evidence but not final acceptance.

