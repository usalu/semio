# EN 1992 compliant examples

Root cause: the authored `compliant-*` DSL documents were under-designed, not an engine bug. Failing checks (Eurocode 2 semantics):

Prestressed beam PS1 (2 fails)
- 7.4.2 span/depth: l/d 19.4 > 17.5, because 8 O20 in 350 mm gave rho 1.16 %. Fix: 5 O20 (rho 0.72 %, limit about 21).
- 8.2 clear bar distance: 8 O20 in one layer left under 21 mm. Fix: 5 O20 gives 42 mm.

Office frame (10 fails)
- Beam B1: torsion longitudinal steel in tension chord (1.13) and compression chord (no top bars). Fix: 5 O20 bottom, 2 O16 top (poor bond); anchorage 0.9/1.0 m, laps 1.0/1.2 m (l_bd 757 mm > 700 mm).
- Column C1: M_Ed 56 kNm > M_Rd 54, second order 61 kNm, 8 O20 in one layer (clear 11 mm), anchorage 853 mm > 700, lap 853 > 850, fire a 35 < 36 mm. Fix: section 0.35 x 0.40 (d 0.36), 4 O20 bottom + 4 O20 top, anchorage 0.9 m, lap 1.1 m, axis distance 0.048 (cover + stirrup + phi/2).
- Slab S1: l/d 33.3 > 31.1. Fix: h 0.30, d 0.26.

Failing examples untouched (their own DSL assets and fixtures); the Rust fixtures in the snapshot schema were intentionally not changed.

Changed files (under `en1992/standards/1/subsets/any/`):
- `assets/compliant-prestressed-beam/.../dsl.semio`
- `assets/compliant-office-frame/.../dsl.semio`
- temporary DEBUG eprintln in the two compliant example tests added and removed again.

Result: `test result: ok. 7 passed; 0 failed` (73.83 s) before DEBUG line removal; the removal is a pure deletion.
