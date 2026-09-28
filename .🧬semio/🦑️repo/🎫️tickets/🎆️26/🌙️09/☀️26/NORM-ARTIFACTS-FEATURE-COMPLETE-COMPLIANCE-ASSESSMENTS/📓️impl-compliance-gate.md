# Compliance gate — 2026-09-28 15:06

Command: `NX_DAEMON=false bun nx run @semio-tech/norm-plugin:test --skip-nx-cache -- quick --test compliance_gate`

Summary: `1 test run: 0 passed, 1 failed, 0 skipped` in 1.159s. Compile succeeded. Panic: `10/15 families failed compliance gate`.

Pass: din16798, en1990, en1991, en1995, vdi3805.

Fail: every listed check stayed `Fail` after `apply_remedy_edit` (option index 0) and after applying every applicable remedy in order. A lower utilization is not a clear.

- din4108: din4108-2.summer.zone-living; din4108-2.zone-ht.zone-living; din4108-6.u-prime.wall-north; din4108-10.app.wall-north.thin-eps; din4108-6.u-prime.roof
- din18599: din18599.2.heating-demand
- en1992: en1992.6.1.flexure.acc.beam-B1; en1992.7.2.sigma-s.beam-B1; en1992.7.2.sigma-c.beam-B1; en1992.7.2.creep.beam-B1; en1992-4.cone.anc-1; en1992-4.splitting.anc-1; en1992-4.interaction.anc-1
- en1993: en1993.1-8.4.5.directional.joint-w1 (utilization landed at 1.0000 and stayed Fail)
- en1994: en1994.6.6.6.vlrd.beam-B1; en1994.9.7.2.mrd.slab-S1; en1994.6.6.6.vlrd.girder-G1; en1994.6.4.ltb.girder-G1; en1994.7.3.1.deflection.girder-G1; en1994.7.4.crack.girder-G1
- en1996: en1996.3.1.material.wall-weak; en1996.6.3.flexure.wall-weak.uls-bad; en1996.6.1.3.concentrated.wall-weak.uls-bad.beam-A
- en1997: en1997.6.5.bearing.footing-F1.lc-bsP; en1997.6.6.settlement.footing-F1; en1997.7.6.3.tension.pile-P1; en1997.9.sliding.wall-W1
- en1998: en1998.1 p-delta stories s1–s4 on sys-x and sys-y of bldg-weak
- en1999: beam-fail flexure/buckling/deflection/stress/sls, weld-thin, bolt-short, fire-hot, sheet-fail, shell-fail
- iso16757: searchTags dup, selection.empty, bim index, clearance geom-valve-50, partNumber, scriptLimits

Do not weaken the gate. Remedies must make status not Fail.
