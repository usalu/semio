# Sun Wash Opacity

## Problem
The yellow Polygon washes onto Verschattung / Speichermasse still read too solid on the Nachtlüftung day frame.

## Fix
Lower `fill_opacity` on both washes in `Beat5_Nachtlueftung` (`rays` and `leak`).

## Check
Preview frame during the shade caption: wash is a faint yellow haze; blinds and Speichermasse stay readable.
