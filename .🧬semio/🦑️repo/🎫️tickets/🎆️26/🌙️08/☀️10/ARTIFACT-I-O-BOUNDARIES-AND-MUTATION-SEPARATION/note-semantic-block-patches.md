# Note Semantic Block Patches

`NoteBlockPatch.block` retains a typed canonical `NoteBlockNode`; it no longer carries JSON text. Whole-block patch construction, block collection validation/application, and drag mutation diff construction now live in canonical schema owners. The text/binary facets encode only at IO. TypeScript declarations and shared neutral fixtures are coordinated with the TypeScript owner. Native compilation is pending.

- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🚪️io/📝️text/🧬️mutations/🦀️.rs
