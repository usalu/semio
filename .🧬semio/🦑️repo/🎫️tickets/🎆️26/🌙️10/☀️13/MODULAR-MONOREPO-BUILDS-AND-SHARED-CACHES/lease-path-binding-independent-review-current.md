# Lease Path Binding Independent Review

Read-only textual inventory on 2026-10-15. No tests, Originals, enrollment, production edits, or implemented-routing claim. Scope: TypeScript under `🧰️framework/🔨️modules/🏃️process` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching`, excluding directories named `🧪️tests`.

Textual identifier counts (include declarations/imports, not executable call counts): `resourceLeasePathV1` 12 occurrences/3 files; `resourceQueueCatalogPathV1` 3/2; `requireResourceLeaseCustodyV1` 16/4. These do not include indirect runtime consumers, ticket fixtures, generated SDK captures, other languages, or aliases without those identifiers.

## Required binding locations

| Actual Source location | Current assumption | Required integration boundary |
| --- | --- | --- |
| Process `🔒️leases/🟦️.ts` 46–50, 101–113, 141–164 | SHA256 logical resource deterministically names the physical SQLite path; private issued custody stores only actual path/signal/liveness | Retain exact logical namespace/resource identity and actual physical binding privately for every issued lease. Resolve/authenticate PIN/GUARD binding before physical open; preserve existing old inode route. A public supplied physical path must not become a forgery port. |
| Same 75 | Custody recomputes hash path and compares it to issued physical path | Routed slots would fail genuine custody. Compare requested logical identity against private issued logical identity, and verify the held physical binding/inode belongs to it. Do not weaken to public resource string/mode alone. |
| Same 207, 214–217, 222–230 | Claim names are hash SQLite names; claim identity implies both physical claim path and `<resource hash>.queue` | Keep queue logical identity separate from physical guard route. Claim census/authentication/retirement must resolve exact enrolled claim binding rather than recompute a hash path. Preserve actual EX claim exclusion and inode checks through unlink. |
| Same 243–249 | `databasePath(resource).replace(.sqlite,.queue)` equals queue derived from `resourceLeasePathV1` | Stable GUARD slots would move this queue to a slot-dependent name and break identity reconstructed at 217. Define one authenticated logical queue binding shared by admission, census, poll, cleanup and release. PIN/GUARD physical filenames cannot silently replace queue identity. |
| Same 255, 261–278 | Foreign-claim sweep and resource acquisition can generic-open/create at computed hash path | Route every acquisition consistently, including poll/create, eligible waiter and release handle. Preserve old already-open handles and existing queue cleanup; prohibit accidental new hash files when a key is assigned a slot. |
| Process `🔒️leases/📦️metadata/🟦️.ts` 8–10 | Canonical catalog EX custody uses direct-hash predicate | It depends on corrected private logical/physical custody; preserve exact existing reserve inode and canonical namespace identity. No authority comes from a candidate slot descriptor alone. |

## Direct external consumers

Repo paths below are relative to the scoped caching root.

- `💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts` 22–28 inspects `resourceQueueCatalogPathV1` with exists/lstat, treating missing/zero physical hash catalog as trigger for exclusive-image quiescent claim retirement. A routed catalog needs an authenticated physical binding lookup; a pure hash helper cannot truthfully report absence. Its claim reacquisition at 25 must acquire the exact same binding read by census before inode comparison/unlink.
- `🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts` 27,30,41 and `🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/📥️physical/🟦️.ts` 27,28,30,42 consume private custody for `nx-tooling-provision`. They need the logical custody predicate to remain valid when its DB is a GUARD slot; they need no caller-selected physical path.
- Process `🏗️build/🎟️capacity/🟦️.ts` 73 uses `resourceLeasePathV1` as preflight queue/path admission; `🏗️build/🗃️state/🟦️.ts` 35 uses it as resource-path preflight. These do not physically read a DB there. Keep logical validation/path ceilings separate from actual bound physical path validation, including real sidecars and queue ticket. Their current pure helper must not be described as resolving a routed binding.

## Bootstrap namespaces remain distinct

Repo opening 18 selects the canonical lease-data namespace inside the finite lease image. Opening 21 opens that image using `budget.controlDirectory` and explicit 16 KiB direct resource DB cap. Opening 36 opens other partition images using the retained canonical data namespace. Those two custody layers cannot be collapsed: routing the lease image's own bootstrap/control guard through that unopened lease image creates a cycle.

Process `📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts` 39–40,63,69–70 and `💾️storage/🧱️fixed/🟦️.ts` 37,46 have ordinary queued image readers plus direct nonwaiting EX maintenance acquisition of the **same** logical image resource. All paths must resolve the same physical inode within their selected namespace. Preserve the separate capped bootstrap/control route when `maximumLeaseDatabaseBytes` is supplied; data-namespace image guards can use the finite binding only with its actual retained image custody.

`💾️storage/🟦️.ts` 79 similarly uses queued `native-storage:<identity>` in its authored leaseDirectory and must inherit that directory's binding, not assume every arbitrary leaseDirectory is canonical data. Other test/private namespaces must not be auto-enrolled merely because they call generic lease acquisition.

## Narrow conclusion

The central required change is a private issued binding containing logical identity plus physical custody. Queues and catalog absence probing are the two concrete consumers that need more than changing `databasePath`. Existing old inode preservation and new stable slot routes must converge at every generic/direct/queued acquisition entry, while bootstrap guards remain independently openable. No actual enrollment is recommended or qualified here.
