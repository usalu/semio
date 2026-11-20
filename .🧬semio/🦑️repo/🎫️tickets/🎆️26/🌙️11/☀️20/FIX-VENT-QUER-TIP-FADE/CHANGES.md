# Smoother Querlueftung Tip Fades

Scene: Beat3_Querlueftung.

## Problem
Top labels (`A_eff`, Sommer tip, Winter tip) swapped with abrupt `replace` morphs at ~0.9–1.0 s.

## Fix
1. Tip and Aeff transitions use recipe `fade` with `ease-in-out`.
2. Durations raised to 1.4–1.7 s; scene `run_time` matched.
3. Initial `A_eff` FadeIn uses smooth rate.
