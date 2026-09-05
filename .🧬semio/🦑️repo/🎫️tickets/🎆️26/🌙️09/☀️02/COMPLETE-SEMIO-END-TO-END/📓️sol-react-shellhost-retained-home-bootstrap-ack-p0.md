# React ShellHost Retained Home Bootstrap ACK P0

## Implemented boundary

The React shell now owns one hidden Home instance independently of the visible
application session. It is created only after authenticated identity, the live
host plugin, and the exact landing application are available. The worker's
`directory-bootstrap-open` is posted only after that instance exists. Visible
Home/Studio/Space switching does not replace this owner.

`DirectoryProjectionReceiptV1` is a closed five-field record:

- `schema = semio.space.home.directory-projection-receipt.v1`
- exact 64-character lowercase session-binding SHA-256
- positive safe-integer authorization generation
- nonnegative safe-integer committed directory frontier
- exact 64-character lowercase page receipt SHA-256

The real Home `applyDirectoryEventPage` retained command emits that typed
receipt from the replacement `HomeConfig`. The renderer's retained
typed-operation bridge keeps the receipt private until the terminal lane is
seen. A receipt-only intermediate page is rejected rather than exposed through
`handleAction`.

For each worker page the hidden owner retains the canonical JSON and complete
binding/generation/frontier/receipt authority while the real plugin action is
pending. The shell compares the terminal receipt exactly before posting
`directory-bootstrap-ack`. A recoverable action failure posts one rejection
before releasing the retained pending record. A malformed or mismatched receipt
closes the epoch and destroys the hidden instance. Explicit cancellation,
identity/base-URL change, plugin replacement, and unmount close the epoch before
instance destruction; explicit cancellation clears visible status only after
that retirement settles.

The status control is a polite, cancellable EN/DE live region while work is
pending, a polite retry state, and an assertive fault. An unsupported locale is
shown visibly rather than silently falling back to a default language.

## Schema and source ownership

- Home receipt and post-publication command result:
  `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
  and `🎮️commands/📬️apply-directory-event-page/🦀️.rs`.
- Neutral receipt schema/fixture:
  `🎮️commands/📬️apply-directory-event-page/🧬️receipt/🧬️.schema.json`
  and `🔣️.json`.
- Retained browser owner and receipt parser:
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🟦️.tsx`.
- Shell lifecycle wiring: the sibling `🏛️ShellHost/🟦️.tsx`.
- Terminal typed-operation bridge: the sibling `🔌️PluginRuntime/🟦️.tsx`.
- Independent/AJV fixture and focused runtime laws:
  `…/📦️packages/🟦️typescript/🎯️targets/⚛️react/📇️directory-home-bootstrap.test.tsx`,
  `…/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🧬️.schema.json`, and
  `🔣️.json`.

## Exact evidence

- Space Home source/neutral receipt gate, session **54882**:
  `NX_ISOLATE_PLUGINS=false bun ./📜️script.ts nx run @semio-tech/space-plugin:home-directory-event-page-owner-check --skip-nx-cache`
  exited 0 with `checks=27`.
- Final registered React gate, session **48403**, exited 0:
  `NX_ISOLATE_PLUGINS=false bun ./📜️script.ts nx run @semio-tech/framework-renderer-react:directory-home-bootstrap-check --skip-nx-cache`.
  It ran 21 independent/AJV source checks, 7/7 hidden-owner/ACK/accessibility
  runtime laws, and the exact PluginRuntime terminal-receipt law
  (1 passed, 63 skipped).
- Registry/launch generation, session **85363**, exited 0 and preserved the
  ordered launch entry `⚖️gate⚛️shell📇️retained-home-bootstrap-ack` at order
  419.9995.
- Generated registry/launch freshness, session **98840**, exited 0.
- The broad renderer TypeScript check, session **6902**, exited 1 on concurrent
  tutorial tuple/state, replication typed-array, Puzzle fixture-path,
  PluginRuntime UI-refresh, directory-export, worker open-plan, and other
  pre-existing diagnostics. It reports no diagnostic in the new
  `directory-bootstrap` contract or its focused test. The current ShellHost
  diagnostic is in the unrelated tutorial title projection at line 5011.

## Honest limits

This proves the React controller and real-shaped plugin-handle lifecycle in the
focused runtime, plus the typed-operation terminal bridge. It does not prove a
real Chromium + real WASM Home + live hub process journey. The existing
browser-capable collaboration launcher still needs the audit's one-user process
phase before browser process acceptance.

The Rust Home native sibling was not run in this packet, so the 27-check Home
result is source/schema evidence only. This does not promote browser
administration, WGPU/native/MCP document transport, restart persistence,
all-plugin activation, or integrated release readiness. Legacy visible-session
directory folding remains available for its prior live-stream consumers, but it
cannot produce this receipt and cannot authorize bootstrap ACK.
