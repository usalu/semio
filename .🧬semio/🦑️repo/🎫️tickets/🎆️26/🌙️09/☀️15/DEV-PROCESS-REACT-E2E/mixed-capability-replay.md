# Mixed Extension Capability Replay

The four declarative Process3d component descriptors contribute machines to `process.machines` for `process3d-play`. The existing shipped-descriptor editor test picked the first capability of each catalog, which produced four Cut measures. The existing mounted `composed_step_timeline_persists_inspector_edits_and_order` test added one Cut, Drill and Attach through kind-only arguments and checked the packed timeline. The timber replay fixture measured three operation kinds on its pre-authored workshop. None of those checks exercised mixed capability kinds from multiple published providers on one retained document.

The catalogue's capability row binds `addStep` with the installed machine and capability IDs, subject to the current stock dimensions. The mounted action resolves those IDs from the document workshop and saves the resulting measure and its origin. Replay uses the saved measures at each prefix and applies subtraction for Cut and Drill and union for Attach. It requires the authored geometry and enabled-step order, so a reopened document can replay after host contributions are removed.

The new neutral fixture uses a 200 × 180 × 50 mm stock box, a Concrete Diamond Saw crosscut, a Robotic 6-Axis Mill bore and a Metal MIG Welder plate attachment. The disc covers the stock footprint and shaves 2 mm from the top. The offset 10 mm radius bore passes through the remaining 48 mm. The 100 × 100 × 8 mm plate overlaps the stock by 2 mm and leaves the bore clear. The saved material volumes in cubic metres are 0.0018 at stock, 0.001728 after the cut, 0.001712920355262769 after the bore and 0.001772920355262769 after the plate. Editing the bore radius to 12 mm yields 0.0017662853115783873.

The retained editor regression installs the shipped contributions, dispatches the workshop and catalogue actions, positions the measures through the inspector, reopens the pack and checks origin, order, position and replay volume. It edits the bore, removes the contributions and reopens the pack again to check replay from its stored measures. The Three.js oracle independently reads the descriptor parameters, validates the solid overlap and clearance, and reproduces all five volumes. It passed with `repo tools exec bun --light -- <ticket>/mixed-capability-replay-oracle.ts`. The scoped Git diff check passed. The mounted Rust test has not run while the repository's isolated Cargo run is active.

[Independent oracle](mixed-capability-replay-oracle.ts)

## Native test source admission

The native policy caps each source closure at 32 files, 4,000 active lines and 262,144 bytes. The editor library test configuration originally measured 19 files, 3,933 active lines and 267,517 bytes after the mixed regression was added. It exceeded the byte limit by 5,373.

The existing `process3d_panels_workshop` integration test now owns both mounted tests that read the published descriptors and their common decoder. The editor unit tests retain the contribution output-gate, pack and archive checks. The Process3d test supervisor already runs the workshop target.

| Test unit | Files | Active lines | Bytes |
| --- | ---: | ---: | ---: |
| Editor library | 18 | 3,732 | 251,771 |
| Workshop integration | 4 | 859 | 56,693 |

The repository source reader counted both manifest-declared roots with the editor manifest directory and `unix,test` configuration. The editor library has 268 active lines and 10,373 bytes of headroom. The relocated fixture resolves from the new test module, the Three.js oracle passed again and the scoped whitespace checks passed. No native compilation was started for the census.
