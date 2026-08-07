# 📋️ Inventory — Framework Repo Product Shape V2 (W8d)

## Owners migrated (0 `⚡️implementations` remain under `🦑️repo/**`)

| Owner | Langs | New package roots |
|---|---|---|
| `🔨️modules/⌨️cli` | 🦀️rust | `📦️packages/🦀️rust` |
| `🔨️modules/📚️library` | 🟦️typescript, 🐹️go | `📦️packages/🟦️typescript`, `📦️packages/🐹️go`; `go.mod` at owner root; `🔣️taxonomy.json` at owner root; `🔍️discovery/`, `🗂️workspaces/` components |
| `🔨️modules/💻️client/⌨️cli` | 🟦️typescript, 🐹️go | `📦️packages/🟦️typescript`, `📦️packages/🐹️go`; `go.mod` at owner root; `🐹️component.go` at owner root (`package main`) |
| `🔨️modules/💻️client/🔌️mcp` | 🐹️go | `📦️packages/🐹️go`; `go.mod` + `🐹️component.go` at owner root |
| `🔨️modules/💻️client/🧩️vscode` | 🟦️typescript | `📦️packages/🟦️typescript` (`🟦️extension.ts` entry) |
| `🔨️modules/💻️client/🪶️sqlite` | 🟦️typescript | `📦️packages/🟦️typescript`; `🛢️*.sql` at owner root |
| `🔨️modules/🖥️server/🎛️coordinator` | 🟦️typescript, 🐹️go | `📦️packages/🟦️typescript` (Next `app/` inside package); `go.mod` + `🐹️component.go` at owner root; `🐳️Dockerfile`, `🌐️Caddyfile`, `.env.example` at owner root |
| `🔨️modules/🖥️server/📚️library` | 🟦️typescript | `📦️packages/🟦️typescript`; `👷worker/🟦️component.ts` |

## Preserved public npm names

- `@semio-tech/repo-lib`, `@semio-tech/repo-client`, `@semio-tech/repo-coordinator`, `@semio-tech/repo-vscode`, `@semio-tech/repo-sqlite`

## Preserved Rust / nx names

- Crate `semio-framework-repo-cli`; nx `@semio-tech/repo-lib`, `repo-go-lib`, `@semio-tech/repo-cli-rs` (unchanged project names; `cwd`/`sourceRoot` updated in-package `📋️project.json` files).

## Verification (ticket logs)

- `🧪️verify-repo-go-lib.log` — `GOWORK=off go test ./...` in `📚️library` → OK
- `🧪️verify-repo-lib-tsc.log` — `tsc --noEmit` in `📚️library/📦️packages/🟦️typescript` → pre-existing `never.toString` errors only (2); playground import restored
- Full `bun ./📜️script.ts test` blocked until root `📜️script.ts` / `package.json` workspace paths are repointed (registrar)
