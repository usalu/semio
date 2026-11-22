# Packed spatial receiving

The rebuilt editor reached archive inverse replay. Its first populated part row stopped with `owned receiving source differs from its declared field`. The independent v1 codec law reproduced the same error. A diagnostic rerun corrected the mistaken fixture ordinal only in the debugger process so the old binary could reach the decoder. The subsequent breakpoint was in the owned `[f64; 3]` receiver for `Puzzle5dPart3d`, reached through the parts table of a scene splice. The diagnostic is recorded by `scene-codec-fixture-byte.gdb`.

The canonical packed numeric tag carries no tuple marker. The ordinary Pack decoder recovers the tuple from the declared shape. The retained token decoder only recognized explicit Tuple declarations and constructed a List for Coord, Dir, Dim, and Range. It now recognizes the same fixed shapes as the ordinary decoder.

The SQLite oracle independently checked the arity and numeric fields for a coordinate, direction, dimension and range. It also assembled the simple deletion example with its ordinal-zero first field. In the rebuilt v1 crate, the scene-splice law passed the direct, retained and prepared byte comparisons and replayed both archive forms. The standalone Pack unit target uses obsolete receiving and retirement test interfaces and failed to compile. The v1 law supplies the native integration regression.
