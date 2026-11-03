# EN 1997-2 ground investigation (wave 2, partial)

Nothing compiled. Gates: spec-trace verifier (en1997 60 ids ok), oracle catalogue module imports, strict schema twin check via heavy.sh (no schema JSON touched, expected clean), source-boundary scan (no en1997 entry).

## Done
- Trace rows GI-01 ... GI-16 added to `🧭️spec-trace/🔣️.json` (twin regenerated with the shared codegen `--write`; side effect: it rewrote the spec-trace twins of all codes with identical content, only mtimes moved). All GI entries were already implemented in the catalogue (`📇️catalogue/🔬️investigation`), checks for lab series (`🌐️geo-entities`), and cases/oracle (`📇️catalogue.py`, scipy t quantile, linregress). Keys `en1997.gi-NN.` follow the catalogue bridge; GI-01 additionally carries the project check `en1997.2.4.2.investigation.`.
- G-02 key `en1997.gi-17.` fixed: G-02 (characteristic value X_k, spec 2.2) now traces `en1997.g-02.` with entity `cases`; the lab series lower bound (code entry GI-17, not in the spec) is listed under GI-13.
- GI-01 3D: every footing gets `<footing>:investigation-ghost`, a translucent column (opacity 0.18) from the ground surface to the required depth z_a = max(6 m, 2 B) (`part_2::min_investigation_depth_for_gk`), coloured by the utilisation of its investigation check; detail says covered or too shallow (EN + DE). Files: `🎬️scene/🦶️footings/🦀️.rs` (`investigation_ghost`), test `every_footing_carries_a_ghost_column_of_the_required_investigation_depth` in `🎬️scene/🧪️tests/🔬️unit/🦀️.rs`. Scene golden/entity counts move (one entity per footing): regenerate with SEMIO_UPDATE_GOLDEN=1 after the build.

## Not done (blocked or too large without a compiler)
- `investigations[]` collection (boreholes/CPT as rows): needs a new collection through ~80 files (model, 5 twins, diff, SQLite, io text, mutations, edit rules, fixtures). `new_collection.py` is hard-wired to en1991; no en1997 profile; the `field_adder` profile has no `layers` entity either. Needs a profile first (owner decision / tooling task).
- GI-03 feeding GEO-F02/SLOPE: needs a layer field `cptNkt` (user input, 0 = not given; no invented value) plus `layer_cu_pa` replacing `layer.cohesion_undrained` at ~15 sites of the footing checks. Registry row for N_kt requested (requests.md). The entry `gi_03` still defaults nkt 15.0 (unverified).
- SPT/pressuremeter/oedometer correlations as layer-derived values, soil profile diagram (q_c/N/c_u vs depth): same blocker (layers lack the input fields; `cpt_qc`, `spt_n` exist, pressuremeter/oedometer series not on layers). Characteristic derivation (GI-13) is complete in code and oracle (scipy.stats.t.ppf); the Rust t quantile is tabulated and interpolated between nu = 10, 20, infinity (differs slightly from exact for nu 11-19).

## Open
- [open] en1997 -> shared: N_kt registry row, spec GI-17 (see requests.md).
