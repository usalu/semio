# G4, G6, G16 Natural Source Ingress Integration

Scope: the ingress and admission boundary of the large natural PDF source. The paged decoder (G5), archive reopen (G7) and the segmented Save producer (G10) stay with their owner. No decoder code was rewritten.

## What was applied to the replica

The staged generators were rerun against the current replica (`natural-pdf-integration-stage.py`, `natural-pdf-sdk-stage.py`, `natural-pdf-publication-stage.py`). `natural-source-sdk-publication.patch` is a superset of the wire, receiver, paid-open, transport, integration and SDK patches (verified by file set). It was applied with `patch -p1` after a clean `git apply --check`, followed by `native-natural-source.patch` (disjoint files).

- G4: kind 241 in the replication ingress, guest receiver and prologue, WIT `instance-open-event.purpose`, `natural-source-prepare` and `natural-source-control`, paid creation and launch grant, `NaturalFilePublication` with `take_source`, `finish_format` after real publication, and cancellation that retires the donated source. The native WGPU sender and queue (G16 sender) are in.
- G6: purpose-bound factory `import-natural-source` (ordinal 22, schema `framework.reserved.import-natural-source.v1`, Artifact and Child lanes). It is registered only for apps whose paged decoder extent is nonzero. `import-media` keeps its 8 MiB caps. The progress validator now bounds consumed input by the authenticated source extent, not by the 8 MiB command wire cap.

### Limit design (not arbitrary constants)

The app declares one number, its paged source extent (the PDF editor: 16 MiB selection). Output is bound at exactly two source extents (32 MiB, one decoded backing plus the Pack export) and the raw wire is one byte because the body travels as paged source, never as a wire body. The law `natural-source-limits.json` states the relations: fixture 9,437,877 is at most 16 MiB, it exceeds the ordinary 8 MiB output, the 9,437,184 byte image fits the natural output, and output equals extent times two.

### Per-turn grants versus one large raw stream

The 32 MiB output ceiling is an admission bound only; it does not enlarge the mounted INTERACTIVE grants (16 MiB capacity, 1 MiB release per turn). A single 9,437,184 byte raw stream therefore cannot be released in one turn. Retirement of such a backing must be paged per 4 KiB leaf under the grant (the octet retirement overlay of the G5 owner). The natural owner here retires its donated source page by page (`NaturalFilePagedSource::step(body, effect)`), never as one release. Aggregate ledger proof under the real grants remains a mounted verification item.

### Regression repaired during this work

A concurrent writer replaced `plugin/👷️job/🏛️reserved/🦀️.rs` with an older copy (last writer wins), dropping the ordinal 22 natural factory, its registration, `ReservedCommitMetadata`, `setActiveTool`/`setActiveUtility` and my limits test module, which broke `semio-plugin-instance-jobs`. I restored the file from the staged index copy and kept the other writer's new `Interaction` publication lanes on `cut` and `paste`. `retire_born_candidate_step` is not defined anywhere in the replica or in HEAD; its callers (`instance/👷️jobs/…/🏷️reserved`, `instance/🖼️render/…/🏷️reserved-jobs`) belong to an in-flight action-bus wire construction change by another agent, not to this work.

## Changes made on top of the staged patch

1. The `semio-plugin-instance-app` crate exceeded its source boundary (267,663 of 262,144 bytes). Another agent raised it to 294,912 meanwhile. Independently, the natural owner and its turn driver were extracted into a new crate `semio-plugin-instance-natural-publication` (`🧵️instance/🎟️natural-publication`), which holds `NaturalFilePublication`, `natural_source_capability::<A>()` and the host-independent `advance_natural_publication` over a narrow `NaturalPublicationHost` trait. `plugin-app-impl/🎟️natural-publication` is now only the trait impl. The type moved out of `instance-docs` (which was at 262,091 bytes). Registered in `Cargo.toml` (both workspace files) and `Cargo.lock`.
2. G16 tests: two cancellation laws were added to `native-natural-source-test.rs`: cancellation half way through the 2,305 page fixture stops after the acknowledged page with exactly that many bytes read, and cancellation raised while a blocking read is in flight drops that page and closes with Cancel. The harness `g16-run.py` compiles the real product sender and queue sources.
3. G6 tests: Rust law `reserved/natural-source-limits-tests.rs` and independent SQLite based Bun law `reserved/natural-source-limits.test.ts`, sharing `natural-source-limits.json`.

## Evidence

- `repo tools exec --light python -- g16-run.py`: 8 of 8 native sender and queue tests pass (including the 2 new cancellation laws and the 9,437,877 byte transfer in 2,305 pages). Ajv geometry and an independent SHA-256 agree on the received file.
- `repo tools exec --light bun -- test ./…/reserved/natural-source-limits.test.ts`: 1 pass.
- `repo cargo check` of the instance, docs, new natural crate and `semio-plugin-instance-app`: see Verification status.

## Verification status and what remains

See the end of this file for the final cargo results. Not verified here: a mounted open of the 9,437,877 byte file (needs the PDF app to declare its paged decoder, which belongs to G5), the real WIT bridge round trip with the actual guest, and the native product link (G9).
