# Shared Playground Input View

The complete `repo:test` retry passed native output restoration and the newer wasm publication checks, then spent several minutes in editor/playground validation. Sampling its owned Bun process recorded a 3.4 GB footprint and active recursive file reads. Source inspection confirmed that default `generatePlaygroundRegistry` passed no view to `registryExampleCatalog`, whose default creates a repository input view. This happened again for every playground's examples. Production registry projection already passes an explicit view and avoids that repeated discovery.

Only this verification's Bun child was terminated with SIGTERM; native process inspection confirmed it exited. The overall run is cancelled, not passing. Other development processes and generated stores were left intact.

Default playground discovery now creates one view and forwards it to plugin discovery, manifest/descriptor reads and every example lookup. An explicit caller view is retained. The language-neutral two-playground fixture failed before the fix (1.6 seconds) because readers received no shared view. Its permanent regression instruments the production function through TypeScript and compares the rows with lodash. A focused runtime comparison with the existing generated projection is running before the complete suite is restarted.

The focused regression passed in 2.2 seconds. The first live comparison completed discovery but incorrectly compared in-memory optional `undefined` properties with serialized JSON; the probe now compares the serialized projection contract. The subsequent runtime check passed: all 61 playground records matched, with 33,619 ms spent in default discovery/comparison and 40.6 seconds for the enclosing Nx test. The complete suite was restarted with the shared-view fix. These measurements overlap other workspace activity and do not establish an isolated speedup ratio.

## Projected Session Filter Follow-Up

The retry exposed a second live-read path in that validation stage. `buildPlaygroundSession` supplies a complete projection but filters by a bare plugin ID. When that ID is not itself a playground variant or alias, `resolveRegistryPluginIdsForFilter` fell back to live manifest discovery despite receiving the projected rows. The resolver now treats a supplied playground array as the complete lookup scope; unknown projected IDs stay unknown.

The permanent test reuses the runtime component closure's language-neutral vectors, including bare IDs, variant names, aliases and unknown IDs. A guard throws on any live lookup. After correcting the test import path, the regression failed on the actual forbidden lookup; the resolver fix passed in 509 ms. Only the second verification's owned Bun child was stopped with SIGTERM, and process inspection confirmed its exit. A real 61-session comparison and another complete suite are running. This second cancelled run is not an overall pass.

The real session comparison passed in 803 ms through Nx. Building and comparing all 61 sessions with the default generated projection versus one explicit projection took 25 ms. Every selected registry plugin ID matched its playground record. The complete suite and refreshed resolved artifact audit remain active.
