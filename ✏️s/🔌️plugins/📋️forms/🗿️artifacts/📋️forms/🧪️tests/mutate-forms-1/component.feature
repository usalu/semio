@capability-forms-1-mutate
@no-oracle-forms-document-mutation-semantics
@comparison-ordered-json-v1
@mutations-forms-1-any
Feature: Apply every typed forms document mutation to its committed specification vectors
  `s.forms.form` is a semio-NATIVE artifact: it is persisted as `.dsl.semio` text and `.pack.semio`
  binary through this subset's own codecs, and no third party reads or writes either. There is
  therefore no reference implementation to register as an oracle — recorded as the
  `forms-document-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, whose substitutes are the
  committed specification vectors and the inverse law. Because that decision is recorded, the runner
  dispatches NO oracle role for this case: every assertion below lives inside the subject handler,
  and a handler that merely ran the mutation and returned would report a pass having checked nothing.

  What distinguishes this subset is that its document is almost entirely NOT in its snapshot.
  `FormsSnapshot` persists a title and two composed CHILD handles — an `s.stdio.semio.value`
  `structure` and an `s.stdio.semio.table` `results` — and the whole steps-and-questions tree the
  ten kinds edit lives behind the first of them, in a session-side working scene. Both handles are
  content-addressed: a successful mutation re-mints them from a `DefaultHasher` digest of the child
  content.

  ⚠️ That has a consequence this feature states rather than hides. NINE of the ten committed vectors
  pin a REJECTION or a NO-OP branch, not an effect, and the leaves say why: hand-authoring an
  effect vector's `➡️after` would mean forging a value out of `std`'s deliberately unspecified
  default hasher (`🌱create-step/🧪️tests/rejects-a-duplicate-step-id/🦀️component.rs`). So nine of
  the ten kinds are named in the adapter's `GUARD_VECTORS` list and exempted from the observability
  law — the largest exemption in this wave, and a real limit on what this case proves. In exchange
  each `mutate-<kind>` scenario asserts something a plain effect vector never does: the committed
  `🎯️outcome`'s declared status AND its declared diagnostic code, so `create-step`'s Fatal
  `mutation.duplicate-id` is distinguished from the Error-level `mutation.target-missing` that the
  delete, rename, reorder, move and replace verbs raise, and from the Warning-level `mutation.no-op`
  that a genuinely idempotent request raises. `change-form-title` is the one kind whose vector is an
  effect vector, and it carries the observability law unexempted.

  📌️ The `scene` column is the other half of each before-state. The persisted `⬅️before` vector
  carries only the child handle, so the rows behind it have to be seeded before the mutation runs —
  without that every addressed id is absent and all ten kinds collapse onto one
  `mutation.target-missing` path, which would look green while testing a single branch ten times.
  Each cell is transcribed from that leaf's OWN `🧪️tests/<fixture>/🦀️component.rs::before()`, which
  is where this subset keeps that half today. Committing the scene as a fixture file beside the
  snapshot is the fix; until it lands, an external host cannot reconstruct it and this column is the
  honest way to carry it.

  Every scenario reads the committed vectors where the domain already keeps them, through
  `asset://`, and never writes to them.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️component.json
    And the committed outcome vector asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️component.json
    When <id> is applied through apply_form_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>", "scene": <scene>}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id                      | vector                                                                               | scene                                                                                                                                                              |
      | create-step             | 🌱create-step/🧪️tests/rejects-a-duplicate-step-id                                     | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | delete-step             | 🗑️delete-step/🧪️tests/rejects-deleting-a-step-the-scene-does-not-hold                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | reorder-step            | 🔀reorder-step/🧪️tests/no-ops-when-the-step-already-sits-at-that-index                | [{"id":"step-basics","title":"Basics","blocks":[]},{"id":"step-photos","title":"Photos","blocks":[]},{"id":"step-summary","title":"Summary","blocks":[]}]          |
      | rename-step             | ✏️rename-step/🧪️tests/no-ops-when-the-step-already-carries-that-title                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | change-step-description | 📝change-step-description/🧪️tests/no-ops-when-clearing-an-already-absent-description  | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | create-block            | ➕create-block/🧪️tests/rejects-a-block-for-a-step-that-does-not-exist                 | []                                                                                                                                                                 |
      | delete-block            | ➖delete-block/🧪️tests/rejects-deleting-a-block-missing-from-an-existing-step         | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | move-block-to-step      | 📦move-block-to-step/🧪️tests/no-ops-when-the-block-stays-at-its-index-in-its-own-step | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text"},{"id":"q-visit-date","label":"Visit date","kind":"text"}]}] |
      | replace-block           | 🔁replace-block/🧪️tests/no-ops-when-the-replacement-block-is-identical                | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text","required":true}]}]                                          |
      | change-form-title       | 🏷️change-form-title/🧪️tests/titles-an-untitled-survey                                | []                                                                                                                                                                 |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    When <id> is applied and then its own computed inverse is applied through apply_form_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>", "scene": <scene>}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id                      | vector                                                                               | scene                                                                                                                                                              |
      | create-step             | 🌱create-step/🧪️tests/rejects-a-duplicate-step-id                                     | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | delete-step             | 🗑️delete-step/🧪️tests/rejects-deleting-a-step-the-scene-does-not-hold                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | reorder-step            | 🔀reorder-step/🧪️tests/no-ops-when-the-step-already-sits-at-that-index                | [{"id":"step-basics","title":"Basics","blocks":[]},{"id":"step-photos","title":"Photos","blocks":[]},{"id":"step-summary","title":"Summary","blocks":[]}]          |
      | rename-step             | ✏️rename-step/🧪️tests/no-ops-when-the-step-already-carries-that-title                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | change-step-description | 📝change-step-description/🧪️tests/no-ops-when-clearing-an-already-absent-description  | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | create-block            | ➕create-block/🧪️tests/rejects-a-block-for-a-step-that-does-not-exist                 | []                                                                                                                                                                 |
      | delete-block            | ➖delete-block/🧪️tests/rejects-deleting-a-block-missing-from-an-existing-step         | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | move-block-to-step      | 📦move-block-to-step/🧪️tests/no-ops-when-the-block-stays-at-its-index-in-its-own-step | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text"},{"id":"q-visit-date","label":"Visit date","kind":"text"}]}] |
      | replace-block           | 🔁replace-block/🧪️tests/no-ops-when-the-replacement-block-is-identical                | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text","required":true}]}]                                          |
      | change-form-title       | 🏷️change-form-title/🧪️tests/titles-an-untitled-survey                                | []                                                                                                                                                                 |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the artifact is parsed to a FormsSnapshot, printed back to `.forms` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
