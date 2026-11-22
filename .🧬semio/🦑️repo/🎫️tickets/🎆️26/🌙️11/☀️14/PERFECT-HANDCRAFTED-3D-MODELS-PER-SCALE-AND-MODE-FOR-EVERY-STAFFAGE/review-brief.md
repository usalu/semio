# Review brief — independent acceptance of the reworked staffage (2026-11-22)

You did not author these models. You are the dev's eye: the previous verdict was "modeled incredibly sloppy: limbs and objects completely wrong proportioned, missing detail, wrong skeleton". An author has since reworked the assets of `T/rework/<NN>.txt` and reported in `T/rework/<NN>.md`. Do not trust the report; look.

`T` = this ticket folder, `S` = `🎓️teaching/🏛️architecture/🧍️staffage`. Tools and rules: `T/rework-brief.md` (read it completely; its tool list, its shared-worktree rules and its one-process limit bind you too) and `T/r3-tools.md`.

## Per asset

1. Render `r3-sheet` at 1:50, 1:100, 1:200, 1:500 for `body`, `thermal`, `structure` in the default pose; at 1:50 every other pose and two instants of every clip; close-ups (`--only`) of the hard parts (hands, face, feet, joints, wheels, hardware, crown interior). Render `r3-2d` at every scale. Read every image.
2. Write your own reference numbers for the subject first, then measure the model (rig bone lengths, section sizes, extents) and list every deviation over 5 %.
3. Verdict per asset and per mode at each scale, in `T/review/<NN>.md`: `accept` or `reject` with the concrete defects (what, where, measured against what). Reject anything a professional modeller would not sign: wrong proportions, wrong joint order or pivots, asymmetric rigs, floating or interpenetrating parts, cracks at joints in any pose, boxes standing in for organic or turned forms, missing characteristic detail at 1:50, coarse scales that are deleted lists instead of generalised models, thermal or structure lists that are not the same object, structure that is not the real load path or skeleton, 2D drawings that disagree with the model, `r3-validate.ts <asset> --catalog-only --volumes` not `VALID`.
4. Fix what you reject yourself, by hand, to the standard of `T/rework-brief.md` step 3, validate, publish with `volumes-insert.py`, re-render, and record before/after in your review. Leave nothing as "noted".

Return only the review path and at most five lines (accepted / fixed / still open).
