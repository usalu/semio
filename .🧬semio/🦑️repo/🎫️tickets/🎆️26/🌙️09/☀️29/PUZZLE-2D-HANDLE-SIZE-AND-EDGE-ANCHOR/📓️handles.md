# Puzzle 2d Handle Size And Edge Anchor

The board parsed every puzzle 2d handle with `radius: None`, so the cap fell back to the default glyph radius of 8. Nakagin authors the caps at radius 3 on nodes of radius 20, which is what the screenshot was overshooting.

Edges were anchored on the node circle, at the centre of the cap. They now leave the cap's outer peak, along the handle's outward normal.
