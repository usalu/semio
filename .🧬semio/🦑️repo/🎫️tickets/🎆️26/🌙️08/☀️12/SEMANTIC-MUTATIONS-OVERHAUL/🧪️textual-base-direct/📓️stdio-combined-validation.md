# STDIO Combined Validation

## 2026-08-27 Initial Integration Run

Command: `bun nx run @semio-tech/stdio-plugin:test-quick -- --no-default-features`

Result: exit `1`. The library compilation stopped after 116 errors and 378 warnings; the test-library compilation stopped after 258 errors and 377 warnings. Nx reported `@semio-tech/stdio-plugin:test-quick` failed. This was the single combined run covering the deferred glTF proof and the JSON RFC8259 Any, XML 1.0 Any, SVG 1.1 Any, and TXT UTF-8 Any direct-leaf cutovers.

The complete Cargo JSON diagnostic streams copied from the exact fingerprints produced by this run are:

- `stdio-combined-lib-diagnostics.jsonl` — SHA-256 `7a0c33230b804a662b711d29a19acc56d3ffe30145a11fc65c6bf2d794080d7a`
- `stdio-combined-test-lib-diagnostics.jsonl` — SHA-256 `aaac9afba2820bb53a1a54c4d4f0251f8d81bfa8f58583aa95a055e1206a4ec7`

Owned repair classes are glTF support/leaf imports and types; missing JSON/XML/SVG/TXT canonical codec metadata; XML/SVG snapshot codec support relocation; and stale JSON/XML/SVG/TXT direct-aggregate consumers. PDF constructor failures are cross-owner and are recorded in `../📓️trinity-jack-rewrite-runtime-blocker.md`.

## Direct Consumer Repair Closure

The shared dependency checkpoint reduced the STDIO library failures from 116 to 31. The owned closure then restored the direct glTF `apply_gltf_mutation` entry point; converted the JSON RFC8259 Any, XML 1.0 Any, SVG 1.1 Any, and TXT UTF-8 Any exhaustive subject adapters to direct `Apply(payload)` leaves; and corrected the authorized RFC8259 I-JSON `lower()` bridge and its direct expectations without changing the I-JSON roster.

The I-JSON bridge maps its retained `NoMutation` to a root `SetScalar` carrying the base value and its retained `SetSnapshot` to a root `SetScalar` carrying the requested snapshot value. All inherited operations lower to their corresponding direct Any leaf. This is the only cross-root consumer edit in this textual batch; the I-JSON mutation architecture itself remains unchanged.

The four exhaustive feature tables now expose the exact direct rosters: JSON `5`, XML `6`, SVG `9`, TXT `5`. The exact executable stale-constructor scan across those four Any roots and adapters returned `0`. `bun 🧪️textual-base-direct/📜️script.ts validate` returned `Ajv descriptors=25 payloads=25 catalogs=4 surfaces=150 errors=0`.

One shared `cargo check -p semio-s-plugin-stdio --lib --no-default-features` was started by the coordinator after both this lane and the PDF owner reported coherent. It exited `0`: `Finished dev profile` in `9m 13s`, with `394` warnings and no errors. Its transcript is `../🧪️stdio-shared-check-after-direct-consumer-repair.log`; no duplicate build was started by this lane.

The four canonical root grammar/protocol specifications were then aligned to the already-compiled generic Rust frame (`<artifact>-mutation payload=<lowercase hex>` and a zero-header opaque binary payload), while visibly retaining every direct descriptor identity and binary tag. SVG's EBNF and ANTLR counterparts were aligned to the same frame. TXT's committed binary mutation fixture was regenerated from the direct `InsertLine(Apply(payload))` serde representation (`84` bytes).

The final ticket validator returned `Ajv descriptors=25 payloads=25 catalogs=4 surfaces=150 rootCodecs=10 errors=0`. Scoped scans returned `0` source `[DEBUG]` markers, `0` sentinel/snapshot-fallback terms inside the four mutation roots, `0` nested `🦠️mutation` owners, and `0` stale aggregate constructors; scoped `git diff --check` passed.

A focused registered Nx runtime attempt was made once after the compiled check. It exited `1` before Nx/Cargo because unrelated concurrent taxonomy state was invalid: `generatorContracts["assets-build"].outputRoots` was not uniquely lexically ordered and tracked output `🧰️framework/🔨️modules/🖼️assets/📃️readme/📝️.md` was missing. The preserved transcript is `stdio-textual-codec-runtime.log`. No STDIO test executed in that attempt, no runtime-pass claim is made from it, and no taxonomy file was touched.
