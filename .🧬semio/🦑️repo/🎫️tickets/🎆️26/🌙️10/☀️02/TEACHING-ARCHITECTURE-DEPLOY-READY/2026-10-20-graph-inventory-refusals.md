# Frozen Graph Inventory Refusal Catalog

This is a finite Source-only lookup for the next actual SDK-body error. It does not identify release17's hidden rejection, authorize cleanup, diagnose a hang or propose a repair. No subject module was imported/evaluated, no Source body was copied into this ticket, no graph namespace was walked, and no product/private state was changed.

## Exact Source Authority

The parent-selected graph owner is `library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts`. A bounded actual-private read of three exact files returned:

| File under graph owner | Private SHA256 | Bytes | Host relationship |
|---|---|---:|---|
| `🟦️.ts` |354b890f97621f1a314340350a218aacd76e0e7f976bccd815b386ea42a13f0d|91682|Exact hash/extent equality|
| `📒️journal/🟦️.ts` |45a93a4cea68ff59a4b758d560bc596eadd5576b01981d7331e7755cba20ed01|6216|Host differs: bf51cea7…/6238B; catalog uses actual-private lines9–29|
| `🧬️schema/🔣️.json` |615bee6978d28710f7ccbeaedc860677ba26a580a3fefb19c71ccc8bab8b5cf2|35953|Exact hash/extent equality|

The three-file private metadata pass read133851B, within its explicit262144B aggregate/131072B file limits. One additional6216B journal read emitted only selected lines9–29, making the complete private read extent140067B; no Source body was retained. The journal imports the shared graph schema above, not a separate journal/schema file. An initial guessed journal schema path was absent; it was not substituted or admitted. Only the finite inventory/bytes/check/descriptor helpers, their hardlink receipt reader, adjacent callers and already-audited catch behavior are covered. The unrelated large current-maintenance functions later in the graph owner are excluded.

## Exact Publishers And Ordering

There are two literal `nx:graph-inventory` publishers in the graph owner:

| Location | Publisher boundary | What a publication does not establish |
|---|---|---|
| `inventory`, line131 | Once per namespace key, before key/roster validation, directory receipt read, descriptor validation and `bytes` | A valid key, receipt or completed inventory |
| `bytes`, line129 | Every16 visited entries, after current control, lstat, ancestry and entry-count checks, before processing that entry's children/file allocation | Entry kind/allocation acceptance, byte-cap acceptance or walk completion |

Both await the issued control's publish and yieldContinuation and then check again. The resource does not name the namespace, entry ordinal or caller. Repeated identical markers can originate from different inventory calls or either publisher. No separate completion marker exists in these two functions.

## Inventory And Namespace Receipt Refusals

All messages below are fixed Source literals. Unless a specific exception is noted later, they propagate through this body.

| Exact message | Predicate / phase | Source location |
|---|---|---|
| `Closed Nx graph toolkit directory required` | An immediate name under `root/nx/graphs` is not20lowercase hex characters; before that toolkit's namespace publications | graph131, inventory |
| `Original Nx graph toolkit ancestry required` | Its toolkit row is not a directory or is a symlink | graph131, inventory |
| `Bounded Nx graph namespace roster required` | After the per-key publication: key is not20lowercase hex, or the accumulated namespace roster already has64rows | graph131, inventory |
| `Original Nx graph namespace ancestry required` | Selected namespace row is not a directory or is a symlink | graph131, inventory |
| `Original Nx graph namespace receipt required` | `.namespace.json` is not a regular file, is a symlink, has link count other than1, or exceeds2048B | graph131, inventory |
| `Original Nx graph namespace identity required` | Namespace schema fails, workspaceRoot is not absolute, or resolve(root) differs from root | graph87, descriptor; invoked by inventory131 and namespace path derivation |
| `Original Nx graph namespace receipt changed` | The receipt dev/ino/nlink/size/mtimeNs/ctimeNs stamp changes, or the canonical namespace directory derived from its parsed identity differs from the observed directory | graph131, inventory |

The closed Namespace schema requires version1, toolingIdentity and implementationIdentity as64lowercase hex, graphIdentity as null or64hex, and workspaceRoot length1–256. The path's SDK component uses the first20tooling hex characters. Its namespace component is the first20SHA256 characters of the serialized normalized-workspace-root/graphIdentity/implementationIdentity tuple. This identity mapping is not a directory-name repair rule.

Missing paths and directory/read failures throw their original filesystem errors. Malformed `.namespace.json` throws JSON.parse's original SyntaxError. These cases are not converted to a fixed inventory message.

## Namespace Byte Walk Refusals

| Exact message | Predicate / phase | Source location |
|---|---|---|
| `Original graph namespace physical ancestry required` | The lstat row is a symlink or its device differs from the Store root device; before the next cadence publication | graph129, bytes |
| `Bounded Nx graph namespace inventory required` | Visited entry count exceeds4096, including directories/files; before publication for that entry | graph129, bytes |
| `Regular owned Nx graph namespace file required` | A nondirectory row is not a regular file | graph129, bytes |
| `Bounded Nx graph auxiliary storage required` | A nonexempt auxiliary file exceeds67108864B, or accumulated auxiliary allocations exceed134217728B | graph129, bytes; the same literal is also used by the adjacent auxiliary-admission caller at188 |
| `Nx graph namespace byte limit exceeded` | Total is not a safe integer, total exceeds939524096B, or steady allocation exceeds536870912B | graph129, bytes |

