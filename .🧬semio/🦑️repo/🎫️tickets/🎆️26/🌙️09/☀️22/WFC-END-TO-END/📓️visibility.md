# Fill Thinking Is Visible

The solver no longer publishes a preview every few micro-steps. Each host step drains `WfcJob::advance_one` until fuel, the interactive deadline, or 48 newly visible cells, then publishes one tick. The tick carries the live singletons plus a retained trace (last 512 collapses and discards). Discarded cells stay painted from that trace after the search undoes them.

## Evidence

- bitmap fill tests, including a burst in one host step and an odd periodic checkerboard whose discarded cell changes the output layers: 6 passed
- bitmap oracle vector (includes a discarded event) and `the_python_oracle_vector_matches_the_rust_payload_shape`: passed
- bitmap output partial/pin/contradiction paint tests: passed
- `📜️bitmap-fill-oracle.py`: ok
- grid2d fill tests (pipes), including one tick with more than one collapse: 5 passed
- wfc2d fill tests (hex ring) plus partial preview paint: 6 passed
- grid3d fill tests (blocks): 5 passed
- wfc3d fill tests (tower stack): 5 passed

## Hot path

Singleton changes are recorded when a domain becomes or stops being a singleton. The fill drain reads that list. It no longer scans every cell after every solver unit.

`rooms-16` paints more than one new cell in a single host step. That test passed with the other bitmap fill tests in 0.20s.
