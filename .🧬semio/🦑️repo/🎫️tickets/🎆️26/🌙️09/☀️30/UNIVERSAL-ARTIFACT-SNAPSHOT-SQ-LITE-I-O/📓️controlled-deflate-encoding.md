# Controlled Deflate Encoding

## Contract and Scope

The first-party raw Deflate encoder owns a domain-neutral `DeflateEncodeControl` interface with cumulative `admit(bytes)` and fallible physical work checkpoints. Progress distinguishes initialization slots, known input bytes, candidate-search units (unknown total zero), produced bytes, and finalized bytes. The implementation must retain the ordinary encoder’s fixed-Huffman, greedy hash-chain output exactly; it introduces no Value dependency or runtime library.

The neutral fixture and strict JSON Schema live in framework/deflate/fixtures/controlled and schema/controlled. Tests compare every encoded byte with the ordinary encoder, independently decode using miniz_oxide and Bun node:zlib, prove the fixed-Huffman capacity ceiling, cancel inside each physical phase, and test exact cumulative admission boundaries. Ajv strict validation and JSON parsing of the project and both launch registrations actually passed on 2026-10-02.

## Physical Design Evidence

[RFC 1951](https://www.rfc-editor.org/rfc/rfc1951) defines fixed literal/length Huffman lengths, distance codes, bit packing, lengths through 258, and distances through 32768. The existing encoder uses those exact tables. Every literal costs at most nine bits. Exhaustive length/distance tests establish each match costs no more than nine bits per represented byte. The three header bits and seven-bit end-of-block symbol therefore permit a reserved output ceiling `(9*n+17)/8` bytes.

The controlled producer must admit owned buffers before reservation and initialization, use checked size arithmetic, checkpoint inside candidate probes and byte comparisons, and bound initialization and emission work. Stack tables have no heap allocation; owned code tables, hash heads, previous positions and output capacity require explicit admission.

## TDD and Registration

A strict-refusal `deflate_controlled` stage was mounted before implementation. The registered `@semio-tech/framework-rs:test-deflate-encoding` invocation is running against three native neutral laws, with `SEMIO_TEST_LEVEL=quick` and the existing warm task-owned target/build directories. Its current queue is an infrastructure prerequisite, not a feature RED or passing result. Logs are retained under generated/deflate-controlled-native-red.log.

The permanent command extends framework’s existing Rust script. The project target calls that script, and both `.vscode/launch.json` and `.claude/launch.json` expose the same command. No extra script file was created. The replication adapter and Pack output framing remain owned by the I/O executor; their output dispatch is not activated by this work.

## Authentic Runtime RED

Nextest `ed4ed0e3-4dfd-49c9-b1d2-04adfedd0c25` actually selected and executed all three neutral laws: the exhaustive fixed-Huffman capacity proof passed, and both controlled positive/cancellation laws failed on the explicit missing producer refusal. Assertions took 1.710 seconds; registered Nx took 1 minute 24 seconds. Nineteen unrelated laws were excluded by the focused selector, not silently counted as passing.

## Implementation Stage

The controlled sibling retains the ordinary fixed-code assignment, greedy longest-match choice, 128-candidate chain, 32 KiB window, and insertion of every consumed input position. It admits `4*(32768+n)+2*(288+30)+ceil((9*n+10)/8)` heap bytes cumulatively before any producer reservation. Canonical count/next-code arrays use fixed stack storage; position and code arrays initialize through real 256-slot callbacks. Every candidate and byte comparison contributes to bounded search work. Input insertion publishes exact 256-byte progress; emitted-byte checkpoints use actual output length and the proved maximum. Checked arithmetic and the u32 position sentinel reject unrepresentable inputs before allocation.

An additional actual allocator law observes the producer under denied admission and initialization cancellation, asserting no allocation reaches 1024 bytes before refusal. This uses the same neutral admission and cancellation properties and preserves the independent zlib laws. A current registered green attempt is still pending graph admission; no passing implementation result is claimed yet.

## Verified Controlled Producer

Nextest `704e897c-b331-4c6e-976e-eb7cc30edc89` actually executed all four focused producer laws and passed all four (2.325 seconds assertions, 27.6 seconds registered Nx). This includes the independently decoded exact streams, exhaustive capacity proof, real interior cancellation/admission, and allocator observation. Nineteen existing non-selected Deflate laws remain separate; a full `--lib` regression is running. Its passing status is not inferred from the focused result.

The focused run accidentally used a misspelled generated-artifact directory in its environment. Its single owned Nextest trace directory was moved into this active ticket’s generated directory, and only the newly created empty mistaken directories were removed. Source, fixtures and reports were untouched.

## Full Regression and Ownership Handoff

Nextest `fb2e9524-a4b1-4a02-a16c-48529b0d5b6e` actually executed all 24 Deflate library laws and passed all 24 with zero skipped (1.224 seconds assertions, 28.4 seconds registered Nx). This includes existing inflater, ZIP and miniz differential laws, plus five controlled encoder laws. The fifth observes the exact output-capacity allocation and proves initial WriteOutput refusal precedes that reservation. Two unnecessary `size_of` qualifications were subsequently removed as the compiler requested; the fresh default five-law invocation verifies that narrow cleanup.

The API and full runtime proof were sent to the I/O executor for its independently owned cumulative NativeEncodeControl adapter. This does not establish Pack document framing or erased output dispatch completion.

Authored files: framework/deflate root Rust producer; controlled neutral JSON Schema and fixture; controlled Rust test facet and its existing unit mount; framework Rust package script/project target; `.vscode/launch.json` and `.claude/launch.json` exact command registrations. No ordinary encoder behavior, runtime dependency, cache policy or timeout policy was changed.

The final default invocation after qualification cleanup is actual GREEN: Nextest `0f866185-62ea-4184-a90d-dc95267914aa` ran all five focused laws and passed all five (0.559 seconds assertions, 16.3 seconds Nx), with no qualification warning. The full 24-law proof remains valid apart from that nonbehavioral compiler cleanup. The single native lane has now returned to the qualified Semio reader and literal-reference baseline.
