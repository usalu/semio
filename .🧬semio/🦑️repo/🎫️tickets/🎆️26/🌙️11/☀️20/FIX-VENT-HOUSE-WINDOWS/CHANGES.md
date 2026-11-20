# Outer Leaf House Windows Everywhere

## Problem
House windows read as flat teal sticks floating mid-wall (hinge on the inner leaf, almost no joinery).

## Shared fix (`🏗️authoring/🎨️visuals`)
1. `window_glyph`: sash flush with the **outer** wall face (`hinge = x − inward·wall/2`).
2. Readable closed pane, cross mullion, sill/lintel caps, exterior edge line.
3. Sash stays two-part so `open_window` still morphs; open stroke/glass opacity matched.
4. `house_section` uses the richer depth.

## Clip wrappers
- `❓️need` / `🌿️natural` `_window` thin wrappers (closed by default).
- `📦️volume` `_pf_house` keeps both façades on the shared glyph.

Every beat that builds houses through `house_section` / `window_glyph` picks this up (Luftwechselrate, Lüftungsweg, need, natural, mechanical, heating consumers of the same glyph).
