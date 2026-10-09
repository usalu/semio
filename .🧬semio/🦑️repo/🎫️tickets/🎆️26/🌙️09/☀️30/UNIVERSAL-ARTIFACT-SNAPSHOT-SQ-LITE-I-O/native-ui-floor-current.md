# Native UI Floor Current

Read-only 2026-10-09. Original Window root-metadata-native.log contains E0533 failures before Window laws. Log ends Kernel compilation failure (11 errors), not runtime assertions. Socket current10 separately reports replication2 compiler errors; older native-job metadata log reports Kernel lib-test689 errors. These logs are different preparation snapshots, not combined current runtime results.

Actual enum is `🧰️framework/🔨️modules/🧵️job/🦀️.rs:1200–1204`: InteractiveJobCloseStep::Refused { kind, progress }. Its progress/validate1210–1212 intentionally carries retirement progress on refusal. Current UI source still constructs tuple Refused, so E0533 remains a real prerequisite at observation.

UI canonical source `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs`: prepared_close_gate15/17, log demand747, abandonment state1878/2175, depth2681 and other tuple constructors returned by local rg. Draw canonical source `…/🖍️draw/🏷️types/🦀️.rs:616–617` has demand error and zero-depth tuple Refused. Compiler paths via `📦️packages/🦀️rust/../../` resolve to these canonical files.

Precise repair: use struct variant; error path kind:error.kind, progress:error.retained_progress(); pre-work depth/invariant refusal progress default. For refusal after physical work retain actual cumulative turn progress; do not blindly set default on all cases. Preserve gate behavior: zero item/insufficient release returns Pending with zero progress and keeps owner; depth zero refuses, never Complete. Keep actual Window metadata law unchanged.

Kernel Store canonical `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` also still has tuple Close::Refused at10289/10304/10309/10316/10320/10335/10339/10345/10350/10372/10377. These are actual same enum aliases; apply equivalent progress-preserving construction. Validate whether earlier action already made progress before each invariant branch. Job authentic existing wake paths use error.retained_progress (`🧵️job/♻️retirement/🔔️wake/🦀️.rs:74,79`) as the proper original pattern.

No Native Window success claim. After canonical prerequisites are corrected, replay original route and require actual law assertion/terminal receipt; compiler success alone does not qualify metadata or ownership.
