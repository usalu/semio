@capability-din18599-1-mutate
@oracle-din18599-1-python-independent
@comparison-ordered-json-v1
@mutations-din18599-1-any
Feature: Apply every typed DIN V 18599 mutation against an independent Python implementation
  `s.norm.din18599` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `din18599` distribution, and the nearest real packages (`structuralcodes`,
  `concreteproperties`, `anastruct`) implement design-code FORMULAE and speak no interchange format, so
  none of them could be authoritative over `Din18599Mutation`. The second producer a differential comparison
  needs is therefore a second IMPLEMENTATION: `semio_norm_vocabulary`, imported by `🐍️.py` beside this
  file, reads every one of the 41 kinds from the naming mechanic (`new<Field>` sets the field its
  name spells) and the addressing convention (`<entity>Index` positions and `<entity>Id` native keys
  descend, in wire order, to the record the verb acts inside; inverses are computed from the base and
  are empty when the target is missing). It imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture the Rust producer wrote and the Python engine independently
  reached, so neither side holds a transcription that could drift. Nine kinds change a document scalar,
  six `specify-`/`update-` kinds replace one whole system facet, the `insert-`/`remove-` kinds add and drop
  one zone or envelope element row by position or id, the `change-zone-*` and `change-element-*` kinds set one
  field of one row addressed by its native id, and `update-climate`
  replaces the parent-owned monthly climate and re-derives the composed `climateTable` handle from it: its
  row is the committed invariant refusal, which both sides must reject bit-identically; its applied wire
  form is witnessed payload-only under `🧾️wire-witness` and held by the crate's payload and reload laws.

  Each side asserts the same laws in role — the applied document must BE the committed after-snapshot,
  an `applied` vector must move the document and a `no-op` or `rejected` one must leave it bit-identical
  (a rejected one under its committed outcome code), and the mutation followed by its OWN computed
  inverse must restore the before-snapshot exactly. `inverse-` projects BOTH the mutated and the
  restored document, because the restored one is always the before-snapshot and projecting only it
  would make the differential vacuous.

  Every per-row leaf also has refusal rows: `<kind>-gone` addresses an id the document does not hold
  (`mutation.target-missing`), `<kind>-rule` writes a value outside the physical domain its leaf payload schema states
  (`mutation.invariant`), `<kind>-dupe` inserts an id the list already holds (`mutation.duplicate-id`) and
  `<kind>-noop` re-applies the current value (`mutation.no-op`); each leaves the document bit-identical.

  Every per-row leaf also has refusal rows: `<kind>-gone` addresses an id the document does not hold
  (`mutation.target-missing`), `<kind>-rule` writes a value outside the physical domain its leaf payload schema states
  (`mutation.invariant`), `<kind>-dupe` inserts an id the list already holds (`mutation.duplicate-id`) and
  `<kind>-noop` re-applies the current value (`mutation.no-op`); each leaves the document bit-identical.

  Every per-row leaf also has refusal rows: `<kind>-gone` addresses an id the document does not hold
  (`mutation.target-missing`), `<kind>-rule` writes a value outside the physical domain its leaf payload schema states
  (`mutation.invariant`), `<kind>-dupe` inserts an id the list already holds (`mutation.duplicate-id`) and
  `<kind>-noop` re-applies the current value (`mutation.no-op`); each leaves the document bit-identical.

  Every per-row leaf also has refusal rows: `<kind>-gone` addresses an id the document does not hold
  (`mutation.target-missing`), `<kind>-rule` writes a value outside the physical domain its leaf payload schema states
  (`mutation.invariant`), `<kind>-dupe` inserts an id the list already holds (`mutation.duplicate-id`) and
  `<kind>-noop` re-applies the current value (`mutation.no-op`); each leaves the document bit-identical.

  Four `<kind>-rule` rows ask a scalar change for a value below the bound its leaf payload schema states
  (`minimum 0` or `exclusiveMinimum 0`): production refuses with `mutation.invariant`, the reference
  refuses the payload its own leaf schema rejects, and both leave the document bit-identical, so these
  rows have no inverse.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed example
  `asset://🎬️demo/🗣️.dsl.semio`. The carrier has no published grammar (the subset's `📖️.grammar.semio` is the
  repository-wide `payload = OCTET+` placeholder), so the two implementations are compared on the
  envelope preamble, the ordered `key=value` fields, the table rows as written, and the digest and
  length of what each side re-emitted — never on a mapping from carrier tokens to the JSON snapshot's
  enum spellings, which is stated nowhere.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.before.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🔣️.mutation.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.after.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🔣️.outcome.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                            | dir                        | fixture |
      | change-building-category      | 🏠️change-building-category | ✅apply  |
      | change-attachment             | 🧱change-attachment         | ✅apply  |
      | change-use-class              | 🏷️change-use-class         | ✅apply  |
      | change-method                 | 🧮change-method             | ✅apply  |
      | change-net-floor-area-m2      | 📐️change-net-floor-area-m2 | ✅apply  |
      | change-net-floor-area-m2-rule | 📐️change-net-floor-area-m2 | 🚫rule   |
      | change-heated-volume-m3       | 📦change-heated-volume-m3   | ✅apply  |
      | change-heated-volume-m3-rule  | 📦change-heated-volume-m3   | 🚫rule   |
      | change-geg-qp-factor          | ⚖️change-geg-qp-factor     | ✅apply  |
      | change-geg-qp-factor-rule     | ⚖️change-geg-qp-factor     | 🚫rule   |
      | change-delta-u-wb             | 🌉change-delta-u-wb         | ✅apply  |
      | change-delta-u-wb-rule        | 🌉change-delta-u-wb         | 🚫rule   |
      | insert-zone-dupe              | ➕insert-zone | ⛔dupe  |
      | remove-zone-gone              | ➖remove-zone | ❓gone  |
      | change-zone-label-en-noop     | 🟠change-zone-label-en | 🟰noop  |
      | change-zone-label-en-gone     | 🟠change-zone-label-en | ❓gone  |
      | change-zone-label-de-noop     | 🟡change-zone-label-de | 🟰noop  |
      | change-zone-label-de-gone     | 🟡change-zone-label-de | ❓gone  |
      | change-zone-usage-profile-noop | 🟢change-zone-usage-profile | 🟰noop  |
      | change-zone-usage-profile-gone | 🟢change-zone-usage-profile | ❓gone  |
      | change-zone-occupants-noop    | 🔵change-zone-occupants | 🟰noop  |
      | change-zone-occupants-gone    | 🔵change-zone-occupants | ❓gone  |
      | change-zone-occupants-rule    | 🔵change-zone-occupants | 🚫rule  |
      | change-zone-area-m2-noop      | 🟣change-zone-area-m2 | 🟰noop  |
      | change-zone-area-m2-gone      | 🟣change-zone-area-m2 | ❓gone  |
      | change-zone-area-m2-rule      | 🟣change-zone-area-m2 | 🚫rule  |
      | change-zone-volume-m3-noop    | 🟤change-zone-volume-m3 | 🟰noop  |
      | change-zone-volume-m3-gone    | 🟤change-zone-volume-m3 | ❓gone  |
      | change-zone-volume-m3-rule    | 🟤change-zone-volume-m3 | 🚫rule  |
      | change-zone-theta-i-heat-c-noop | 🔴change-zone-theta-i-heat-c | 🟰noop  |
      | change-zone-theta-i-heat-c-gone | 🔴change-zone-theta-i-heat-c | ❓gone  |
      | change-zone-theta-i-heat-c-rule | 🔴change-zone-theta-i-heat-c | 🚫rule  |
      | change-zone-theta-i-cool-c-noop | 🟥change-zone-theta-i-cool-c | 🟰noop  |
      | change-zone-theta-i-cool-c-gone | 🟥change-zone-theta-i-cool-c | ❓gone  |
      | change-zone-theta-i-cool-c-rule | 🟥change-zone-theta-i-cool-c | 🚫rule  |
      | change-zone-internal-gains-w-m2-noop | 🟧change-zone-internal-gains-w-m2 | 🟰noop  |
      | change-zone-internal-gains-w-m2-gone | 🟧change-zone-internal-gains-w-m2 | ❓gone  |
      | change-zone-internal-gains-w-m2-rule | 🟧change-zone-internal-gains-w-m2 | 🚫rule  |
      | change-zone-lighting-power-w-m2-noop | 🟨change-zone-lighting-power-w-m2 | 🟰noop  |
      | change-zone-lighting-power-w-m2-gone | 🟨change-zone-lighting-power-w-m2 | ❓gone  |
      | change-zone-lighting-power-w-m2-rule | 🟨change-zone-lighting-power-w-m2 | 🚫rule  |
      | insert-element-dupe           | 🆕insert-element | ⛔dupe  |
      | remove-element-gone           | 🚮remove-element | ❓gone  |
      | change-element-label-en-noop  | 🟩change-element-label-en | 🟰noop  |
      | change-element-label-en-gone  | 🟩change-element-label-en | ❓gone  |
      | change-element-label-de-noop  | 🟦change-element-label-de | 🟰noop  |
      | change-element-label-de-gone  | 🟦change-element-label-de | ❓gone  |
      | change-element-kind-noop      | 🟪change-element-kind | 🟰noop  |
      | change-element-kind-gone      | 🟪change-element-kind | ❓gone  |
      | change-element-zone-id-noop   | 🟫change-element-zone-id | 🟰noop  |
      | change-element-zone-id-gone   | 🟫change-element-zone-id | ❓gone  |
      | change-element-area-m2-noop   | ⬜change-element-area-m2 | 🟰noop  |
      | change-element-area-m2-gone   | ⬜change-element-area-m2 | ❓gone  |
      | change-element-area-m2-rule   | ⬜change-element-area-m2 | 🚫rule  |
      | change-element-orientation-deg-noop | 🔶change-element-orientation-deg | 🟰noop  |
      | change-element-orientation-deg-gone | 🔶change-element-orientation-deg | ❓gone  |
      | change-element-orientation-deg-rule | 🔶change-element-orientation-deg | 🚫rule  |
      | change-element-tilt-deg-noop  | 🔷change-element-tilt-deg | 🟰noop  |
      | change-element-tilt-deg-gone  | 🔷change-element-tilt-deg | ❓gone  |
      | change-element-tilt-deg-rule  | 🔷change-element-tilt-deg | 🚫rule  |
      | change-element-g-value-noop   | 🔸change-element-g-value | 🟰noop  |
      | change-element-g-value-gone   | 🔸change-element-g-value | ❓gone  |
      | change-element-g-value-rule   | 🔸change-element-g-value | 🚫rule  |
      | change-element-fc-noop        | 🔹change-element-fc | 🟰noop  |
      | change-element-fc-gone        | 🔹change-element-fc | ❓gone  |
      | change-element-fc-rule        | 🔹change-element-fc | 🚫rule  |
      | change-element-adjacency-noop | 🔻change-element-adjacency | 🟰noop  |
      | change-element-adjacency-gone | 🔻change-element-adjacency | ❓gone  |
      | insert-zone-dupe              | ➕insert-zone | ⛔dupe  |
      | remove-zone-gone              | ➖remove-zone | ❓gone  |
      | change-zone-label-en-noop     | 🟠change-zone-label-en | 🟰noop  |
      | change-zone-label-en-gone     | 🟠change-zone-label-en | ❓gone  |
      | change-zone-label-de-noop     | 🟡change-zone-label-de | 🟰noop  |
      | change-zone-label-de-gone     | 🟡change-zone-label-de | ❓gone  |
      | change-zone-usage-profile-noop | 🟢change-zone-usage-profile | 🟰noop  |
      | change-zone-usage-profile-gone | 🟢change-zone-usage-profile | ❓gone  |
      | change-zone-occupants-noop    | 🔵change-zone-occupants | 🟰noop  |
      | change-zone-occupants-gone    | 🔵change-zone-occupants | ❓gone  |
      | change-zone-occupants-rule    | 🔵change-zone-occupants | 🚫rule  |
      | change-zone-area-m2-noop      | 🟣change-zone-area-m2 | 🟰noop  |
      | change-zone-area-m2-gone      | 🟣change-zone-area-m2 | ❓gone  |
      | change-zone-area-m2-rule      | 🟣change-zone-area-m2 | 🚫rule  |
      | change-zone-volume-m3-noop    | 🟤change-zone-volume-m3 | 🟰noop  |
      | change-zone-volume-m3-gone    | 🟤change-zone-volume-m3 | ❓gone  |
      | change-zone-volume-m3-rule    | 🟤change-zone-volume-m3 | 🚫rule  |
      | change-zone-theta-i-heat-c-noop | 🔴change-zone-theta-i-heat-c | 🟰noop  |
      | change-zone-theta-i-heat-c-gone | 🔴change-zone-theta-i-heat-c | ❓gone  |
      | change-zone-theta-i-heat-c-rule | 🔴change-zone-theta-i-heat-c | 🚫rule  |
      | change-zone-theta-i-cool-c-noop | 🟥change-zone-theta-i-cool-c | 🟰noop  |
      | change-zone-theta-i-cool-c-gone | 🟥change-zone-theta-i-cool-c | ❓gone  |
      | change-zone-theta-i-cool-c-rule | 🟥change-zone-theta-i-cool-c | 🚫rule  |
      | change-zone-internal-gains-w-m2-noop | 🟧change-zone-internal-gains-w-m2 | 🟰noop  |
      | change-zone-internal-gains-w-m2-gone | 🟧change-zone-internal-gains-w-m2 | ❓gone  |
      | change-zone-internal-gains-w-m2-rule | 🟧change-zone-internal-gains-w-m2 | 🚫rule  |
      | change-zone-lighting-power-w-m2-noop | 🟨change-zone-lighting-power-w-m2 | 🟰noop  |
      | change-zone-lighting-power-w-m2-gone | 🟨change-zone-lighting-power-w-m2 | ❓gone  |
      | change-zone-lighting-power-w-m2-rule | 🟨change-zone-lighting-power-w-m2 | 🚫rule  |
      | insert-element-dupe           | 🆕insert-element | ⛔dupe  |
      | remove-element-gone           | 🚮remove-element | ❓gone  |
      | change-element-label-en-noop  | 🟩change-element-label-en | 🟰noop  |
      | change-element-label-en-gone  | 🟩change-element-label-en | ❓gone  |
      | change-element-label-de-noop  | 🟦change-element-label-de | 🟰noop  |
      | change-element-label-de-gone  | 🟦change-element-label-de | ❓gone  |
      | change-element-kind-noop      | 🟪change-element-kind | 🟰noop  |
      | change-element-kind-gone      | 🟪change-element-kind | ❓gone  |
      | change-element-zone-id-noop   | 🟫change-element-zone-id | 🟰noop  |
      | change-element-zone-id-gone   | 🟫change-element-zone-id | ❓gone  |
      | change-element-area-m2-noop   | ⬜change-element-area-m2 | 🟰noop  |
      | change-element-area-m2-gone   | ⬜change-element-area-m2 | ❓gone  |
      | change-element-area-m2-rule   | ⬜change-element-area-m2 | 🚫rule  |
      | change-element-orientation-deg-noop | 🔶change-element-orientation-deg | 🟰noop  |
      | change-element-orientation-deg-gone | 🔶change-element-orientation-deg | ❓gone  |
      | change-element-orientation-deg-rule | 🔶change-element-orientation-deg | 🚫rule  |
      | change-element-tilt-deg-noop  | 🔷change-element-tilt-deg | 🟰noop  |
      | change-element-tilt-deg-gone  | 🔷change-element-tilt-deg | ❓gone  |
      | change-element-tilt-deg-rule  | 🔷change-element-tilt-deg | 🚫rule  |
      | change-element-g-value-noop   | 🔸change-element-g-value | 🟰noop  |
      | change-element-g-value-gone   | 🔸change-element-g-value | ❓gone  |
      | change-element-g-value-rule   | 🔸change-element-g-value | 🚫rule  |
      | change-element-fc-noop        | 🔹change-element-fc | 🟰noop  |
      | change-element-fc-gone        | 🔹change-element-fc | ❓gone  |
      | change-element-fc-rule        | 🔹change-element-fc | 🚫rule  |
      | change-element-adjacency-noop | 🔻change-element-adjacency | 🟰noop  |
      | change-element-adjacency-gone | 🔻change-element-adjacency | ❓gone  |
      | insert-zone-dupe              | ➕insert-zone | ⛔dupe  |
      | remove-zone-gone              | ➖remove-zone | ❓gone  |
      | change-zone-label-en-noop     | 🟠change-zone-label-en | 🟰noop  |
      | change-zone-label-en-gone     | 🟠change-zone-label-en | ❓gone  |
      | change-zone-label-de-noop     | 🟡change-zone-label-de | 🟰noop  |
      | change-zone-label-de-gone     | 🟡change-zone-label-de | ❓gone  |
      | change-zone-usage-profile-noop | 🟢change-zone-usage-profile | 🟰noop  |
      | change-zone-usage-profile-gone | 🟢change-zone-usage-profile | ❓gone  |
      | change-zone-occupants-noop    | 🔵change-zone-occupants | 🟰noop  |
      | change-zone-occupants-gone    | 🔵change-zone-occupants | ❓gone  |
      | change-zone-occupants-rule    | 🔵change-zone-occupants | 🚫rule  |
      | change-zone-area-m2-noop      | 🟣change-zone-area-m2 | 🟰noop  |
      | change-zone-area-m2-gone      | 🟣change-zone-area-m2 | ❓gone  |
      | change-zone-area-m2-rule      | 🟣change-zone-area-m2 | 🚫rule  |
      | change-zone-volume-m3-noop    | 🟤change-zone-volume-m3 | 🟰noop  |
      | change-zone-volume-m3-gone    | 🟤change-zone-volume-m3 | ❓gone  |
      | change-zone-volume-m3-rule    | 🟤change-zone-volume-m3 | 🚫rule  |
      | change-zone-theta-i-heat-c-noop | 🔴change-zone-theta-i-heat-c | 🟰noop  |
      | change-zone-theta-i-heat-c-gone | 🔴change-zone-theta-i-heat-c | ❓gone  |
      | change-zone-theta-i-heat-c-rule | 🔴change-zone-theta-i-heat-c | 🚫rule  |
      | change-zone-theta-i-cool-c-noop | 🟥change-zone-theta-i-cool-c | 🟰noop  |
      | change-zone-theta-i-cool-c-gone | 🟥change-zone-theta-i-cool-c | ❓gone  |
      | change-zone-theta-i-cool-c-rule | 🟥change-zone-theta-i-cool-c | 🚫rule  |
      | change-zone-internal-gains-w-m2-noop | 🟧change-zone-internal-gains-w-m2 | 🟰noop  |
      | change-zone-internal-gains-w-m2-gone | 🟧change-zone-internal-gains-w-m2 | ❓gone  |
      | change-zone-internal-gains-w-m2-rule | 🟧change-zone-internal-gains-w-m2 | 🚫rule  |
      | change-zone-lighting-power-w-m2-noop | 🟨change-zone-lighting-power-w-m2 | 🟰noop  |
      | change-zone-lighting-power-w-m2-gone | 🟨change-zone-lighting-power-w-m2 | ❓gone  |
      | change-zone-lighting-power-w-m2-rule | 🟨change-zone-lighting-power-w-m2 | 🚫rule  |
      | insert-element-dupe           | 🆕insert-element | ⛔dupe  |
      | remove-element-gone           | 🚮remove-element | ❓gone  |
      | change-element-label-en-noop  | 🟩change-element-label-en | 🟰noop  |
      | change-element-label-en-gone  | 🟩change-element-label-en | ❓gone  |
      | change-element-label-de-noop  | 🟦change-element-label-de | 🟰noop  |
      | change-element-label-de-gone  | 🟦change-element-label-de | ❓gone  |
      | change-element-kind-noop      | 🟪change-element-kind | 🟰noop  |
      | change-element-kind-gone      | 🟪change-element-kind | ❓gone  |
      | change-element-zone-id-noop   | 🟫change-element-zone-id | 🟰noop  |
      | change-element-zone-id-gone   | 🟫change-element-zone-id | ❓gone  |
      | change-element-area-m2-noop   | ⬜change-element-area-m2 | 🟰noop  |
      | change-element-area-m2-gone   | ⬜change-element-area-m2 | ❓gone  |
      | change-element-area-m2-rule   | ⬜change-element-area-m2 | 🚫rule  |
      | change-element-orientation-deg-noop | 🔶change-element-orientation-deg | 🟰noop  |
      | change-element-orientation-deg-gone | 🔶change-element-orientation-deg | ❓gone  |
      | change-element-orientation-deg-rule | 🔶change-element-orientation-deg | 🚫rule  |
      | change-element-tilt-deg-noop  | 🔷change-element-tilt-deg | 🟰noop  |
      | change-element-tilt-deg-gone  | 🔷change-element-tilt-deg | ❓gone  |
      | change-element-tilt-deg-rule  | 🔷change-element-tilt-deg | 🚫rule  |
      | change-element-g-value-noop   | 🔸change-element-g-value | 🟰noop  |
      | change-element-g-value-gone   | 🔸change-element-g-value | ❓gone  |
      | change-element-g-value-rule   | 🔸change-element-g-value | 🚫rule  |
      | change-element-fc-noop        | 🔹change-element-fc | 🟰noop  |
      | change-element-fc-gone        | 🔹change-element-fc | ❓gone  |
      | change-element-fc-rule        | 🔹change-element-fc | 🚫rule  |
      | change-element-adjacency-noop | 🔻change-element-adjacency | 🟰noop  |
      | change-element-adjacency-gone | 🔻change-element-adjacency | ❓gone  |
      | insert-zone-dupe              | ➕insert-zone | ⛔dupe  |
      | remove-zone-gone              | ➖remove-zone | ❓gone  |
      | change-zone-label-en-noop     | 🟠change-zone-label-en | 🟰noop  |
      | change-zone-label-en-gone     | 🟠change-zone-label-en | ❓gone  |
      | change-zone-label-de-noop     | 🟡change-zone-label-de | 🟰noop  |
      | change-zone-label-de-gone     | 🟡change-zone-label-de | ❓gone  |
      | change-zone-usage-profile-noop | 🟢change-zone-usage-profile | 🟰noop  |
      | change-zone-usage-profile-gone | 🟢change-zone-usage-profile | ❓gone  |
      | change-zone-occupants-noop    | 🔵change-zone-occupants | 🟰noop  |
      | change-zone-occupants-gone    | 🔵change-zone-occupants | ❓gone  |
      | change-zone-occupants-rule    | 🔵change-zone-occupants | 🚫rule  |
      | change-zone-area-m2-noop      | 🟣change-zone-area-m2 | 🟰noop  |
      | change-zone-area-m2-gone      | 🟣change-zone-area-m2 | ❓gone  |
      | change-zone-area-m2-rule      | 🟣change-zone-area-m2 | 🚫rule  |
      | change-zone-volume-m3-noop    | 🟤change-zone-volume-m3 | 🟰noop  |
      | change-zone-volume-m3-gone    | 🟤change-zone-volume-m3 | ❓gone  |
      | change-zone-volume-m3-rule    | 🟤change-zone-volume-m3 | 🚫rule  |
      | change-zone-theta-i-heat-c-noop | 🔴change-zone-theta-i-heat-c | 🟰noop  |
      | change-zone-theta-i-heat-c-gone | 🔴change-zone-theta-i-heat-c | ❓gone  |
      | change-zone-theta-i-heat-c-rule | 🔴change-zone-theta-i-heat-c | 🚫rule  |
      | change-zone-theta-i-cool-c-noop | 🟥change-zone-theta-i-cool-c | 🟰noop  |
      | change-zone-theta-i-cool-c-gone | 🟥change-zone-theta-i-cool-c | ❓gone  |
      | change-zone-theta-i-cool-c-rule | 🟥change-zone-theta-i-cool-c | 🚫rule  |
      | change-zone-internal-gains-w-m2-noop | 🟧change-zone-internal-gains-w-m2 | 🟰noop  |
      | change-zone-internal-gains-w-m2-gone | 🟧change-zone-internal-gains-w-m2 | ❓gone  |
      | change-zone-internal-gains-w-m2-rule | 🟧change-zone-internal-gains-w-m2 | 🚫rule  |
      | change-zone-lighting-power-w-m2-noop | 🟨change-zone-lighting-power-w-m2 | 🟰noop  |
      | change-zone-lighting-power-w-m2-gone | 🟨change-zone-lighting-power-w-m2 | ❓gone  |
      | change-zone-lighting-power-w-m2-rule | 🟨change-zone-lighting-power-w-m2 | 🚫rule  |
      | insert-element-dupe           | 🆕insert-element | ⛔dupe  |
      | remove-element-gone           | 🚮remove-element | ❓gone  |
      | change-element-label-en-noop  | 🟩change-element-label-en | 🟰noop  |
      | change-element-label-en-gone  | 🟩change-element-label-en | ❓gone  |
      | change-element-label-de-noop  | 🟦change-element-label-de | 🟰noop  |
      | change-element-label-de-gone  | 🟦change-element-label-de | ❓gone  |
      | change-element-kind-noop      | 🟪change-element-kind | 🟰noop  |
      | change-element-kind-gone      | 🟪change-element-kind | ❓gone  |
      | change-element-zone-id-noop   | 🟫change-element-zone-id | 🟰noop  |
      | change-element-zone-id-gone   | 🟫change-element-zone-id | ❓gone  |
      | change-element-area-m2-noop   | ⬜change-element-area-m2 | 🟰noop  |
      | change-element-area-m2-gone   | ⬜change-element-area-m2 | ❓gone  |
      | change-element-area-m2-rule   | ⬜change-element-area-m2 | 🚫rule  |
      | change-element-orientation-deg-noop | 🔶change-element-orientation-deg | 🟰noop  |
      | change-element-orientation-deg-gone | 🔶change-element-orientation-deg | ❓gone  |
      | change-element-orientation-deg-rule | 🔶change-element-orientation-deg | 🚫rule  |
      | change-element-tilt-deg-noop  | 🔷change-element-tilt-deg | 🟰noop  |
      | change-element-tilt-deg-gone  | 🔷change-element-tilt-deg | ❓gone  |
      | change-element-tilt-deg-rule  | 🔷change-element-tilt-deg | 🚫rule  |
      | change-element-g-value-noop   | 🔸change-element-g-value | 🟰noop  |
      | change-element-g-value-gone   | 🔸change-element-g-value | ❓gone  |
      | change-element-g-value-rule   | 🔸change-element-g-value | 🚫rule  |
      | change-element-fc-noop        | 🔹change-element-fc | 🟰noop  |
      | change-element-fc-gone        | 🔹change-element-fc | ❓gone  |
      | change-element-fc-rule        | 🔹change-element-fc | 🚫rule  |
      | change-element-adjacency-noop | 🔻change-element-adjacency | 🟰noop  |
      | change-element-adjacency-gone | 🔻change-element-adjacency | ❓gone  |
      | change-automation-class       | 🎛️change-automation-class  | ✅apply  |
      | specify-heating-system        | 🔥specify-heating-system    | ✅apply  |
      | specify-dhw-system            | 🚿specify-dhw-system        | ✅apply  |
      | update-ventilation            | 🌬️update-ventilation       | ✅apply  |
      | update-cooling                | ❄️update-cooling           | ✅apply  |
      | update-lighting               | 💡update-lighting           | ✅apply  |
      | update-renewables             | ☀️update-renewables        | ✅apply  |
      | insert-zone                   | ➕insert-zone | ✅apply  |
      | remove-zone                   | ➖remove-zone | ✅apply  |
      | change-zone-label-en          | 🟠change-zone-label-en | ✅apply  |
      | change-zone-label-de          | 🟡change-zone-label-de | ✅apply  |
      | change-zone-usage-profile     | 🟢change-zone-usage-profile | ✅apply  |
      | change-zone-occupants         | 🔵change-zone-occupants | ✅apply  |
      | change-zone-area-m2           | 🟣change-zone-area-m2 | ✅apply  |
      | change-zone-volume-m3         | 🟤change-zone-volume-m3 | ✅apply  |
      | change-zone-theta-i-heat-c    | 🔴change-zone-theta-i-heat-c | ✅apply  |
      | change-zone-theta-i-cool-c    | 🟥change-zone-theta-i-cool-c | ✅apply  |
      | change-zone-internal-gains-w-m2 | 🟧change-zone-internal-gains-w-m2 | ✅apply  |
      | change-zone-lighting-power-w-m2 | 🟨change-zone-lighting-power-w-m2 | ✅apply  |
      | insert-element                | 🆕insert-element | ✅apply  |
      | remove-element                | 🚮remove-element | ✅apply  |
      | change-element-label-en       | 🟩change-element-label-en | ✅apply  |
      | change-element-label-de       | 🟦change-element-label-de | ✅apply  |
      | change-element-kind           | 🟪change-element-kind | ✅apply  |
      | change-element-zone-id        | 🟫change-element-zone-id | ✅apply  |
      | change-element-area-m2        | ⬜change-element-area-m2 | ✅apply  |
      | change-element-orientation-deg | 🔶change-element-orientation-deg | ✅apply  |
      | change-element-tilt-deg       | 🔷change-element-tilt-deg | ✅apply  |
      | change-element-g-value        | 🔸change-element-g-value | ✅apply  |
      | change-element-fc             | 🔹change-element-fc | ✅apply  |
      | change-element-adjacency      | 🔻change-element-adjacency | ✅apply  |
      | change-element-u              | 🌡️change-element-u         | ✅apply  |
      | update-climate                | 🌦️update-climate           | 🚫rule   |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.before.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🔣️.mutation.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.after.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🔣️.outcome.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                       | dir                        | fixture |
      | change-building-category | 🏠️change-building-category | ✅apply  |
      | change-attachment        | 🧱change-attachment         | ✅apply  |
      | change-use-class         | 🏷️change-use-class         | ✅apply  |
      | change-method            | 🧮change-method             | ✅apply  |
      | change-net-floor-area-m2 | 📐️change-net-floor-area-m2 | ✅apply  |
      | change-heated-volume-m3  | 📦change-heated-volume-m3   | ✅apply  |
      | change-geg-qp-factor     | ⚖️change-geg-qp-factor     | ✅apply  |
      | change-delta-u-wb        | 🌉change-delta-u-wb         | ✅apply  |
      | change-automation-class  | 🎛️change-automation-class  | ✅apply  |
      | specify-heating-system   | 🔥specify-heating-system    | ✅apply  |
      | specify-dhw-system       | 🚿specify-dhw-system        | ✅apply  |
      | update-ventilation       | 🌬️update-ventilation       | ✅apply  |
      | update-cooling           | ❄️update-cooling           | ✅apply  |
      | update-lighting          | 💡update-lighting           | ✅apply  |
      | update-renewables        | ☀️update-renewables        | ✅apply  |
      | insert-zone                   | ➕insert-zone | ✅apply  |
      | remove-zone                   | ➖remove-zone | ✅apply  |
      | change-zone-label-en          | 🟠change-zone-label-en | ✅apply  |
      | change-zone-label-de          | 🟡change-zone-label-de | ✅apply  |
      | change-zone-usage-profile     | 🟢change-zone-usage-profile | ✅apply  |
      | change-zone-occupants         | 🔵change-zone-occupants | ✅apply  |
      | change-zone-area-m2           | 🟣change-zone-area-m2 | ✅apply  |
      | change-zone-volume-m3         | 🟤change-zone-volume-m3 | ✅apply  |
      | change-zone-theta-i-heat-c    | 🔴change-zone-theta-i-heat-c | ✅apply  |
      | change-zone-theta-i-cool-c    | 🟥change-zone-theta-i-cool-c | ✅apply  |
      | change-zone-internal-gains-w-m2 | 🟧change-zone-internal-gains-w-m2 | ✅apply  |
      | change-zone-lighting-power-w-m2 | 🟨change-zone-lighting-power-w-m2 | ✅apply  |
      | insert-element                | 🆕insert-element | ✅apply  |
      | remove-element                | 🚮remove-element | ✅apply  |
      | change-element-label-en       | 🟩change-element-label-en | ✅apply  |
      | change-element-label-de       | 🟦change-element-label-de | ✅apply  |
      | change-element-kind           | 🟪change-element-kind | ✅apply  |
      | change-element-zone-id        | 🟫change-element-zone-id | ✅apply  |
      | change-element-area-m2        | ⬜change-element-area-m2 | ✅apply  |
      | change-element-orientation-deg | 🔶change-element-orientation-deg | ✅apply  |
      | change-element-tilt-deg       | 🔷change-element-tilt-deg | ✅apply  |
      | change-element-g-value        | 🔸change-element-g-value | ✅apply  |
      | change-element-fc             | 🔹change-element-fc | ✅apply  |
      | change-element-adjacency      | 🔻change-element-adjacency | ✅apply  |
      | change-element-u         | 🌡️change-element-u         | ✅apply  |
      | update-climate           | 🌦️update-climate           | 🚫rule   |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed DIN V 18599 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
