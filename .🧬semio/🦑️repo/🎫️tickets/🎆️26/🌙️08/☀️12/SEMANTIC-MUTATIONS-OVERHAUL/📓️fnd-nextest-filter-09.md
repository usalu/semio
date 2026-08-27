# FND-NEXTEST-FILTER-09 — Nextest Filter Routing

## Scope

Changed only the repo-library TypeScript Nextest command partitioner, its existing registered test region, and the neutral command-vector schema/fixture. No Rust source, mutation leaves, discovery policy, or `compose/**` path was read or changed.

## Contract

`cargo nextest run --help` (observed 2026-08-27, exit 0; no build or tests started) declares:

- pre-separator `[FILTERS]...` are test-name filters;
- post-separator `[FILTERS_AND_ARGS]...` are emulated libtest arguments.

`cargo nextest list --help` was additionally observed on 2026-08-27 with exit 0 and no build or tests started. Its identical argument grammar establishes the metadata-build boundary. The relevant output is retained in [🧪️nextest-list-help-2026-08-27.md](../🧪️nextest-list-help-2026-08-27.md). Its valued metadata options are retained by the partitioner: package/target selectors, feature and target paths, `--build-jobs`, `--cargo-profile`, `--cargo-message-format`, `--timings`, Cargo config options, `-Z`, listing/reuse/config values, and profile/color values. Required values fail closed; `--timings[=<FMTS>]` accepts a value only in its equals-joined form, so bare `--timings` cannot consume positional filters; joined `-p`, `-F`, `-j`, `-Z`, `-P`, and `-T` forms remain intact.

The partitioner therefore sends pre-separator non-option tokens to the metadata-backed `nextest run` command, retains Cargo options in the warm build command, and preserves all post-separator tokens for libtest.

Cargo target selectors that require values (`--test`, `--bin`, `--bench`, `--example`) remain build options with their values. The regression vector reproduces the supplied command ending in `language_neutral_forward_and_concrete_inverse`; that token now occurs only in `executionArgs`, rather than the metadata build argument list.

## Evidence

Red observations before implementation: the focused Nx/Bun registered test first exited 1 because the old partition result had no `libtestArgs` field required by the new neutral vectors. The `bare-timings-does-not-steal-positional-filters` regression then exited 1 because the old optional-value branch moved `first_filter` to `buildArgs`.

Green command, observed 2026-08-27, exit 0:

```text
SEMIO_TEST_BUDGET_MS=180000 bun nx run @semio-tech/repo-lib:test-quick --skip-nx-cache -- --timeout 60000 -t 'preserves language-neutral build and execution vectors with an independent Node parser'
```

Initial vector-only result: 1 pass, 0 fail, 29 assertions; 291 tests filtered. Final region command, observed 2026-08-27, exit 0:

```text
SEMIO_TEST_BUDGET_MS=180000 bun nx run @semio-tech/repo-lib:test-quick --skip-nx-cache -- --timeout 60000 -t 'nextest execution filters'
```

Final result: 2 pass, 0 fail, 68 assertions; 291 tests filtered. The vector test validates the JSON schema with Ajv and independently parses options using Node `util.parseArgs`. It compares the original ordered pre-separator positionals with the execution positionals and requires an empty build-positionals result. No Cargo build was run; the separate Nextest `--help` grammar checks and both `--timings` read-only parser invocations exited 0.

The registered repo-library lint command was also run. It is red for five cross-package TypeScript configuration errors outside this packet (no baseline lint run was made here): `ImportMeta.env` and `ImportMeta.glob` in the UI styling package, plus three `TS6059 rootDir` imports from the OS plugin store/generated playground modules. It reported no diagnostic in the Nextest partitioner, test region, or neutral vector/schema paths changed here.

## Source Release

This packet is ready for root review. The source release is the current shared working tree, with the exact changed paths enumerated in the handoff message.