Single-link files are charged max(blocks×512, ceil(size/4096)×4096); other links enter the direct journal allocator below. Total and steady charges deduplicate dev:ino separately. Steady excludes names ending `~<16hex>.prior` or `.next`. Auxiliary exemptions are `.namespace.json` and the exact project-graph/source-maps/file-map JSON names, including their prior/next variants. The walk subsequently adds a present sibling native/fixed.dmg allocation to returned total/steady values; that final addition has no new fixed refusal at the addition itself. Callers still consume the returned values. These statements describe current accounting, not a new policy or a demonstrated defect.

## Direct Hardlink Journal Reader

This path is entered by graph129 only when the file's link count is not1. Its actual-private Source, rather than the drifted host file, is authoritative.

| Exact message | Predicate / phase | Actual-private location |
|---|---|---|
| `Graph publication allocation refused` | Allocation row is not a regular no-symlink file, or nlink is outside1–2 | journal25, allocation |
| `Graph publication hardlink journal required` | A two-link file has no sibling publication.json | journal25→journalRead21 |
| `Original graph transaction scope required` | The physical byte reader has no current privately installed check function; public allocation wrapper installs the supplied control check | journal10/28 |
| `Closed graph publication file required` | A receipt/pair row is not a no-symlink regular file, has a wrong exact link count, or exceeds the passed byte maximum | journal17, regular |
| `Graph publication file changed` | Path/fd stamp differs before reading or after reading; stamp includes dev/ino/size/nlink/mtimeNs/ctimeNs | journal19, bytes |
| `Graph publication file truncated` | readSync returns0 before the captured extent is consumed | journal19, bytes |
| `Closed graph publication journal required` | The closed GraphPublicationJournal schema rejects the parsed value | journal20, journalValue |
| `Complete graph publication journal required` | Duplicate names, absent project-graph or source-maps row, or a non-preparing phase with null next identity | journal20, journalValue |
| `Graph publication prior pair required` | The queried path is not a journal row's canonical/prior path with a nonnull prior fact | journal25, allocation |
| `Graph publication hardlink identity refused` | The canonical and prior pair do not share dev:ino, or that identity differs from the journal's prior identity | journal25, allocation |

Journal byte reads are capped16384B; canonical/prior files are capped536870912B and must each have exactly two links for the paired case. The schema permits2–3distinct rows selected from project-graph.json/source-maps.json/file-map.json, version1, a16hex generation, preparing/publishing/committed phases, bounded bytes and identity/hash facts. An invalid JSON document propagates its original SyntaxError. A close failure is thrown only without a primary read failure; otherwise the reader attempts to attach graphPublicationCloseErrors to the original Error. The allocator validates the prior pair's identity; it does not perform a full snapshot payload-hash read in this path.

## Immediate Caller And Failure-Propagation Lookup

| Caller / message | Meaning and catch behavior |
|---|---|
| `check`, graph125 | Checks actual control, privately issued Store membership, original storage reservation and issued operation control. Its own fixed messages are `Original enforcing Nx graph pool required` and `Original funded Nx graph store required`. Lower control/storage errors are outside this catalog. |
| `room`, graph144 | Runs inventory, computes capacity, then tries idle namespace retirement. Only the exact `Resource lease timed out: <namespace resource>` from its zero-timeout exclusive lease is skipped. Other errors propagate. A changed retirement receipt stamp throws `Original retired Nx namespace identity changed`; exhausted admissible room throws `Funded Nx graph cache capacity exhausted`. |
| `acquireNamespace`, graph149/153 | Runs inventory before matching existing identity/room and before shared lease/native child creation. Its adjacent mismatch is `Nx graph namespace identity collision`. On failure it awaits child?.close, attempts lease?.release, suppresses those cleanup errors, then rethrows the original value. That await may delay outward error visibility. |
| Publication-capacity caller, graph169 | Runs inventory before checking `Registered Nx graph publication namespace required` and `Nx graph steady publication limit exceeded`, then room/rechecks/body/bytes. No inventory error normalization is present in this body. |
| Auxiliary caller, graph188 | Runs inventory, applies the same bounded-auxiliary refusal literal, then room/rechecks/body/bytes. No inventory error normalization is present in this body. |
| Pool admission, graph140 | Preserves and rethrows the primary value. Its final lease.release failure is attached as graphPoolReleaseError when possible; it replaces success only when no primary exists. |
| Store owner, graph83 | Preserves the primary through awaited child-reader closure and lease release. Without a primary, cleanup failures can throw an individual error or `Graph namespace acquisition and close failed`. This does not turn an inventory refusal into success. |

Runtime's separate frozen outer audit is `2026-10-20-course17-body-unwind.md`: the SDK reader preserves the exact body failure through final Store/index checks; toolkit-use witness/server cleanup precedes its body-rejected observation. Bootstrap's runWithPreparedToolingStore translates only an actual local SIGINT/SIGTERM flag and otherwise rethrows. No generic normalization/cache-miss swallow was identified in that finite audit. Wider awaited control/Store finalizers can delay propagation; neither this catalog nor the resource name identifies the active one.

Release17's actual record remains: graph-inventory publication precedes call3 body-rejected, then final-recheck-before/after; the failure value was not exposed. The run was deliberately cancelled after the quiet interval. No fixed message in this catalog is assigned as that cause before actual failure metadata is observed. No retention/cache deletion or guard/policy change was performed or authorized by this lookup.
