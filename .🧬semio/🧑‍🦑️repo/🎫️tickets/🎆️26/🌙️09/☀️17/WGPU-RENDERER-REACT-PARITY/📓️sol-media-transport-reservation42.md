# WGPU Media Transport Reservation

## Outcome

The WGPU retained-document resolver now reserves the exact versioned extension address `framework.media.transport@1`. A valid canonical media transport packet remains an `ExternalSlot`, retains its exact address and `paramsJson`, reserves the producer-declared `hostContentHeight`, and paints the packet's localized `labels.unsupported` text. The resolver intentionally ignores a producer capability of `ready` or `loading`: no native decoder exists, so projecting playback controls or a playable state would be false.

Malformed reserved packets are distinguishable from valid-but-unsupported packets. When the malformed packet still declares a supported locale, WGPU paints a localized invalid-contract status. When the locale itself is absent or invalid, the status is the language-neutral code `framework.media.transport@1 · invalid-contract`. Generic extension addresses preserve their prior behavior and continue to paint their body key.

This is an honest intermediate capability state, not completed media parity. Browser WGPU can eventually delegate playback to a host `HTMLMediaElement` bridge. Native WGPU still needs first-party system decoder interfaces before it can report `ready`.

## Source

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs` owns the reserved ID, exact contract validation, locale-aware invalid status, and projection into `UiExternalSlotNode`.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` adds the optional `hostStatus` presentation lane without changing `bodyKey` identity.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` paints `hostStatus` when present and otherwise keeps the generic body-key placeholder.
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` mirrors the optional wire field.

The runtime validator matches the canonical schema and the stronger producer parser: exact keys, schema version, kind, MIME and revision bounds, unsigned-64 decimal revisions, safe-integer timing, selection pair/order/bounds, resource revision equality, exact output port, capability/resource consistency, labels, reason length, and finite content height.

## Neutral and executable evidence

The language-neutral vectors are `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎬️media-transport-reservation/🔣️.json`. They cover source `ready`, source `loading`, malformed German, invalid locale, and a generic extension. The Rust law in `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️document-tree-reconcile/🦀️.rs` projects every vector through the real retained document resolver and checks identity, status, and untouched params. Its native execution is pending the parent-owned focused native gate.

The Bun test `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎬️media-transport-reservation/🟦️.ts` validates every packet in Ajv 2020 strict mode against the canonical schema at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🧬️contract/🔣️.json`. The schema owner added explicit object types to its conditional subschemas after the first fail-first receipt. The saved scoped Nx receipt `🗑️generated/sol-flow34/media-transport-reservation.log` passes 2/2 with 8 assertions.

The earlier retained UI/readback rerun is recorded in `🗑️generated/sol-flow34/ui-six-laws-2.log`: Nextest run `b3550868-1be5-40d3-b4dd-5c4df6795faf` passed 6/6 with 720 skipped after 30m20s compilation. It includes same-size moved-origin atomic publication, meaningful retained progress, Tree action-row label preservation, two neutral readback laws, and the actual production GPU readback/cancellation law.

## React host integration audit

React's generic resolver already has the correct reservation seam: `🧰️framework/🔨️modules/🎠️kernel/🧩️extensions/🟦️.ts` skips plugin contributor resolution for IDs in `hostExtensions`. `ShellHost` currently does not pass that set. A real React host implementation therefore needs to:

1. register only `framework.media.transport@1` in `hostExtensions`, before generic plugin/app resolution;
2. render that retained extension through a host-owned media component rather than through the generic unavailable-plugin text;
3. parse the canonical contract before mounting controls, preserve `hostContentHeight`, expose an accessible localized status, and retain the exact revision/resource identity;
4. submit `playback:out` through `plugin_submit_media_export`, poll one bounded slice at a time with `plugin_poll_media_export`, and cancel on owner retirement or resource/revision replacement with `plugin_cancel_media_export`;
5. retain and compare the complete `ArtifactMediaExportHandle` authority (`app_instance_id`, `parent_document_id`, `operation_id`, `base_revision`, `generation`) so stale or superseded completions cannot reach the media element;
6. publish loading progress while `ArtifactMediaExportPoll::Running`, bind a terminal artifact only after `Complete`, and retire `Cancelled`/`Failed` without replaying a stale object URL.

No React files were changed in this packet. The reserved ID alone is insufficient until `ShellHost`, the contract renderer, and the browser artifact-byte/object-URL lifetime are connected. Native remains unsupported until a system-decoder interface exists.
