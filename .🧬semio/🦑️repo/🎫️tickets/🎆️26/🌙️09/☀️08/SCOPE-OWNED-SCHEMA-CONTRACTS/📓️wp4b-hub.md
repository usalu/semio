# WP4b — `🌎️hub` external schema reads rewired onto scope modules

Partition: `🌎️hub/**`. Rows 26 and 50 of `📋️cross-partition-requests.md`.
Repo MCP was down for the whole session — no ticket tool was called, `🗑️generated/` was never deleted.

W4b was interrupted by a process restart partway through; its edits were already **staged** on disk
(`git diff --cached`, not `git diff` — the repo auto-commits, so an empty `git diff --stat -- 🌎️hub` is
not evidence of an empty partition). This report covers the whole of W4b: what the interrupted run had
landed, verified by re-running it, plus the work that was still missing.

## 1. Result

Every schema the hub script reads is now resolved through **one** catalog-backed draft-07 validator, with
four exceptions that are blocked on other partitions and listed in §4 with their exact expected targets.
The resolver is no longer duplicated: it lives once in `🌎️hub/📦️packages/🟦️typescript/🟦️.ts` and is
imported by both the router script and the hub integration test.

## 2. Rewired reads

`schema://<scope id>/<ExportId>` → `hubSchemaExport(repoRoot, uri)`. All of these previously read a
fixture-local or module-local `*.schema.json` (most already deleted by their owning partition) and
compiled it in a private `Ajv` / `Ajv2020` instance.

### 2a. Row 50 — the six `s.stdio.registry` reads (landed by the interrupted run, re-verified here)

| `📜️script.ts` site | Was | Now binds |
|---|---|---|
| `proveNativeOpenableCatalogProviderFixture` | `📇️registry/🧬️schema/🧬️native-codec-factories.schema.json` | `schema://s.stdio.registry/NativeCodecFactories` |
| ″ | `…/📇️native-catalog-surface/🧬️.schema.json` | `schema://s.stdio.registry/NativeCatalogSurface` |
| ″ | `…/📇️native-catalog-surface/🧬️commitment-cases.schema.json` | `schema://s.stdio.registry/NativeCatalogSurfaceCommitmentCases` |
| ″ | `…/📇️native-catalog-surface/🧬️imports.schema.json` | `schema://s.stdio.registry/NativeCatalogSurfaceImports` |
| ″ | `…/📇️native-catalog-surface/🧬️budget.schema.json` | `schema://s.stdio.registry/NativeCatalogSurfaceBudget` |
| ″ | `…/🧾️claim-authority/🧬️.schema.json` | `schema://s.stdio.registry/ClaimAuthority` |
| `proveNativeStdioCommitmentSchema` | absolute `…/📌️commitment.schema.json` | `schema://s.stdio.registry/NativeCatalogSurfaceCommitment` |
| `proveTrustedBootstrapCodecCaptureFixture` | `schemaPaths.stdio` | `schema://s.stdio.registry/NativeCodecFactories` |

`claimRoot` / `surfaceRoot` / `fixtureRoot` and the two reads of `📜️native-codec-factories.json` are
**data** paths and are unchanged, exactly as `📓️wp4-plugins.md` §6A requires.

### 2b. Row 26 — every other external schema (rows 1–7 landed by the interrupted run, 8–11 new here)

