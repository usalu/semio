# Parent Worker Shutdown and Exact Private Recovery Source Implementation

The private parent fixture now settles its actual stdin async generator before emitting the closed acknowledgement. Both Native worker and Source worker use the same framing and input-return functions. Primary values including undefined/frozen objects and distinct iterator-close failures retain their identity/order. The parent waits for actual stdout EOF after the closed acknowledgement before leaving the abortive owned-channel scope, then still requires actual captured status0 and null signal. Native workers neither explicitly close stdout nor spawn a descendant after acknowledgement; Source AST laws enforce these closed-protocol preconditions. EOF is never accepted as generic terminal proof. Generic process execution/channel semantics and Native/storage implementation sources are unchanged.

The Native path saves bounded actual held.capture to own fixture reader-terminal.json and prints it with DEBUG before the existing status/signal assertion. Captured stdout/stderr retain the existing65536-byte/256-line cap. No capture is fabricated if the owned call rejects before returning.

## Test-Driven Runtime Evidence and Limits

- Root's earlier physical run46364 failed at Actual reader terminal/drain required after the application closed acknowledgement. Root retained pf-q157nV and reported parentReaderClosed:true, original allocation5024e08c20c0eefcfc42bdebcd7c0e45. That old physical capture status/signal was not logged and remains unknown. The Source signal below must not be attributed to that physical failure.
- The initial actual Source negative control used a50ms postack delay and immediate callback return: capture status:null/signal:SIGKILL. The new parent awaited EOF from the same real worker and captured0/null. Later50ms immediate-return attempts naturally exited0 before forced close and correctly failed the required-negative-control assertion. Those are inconclusive timing/harness attempts, not accepted negative proofs.
- The durable negative control therefore uses an explicit Source-only pending-work fault port after the closed acknowledgement, waiting on the actual finite10-second control signal/deadline timer. It cannot naturally exit0 before forced owner closure. The normal graceful Source worker retains its50ms delay. No delay or pending-work fault was added to the Native worker; Original60-second Source and120-second private/recovery limits are unchanged.
- A first in-process loop correctly failed because cancelling a descendant closes Original process admission for further work. Cases now run in actual inherited owned check processes within the same root Original window. No Original is renewed, and no control/capability/native authority is fabricated.
- Final standalone --source terminal0 under the genuine60-second Original: six actual worker checks followed by **11pass/2intentional worker skips/0fail/922assertions/31.31s** in the existing permanent parent law file. Historical preworker nine-law gate was9pass/706assertions; it did not qualify physical worker exit. Intermediate worker-only extension was10pass/784assertions/27.80s.
- Final standalone --strict terminal0 (exec session92212): same six real worker checks, then strict TypeScript over the existing eight authored roots, including the new actual caller/closure laws and JSON imports. An earlier strict attempt correctly failed two self-referential test parameter type names; those own annotations were fixed. No diagnostics are suppressed.
- Scoped diff whitespace check terminal0. Native/protected journal reads, physical recovery/run, mounting, cache mutation, child adoption and parent retirement were not performed by this implementation session.

The final strict preflight produced these actual observations. Refused operations do not return a capture; the proof there is exact expected primary plus actual child PID absence after the owned promise joined/killed/drained.

| Case | Actual trace | Actual capture | PID absence after join |
| --- | --- | --- | --- |
| actual-immediate | ready → closed → terminal | status=None, signal=SIGKILL | true (pid93985) |
| actual-eof | ready → closed → eof → terminal | status=0, signal=None | true (pid94022) |
| actual-cancel | ready → closed → refused → terminal | Unavailable after genuine refusal | true (pid94042) |
| actual-expiry | ready → closed → refused → terminal | Unavailable after genuine refusal | true (pid94069) |
| actual-undefined | ready → closed → refused → terminal | Unavailable after genuine refusal | true (pid94209) |
| actual-frozen | ready → closed → refused → terminal | Unavailable after genuine refusal | true (pid94240) |

