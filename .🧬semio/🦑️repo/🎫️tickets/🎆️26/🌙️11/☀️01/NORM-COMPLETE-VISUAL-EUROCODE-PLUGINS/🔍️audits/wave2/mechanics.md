# Mechanics wave 2: finite-element cross-section solver in the shared kernel

Nothing was compiled or run in Rust (rule). The algorithm was prototyped line by line in Python (`🧰️scripts/mechanics-section/{proto,thin_proto,profiles_proto}.py`, run with `.venv/bin/python -I`) and measured against sectionproperties 3.10.2; the Rust files are the port, checked only by reading. Rebuild requested in `🗑️generated/mechanics/latest.md`, `build-en1999/latest.md`, `build-en1993/latest.md`.

## Kernel (`✏️s/🔌️plugins/📕️norm/🧮️mechanics/🔲️section`, `pub mod section`, re-exports `analyse_section`, `analyse_section_with`, `analyse_thin_walled`)
- Schema first: `SectionMethod`, `SectionOptions`, `ThinWall`, `SectionConstants` in `🧬️schema/🦀️.rs` and the proto, graphql, ts twins (the existing twin parity test covers them).
- `🕸️mesh`: ring cleaning (collinear and duplicate vertices), orientation, hole bridges + ear clipping, Lawson flips, Rivara 4T-LE longest-edge refinement to `2A/P / cells_across_wall` (default 4), triangle cap 40 000; deterministic (BTreeMap/BTreeSet), `Control` checkpoints.
- `🧊️fem`: six-node triangles (6-point degree-4 rule), pinned Neumann solves for the warping function and the two Pilkey shear functions with one shared skyline Cholesky (RCM from `statics::rcm`, now `pub(crate)`); outputs J, I_w about the elasticity shear centre, shear centre, shear areas with Poisson ratio from the material (`nu = E/2G - 1`). Exact polygon formulas for area, centroid, inertia, elastic and plastic moduli (plastic by half-plane clipping and bisection).
- `🪶️thin-walled`: open centreline wall tree (T-junction splitting, cycle and disconnection refused): centroid, inertia incl. the t^3/12 terms, J = sum l t^3/3, sectorial coordinates with Trefftz orthogonality for the shear centre and I_w, shear areas from the shear-flow energy. Elastic and plastic moduli are zero (no outline).
- `📐️profiles`: outlines of I, mono-I, channel, tee, angle, RHS (rounded), EHS, CHS, rectangle, ported from the sectionproperties constructors (symmetric difference with the library polygons 1e-12).
- Progress stages `mesh`, `assemble`, `factorise`, `solve`, `integrate`; `AtomicBool` cancellation.
- Tests `🧪️tests/🔲️section/🦀️.rs` (13): sectionproperties oracle for 14 cases at 0.5 % (python `🔮️oracles/🔲️section/🐍️.py`, about 60 s, independent polygon lists plus the library constructors), Poisson 0.3 shear areas, Prandtl series and 5/6 Cowper factor, CHS polar J and no warping, translation/orientation invariance, mesh conformity (area sum, manifold edges, boundary length), progress and cancel, determinism, refusals, thin-walled consistency with FE (I, channel, angle, tee) and closed forms (J, I_w = t_f b^3 h^2 / 24), profile areas. Four prose scenarios appended to `🧪️tests/🧮️mechanics/🥒️.feature`.

## Prototype results against sectionproperties (relative difference)
| case | mesh edges per wall | J | I_w | A_v,y / A_v,z | shear centre |
|---|---|---|---|---|---|
| rectangle 70x140 | 4 | 0.005 % | 0.004 % | 0.002 / 0.0002 % | 1e-13 m |
| I 100x120x8x6 sharp | 3 | 0.05 % | -0.007 % | 0.02 % | 2e-7 m |
| channel / angle / tee sharp | 3 | 0.03 / 0.05 / 0.07 % | -0.005 / -0.006 / -0.007 % | <= 0.04 % | <= 4e-6 m |
| RHS sharp, CHS 96-gon | 3 | 0.006 / 0.001 % | 0.03 % / n.a. (I_w = 0) | <= 0.01 % | 1e-7 m |
| rectangle with hexagon hole | 4 | 0.09 % | -0.24 % | 0.06 / 0.14 % | 4e-5 m |
| IPE300, PFC200, L100x10, tee, plate girder, rounded RHS, EHS (library outlines) | 2 | <= 0.05 % | <= 0.011 % | <= 0.02 % | <= 2e-6 m |
| rectangle, Poisson 0.3 | 4 | 0.005 % | 0.004 % | 1e-4 % (0.0076875, 0.0081628) | 0 |
Thin-walled against FE (sharp polygons vs centreline walls): J +2.1..3.3 %, I_w -1.6..+0.24 % (angle and tee near zero), shear centre 0.35 mm, A_v -2..-6 %, area +0..2.4 % (junction overlap) as asserted in the test tolerances (4 %, 3 %, 1 mm, 10 %, 3.5 %).

## EN 1999
`⚖️checks/🌀️section-solver`: raster `solve`/`Solved`/unit tests deleted, crate keeps `stability`. `🪢️free-outline-torsion::free_constants` calls `analyse_section` (E 70 GPa, G 27 GPa); `wall_cells` = mesh edges across the mean wall (Warning below 3), EN/DE info text updated. Cargo dependency and hand-edited `Cargo.lock` edge added. Previous accuracy about 3 % on I_t, now 0.5 %.

## EN 1993
`⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs`: adapters `en1993.section.{tee, mono-i, ehs, rhs, channel, angle}`, `en1993.6.3.2.mcr-i` (via `stability::mcr_closed_form`), IPE300 fallback in `rolled_i_catalogue`; registry 27 -> 34. Quantities offered by `kernel_section`: A, Iy, Iz, I_major, I_minor, It, Iw, Wel_y/z, Wpl_y/z, y_sc, z_sc. Requests `[mechanics]` in `📌️important/requests.md` (announce, en1999 done, en1993 done, en1994 open).

## Open / unverified
- Not compiled. Likeliest rustc problems: closure/borrow details in `fem::solve` (`pinned`/`dot` closures), `or_insert_with` capturing `coords`, `fold` closure with `fn(f64,f64)->f64` pointer in `section::geometry`, `n.saturating_sub(1).max(1)` precedence in `profiles::arc`, array `extend` calls in profiles.
- Cargo.lock edits of en1999 by hand (checks package); en1993 already had the optional mechanics edge.
- Test run time in a debug build: the plate girder (about 6.7k triangles, about 14k nodes) and 14 sectionproperties cases; the python oracle takes about 60 s. Default `cells_across_wall = 4` gives up to 21k triangles for fillet-rich 0.8 m sections (skyline factorisation about 5e9 flops in release); tests use 2 to 4 per case.
- Mesh is uniform (no grading toward re-entrant corners); sharp re-entrant corners converge about 0.05-0.2 % on J at 3 edges per wall.
- Polygon-hole warping constant (1.4e-10, nearly zero) is the tightest case (-0.24 % at 4 edges per wall against the 0.5 % tolerance).
- Plastic moduli are about the geometric y and z axes through the equal-area lines (sectionproperties `get_s`), not the principal axes; the thin-walled solver gives none.
- Hole bridges avoid vertices that already carry a bridge (multiple holes tested only by reading).
- Closed thin-walled cells are refused by design (use the FE solver). EN 1994 `Iw` and plastic fillet moduli can now be wired by their owners (open request).
- The HEB catalogue It/Iw mismatch (third-party finding) is not touched; `rolled_i_catalogue` still compares the catalogue rows for HEB.
