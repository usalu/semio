# Node CString And Journal Read Boundary

Read-only Source evidence; no profile, Native, compiler or test execution.

Node runtime native/system/TS nodeNativeResultV1 maps every declared cstring return to cstringAt. That helper reads exactly one byte through core.peek per iteration, allocates a Uint8Array wrapper, pushes the byte into a JavaScript array, then creates a Buffer and decodes UTF8 at the first NUL. For an L-byte string, this is L+1 synchronous addon crossings, L+1 copied ArrayBuffers and wrappers, L pushes, and a final full copy/decode. The protocol limit is1,048,576 bytes. Failure to find a terminator requires that many crossings before nativeLength refusal.

Native core Rust peek at242–257 calls napi_create_arraybuffer for every request and memcpy for its full length. Thus a one-byte scan does not merely perform a cheap JavaScript pointer read. The existing native core also exposes view(address,length), a zero-copy externally owned ArrayBuffer, and peek supports a supplied bounded range. Neither supplies a bounded NUL-terminated string primitive. Reading a guessed large range risks crossing the actual allocation boundary even if the terminator is early; a fixed chunk overread is not justified by the current string return contract.

The existing one-byte helper preserves stop-at-NUL semantics. A performance repair should retain that semantic boundary and the existing finite string limit while reducing crossings; it must not infer readable extent from a pointer alone or use unbounded CStr scanning. Registered return conversion is generic, so large JSON return strings would amplify this cost, but actual journal return length and invocation counts were not measured by this review. No causal attribution beyond the Source operation-count bound is claimed.

The filesystem journal public witness includes epoch,sequence,globalIdentity/selectedMembership path vectors+SHA and changes; its existing maximumPaths and maximumEvents are caller policy. This establishes potentially repeated path serialization, not the exact Native return shape. Bootstrap was asked for the precise profiled FFI symbol/producer to complete byte bounds without broad assumptions. Original resource-loan deadline refusal remains a real terminal result, not evidence that the deadline guard should change.

## Correct Profiling Process

The fixture runtime loads the SDK client with createRequire in the issuer at48, reflects its Original environment at62, and constructs DaemonClient at71 before getProjectGraphAndSourceMaps at72. The immutable SDK patch wraps client startDaemonIfNecessary in withNxDaemonStartupV1 in that same process. Startup Source admits nx:daemon-startup and executes receive→tooling store→service birth→lease before invoking the client callback. Any Native journal replay during those store/birth steps executes in the issuer process; loading bundled authorizer bytes does not move it to the daemon.

The separately qualified Node child is started at65 and its stdout/stderr are redirected into daemon-output-service.log at66. Therefore nx:daemon-startup progress visible directly in the Root CLI log is evidence of the client-side owner, consistent with the reported terminal trace. Profiling only the Node child cannot capture that issuer path. The existing closed CPU request must also be applied at the captured Bun issuer launch boundary, analogous to retained framework-profile.cjs, preserving separate issuer/daemon PID and profile identities. The daemon can still perform its own later Native work; this does not claim every replay belongs to the client or establish CString causal CPU costs. No profiling or Native execution was launched by this review.

## Progress Units And New Profile Boundary

Actual journal read43763 is bytes; replay20115:64 is bytes/events, not row counts. CString conversion cost must be tied to actual returned UTF8 bytes, not these assumed row counts. The later actual25826 profile stopped on finite slot saturation and did not reach the critical client-startup phase; its profiles must not be claimed as reproduction of63471. This correction was sent to Bootstrap.
