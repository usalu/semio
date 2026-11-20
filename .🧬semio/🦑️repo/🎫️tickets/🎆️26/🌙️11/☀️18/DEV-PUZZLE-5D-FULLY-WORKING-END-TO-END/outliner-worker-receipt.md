# Outliner replay worker receipt

Replaying an already-applied outliner flag reached mounted maintenance stage zero. The typed worker returned a two-item, 270-byte receipt. Its own lifecycle had already accepted that receipt. The outer maintenance scheduler tried to settle it again against a one-item loan.

Stage zero now drives the worker directly with the remaining mounted grant installed on the same caller context. The worker still selects and settles its own phase. The maintenance rotation retains its cursor and accounts for returned resources separately from new spending.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/🦀️.rs`
