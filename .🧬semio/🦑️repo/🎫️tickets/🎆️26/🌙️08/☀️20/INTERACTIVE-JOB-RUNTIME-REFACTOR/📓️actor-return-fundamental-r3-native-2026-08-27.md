# Actor Fundamental Regression R3

Command used `--args='--lib'`, which currently selects the repository's default `fundamental` nextest profile. Actual exit0, **15 passed / 97 skipped / 0.073s**. This is not full112 coverage. Raw basename includes `full-r3` because that was the intended scope before reading the actual output; the observed scope supersedes that filename.

```text
Nextest run with nextest profile: fundamental
Starting 15 tests across 1 binary (97 tests skipped)
Summary [0.073s] 15 tests run: 15 passed, 97 skipped
NX Successfully ran target test for project @semio-tech/framework-actor-rs
```

The existing router accepts explicit `exhaustive`; the next run uses that level with `SEMIO_COVERAGE=0` to avoid creating a new release/coverage build graph. No production limit changes.
