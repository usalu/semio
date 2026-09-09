# Artifact Creation Catalog Generation

Artifact creation now retains the exact selected catalog generation from the Shell request through the worker-owned HTTP saga. The worker wire requires a nonzero lowercase 64-hex `expectedCatalogGenerationId`; every public creation status requires a nonzero lowercase 64-hex `catalogGenerationId`.

The worker includes the selected generation in the canonical create body, every locally synthesized status, duplicate-owner equality, and retained operation state. A Hub status is projected only when its request, Space, catalog generation, and ready kind all match the retained owner. Missing and rotated status generations are ignored and polling continues until an exact status or the existing deadline settles the operation.

The language-neutral fixture and Draft 7 schema live under the OS-owned schema and fixture roots. Ajv validates the fixture, fast-deep-equal validates the neutral request/status projections, and the native Pack worker codec rejects missing, zero, and extra generation fields.

An initial create `POST` returning a bare `409` now produces a separate local `space-artifact-creation-catalog-refresh-required` disposition before the unchanged terminal `failed` status. The disposition carries the exact request, Space, and captured catalog generation. It does not assert that the Hub proved a catalog mismatch: a bare conflict can also represent an idempotency or Directory conflict. It never retries the create request. It is emitted only while the retained operation object, worker epoch, and abort owner are current after the awaited initial `POST`; ordinary poll and cancel conflicts do not emit it. A held-response test retires the owner before releasing the `409` and proves that neither a refresh request nor a late failure is published.

The Shell consumes that disposition only while the original creation remains nonterminal and unopened, the selected catalog authority still has the same runtime, client, Space, and generation, and the same session still owns the Space-index mount. It synchronously withdraws the selected catalog and its UI authority to `loading` before posting one catalog-open request, so a duplicate disposition cannot start a second fetch. The creation request is not resubmitted or rewritten; its ordinary following `failed` status remains authoritative. A 12-row neutral corpus covers missing, terminal, opening, replaced-request, rotated-generation, replaced-runtime/client/session, and wrong-document owners through the canonical Directory Draft 7 schema and the independent Shell projection.

Evidence:

- Root behavioral RED `59764`: the generation was stripped from the request and rotated/missing Ready statuses were accepted.
- First focused attempt did not reach tests because the root Nx registry referenced a relocated Trinity retirement test. The registry now resolves the canonical `🔬️document-retirement/🟦️.ts` export.
- `artifact-creation-catalog-generation-focused.log`: 2 passed, 331 skipped, terminal success.
- `artifact-creation-catalog-generation-full-os.log`: 333 passed across 5 files, terminal success before the refresh disposition extension.
- `artifact-creation-catalog-refresh-red.log`: the new neutral `409` law observed only accepted and failed, proving the missing refresh signal.
- `artifact-creation-catalog-refresh-green.log`: 4 passed, 331 skipped, terminal success. It covers the strict wire/fixture oracle, current-owner `409` ordering, no resubmission, and stale-owner suppression.
- `artifact-creation-catalog-refresh-full-os.log`: 335 passed across 5 files, terminal success on the complete current OS suite.
- `artifact-creation-catalog-refresh-shell-red.log`: the focused renderer law failed because no Shell refresh predicate existed.
- `artifact-creation-catalog-refresh-shell-green.log`: 1 passed and 711 skipped across the registered renderer suite, terminal success for the 12-row schema and owner-fence law.
- `artifact-creation-catalog-refresh-shell-full-renderer.log`: 712 passed across 19 files, terminal success with the current Shell mount/readiness and PluginRuntime composition packets.