Nine portable closure cases execute the exact actual helper input-return body with explicit iterator-close fault ports; independent Ajv/SQLite verifies arbitrary primary and cleanup ordering. The actual Source preflight runs genuine workers with the shared functions. Both actual caller/worker ASTs enforce no stdout end/destroy/close and no postack child spawn. SQLite independently verifies acknowledgement/EOF/terminal ordering from the neutral trace corpus. These tests qualify the closed fixture protocol only, not generic EOF-as-terminal semantics.

## Narrow Canonical Recovery Entry

The existing helper adds --recover pf-q157nV. Existing generic EN/DE launch entries already select parent-factory and accept explicit helper arguments, so this mode requires no new script/launch wrapper. Internal worker/check modes require the actual inherited Original and are not standalone launch routes.

Only own saved creation input was read, bounded to4096 bytes through nofollow descriptor checks; no protected Native state was read. The actual687-byte created.json SHA is456ce7ac8b8c0868002d747d0aa90a6e7aa960f718908c788c89ce066b301f10. The recovery input pins that exact name/hash, full closed allocation/backing DATA, and full original request protocol. It independently derives the current parent policy/child request SHA before any Native open. No directory prefix, owner string, old token or historical DATA becomes Native authority.

Recovery authority remains a fresh ordinary canonical parent handle and canonical recovery of the exact original allocation, with all request/backing/policy fields checked by existing parentFixtureCleanupV1. The recovery verification captures a fresh current physical request under actual image operation control, opens only the known child, checks its authored65536-byte payload using the bounded independent Node trace, captures the current raw journal, and rechecks current parent CAS. There is no saved historical raw-journal SHA or historical physical request, and no such history is inferred. Then existing canonical child retirement, zero allocation/charge, complete close and exact current-image parent retirement must all succeed. Failed input/physical custody/provenance is retained; there is no raw filesystem removal. Successful receipt remains in the outer own fixture directory alongside original input.

The language-neutral recovery corpus has31 cases (one ordinary,30 refusals). Exact actual declaration and caller bodies execute with genuine Original/control and explicitly modeled Native/trace/cleanup effects. Every changed saved field, name/SHA, request protocol and policy refuses before model effects. Independent SQLite json_tree symmetric difference compares all known facts and request fields; Ajv checks the closed corpus; actual source snapshot validation uses the canonical Native schema. Positive modeled order is fresh ordinary parent → exact recovery → payload verification → child retirement → zero charge → fresh close → parent retirement. The existing exact parentFixtureCleanupV1 Source laws separately retain its real borrower/lease/failure closure tests. No Native recovery acceptance is claimed.

## Settled Input Manifest

Hashes below bind the Source-only review checkpoint. Root alone coordinates later actual Native recovery/fresh full proof after independent review. Bounded generated worker evidence is parent-worker-source.json under ticket Generated; preserved failed physical input pf-q157nV remains untouched.

| Path | Bytes | SHA256 |
| --- | ---: | --- |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/parent-factory/📜️script.ts` | 37522 | `2a164c920bb20c9f9002cca8c663ec6c7c7db2e6f76342d9dd75175690506871` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/parent-factory/worker.schema.json` | 2569 | `2e91c0a779a49ce86c02805f730d92619c22fff3c775a84abe1d14345dff94bb` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/parent-factory/worker.fixture.json` | 2159 | `186687979918e6516febe9a1a7978114be8b09ea5308e0b13f59fc4fdab1ede9` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/parent-factory/recovery.schema.json` | 3296 | `b9fb5f9a5e9f95f6084b33df03555a68ff6d15178808531ee7dbf08de6b5b79d` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/parent-factory/recovery.fixture.json` | 3844 | `b30f6e728b1bc6acb539218b2fcb63292664da06f59f0615bad1d3fc3a95c00b` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/📥️parent/🟦️.ts` | 102685 | `96b6988dd9b64fb50d1e72fa8c79b8694f1bb0b1b866a9dac4756becd3d0ea90` |
