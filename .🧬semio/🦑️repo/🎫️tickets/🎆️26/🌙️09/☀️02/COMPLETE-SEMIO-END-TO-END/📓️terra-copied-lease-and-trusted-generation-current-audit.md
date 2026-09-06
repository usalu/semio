# Copied Lease And Trusted Generation Current Audit

Status: read-only current-source audit, 2026-09-06. No product files were
changed and no build was run. Root reported `14082` GREEN for the eleven
fresh-handoff laws and `83439` GREEN for the twelve generation-stage laws.

## Verdict

The copied component lease and the final generation file fence are coherent
byte-owner boundaries. The P0 remaining before a trusted catalog payload is
`trustedBootstrapSourceCodecs`: it reads mutable JSON directly, projects `any`,
and validates neither the source schemas nor the three values that are framed
into a generated profile. The lease is not the blocker.

This report does not claim a browser actor, loader-held bytes, public plan or
lease handoff, Worker containment, or GIS execution.

## Copied Lease: No Current P0

`withFreshComponentLease` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:343-383`
is private and supplies only a frozen one-shot `consume` method. It makes a
bounded separate component copy, checks cooperative control while copying,
wipes that copy on settle, rejects duplicate/late consumption, and drains an
already-started detached consumer. `stageFreshComponentInputs` wipes its
component and descriptor owners in `finally` (`:386-414`); the outer producer
also wipes raw/core and deletes its private work root (`:417-469`).

The fixture covers mutation isolation and zeroing (`:568-592`), thrown/cancelled
derive cleanup (`:593-610`), and detached consumer success/failure drainage
(`:612-631`). `FreshComponentReceiptV1` remains metadata only (`:33-41`). Its
generic `derived` result is not an actor proof: the only current Hub caller
hashes the loan and compares it with the receipt (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:4754-4763`). A later actor builder must similarly compare its own
component digest before any actor record is admitted.

## Current Generation Fence: Fixed

`trustedBootstrapVerifyGeneration` at
`🌎️hub/📦️packages/🦀️rust/📜️script.ts:4637-4670` requires the exact fixed
directory closure, rejects symlinks, retains stable bounded reads for the
bundle plus all four package files, checks each receipt digest/length, then
makes one final checkpoint **before** a synchronous all-path identity pass.
No callback occurs after that pass begins (`:4665-4668`). The fixture asserts
this one-call property and injects a post-read component replacement at
`:4707-4716`.

Receipt path spreading is also corrected: the verifier projects only
`byteLength` and `sha256` from each receipt at `:4651-4654`; the hostile
`receipt-path` row cannot select a file. The verifier now protects first
creation/existing reuse (`:4830-4836`) and rotation before rename
(`:4918-4922`). I found no remaining JavaScript scheduling window between this
synchronous final pass and `renameSync` in the private materializer.

P1 coverage only: add post-read replacement rows for the Stdio descriptor,
catalog bundle, and a package directory. The implementation already fences
them; the present neutral corpus mutates only the GIS component after read.

## P0: Unadmitted Codec Source Can Publish An Invalid Target

`trustedBootstrapSourceCodecs` at
`🌎️hub/📦️packages/🦀️rust/📜️script.ts:4628-4635` reopens the two source JSON
files using `readFileSync` and directly maps `any` values:

* `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/📜️native-codec-factories.json`
  (currently 16,318 bytes), and
* `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🔣️.json` (currently 1,269 bytes).

Neither source is read through `readStableBuildFile`, retained, bounded as an
aggregate, schema-checked, de-duplicated, nor canonically ordered. A concrete
current failure is changing the GIS Map row's `protocolSha256` to `"zz"`.
There are still exactly two rows and both required Map/Terrain kind/schema
pairs, so `:4767-4771` admits them. The target carries `"zz"` at `:4776-4787`;
`Buffer.from("zz", "hex")` silently produces an empty byte field in the profile
framing (`:4496-4523`), and the final file fence correctly publishes those
self-consistent but invalid bytes.

The source schemas already specify the intended fixed closure:

* `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🧬️native-codec-factories.schema.json`
  requires the exact Stdio root and 26 full rows with nonzero lower-case
  64-hex hashes.
* `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧬️.schema.json` requires the two exact
  Map/Terrain records and bounded protocol fields.

### Smallest Repair

Replace the direct reader with private
`captureTrustedBootstrapCodecsV1(repoRoot, control)` before
`profileSummary`/`generationId`:

1. Capture both JSON files once with `readStableBuildFile`, a 128 KiB shared
   budget and 64 KiB per-file bound; UTF-8 decode fatally; wipe the retained
   source arrays after frozen projections are made. Do not reopen either path.
2. Enforce the schemas' exact root/row keys and identity bounds in the
   production capture, including nonzero `^[0-9a-f]{64}$` before *any*
   `Buffer.from(hash, "hex")`. Reject duplicate projected
   `(artifactKind, artifactSchema)` pairs.
3. Sort frozen projections with a deterministic code-unit comparator, not
   `localeCompare` at `:4496`, and use that exact array for both profile
   framing and bundle output. Locale-dependent ordering is unsuitable for a
   generation identifier.

The first neutral rows should be invalid Map hash, duplicate Stdio projection,
unknown field, source replacement after capture, and valid-row permutation
with an identical projected generation. Every rejection must remove the stage
and leave no generation/current pointer.

## Narrow P1s

`TrustedBootstrapMaterializationV1` contains only strings, so a shallow
`Object.freeze` of each materializer/current return is enough; a deep freezer
adds nothing. `Readonly` at `:4606` is compile-time only. This closes a
colocated caller mutating `profileId` or `bundlePath` across candidate/pointer
steps, but is not an external bypass today.

`trustedBootstrapWriteNew` (`:4608-4620`) treats a zero-byte `writeSync`
return as progress and has no cancellation checkpoint. Reject `count <= 0`.
For materialization callers, checkpoint per 64 KiB so existing partial-file
cleanup handles cancellation. This is a durable-progress P1, not an observed
content substitution.

`trustedBootstrapReadRegular` in rotation is weaker than the stable reader,
but currently hashes the returned bytes against the pointer-bound source record
before copying (`:4883-4885`) and the destination receives the final generation
fence. Prefer the shared stable reader on its next refactor; it is not a
remaining altered-byte P0.
