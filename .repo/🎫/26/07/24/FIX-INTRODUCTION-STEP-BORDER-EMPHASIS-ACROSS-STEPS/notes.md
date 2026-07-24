# Fix Introduction Step Border Emphasis Across Steps

## Problem
Step 1: hover → emphasized border, leave → normal. After Next, later steps kept
emphasized border even with the pointer outside.

## Root cause
`[data-slot="introduction-info-box"]:focus-within` stayed true because clicking
Next left focus on the Next button across step changes.

## Fix
Introduction info box emphasizes on `:hover` only. Dialogs still use
`:hover` + `:focus-within` (form fields).

## Validation
`bun ./script.ts test -t "introduction and dialog glass boxes emphasize"` — pass
