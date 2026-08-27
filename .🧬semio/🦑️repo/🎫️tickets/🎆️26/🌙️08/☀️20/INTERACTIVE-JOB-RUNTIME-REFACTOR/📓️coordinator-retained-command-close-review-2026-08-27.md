# Shared Retained Command Close Review

## Confirmed Capacity Deadlock

The Flow executor found and the coordinator confirmed that ArtifactRetainedCommandJob drains initialized raw bytes, then refuses to release the empty Vec unless the caller's byte grant covers its entire reserved capacity. The constructor reserves maximum_raw_bytes up front. A 16,384-byte reservation can therefore never finish close under the mounted 4,096-byte grant, even after all semantic bytes are retired.

The plugin executor owns a schema-first regression and repair. Empty uninitialized allocation release is a distinct allocation boundary; it cannot require nonexistent semantic-byte credit or report those bytes twice. Maximum allocation/timing/global admission still needs explicit proof.

## Additional Uncompleted Lifecycle

The same helper's retire_one macro directly destroys emit, ephemeral values, command, snapshot, config, history, interaction, hover, context, and operation owners, reporting one item and zero bytes. Those types can carry strings, collections and final ordered roots. A generic whole-owner drop is not bounded domain retirement, even if some scalar fixtures pass.

New parameter features must use explicit typed payload/root/disposer ownership. The capacity repair alone does not certify the helper's complete lifecycle. The constructor's upfront maximum-sized allocation and whole binary decode also remain unproved for large accepted commands.

The coordinator read the exact constructor, wire/decode phases, and complete close tail. No full-helper runtime or all-app completion is inferred.
