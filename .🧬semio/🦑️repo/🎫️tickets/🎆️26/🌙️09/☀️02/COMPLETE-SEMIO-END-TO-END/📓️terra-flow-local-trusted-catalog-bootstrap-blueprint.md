# Flow Local Trusted-Catalog Bootstrap Blueprint

## Current Verdict

**RED: no ordinary launch supplies a trusted catalog selection.** The hub accepts
the pair `OS_HUB_TRUSTED_CATALOG_BUNDLE` and
`OS_HUB_TRUSTED_CATALOG_PROFILE` only together, but the ordinary dev, secure suite,
native, MCP, browser, and devcontainer paths set neither. Consequently a normal
local launch is correctly partial-ready rather than Flow-open-ready.

This is a read-only current-source audit. No code, launch seed, build, or process
was modified or run.

## Existing Configuration Map

| Surface | Current source | Trusted-catalog result |
| --- | --- | --- |
| Hub process | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:380-391,5315-5321` | Both variables absent returns `Ok(None)`; only one is a hard failure; both invoke `TrustedCatalogLoader`. |
| Ordinary launch | `.vscode/🧩️launch.seed.jsonc:3729-3747` | Sets only port and data path. `os-hub:dev` starts with no catalog. |
| Secure browser/native/MCP launches | `launch.seed.jsonc:3768-3827` | Also set no bundle/profile. They authenticate a client but do not make open-plan readiness true. |
| Hub launcher | `🌎️hub/📦️packages/🦀️rust/📜️script.ts:586-676,3851-3949` | `startLocalHub` inherits `process.env` and has no typed catalog option. Normal dev can accidentally inherit a user shell selection; the source launch itself supplies none. |
| Security smoke/admin journey | `📜️script.ts:617-623,1394-1455,2167-2174` | Explicitly deletes the catalog pair for isolated smoke and accepts truthful partial readiness. This behavior must stay. |
| Devcontainer | `.devcontainer/devcontainer.json` and post-create/start scripts | No hub catalog variables or materialization. The same Nx launch path runs inside the container. |
| Tests | `🚀️bin.rs:5562-5691` | A temporary stdio-only bundle proves loader/readiness mechanics, but it is manually injected into test state and does not represent launch configuration. |

The production loader already provides the correct trust boundary. It canonicalizes
the bundle path; bounds and dual-hashes components; bounds and hashes descriptors;
validates package/dependency/profile selection; rejects missing, duplicate, or extra
native bindings; validates every open target; preflights the entire assembly; and
only then registers codecs (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:342-504`).
The launcher must feed this boundary, not replace it with a client assertion or a
browser-supplied catalog row.

## Minimal Zero-Touch Local-First Path

Add one server-owned `dev flow-catalog` mode in the existing hub
`📜️script.ts`; do not make the browser, WGPU process, MCP, or a `.env` file select
the profile.

1. **Materialize server-owned input before spawn.** Add a private
   `prepareLocalFlowCatalog(repoRoot, runRoot)` beside `startLocalHub`. It takes no
   HTTP, browser, command-line client, or client environment input. It obtains the
   Flow component/descriptor/build receipt from the source-owned stdio provider
   output, creates `runRoot/catalog/<immutable-generation>/components` and
   `.../descriptors`, and writes the bundle/profile there. It uses `resolve`,
   `mkdtempSync`, `rename`, `spawn(..., shell:false)`, and bounded Node byte APIs,
   all of which are already the launcher's Windows/macOS/Linux pattern. On Unix it
   applies the existing `0700` run-root policy; Windows relies on its user-private
   temporary directory/ACL rather than a POSIX-only assumption.

2. **Pass a typed selection only to the hub child.** Extend the private
   `startLocalHub` options with a `trustedCatalog` value containing the generated
   bundle path and literal profile ID. Before spawning, delete both inherited
   `OS_HUB_TRUSTED_CATALOG_*` values unconditionally. If `trustedCatalog` is
   present, set both variables from that server-generated value; if absent, leave
   both absent. This prevents an IDE/browser/native child or a developer's ambient
   shell variables from selecting a catalog. It also retains the present partial
   readiness behavior for security smoke.

3. **Make Flow a real catalog closure first.** The materializer cannot use the
   current Flow editor/viewer declaration as a substitute for a native provider
   entry. First add a real Flow native codec receipt, the corresponding static
   provider binding, and descriptor-owned React/WGPU viewer/editor `openTarget`
   records. Its component SHA-256, component BLAKE3, descriptor SHA-256, artifact
   schema/pack hash, parent dialect, factory identity, and package/version must all
   originate in that build receipt. The trusted loader remains the final verifier.

