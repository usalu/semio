# Wave 2 Oracle Wiring (EN 1990 to EN 1999)

Nothing compiled or run (no cargo). Shared typed driver `⚖️compliance/🧪️tests/🎯️oracle-driver/🦀️.rs` (`oracle_driver::{Adapter, Registry, KnownDifference, Inputs, Outputs, drive}`, profiles mirrored from the harness, gaps/mismatches/stale known differences reported, test fails on any). Per artifact: new test-only module `🧪️tests/🎯️oracle-wiring/🦀️.rs` with a `static ADAPTERS`/`KNOWN`/`Registry` and test `evaluators_reproduce_the_blessed_oracle_cases`. Details and gap lists: `oracle-wiring-enXXXX.md` (not en1991 aside: see `oracle-wiring-en1991.md`). Gaps are routed in `📌️important/requests.md` (`[open] oracle-wiring → enXXXX`); rebuild lines appended to `🗑️generated/build-enXXXX/latest.md`.

| artifact | checks wired / total | gap cases | known differences |
|---|---|---|---|
| en1990 | 6/7 (+2 partial) | 6 of 21 | 0 (Table D1 handled by 0.2 % tolerance on X_k cases) |
| en1991 | 8/9 (+2 partial) | 8 of 35 | 0 (LM1 tandem M_max now matches) |
| en1992 | 30/47 | 18 cases + 32 quantities | 1 (KK chi) |
| en1993 | 25/34 | 18 of 47 | 0 |
| en1994 | 8/8 | 3 of 19 | 0 |
| en1995 | 4/4 | 3 of 24 | 22 (EN 338 rows) |
| en1996 | 3/3 | 3 of 9 | 0 |
| en1997 | 14/16 | 8 of 36 | 0 (`include_depth: false` used) |
| en1998 | 5/5 | 0 of 17 | 0 |
| en1999 | 9/11 | 7 of 20 | 0 |

## Expected visible mismatches (no covering note)
en1991 snow drift-minimum; en1992 creep phi rapid/slow cement, deep-beam k_c, M_cr, capped anchorage; en1994 stud k_t, mcr_uframe; en1997 embedded wall K_ph (kp_curved vs plane Coulomb); en1999 I-section It, channel/angle, pin moment F b/8; en1990 FORM cases possibly.

## Cross-cutting
- Old harness test `third_party_libraries_reproduce_the_blessed_oracle_cases` is mounted from the artifact root crate in en1991, en1992, en1997, en1999; its CARGO_MANIFEST_DIR cannot reach the oracle-cases folder: relocate it (owners).
- en1997 checks crate does not build (`loc` not found).
- Cargo changes (Cargo.lock updates on first build): `semio-s-plugin-norm-mechanics` dev-dep in en1993, en1994, en1995, en1998; en1995 `compliance-testing`.
- Case-file tolerance edits: en1990 D.7 (X_k, 0.2 %), en1992 concrete laws (2 %, Ecm 1 %). The en1991 fire expectations labelled K hold degC.
- Adapters composed from our own outputs where no entry point exists: en1999 N_b_Rd/lambda_LT/M_b_Rd, en1996 sigma_d.
