# SCENE v3 / F5: en1992 crack pattern (nothing compiled)

The 3D crack triangles (`members::cracks`, layer `cracks`) already existed. Added the missing tension-face view.

- New module `🎬️scene/🪨️crack-pattern/🦀️.rs` (mounted privately in scene `🦀️.rs`, hook in `📈️diagrams`): diagram `<member>/crack-pattern`, only for non-column members whose check `en1992.7.3.crack.<id>` has stations and a cracked zone.
- Crack positions: `crack_positions(lo, hi, s_r,max, 40)` centred in the zone where the check stations give w_k > 0; s_r,max from `mechanics::crack_state` (the same value the 3D cracks and the section diagram use; the check result itself does not export s_r,max), w_k = station utilisation field x w_max of the check (not recomputed).
- Width coding (non-colour cues too): w_k <= w_max/2 dotted Info 1 px, <= w_max dashed Warning 2 px, > w_max solid Error 3 px; legend EN+DE, description cites the check id. Table twin: position, w_k [mm], w_k/w_max, s_r,max, crack height (h - x_na).
- Test `the_crack_pattern_places_lines_at_the_crack_spacing_coded_by_the_check_widths` (appended): spacing/centring worked values (zone 1.0-2.0 m, s 0.3 m -> 3 cracks, first at 1.20 m), cap 40, class thresholds, beam-B1 s_r,max = 0.2196804883 m (oracle const), spacing of all lines, table twin 5 columns, diagram validates.

Open: 3D crack triangles are still Neutral material (not width-toned); scene golden re-bless; `Dash::Dotted` for Fine assumes renderer support (enum has it). Not compiled: `super::norm` import and `pub(crate)` visibility unverified.
