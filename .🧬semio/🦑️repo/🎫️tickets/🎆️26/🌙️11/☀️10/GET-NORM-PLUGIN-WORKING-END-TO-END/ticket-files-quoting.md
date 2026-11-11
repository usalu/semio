# Ticket Files Quoting

## Root cause

`git diff --name-status` and `git diff --cached --name-only` ran without `-z`, and the
unified-diff header parser used `strings.Fields`. Git C-quotes any path with non-ASCII bytes
(`"b/\360\237..."`), so the quoted header and the quoted name-status field reached the ticket
close files list verbatim. The stored entries then had `\` and `/` mangled and the trailing
extension dropped.

## Fix

- `🧩️providers/.../🔒️git/🐹️.go`: `StagedFiles` uses `--name-only -z`; `GetGitDiffStatus` uses
  `--name-status -z` and walks NUL fields; `GetGitDiffLines` runs with `core.quotepath=off` and
  parses `diff --git` / `+++` headers via `gitDiffNewPath` / `gitPathField` / `gitUnquotePath`
  (C-unquote for `"`, `\`, control bytes and octal escapes).
- Same fixes mirrored in the client copies: `🔒️git-devcontainer-providers`, `🧮️scope-and-ticket-files`
  (diff lines), `🆔️artifact-id-builders` (diff status).
- `🧩️providers/🌿️version-control-and-sandbox/🦀️.rs`: `staged_files` uses `-z`; unit test updated.

## Test

- New language-agnostic scenario `staging-lists-an-emoji-path-verbatim` in
  `🧪️tests/🌿️git-version-control/🥒️.feature`, implemented in the Go, Rust and TypeScript (oracle)
  adapters.
- Coverage gap: the ticket-close diff-status and diff-lines paths have no emoji-path scenario yet.

## Verification

- `repo run @semio-tech/repo-providers-go:test` did not finish within 10 minutes (slow graph, killed by my timeout). The Go package could not be compiled through the repo overlay here.
- Extracted the changed Go functions into a scratch ASCII module and ran them against a real git repository with emoji, space and quote paths: `GetGitDiffStatus`, `GetGitDiffLines`, `gitDiffNewPath`, `gitPathField` all return the real paths. The `+++` trailing tab git adds for names with spaces is handled.
- gofmt is clean on all five touched Go files. rustfmt --check reported differences on the Rust provider and test files; not investigated.
- The Rust and TypeScript scenario implementations were not compiled or run.

## Ticket repair

The `files` entries of `GET-NORM-PLUGIN-WORKING-END-TO-END/🎫️ticket.json` (interaction `ticket.close`)
were decoded by matching their lossy stored form (octal bytes, `/`, `-` and extension dropped)
against `git ls-files`. Six entries resolved uniquely and exist on disk. `⚙️bunfig` was mapped to
`bunfig.toml` (the only existing candidate, inferred).
