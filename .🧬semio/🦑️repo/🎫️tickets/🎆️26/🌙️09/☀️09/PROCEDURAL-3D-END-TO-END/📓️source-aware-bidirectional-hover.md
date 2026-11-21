# Source-aware bidirectional hover

The framework now projects the ephemeral source of a live, channel-qualified interaction mark. Edit, Generate and View carry that provenance with the grouped World3d instance marks. An explicit panel origin is JSON null. An unknown or absent origin omits the field. A new authored mark from another window or the panel takes over local viewport paint. The viewport keeps its latest ray when its own delayed echo arrives or only selection changes.

Native World3d uses the mounted window instance id for incoming arbitration, outgoing hit actions and leave actions. Trace cursor echoes use that same window identity. A queued ray can owe a subsequent leave before publishing anything. An explicit local miss relinquishes its paint when leaving and reveals the authored graph group without issuing a clear it does not owe. Native graph hover actions address the window instance recorded during attach. Leaving the graph publishes a single scoped clear and allows the same port to be picked on reentry.

## Verification

- The shared hover fixture has 13 handoff cases for identical groups moving between windows, same-window echoes, selection and ordering changes, panel takeover, explicit misses and an omitted source. All 16 focused Vitest tests passed. The independent RxJS projection passed all 13 fixture assertions.
- Chromium WebGL mounted the production React World3d renderer. Three projected regions changed by 471, 625 and 625 pixels, and the unrelated region held. A real ray over the unrelated instance emitted the graph-qualified handle and changed 445 pixels in that region. Its own-window echo left the ray intact. A foreign owner with the identical group switched all three graph regions without pointer movement. Leaving cleared the author projection and left geometry revision at 1.
- The scoped native source witnesses passed for the declaration owner read and JSON projection, the World3d selection bridge, the graph bounded action receipt and leave, and the ordered native pointer boundary. The native selection fixture failed at its same-window echo before the repair. The graph witness ran against the actual bounded action and WGPU input libraries. Native window setup and scene host plumbing remain covered by registered tests awaiting their linked runs.
- Rust syntax, the measured source boundaries and whitespace checks passed for the touched owners. The official Flow BRep run is still behind local disk admission. The filesystem had 2.81 GiB available and the official build needs at least 5.5 GiB. The Chromium probe exercises the mounted viewport. The complete procedural application still needs verification.

## Remaining

Run the linked Flow, composition and native renderer suites, then mount the canonical procedural app and verify the graph windows, all preview modes and the native pointer sequence together.

## Graph projection

The editor Flow graph scene carries the same hover provenance through the Rust, retained value and TypeScript scene representations. The graph keeps a newer pointer hit when its own older mark comes back, and applies an identical mark from another window or the panel. Native cleanup preserves an authored mark while retiring the last local graph hover.

The two source-aware graph cases failed before the change. All 26 graph hover and publication tests then passed. The independent RxJS witness passed 12 assertions. A native run compiled the complete node-graph scene schema and eight registered schema tests using cached dependencies. It also compiled the native reconciliation function and passed the shared source cases, for nine tests total. One additional registered native case exercises actual graph pointer movement through the mounted scene host and awaits its linked run.
