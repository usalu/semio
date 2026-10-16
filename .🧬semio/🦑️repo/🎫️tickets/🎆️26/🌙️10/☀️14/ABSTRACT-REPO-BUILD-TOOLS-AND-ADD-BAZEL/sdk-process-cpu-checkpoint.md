# Current SDK Process CPU Checkpoint

The metadata5803 benchmark is retained in sdk-native-metadata-benchmark.md. Root rejected production lstat conversion as a whole-SDK fix: the4096-row paired wall difference is modest and overlapping, and does not explain clone runs exceeding90s. No production Source was changed.

Ticket cpu-inspector/📜️script.ts now records actual builtin process.cpuUsage deltas separately from wall time at capture invocation and after profiler cleanup. The capture scope includes inspector work; the later snapshot includes profile stop/serialization/write/disable/disconnect. These are microseconds of this actual process's user/system CPU, not sample weights, and wall minus CPU is not an exact IO-wait measurement. It does not account for child processes as if they were this process.

Both SDK90s and workspace240s modes retain their exact actual dispatcher arguments, unchanged parentOriginal120/300, one-shot promise,64MiB profile ceiling and existing task lifetime. A fixed bounded8KiB .cpu.json sidecar is attempted in finally even when profile stop/serialization/write fails. Existing arbitrary primary, including thrown undefined, wins if later profiler cleanup or metadata writing also fails. No extra process or Source instrumentation was introduced for the real SDK capture.

The common captureInspectorProfile function is used by both real preloads and the existing isolated arithmetic probe. Actual probe terminal0 in0.541s produced2177B,116samples,5nodes and the actual profileFixtureArithmetic Source frame;400774µs sample deltas. Trigger wall401.41725ms, CPUuser150097µs/system7587µs; after cleanup wall412.417958ms, CPUuser153114µs/system8531µs. Independent SQLite verified monotonic wall/user/system fields and the CPU µs-to-ms formula157.684ms. Metadata limits and successful profile status were checked. This mechanical probe uses no SDK/tools/native storage and is not a public setup acceptance. No synthetic failure seam was added merely to test diagnostic output; no runtime sidecar-failure claim is made.

The actual fresh SDK diagnostic remains Root-coordinated after High's current retirement Source batch holds and current physical funding is rechecked. No SDK retry was run by Core. Reports must keep old90s profile snapshots distinct from the new Source.

## Current Source-Only Await Map

Capture currently discovers bounded depth layers, uses async file descriptors for hash open/stat/read64KiB/close, yields between chunks, checks each current source ancestor, and retains final rooted/index/control proofs. Clone checks current source parent+row stamp, current destination parent, performs mkdir/copy+chmod/hardlink/typed symlink, then records and validates created identity through the destination tree. It publishes/yields every128rows and rechecks the enforcing store. Source and destination owners each complete exact membership/root proofs; clone then runs projection recheck. Projection recheck traverses actual captured rows again under its original guard; it is not a cached success. This describes call structure only; none is labelled redundant or safe to skip from metadata timing.

Exact current inspected Source hashes (descriptive snapshot, not an authority grant):

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts`: `c8b9fee47068209cb0a48179c5cafd654281bb701f7b110bac9999364755b0a4`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts`: `01a290658bd36f54b1084dd4b04a61e49b86e73d6531fe9c0489ebd731440e86`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/📂️tree/🟦️.ts`: `b11f2db334d0075a566b44438bea7208df14fedf7f16da700756ddda2011d5b3`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🟦️.ts`: `07ee0e0c601609eb4f143410b4a54fa90ed9bcdba472c0b9b1555050c53cd432`
