# WGPU Kernel API Audit

Source inspection found additional callers of the changed actor and job APIs. Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu`.

- `🧊️renderer/📊️scale-bench/🦀️.rs:100`, 260 and 317 and `🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs:305` still construct `TurnBudget` with `deadline_ms`.
- `📊️scale-bench/🦀️.rs:277` and 336, and `📦️kernel-runtime/🏊️kernel-pool/🚀️turn-dispatch-and-pool-runner/🦀️.rs:237` construct actor `TurnResult` without `retained_receipt`. `📦️kernel-runtime/🧮️kernel-core/🦀️.rs:281` also constructs the kernel result and needs the original receipt carried across the conversion. These modules are linked by the WGPU kernel runtime and protocol crate roots.
- `🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🎟️task-ingress/🦀️.rs:108` and `🧊️renderer/🛎️frame-actions/🦀️.rs:20` use the old one-argument `admit_loan(cx)`. The current declaration in `🧰️framework/🔨️modules/🧵️job/⏱️context/🎟️source/🦀️.rs:125` takes a mutable output option and context, returning a boolean. The same old call occurs in native-caller tests at lines 21, 42, 44 and 58 and CreateApp source test line 19.

This is a static scan, not a compilation; active edits may have changed these locations since inspection. A proper receipt must preserve the issued turn's ownership rather than fabricate a default. No production files were edited or compiler started.
