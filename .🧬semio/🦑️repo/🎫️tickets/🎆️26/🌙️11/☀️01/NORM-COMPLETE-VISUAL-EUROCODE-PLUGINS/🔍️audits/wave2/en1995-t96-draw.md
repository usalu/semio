# en1995 T-96 drawing
1. Bracing drawing (cases + rows/special: dispatch, size, bracing()) keyed on `n_braced` instead of `count`; tags use `n_braced`.
2. Each stabilised member gets an N_d arrow shaft `force{i}` (both renderers), tagged `n_d`.
3. Grip `count_grip` now writes `n_braced` (path via the same stabilisedCount field).
4. `count` bind removed from timber entity bindings and the `count` number from the bracing case numbers; no other users.
5. Scene test lists `bracing-1:force1`/`force4`. No builds run; marked ready for rebuild.
