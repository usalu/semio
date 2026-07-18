# Norm Technology — Part Checklist

All parts implemented with public `check_*` / `compute_*` entry points and e2e unit tests.

## norm_core — done
## norm_din_4108 — parts 2,3,4,6,7 — done
## norm_din_en_16798 — parts 1,3,5,7,9,13,15,17 — done
## norm_din_v_18599 — parts 1–12 — done
## norm_en_1990 — done (na_de)
## norm_en_1991 — parts 1_1–1_7, 2, 3, 4 — done (na_de)
## norm_en_1992 — parts 1_1, 1_2, 2, 3, 4 + fem_core — done
## norm_en_1993 — parts 1_1–1_12, 2–6 — done
## norm_en_1994 — parts 1_1, 1_2, 2 — done
## norm_en_1995 — parts 1_1, 1_2, 2 — done
## norm_en_1996 — parts 1_1, 1_2, 2, 3 — done
## norm_en_1997 — parts 1, 2 — done
## norm_en_1998 — parts 1–6 — done
## norm_en_1999 — parts 1_1–1_5 — done

Verification: `cargo test -p norm_core -p norm_din_* -p norm_en_199*` — 17 tests passed.
