# Fix Thermal Bridge Formula Overlap

## Problem
In Beat6 Wärmebrücken, the detailed `H_T` panel morphs into the flat-rate panel with recipe `replace`. Different MathTex rows reshape into each other, so glyphs stack for the whole transition (psi term and Delta U_WB visible at once).

## Fix
Same pattern as the equilibrium handoff: `_drop_panel(p_det)` then `FadeIn` / `Create` the flat panel. Removed unused `flat.formula-head` / `flat.formula-box` transitions.

## Tests
AST case `flatFormulaHandoff` forbids the replace morph ids and requires drop-then-appear helpers.
