# Trusted Catalog Opened-Root / No-Link Boundary

Status: schema/source green on 2026-09-06; current native compile and exact platform laws are not yet terminal.

## Implemented boundary

- `TrustedCatalogDataRoot` opens the configured Hub data directory segment-by-segment without following links.
- It opens only the literal canonical `trusted-catalog/current.json`, then the exact digest-named generation directory.
- `TrustedCatalogGenerationRoot` opens parsed `TrustedCatalogRelativePath` values relative to its retained descriptor. Duplicate selected-closure paths are lexical capability identities, not canonical pathnames.
- `TrustedCatalogOpenedFile` inspects the opened handle, then reads that same handle in cancellable 64 KiB chunks with an opened-length-plus-one fence. Growth is rejected even when it remains below the caller maximum, truncation is rejected, and path replacement after open cannot redirect the read.
- Unix uses descriptor-relative `openat` with `O_NOFOLLOW | O_CLOEXEC`, `O_DIRECTORY` for ancestors, and `O_NONBLOCK` for a final leaf so a substituted FIFO cannot block before handle-type rejection. Windows uses rooted `NtCreateFile` with `OBJ_DONT_REPARSE`, checks attribute-tag information, and rejects reparse/directory mismatches behind the same first-party owner.
- `TrustedCatalogLoader::load_current` accepts the server-owned data root, verifies canonical current-pointer bytes, bundle digest, selected profile, and generation before publication. The arbitrary bundle-path loader and the ambient `OS_HUB_TRUSTED_CATALOG_BUNDLE` / `OS_HUB_TRUSTED_CATALOG_PROFILE` authority inputs are removed.
- Candidate process validation now stages and revalidates the immutable generation plus current pointer below its isolated `candidate-data` root before launch.
- The macOS linked-root law canonicalizes only the fixture-owned parent first (`/private/var/...`), then appends the linked leaf. This proves rejection of the intended initial-root link rather than succeeding accidentally on the platform `/var` link.
- The binary native-openable fixture writes the current pointer in the production struct's exact `profileId`, `generationId`, `bundleSha256` order rather than relying on `serde_json::Value` map ordering.

## TDD and registrations

The neutral JSON Schema and fixture live at `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🛡️opened-root/`. The first registered source run was RED on the intentionally absent owner/laws. After implementation:

```text
trusted-catalog-opened-root: AJV=1 paths=15 denials=7 same-handle=1 source=14 startup=no-ambient-path
NX Successfully ran target trusted-catalog-opened-root-check
```

This source receipt was rerun after the canonical fixture-parent correction and remained green. It is not a native filesystem verdict.

The existing producer/source corpus remained green after the isolated candidate migration:

```text
trusted-generation-stage: AJV=1 WebCrypto=1 cases=19
trusted-codec-source: AJV=3 WebCrypto=1 cases=15
trusted-rotation-source: WebCrypto=1 cases=6 writes=7
trusted-stdio-gis-bootstrap-oracle: packages=2 codecs=28 targets=1 hostile=19 cancellation=8 descriptor-pairs=4 stale-plan=1
NX Successfully ran target trusted-stdio-gis-bundle-check --source
```

The source packet was rerun after the shared Hub-script format pass. Its fresh ticket-owned evidence is `fresh-component-staging-GnC6YT`, `generation-stage-yG3J4j`, `codec-source-WA4C3Y`, and `rotation-source-8pTSB9` beneath `🗑️generated/trusted-candidate-stage-source`. It still ends explicitly with `native materialization, candidate hub, and current pointer remain unclaimed`.

Registered targets:

- `os-hub:trusted-catalog-opened-root-check`
- `os-hub:trusted-catalog-opened-root-native-check`
- launch orders `411.134` and `411.135`, generated lines 6525–6553

Plugin-registry generation and immediate `check-generated` are green.

## Native qualification state

The prior registered six-law `trusted-browser-actor-catalog-native-check` owns `public-member-open-sol-target` as Bun PID `32045` / Nx PID `32061`. It is still compiling the full Stdio dependency graph and has no terminal verdict. Because Cargo will compile the current Hub sources after that dependency, it is also the first current-code compile signal for this migration. No second Cargo build is queued against the cache.

The new exact native packet contains six laws. Three library laws cover linked initial/current/generation roots plus component/intermediate/actor links; opened-handle swap, zero/maximum/growth/cancellation bounds and an actual Unix FIFO final leaf; and neutral relative-path parity. Three binary laws cover unconfigured data-root absence, configured-without-provider refusal, and the real selected Stdio provider's current-pointer-to-readiness transition. A macOS terminal receipt is still required, followed by equivalent Linux and Windows native rows. Source green is not represented as filesystem-runtime qualification.
