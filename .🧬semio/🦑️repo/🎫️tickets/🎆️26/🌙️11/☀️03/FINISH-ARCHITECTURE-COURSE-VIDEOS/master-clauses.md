# Clause Ranges On Lesson Masters

`preview-gate.md` showed that `master_fresh` compared a lesson sidecar with `segment_state({script, scene})`. Published chapters store `segment_state` of the curriculum segment, and that segment keeps `clauses`. Heating, cooling, and energy balance were therefore stale to the joiner even when the canonical source clip was reusable.

`lib.master_fresh` now compares each recorded chapter segment with the source fingerprint plus the declared curriculum segment, including `clauses`. Source clips stay clause-free. A source that is actually stale still blocks the master.

The check is `test_master.py` beside `lib.py` in the staffage `r3-media` folder. Three cases passed.

After the fix, `status.py en speaker --video heating --stale-only` reports heating `35/35` fresh and the master fresh. Cooling is `19/22` with `Beat1_WinterGains`, `Beat2_SummerOverheat`, and `Beat3_CoolingSystem` stale. Energy balance is `22/26` with `Beat1_DreiBilanzgrenzen`, `Beat2_Primaerenergiefaktor`, `Beat3_Rechenbeispiel`, and `Beat4_Referenzgebaeude` stale. Those two lessons still need a render. Their fresh shared segments are left in place.
