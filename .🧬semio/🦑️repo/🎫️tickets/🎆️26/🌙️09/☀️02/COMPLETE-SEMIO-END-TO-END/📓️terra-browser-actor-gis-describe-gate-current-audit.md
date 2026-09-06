# Real GIS Describe Gate: Current Audit

## Verdict

`browser-actor-gis-describe-check --native` is the right narrow evidence boundary: it owns one
fresh GIS component, derives its closed actor through the one-shot lease, verifies the staged
descriptor receipt, then transfers that actor into a real Chromium dedicated Worker and invokes
only `describe.describe`.  It neither starts the Hub binary nor makes a Hub/session/renderer/Map
claim.

The route is nevertheless **not a fast GIS-only compile**.  The actual GIS Cargo manifest depends
on `semio-s-plugin-stdio` with `full-artifact-catalog` at
[GIS Cargo.toml](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:77).
The gate avoids compiling `os-hub`, but its fresh Wasip2 GIS build can still compile that Stdio
closure.  Earlier wording that it avoided Stdio compilation was incorrect.

No build or browser run was performed in this audit.  The root reported the TDD RED before the
implementation and source gate pending; neither is treated here as a native receipt.

## Correct current boundaries

The fresh owner is consumed exactly once by
`lease.consume(component => buildClosedBrowserActorArtifactV1(component, ...))` at
[Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4179).
`withFreshComponentLease` makes a private chunked component copy, waits for its consumer, and
zeroes that copy before the lease closes at
[describe script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:343).
The gate then requires the actor component digest, actor-byte SHA and byte length to agree with
the fresh receipt, and reads the exact staged descriptor through the stable regular-file reader
with receipt length/SHA validation at [Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4183).

The runtime descriptor oracle is also correct.  The helper requires canonical Pack input, exact
three-hash shape, blank runtime hashes, and exact canonical equality after blanking the three
staged hashes at [describe helper](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts:17).
It does not make the false byte-for-byte comparison against final `descriptor.semio`.

The 1 MiB Worker result constraint is correctly checked before Vite/Chromium work:
`assertBrowserActorDescribeCapacityV1` accounts for the child value framing (`outputBytes - 8`) at
[the helper](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts:5).
The normal path requires source detachment, a child rehash/load, one detached result transfer, and
zero child capacity after close at [Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4217).

## Actionable current deltas

### P1: Body acquisition has no parent-owned deadline

After reservation is ready, the page uses
`await (await fetch("/__semio-gis-child-body")).arrayBuffer()` at
[Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4218).
It does not check response status or header length, has no abort signal/deadline, and allocates the
entire response before checking final length/SHA.  A body request that never resolves therefore
never reaches the page `finally`; the already-ready child has no active boot/load deadline and its
capacity reservation remains live until the containing browser process is forcibly closed.

Keep this gate deliberately separate from Backbone's production
`readBoundedExecutionTargetBody`: it is a local Vite transport test, not protected asset-route
evidence.  It still needs a small local owner protocol:

1. start an `AbortController` before fetch with the gate's fixed body/load deadline;
2. require an OK, non-redirect response and exact parseable `content-length` equal to the already
   admitted actor length before `arrayBuffer()`;
3. abort/clear the timer in `finally`, wipe any obtained buffer on a failure, and retain the
   existing `owner.close()` finally;
4. add a local middleware stall row which proves rejection and
   `browserActorChildCapacity() == { actors: 0, bytes: 0 }` without relying on Chromium/browser
   teardown.

This does not widen any production body route.  The protected Backbone path must continue to use
its shared bounded stream reader and current lease assertion after every await.

### P1: Ticket work directory survives every run

The gate creates `work`, `target`, `stage`, and a Vite cache below the ticket artifact root at
[Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4173), but its outer
`finally` only wipes actor/descriptor bytes and closes the build control at line 4242.  It never
removes `work`.  A real GIS Wasip2 build can leave a material target and staging copies behind for
both success and failure.

After browser/Vite closure, actor/descriptor wiping, and `build.close()`, remove only this exact
`mkdtempSync` work directory (`rmSync(work, { recursive: true, force: true })`).  Do not remove the
ticket artifact root or another agent's generated evidence.

### Fixture coverage: corrupt the staged side too

The seven neutral rows cover blank/nonblank guest hashes, a guest semantic mutation, guest trailing
data, a guest extra field, and output capacity.  `trailing-byte` alters only the guest;
`raw-staged` is a final staged descriptor supplied as the *guest*, not an invalid staged input.
Add `staged-trailing-byte` (and, if keeping a separate vector, malformed staged hash) so the
helper's declared canonical staged-side rejection is exercised.  This is test completeness, not a
new catalog-authority bypass: the fresh producer independently validates/stages that descriptor.

## Current nonclaims

Even a green native gate proves only fresh GIS component → closed bytes → dedicated Worker → one
real descriptor call.  The Vite test URL is local and fixed, not a public actor endpoint.  It does
not prove authenticated plan/lease admission, asset-route revalidation, renderer activation,
reactor effects, a document Store mutation, WAL commit, undo, MCP, or collaborator visibility.

No product edits or builds were performed.

## 2026-09-06 Addendum: Applied Body Deadline and Cleanup Review

The formerly reported body-owner and work-directory gaps are resolved in the current source. The
source gate has nine semantic descriptor rows, including staged trailing data and a bad staged hash,
and its `fast-deep-equal` oracle is deliberately separate from the helper's Pack canonicalisation
at [Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4153). This audit did
not run the source or native gate; the parent reported the prior native attempt was intentionally
cancelled while its owned Bun process was still compiling, not a runtime failure.

The native page now rejects non-OK, redirected, or length-mismatched local responses before body
allocation, passes an `AbortController` to both the deliberate stall and normal fetch, clears each
deadline in `finally`, zeros any retained source/result, and requires capacity to return to zero after
the stalled reservation at [Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4224).
The outer `finally` zeros private actor/descriptor copies, closes the build control, and removes
only its own ticket-rooted `mkdtempSync` directory at [the same file](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4272).
No caller-selected path is deleted.

One qualification detail is not a demonstrated product leak: the Vite stall middleware intentionally
does not end its response at [Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4209).
`IncomingMessage.close` is not a safe immediate-destroy trigger—it can denote normal request
completion before the response ends. The parent-owned `activeStalls` set, final explicit destruction
of still-owned responses, and a `vite.close()`/empty-set observation are the correct test-harness
qualification. They should not be described as a proven Vite leak before the fresh native gate
observes one.

I found no current body-ownership, byte-identity, descriptor-normalisation, or cleanup blocker in
this narrow native gate. It remains a fresh materializer-to-Chromium proof only; it does not
exercise a Hub asset route or authenticate a document.
