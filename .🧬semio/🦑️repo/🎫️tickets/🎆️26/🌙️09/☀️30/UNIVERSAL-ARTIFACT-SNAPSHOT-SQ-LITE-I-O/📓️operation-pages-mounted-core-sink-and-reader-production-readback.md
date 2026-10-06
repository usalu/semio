# Mounted Core Sink and Borrowed Reader Production Readback

Current production source readback only; no executions. Immutable reports Core streamed-owner Native 1/1, run `2ff52161`, in `operation-record-pages-native-after-sink-retry.log`. That receipt is distinct from pending reader joins, full Core regression and ChildEmit/OpBinary publication.

## External Encoder

Core Pack `🌱️value/🦀️.rs:31` exposes `encode_record_body_into(spec, record, options, &mut dyn OperationByteOutput, &mut NativeEncodeControl)`. Its docstring explicitly appends after an existing caller prefix and retains accepted bytes on refusal.

The mounted `🌱️value/🛫️encode/🦀️.rs:140` uses the same symbol discovery, sorting, measuring and Encoder body as the controlled ordinary path. It respects the supplied options and control maximum, checks body max_file_len and remaining allocation admission, then constructs `Output { bytes: None, external: Some(output), length: 0 }`. No contiguous payload Vec adapter is constructed in this branch. `Output::bytes` delegates directly to the borrowed sink; it does not clear existing bytes. The length returned on success is the appended body length, excluding the caller prefix. On error the internal attempted length is discarded, while the original caller-owned sink and accepted physical prefix remain accessible.

Replication `🎮️mutation/📦️bytes/🦀️.rs:141` implements the sink directly on `OwnedOperationBytes`. Each physical reserve uses its actual next allocation demand, requires at most 4096, checkpoints and charges the supplied encode control before reserve, then pushes individual bytes and advances that same control. It borrows the source fragments; no whole-fragment copy buffer is created. Symbol/frontier allocations remain controlled scratch, separate from paged payload ownership. The logical measurement is conservative admission, not proof of every physical capacity by itself; the concrete paged sink supplies that physical authority.

The ordinary pure `encode_record_body` at Core root line 3150 still follows its existing `build_symbols`, HashMap, ordinary EncCtx/PackWriter and Vec construction. The sink is a new controlled entrypoint, rather than a redirect of this ordinary API. Existing ordinary contiguous behavior therefore remains; it must not be credited as paged behavior.

## Borrowed Reader and Remaining Join

Replication codec root lines 120–187 mounts a private `ByteStorage::{Slice,Paged}` and checked `ByteSpan` borrowing the exact `OwnedOperationBytes` lifetime. `ByteReader::from_source` retains that genuine concrete owner; spans/numeric fixed-size arrays read page bytes directly. `read_bytes` refuses a paged source with UnsupportedOwner before advancing the cursor, rather than materializing a contiguous Vec. Truncated `read_span` also leaves the cursor unchanged.

The actual current Core Record decoding consumers still call `read_bytes` for inline blobs (2054), table bitmaps (2554/2595) and symbol UTF8 (3215). The repository source scan found `ByteReader::from_source` consumers only in the Replication floor tests, not in a mounted Core Record decode entrypoint. Thus the reader provider is mounted, but full paged Record decode requires the explicit borrowed-span/materialization joins and a real Core source entrypoint. This is a concrete incomplete join, not a hidden unbounded fallback: current paged read_bytes refuses instead of copying.

Likewise `encode_record_body_into` currently has its Core owning test consumers; the source scan does not establish a first-party ChildEmit or OpBinary producer adoption. The genuine streamed-owner law can substantiate its own route, but cannot substantiate the pending publication family.