| # | `📜️script.ts` site | Was | Now binds |
|---|---|---|---|
| 1 | `browserDocumentOpenFixture` | `📇️directory/🧬️schema/🌐️browser-actor/🔣️.schema.json` + fixture wrapper | `schema://os.directory/BrowserDocumentOpenTransportV1`, `…/BrowserDocumentOpenTransportPlan` |
| 2 | `BrowserActorDocumentReservationCheckScript.run` | four `🧬️*-v1.schema.json` in `os/🧫️fixtures/📇️directory` | `schema://os.directory/{DocumentBrowserActorReservationV1, ExecutionTargetBodyReadV1, DocumentBrowserActorSessionV1, DirectoryArtifactBootstrapOwnerV1}` |
| 3 | `proveCheckpointPublicationCommandV1` (2 sites) | `📣️checkpoint-publication-command-v1/🧬️.schema.json` | `schema://os.directory/CheckpointPublicationCommandV1` |
| 4 | `proveSpaceArtifactCreationContractV1` | `🌱️space-artifact-creation-v1/🧬️.schema.json`, plus 3 `$defs`-splice compiles | `schema://os.directory/{SpaceArtifactCreationV1, DocumentOpenPlanV1, DocumentExecutionTargetLeaseFieldsV1, ArtifactFrontier}` |
| 5 | `proveDocumentBrowserActorIdentityFixture` | `🌱️value/🔁️codec/🧪️fixtures/🧬️.schema.json` | `schema://framework.value.codec/CodecFixture` |
| 6 | `proveMemoryBackendBackingFixture` | `🛢️db/🗄️storage/🧪️fixtures/🧮️memory-backing/🧬️.schema.json` | `schema://os.db.storage/MemoryBackingV1` |
| 7 | `proveNativeOpenableCatalogProviderFixture` | `🔌️plugin/🏗️builder/🧪️fixtures/📇️topic-contributions/🧬️.schema.json` | `schema://os.plugin.builder/TopicContributionsV1` |
| 8 | `proveDocumentBrowserActorIdentityFixture` | direct read of `📇️directory/🧬️schema/🔣️.json` + private `Ajv` | `schema://os.directory/{DocumentBrowserActorPlan, DocumentBrowserActorLease}` |
| 9 | `proveTrustedCompiledDependenciesFixture` | `🛂️manifest/🧬️schema/🔣️.json` `$defs` splice + `Ajv2020` | `schema://framework.manifest/ArtifactKindFormatsFixture` |
| 10 | space-administration page parity | direct read of `📇️directory/🧬️schema/🔣️.json` + `Ajv2020({strict:false})` | `schema://os.directory/DirectorySpaceAdministrationPageV1` |
| 11 | `🧪️tests/🤝️integration/🟦️.ts` × 5 | its own second module-path-keyed `hubSchemaExport` | `scopeExport(root, "<scope id>")` over the shared resolver |

Rows 8–10 also removed three now-dead local `Ajv2020` bindings and the top-level
`import Ajv2020 from "ajv/dist/2020"` from `📜️script.ts`, and the `Ajv2020` import from the test.

## 3. The shared Ajv change

### 3a. One resolver, in one place

The interrupted run had replaced `HUB_SCHEMA_SCOPES` (a hand-maintained ten-row scope-id → path table)
with a read of the generated catalog, but left it inside `📜️script.ts`, where the hub integration test
cannot reach it — `📜️script.ts` ends in an unguarded `await runBundleScriptMain(...)`, so importing it
runs `dev`. The test therefore carried a **second**, module-path-keyed `hubSchemaExport` with its own
per-module `Ajv`, and the moment `hub.artifact-authority.trusted-catalog` grew a cross-scope `$ref` that
second resolver broke (§5.1).

The whole region now lives in `🌎️hub/📦️packages/🟦️typescript/🟦️.ts` (`//#region 🧬️Scope-owned schema
resolution`) and is imported by `📜️script.ts` and by `🧪️tests/🤝️integration/🟦️.ts`. There is exactly one
`Ajv` instance for the process.

- `schemaCatalog(repoRoot)` reads `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`
  once and **fails loudly** if any `🌎️hub/**/🧬️schema` directory on disk is absent from it (walks the
  partition; no allowlist, no fallback to a hand-written table).
- `schemaModulePath` takes `scopes[<id>].path` + `formats["🔣️jsonschema"]`. No nearest-parent search, no
  glob, no fixture-local fallback.
- `schemaModuleId` asserts the draft-07 dialect and a `https://semio.tech/schema/` `$id` (and a
  `…/hub/` `$id` for `hub.*`), then recursively loads every dependency **before** `addSchema`, so a
  cross-scope `$ref` resolves at compile time instead of being restated.
- `hubSchemaExport` returns a plain `(value: unknown) => boolean`. It deliberately does **not** return
  Ajv's `ValidateFunction`: per CLAUDE.md no exported API may require a type from outside the codebase.
  That is why the test lost its `JSON.stringify(validate.errors)` assertion messages.

### 3b. Dependency discovery: catalog `dependsOn` ∪ the document's own `$ref`s

Row 26 asks for catalog `dependsOn` to feed the shared Ajv. It does — but `dependsOn` alone is not
sufficient today, so `schemaModuleId` loads the **union** of `dependsOn` and the foreign scopes the
document actually `$ref`s (`schemaReferencedScopes`, which walks the document and maps each
`https://semio.tech/schema/<scope path>/<facet>.json` back to its scope id per the §A grammar):

```
$ node -e '…' 🔣️schema-catalog.json
scopes: 2620 with non-empty dependsOn: 27
hub.artifact-authority.trusted-catalog dependsOn []   (it $refs os.directory)
hub.inference                          dependsOn []
framework.manifest                     dependsOn []
```

The catalog is generated, so it is stale between a schema edit and the next `schema generate`; the
document's own `$ref` set is the ground truth `dependsOn` is derived from. Loading both is not a
fallback — `dependsOn` may legitimately name a dependency that is not a JSON-Schema `$ref` — but if WP2
would rather the catalog be the sole authority, `schemaReferencedScopes` can be deleted the moment
`schema generate` populates `dependsOn` and is re-run on every schema edit. Flagged in §6.

