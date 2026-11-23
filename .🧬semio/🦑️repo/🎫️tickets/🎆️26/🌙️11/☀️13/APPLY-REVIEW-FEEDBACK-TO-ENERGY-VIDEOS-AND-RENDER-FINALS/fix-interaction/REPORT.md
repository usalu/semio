# Fix: one pointerdown per press in render.mjs

Cause: every frame opens a fresh page, so a press spanning several frames reported one `pointerdown` per frame.

Change (`presentation/browser/render.mjs`): the replay branch now runs for every frame whose predecessor was pressed (not only release frames). It renders the previous frame, moves to the previous point, `mouse.down()`, then clears `pointerEvents`; the frame is rendered without reset. The early `mouse.up()` on an opened-path change stays limited to release frames.

Results (SEMIO_REPO_BINARY and PATH set to the newer repo): interaction 21 OK (446 s), browser 11 OK (320 s), posters 22 OK (212 s), hierarchy and hierarchy pointer see below. Chapters module (482 s) not re-run.
