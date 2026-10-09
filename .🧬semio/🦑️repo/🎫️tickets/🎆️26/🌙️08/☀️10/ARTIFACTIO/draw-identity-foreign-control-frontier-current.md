# Draw Foreign IO Control Frontier

This is a remaining architectural candidate, not an exemption and not a completed runtime claim.

Draw SVG production owner: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs`. Its original `SvgIntoDraw` implements the framework foreign Deserializer and is registered in Draw any/io/🦀️.rs with deserializer_entry. Its direct native fixture owner is the adjacent document/unit test; text, binary and malformed UTF8 vectors remain.

Shared owner: `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs`. `Deserializer::deserialize(payload)` lacks caller control. `IoEntry.run` is `fn(&IoPayload)->IoResult<IoPayload>`. `resolve_run` calls that pointer with only the current physical payload. `deserializer_entry` invokes the uncontrolled trait method and then `encode_pack`; its text sibling prints DSL. `io_run_with_snapshot_control` supplies its real progress/control only to semantic SQLite hops, while foreign entry hops use resolve_run. The public io_run convenience entry additionally synthesizes an always-true progress callback.

A controlled inherent SVG helper would leave this actual registered runner unbound. A private thread-local/global callback supplier, optional synthetic fallback, or root forwarding alias would not close ownership. Required next design is an explicit control-bearing physical runner contract and propagation from actual host entry lifetime, including native snapshot output assembly. The foreign Deserializer future currently requires Send; borrowing NativeEncodeControl with a non-Send callback across that future requires an intentional contract choice, not a blind signature substitution. No shared framework edits were made by this Draw receiver pass.

The Draw generic synchronous paste override was removed because the actual app registers its reserved clipboard job. The explicit synchronous clipboard API now requires supplied control; the reserved original paste job retains the real caller StepContext and cancellation lease. No replacement token is fabricated.
