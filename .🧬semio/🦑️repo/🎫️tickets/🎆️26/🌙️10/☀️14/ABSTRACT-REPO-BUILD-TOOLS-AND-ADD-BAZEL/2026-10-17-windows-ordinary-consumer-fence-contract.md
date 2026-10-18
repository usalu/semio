# Windows Ordinary Consumer Fence Contract

## Existing Interface Constraint

The Native cleanup interface exposes a synchronous `recheck()` callback. The current private Windows observation opens and closes its native descriptors for each awaited readonly call. Its mapped result is sampled DATA and cannot become a synchronous proof after its descriptors close.

Keeping an existing strict readonly snapshot open across a journal mutation is also incorrect: the exact before/after witness checks include the VHD write/change times and mapped-root directory times. Actual authorized filesystem writes can change those fields. Suppressing the failure or calling cached facts would weaken the physical check.

## Proposed Contained Receiver Context

The next unit will add a distinct closed mechanical receiver-read mode in the existing neutral Native System module, issued only by the existing private actual Fixed owner. It is not the PUB accepted phase context, an enforcing port, a reservation, a provider registry or a caller-selected metadata exclusion list.

The mode holds actual host, backing, receipt and mapped-root descriptors while the actual cleanup/journal callback runs. Its synchronous fence samples the actual live descriptors on every call. The exact host and receipt metadata remain immutable. The backing and mapped-root comparisons pin native device, full128 identity, volume GUID/serial, kind, size/extent as appropriate, links, attributes, creation, security and unnamed-stream metadata. Only their write/change timestamps are mutable under the separate actual enforcing/journal ownership; current values are still observed, never synthesized. The current receipt's backing identity already deliberately excludes those mutable timestamps. Fresh samples must remain bounded by the authentic declared filesystem/physical policy.

The actual fixed command owner separately observes the complete disk/partition/device mapping immediately before and after the mechanical callback using its genuine body or captured terminal authority. A mapping object itself cannot issue this context. Callback admission and descriptor work share the already-derived cumulative account and actual requested0 or accepted-cleanup scope, including failure poison, drain and terminal closure. Returned or escaped synchronous fences refuse after the callback ends. All foreign/copy/replaced guard cases retain the existing private checks.

Existing PUB strict readonly mechanics keep complete before/after equality and never receive this mode. The new mode must be nominally distinct at the actual private Fixed dispatcher; no future caller can request arbitrary ignored fields. Ordinary legacy POSIX bodies remain unchanged under additive Windows dispatch.

## Prospective Schema and Source Tests

Schema-first closed purpose, exact mutable field pair, exact allowed descriptor roles, root-only constraint, unchanged cumulative work/descriptor bounds and no budget/grant authority. Portable cases must cover legitimate changed backing/root timestamps, changed immutable native facts, extra stream/security, wrong volume/serial/full128 ID, wrong host/member path, receipt replacement, copy/foreign/closed receiver, same-resource replacement SH, finite cancellation/undefined/frozen primary, synchronous post-close call, unawaited child drain, cumulative overspend and ambiguous handle close.

The same actual System defining function must execute against independent Native syscall ports, with Ajv and SQLite field-set/cost arithmetic. Existing full readonly mechanics must still reject every timestamp mutation. The actual Fixed owner Root0/SQLite/Generated FD worker corpus remains required before enabling any consumer dispatch. Actual NTFS, ARM64, driver sharing and creation/retirement qualification remain separate and must not be inferred from these Source ports.

The public Windows guard remains until ordinary existing/fresh/inactive creation, receipt consumers, terminal/disposable retirement and PUB phase integration are all coherent.

## Contained Mechanical Implementation Checkpoint

The existing System function now accepts only an absent mode or the literal `ordinary-receiver`. The latter requires the already-closed mapped root binding and exact `m` root basename before any native library acquisition. It uses no caller field-selection input. For the exact VHD backing and the exact volume-root role, all facts except schema-owned `writeTime` and `changeTime` compare equal. Actual synchronous `facts()` calls check those immutable fields before returning. Host, receipt and all other roles retain complete equality. The final descriptor fence uses the same relation. Allocated bytes, logical size, links, attributes, security and stream identity remain immutable and may conservatively refuse legitimate changes; this is not general acceptance of writable-state drift.

The initial semantic RED was `backing-writeTime` refused by the old full-equality end fence. The current actual defining function and actual descriptor-owner Source execute against explicit Win32 syscall ports in 28 closed portable cases, classified independently by Ajv and SQLite. Source ports distinguish actual field mutations, including mapped-directory stream metadata; one fixture correction was necessary because the old directory port returned EOF before applying the requested stream mutation.

- `windows-ordinary-mechanics-red.log`: semantic old-policy refusal.
- `windows-ordinary-mechanics-green.log`: genuine parent admission became poisoned after an awaited negative tracked Source operation, so iterating more negative cases in that same Original was invalid. That failure remains recorded. The mechanical model now uses explicit non-authoritative async-operation ports, as the existing observer model does; it does not claim new actual parent lifetime acceptance. Genuine owner/control/FD/SQLite acceptance remains the separately executed 32-worker regression in the mapped checkpoint.
- `windows-ordinary-mechanics-green3.log`: one law/1551 assertions, natural exit 0.
- `windows-ordinary-mechanics-strict.log`: two test Buffer-index possibly-undefined diagnostics; exact non-null annotations corrected them without runtime changes.
- `windows-ordinary-mechanics-strict2.log`: four roots, natural exit 0/no diagnostics.
- `windows-ordinary-mechanics-prior-observer.log`: four unchanged-default observer laws/27162 assertions, including independent Python/SQLite directory bytes and negative strict stamp checks, natural exit 0.
- Final current-byte Source rerun: `windows-ordinary-mechanics-final-source.log`.
- Five-input manifest: `Generated/bazel-planner/windows-ordinary-mechanics-five-manifest.json`.

No current ordinary Fixed dispatcher or PUB context selects the new mechanical mode yet. The actual private owner, command association checks, coherent creation/ordinary/terminal consumers and Native Windows runtime qualification remain required before changing the public guard.
