# View model ordered map scoped alias

Descriptor generation `w73S37W` reported E0599 for the two scoped alias returns in the view model. The build observed changes to both owners during this attempt.

The live view model imports the canonical ordered map through its direct path dependency `semio-framework-value-ordered`. The manifest selects the library shim in `🧰️framework/🔨️modules/🌱️value/🗂️ordered/📦️packages/🦀️rust/🦀️.rs`, which unconditionally reexports the ordered map owner two levels up. Its public `return_scoped_alias` method is on `impl<V> OrderedMap<V>`, so both the optional string and trace cursor maps can call it without additional bounds.

The scoped window projection carries a borrowed original and clones its map roots. Drop returns the utility alias before the trace cursor alias. The return method checks exact root identity before surrendering the logical alias, requires the borrowed original to retain ownership, and empties the scoped root before ordinary drop. Static checks verified the selected paths, method placement, and return sequence.

The current source owners agree after a concurrent workspace change. This investigation made no production edits and started no Cargo command. A fresh descriptor compile is the remaining integration confirmation.
