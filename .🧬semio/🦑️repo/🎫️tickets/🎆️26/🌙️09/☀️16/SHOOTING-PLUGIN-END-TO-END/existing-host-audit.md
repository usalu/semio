# Existing Host Reuse Audit

Date: 2026-11-04

No currently proven host can be reused for full shooting acceptance.

## Current Read-Only Observations

- The declared Play E2E metadata roots `🏢️semio-tech/🎡️play/dist/services/forms-e2e` and `🏢️semio-tech/🎡️play/dist/services/e2e` are absent. No prepared session or ready URL is published there.
- Read-only GET requests to the owned readiness endpoint `/__semio/nx-service` on the declared Play port 6033, generic OS port 6070, and shooting port 6019 all returned `ECONNREFUSED`.
- This does not claim that every peer process is stopped: the active Forms E2E target can still be executing its prerequisites, and its later server uses an ephemeral port whose authoritative URL must come from its service record.
- Our earlier shooting serve was independently cancelled through normal SIGINT with exit 130 and explicit lease-release confirmation. No process status was inferred from a tool usage error.

## Precise Reuse Boundary

The Forms target is intentionally scoped:

- `@semio-tech/semio-tech-play:test-e2e-forms` depends on `serve-forms-e2e`.
- That service invokes `serve-forms-test` and the `ServeFormsTestScript` fixes its pane to `forms`.
- It validates `readPlayActivation(workspace, "forms")`, sets the Forms pane, and freezes file watching for its owned test generation.
- The Vite configuration selects runtime targets/assets and activation components from that pane's closure. It does not establish an activated shooting guest with a current shooting source stamp.

A generic Play host can be multi-app: the normal port 6033 service consumes the union of activated pane closures, including shooting. Such a host is not currently listening or publishing full E2E service metadata in these observations. A later host could be considered only after its real configured union, listener identity, current shooting artifact/source stamps, and current frontend are proven. Serving a leftover file from the shared module directory would not establish that proof.

No peer host selection, environment, receipt, source stamp, process, or server was changed. No browser was launched and no replacement server was started.

## Source Owners Read

- `🏢️semio-tech/🎡️play/📋️project.json`
- `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/📜️script.ts`
- `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🧪️e2e/🟦️.ts`
- `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts`
- `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/♻️activation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session/🟦️.ts`

The activation helpers were inspected as source rather than invoked: the unscoped reader can publish a union receipt, which would exceed this read-only audit.

## Resumed Browser Preflight

The current retained probe again passed its owned Node syntax check (exit 0), and `repo tools verify chromium --lang=en` again passed (exit 0; pin/found 1248; status ok). The probe still uses the correct first-party loader/launch policy and full document/undo, nonblank PNG/SVG, and viewer proof. Browser acceptance remains sequenced after the native gate.

## Freshness Evidence Required On Later Reuse

Current activation source has a warm branch that compares a prior receipt's `sourceContentSha256` with the stored source-content marker under that activation runtime's freshness directory. The marker reader reads a previously written hash file; that warm branch does not itself hash current source or the installed module bytes before returning.

This is a source-level acceptance caveat, not proof that a running host serves stale bytes. For later reuse or startup, a warm activation log alone must not substitute for a current guest publication: verify the producer's current module/artifact identity and relevant source stamps independently. No activation or freshness state was changed during this read-only inspection.

Relevant owners: OS dev `♻️activation/🏃️execution/🟦️.ts` and plugin `🏗️build/🔍️freshness/🟦️.ts`.
