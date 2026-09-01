# Mutation Ticket Role Routing 64

## Canonical Test-Only Triplet

Added the approved, unmounted canonical role-routing triplet:

- [schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️ticket-role-routing/🧬️schema/🔣️.json)
- [vectors](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️ticket-role-routing/🔣️.json)
- [test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️ticket-role-routing/🟦️.ts)

It contains five closed supplied-fact cases: output-only ignored, output tracked, output nonignored-untracked, output declared-generator, and direct explicit input. The three positive output origins remain eligible facts; the test does not claim a collector or census observed them. The ignored-only case has no independent origin and establishes only that output routing cannot manufacture `explicit-ticket`.

Every vector field is schema-required, iterated, and compared. The test checks top-level, case, expected-output, and candidate missing-field negatives, unknown case fields, unique IDs, and the retained `error` value as part of full reference equality.

## Independent Reference and Subject

The owned test helper uses test-only `lodash/pick.js` and `lodash/mapKeys.js` to select and rename supplied option facts before independently projecting the closed expected option role. It does not use Ajv output as a semantic oracle.

The isolated subject child imports the real N export family, installs only an `inventoryTaxonomySources` sentinel, reimports N and requires the exact sentinel function identity before importing S, and terminates within ten seconds. It captures the complete child stdout/stderr through `[DEBUG]` output. The child reaches the sentinel for every case; it never falls back to a collector. Root and N hashes are compared before/after the child.

Lodash resolved as `4.18.1` from the lockfile, owned transitively by `@textlint/linter-formatter@15.8.0` (`lodash ^4.18.1`). It was already imported only in library test sources. No package manifest, lockfile, package entry, production, S, N, or P file was changed; root owns making that test dependency direct if/when it mounts the test.

## Executed Result

The first direct Bun attempt used a filename filter without `./`; Bun searched but executed no tests. It is not a feature result.

The executed command was:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️ticket-role-routing/🟦️.ts'
```

After root released S at `c539f565831cce420d5c755f2efe54362956d0f0d769a29ea3805a2e9f62d005`, the exact direct test passed `2/2` with `109` assertions. The sentinel stdout recorded four mutation-CLI calls with no N `ticketDir` and one direct call with `tickets/AUTHORED`; stderr was empty. This proves only option forwarding at the mocked boundary, not output publication or source membership.

The separate retained current-source RED remains [role-result-PUOuo3](../🧫️role-result-PUOuo3/receipt.json). The raw original ticket vectors remain untouched in [mutation-ticket-role-split-64](../🧪️mutation-ticket-role-split-64/🔣️vectors.json).

## Schema Negative Correction

The earlier `109`-assertion green is historical but insufficient: each case-level negative replaced the full vector roster with one case, so the schema’s `cases.minItems: 5` failure could mask the intended required/additional-properties assertion. That receipt was not overwritten.

The repaired test retains the full five-case roster for every negative, replaces only the target case, and requires the corresponding Ajv `instancePath`, `keyword`, and `missingProperty` where applicable. It also replaces the optional `nTicketDir` cast with a runtime-validated owned mapped result from Lodash `pick`/`mapKeys`.

The repaired sources hash as follows:

- Test: `5ce7bed464fc6a1aaba4438d9b36505919b80d9739fc01971cebebd51d8d9d5c`
- Schema: `bb50dce668f680d7cc8eb30885cdef1794d6f5a33f0c3f30c238b30690f3f6a0`
- Vectors: `7ffb8158c3d358df4aac73f9e98e4a1b5a8c216552dd49a37d37bd12853d20e2`

The exact corrected command was the same scoped Nx/Bun command above. It passed `2/2` tests with `199` assertions. Full retained output follows:

```text
bun test v1.3.14 (0d9b296a)

🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️ticket-role-routing/🟦️.ts:
(pass) mutation ticket role routing vectors are closed and every field participates [33.08ms]
[DEBUG] taxonomy ticket role routing child stdout="__MUTATION_TICKET_ROLE_ROUTING__{\"calls\":[{\"id\":\"output-ignored-only-never-becomes-explicit\",\"options\":{\"repoRoot\":\"/virtual/workspace\"}},{\"id\":\"output-tracked-candidate-remains-independent\",\"options\":{\"repoRoot\":\"/virtual/workspace\"}},{\"id\":\"output-nonignored-untracked-candidate-remains-independent\",\"options\":{\"repoRoot\":\"/virtual/workspace\"}},{\"id\":\"output-declared-generator-candidate-remains-independent\",\"options\":{\"repoRoot\":\"/virtual/workspace\"}},{\"id\":\"direct-explicit-ticket-is-the-only-admission-input\",\"options\":{\"repoRoot\":\"/virtual/workspace\",\"ticketDir\":\"/virtual/workspace/tickets/AUTHORED\"}}],\"exportCount\":22}\n" stderr=""
(pass) mutation ticket role routing reaches only the mocked N admission boundary [131.32ms]

 2 pass
 0 fail
 199 expect() calls
Ran 2 tests across 1 file. [238.00ms]
```
