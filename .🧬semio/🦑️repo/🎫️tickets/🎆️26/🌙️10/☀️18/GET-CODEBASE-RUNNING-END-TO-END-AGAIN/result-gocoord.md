# Result gocoord: Go split of P01, P02 and five repo modules

Scope: 9 Go files over 1,000 lines. Every declaration moved verbatim (doc comments included) by a go/ast based splitter (`🗑️generated/gocoord/tool`); only headers and `import` blocks differ. No behaviour change, no renames. All files now under 600 lines (largest 523).

## Files (module root `🧰️framework/🛍️products/🦑️repo/🔨️modules/`)

| Original | Before | After |
| --- | ---: | --- |
| `🖥️server/🎛️coordinator/📦️packages/🐹️go/🐹️.go` | 3,452 | 15 files, 145..523 lines: `🐹️.go` (config, mux, Main), `🛡️durability.go`, `🗄️eventstore.go`, `🩹️recovery.go`, `🔏️envelope.go`, `🧠️projection.go`, `📚️repository.go`, `📋️model.go`, `📡️eventbus.go`, `🧾️diff.go`, `🗂️scopes.go`, `🌐️server.go`, `🎫️ticket_handlers.go`, `🔎️diff_handlers.go`, `🪝️webhooks.go` |
| `🖥️server/🎛️coordinator/🧩️component/🐹️.go` | 1,593 | taxonomy components (projected files): `🧩️component` 379 (AGPL header kept), `📡️event` 126, `🧾️diff` 330, `🎯️scope` 144, `⚠️warnings` 63, `🚨️breaches` 37, `🎫️ticket` 224, `🪝️webhook` 393, each `🐹️.go` in an existing taxonomy member directory |
| `🖥️server/🎛️coordinator/🧪️tests/🗄️g3-event-store/🐹️.go` | 1,031 | test cases `🗄️g3-event-store` 276, `🛟️g3-recovery` 191, `⛔️g3-cancellation-bounds` 179, `🧠️g3-repository-projection` 371, `🧯️g3-hostile-source` 76 |
| `💻️client/🔌️mcp/🧪️tests/🤝️protocol-contract/🐹️.go` | 1,385 | test cases `🤝️protocol-contract` 437, `🧰️protocol-harness` 289, `🚦️protocol-lifecycle` 93, `🛑️protocol-cancellation` 378, `📏️protocol-bounds` 62, `📈️protocol-progress-replay` 223 (README test list updated) |
| `🧩️providers/…/🐹️.go` | 1,964 | `🐹️.go` 319 (license header kept), `🏃️process.go`, `🎄️github.go` 444, `🎯️github_goals.go`, `🕳️null.go`, `🔒️git.go` 316, `🎆️editors.go`, `🪝️hooks.go` 309 |
| `🏠️workspace/…/🐹️.go` | 1,851 | `🐹️.go` 218, `🖍️pattern.go` 403, `🙈️ignore.go` 240, `⚙️config.go`, `🧱️results.go`, `🎨️formatter.go`, `🌿️git.go`, `📄️files.go` 254, `🔤️text.go` 209 |
| `🗂️codebase/…/🐹️.go` | 1,765 | `🐹️.go` 228, `📦️bundles.go`, `🗃️files.go`, `🧩️sections.go` 326, `🎫️links.go`, `🌳️tree.go`, `🛠️technologies.go` 397, `🎯️test_definitions.go` |
| `📊️metrics/…/🐹️.go` | 1,706 | `🐹️.go` 133 (header verbatim), `🗂️classify.go`, `🧩️numstat.go`, `📊️report.go` 353, `🕰️time.go`, `🔌️gitsource.go`, `🧮️pipeline.go`, `📤️render.go`, `⏱️benchmark.go` |
| `🧑️contributors/…/🐹️.go` | 1,688 | `🐹️.go` 375, `🏁️checkpoints.go`, `🪅️sessions.go` 359, `✍️authors.go`, `🗄️store.go`, `🔭️session_source.go` 449, `🎁️interactions.go` |

File names avoid a leading `🧪️` (taxonomy `test-emoji-filename` legacy pattern).

## Verification (all foreground)
- Lossless: `🗑️generated/gocoord/verify.sh` compares sorted per-declaration sha256 (doc comment + body) of original vs all new files for every split: all LOSSLESS (coordinator 227 decls, component 87, g3 36, mcp 50, providers 194, workspace 152, codebase 64, metrics 105, contributors 107).
- `gofmt -l` clean on every touched dir.
- Package modules (GOWORK=repo go.work): `go build ./` + `go vet ./` pass for providers, workspace, codebase, metrics, contributors, coordinator (and `coordinator/📦️packages/🐹️go/📦️main`).
- `go test -count=1 -v ./` per-test result lists before vs after are identical: providers 27 PASS (24+3 Test funcs), workspace 27 PASS + 1 FAIL, codebase 4 PASS, contributors 4 PASS, metrics no test files (build/vet only; its `🧪️tests/*` adapters are run by the host harness, not in this module).
- Pre-existing failure, unrelated: `TestMcpStdioInitializeHandshake` (workspace) fails before and after because the spawned bun run cannot resolve `@semio-tech/schema-validator-ts` (missing node_modules).
- Legacy `repo/server` module (`🖥️server/🎛️coordinator`, not in go.work, projected sources): the whole-module overlay never compiled before (a `package adapter` case conflicts with `package main`). Verified with an overlay replicating `canonicalGoPlan` (`🗑️generated/gocoord/overlay.py`): source files `go vet` OK before/after; sources + g3-event-store + g3-filesystem + schema-contract test cases: vet OK, `go test` result list identical before/after (24 PASS, same 3 FAIL: `TestCoordinatorDecodesOwnerFixtures`, `TestG3LanguageNeutralGoldenEnvelope`, `TestG3OwnedSourceHasNoExternalOrLegacyStoreFallback`; overlay cwd lacks fixtures/sibling dirs). Test counts g3: 21 Test funcs before (5+6+3+6+1 after).
- `repo/mcp` module (`💻️client/🔌️mcp`) also never compiled (its `repo/client` replace target has no Go sources). Substitute: `go/types` check of sources + test files with stub importer (`splitter check`): error set identical before/after (only missing `client.*`); 22 Test funcs before and after (7+0+2+6+2+5).
- Text readers: repo-wide fixed-string search for the old file paths found only `🔌️mcp/README.md` (updated). `TestG3OwnedSourceHasNoExternalOrLegacyStoreFallback` reads only the untouched `🛡️durability`, `🗄️event-store`, `📚️repository` sources.

## Open
- No `bun repo` taxonomy/discovery gate was run (builds belong to the coordinator). New case/component directories reuse existing taxonomy member names (`📡️event`, `🧾️diff`, `🎯️scope`, `⚠️warnings`, `🚨️breaches`, `🎫️ticket`, `🪝️webhook`, `🧩️component`), so no taxonomy extension was needed.
- The `repo/server` and `repo/mcp` legacy modules remain non-compiling as before (not in scope).
- Scratch in `🗑️generated/gocoord/` (tool sources, plans, baselines, `orig/` copies) is left for the coordinator to delete.
