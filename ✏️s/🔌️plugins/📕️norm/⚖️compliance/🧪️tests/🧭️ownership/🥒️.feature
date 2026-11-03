@capability-norm-check-ownership @oracle-python @comparison-closed-form
Feature: Every check is owned by an entity so the 3D scene and the checks window agree
  A check names its owner in `subject.entity_id`. After the evaluation the owner is, in this order: the subject when it is a row of an owning collection; the owning row whose id the check id embeds as a run of dot-separated segments; the first owning row that names the subject row in one of its fields; none for a documented document-level check (shown in the global badge of the scene). An independent Python oracle (`🐍️.py`) applies the same rule to the same cases.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - The embedded id wins by length, then by position: Given the owning rows m1 and m2 When the check id is `en.m1.m2` and its subject is empty Then its owner is m1
  - A referenced row is owned by its first user: Given a member naming the section s1 in `sectionId` When a check has the subject s1 Then its owner is that member And a section that nobody names keeps its subject
  - Document-level checks are declared: Given the id prefix `g.` is document-level When a check `g.annex` has the subject `p` Then its subject is cleared And a check `en.other` with the subject `p` keeps it
  - Emitters that file their own aggregates opt out: Given `Owners::AS_EMITTED` When the report is filed Then no subject changes
