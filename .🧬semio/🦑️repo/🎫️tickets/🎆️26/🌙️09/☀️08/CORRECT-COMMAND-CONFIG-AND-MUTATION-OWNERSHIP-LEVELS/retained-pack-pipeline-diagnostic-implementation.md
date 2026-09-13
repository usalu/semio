# Retained Pack Pipeline Diagnostic Implementation

## Accepted Boundary

The retained anchor, segment, retained-varint and retained-DEFLATE public grant paths now use `PackError::RetainedMalformed { what: &'static str, offset: u64, detail: &'static str }` for malformed hot faults. Each owning cursor keeps its first `PackError`, returns the same inline value on every later grant and rejects later ingress. Close preserves the fault while retiring the cursor's existing logical state.

Cold `PackFile`, identity-chunk, byte-reader and one-shot DEFLATE APIs retain their ordinary owned `Malformed`, `Schema` and `Io` diagnostics. Generation2d and Generation3d snapshot and mutation sessions keep their four existing static mappings and do not format a nested Pack error.

## Owner Behavior

`RetainedPackAnchorCursor` stores a first-fault slot around its single public grant boundary. A malformed non-contiguous event leaves the inline anchor state governed by that fault; later events cannot advance it. The native law couples this cursor to an admitted source owner and proves exact 9,480-byte source backing release after rejection.

`RetainedPackSegmentCursor` applies the same boundary to direct segment faults, retained-varint faults, checksums, limits and delegated retained-DEFLATE faults. Its preflight rejects ingress once faulted. The native laws cover an overlong ten-byte varint and reserved segment flags, repeat the first error and reach the existing terminal close.

`DeflateRetainedCursor` stores its first fault, makes `can_admit` false after rejection and returns the same fault on later grants. The native law decodes 27 real bytes using the existing independent cold compressor, forces a declared-length mismatch, rejects another input byte and reaches logical terminal close.

## Schema and Oracle

The Pack schema and fixture add `retainedPipelineDiagnostic` for the four stages. It requires inline storage, zero diagnostic heap bytes, no public-step allocation, sticky first fault, rejected later ingress, bounded logical close and the explicit `existing-lower-owners-only` physical scope.

Registered target `abstraction-ownership-validation:framework-retained-pack-diagnostic` passed Ajv 2020, strict TypeScript, fast-json-patch and the platform UTF-8 oracle in 15.9 seconds. Durable evidence is `🗑️generated/retained-pack-pipeline-diagnostic-neutral-1.log`.

## Native Red and Green Evidence

The first registered native run reproduced both owned hot variants before the production change:

- retained varint returned `Malformed { detail: String }` instead of `RetainedMalformed`;
- retained DEFLATE returned `Malformed { detail: String }` after decoding all 27 bytes;
- the failing segment law then reached its Drop guard because the expected sticky close path did not exist.

That run exited 1 and remains in `🗑️generated/retained-pack-pipeline-diagnostic-native-red-1.log`.

The final registered target `abstraction-ownership-validation:framework-retained-pack-diagnostic-native` exited zero in 17.3 seconds. It passed:

- three compact diagnostic laws for anchor, segment/varint and retained DEFLATE;
- the existing real codec-1 retained anchor/segment/catalog resumability law;
- the existing hostile anchor CRC and explicit-close law.

The run observed exact source allocation/release of 9,480/9,480 bytes and explicitly records the current retained-DEFLATE backing as unaccounted. Durable evidence is `🗑️generated/retained-pack-pipeline-diagnostic-native-green-2.log`.

## Source and Command Ledger

- `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws/🟦️.ts`
- `🧰️framework/🔨️modules/🎒️pack/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🎒️pack/🧫️fixtures/🔣️.json`
- root `📜️script.ts` and `📋️project.json`
- ticket `validation/project.json`
- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`, orders 311.234 and 311.235

## Following Physical Boundary

The following inflater slice now accounts for DEFLATE physical state with a maximum 32,768-byte retained ring, inline bounded Huffman storage, exact allocation/release demand and mounted aggregate propagation. See `retained-pack-inflater-physical-implementation.md`.

Typed Generation collections, mutation builders and retained SPR/history remain. No full retained-pipeline or recursive-persistence claim is made here.
