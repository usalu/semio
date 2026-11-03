# Span envelope (wave 2)

Nothing compiled or run in Rust (no cargo). Python oracle values were computed with PyNite/anastruct via the venv. `- ready for rebuild: span envelope` is in `requests.md`, integrate and build-en1990/1994/1995 `latest.md`.

## Shared implementation (mechanics, one copy)
- `🧮️mechanics/🌉️continuous-beam/🦀️.rs`: `SpanBeam`, `SpanFactors`, `SpanEnvelope`, `span_subsets`, `permanent_arrangements` (gamma_G,sup / gamma_G,inf per span, 2^n), `continuous_beam_envelope` (moved from en1990 `⚖️checks/🪜️span-arrangements`, which is deleted). Exported from `🧮️mechanics/🦀️.rs`. Placed in mechanics (not compliance) because the compliance crate has no mechanics dependency; factors are passed as plain numbers from `ActionFactors` (`gamma_g_sup/inf/gamma_q_building`).
- en1990 checks re-export the names explicitly.
- Shared oracle adapter `mechanics_wiring::SPAN_ENVELOPE` (`mechanics.beam-span-envelope`) in `🧮️mechanics/🎯️oracle-wiring`; mounted with `#[path]` by en1990, en1994, en1995 wiring (the three private copies are gone). Cargo: mechanics and `serde_json` added (optional under `oracle-wiring`, dev-dep) to en1990 inferences, en1994 checks, en1995 checks.

## en1994 (production fix)
- `catalogue/📉️continuous`: `Load::scaled`, `permanent_internals` (largest sagging/hogging/shear over `permanent_arrangements`, solved by the cracked-stiffness kernel).
- `part_en1990`: `action_load`, `design_permanent_internals` (simple span gamma_G,sup; continuous split); `accumulate` uses it for `uls` and `uls_construction` permanents (the `g_g`/`scale_perm` path removed). Effects: end span sagging +4.6 % (0.1946 -> 0.2036), interior hogging +1.9 %, shear unchanged, end-span hogging unchanged.
- Python oracle `🔮️oracles/🚦️check-outcome` (+ `📉️continuous`): `arrangement_effects`/`permanent_internals` (PyNite, 2^n arrangements); `effects` uses them for continuous ULS.
- Feature rows recomputed from the oracle: pass-11 0.203568507, pass-13 0.277993676, pass-14 0.127783903, pass-17 0.138698641, pass-39 0.110913115, pass-42 0.160350390, pass-43 0.074405482, fail-45 1.28684816, fail-52 1.91900877.
- Regression test (`🧪️tests/⚖️compliance`): two spans 6+6, G 20, Q 15 kN/m: M_sag 151 423.65 against 145 000.36; M_hog 222 750.
- Oracle case `en1994.mechanics.two-span-composite-gamma-arrangement-envelope` (`mechanics.beam-span-envelope`) replaces `mechanics.beam-envelope`; expected from PyNite (anastruct within 1e-4).

## en1995
- No production change: `ContinuousTwoSpan` evaluates the support moment/shear (both spans gamma_G,sup, the EN-correct arrangement) and the sagging is smaller (4.5+4.5, g 1.5, q 2.5 kN/m: sag 10 376.7 vs support 14 618.0).
- Test `the_two_span_support_moment_is_the_hogging_of_the_shared_arrangement_envelope` ties the governing ULS moment to the shared envelope.
- Oracle case replaced (`two-span-timber-gamma-arrangement-envelope`): M_sag 10 376.73 (old, wrong: 10 098.70), R_min_0 1 328.91 (old 2 531.25 was the all-inf reaction), others unchanged.

## Other codes
en1992 (only the deflection K for `Continuous`), en1993 (oracle `beam-continuous` only), en1999: no multi-span envelopes; no change. en1990 oracle cases and regression test unchanged (now via the shared code).
Harness: the `mechanics.beam-envelope` adapters in pynite and anastruct were removed (no remaining case).

## Open / unverified
- Rust never compiled: `#[path]` depths for the mechanics wiring mount (10 `../` en1990, 11 en1994/1995), the `mech::SpanFactors` path in en1994, and the Cargo feature wiring.
- Ranges: gamma_G,inf = 1.0 via registry (`ActionFactors::gamma_g_inf`); normative reading unverified against print.
- en1994 hogging combines the permanent maximum with the variable hogging maximum (sum of maxima, slightly conservative for interior spans); shear of the permanent action is the maximum over arrangements.
- Stale and unrelated: en1994 `🔮️oracles/⚖️compliance/🐍️.py` uses qL^2/16 closed forms for `continuous_2_span` (not the FE statics); not touched.
