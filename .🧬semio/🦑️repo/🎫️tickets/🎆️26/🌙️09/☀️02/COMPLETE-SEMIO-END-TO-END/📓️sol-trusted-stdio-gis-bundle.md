# Sol Trusted Stdio + GIS Bundle

## Boundary

Current source defines a closed server-owned bootstrap profile with exactly two packages (`gis/semio:gis` and `stdio/semio:stdio`), 28 native codec rows, and one read-only GIS Map viewer target. The source and neutral boundary is reproducibly green. Actual component materialization, candidate-Hub activation, authenticated GIS plan issuance from those fresh bytes, and current-pointer rotation have not run successfully yet and remain unaccepted.

The earlier document-open Native8 gate remains RED with zero native assertions at its Stdio compile frontier (session `26716`). The later source audit found all nine HTML5 leaves now carry the exact `MutationLeaf` contract, so that diagnostic may be an older compile cascade, but no replacement native terminal exists and this report does not upgrade it.

## Implemented contract

- Trusted-catalog schema version 2 carries one exact selected closure, its SHA-256, the complete profile generation, and the sole open target including parent dialect and grant.
- The profile generation frames every selected package identity, role, component SHA-256, component BLAKE3, descriptor SHA-256, dependency list, native codec row, and open-target field. The zero-target Stdio package therefore changes the generation when its component, descriptor, or codec authority changes.
- GIS contributes Map and Terrain native codecs but only Map discoverability/open authority. Stdio contributes its exact 26-codec closure and zero targets.
- Stdio and GIS retain independent exact package versions; the profile does not impose an accidental cross-package version-equality constraint.
- Stdio native factory receipts now bind the compiled package version. The native provider is the exact 28-receipt Stdio+GIS set.
- The generic fresh component producer owns separate empty build/stage roots, disables incremental/wrappers, bounds component/core/descriptor/copy sizes, checks WIT actor export and exact package/hash identities, supports deadline/cancellation progress, and never writes owner descriptors or invokes registry generation.
- Fresh Cargo output is resolved through the shared artifact-path authority. Before staging, a side-effect-free registry verifier fatal-decodes diagnostic UTF-8, strict-decodes and canonically re-encodes the packed descriptor, requires JSON/pack equality and the descriptor self-hash, and binds package/version/role/isolated execution plus component/core hashes.
- Failed bounded copies and metadata writes close their file handles before unlinking partial output, including on Windows; failed current-pointer replacement also removes its temporary file without altering the retained current generation.
- The Hub materializer builds Stdio and GIS sequentially into isolated roots, verifies exact 26+2 source closures, writes an immutable generation, fsyncs it, and never loads/registers codecs or publishes `current.json`.
- The candidate path starts a distinct Hub with the immutable bundle/profile pair, reaches full readiness, issues a real local native credential, creates a private probe space, announces a GIS Map descriptor bound to the fresh package/hash row, and requires an authenticated exact Map open plan before publishing canonical bounded `current.json`.
- The ordinary dev command uses a valid current generation or funnels a newly materialized one through that same isolated candidate/open-plan proof, rereads the published pointer, and only then starts the long-lived Hub. It cannot publish directly.
- The native gate builds and resolves its Hub executable from an explicit ticket-owned Cargo target beneath `SEMIO_TEST_ARTIFACT_DIR`; it cannot silently use the ambient workspace target while claiming ticket-local evidence.
- The separate process gate first publishes verified generation A, requires a missing-profile candidate to fail without changing current A, then reissues the zero-target Stdio descriptor with a server-owned bounded rotation label. It recomputes and production-verifies the descriptor self-hash/file hash and full profile generation before atomically retaining generation B. Candidate B reuses the durable directory root, requires A's authenticated plan receipt to fail exactly before issuing the exact B GIS Map plan, and only then publishes current B. This is source-staged and registered but has not run.
- The prior-generation receipt exchange is resolved and authenticated before descriptor/target selection in production, and the process harness clears the retained receipt on every stale-exchange exit. Reusing the exact candidate data root after the rejected candidate also makes handle cleanup observable: generation B cannot start against that root if the failed candidate retained its directory/process ownership.
- Rotation is restart-only. There is no live codec replacement, client-selected bundle/profile path, compatibility form, remote signature claim, browser execution claim, or native execution claim.

## Schema-first oracle

The neutral fixture and JSON Schema are in `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/`. The independent Bun/AJV oracle reads the package-owned Stdio/GIS receipt projections, checks exact 2/28/1 cardinality, canonical big-endian framing, Node and WebCrypto SHA-256 agreement, a first-party BLAKE3 known-answer vector, zero-target Stdio generation sensitivity, path/boundary/cancellation/rotation vectors, and the exact Map/Terrain distinction. Its explicit cross-generation hostile case requires an old plan generation to become stale against a different current generation while the matching new plan remains accepted. It also drives the production fresh-descriptor verifier with one canonical JSON/pack pair and three hostile pairs: substituted JSON identity, trailing packed byte, and invalid UTF-8.

