@capability-fem2d-1-mutate
@oracle-fem2d-python-independent
@comparison-ordered-json-v1
Feature: Read the real derived frame in both languages, and hold the committed carrier to its own law in Rust

  This case carries the whole-document identity law and the derivation provenance for
  `local://🏗️timber-portal-frame.snapshot.json` that used to live inside the artifact-level
  `mutate-fem2d-1` case alongside the `✳️mesh`, `✳️material`, `✳️boundary`, `✳️load` and
  `✳️analysis` mutation Examples. It has no vector and no mutation kind, so unlike its five mutation
  siblings it claims no `@mutations-` catalog — `✳️any` owns no mutation catalog of its own now
  that every collection has its smallest owner.

  The artifact is real. `local://🏗️timber-portal-frame.snapshot.json` was derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-fem2d-frame.py`
  from the artifact's own committed demo model (`asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`):
  a twelve-node timber-and-steel portal frame with a ridge at 7.6 m, nine beam elements, four
  supports, three real materials with their real moduli and densities, four sections with their real
  areas and second moments, a first-floor slab, a dead case carrying an area pressure, a live case
  carrying a nodal load and an area pressure, and an ULS combination at 1.35/1.5 — all carried across
  unchanged.

  What the derivation ADDS, and why. The committed model REFERENCES every entity it holds: every
  material by an element or the slab, every section by an element, every node by an element or a
  support, the slab by two area loads, and both cases by the ULS combination's terms. Deleting a
  referenced entity asks a question no committed document answers, so seven unreferenced spares are
  appended, each taken from a committed specification vector of one of this artifact's subsets and
  repointed only onto ids the model already holds. They are appended LAST, so the committed entities
  keep their indices and every spare is the TRAILING member of its collection — which matters,
  because no `create-` verb in this vocabulary carries an index, so the inverse of a delete is exact
  only for a trailing record. Every `✳️mesh`, `✳️material`, `✳️boundary`, `✳️load` and
  `✳️analysis` mutation case shares its OWN local copy of this same derived model.

  One deliberate exception: `delete-node` (owned by `✳️mesh`) addresses `n3`, which the spare
  support `s_spare` points at. That is not an oversight — the committed vector for this very kind is
  named `removes-node-n3-without-cascading-to-its-support`, so the non-cascade IS the specified
  behaviour and the row exercises it against a real model rather than a two-node sketch.

  Where the assertions live. `🐍️.py` in this directory reads the derived model and additionally
  requires, in role, that it really is the committed frame — a ridge above both eaves, every element
  bound to a material and a section the model holds, every support and every load bound to something
  that exists, and every combination term naming a real case. `🦀️.rs` additionally holds the
  `.dsl.semio` carrier to its own fixpoint law and crosses it against the binary pack codec, through
  the shared law module `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs`.

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real derived frame in both languages, and hold the committed carrier to its own law in Rust
    Given the real derived model local://🏗️timber-portal-frame.snapshot.json
    And the artifact's own committed carrier asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation reads the derived model, and the Rust additionally parses the committed carrier, prints it back and parses it again
    Then both languages read the same nine members, and the Rust reproduces the committed carrier byte for byte
