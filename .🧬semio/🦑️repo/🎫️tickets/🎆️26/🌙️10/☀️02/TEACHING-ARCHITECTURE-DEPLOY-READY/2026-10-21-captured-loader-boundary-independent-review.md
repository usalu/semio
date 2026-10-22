# Captured Loader Declared-Edge Boundary Review

Recorded 2026-10-21T19:26:34.527Z. Read-only static review; no Source edits, runtime Source evaluation, CLI or Native invocation.

## Disposition

CLEAR the reviewed28004/257a218f0a3e3831171acaf3bdaafa000d643f02e1f0618b4879d513e6badbb3. The prior27974/c319d100… observation had removed the static declared-edge predicate. Current peers restored it before this final review. Do not restore a stale whole body or remove the additive diagnostic work.

The exact minimal sound resolution was to restore `||!declared&&candidate!==entry` in the global import onResolve predicate. That restoration is already present at actual Source line61. A captured image is necessary but not sufficient: nonentry imports also need the declaring module import edge. Without this clause, an unmapped absolute/fileURL import of another captured image could execute despite missing declaration.

## Exact Held Comparison

The held27258/c796b9210f10e9084fd6098c766dd498f3949bd84a73c55f04fca53fc4b7dd85 string was retained only in tooling memory from the earlier bounded causal read. Removing only the new diagnostic import and diagnostic closure line, then replacing the three sourceImportRefusal calls with their original Error messages, reproduces the entire held string byte-for-byte and its exact27258/c796b921… hash. Thus the current loader differs from held only by those additive diagnostic changes; acquisition, resolver predicates, CJS/default behavior, import transforms and execution contract are otherwise exact. No Source copy was written to the ticket.

## Guard AST

Actual line59 declaring-module predicate remains `!images.has(path.resolve(name))`; require predicate remains `!target`. Actual line61 import predicate is `!candidate||!images.has(candidate)||!declared&&candidate!==entry`. Parsing the exact actual runtime guard lines as JavaScript yields0 syntax diagnostics and all3 expected sourceImportRefusal call kinds. Import diagnostic receives actual item.path/importer plus declaring-image/declared/target-image booleans.

Both file and stream exported loaders call the same sourceClosureLoaderV1(false/true); guard construction is shared and outside the auxiliary acquisition branch. The new diagnostic module reports bounded closed DATA only, with256Unicode-codepoint clipped context fields and unchanged boolean authority facts. It neither resolves nor admits modules.

## Tests and DATA

Strict Ajv accepts the closed8case fixture and each expected Diagnostic. Cases include known-image/missing-map, missing target, DATA-base-is-not-code, require refusal, escaped fields, ASCII and Unicode bounds. Existing helper law compares output to independent SQLite projection. The second actual-loader law parses both generated loader strings and asserts all3 exact refusal predicates, including restored static-edge condition; this is a structural guard witness, not an actual import execution negative. All5 pinned files parse with0 TypeScript diagnostics. No test or Source was executed by this reviewer.

Detailed bounded nofollow ordinary singlelink pins and exact guard AST facts are retained in `🗑️generated/2026-10-21-captured-loader-boundary-independent-review.json`. Earlier parser audit context pin27974/c319d100… is superseded solely for this reviewed loader leaf by28004/257a218f…. Root still owns focused dispatch/seals and all runtime acceptance.

## Concurrent Later Capture Contract

During final physical pins, a peer changed the same file to28132/e300e4620f63f26a2300670a06d928d3e4e00d633c49306d6782280a348d2cad. It adds an admitted resolution-map capture contract and captured transform metadata, separate from the diagnostic-only28004 review. This reviewer does not claim the entire later capture function is byte-identical to held. The three boundary guards remain exact in the later file; Root owns its independent capture-contract review. The five-file pin record reflects the later file, while the exact diagnostic-only reverse comparison explicitly concerns the retained28004 observation.

## Narrow Current28132 Capture-Map Review

The concurrent admitted-map regions are now independently reviewed and CLEAR statically. Net128bytes over28004 conceals replacement of the old live scanner/schema directory traversal with a closed parent resolution-map consumer. sourceResolutionMapV1 checks closed AdmittedResolutions and AdmittedSources schema, exact canonical absolute root, equal census length, strictly ordered matching path/hash rows, canonical bounded paths and every target present. resolveSourceImportV1 requires an own declaring-map key and admitted target. captureBuildSourceClosureV1 refuses unadmitted entry/images, external node_modules/cachepaths and changed physical bytes, retains byte/file/snapshot/progress/Original checks and performs final requireAdmittedBuildSourceClosureV1. Literal assets and sibling schemas only enqueue already-admitted images; they do not add execution edges or new authority.

The second region derives both ordered physical and resolution metadata from the same validated runtime snapshot and injects them into the captured module transform. The actual authorizer snapshot compiler also defines import.meta.semioCapturedSourceResolutions alongside its existing physical census. No producer/consumer mismatch was found in this narrow check. The three strict refusal predicates remain unchanged. Current whole-file TypeScript parse reports0 diagnostics; no runtime behavior is claimed.

## Final Complete Current Boundary Clearance 2026-10-21T19:30:09.778Z

CLEAR current loader28132/e300e462… with schema14020/8da0c538… and snapshot compiler10349/5329d35b…. All4 diagnostic Source files are pinned, including final test3805/d9d6f9294cd85f0a7ba3bf6f811bad034345f9ec4cf32ad3be602353e5ed4962. Both actual diagnostic laws now have explicit25000ms ceilings; removing only the two `,25000` additions reproduces exact prior3793/8c0489… body/hash. Actual guard AST and shared loader construction remain strict. Closed AdmittedResolutions schema compiles under strict Ajv, accepts finite valid map DATA, rejects extra top-level field and invalid hash.

Seven final bounded nofollow ordinary singlelink pins (loader/schema/compiler+4diagnostics) are in the refreshed generated evidence JSON. Parser-specific clearance and this complete current boundary clearance together resolve the earlier separate loader blocker. No further Source edit is required by this review; all actual runtime/focused/course gates remain Root-owned and unproven until executed.

## Latest Build Main Registration

Current main7122/08714e2d38120af4b01f8c4add2dc35642b8f46a95d53b7c68a114201f9862ef contains the literal diagnostics test import and parses with0 TypeScript diagnostics. Net size is84bytes above prior7038/67e535…, but removing that import does not reproduce the prior hash; exact byte-only attribution is not claimed without the previous full body. Current import resolves the reviewed diagnostic tests; selected focused titles remain explicit. Latest schema14020/8da0c538… is already included and reviewed. Final evidence contains8pins including current build main.

Current main literal-registration semantics CLEAR: AST contains only imports, the existing root initializer and four known build/lease/ticket tests. The diagnostics import resolves exactly to the reviewed two-law file, with no new inline test/function or production execution body. Main parses with0 diagnostics. This clearance concerns the actual7122/08714e… file; no reverse-body or writer attribution is needed or claimed.