The registered source gate also inspects the isolated producer boundary and pins production publication ordering: the materializer contains no current-pointer publication; the candidate and dev paths publish only after their readiness awaits; and the bootstrap hands materialization to candidate validation in order. It additionally requires both authenticated receipt exchange and live socket validation to compare the retained plan generation against the currently loaded catalog generation. This is source/neutral evidence, not a successful rotation runtime.

## Registered evidence

| Evidence | Terminal | Meaning |
| --- | --- | --- |
| `bun ./📜️script.ts nx run os-hub:trusted-stdio-gis-bundle-check --skip-nx-cache -- --source` (`06603e`) | exit 0 | Initial current-source neutral/source proof: packages 2, codecs 28, targets 1, hostile 18, cancellation 8, AJV + Node + WebCrypto + first-party BLAKE3. |
| Same gate after adding publication-order assertion (`eee280`, then `72c2f0`) | exit 1 | Clean TDD REDs: the new source selector matched its own literal rather than the production declaration. No production activation assertion was reached. |
| Same gate with anchored production declarations (`eb5a30`) | exit 0 | Final current-source neutral/source proof including readiness-fenced publication ordering. |
| Same gate after fencing native Hub build/output ownership (`7c9634`) | exit 0 | Current final source proof also requires the native Hub target/executable to stay beneath the ticket artifact root. |
| Same gate after cross-platform close-before-cleanup repair (`e56214`) | exit 0 | Current final source proof also pins partial copy/metadata cleanup and current-pointer temporary cleanup ordering. |
| Same gate after removing cross-package version coupling (`d9fed1`) | exit 0 | Current final source proof retains exact per-package versions without requiring Stdio and GIS releases to match each other. |
| Same gate after authenticated candidate-plan and single dev publication-funnel cutover (`ba4261`) | exit 0 | Current source requires readiness → authenticated exact GIS Map plan → publication, and prohibits direct publication from materializer/dev. Runtime execution remains pending. |
| Same gate after shared Cargo-path and fresh JSON/pack verifier integration (`20fbce`) | exit 0 | Current final source/neutral proof includes one accepted and three rejected production verifier pairs (`descriptor-pairs=4`) and pins both shared path/verifier calls in the producer. |
| Same gate after the process-lifecycle source fence (`241e4a`) | exit 1 | Clean owned TDD RED: the new restart-candidate source selector was not scoped to the production process block. Native materialization was not started. |
| Same gate with the process block selected independently (`495684`) | exit 0 | Current final source/neutral proof pins rejected-candidate current preservation and retained-generation restart ordering; the process runtime remains unrun. |
| Same gate after the schema-first cross-generation hostile (`49ce59`) | exit 0 | Current source/neutral proof: packages 2, codecs 28, targets 1, hostile 19, cancellation 8, descriptor pairs 4, stale-plan 1. It pins both production generation revalidation sites; no rotation runtime is inferred. |
| Same gate after real generation-B process harness staging (`8f2a06`) | exit 0 | Current source/neutral proof additionally pins production-verifier-backed Stdio descriptor reissue, full-generation recomputation, atomic retention, and old-plan denial before fresh-plan issuance. The registered process runtime remains unrun. |
| Same gate after stale-receipt ordering/cleanup strengthening (`1b07f4`) | exit 0 | Current final source/neutral proof pins receipt resolution before target selection, receipt clearing, same-root candidate restart, and server-owned rotation identity. Terra source-qualified the descriptor-reissue design; runtime remains unrun. |
| `bun ./📜️script.ts nx run @semio-tech/plugin-registry:generate --skip-nx-cache` (`4973cf`) | exit 0 | Canonical owner generated 59 plugin crates, 60 playgrounds, 45 framework packages, and launch bytes including source, native, process, and bootstrap commands. |
| `bun ./📜️script.ts nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache` (`a037cd`) | exit 0 | Final project/launch generated bytes are fresh with the dedicated process gate. |

Permanent commands are owned by the existing Hub `📜️script.ts`, called through `project.json`, and present in the canonical launch seed/generated launch:

- `os-hub:trusted-stdio-gis-bundle-check -- --source`
- `os-hub:trusted-stdio-gis-bundle-check -- --native` with a seed-owned absolute ticket-generated artifact root
- `os-hub:trusted-stdio-gis-bundle-process-check` with a separate seed-owned absolute ticket-generated artifact root
- `os-hub:trusted-stdio-gis-bootstrap`

## Remaining acceptance

1. Run `--native` with an absolute ticket-owned `SEMIO_TEST_ARTIFACT_DIR`. It must build both actual WASI components, load the exact immutable generation in a candidate Hub, reach readiness, issue the authenticated exact GIS Map plan, and publish `current.json` only afterward.
2. Prove all 28 codecs register atomically or none do, plus failed-candidate cleanup and cancellation against the actual materializer/provider path.
3. Run the registered process gate to prove the staged failed-candidate/current-preservation and distinct generation-A → generation-B rotation behavior, including terminal rejection of A's authenticated plan before the B plan succeeds. Until that terminal exists, cross-generation runtime remains unclaimed.
4. Re-run the exact document-open Native8 and queued Hub laws from current source. Do not classify source/neutral success as client execution or inference authority.
