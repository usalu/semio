# Protected Sweep Registration Independent Review

Exact preimage comparison:

```json
[
  {
    "path": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY/📥️isolated-verification/📜️script.ts",
    "diff": [
      "--- \n",
      "+++ \n",
      "@@ -236,6 +236,12 @@\n",
      "   const ledger = JSON.parse(readFileSync(join(ticket, \"📥️oct8-runtime-store-corpus-ledger.json\"), \"utf8\"));",
      "   const paths = [...ledger.updated, ...(ledger.tests ?? [])].filter((path: string) => !path.includes(\"📦️native-codec-send\") && !path.includes(\"♻️snapshot-read-retirement\") && !path.includes(\"👁️group-visibility\"));",
      "   await runBudgetedTestCommand(process.execPath, [\"test\", ...paths.map((path: string) => join(root, path))], { cwd: root, env: { ...repositoryEnv, SEMIO_TICKET_DIR: ticket }, budgetMs: 300_000, throwOnFailure: true });",
      "+  process.exit(0);",
      "+}",
      "+if (command === \"plugin-oct8-protected-sweep-tests\") {",
      "+  const { testProtectedActorPublicationV1 } = await import(join(root, \"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🧾️publication/🧪️tests/🟦️.ts\"));",
      "+  await testProtectedActorPublicationV1(root);",
      "+  console.log(\"[DEBUG] actual protected actor publication neutral join/refusal laws completed\");",
      "   process.exit(0);",
      " }",
      " if (command === \"plugin-oct8-hub-provenance-tests\") {"
    ]
  },
  {
    "path": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY/📥️isolated-verification/project.json",
    "oldConfigUnchanged": true,
    "target": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun ./📜️script.ts plugin-oct8-protected-sweep-tests"
      }
    }
  },
  {
    "path": ".vscode/launch.json",
    "oldEntriesUnchanged": true,
    "added": 1,
    "otherTopLevelUnchanged": true
  },
  {
    "path": ".vscode/🧩️launch.seed.jsonc",
    "oldEntriesUnchanged": true,
    "added": 1,
    "otherTopLevelUnchanged": true
  }
]
```

Live/seed old entries and remaining top-level configuration preserve exact parsed values; target only calls existing ticket script. Script diff adds only intended helper dispatch, no comments. New canonical publication schema defines variable per-byte claim/protected response/child execution/publication observation. Four responses are the actual four protected protocol asset operations, not four fixed cases. No fixed inputs/expected case table or separate corpus authority found. Actual authentication/child observation must still be produced, not inferred from const success fields. No test/HTTP/jobs executed. Final derive is required after owner settles this genuine leaf;3539 prior currentness is historical for the new scope.


## Current Join Helper Source Review

joinProtectedActorResponsesV1 reuses actual domain DocumentOpenIntentV1 and DocumentExecutionTargetLeaseFieldsV1 parsers. It requires exactly one of each four protected assets; compares POST/status/origin, forbids query/hash/URL credentials, checks encoded path scope against intent and same full intent across responses. It binds selected generation/package, component/descriptor/actor SHA and lengths, actor original component/descriptor/policy/schema/import interfaces, and actual component BLAKE3. These are meaningful byte/protocol joins rather than schema-only success admission.

At this source snapshot authentication is a supplied boolean and the current main sweep has not yet integrated this helper or exact child execution observer. Final production route must derive authentication from the actual private session/request, observe real child describe/detach/transfer/cancel/capacity, and retain outer current-pointer/input rechecks. Constants in the evidence schema do not establish execution. Exact descriptor domain parse/describe agreement belongs to actual child dataflow, not just SHA equality. Plugin was notified. Current schema still showed the pre-repair facet-less ID/human title; Root's requested canonical eligibility repair remains pending. No execution occurred.


## Current Capture and Child Source, Neutral Receipt

Current schema component ID and Pascal root title repaired. Actual current schema GREEN log545bytes SHA256 f16be86f8d5ef643ffa4cc25226dea42b78e023b9b1671149c67a87ec30ce66a reports one joined byte chain, nine exact refusals (unauthenticated, GET, foreign origin, wrong scope, illegal caller package, corrupt bytes, wrong generation, actor original-source mismatch, duplicate asset), four genuine Ajv response leaves and three independent WebCrypto SHA comparisons; Nx success296ms. Earlier GREEN409ms is a separate545-byte receipt SHA1a96484b8a0836d5be2fbf683399fd53e29b4e7de3fc273f36863cfbd32a199a. The available initial RED is a missing-module setup failure57ms; exact meaningful b52ffd RED path requested from owner and not inferred from this log.

Current network capture derives authenticated from actual request Bearer header and200 response, captures protocol scope/intents and byte bodies through owned interfaces (no Playwright public types). Child execution loads the captured actor through existing API, verifies actual source detachment and load/activation stages, executes describe and compares its real Pack/domain descriptor, observes one transfer, aborts and checks phase/capacity/post-cancel refusal, cleans buffers finally. Current source therefore replaces caller-only evidence booleans with measured operation checks when actually invoked.

Two remaining source bounds were sent to Plugin: childUrl is supplied and imported without local same-origin/known-owner enforcement; final caller must bind exact existing child owner. response.body() allocates before actual size comparison, so Content-Length bounds declared reservation rather than the actual allocation. No production/browser execution is inferred, and explicit publication-neutral remains a separate owner from normal Dev6 acquisition. Outer actual publication pointer/input recheck must be retained in final sweep.


## Current Browser Safety Repair Source

Current capture awaits the original response finished result, refuses compression, and reads independently observed request.sizes responseBodySize before body(). Unknown/unequal/oversized metadata refuses before application allocation; it then checks copied original body length. This captures the original response rather than issuing a replacement fetch. Browser/network may already buffer bytes, so this is bounded application copy, not a claim of zero browser buffering. Current capture also binds wanted selected manifest intent before retaining other assets.

Child URL now equals protectedActorChildUrlV1 for the fixed repository child source through existing Vite /@fs route, and the page itself refuses any different origin before import. Path construction normalizes Windows separators to slashes and supports absolute native/Windows inputs; no actual Windows execution claimed. Exact child/descriptor byte and cancellation checks remain. Earlier arbitrary URL and application-copy seams are closed in current source; actual neutral Chromium receipt remains owner pending.
