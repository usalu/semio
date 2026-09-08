# Copy and Backbone Warnings

Pass563 removes the unused test-only `Copied::retire` trait member and its `Value<T>` implementation. Inspection of every retirement call in the copy module and its tests confirms that cursor cleanup calls `Task::retire`, typed results call `Retire::retire`, and rooted snapshot cleanup calls the snapshot retirement factory. These ownership paths are unchanged. The unused erased-result method had no callers.

The fresh native560 compiler check also reported an unused `DocumentBackboneBindingStateV1` import inside `plugin_handle_document_backbone_binding`. Only this unused name was removed; state inference and all binding decisions are unchanged.

Files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

Compilation and runtime verification remain pending. These edits occurred after native560 started and require a subsequent compiler check.


Pass565 validation: 2/2 files parsed with unchanged source hashes. The existing language-neutral Flow selected-copy schema and third-party canonical oracle passed with 15 assertions.  This does not replace Rust runtime verification.
