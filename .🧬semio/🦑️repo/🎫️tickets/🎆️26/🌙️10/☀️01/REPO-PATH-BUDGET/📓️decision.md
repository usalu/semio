# Path budget outside norm

Measured 2026-10-01. Budget is `collisionPolicy.maxPathBytes = 240`. Norm is excluded; ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING` owns it under design section 14.

## Applied

Option B, on word boundaries only. A case directory on an over-budget path becomes one emoji plus the first hyphen-word (at most 12 ASCII bytes) when that rename brings a file to 240 or under. A leaf drops trailing hyphen-words the same way. A name is never cut inside a word. A rename is kept only when at least one file under it lands on budget. The taxonomy tree, the bundle layout, and U+FE0F stay unchanged.

2,183 fixture directories were renamed. 1,117 schema twins then moved to those same names (237 mutation leaves and 880 test cases), and 51 reference files were updated to the schema paths. Norm was not edited.

## Still over budget

960 files remain over 240. The longest is 301 bytes. No legal slug can clear them: the `standards` / `subsets` / `editor` spine is already at its taxonomy minimum, and mid-word abbreviations are not used.

| Class | Files |
|---|---|
| editor or viewer | 346 |
| other mutation evidence | 411 |
| import or export | 124 |
| examples | 79 |

Largest shares: architect 414, gis 115, cad 79, stdio 76, writer 56, trinity 44, energy 33, reasoning 30.

Option A (drop `/standards/<version>/subsets/<subset>`, 51 bytes) is not applied. Artifacts own standards and standards own subsets, and norm is keeping that shape. Collapsing it only outside norm would leave two tree shapes. A alone still leaves the worst editor path over (301 - 51 = 250). Option C saves 12 to 14 bytes. Option D fights the single-emoji-grapheme rule.

## Scripts

`census.py`, `apply.py`, `twins.py`, and `spine.py` stay in this ticket. Raw counts are in `generated/`.
