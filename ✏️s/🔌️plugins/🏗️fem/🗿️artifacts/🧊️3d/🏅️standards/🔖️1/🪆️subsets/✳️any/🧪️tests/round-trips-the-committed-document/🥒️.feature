@capability-fem3d-1-mutate
@oracle-fem3d-python-independent
@comparison-ordered-json-v1
Feature: Read the real derived frame in both languages, and hold the committed carrier to its own law in Rust

  This case carries the whole-document identity law and the derivation provenance for
  `local://🧊️steel-frame.snapshot.json` that used to live inside the artifact-level
  `mutate-fem3d-1` case alongside the `✳️mesh`, `✳️material`, `✳️boundary`, `✳️load` and
  `✳️analysis` mutation Examples. It has no vector and no mutation kind, so unlike its five
  mutation siblings it claims no `@mutations-` catalog — `✳️any` owns no mutation catalog of its
  own now that every collection has its smallest owner.

  The artifact is real. `local://🧊️steel-frame.snapshot.json` was derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-fem3d-frame.py`
  from the artifact's own committed demo model (`asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`):
  a sixteen-node, two-storey steel frame on an 8 × 10 m grid, four fully clamped column bases,
  sixteen HEA 200 members, two real materials with their real moduli, a first-floor concrete slab
  solid, four pinned slab-corner supports, a dead case with an area pressure, a live case with a
  nodal load and an area pressure, and an ULS combination at 1.35/1.5 — all carried across unchanged.

  What the derivation ADDS, and why. The committed model REFERENCES every entity it holds, so six
  unreferenced spares are appended, each taken from a committed specification vector of one of this
  artifact's subsets and repointed only onto ids the model already holds. They are appended LAST, so
  the committed entities keep their indices and every spare is the TRAILING member of its
  collection — which matters because no `create-` verb in this vocabulary carries an index, so the
  inverse of a delete is exact only for a trailing record. Every `✳️mesh`, `✳️material`,
  `✳️boundary`, `✳️load` and `✳️analysis` mutation case shares its OWN local copy of this same
  derived model.

  One deliberate exception: `delete-node` (owned by `✳️mesh`) addresses `n3`, which the spare
  support `s_spare` points at. The committed vector for this kind is named
  `removes-the-column-head-node-under-a-live-frame`, so the non-cascade IS the specified behaviour,
  and the row exercises it against a real frame.

  Where the assertions live. `🐍️.py` in this directory reads the derived model and additionally
  requires, in role, that it really is the committed frame: four ground corners with two storeys
  above, every element bound to a material and a section the model holds, every support and every
  load bound to something that exists, and every combination term naming a real case. `🦀️.rs`
  additionally holds the `.dsl.semio` carrier to its own fixpoint law and crosses it against the
  binary pack codec, through the shared law module
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs`.

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real derived frame in both languages, and hold the committed carrier to its own law in Rust
    Given the real derived model local://🧊️steel-frame.snapshot.json
    And the artifact's own committed carrier asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation reads the derived model, and the Rust additionally parses the committed carrier, prints it back and parses it again
    Then both languages read the same nine members, and the Rust reproduces the committed carrier byte for byte
