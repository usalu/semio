# Transfer and Validation Plan

Goal: transfer all authored Niloufar teaching video sources to the existing teaching domain in Ueli's checkout, preserve unrelated concurrent changes, and stage the complete working result.

Source: C:/git/semio.worktrees/🦢niloufar/⛳wip/tutorial. Destination: C:/git/semio/🎓️teaching. The 131 canonical files include five full series, 26 authored chapter scenes, shared rendering/TTS helpers, timing fixtures, configuration, documentation, and the intro PDF. Generated media and agent configuration are excluded.

Validation starts with an independent JSON video contract and import/portability tests, then exercises Manim to render every series at authored timing and low quality (854×480,15fps). FFprobe metadata and full FFmpeg decoding validate final outputs, concat duration/frame totals, and usable beginning/middle/end frames. Generated output stays under this ticket and is removed after evidence is recorded in Markdown. Audio synthesis requires external credentials and is optional; the source has no rendered audio/video artifacts, and its canonical full-video outputs are silent.

Fleet: main coordination requested GPT 6.1 Sol xhigh, two low-effort read-only audits, and high-effort execution agents. Four concurrent slots including main are available; they remain filled during independent work.

Infrastructure: repo not on PATH. Cached repo.exe exposes MCP only; cached semio-repo.exe lacks command execution. Goals read and ticket opened through repo MCP. Existing ticket index stalled, so a targeted filesystem ticket-name audit established no matching transfer ticket. The repo MCP model allowlist rejects GPT6.1, so ticket metadata uses supported generic codex label with actual fleet recorded in prompt. A ticket-local repo launcher is built using a Go overlay (no tracked source copies).

## Native prerequisites

The transferred project pins Manim 0.22.0, NumPy 2.5.4, and imageio-ffmpeg 0.6.0 in an isolated Python 3.12 project. The teaching repo script provisions that environment through uv. Native shell bootstrap now includes Cairo/Pango development packages for apt, dnf, pacman, and zypper and Homebrew Cairo/Pango; the devcontainer adds its matching development headers. A language-neutral prerequisite fixture failed 11 assertions before the changes, then its two test groups passed. Git Bash syntax validation passed; macOS/Linux/container execution is not available on this Windows host.

Fresh-env repo teaching test initially passed five cases with optional Manim check skipped while extra cache regression tests were being added; final testing will enable Manim validation explicitly. Independent jsonschema validates five videos and 26 authored chapters against the checked-in schema.

## Execution and Review

The Windows host has 16 logical CPUs and 64 GiB RAM. Four original complete-series renders are supplemented by eight isolated chapter workers; full authored timelines remain 854×480 at 15fps. Chapter jobs keep independent text/partial/config/log paths and retain outputs until safe final publication. The first scheduler was corrected to hold publication while forced old parents remain active; its eight Manim child renders were preserved using native process handles.

Only explicit teaching files and selected README/native prerequisite hunks are staged. Shared-file staging reads the current index and applies task-specific edits, preserving other worktree/index changes. No commits, stashes, branch checkouts or new worktrees were made. The source curriculum files are transferred into the teaching domain; an existing separate Bauen mit Bestand MP4 is byte-identical in destination and needs no transfer.

Path correction: initial PYTHONPYCACHEPREFIX mirrored long absolute import paths. Ticket-local startup .pth disables bytecode before virtualenv hooks, and those caches were removed. Subsequent audits show no generated paths above the 256-character limit. Production runner and wrappers disable bytecode output and cache only explicitly selected render artifacts.

Final entry-point test now passes all seven cases including independent Manim scene discovery. Independent JSON Schema and native prerequisite checks pass, with Bash syntax validation.

The font regression suite uses FontTools only in the explicit test dependency group (4.63.0). The repository teaching script provisions that group for test runs and supplies a default test output root under its local cache, so the documented zero-touch test command needs no preconfigured environment variables. Task validation overrides the cache root into this ticket.
