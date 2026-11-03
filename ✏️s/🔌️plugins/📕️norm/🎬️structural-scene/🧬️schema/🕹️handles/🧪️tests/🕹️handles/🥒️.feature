@capability-handle-schema
@oracle-serde-json-canonical
@comparison-canonical-json-v1
Feature: Handles bind one scalar drag to field paths
  Executed by `🔬️unit/🦀️.rs`; drag evaluation is the pure function `Handle::writes_at`.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - A drag clamps, snaps and writes every bound field: Given a length handle limited to 0.5..12 m with 0.05 m increments and the catalogue value 4.0 m When the drag parameter is 99 and later 4.01 Then the writes are 12.0 and later 4.0 for the length, and the placement follows the affine law base 1.0, k -0.5 rounded to 0.5
  - Quantisation <rule>: When <input> is quantised Then the result is <output> Examples (3 rows: round-step-0-5-4-26-4-5, choice-12-16-20-24-17-16, int-2-5-3)
  - Invalid handles are reported at paths: Given a handle without writes, with reversed limits, a zero axis and negative snap tolerance When it is validated Then "$.writes" is non-empty, "$.limits" ordered, "$.kind.axis" non-zero and "$.snap.tolerance" non-negative are violated
  - A token binding writes the enum token: Given a choice stepper bound to an enum field with the tokens "None", "Flat", "Tee" When the drag parameter is 1.4 and later 1.6 Then the written tokens are "Flat" and later "Tee", out-of-range parameters clamp to the first or last token, and duplicate or missing tokens are invalid
