# Home Host Session Identity Terra Audit

Date: 2026-09-13. Source-only audit; no Cargo, native, Bun, or Nx commands were run and no product sources were changed.

## Source aliases

- REACT_SHELL: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx
- WGPU_SHELL: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- HOME_EDITOR: /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- HOME_CONFIG: /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
- SPACE_ENGINE: /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs

## Verdict

The framework schema and Home/Space consumer boundaries are coherent, but the end-to-end host guarantee is not satisfied. React has two principal guest crossings without an identity restamp, WGPU has a native URI-action bypass, and Home's fixed-route 4096 proof does not use the public contract's scalar accounting.

| Severity | Finding |
| --- | --- |
| P0 | React ordinary actions and commands can forward carried stale or missing sessionIdentity. |
| P0 | WGPU apply_shell_uri forwards session.view_state, not the current live_view_state. |
| P1 | Home's fixed scalar admission sums UTF-8 bytes; public invocation admission uses escaped UTF-16-aware cost. |

## Actionable findings

### P0 — React action and command paths bypass the live host projection

REACT_SHELL:4295-4308 correctly derives render state from the target state and overwrites sessionIdentity from identityRef.current; directory publication likewise refreshes identity at REACT_SHELL:3358-3403 and /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📇️directory-bootstrap/🟦️tsx:103-150,220.

Ordinary actions instead base dispatch state on retained target view state at REACT_SHELL:6492-6502, then pass it to direct and Wasm actors at 6559-6562 and 6577-6579. Commands repeat that pattern at 9076-9099. Other direct crossings include URI Studio actions (5843-5852), utility actions (6282,6337), spawned document synchronization (5194-5237), and the index-artifact command (8594).

An A → B identity change, sign-out, reused window, or reused document can therefore expose A, omit B, or retain an identity which must have been cleared. Strict Home/Space consumers may reject the request, but that does not meet the requirement that every action receive current host identity.

Remediation: centralize host-to-guest state derivation so it overwrites or clears identity before adding interaction context, and use it at every actor, command, document, URI, and directory crossing. Add action and command regressions with carried A, current B, sign-out, and a second window/document.

### P0 — WGPU URI dispatch bypasses live_view_state

WGPU_SHELL:3701-3722 correctly overwrites or clears identity in live_view_state; ordinary actions use it at 5083-5101, commands at 5210-5218, and directory publication snapshots live identity at 5542-5590. But WGPU_SHELL:6273-6285 sends session.view_state into openSpace from apply_shell_uri.

This is a reachable native action path that can forward stale or absent identity after a principal transition. Route it through live_view_state and audit every direct guest call passed session.view_state. Add a URI-path regression with carried A, host B, then no host identity. The source test at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:136-146 tests live_view_state itself only, so it cannot catch this bypass.

### P1 — Home's 4096 aggregate is measured differently from the public contract

HOME_EDITOR:62-105 applies PUBLIC_INVOCATION_STRING_BYTES to the nine migrated routes, but home_retained_extent sums String::len() (UTF-8 bytes). /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4549-4577 defines public invocation cost differently: controls cost five, and non-ASCII costs five times UTF-16 width.

For example, 2,048 é characters occupy 4,096 UTF-8 bytes and pass Home's helper, but cost 10,240 under public admission. Astral and control characters diverge as well. This is not evidence that normal public ingress accepts an oversized request; public validation should reject it. It is a false fixed-route proof/admission result for typed or factory callers, so the claimed exact 4096 alignment is false.

Remediation: sum the shared public_invocation_char_cost for every scalar and aggregate that value. Add exact/max-plus-one tests for BMP non-ASCII, astral, controls, and mixed fields; current coverage at /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:28-48 uses ASCII only.

## Verified source evidence

- Contract/schema: Option<ViewSessionIdentity> is defined at /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4389-4443; TypeScript parses an optional exact object at /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:805-915; JSON schema makes it optional and strict at /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json:34-36,82-103.
- Home identity/context: shared validation is at /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs:445-457. The editor requires identity and retained view state at HOME_EDITOR:30-32,109-124, copies request.context into the job at 401-427, and the framework serializes/deserializes it at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:222-259,320-356. Viewer main requires identity at /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs:71-78.
- No reachable Home local-owner fallback: HOME_EDITOR:504-526 and /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-studio/🦀️.rs:37-75 supply the validated host principal. A defensive low-level local default remains at /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs:217-226, but the audited Home path cannot reach it without first failing identity validation.
- Space presence: its empty-payload command validates identity at /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/👥️presence-heartbeat/🦀️.rs:8-29; the retained reducer obtains request view state at SPACE_ENGINE:381-388; its HostOnly contract is 443-456. This source behavior depends on the host-restamping fixes above for live calls.
- No client mirror ownership: no client_id, client_name, SetClient, or equivalent was found in the audited current Home/Space config, command, mutation, and facet sources. Home persists only directory projection, binding digest, authorization generation, and receipt at HOME_CONFIG:116-132, with only snapshot/fold/replace mutations at 247-265.
- Directory projection: HOME_CONFIG:121-183 has no current user ID/display name and rebases directory state on authorization transition. The binding/auth-generation fields remain appropriate authenticated read-model authority, distinct from current host identity.
- Route extent: nine scalar routes are admitted and tool-jobbed at HOME_EDITOR:62-105,401-427; seven routes remain BatchOnlyPendingRewrite at HOME_EDITOR:608-623, subject to the P1 accounting correction.

## Validation boundary

The milestone's neutral browser .236 r8 result is historical documentation only and was not independently rerun. Native post-change .237 and the full Home90 suite remain NOT RUN. This audit makes no runtime, Cargo, or native pass claim.
