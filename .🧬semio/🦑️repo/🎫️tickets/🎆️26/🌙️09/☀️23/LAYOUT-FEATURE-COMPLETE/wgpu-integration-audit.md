# WGPU Integration Audit

Read-only current-source inspection, 2026-11-20. No compilation or runtime result is claimed. Paths below are relative to 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu.

## Stale close callback signatures

The current 🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🦀️.rs:6 requires close_original_retained_step callbacks to return RetainedWorkOutcome, including independently measured work. The following production callbacks instead return bare Result<RetainedCloneStep, _>, so they are probable E0308 failures if included in the build target:

- 🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🎟️task-ingress/🦀️.rs:45–46.
- 🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🐣️create-app/🧩️compile/🦀️.rs:29, 32–33, 56, 80.
- 🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🐣️create-app/🎟️source/🦀️.rs:50–52.

Target reachability under wasm was not established. Preserve actual ownership, released capacity, denial behavior and Work when updating; the surface-lane:273 and renderer asset-decode-jobs:407 supply a current one-work example for a simple terminal owner transfer. Future and heap-owner releases need their actual work and release accounting, not mechanical result wrapping. Test-only task-ingress/🧪️tests/🦀️.rs:16,24 contains the same stale pattern.

## PNG transfer and retirement

🧊️renderer/🦴️glb-decode/🎟️png-texture/🦀️.rs:31–42 admits the retained demand, performs one Work before moving, and records the declared move progress, including on refusal. At 79–107 failed PngDecodeJob::new restores the original source. At 126–134 decoder Work and last-turn progress are received; at 141–163 output first moves into an owned intermediate, then the decode job is retired before the output pixels and binding are handed off. At 166–195 cancellation closes job, image and binding incrementally. Current PngDecodeJob::advance(effect) and take_result(grant) signatures match these call sites. This inspection alone does not prove Work accounting, allocator effects, denial, or cancellation correctness. Existing tests at 220–286 cover restored input, zero-grant denials, cross-caller refusal, pointer custody and cancelled retirement, but have not been executed here.

## Stale InteractiveJob implementations

The current trait requires a return-claim parameter and InteractiveJobStep return, as used by 📐️surface-lane/🦀️.rs:143. Old two-argument Result<Option<JobOutcomeBorrow>, OriginalTurnFailure> implementations remain at 🧵️frame-job/🦀️.rs:50 and 477, 🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🐣️create-app/🧩️compile/🦀️.rs:52, and task-ingress/🧪️tests/🦀️.rs:21. These are probable signature errors when compiled; preserve outcome authority and effects when changing them.

## Production kernel admission update

Task ingress and CreateApp now share the original retained construction admission. The settled admission moves failed constructor custody into a typed owner, retires it on the original Source, and publishes compiler state through a funded metadata effect. The compiler and frame jobs return the current interactive callback claim. The CreateApp client continues to pass borrowed fields through the real queue.

Five focused neutral tests passed with 201 assertions across the task-ingress, compiler and CreateApp source laws. Ajv and SQLite validate the shared fixtures. The Shell fault law follows the current prefs module and checks the localized notice reaches chrome. Rust source parsing passed. Native compiler and installed browser verification remain pending.
