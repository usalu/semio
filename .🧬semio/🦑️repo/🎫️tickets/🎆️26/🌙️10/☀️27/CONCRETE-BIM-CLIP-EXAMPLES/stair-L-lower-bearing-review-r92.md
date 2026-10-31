# Independent Lower L-Post Bearing Review R92

Native diagnostic stair-L-lower-audit-r92.py used source snapshotb06d7d9869ada7d058ce5039f568b52c6633650917bb9b92db334613c5872906. The invalid proposed SL1 material reserve was omitted in memory only to inspect unreserved native slab geometry; no source file was edited. Output: 🗑️generated/stair-L-lower-audit-r92.json.

The L footprint area is0.0228m². Its horizontal leg overlaps the authored stair void by0.013200000015759222m². Its vertical tail lies OUTSIDE that void by0.00959999998424082m². The footprint is therefore not wholly inside the opening.

Actual occupied SL1 structural region R-PRIMARY-SL-1-50 beneath the tail spansZ2.91..3.14m. Ten native pieces intersect the L footprint in projected UNION area0.009599999984128018m², agreeing with the raw outside-void area within1.13×10⁻¹³m². SL1 has zero finish parts in this source.

The post starts at3.14m, so its lower interface is positive-area FACE contact with the slab top and zero positive VOLUME overlap. That is a supported bearing, not a floating post. A material reserve correctly rejects this zero-volume case because no slab material is displaced; no reserve, slab patch or base relocation is required. The independent upper sloped-guard bearing remains a separate0.011m² projected interface.

Author145 explicitly identifies L-post starter bars in original FLC L2(17)200, showing the stair opening trimmers during first-ceiling construction. This corroborates a first-slab/trimmer birth; it is an author attribution, not a newly calibrated hidden-joint measurement. The3.14m base uses the registered structural top and remains a coordinated construction inference. Source141 describes slab and guards as one self-supporting stair assembly resting on slabs and two auxiliary posts.

Earlier absence-of-contact suspicion came from treating positive-volume ownership intersection as a bearing test. The numeric face-contact proof above supersedes that suspicion while retaining the distinction between contact and displaced material. Native SDK product/body/cap validation alone did not establish this lower bearing; this additional interface check does.