### 3c. `TrustedBundleBrowserActorV1` restatement → `$ref`

`os.directory` is now draft-07 with `$id https://semio.tech/schema/os/directory/component.json`, so the
restated browser-actor identity fields in
`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json` were replaced by

```json
"allOf": [
  { "$ref": "https://semio.tech/schema/os/directory/component.json#/$defs/DocumentBrowserActorIdentity" },
  { "type": "object", "additionalProperties": false, "required": ["path", "byteLength"], "properties": { … } }
]
```

(the second arm re-lists the inherited property names with `{}` so `additionalProperties:false` still
closes the object, and adds the two bundle-only fields). This closes `📓️wp4-hub.md` §7.3.

### 3d. `UNCATALOGUED_SCHEMA_MODULES`

One entry: `s.stdio.registry` → `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🔣️.json`. The module exists
and is correct (draft-07, `$id https://semio.tech/schema/s/stdio/registry/schema.json`, 14 `$defs`), but
the catalog does not emit it — `✏️s/🔌️plugins/<p>/📇️registry` is not an eligible owner level in the
execution contract §A / `🔣️taxonomy.json`, so the generator emits `s.stdio` (plugin root) and 844
artifact/mutation scopes under it but not `s.stdio.registry`. The entry self-retires: declaring a scope
the catalog already carries is a hard error, never an override. See request §6.1.

## 4. Pending externals — still read outside the scope system

| `📜️script.ts` / test site | Reads | Blocker | Exact expected target |
|---|---|---|---|
| `proveTrustedBootstrapCodecCaptureFixture` (`:9170`) | `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧬️.schema.json` | no `🧬️schema/` module; the file is still 2020-12 with `$id semio.gis.native-codec-receipts/v1` | `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧬️schema/🔣️.json`, draft-07, `$id https://semio.tech/schema/s/gis/native-codecs/schema.json`, receipt corpus as a named `$defs` export → hub binds `schema://s.gis.native-codecs/<ExportId>` |
| `BrowserActorChildWorkerContainmentCheckScript.run` (`:5867`+) | `…/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🔣️.json` | module exists but `$id` is `https://semio.tech/schemas/os/browser-actor-child-v1.json` (`schemas` plural, no scope path, no facet) and it is **not** in the catalog; the contract lives at the document root, not in `$defs` | `$id https://semio.tech/schema/os/plugin/browser-bundle/child/schema.json`, scope `os.plugin.browser-bundle.child`, root contract moved to a PascalCase `$defs` export → `schema://os.plugin.browser-bundle.child/<ExportId>` |
| `proveGisChildBrowserActor` (`:6286`), `BrowserActorGisDescribeCheckScript.run` (`:6436`) | `…/🌐️browser-bundle/🧾️describe/🧬️schema/🔣️.json` | same defect; `$id https://semio.tech/schemas/os/browser-actor-describe-v1.json`, no `$defs` at all | `$id https://semio.tech/schema/os/plugin/browser-bundle/describe/schema.json`, scope `os.plugin.browser-bundle.describe`, root contract moved to a `$defs` export → `schema://os.plugin.browser-bundle.describe/<ExportId>` |
| `proveSpaceArtifactCreationContractV1` (`:14637`), test `:377` and `:724` | `os.directory` `DirectoryEventBody` / `DirectoryStreamMessage` | see §5.2 — the OpenAPI `discriminator` keyword | delete the five `discriminator` annotations from `📇️directory/🧬️schema/🔣️.json`; then `schema://os.directory/DirectoryEventBody` etc. bind directly and the branch splice and both `strict:false, discriminator:true` Ajv instances in the test go away |

`📜️script.ts:3585` and the three other reads of `📇️directory/🧬️schema/🔣️.json` are **source-text**
parity checks (`readFileSync(..., "utf8")` compared against the Rust/TS twins), not schema compiles, and
are deliberately untouched.

## 5. Four defects found on the way (two mine, two peer-caused)

### 5.1 The hub integration test could not resolve the new cross-scope `$ref` (fixed)

```
FAIL |os-hub-ts| ../../🧪️tests/🤝️integration/🟦️.ts > validates the neutral immutable trusted-catalog bundle with AJV and Node crypto
Error: can't resolve reference https://semio.tech/schema/os/directory/component.json#/$defs/DocumentBrowserActorIdentity
       from id https://semio.tech/schema/hub/artifact-authority/trusted-catalog/schema.json
```

Cause: the test's own duplicate resolver loaded one module per `Ajv`. Fixed by §3a (the test now imports
the shared resolver). This is the concrete argument for one resolver rather than two: the `$ref` change
in §3c was correct and still broke a test that no one had rewired.

