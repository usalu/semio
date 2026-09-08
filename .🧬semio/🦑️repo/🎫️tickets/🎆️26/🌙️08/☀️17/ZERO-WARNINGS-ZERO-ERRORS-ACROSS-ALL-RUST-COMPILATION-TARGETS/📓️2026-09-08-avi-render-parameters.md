# AVI Render Parameters

Native582 reported unused view_state parameters in the AVI HDRL editor and viewer render trait implementations. Both render from the document snapshot alone. Pass599 names those required but unused trait parameters _view_state. Function signatures, rendering bodies and behavior are unchanged; no artificial reads or lint allowances were added.

Files:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/👁️viewer/🦀️.rs`

Fresh compiler validation remains pending.
