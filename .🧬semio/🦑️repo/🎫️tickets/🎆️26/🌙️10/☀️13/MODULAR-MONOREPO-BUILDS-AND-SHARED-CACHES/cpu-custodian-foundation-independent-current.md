# CPU Custodian Foundation Independent Audit

Source-only review, 2026-10-15. No probes, tests, processes or images. Scope is the direct-child foundation, not Cargo integration or general descendant enforcement.

## Actual Paid Ownership

`process/🏗️build/🎟️capacity/🛟️custodian/🟦️.ts` launches only the custodian locally. Actual declared compute jobs are spawned by that custodian after it acquires the existing real gate/core reservation with environment={} rather than accepting inherited markers. Each job increments paid concurrency before its accepted operation and refuses sums above the held reservation. Requests are single-use by index. The server strips inherited count/held flags from job environment; the public client port only sends closed run requests and cannot spawn a local compute fallback.

On client disconnect/EOF, cancel or original signal abort, server stops admission and aborts its owned captures. It waits all accepted capture promises, whose owned runner waits actual direct-child close, before releasing capacity and reporting settled. Because the paid lease belongs to the dedicated custodian process, death of the requesting parent does not itself release the cores while the direct child remains alive. This does not protect arbitrary grandchildren or survive killing the custodian itself; neither is claimed.

Client port.run is registered withIssuedResourceOperation before sending. The client callback return triggers drain, joins actual custodian close, and the enclosing issued control drains accepted request promises. Callback failure uses an explicit boolean, preserving thrown undefined ahead of a cleanup failure. The same serialized original deadline/cancellation is carried into the actual custodian receiver; there is no new Source-root birth in the production client. Idle/await-ready finite handling uses the original receiver's actual deadline timer and cancellation.

## Closed Bounds

Plan validation limits serialized UTF-8 to 1 MiB, jobs to 32, arguments to bounded arrays, and program/cwd/path strings to 256 characters. Declared coverage must be direct-child. Schema validation and exact IPC handle ownership are separate: the wire content is a declaration, while genuine capacity is acquired by the spawned server. Responses/results are closed and bounded by owned capture output limits; diagnostic stream retention is a bounded 65536-character tail, not a claimed 65536-byte cap. No new global discovery namespace was added.

## One In-Progress Original-Semantics Correction

At inspection server accepted work caught capture failure and sent refused without rethrowing, so its own actual Source Original could settle successfully even though the requesting client's registered port operation correctly fails. CPU owner independently identified this and is correcting it by sending refusal then rethrowing the same error inside the registered server operation. This does not create an observed early capacity release—the server already drains before release—but retaining failure in both actual Original ledgers is appropriate. Final Source confirmation is required before marking this point closed.

## Test Meaning and Limits

Current test Source invokes actual declared child processes and independent Python SQLite core probes, tests blocked competing capacity, returned/detached/duplicate requests, cancellation, and a stopped custodian during requester death. The latter checks that cores remain busy until actual direct child settlement rather than merely eventual EOF. A forged result-shaped request is rejected without local compute fallback. Negative actors catch only outside their full receiver scope, preserving failed Original outcome. The finite-deadline actor explicitly rejects inherited envelopes and authors its finite test root before custodian birth; its observer does not create a replacement compute grant.

These are meaningful runtime test designs. Final runtime/strict results are pending elsewhere and were not run or independently verified here. No foundation result closes the existing Cargo inherited-marker bypass, proves actual job thread counts, covers every platform backend or establishes general descendant custody.
