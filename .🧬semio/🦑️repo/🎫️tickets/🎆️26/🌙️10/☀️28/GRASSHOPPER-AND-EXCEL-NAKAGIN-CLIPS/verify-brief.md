# Final Verification Brief (Haiku)

You verify one video after its fix round. Edit nothing except your report `T/verify-<series>-<topic>.md`; you may run the keyframe capture (it writes only to `T/🗑️generated/`).

1. Re-capture both languages first (fresh frames): `cd /c/git/semio; C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe T/capture.py <series-folder> <topic-folder> <SceneClass> en` and then `de` (run one language at a time, in the background if > 2 min, and wait for "captured").
2. Read the earlier audit `T/audit-<series>-<topic>.md`. For every blocker and major: state FIXED / NOT FIXED / PARTLY with the frame that shows it.
3. Look at every fresh PNG in both languages; list only NEW blockers/majors (overlap, clipping, wrong value, contradiction between caption and drawing, wrong German, a number without object/unit).
4. Verdict: READY or NOT READY, with the short list of what must still change. The 🏢️nakagin model is authoritative for all numbers.

Return a 5-line summary.
