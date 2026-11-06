# External equality parent chain

Energy paths below are relative to `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`.

- `🧳️native/🏷️text/🔎️name/🦀️.rs` NameLookup is owned by `🧳️native/🏷️text/✅️validation/🦀️.rs` TextValidation, implementing OwnedRowValidation from `🧳️native/🧬️value/🦀️.rs`. OwnedRowPreparation implements RowFieldPreparation there; `🧳️native/🧬️value/🪆️leaf/🦀️.rs` supplies the leaf adapter.
- `🔋️model/🧳️retained/🦀️.rs` IdentityChangePreparation is held by `🔋️model/🧳️retained/🪆️leaf/🦀️.rs` IdentityLeafPreparation, implementing RetainedMutationLeafPreparation. Its current decide takes body and grant, returning Result with ValueError.
- `🧳️native/🔢️scalar/🦀️.rs` CursorClose and close_cursor expose only grant and ValueError. They cannot preserve child actual work, construction allocation marker, or OriginalRetirementFailure custody. OwnedRowValidation also exposes only grant and ValueError. Work quotation and genuinely funded remaining work, actual construction receipts, and typed errors must reach the relevant parent interfaces and caller-owned frames. Equality owner Box birth requires genuine construction custody.

The procedural fold is `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🧬️generation/🧵️fold/🦀️.rs`. It implements OriginalScratchWork from `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🌊️flow-source/🎟️driver/🦀️.rs`. The trait accepts source and grant only, while drive calls advance before receive_original_retained_work. Retirement uses close_original_retained_step without an explicit selected-work quotation. The actual original work loan and construction frame, selected work and typed retirement failure must reach the child through this chain; a fabricated marker or fixed fuel does not establish that authority.

This is a source audit only; no tests were run.
