# Independent Retirement and Mounted Metadata Source Audit

Date: 2026-10-29. Read-only source review of the supplied unready-capsule and mounted-initial packets. Only this Markdown report was authored. No code edit, build, semantic test, Native/kernel operation, real cache effect or peer process action ran. Shared storage/publication files contain independent peer hunks; this review addresses root's unready plan/reference/collector/effect/driver integration and the new mounted initial helper plus its defining call.

## Concrete Finding

### P1: Capsule Intent Is Removed Before the Payload Directory, Breaking Interrupted Retirement Retry

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts`.

At line219, retireRepositoryDependencyRelativePayloadV1 orders each directory's remaining entries by only postponing bindings.json, then bytewise basename. Lines221-222 process payload subtree directories before the capsule root, but the p directory itself remains a root entry until root processing. For canonical root names i,p,r, the root effect therefore removes i first, then p, then r.

A cancellation, process exit or crash after the successful i unlink and before p unlink leaves an empty p directory with r and no i. The next genuine fresh plan at lines185-186 treats any present p as payload-bearing and requires canonical i; it refuses this reachable cut. It also cannot classify the capsule as metadata-only until p is absent. Thus the implementation's own valid partial retirement can strand setup, despite all descendants already being deleted. This is a cut/liveness correctness defect, not a request to infer authority from old metadata.

Required correction: keep both fixed capsule metadata roles i/r until p has been durably removed; then retire current metadata remnants. Preserve the current bindings.json-last behavior for ordinary ready payloads. Add an actual effect-law interruption immediately after each root unlink and demonstrate a fresh plan/owner can resume the resulting p-absent metadata remnant. The strict raw intent fence should remain unchanged for p-present input.

This finding describes the source as read before any root response/fix. Root was notified immediately. No correction is claimed by this audit.

## Other Bounded Findings

No additional concrete authority defect was established in the inspected boundaries below. These observations do not seal uninspected shared peer hunks.

- The plan classifier in the same publication file lines170-199 uses physical capsule prefix separately from intent generation. With p present, lines186-187 require exact canonical text, canonical SHA equal to the observed original-byte SHA, capsule digest basename and the exact original generation-slot locator. Lossy UTF-8 decoding cannot silently authorize a malformed raw intent. With p absent, bounded opaque i/r observations issue no historical identity or readiness claim.

- Reference ownership at lines225-261 pins all physical candidate prefixes, retained capsule original generation prefixes and every unknown-but-absent target prefix. The complete actual roster must show those missing targets absent before and after the body. Sources include all current bindings and retained ready metadata's historical bindings; alias digest is refreshed at boundaries. The inert generation-reference predicate must still constrain current closure to current qualified generations; missing targets are only tolerable for inactive historical references. Metadata does not supply Native authority.

- Actual effect admission at lines278-311 validates genuine issued invocation and exact original tools/setup/all arguments, actual whole Native artifact custody and unchanged journal/marker, global bindings EX, ALL current Bun ownership, complete current scopes and fresh selected snapshot. It captures all physical payload entries through first-party relative lineage and validates never-ready roles before effects. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🏷️roles/🟦️.ts:12-23` rejects bindings.json in never-ready payload roots, allows fixed i/r files and p directory, and handles borrowed node_modules links as leaves. The opaque observer exposes no deletion API.

- Collector `.../🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:199-230` reads bounded complete physical inventory and hashes ordinary file bytes, keeps links as leaves and fences final complete names/identity. The effect publication file lines211-222 performs postorder through actual capabilities, exact remaining rosters, same-entry removal and explicit remaining-hardlink nlink/ctime transitions. Progress/checkpoints support cancellation. The capsule root ordering finding above is the material exception to the intended receipt-last protocol.

- Setup retirement driver `.../📤️publication/⚙️setup/♻️retirement/🟦️.ts:16-24,34-60,64-66` obtains a fresh plan, authentic observe snapshot and authentic retire command per candidate, retains the whole charge, rejects cycles/current references, distinguishes capsule physical prefix from intent generation and validates terminal settlement instead of treating DATA as authority.

## Mounted Initial Helper

