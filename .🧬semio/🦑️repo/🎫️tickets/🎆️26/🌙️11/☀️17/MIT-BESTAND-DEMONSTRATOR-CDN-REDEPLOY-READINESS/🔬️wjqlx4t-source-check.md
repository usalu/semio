# wJqLX4t Source Check

Snapshot during the active build. The current five source files below each have a generation-private `cs` body whose bytes were independently read and checked against its SHA-256 filename. A matching body in the store alone does not prove that the driver selected it for a given path or that the final compiler seal will succeed.

| Source | Bytes | Current and stored BLAKE3 | Artifact-associated dep-info |
| --- | ---: | --- | --- |
| Plugin `🪜️maintenance-ladder/🦀️.rs` | 82,325 | `c322df79219a992bd890a1213ecbedc663d3e9c1942bd0a319f5f858a855760f` | `semio_plugin_instance_ladders-aec9df958f9dd59b.d`, matching checksum and extent |
| Plugin instance render `🛠️tool-machine/🦀️.rs` | 40,358 | `c4dcbf80954eb3f237edbee75d12500fc2ace0cdeef04ad126c8d4cab005e74a` | `semio_plugin_instance_render-53be129cb06bb016.d`, matching checksum and extent |
| Store `🧵️canonical-edit/🦀️.rs` | 35,816 | `7a8bcf62ec645c6c696b62b63937e1ae49369db6ac830135fa01655c32d95346` | `semio_os_store_one_item_publication-618a5148b42a4d46.d`, matching checksum and extent |
| Process3D `🪟️editor-impl/🦀️.rs` | 30,582 | `73c4e1a880e1fd52d83cab9f5131d58b9aa848f5b95fe3a8f6f600fb665a2ca2` | Not yet found at snapshot |
| Process3D `🧵️retained-commands/🦀️.rs` | 27,022 | `8b31f48e8d731d443abf63df3022cb00c51491648472502de622b30529299ef8` | Not yet found at snapshot |

The first three dep-info files are adjacent to artifact paths announced by this run's Cargo stream in lane `p10be29`; their absolute `../..` source paths normalize to the selected live files. These are mutable working-lane observations, not the driver's sealed observations. No source mismatch was observed among the selected completed units. Remaining units and any concurrent later edits remain subject to normal source and output seal checks. The build was not paused or altered.

The first Cargo pass later reported E0432 for `graph_channel_code_unit_width` and `graph_output_channel_body` in `semio-s-artifact-procedural-3d-transforms`; the driver began its same-generation retry plan. The snapshot above predates that retry and does not certify its source selection.

The same-generation retry source admission failed before link. Its report names `process.source.boundary-exceeded` (262,314 bytes against 262,144); the terminal stack also prints `process.source.read-bytes-exceeded` (292 observed against 122 remaining). These are one causal chain: `sourceBoundaryRefusalV1` wraps the reader error as its cause, and 262,144 − 122 + 292 = 262,314. The first-pass compiler E0432 remains a separate diagnostic. The probe ended without an input; the corrected watcher moved to the queue's second attempt `wzhjRpw`.

The second queue attempt `wzhjRpw` retained SHA-verified bodies for all five same current inputs above. Its lane `p4e96ae` Cargo stream announced the same three ToolOverlay/canonical units; their adjacent dep-info checksums and extents match the listed current bodies. Process3D editor and retained-commands dep-info was not yet present at this snapshot. This is a bounded mutable-lane check, not the final custody seal.
