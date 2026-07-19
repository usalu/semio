# Norm Feature Complete — Part Checklist

All parts implemented with clause-cited formulas, DE NA where applicable, and worked-example tests.

## norm_core — done
## norm_din_4108 — parts 1,2,3,4,5,6,7,8 — done (Glaser, climate U-limits, 17 materials)
## norm_din_en_16798 — parts 1-16 — done (PMV, CO2, full ventilation tables, na_de)
## norm_din_v_18599 — parts 1-12 — done (wired to 4108 + 16798, from_building)
## norm_en_1990 — full combinations 6.10/6.10a/6.10b, SLS, DE ψ/γ/ξ tables — done
## norm_en_1991 — correct part mapping, wind/snow/DE zones — done
## norm_en_1992 — flexure/shear/punching/torsion/crack/fire/FEM — done
## norm_en_1993 — section class, buckling curves, distinct parts — done
## norm_en_1994 — composite partial interaction, b_eff, V_L — done
## norm_en_1995 — k_mod matrix, LTB, fire charring — done
## norm_en_1996 — masonry shear/sliding/fire/retaining — done
## norm_en_1997 — DA1/2/3, Meyerhof bearing, piles — done
## norm_en_1998 — spectrum base shear, q factors, distinct parts — done
## norm_en_1999 — alloys, HAZ, fatigue, hollow sections — done

Verification: 94 tests passed via `CARGO_TARGET_DIR=/tmp/semio-norm-target-fc cargo test -p norm_*`
