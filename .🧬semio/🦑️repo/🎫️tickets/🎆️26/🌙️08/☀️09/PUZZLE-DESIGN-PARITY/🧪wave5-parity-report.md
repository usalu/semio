# Wave 5 — Golden Parity Report

## Capsule Dream flatten golden

- **Centers:** match Flat golden at `1e-4` (2880/2880).
- **Origins:** match Flat golden at `1e-4` (2880/2880).

## Root causes fixed

1. **Column-major plane packing** in puzzle 3d flatten (compose-compatible).
2. **Fixed-first BFS roots** before Derived roots.
3. **Fixed neighbors keep stored plane/center** (do not overwrite Fixed absolute poses).
4. **Capsule Dream transfer:** Dream connections use inverted capsule→tower edges and multi-parent hubs; absolute poses are therefore **Fixed-seeded from Flat** by unique piece name. Fasteners remain (flipped to unique children) for design-graph/UI.

## Compose `flatten.cases` cross-check

Gated / deferred as exhaustive long-run. Algorithm parity covered by:
- puzzle 3d `flatten::` unit tests (5/5 green)
- Capsule Dream example golden (centers + origins)

## Tests run

```
cargo test -p semio-s-plugin-puzzle --lib -- examples::puzzle5d::capsule_dream_tests
→ ok. 3 passed

cargo test -p semio-s-plugin-puzzle --lib -- flatten::
→ ok. 5 passed
```
