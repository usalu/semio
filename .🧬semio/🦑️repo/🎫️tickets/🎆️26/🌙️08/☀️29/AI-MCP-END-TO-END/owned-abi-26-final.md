# Original Owned ABI 26

This is the proposed final 36-export contract for the source-controlled cutover. Channel 26 must not be accepted until the guest, Core, runtime, host controllers and derived descriptors agree. There is no tuple or numeric-authority fallback.

`B19` is the existing 19-scalar `OriginalJobBudget` representation: original operation, generation, epoch, clock binding, absolute deadline, total work, five total retained axes, five scoped retained axes, scoped work, scoped deadline and cancellation. Every scalar is an `i64`. The fields are data from an actual issuer-held loan, and cannot independently create or renew authority.

| Export suffix after `semio_owned_` | Parameters | Result |
|---|---|---|
| `alloc_v1` | `i32` length | `i32` |
| `dealloc_v1` | `i32` pointer, `i32` length | none |
| `channel_version_v1` | none | `i32` |
| `checkpoint_v1` | B19 | `i64` |
| `restore_v1` | B19 | `i64` |
| `describe_v1` | B19 | `i64` |
| `cancel_job_v1` | `i64` job, B19 | `i64` |
| `admit_actor_input_v1` | `i32` pointer, `i32` length, B19 | `i64` |
| `publish_actor_failure_v1` | `i64` job, B19 | `i64` |
| `read_actor_failure_v1` | `i64` job, B19 | `i64` |
| `borrow_actor_failure_v1` | job, operation, generation, offset, length (`i64`) | `i64` |
| `acknowledge_actor_failure_chunk_v1` | job, offset, length (`i64`), B19 | `i64` |
| `acknowledge_actor_failure_v1` | `i64` job, B19 | `i64` |
| `start_job_v1` | `i64` job, B19 | `i64` |
| `step_job_v1` | `i64` job, B19 | `i64` |
| `read_job_output_v1` | `i64` job, B19 | `i64` |
| `acknowledge_job_output_v1` | `i64` job, B19 | `i64` |
| `close_jobs_v1` | B19 | `i64` |
| `borrow_job_output_v1` | job, operation, generation, offset, length (`i64`) | `i64` |
| `acknowledge_job_output_chunk_v1` | job, offset, length (`i64`), B19 | `i64` |
| `poll_v1` | B19 | `i64` |
| `admit_actor_context_v1` | B19 | `i64` |
| `close_actor_context_v1` | B19 | `i64` |
| `close_actor_output_v1` | B19 | `i64` |
| `pack_schema_hash_v1` | B19 | `i64` |
| `genesis_v1` | B19 | `i64` |
| `print_mirror_v1` | B19 | `i64` |
| `apply_ops_v1` | B19 | `i64` |
| `replay_envelopes_v1` | B19 | `i64` |
| `sqlite_schema_v1` | B19 | `i64` |
| `sqlite_export_v1` | B19 | `i64` |
| `sqlite_import_v1` | B19 | `i64` |
| `sqlite_take_payload_v1` | B19 | `i64` |
| `sqlite_take_file_v1` | B19 | `i64` |
| `sqlite_close_v1` | B19 | `i64` |
| `sqlite_retirement_v1` | B19 | `i64` |

Mandatory actor-context admission establishes the real guest Source and UI arena before bundle initialization. Infrastructure actor-input admission receives source JSON tagged as Job, Codec, Snapshot, Poll or Restore. It retains the wire, parser, partial typed receiver and completed value under that same Source. Semantic exports reference that retained input under a fresh actual scoped budget. Requested input ceilings remain data and are checked against the actual semantic context.

Actor admission uses the existing resident 136-byte fixed frame. Jobs and funded failure publication use the existing 184-byte fixed frame. Failure output pages remain owned by their original publication until the actual byte receiver and page acknowledgement complete. WIT read operations return owned page bytes; the pointer-borrow and chunk-handback exports are Owned-only mechanisms.

Semantic result encoding happens inside a real scoped context. The resulting bytes remain in a preborn actor-output owner. `dealloc` records only the prepaid exact loan handback for resident or source-owned output; it does not destroy the retained body. `close_actor_output_v1` performs actual funded deep retirement. It replaces the narrower `close_turn_output_v1` name.

The allocator and immutable channel query remain Core-runtime boundaries. All body execution, initialization, codec, snapshot, poll and output retirement calls require the explicit scope above. The scalar Core call frontier must support 22 arguments, which the chunk acknowledgement contracts require.

The contract is not runtime evidence. Actual current guest compilation, publication hash, all 36 real export signatures, funded execution and real heap/retirement laws remain mandatory gates.
