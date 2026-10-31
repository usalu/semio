# Generated Directory Release Audit

The reported 08:51–08:53 deletion remains unattributed. This read-only Source audit found no canonical borrowed-ticket-root cleanup path that explains removal of both active ticket generated directories. No deletion, rerun, private environment inspection or cleanup was performed. Parent reports its clean removed zero entries; that is separate evidence, not an attribution.

| Owner | Actual removal boundary | Result |
|---|---|---|
| Core testing/📥️invocation/🟦️.ts:68–76 | bornInProcessV1 removes artifacts only when it created mkdtemp and no SEMIO_TEST_ARTIFACT_DIR was configured | A configured borrowed artifact directory is retained. |
| Repo library/testing/🪪️native-birth/🟦️.ts | Configured artifact path uses repoTestArtifactEnvironment with no-op release; absent path owns sandbox release | No borrowed-root removal found. |
| Repo library/process/environment/test-output/🟦️.ts | Resolves and creates configured artifact path; no recursive removal | Environment projection does not own the configured root. |
| Core testing/sandbox/root/🟦️.ts | Owns random eight-hex child below host sandbox/session; dispose removes that root and only nonrecursively removes session home | Adopted sandbox environment release is a no-op. |
| Core testing/sandbox/reclaim/🟦️.ts | Session reclaim can recursively remove its entire session home when no recognized live eight-hex child remains | This is an actual broad cleanup boundary, but home is derived from host sandbox parent plus session.resources.tests.root, not SEMIO_TICKET_DIR or configured artifact root. No evidence connects either deleted directory to that home. |
| Repo modules/test/storage/🟦️.ts | Retention removes genuine diagnostics-store tests/task/run children after actual leases and run receipt validation | No SEMIO_TICKET_DIR input or arbitrary configured artifact-root deletion found. |

The sandbox parent resolver (sandbox/layout/🟦️.ts:54) appends the declared sandbox parent to physical temp/explicit parent; it does not directly return a ticket artifact path. Session reclamation also follows directory stat results; this audit does not prove its behavior against adversarial symlinks or establish any such link existed. A matching recursive-removal statement alone cannot identify the historical actor.

Remaining attribution boundary: no retained historical unlink/rmdir audit event, responsible process receipt or actual resolved cleanup-root trace was available in the reviewed evidence. Preserve the original actor-unknown status. A future genuine cleanup event must record its owned root and issuer to support attribution; no new instrumentation or replay was introduced here.
