@capability-structural-scene-projection
@oracle-mock-artifact-full-projection
@comparison-ordered-json-v1
Feature: Project a document incrementally into a structural scene
  The incremental projector must equal a full projection of the same document, and diff/apply must be exact inverses.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Editing a member length changes only that member, its load and its dimension: Given a mock document with 8 members, one load each, and a full projection When the length of member "m3" is set to 4.5 Then the scene delta changes exactly the entities "m3,l3,dim-m3-length" And exactly 3 fragments are re-projected and 14 are reused
  - A derived layout propagates only through declared dependencies: Given a mock document whose members are laid out left to right When the length of member "m1" changes Then the scene delta changes member "m1" and every later member with its load and dimension And member "m0" is unchanged
  - Evaluation changes patch the entity, not its dependents: Given a full projection with utilisation 0.5 on member "m1" When the utilisation of member "m1" becomes 0.9 Then the scene delta contains exactly one replace patch for "m1"
  - The laws hold for random mutation sequences (<layout> layout): Given a mock document and a deterministic pseudo-random sequence of 80 mutations When every step is projected incrementally with its changed set Then the incremental scene equals a fresh full projection And applying the delta to the previous scene yields the current scene And the difference of a scene with itself is empty Examples (2 rows: stored, chained)
  - The text alternative is available in English and German: Given a projected scene with utilisation 1.25 on member "m2" When the scene description is generated Then it lists the element counts per kind and names the governing element in both languages And every entity has a non-empty accessible name in "en" and "de"
