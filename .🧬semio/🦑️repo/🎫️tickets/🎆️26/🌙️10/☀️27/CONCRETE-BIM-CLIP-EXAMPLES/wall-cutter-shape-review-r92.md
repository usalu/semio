# Wall Cutter Shape Review

Actual source575/d575c427 exclusive-parts inventory contains ten meaningful cases where the wall-cutter interface drops occupied-shape detail: six bored column sections (2 parts2/3,4 parts2/3,2-FIRST part0,4-FIRST part0), and four sloping cap components of STAIR-RETURN-L. The initial raw-key count103 included empty holes arrays and is not a defect count. No current exclusive beam/slab/window/door/curtain part with a nonempty discarded mesh, section, loft or hole was observed.

The six column bores represent intentional service airways. Their projected reservation alone does not establish missing wall material, because filling such airways would block the intended service void. The L cap requires intersection with actual raw wall material at the sloped upper region before a wall overcut can be claimed. No such positive architectural witness has yet been established. Shared-reference cutters have not been separately recaptured; this inventory is expressly the actual exclusive body subset, not a universal cutter correctness certificate.

Typed inventory: [wall-cutter-shape-review-r92.json](wall-cutter-shape-review-r92.json).


## L Cap Local Negative Control

The two nearby raw WC partition strips do not meet the actual concave L footprint: the north wall beginsY10.179118, while the horizontal L leg endsY10.150220372 (28.897628mm gap); the east wall endsX6.326765, while the vertical L leg startsX6.347144829 (20.379829mm gap). Their overall bounding boxes can overlap the overall L box, but the actual leg regions do not. This provides no cap-induced missing-wall witness. Roof stair enclosure lies above the cap, and cellar/ground-only walls below its sloped upper region. This is a bounded current-model negative control, not a promise that the projection interface supports arbitrary future nonvertical cutters.
