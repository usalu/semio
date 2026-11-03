@capability-diagram2d-schema
@oracle-serde-json-canonical
@comparison-canonical-json-v1
Feature: Diagram2d schema is valid, bilingual and mirrored in every twin
  Executed by `🔬️unit/🦀️.rs`; the wire form is camelCase with adjacent `kind`/`data` variant tags and is regenerated from the Rust declarations by `wp02-gen-twins.py --check`.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - A complete diagram round-trips: Given a diagram with a polyline, polygon, band, scatter, limit, ray, section and 3D surface series, a bound marker and a table twin When it is validated, encoded and decoded Then validation reports no violation and the canonical JSON is unchanged
  - Invalid diagram "<defect>" is reported at a path: Given a valid diagram with the defect "<defect>" When it is validated Then the violation "<rule>" is reported at "<path>" Examples (6 rows: only-one-axis-axes-exactly-two, marker-id-equals-series-id-series-unique-ids, log-axis-with-zero-minimum-axes-0-domain-positive-on-log-axi, table-columns-of-unequal-size-tabletwin-columns-equal-length, empty-german-title-title-de-non-empty, ray-through-its-origin-series-5-through-distinct-from-origin)
