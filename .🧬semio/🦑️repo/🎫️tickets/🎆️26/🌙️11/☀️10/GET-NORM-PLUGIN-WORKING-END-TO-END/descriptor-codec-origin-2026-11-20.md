# Descriptor codec origin

Process samples of both in-flight Eurocode descriptor jobs showed interpreter steps inside `assemble_codec_origin`. The sampled call was preparing an empty-schema codec origin, before asking for the real structural hash. The owned actor entry points for descriptor and codec both run the same bundle initialization. The general host initializes codec origins lazily for compiled guests and keeps each request isolated on a fork.

The descriptor publisher now requests an initialized private origin from the actor that completed the descriptor. The original compiled handle retains its original cache. The descriptor's next call still runs the same real codec export on a fork, preserving channel admission, schema checks, fuel and absolute deadline. It still refuses missing, empty and unowned exact structural fingerprints as before. The separate `codecs` operation still starts from its regular codec origin. Both CLI paths now emit the active schema, observed fuel and elapsed time. The existing Wasmtime dependency remains only the independent descriptor test oracle.

The focused runtime fixture compares descriptor bytes and actual owned codec bytes with fresh-instance answers, checks that the original compiled handle remains unchanged, confirms lower codec fuel, and verifies a foreign codec refusal. The registered host emitter build runs separately from already-started publishers. Existing publisher jobs continue with the emitter they acquired.

## Verification

The registered native build produced the updated emitter after 572 compiler-artifact records. The updated executable verified the exact 228,740,562-byte component wrapper at receipt `9633bb80c5867558c2467fd0d605382e57ff156c7e55f57c45c55364f55a1f0e`. It finished in 498.6 seconds, including 495.7 seconds and 12,380,513,520 fuel for the descriptor. The actual owned `semio.norm.en1995/v1` codec export then used 2,620,061 fuel in 69 milliseconds. Both canonical outputs matched the already published descriptor byte-for-byte, with semantic SHA-256 `8147ee989a10ed08d53c2693462810751e844adef593399e044d9482013ec97f`. Only the ticket's scratch output was written for this comparison.

The shared emitter artifact remained at the old generation at the time of this check. Native publication exited 75 after a concurrent `framework/Cargo.lock` change during its provenance recheck. The first retry compiled the executable, then exited 75 on a concurrent edit to the owned codec completion registry during its provenance read. A further registered retry also exited 75 when the shared replication canonical-seal source changed during the provenance read. The post-restart build reached Cargo and compiled 245 units, then exited 75 on a concurrent edit to the shared UI viewport binding source. The retry against that source also exited 75 when the shared value byte contract changed. The canonical descriptor stage requires the owned emitter and its exact retained compiler inventory. Its cache key includes the emitter binary digest. The canonical tool retained the previously published generation on those failed attempts. The exact compiled executable was independently verified before retrying. The smaller neutral Rust regression requires the host fixture component and has not run in this checkout.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🧪️tests/🔬️owned-instance-open/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs`
