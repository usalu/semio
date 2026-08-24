@capability-writer-1-mutate
@no-oracle-writer-document-mutation-semantics
@comparison-ordered-json-v1
@mutations-writer-1-any
Feature: Apply every typed writer document mutation to its committed specification vectors
  `s.writer.writer` is a semio-NATIVE artifact: it is persisted as `.dsl.semio` text and
  `.pack.semio` binary through this subset's own codecs, and no third party reads or writes either.
  There is therefore no reference implementation to register as an oracle — recorded as the
  `writer-document-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, whose substitutes are the
  committed specification vectors and the inverse law. Because that decision is recorded, the runner
  dispatches NO oracle role for this case: every assertion below lives inside the subject handler,
  and a handler that merely ran the mutation and returned would report a pass having checked nothing.

  What distinguishes this subset from every other mutation vocabulary in the repository is how
  LITTLE state it has. `WriterSnapshot` carries exactly five persistent fields — `schema`, `id`,
  `languageId`, `uri` and `document` — with no id-keyed collections, no ordered lists, no
  relationships and no hierarchy, so `📓️derivation-rules.md` recipe §1 yields four document-level
  scalar kinds and nothing else. `schema` is the fixed `WRITER_DOCUMENT_SCHEMA` constant rather than
  authored content, which is exactly why the vocabulary is four kinds over five fields.

  The fifth field is the interesting one. `document` is not inline text: it is a composed
  `s.stdio.semio.document` CHILD HANDLE — a `(childId, target)` pair whose id is content-addressed —
  so `edit-text` is the only kind of the four that reaches outside the writer snapshot at all. Its
  diff oracle compares the payload against the CURRENT body first and, on equality, raises
  `mutation.no-op` and returns without re-minting the handle. The committed vector for `edit-text`
  is deliberately that guard case (`warns-that-the-brief-body-is-unchanged`): before and after are
  the same document by construction, because what it pins is that a save with no keystrokes behind
  it must NOT rewrite the document's content address. It is therefore named in the adapter's
  `GUARD_VECTORS` list and exempted from the observability law — and in exchange its `mutate` handler
  asserts something the other three cannot: the committed `🎯️outcome` vector's `applied` status with
  exactly one `mutation.no-op` warning, AND that `document.childId` is the identical handle
  afterwards. The other three kinds — `rename-writer`, `change-uri`, `change-language` — are pure
  scalar swaps over the same handcrafted `brief` document, and each must move the projection.

  ⚠️ Reading of the fixture census: this subset commits exactly one specification vector per kind,
  and three of the four move the document. That ratio is the best of the five artifacts covered in
  this wave and it is still thin — one vector per kind means a kind is exercised at exactly one
  point of its input space.

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
    When <id> is applied through apply_writer_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id              | vector                                                                |
      | rename-writer   | 🏷️rename-writer/🧪️tests/renames-the-document-to-mission-brief          |
      | change-uri      | 🔗change-uri/🧪️tests/republishes-the-brief-under-a-new-uri            |
      | change-language | 🌐change-language/🧪️tests/switches-the-brief-from-plaintext-to-markdown |
      | edit-text       | ✏️edit-text/🧪️tests/warns-that-the-brief-body-is-unchanged             |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    When <id> is applied and then its own computed inverse is applied through apply_writer_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id              | vector                                                                |
      | rename-writer   | 🏷️rename-writer/🧪️tests/renames-the-document-to-mission-brief          |
      | change-uri      | 🔗change-uri/🧪️tests/republishes-the-brief-under-a-new-uri            |
      | change-language | 🌐change-language/🧪️tests/switches-the-brief-from-plaintext-to-markdown |
      | edit-text       | ✏️edit-text/🧪️tests/warns-that-the-brief-body-is-unchanged             |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed jack document and print it back without losing or copying anything
    Given the real committed artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the artifact is parsed to a WriterSnapshot, printed back to `.writer` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
