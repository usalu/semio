@capability-forms-1-mutate
@oracle-forms-python-independent
@comparison-ordered-json-v1
@mutations-forms-1-any
Feature: Apply every typed form document mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a second
  implementation of the `s.forms.form` document, its ten typed mutations and its `.dsl.semio` text
  carrier, written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`,
  `📝️definition/🔣️.json`, the carrier grammar `🚪️io/📸️snapshot/📝️text/📖️.grammar.semio`, the ten mutation
  leaf payload schemas, rules 1, 2 and 3 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` and the ten
  committed vectors. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library. XForms, JSON Schema forms and ODK each
  model a survey, but none of them models this document's composed child handles, reads its carrier or
  answers its mutation vocabulary, so the reference is written from this subset's own schemas.

  📌️ WHAT THIS CASE'S EVIDENCE ACTUALLY COVERS, stated plainly rather than left to be inferred from a
  green row. The document carries its survey INLINE (`definition.steps[].blocks[]`) beside `responses`
  and two composed child handles. Nine of the ten committed vectors pin a DIAGNOSTIC and leave the
  document byte-identical, and the reference DERIVES each diagnostic — status, code and path — from the
  before-document's own steps rather than reading it off the committed outcome. The `scene` cell of each
  row is that same step list, and the reference requires it to equal the committed before-document's
  `definition.steps`. Only `change-form-title` moves the document: it adds the `title` member, and its
  inverse removes it again. No committed vector yet exercises a create/delete/move/replace that
  SUCCEEDS; that is a real gap in the case's fixtures, stated here rather than passed over.

  📌️ A CROSS-CASE DIVERGENCE THE REFERENCE SURFACED, which neither case could see alone.
  `s.playbook.playbook` is the same shape with the same verbs, and the two subsets answer the same
  situation differently: a duplicate step id is a REJECTED `mutation.duplicate-id` here
  (`create-step`) and an APPLIED `mutation.no-op` there (`add-step`). It is reported rather than
  absorbed into a per-case table.

  🔁️ `identity-round-trip` reads the real committed `🗣️.dsl.semio` artifact through the reference's own
  grammar-driven carrier reader and answers the document it holds; the subject parses, prints and
  reparses it with this subset's codec, and the two documents are compared.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome vector shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json
    When <id> is applied through apply_form_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>", "scene": <scene>}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id                      | vector                                                                               | scene                                                                                                                                                              |
      | create-step             | 🌱create-step/🧪️rejects-a-duplicate-step-id                                     | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | delete-step             | 🗑️delete-step/🧪️rejects-deleting-a-step-the-scene-does-not-hold                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | reorder-step            | 🔀reorder-step/🧪️no-ops-when-the-step-already-sits-at-that-index                | [{"id":"step-basics","title":"Basics","blocks":[]},{"id":"step-photos","title":"Photos","blocks":[]},{"id":"step-summary","title":"Summary","blocks":[]}]          |
      | rename-step             | ✏️rename-step/🧪️no-ops-when-the-step-already-carries-that-title                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | change-step-description | 📝change-step-description/🧪️no-ops-when-clearing-already-absent  | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | create-block            | ➕create-block/🧪️rejects-a-block-for-a-step-that-does-not-exist                 | []                                                                                                                                                                 |
      | delete-block            | ➖delete-block/🧪️rejects-deleting-a-block-missing-from-an-existing-step         | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | move-block-to-step      | 📦move-block-to-step/🧪️no-ops-when-block-stays-index-own-step | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text"},{"id":"q-visit-date","label":"Visit date","kind":"text"}]}] |
      | replace-block           | 🔁replace-block/🧪️no-ops-when-the-replacement-block-is-identical                | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text","required":true}]}]                                          |
      | change-form-title       | 🏷️change-form-title/🧪️titles-an-untitled-survey                                | []                                                                                                                                                                 |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json
    When <id> is applied and then its own computed inverse is applied through apply_form_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>", "scene": <scene>}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id                      | vector                                                                               | scene                                                                                                                                                              |
      | create-step             | 🌱create-step/🧪️rejects-a-duplicate-step-id                                     | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | delete-step             | 🗑️delete-step/🧪️rejects-deleting-a-step-the-scene-does-not-hold                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | reorder-step            | 🔀reorder-step/🧪️no-ops-when-the-step-already-sits-at-that-index                | [{"id":"step-basics","title":"Basics","blocks":[]},{"id":"step-photos","title":"Photos","blocks":[]},{"id":"step-summary","title":"Summary","blocks":[]}]          |
      | rename-step             | ✏️rename-step/🧪️no-ops-when-the-step-already-carries-that-title                | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | change-step-description | 📝change-step-description/🧪️no-ops-when-clearing-already-absent  | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | create-block            | ➕create-block/🧪️rejects-a-block-for-a-step-that-does-not-exist                 | []                                                                                                                                                                 |
      | delete-block            | ➖delete-block/🧪️rejects-deleting-a-block-missing-from-an-existing-step         | [{"id":"step-basics","title":"Basics","blocks":[]}]                                                                                                                |
      | move-block-to-step      | 📦move-block-to-step/🧪️no-ops-when-block-stays-index-own-step | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text"},{"id":"q-visit-date","label":"Visit date","kind":"text"}]}] |
      | replace-block           | 🔁replace-block/🧪️no-ops-when-the-replacement-block-is-identical                | [{"id":"step-basics","title":"Basics","blocks":[{"id":"q-site-name","label":"Site name","kind":"text","required":true}]}]                                          |
      | change-form-title       | 🏷️change-form-title/🧪️titles-an-untitled-survey                                | []                                                                                                                                                                 |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://🎬️demo/🗣️.dsl.semio
    When the artifact is parsed to a FormsSnapshot, printed back to `.forms` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
