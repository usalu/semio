# W4 Structural Migration Snapshot

## Done

- All standard-level engine trees moved under subset any/engine (94 subset engines; 0 standard engines left).
- All artifact- and standard-level examples relocated under owning subset examples (or merged into existing subset demos).
- Phantom standard trees under stdio md and xml deleted.
- Glue path rewrites applied across 27+ plugin glue files; stdio paths resolved to real standard/subset locations.
- Leftover multi-standard artifact demos for gif/ifc/dwg merged into primary standards then deleted.

## Remaining (non-structural)

- Reference subset proof (W3): manifests, inferences, IO, TS mirrors, harness wiring for 7 archetypes.
- Facet totality: missing inferences/IO for many subsets; meta-only TS stubs.
- Glue shim block deletion after green builds.
- Policy promotion / seal.