4. **Expose a source launch, not generated metadata.** Add an
   `os-hub:dev-flow-catalog` target in
   `🌎️hub/📦️packages/🦀️rust/📋️project.json` that invokes only
   `bun ./📜️script.ts dev flow-catalog`. Add the matching source configuration to
   `.vscode/🧩️launch.seed.jsonc`, then regenerate `launch.json`. The seed needs
   only ordinary port/data values; it must not contain a user-editable catalog
   path/profile or client trust material.

5. **Use the same target unchanged everywhere.** Devcontainer, Windows, macOS,
   and Linux invoke `bun nx run os-hub:dev-flow-catalog`. No platform startup file
   should copy a bundle, export trust variables globally, or pass them to the
   browser/native/MCP child. The launcher owns the generated run-root and erases it
   through its existing `finishLocalHub` cleanup (`📜️script.ts:773-786`).

## Atomicity and Rotation

The profile must be immutable for a running process. Materialize a complete new
generation in a private sibling directory and atomically rename it before the hub
child receives either environment variable. The loader's assembly is then the
single publication point. A failed component/descriptor read, digest mismatch,
missing Flow factory, extra provider binding, malformed target, or missing profile
means the child exits before readiness; it must not fall back to the prior catalog
or listen as `openPlan=true`.

There is no current hot catalog reload authority. The smallest honest rotation is a
controlled hub restart against the same `OS_HUB_DATA`: prepare generation B, stop
generation A, start B, and issue only B's generation. Existing documents whose
descriptor no longer resolves under B must fail closed as component-unavailable;
the launcher must never rewrite their descriptors to fit B. Clients discard their
execution-target lease when the connection ends and must obtain a newly selected
target before opening after the restart.

The run-root is currently ephemeral, while `OS_HUB_DATA` may be persistent. Keep
that separation: catalog bytes are launch-time authority material, not mutable
directory/CQRS state and not a blob fetched from a client.

## Required Neutral and Process Laws

Add one language-neutral `semio.hub.local-flow-catalog-bootstrap/v1` fixture with
the canonical Flow receipt fields, one selected profile, bounded component and
descriptor bytes/digests, and expected generation. An independent AJV + Node
SHA-256/BLAKE3 framing oracle must cover:

- valid complete Flow closure and exact selected profile;
- profile missing, paired-variable absence/presence mismatch, noncanonical path,
  component SHA-256/BLAKE3 mismatch, descriptor SHA mismatch, wrong version,
  missing/extra/duplicate factory, pack-hash mismatch, duplicate target, and
  parent-dialect/surface mismatch;
- a same-data rotation yielding a distinct immutable generation only when the
  relevant target fields differ; and
- an unchanged persisted document descriptor rejected after a deliberately
  incompatible rotation, with no fallback selection.

Register a focused hub process gate in the existing `🌎️hub/📦️packages/🦀️rust/📜️script.ts`:

1. Start the actual `os-hub` through `startLocalHub` with a materialized Flow
   selection. Observe `/readyz` `200`, `artifactAuthority.ready=true`, and
   `features.openPlan=true`; then issue a plan whose package/artifact/surface/
   catalog generation exactly equal the materialized selection.
2. Start with each invalid fixture. Observe child failure before a ready endpoint
   is usable, no published codec/open target, and no leakage of raw component or
   descriptor bytes in bounded diagnostics.
3. Start A, persist only a descriptor, stop, start B over the same data path, and
   prove stale target rejection/no client publication. A new matching B descriptor
   may receive a plan only after the real loader succeeds.
4. Keep `secure-local-smoke` unchanged and explicitly assert it remains
   partial-ready with the pair absent. It is a security bootstrap test, not Flow
   readiness.

After that hub bootstrap gate is green, the separate browser and native real-hub
Flow runtime laws in `📓️terra-flow-provider-ordinary-os-startup-audit.md` can use
the launcher. They must derive a client execution-target lease from locally
verified package bytes; the launcher must not send bundle paths, profile IDs, or
catalog authority through browser input, fd3 credentials, or MCP input.

## Exact Ownership Split

- **Hub launcher owner:** `🌎️hub/📦️packages/🦀️rust/📜️script.ts` owns local
  materialization, child-only environment construction, rotation/restart process
  proof, and redaction.
- **Provider/descriptor owner:** stdio Flow receipt, static binding, package build
  receipt, and declared open targets. It must not add a client-controlled profile
  selection.
- **Hub runtime owner:** existing `configured_artifact_authority` and
  `TrustedCatalogLoader`; preserve their fail-closed behavior and atomic assembly.
- **Launch owner:** source seed and project target only; regenerate derived launch
  output after the source change.
- **Client owners:** consume only an immutable selected execution-target lease and
  public member-open result after server admission. They are not trust roots.
