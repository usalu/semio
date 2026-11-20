# Hide Counting Chips When Formula Shows

Scene: `Beat1_Lueftungsbedarf` personen → jahreszeit.

## Problem
`4 ×`, `30 m³/h` and `= 120 m³/h` stayed under the room after the formula panel appeared (readout updaters survived a plain `FadeOut`).

## Fix
`_hide` clears updaters and fades the strip, then `remove` so the season beat only shows the formula.