File: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🎛️owner/🌅️initial/🟦️.ts`.

No additional concrete source defect was established in this helper's inspected path:

- Lines44-52 demand a strict predecessor envelope (at least one declared ceiling differs and neither predecessor ceiling exceeds current), exact current ancestry, coherent backing/receipt, mounted root on a distinct device, complete base/metadata roles and a unique current image/kernel mount association. Equal-current metadata stays refused. Historical bootSession is schema-validated DATA, with no current-boot adoption claim.
- Lines55-65 require actual refined body, accepted Native cleanup scope and exact current writer proof; all ordinary ancestor identities remain fenced. The helper obtains actual control child/transport grants.
- Lines68-74 close receipt/descriptor observations before the single owned hdiutil info child; descriptor roots are opened only after the child returns. No descriptor is intentionally held across that child. The real complete kernel info must succeed with closed output before the inspector is reached.
- Lines75-90 use the mounted storage ImageReceipt schema and current ancestry, an independently rooted metadata namespace, actual Stats-to-extent normalization, closed bounded metadata names and current facts. Each metadata file is recaptured, attested and removed using the SAME fresh entry after preceding unlinks. intent.json is last. The emptied namespace is freshly reacquired from its unchanged parent and removed after its independent receiver closes.
- The defining `.../📤️publication/🎛️owner/🟦️.ts:94` invokes this only in observed mounted publication after actual accepted cleanup enrollment and Native EX writer proof, before ordinary observed-publication capture. Strict predecessor receipt/backing/payload remain untouched, so interrupted metadata-only deletion does not expose an ordinary current envelope.

## Evidence Limits

The reported actual unready13behavior laws, opaque15case pass, descriptor effect-port law over three namespaces and prior strict failures/fixes were read from retained reports, not rerun. The Mac descriptor IO port law models Native issuance and kernel association and therefore is not genuine Native/kernel proof. Final runtime16370 and canonical compiler50197 were pending in root's handoff; this audit supplies no terminal receipt for either. Writer14137's ENOSPC/deadline termination supplies no runtime pass.

Full Source/owning-target verification after correction, genuine cold admission, live canonical publication/funding, directory-anchor qualification and actual layout-plugin end-to-end remain unproved here. Live holds, ticket-open status and active goal remain unchanged.


## Focused P1 Recheck After Root Sorter Correction

The original reachable **process interruption** defect is closed in the current source. Publication owner line219 now computes last(name) from ordinary bindings.json plus root-only schema UnreadyRetirementProtocol.capsuleReceiptLast. Thus the capsule root order is p,i,r, after all p descendants are emptied. The ordinary bindings.json-last behavior is retained. Fresh p-absent plan classification at lines183-189 accepts i/r, r-only or empty metadata remnants as unready-metadata, and selected admission at lines272-274 plus the current never-ready collector/role route can retire those bounded fixed metadata roles without canonical intent/history claims.

I read original normal RED94455 and GREEN95002 logs: RED has0passed/1failed/6expectations and explicitly reproduces p remaining when i has disappeared; GREEN has1passed/0failed/32expectations with actual2048MB admission. I did not rerun either.

The defining new cuts.ts law extracts the current production postorder function, retaining actual first-party relative descriptor operations over unique ticket fixture directories. Its boundary observes actual root unlink deltas, and its four fixture rows cover successful completion plus interruption after p, after i and after r. Expected p,i,r is independently derived by SQLite and neutral fixture grammar checked by Ajv. Each interrupted remaining roster is asserted, and metadata-only roles are checked through the actual defining role validator. Native private caller issuance/control is explicitly modeled; it does not execute a genuine whole Native selected retirement command, fresh plan/retry command or kernel custody. Source inspection supplies the subsequent classification argument; the test does not itself prove full fresh-owner resumption.

### Remaining Durable Ordering Gap

The sorter fixes in-process namespace order, but line219 does not flush the root after unlinking p and before unlinking i/r. Root directory sync occurs only after the entire processDirectory loop at line220. The defining descriptor remove implementation, `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:247` (POSIX) and :389 (Windows), verifies exact unlink/current-entry transitions but does not flush the parent; sync is a distinct method.

Consequently the source has no durable barrier establishing p absence before metadata retirement. A system/storage crash can occur with later metadata unlink issued while p removal has never been durably committed. The generic exact-descriptor API makes no ordering guarantee that would eliminate the original p-present/missing-i remnant after recovery. To claim durable receipt-last recovery, flush the capsule root after p removal and before the first fixed metadata unlink, analogously to the existing pre-bindings.json sync. The current four cuts test process interruption and current namespace visibility, not durability across storage recovery.

This is a bounded remaining correctness gap, not evidence that such a remnant was produced in the live cache. Root was notified. No code changes, tests, Native/frontend/cache effects or live retries ran during this recheck.

Root reports Mac normal16370 completed2tests/5302assertions and ordinary compiler60686 passed before the latest fix; final41640 was pending at handoff. These are contextual receipts, not independently executed or polled here. The original Native/runtime and full owning-target limitations remain.


## Final Focused Durable Capsule Recheck

Both reported capsule defects are now **closed in current Source**. Publication owner line219 retains p,i,r ordering and now executes `if(key===""&&last(name))await actual.sync()` before every root metadata unlink. Because last includes schema capsuleReceiptLast i/r, the first such parent flush durably commits p-directory removal before i can be removed; the next commits i removal before r. Ordinary root bindings.json retains its prior sync/last behavior. A sync failure exits before metadata deletion, retaining either p+i or p-absent fixed metadata, each covered by the defining fresh classification. The final empty-root sync and fresh parent removal remain intact.

I read updated defining cuts.ts:24-28 and original capsule-durable-cuts RED/GREEN logs. The actual-descriptor wrapper delegates sync to the real capability before updating its durable roster and checks that roster before each i/r unlink. Thus this law now catches the missing barrier rather than merely checking sort order. All four current root unlink cuts remain covered, and an independent SQLite predicate rejects the durable p-present/i-absent state. RED46399 records0pass/1fail/6assertions at the missing durable barrier; normal GREEN40399 records1pass/0fail/41assertions after actual2048MB admission. No tests were rerun by this auditor.

No additional concrete gap remains in this narrowly rechecked correction. The observer's durable roster models the guarantees of a successful first-party directory flush; the law does not simulate a physical power loss. Native private caller/control issuance remains modeled, while descriptor IO is real over isolated ticket fixtures. Full genuine Native selected-owner retirement and fresh-owner retry remain pending; this is no live cache or runtime end-to-end proof.

The prior canonical14-root strict41640 pass predates this small correction; root will obtain the final current strict receipt with its other current fixes. No new build, live effect, Native/frontend execution, cache mutation or peer process action occurred during this final audit.
