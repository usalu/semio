# Build-fixer v8 (en1994 run 16, 33 errors)
File (handwritten, not a twin): 🧩️en1994/.../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs
- Imported FatigueDetail, FireRating, StructureKind (27 E0425/E0433).
- bound_actions now takes [&str;4]; callers pass id, action_kind_text(kind), category, action_stage_text(stage) (3 E0308); column action used column_kind_text instead of action_kind_text (1 E0308).
- ProfiledSheeting bound uses sheeting_profile_text(profile).len() (E0599).
- unsigned(*b,19) deref (E0308).
Same pattern elsewhere: missing imports in sqlite snapshot io of en1991 (FireOccupancy), en1992 (CementClass), en1997 (ApproachChoice) added. en1990/1993/1998/1999/1995/1996 and DIN crates scanned: clean.
No freeze present at edit time. No generated twins touched.
