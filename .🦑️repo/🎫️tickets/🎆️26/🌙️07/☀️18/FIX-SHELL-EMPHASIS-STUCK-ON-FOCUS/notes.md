# Notes

## Bug

Clicking a toggle or pane left focus inside the shell. `:focus-within` on navbar/footer/panel/pane/dialog kept the parent border emphasized after the cursor left.

## Fix

Shell parent-hover emphasis is `:hover` only — no `:focus-within` on navbar, footer, panel, pane, introduction info box, or dialog box.
