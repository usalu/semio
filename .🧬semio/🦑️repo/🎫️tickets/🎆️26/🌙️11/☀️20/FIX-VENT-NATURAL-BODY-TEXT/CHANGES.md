# Import Body Text For Stoss Kipp Labels

Scene: Beat2_StossOderKipp.

## Problem
`NameError: body_text is not defined` at pose labels (weit auf / gekippt).

## Fix
Import `body_text` from the fonts module alongside `LABEL_FONT_SIZE`.
