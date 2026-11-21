# Typed generation controls

Flow variables declare a registry schema and a named input/output port. The Generate form projects unwired variables with the registered text, number, boolean, point and vector schemas. It uses text and number inputs, toggles and coordinate groups from the semantic UI framework. Point and vector labels use X, Y and Z. The single-step form renders the localized empty-state message when no controls are present. Flow does not declare an enum option list. The former one-option select was removed.

The command decoder carries the axis index to Playbook. Playbook validates the question and answer, replaces one axis in the active vector, and emits one whole-value mutation. A malformed type or out-of-range axis produces no value mutation. Generation3d preview ticks hand typed dictionaries to Flow under the original schema and port name. The regular seed snapshot comparison follows value changes into dependent nodes. A relay with an incoming wire retains its wired input. Snapshot overlays preserve the source widget’s name and port.

Generation2d and standalone Flow share the same producer. Their generation command and evaluation paths also receive typed values, so remaining consumers of the shared producer keep their form controls and preview aligned.

## Verification

- The portable Flow form corpus describes all five schema defaults, accepted edits, rejected edits, complete vector mutations and expected seed fields. The independent Bun oracle validates the JSON Schema with AJV and derives the resulting answers and outputs using JavaScript array and object operations.
- The scoped Rust harness compiled the producer, seed and indexed-edit code against cached native Flow and value libraries. It also ran the actual semantic vector control against a registered native UI arena and checked the numeric component, accessible axis label and commit binding. The same harness exercised exclusion of relays with incoming wires.
- Native source tests cover Flow seed port/schema and wired relay exclusion, Playbook defaults and two consecutive vector edits, and the Generation3d command’s whole-vector mutation.
- The full Generation3d application and browser suite were outside the available build storage. The isolated checks exercised the changed decisions and UI contract but did not rebuild the application.

Run the scoped checks with `repo tools exec --light bun -- typed-generation-form-oracle.mjs` and `repo tools exec --light python -- typed-generation-form-check.py`, using the scripts’ paths from the repository root.
