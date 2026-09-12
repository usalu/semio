# Go And Policy Audit

Date: 2026-09-12  
Scope: independent read-only review of the live Go-domain relocation and focused implementation-leaf enforcement changes.

## Result

No correctness defect was found in the audited Go relocation or in the four reviewed enforcement corrections. The live tree has 39 Go leaves in the CLI, MCP, and coordinator owners when fixtures are included; the 37 non-fixture leaves and the two fixture leaves all use the anonymous `🐹️.go` basename. This independently agrees with the execution report's 27 moved named implementation leaves plus existing anonymous entrypoints and tests.

`canonicalGoPlan` derives its input names from taxonomy, stops descent at nested `go.mod` boundaries, and follows only filesystem-backed `replace` dependencies. The live plans have these primary package selections:

| Owner | Primary packages | Own semantic sources | Own semantic tests |
| --- | --- | ---: | ---: |
| CLI | `.`, `./internal/command`, `./internal/eventstore` | 3 | 5 |
| MCP | `.` | 6 | 1 |
| Coordinator | `.` | 6 | 3 |

The MCP overlay also adds the three CLI semantic sources through its local replacement. It does not add the replaced module's packages to the MCP invocation, so Go keeps normal dependency selection. The coordinator overlay retains both platform files; Go's source build constraints select `!windows` and `windows` at compilation.

## Compiler Evidence

All temporary output, Go cache, module cache, and cross-built binary were isolated under `🗑️generated/terra-go-policy`.

| Check | Result |
| --- | --- |
| CLI root compiler oracle | `go test -overlay=... -run '^TestCanonicalFixtureGlob$' .` passed in 0.504 s. This compiles the domain projections and the direct imported Go packages. |
| Coordinator Windows compiler oracle | `GOOS=windows GOARCH=amd64 go build -overlay=... .` passed; emitted `coordinator-audit.exe` is a PE32+ x86-64 Windows executable. |
| Overlay necessity | The same coordinator package without an overlay reports `no Go files`, as expected after source leaves moved into domain directories. |
| Portable projection oracle | Existing registered `test go-input-projection` evidence in the execution report uses a materialized two-module fixture and native `go test -overlay`; it covers nested-module fences, private test-package selection, opaque fixtures, and transitive local replacement. |

The three build/test routers call `runCanonicalGoBuild` and `runCanonicalGoTests`. The CLI builds `./cmd/repo`; MCP builds `.`; the coordinator builds `.`. Their dev routes run the Nx prerequisite binary. The CLI entity-kind projection is at `⌨️cli/🏷️entity-kinds/🐹️.go`, has SHA-256 `29342aadd5d45848a4046968b8a591871e34d0ced2b1039588e1d2facefb7770`, is declared by `schema-entity-catalog`, and its consumer comment points to that location.

The tracked coordinator `server` binary has no live diff from `HEAD`; its current bytes hash to `537cb99adc1e8aa5752b8f31b06092ea744aad0176451dae7dbd2ec3c265b927`, equal to the `HEAD` content hash. Git cannot prove what an unrecorded pre-turn worktree binary contained, so this verifies only that the completed lane did not leave a binary diff or overwrite a currently represented change.

## Enforcement Review

The registered direct command below passed with six tests and 42 expectations:

```text
TMPDIR=<ticket-generated> bun …/📜️script.ts test kind-only-basename
6 pass, 0 fail, 42 expect() calls
```

The fixture has an independent `fast-glob` filesystem census, Ajv predicates, and an `@iarna/toml` Cargo oracle. The reviewed corrections are present and exercised:

- `build.rs` admission now requires an active adjacent Cargo manifest that resolves to Cargo's conventional build script. The naked package-shaped path is a `taxonomy/kind-only-basename` finding.
- `🤖️generated` is no longer a blanket ignored pattern. The fixture includes a named generated TypeScript leaf and expects the finding.
- `.d.mts` and `.d.cts` are registered TypeScript longest extension chains, with canonical and named cases.
- The launch seed contains 1,331 named configurations. Every one occurs once and byte-for-byte identically in `launch.json`; the generated launch catalog has 2,500 named configurations. The three focused commands occur exactly once in both files.

## Limits

This did not repeat the known expensive full CLI suite or Nx project-graph work. The focused compiler check used `GOWORK=off` with each module's committed local `replace` directives, whereas registered commands use the workspace `go.work`; this proves the relocation/compiler projection without claiming a fresh full workspace gate. Package-body purity, remaining non-Go source migrations, and the global focused census remain separate lanes.

## Evidence

- `🗑️generated/terra-go-policy/canonical-go-plans.json`
- `🗑️generated/terra-go-policy/cli-root-narrow-test.log`
- `🗑️generated/terra-go-policy/coordinator-windows-build.log`
- `🗑️generated/terra-go-policy/coordinator-windows-no-overlay.json`
- `🗑️generated/terra-go-policy/kind-only-basename.log`
- `🗑️generated/terra-go-policy/launch-seed-coverage.json`
