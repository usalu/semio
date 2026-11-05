# React Picker Independent Review

Read-only source/fixture review, 2026-11-05. No new implementation defect found. No Native, server, full Vitest, or browser execution performed.

## Public behavior

Renderer owner prefix: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`.

`🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts:104` requestFileOpen delegates to existing requestFileSelectionV1. The latter returns [] without DOM, installs handlers before click, settles cancellation/empty change/click exception once, clears both handlers and removes the transient input. Repeated terminal events cannot cause additional cleanup or reads.

After selection, requestFileOpen retains order by sequentially awaiting each File. Exact name is preserved, readAs=dataUrl uses actual FileReader.readAsDataURL, and every other readAs uses File.text. Error/abort/synchronous FileReader read refusal skip the failed file and continue; these are ordinary per-file behavior, not a bounded read/cancel protocol. NoDOM/cancel paths never instantiate FileReader. File.text rejection now propagates through the public async function rather than being lost inside an async DOM callback. No changes to Lowpoly dataURL/base64 decoding are implied.

This fixes the concrete unresolved React chooser promise identified in host-media-contract-audit.md. It reuses the existing picker owner rather than duplicating settlement state. Large-file reads remain whole-file and have no size admission, progress, or AbortSignal in this helper; do not claim the bounded natural-file read API is now used here.

## Tests and fixture checks

Permanent owner `🧪️tests/💾️media-export-and-natural-files/🟦️.ts` calls the real public requestFileOpen for cancel, empty change, click exception, handler removal, exactly-once cleanup, ordered selection, options, Unicode filename and exact dataURL text. It restores DOM spies in finally. Ticket file-open.test.ts independently exercises the same helper through actual JSDOM File and FileReader, strict Ajv-validating the neutral fixture. Source review confirms it does not copy/reimplement picker logic.

NoDOM and successful text-mode branch are not included in the four executed ticket tests/current added permanent cases. Their source paths remain straightforward and unchanged in encoding semantics; absence is a coverage limit, not a found regression. The actual reported before/after counts (0/4 red → 4/0 green,21 assertions) are root's retained execution evidence; this independent review did not re-run unchanged tests.

Independently recomputed both expected dataURLs with Node Buffer UTF8→base64: café.obj and triangle.stl fixture contents are exact. Strict schema forbids additional properties, constrains exact chooser options/terminal-event set and file shape. MIME values are valid fixture values; extension is preserved through names.

Focused absolute UTF16 paths: production helper156, permanent test128, fixture136, schema143, ticket test106, existing root report117. All ≤256.

## Readiness snapshot

Executed exactly once: `repo doctor --only=dependencies --json`, exit1 at1793843725263. Verdict blocked: dependencies.unpublished, no published pointer;36 unfinished generations. Did not run suggested ensure/setup.

Draw immediate wait from cursor19 returned cursor `c65249d6-65bd-47f4-894d-d5f6f48c226f:20`, active/inProgress. Latest commentary says aggregate timeout exposed outdated collision expectation;13 cases split into named tests with300-second per-case limit, valid recovery separated from invalid-ready receipt refusal, strict TypeScript passes. This is progress commentary, not dependency publication. No foreign message sent. Declared React dev/full Vitest remains gated by unpublished JS dependencies.