### 5.2 `os.directory` carries five OpenAPI `discriminator` keywords (NOT fixed — not my partition)

Binding `schema://os.directory/DirectoryEventBody` fails:

```
error: strict mode: unknown keyword: "discriminator"
  at hubSchemaExport (…/📜️script.ts:209:30)
  at proveSpaceArtifactCreationContractV1 (…/📜️script.ts:14748:25)
```

`discriminator` is an OpenAPI keyword, not JSON Schema; Ajv in `strict: true` rejects it. Five sites in
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json`:

```
/$defs/DirectoryEventBody/discriminator        = {"propertyName":"kind"}
/$defs/DirectoryCommand/discriminator          = {"propertyName":"kind"}
/$defs/DirectoryCommandResultV1/discriminator  = {"propertyName":"kind"}
/$defs/AdminIntentV1/discriminator             = {"propertyName":"kind"}
/$defs/DirectoryStreamMessage/discriminator    = {"propertyName":"kind"}
```

All five are redundant: every branch of every one of those `oneOf`s already pins `kind` with a `const`.
The hub deliberately does **not** teach its validator the keyword — that would be a compatibility layer
that silently masks the defect. The one site that needs the export keeps a local draft-07 compile of the
named `document.indexed` branch behind an explicit `🚧️` docstring naming the blocker; the test's two
pre-existing `new Ajv({ strict: false, discriminator: true })` instances are left as they were. Request
in §6.2.

### 5.3 A peer's test-layout move broke a hub source-text assertion (fixed)

```
AssertionError: trusted catalog lacks the opened-root/no-link owner or exact laws
  at proveTrustedCatalogOpenedRootFixture (…/📜️script.ts:8472:3)
```

`proveTrustedCatalogOpenedRootFixture` asserts that the trusted-catalog module declares 14 named things
(production types plus three `#[cfg(test)]` law names). It read exactly two files —
`🔏️trusted-catalog/🦀️.rs` and `🛡️opened-root/🦀️.rs` — and the END-TO-END-TESTING-REFACTOR ticket has since
moved all three law names into `🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs`.

