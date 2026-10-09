# Clipboard Codec Audit — 2026-10-09

Read-only independent audit of current source; no tests or builds were executed. The existing clipboard-hydration audit was read first. Files were changing concurrently, so observations identify current contracts rather than completed implementation.

## JSON feasibility

Framework JSON `JsonWriteSource` directly borrows ordinal nodes and keys. `JsonWriteNode::NativeString` borrows `Utf8Text`; `JsonWriteCursor` state 6 retains chunk index and byte offset and emits one UTF-8 scalar per transition, including escapes. There is no whole native-string projection. Source lookup reconstructs a bounded path each turn. Draw's source uses paged ordinal collection reads; asset BTreeMap nth access scans at most the admitted asset count and is quadratic over the table.

`JsonWriteCursor::step` retains its complete original owner through measurement and writing; `take_source` becomes available only after output completion. Cancellation checkpoints precede transitions. Output is one exact-capacity String, admitted between the two passes, rather than externally drainable chunks. Thus it supports bounded physical writes but not bounded physical output allocation. This matches the existing ClipboardFragment whole-string contract if the job admits that full allocation. A future chunk sink cannot be obtained by treating `step` results as fragments.

`write_json_source_into` is an alternative borrowed sink: raw output fragments are fixed scalar/escape spans (at most six bytes for strings) and native chunk text is traversed without joining it. It is synchronous recursive traversal with checkpoints, however, and has no persistent continuation after refusal. The sink keeps its accepted prefix; blindly restarting duplicates that prefix. Use the retained cursor for cooperative jobs unless a genuine retained sink cursor is introduced.

## Hydration gaps and ownership

The generic `FromValue::from_value_controlled(&DslValue, NativeDecodeControl)` remains synchronous recursive typed decoding. It accounts/checkpoints but does not retain a resumable typed call stack. The current Draw `ClipboardHydration` explicitly strips large fields and hydrates bounded shell/leaf records, text at UTF-8-safe 1,024-byte boundaries, and one collection element per turn. This is feasible without a second JSON grammar.

The current hydrator has no decode-control/grant argument. Vec task growth and path clones are not preadmitted; no explicit nesting-depth guard exists. Paths can therefore grow with authored nesting even though each record shell is bounded. Boolean reference leaf validation currently restricts strings to 64 bytes through `bounded`, whereas selected identities admit 4,096 bytes. Shell decoding consumes the temporary source through unchecked `FromValue`, so error-path ownership and physical retirement require deliberate handling; a derive RetireOwned declaration alone does not prove rejected owners avoid synchronous destruction. Stripped tasks retain original parsed fields until retirement, which is sound provided the enclosing job owns and drives that retirement.

No live clipboard reserved-job producer was present in the audited source searches. The command agent confirmed it is being implemented: cancellation, cumulative NativeDecodeControl, source-task retirement, native paged task scaffolding, reference limit 4,096 and depth 32 are planned. Exact preadmission before scaffold/page growth remains necessary. The existing unit test exercises serde equality, repeated encode turns, hydration turns and interrupted retirement in source; it was not run in this audit.

Publication should occur only after hydration, complete packet/reference/asset validation, current-source generation validation, and candidate preparation. Cancellation/refusal must retain parser, hydrated candidate, output String, source packet and partial effect ownership until bounded retirement. One fuel per logical hydrator step does not itself bound allocation/copy/release; grants must cover those axes separately.

## Native affine extension note

PreparedSceneNode already carries the complete six-coefficient affine matrix. Its world bounds apply that matrix to paths and text/image fallback rectangles; selection preview composes matrices through the shared geometry multiply routine. Therefore schema/view records already preserve shear/reflection/nonuniform scale. Native renderer work should consume that exact matrix for geometry, images, text and gradients rather than reconstructing a rotation/scale approximation. Actual native renderer dispatch was outside this narrow audit; no runtime equivalence is claimed.
