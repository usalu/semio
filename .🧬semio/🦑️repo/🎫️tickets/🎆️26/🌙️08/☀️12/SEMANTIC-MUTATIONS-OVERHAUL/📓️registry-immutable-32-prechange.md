# Immutable Registry 32 Pre-Change Evidence

The new schema-first matrix is retained at `🧪️registry-immutable-32`. Its independent Bun/Node
crypto oracle validates 21 frozen identity-field preimage sensitivity checks and 9 registration
laws: complete batch admission and lookup, equal idempotence, leaf and semantic conflicts without
replacement, invalid metadata rejection, same-batch and existing-entry atomic conflicts,
distinct-owner wire-tag reuse, and empty-batch success. The exact Node SHA-256 preimage is the
frozen UTF-8 domain string, a zero byte, then compact ordered JSON; it excludes source provenance
and `workspace_token`. Its pinned fingerprint is
`77bcc0966fca8b3984ea0d87e0cf5172e0ca44095c2b14988ce4fe02ce007a54`.

The retained genuine client now has exactly 9 fixture-named tests. It asserts all public getters,
full leaf and semantic equality after lookup, the pinned Node fingerprint, equal idempotence,
typed conflict payloads with unequal fingerprints, both no-partial-commit cases, invalid envelope
and leaf metadata rejection, and valid constructor-admissible identity sensitivity. Invalid
`leaf.schemaVersion` and deliberately invalid enum/array raw values remain solely in the separate
Node preimage sensitivity loop; they are not falsely presented as admissible constructors.

The current genuine kernel API was compiled through scoped Nx and rustc with both borrowed formats:
`🧪️registry-immutable-32/🧫️run-B1AEv8/`. The compiler exited 1 with no signal or spawn error,
as expected before the root implementation. It reports all three intended API gaps:

- `MutationDescriptorRegistry` is absent from the public kernel;
- `MutationDescriptor::new` takes three arguments instead of the frozen five immutable identity
  arguments;
- construction does not return `Result`, so the client cannot propagate validation failure.

The exact artifact hashes retained in that run's `🔣️results.json` are:

- rlib `00c7fb17b3c0ed3220038393633a0f6d92a624491aa951dc2418feb639259303`;
- rmeta `12b0c017ebfb4337ce58c05b8a90a84fb56d037f5654b4dc44c9e53c87d987be`.

While root's fresh Cargo build was in flight, the formerly loaned rlib was temporarily absent while
its rmeta remained. `🧪️registry-immutable-32/🧪️prechange-red-final.log` records the resulting
early pair-presence rejection; it is not compiler evidence and does not replace `run-B1AEv8`.
This was a coordinated in-flight rebuild, not external artifact loss and not an action by this
lane. The script's `green` mode deliberately requires
`SEMIO_REGISTRY_IMMUTABLE_DEPS` and a fresh coherent rlib/rmeta pair, fingerprints client,
fixture, schema and both artifacts before and after compilation, then lists and runs the 9-test
binary.

The loaned target subsequently changed again without a release notice: its pair reappeared as
rlib `3093e2079441bbb95dcaa978cc066d01daf612e93c7df7ff0248533a4e86638c` and rmeta
`684abd57149b2b06d6fb1b326926b38552b2ad0338b79ad58e251ff11c3d283c`.
`🧪️registry-immutable-32/🧫️run-byVBpq/` shows that this new pair compiles the full client, so
the pre-change-mode guard intentionally rejects it as unexpectedly green. No test binary was
listed or executed from that pre-release observation. Root subsequently confirmed this exact,
held fresh pair. The scoped Nx green command then compiled, listed and ran the genuine client in
`🧪️registry-immutable-32/🧫️run-cQ0ZW0/`: all 9 listed tests passed, with status `0`, no signal,
no spawn error, and stable before/after hashes for the client, schema, fixture, rlib and rmeta.
The retained binary SHA-256 is
`c1052b0bc71245a81cc890d3b42b36c927288ae39550c3534961e5d64e56400a`.

Read-only implementation review found the command registry, both derive mirrors, public reexports
and OS config result propagation aligned with the frozen API: construction validates then
fingerprints the immutable envelope; registration preflights existing and pending identifiers before
the one commit; both derives construct every descriptor with `?` then make one batch call. This is a
source review only, not a compilation or runtime acceptance.

This packet retains both the pre-change red and subsequent nine-test green checkpoints described
above; neither is a claim of whole-repository registry/startup readiness. No production source,
derive, caller, or shared registry file was modified by this review lane.