The **same** move hit a second hub assertion twenty minutes later, in the middle of the final verification
sweep: `proveSpaceArtifactCreationContractV1` reads
`✏️s/🔌️plugins/🪐️space/…/✳️any/✏️editor/🦀️.rs` for
`create_artifact_dialog_submission_preserves_the_exact_catalog_choice_in_the_host_relay`, which had just
moved to `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (`space-artifact-creation-check` passed at 19:52 and failed at
20:02; the editor file is mtime 19:51:25, `git status` ` M`).

Both readers are in this partition and the mover is not, so both were fixed here with one helper:
`moduleRustSource(root)` collects every `*.rs` under a module directory, so these assertions are about the
module rather than about a file path and survive further test-layout moves. Nothing was reverted in the
peer's tree. This is a general hazard for the whole ticket — any `readFileSync(…"🦀️.rs")` source-text law
in any partition is a landmine while the test-layout refactor is in flight.

### 5.4 A peer moved the stdio codec **data** file out of the schema module (fixed)

`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/📜️native-codec-factories.json` →
`✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json`, which is correct (contract §B: inside a
`🧬️schema/` module only the canonical five files). Two hub readers still pointed at the old path —
`captureTrustedBootstrapCodecsV1` (`readStableBuildFile`) and `proveTrustedBootstrapCodecCaptureFixture`
(`sourcePaths.stdio`) — and were repointed, together with the §7.6 probe. Both are data reads; the
`NativeCodecFactories` **schema** binding was unaffected.

## 6. Cross-partition requests

1. **WP2 — regenerate `🔣️schema-catalog.json` so it emits `s.stdio.registry`. SETTLED, PENDING A RUN.**
   The coordinator amended `📋️execution-contract.md` §B during this session to make `plugin-submodule`
   (`✏️s/🔌️plugins/*/<module>`, explicitly naming `🗄️stdio/📇️registry`) an eligible owner level, so the
   only thing left is a `schema generate` run: the catalog on disk (2620 scopes, generated 18:10) still
   predates the amendment and carries `s.stdio` but not `s.stdio.registry`. The hub's one
   `UNCATALOGUED_SCHEMA_MODULES` entry hard-errors the moment the regenerated catalog carries the scope,
   so it self-retires on that run with no further hub edit.
2. **W5 os (`os.directory`) — delete the five `discriminator` annotations** listed in §5.2. They make
   five exports uncompilable by the shared strict draft-07 validator and are semantically redundant.
3. **W5 os (`os.plugin.browser-bundle`) — two modules with non-conforming `$id`s**, see the §4 table
   rows 2 and 3: `https://semio.tech/schemas/os/browser-actor-{child,describe}-v1.json` does not match
   the §A grammar `https://semio.tech/schema/<scope path>/<facet>.json`, and neither module names its
   contract as a `$defs` export, so neither is catalogued and the hub cannot bind them.
4. **W6 plugins (gis) — `📇️native-codecs` has no scope module**, see §4 row 1. This is the last
   surviving `*.schema.json` read in the hub script.
5. **W6 plugins / W5 os — `🌱️artifact-document-id-v1/🧬️.schema.json` was deleted, two plugin scripts
   still read it.** Not a hub file, but it is what currently reds two of the commands in §7:
   - `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:54` (`proveGisNativeCodecReceipts`)
   - `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📜️script.ts:34` (`proveVcsNativeCodecReceipts`)

   Both also read a sibling `📇️native-codecs/🧬️.schema.json` on the line above with `Ajv2020`, so
   fixing request 4 and this one together retires both plugin readers.
6. **WP2 — catalog `dependsOn` is empty for 2593 of 2620 scopes.** See §3b. If the catalog is meant to
   be the sole dependency authority, `schema generate` must populate `dependsOn` from each document's
   cross-scope `$ref` set (and be re-run after schema edits); the hub can then drop
   `schemaReferencedScopes`.

## 7. Verification — real output

All runs `cd /Users/ueli/Documents/semio`, with
`SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/wp4b-hub` and `CARGO_TARGET_DIR=$SEMIO_TEST_ARTIFACT_DIR/cargo`
(the `headlessStdio*` guards require the Cargo target to be a private child of the artifact root, and the
artifact root to contain a `🗑️generated` segment).

### 7.1 `bun … gis-inference-ledger-oracle` — all nine hub oracles green, aborts in the **gis plugin's** script

```
inference-server-identity-oracle: cases=11 fields=5 composite-bounds=2 ajv+node=1
gis-inference-ledger-oracle: traces=9 hostile=13 identity-hostile=17 sqlite-integers=6 ajv+typescript=1 hashes=6 independent-bounds=1; no executor/route/approval claim
inference-wal-proof-oracle: traces=17 ownership=3 binding-hostile=2 protocol-envelope=1 node-sha256=1; committed-WAL runtime still required
inference-command-oracle: vectors=20 ajv=1 node+webcrypto-hash=2; no GIS execution authority
inference-approval-request-oracle: valid=1 hostile=14 byte-boundaries=2 ajv+node=1; no approval authority
inference-author-oracle: cases=16 accepted=1 ajv+sqlite=1; no retained grant or submit authority
inference-wal-chain-oracle: exact=14 hashing-ownership=3 retained-boundaries=2 ajv=1 crc-valid=14 blake3-known-answer=1; Rust replay and third-party blake3 parity pending
inference-catalog-projection-oracle: exact=12; no native provider or route authority
trusted-catalog-identity-oracle: exact=6 canonical-kind=1 descriptor-sha256=1 package-ref-blake3=distinct; no GIS provider activation

error: can't resolve reference #/$defs/field from id https://semio.tech/schema/s/gis/schema.json
      at async proveGisNativeCodecReceipts (…/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:49:26)
      at async run (…/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7827:92)
exit=1
```

The blocker moved during the session: it was an `ENOENT` on the deleted
`🌱️artifact-document-id-v1/🧬️.schema.json` earlier (request §6.5), and is now an unresolved `$ref` inside
the gis plugin's own module — W6 is mid-refactor there. Either way the whole stack is in
`✏️s/🔌️plugins/🌍️gis/**`; nothing in `🌎️hub` is on it, and every hub oracle in the command prints green
first.

### 7.2 `bun … native-openable-catalog-provider-check --oracle-only` — same external blocker, in the **vcs plugin's** script

`--oracle-only` runs `proveHeadlessStdioLaunchIsolation` → `proveHeadlessStdioMetadataCaptureContract` →
`proveVcsNativeCodecReceipts` → `proveVcsNativeProviderSelectionFixture` →
`proveNativeOpenableCatalogProviderFixture` (the row-50 function) and returns before any cargo stage.

```
headless-stdio-metadata-capture-oracle: private-commands=2 replacement-stable=1 replacement-during-capture-denied=1 second-dependency-root-denied=1 outside-target-denied=1
ENOENT: no such file or directory, open '…/🔌️plugin/🧪️fixtures/🌱️artifact-document-id-v1/🧬️.schema.json'
      at proveVcsNativeCodecReceipts (…/✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📜️script.ts:34:97)
      at async run (…/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5267:92)
exit=1
```

It aborts **one call before** `proveNativeOpenableCatalogProviderFixture`, so the row-50 bindings are not
reached by this command today. They are proved instead by §7.6 against the real fixture data.

### 7.3 Commands that do reach the rewired code — all green

```
$ bun … document-browser-actor-identity-check                                    exit=0
exact-integer-value-oracle: AJV=1 targets=8 raw=38 admitted=104 arithmetic=BigInt; native production parity is a separate exact group
document-browser-actor-identity: AJV=2 cases=49 projection=2 equality=10 ownership=9 projection-binding=4; metadata only, no catalog/activation claim
[DEBUG] verified parent dialect source:18 fields,3 digest substitutions,8 hostile rows,public plan+private exchange+socket equality; native authority unverified
document-open-plan-oracle: descriptor=1 catalog=2 receipt=1 independent-codecs=3 issuer=11 consume=9 negative=22 exchange-negative=5 redaction=1 activation=catalog-gated-issuer+exchange passed
document-open-plan-production-parity: codecs=3 rejected=17 exchange-rejected=5 passed
browser-document-open-oracle: ajv=1 paths=3 installed-target=1 scope-keys=2 authority=1 exchange=1 websocket=1 rust-worker-bypass=denied hostile=25 bound=65536 redaction=6 passed
execution-target-lease-oracle: ajv=1 positive=1 manifest-fields=49 byte-vectors=13 lifecycle=11 hostile=73 component-bytes=1024 descriptor-bytes=656 node+webcrypto-sha256=agree first-party-blake3=known-answer status=5 passed
trusted-browser-actor-catalog: scope-export=TrustedBundleBrowserActorV1 cases=26 bodies=8 raw-lengths=8 WebCrypto=1; metadata/framing oracle only, no native loader/activation claim
```

That command covers §2b rows 1, 2, 5, 8 and the §3c `$ref`.

```
$ bun … space-administration-check                                               exit=0
space-administration-oracle: AJV=2 vectors=4 cursors=8 hostiles=9 source-hostiles=9 component-schema=9 sha256=1 binding=1
space-administration-check: checks=41 phase=source

$ bun … checkpoint-publication-check                                             exit=0
checkpoint-publication-command-oracle: valid=2 rejected=12 ajv=1 typescript=1 sha256=2 actor-snapshot=1 final-writer-fence=1 durable-idempotency=1 process-route=1
checkpoint-publication-check: checks=22 phase=source
```

Covering §2b rows 3 and 10.

### 7.4 Hub vitest — green

```
$ cd 🌎️hub/📦️packages/🟦️typescript && bun ./📜️script.ts test quick
 RUN  v4.1.10 /Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript
 Test Files  1 passed (1)
      Tests  11 passed | 1 skipped (12)
   Duration  24.63s
exit=0
```

Before the §3a/§5.1 fix this was `1 failed | 10 passed | 1 skipped`.

### 7.5 The remaining three commands — green after two peer-caused repairs

All three initially aborted in `loadCatalogTaxonomy` before reaching any hub code: a peer was landing
`schemaScopeOwnerLevels` into `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` during
the run (status `MM`, mtime 57 s before the first attempt; the error text changed between two attempts
four minutes apart, from `semanticDescendantContracts[…]` to `schemaScopeOwnerLevels.…`). Once that
settled:

```
$ bun … space-artifact-creation-check                                            exit=0
[DEBUG] space artifact creation contract: cases=34 raw-json=14 relay=10 authority=6 responses=6 AJV=1 scope-exports=4 TypeScript=1 Pack=1 ready-only-coordinate=1 ordinary-bridge=3; runtime genesis remains a separate gate
[DEBUG] document index fixture: ordered-client=10 AJV=1 independent-node-SHA256=10; backend transactions not executed
[DEBUG] creation transaction oracle: scope-export=32; backend transactions not executed
[DEBUG] creation accepted recovery oracle: scope-export=3; real deadline/backend not executed
[DEBUG] creation cancellation oracle: scope-export=9; backend concurrency not executed
[DEBUG] creation operation: scope-export transitions=30 independent-node-SHA256=1; native reducer/backend not executed
[DEBUG] genesis frontier: TypeScript=8 AJV=1 open/lease=8 independent-node-SHA256=5; native factory/publication not executed

$ bun … trusted-stdio-gis-bundle-check --two-author-source                        exit=0
[DEBUG] GIS Map witness socket retirement: neutral=5 Bun=1 third-party-ws=1 exact-close-events=2; no Hub restart claim
GIS Map two-author composition fixture laws=16 hostile=10 current-coordinates=5; real mounted journey remains separately required

$ bun … trusted-catalog-opened-root-check                                         exit=0
trusted-catalog-opened-root: scope-exports=2 paths=15 denials=7 same-handle=1 source=14 startup=no-ambient-path; source contract only, native platform laws require --native
```

`trusted-catalog-opened-root-check` needed a repair first — see §5.3.

### 7.6 Every non-hub export binding, against the real fixture data

`wp4b-hub-external-exports.mjs` (in this ticket folder) compiles each binding through one shared
draft-07 Ajv seeded from the catalog + the bridge table, exactly as the script does, and validates the
committed fixture instance:

```
$ node .🧬semio/…/SCOPE-OWNED-SCHEMA-CONTRACTS/wp4b-hub-external-exports.mjs .
OK   os.directory/BrowserDocumentOpenTransportV1     <- …/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json
OK   os.directory/BrowserDocumentOpenTransportPlan   <- …/🌐️browser-document-open-v1.json
OK   os.directory/DocumentBrowserActorReservationV1  <- …/🧵️browser-actor-reservation-v1.json
OK   os.directory/ExecutionTargetBodyReadV1          <- …/🧵️execution-target-body-read-v1.json
OK   os.directory/DocumentBrowserActorSessionV1      <- …/🧵️browser-actor-session-v1.json
OK   os.directory/DirectoryArtifactBootstrapOwnerV1  <- …/🧵️artifact-bootstrap-owner-v1.json
OK   os.directory/CheckpointPublicationCommandV1     <- …/📣️checkpoint-publication-command-v1/🔣️.json
OK   os.db.storage/MemoryBackingV1                   <- …/🛢️db/🗄️storage/🧪️fixtures/🧮️memory-backing/🔣️.json
OK   os.plugin.builder/TopicContributionsV1          <- …/🔌️plugin/🏗️builder/🧪️fixtures/📇️topic-contributions/🔣️.json
OK   framework.value.codec/CodecFixture              <- …/🌱️value/🔁️codec/🧪️fixtures/🔣️.json
OK   s.stdio.registry/NativeCatalogSurface           <- …/📇️native-catalog-surface/🔣️.json
OK   s.stdio.registry/NativeCatalogSurfaceCommitmentCases <- …/📇️native-catalog-surface/🧪️commitment.json
OK   s.stdio.registry/NativeCatalogSurfaceImports    <- …/📇️native-catalog-surface/🧪️imports.json
OK   s.stdio.registry/NativeCatalogSurfaceBudget     <- …/📇️native-catalog-surface/🧪️budget.json
OK   s.stdio.registry/ClaimAuthority                 <- …/🧾️claim-authority/🔣️.json
OK   s.stdio.registry/NativeCodecFactories           <- …/📇️registry/🧬️schema/📜️native-codec-factories.json
OK   compile s.stdio.registry/NativeCatalogSurfaceCommitment
OK   compile os.directory/SpaceArtifactCreationV1
failures=0
```

This is what proves row 50 end-to-end while §7.2's command is blocked upstream.

### 7.7 `cargo check -p semio-hub --no-default-features --features sqlite --lib`

**Attempted twice; blocked both times by a live peer refactor of the os kernel / plugin tree, never by
`🌎️hub`.** `semio-hub`'s own lib was never reached — a dependency failed first, so this is *not* a claim
that the hub lib compiles today.

```
$ CARGO_TARGET_DIR=<scratchpad>/target-w4 RUSTC_WRAPPER="" \
    cargo check -p semio-hub --no-default-features --features sqlite --lib

attempt 1 (19:41–19:57)
error[E0599]: no method named `ensure_durable_group_idle` found for mutable reference `&mut SpaceHost<M>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18925:14
error[E0609]: no field `envelope` on type `&mut SpaceHost<M>`                        …/🏪️store/🦀️.rs:18926:14
error[E0599]: no method named `replace_backbone_retained` …                          …/🏪️store/🦀️.rs:18927:14
error[E0599]: no method named `pump` …                                               …/🏪️store/🦀️.rs:18928:14
error[E0599]: no method named `bump` …                                               …/🏪️store/🦀️.rs:18929:14
error: could not compile `semio-framework-os-kernel` (lib) due to 5 previous errors

attempt 2 (19:58–…, after the peer fixed all five of the above)
error[E0252]: the name `FromValue` is defined multiple times   …/🔌️plugin/../../📡️backbone/🔗️binding/🦀️.rs:2:36
error[E0252]: the name `ToValue` is defined multiple times     …/🔌️plugin/../../📡️backbone/🔗️binding/🦀️.rs:2:47
error[E0599]: no method named `tick_backbone_reports` found for struct `ArtifactStore<P, Mutation>`
                                                              …/🔌️plugin/../../🦀️.rs:22345:38
error[E0308]: mismatched types                                …/🔌️plugin/../../📡️backbone/🔗️binding/🦀️.rs:113:58
error: could not compile `semio-framework-plugin` (lib) due to 4 previous errors
```

Attribution, not assumption: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` was mtime
`19:54:32` when attempt 1 failed at `19:57` and `19:58:04` when attempt 2 started, with
`git status --short` reporting ` M` (unstaged) throughout — a peer is renaming `SpaceHost`'s
backbone/pump API right now, and the failure moved crate between the two attempts
(`semio-framework-os-kernel` → `semio-framework-plugin`) as they rolled forward. Neither run mentions
`🌎️hub` at all (`grep -c 🌎️hub <log>` → `0`).

This ticket changed **no Rust in this partition** — every edit here is TypeScript (`📜️script.ts`,
`📦️packages/🟦️typescript/🟦️.ts`, `🧪️tests/🤝️integration/🟦️.ts`) plus one JSON Schema
(`🔏️trusted-catalog/🧬️schema/🔣️.json`, §3c), which no Rust `include_str!`s. The Rust that `📓️wp4-hub.md`
§6.7 reported green (`lib: 0 errors, lib test: 0 errors`) is untouched. Re-run once the os `SpaceHost` /
backbone-binding refactor lands:

```
CARGO_TARGET_DIR=<private> RUSTC_WRAPPER="" cargo check -p semio-hub --no-default-features --features sqlite --lib
```

### 7.8 Final sweep — everything re-run after the last edit

```
document-browser-actor-identity-check              exit=0
space-administration-check                         exit=0
checkpoint-publication-check                       exit=0
space-artifact-creation-check                      exit=0
trusted-catalog-opened-root-check                  exit=0
trusted-stdio-gis-bundle-check --two-author-source exit=0
external-export probe                              failures=0
hub vitest (bun ./📜️script.ts test quick)          exit=0  Tests 11 passed | 1 skipped (12), 20.00s
```

`gis-inference-ledger-oracle` and `native-openable-catalog-provider-check` remain red on the two plugin
scripts in §7.1/§7.2 (request §6.5). The hub vitest is close to its 30 s `quick` budget on a loaded
machine — one run in this session was killed at 30 s and passed in 20 s on the retry; if that keeps
happening the target belongs at `long`, flagged for whoever owns the budget.

## 8. Files changed

- `🌎️hub/📦️packages/🟦️typescript/🟦️.ts` — new `//#region 🧬️Scope-owned schema resolution`: the
  catalog read, the shared `Ajv`, `hubSchemaExport`. (+117)
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — resolver moved out and imported; §2b rows 8–10 rewired; three
  dead local `Ajv2020` bindings and the top-level `ajv/dist/2020` import removed. (interrupted run: §2a,
  §2b rows 1–7, `HUB_SCHEMA_SCOPES` → catalog read)
- `🌎️hub/🧪️tests/🤝️integration/🟦️.ts` — duplicate resolver deleted, five call sites moved to scope ids,
  `ajv/dist/2020` import removed, `.errors` assertion messages replaced (the shared resolver returns a
  plain predicate on purpose, §3a).
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json` — §3c `$ref` (interrupted run).
- `<ticket>/wp4b-hub-external-exports.mjs` — the §7.6 probe (input file, kept).

Also repaired in `📜️script.ts` (peer-caused, both in this partition, §5.3/§5.4):
`moduleRustSource()` replaces the hardcoded Rust file reads in `proveTrustedCatalogOpenedRootFixture` and
`proveSpaceArtifactCreationContractV1`; the two `📜️native-codec-factories.json` **data** reads follow the
file out of the `🧬️schema/` module.

`🗑️generated/wp4b-hub/` holds this run's command artifacts and its private Cargo target; it was created
by these runs and was **not** deleted.

## 9. Open questions

1. **Should `schemaReferencedScopes` survive?** See §3b/§6.6. It is the only piece of the resolver that
   does not take the catalog at its word, and it exists solely because `dependsOn` is empty for 99 % of
   scopes. If WP2 makes the catalog authoritative it should be deleted, not kept "just in case".
2. **`UNCATALOGUED_SCHEMA_MODULES` is a bridge table**, which the contract's "no fallbacks" rule would
   normally forbid. It is here because the hub cannot regenerate a catalog whose generator it does not
   own, and it hard-errors the moment the scope is catalogued. If the coordinator prefers, the hub can
   instead fail loudly on `s.stdio.registry` and leave `native-openable-catalog-provider-check` red
   until request §6.1 lands — say which.
3. **`os.directory` `$id` facet is `component`, not `schema`** (`…/os/directory/component.json`). The
   §A grammar allows any facet filename, and the scope id derives correctly (`os.directory`), so nothing
   is broken — but the five other os modules the hub touches use `schema`. Worth settling whether
   `component` is a facet the taxonomy blesses.
