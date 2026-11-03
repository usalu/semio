# Kind kebab (wave 2)
Renamed 79 mutation kind strings (en1991 4, en1992 14, en1993 2, en1994 5, en1998 30, en1999 24) to the derive's to_kebab(variant), content and folder names (leaf and fixture folders, #[path] strings), via `🧰️scripts/kind-kebab/rename.py <art>` (word-boundary, longest first; Rust identifiers untouched).
Verified: scanner 0; twins generate --under per artifact, then `check --strict` drift 0 (7745 identical); fixture checker 0 failures for all six; binary-tags and i18n-mutations ok.
New law test: `⚖️compliance/🧪️tests/🧷️kind-kebab/{🐍️.py,🥒️.feature}` (2545 leaves, 0 mismatches). Freeze at start named only en1997 (untouched).
