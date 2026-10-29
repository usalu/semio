# Current Native Linux Installed Candidate

The current frozen Go candidate was rebuilt and installed successfully. Actual search completion remains unproved: the first bounded search did not finish within its explicit 120-second verification budget. Independent MCP metadata and active cooperative cancellation passed.

## Source and Build Provenance

Fresh host metadata covered 1127 present nonignored files, 16584812 bytes, including all 800 current Go files, Go module/workspace manifests and changed/untracked implementation/configuration files. It contains paths, sizes, modes and SHA-256 values only, with no copied Source payload. The existing private Source-overlay container compared all selected file modes and bytes with zero drift.

No actual Go embed directive was found in those selected current Go sources. The unchanged private Source view and existing owned caches were retained; no Source copy, endpoint reset, worktree or full workspace setup was used.

The actual registered `repo run repo-mcp:install` command completed0 as handle21913 under normal source/compiler admission. Its streamed output is `🗑️generated/native-final-install.log`.

The installed ordinary ELF is /home/vscode/.local/bin/repo, mode0755, 20336729 bytes. SHA-256:

`1a42b0b11bd20e24033b6a69e643765b41a7e9708618a4551e61a9747da727ea`

The current installed.json receipt names exactly that digest and executable, with prototype mode and the owned native build product. Source comparison output is `container-source-final-candidate.log`.

## First Search Attempt

Handle25550 completed1 because the first query exceeded its own 120000 ms budget:

`repo --repo=/workspaces/semio --format=json search linux --limit=8`

The process emitted its first structured stderr progress in11.7 ms and37 progress events overall. Observed stages included fingerprinting, fingerprinting-source, discovering-folders and discovering-files. The final observed counter was252416 for discovering-files. These counters are reported as emitted; they are not an independently established file census.

At the explicit budget boundary the verifier sent SIGTERM to its exact owned Go child. The Go process returned status130, no OS termination signal, at120027 ms, with empty stdout. The proof field forced=true records that the verification budget fired; it does not mean SIGKILL was sent. No search result, completed index or warm-cache success is claimed.

The batch stopped after this first completion failure. It did not repeat the warm, only-file, unscoped tree or scoped tree completion attempts. Root assigned the leading stall diagnosis to the Go owner. Full streamed output is `native-current-validation.log`, and parsed process facts are in `native-current-attempt-proof.json`.

## Independent Bounded Controls

At Root's explicit direction, one MCP metadata request sequence and one positive-progress SIGINT probe ran independently, each capped at25 seconds. Handle67809 completed0.

MCP initialization, initialized notification, tools/list and resources/list completed0 in21 ms. Replies preserve exactly IDs1,2,3, contain no errors, negotiate protocol2025-06-18 and expose6 tools and8 resources. The notification produces no reply. No MCP search or other backend tool execution is claimed.

The cancellation query emitted a positive fingerprinting-source counter19968. The verifier then sent SIGINT only to that owned child. It settled2 ms after the signal, status130, no OS signal, empty stdout, total1019 ms, with no verification timeout. This proves cooperative cancellation during active fingerprinting; it does not establish cancellation responsiveness inside the later stalled phase.

Proof is `native-current-control-proof.json`, with raw per-command stdout/stderr reconstructed from the directly streamed host log. No CLI cursor flag was invented or used.

## Diagnostic Coordination and Remaining Work

The Go owner reported starting its sole authorized7-second stack helper45331 just before receiving the controls message. A possible brief overlap with the roughly one-second independent controls is recorded rather than excluded. The original120-second search had already terminated. No second completion attempt or duplicate stack query was launched by this agent.

All of this agent's native/control handles are terminal and its child processes have settled. The Source overlay and installed ELF remain unchanged. Search cold/warm/scoped/tree completion, full setup and full monorepo gates remain outstanding. The separate live caching-owner manifest size issue and Nx watch-capacity investigation remain recorded in their own audits.
