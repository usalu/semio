# R2 ff report: repo ffmpeg registration and frame-timing vitest (2026-11-01)

Agent `ff`. Result: frame-timing vitest still FAILS on this host. The cause is that ffmpeg is not installed, not that the repo registration is broken. Nothing was installed and no repo infrastructure was edited.

## 1. How repo registers ffmpeg

Registry entry: `🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json`, lines 2526-2597 (`id: "ffmpeg"`).
- `pin.state: "system"`, no version, no `digests`, no download URL, no checksum. Repo does not provision ffmpeg.
- `provision.kind: "system"`, `provision.command: "repo tools verify"`, win32 hint `winget install Gyan.FFmpeg` (darwin `brew install ffmpeg`, linux `apt-get install ffmpeg`).
- `locate`: env `SEMIO_TOOL_FFMPEG`, then PATH `ffmpeg`. Posix dirs `/usr/bin`, `/usr/local/bin`, `/opt/homebrew/bin` apply only off Windows.
- `probe`: `ffmpeg -version`, pattern `^ffmpeg version n?(...)`.

## 2. Verification on this host

- `repo tools list` (mcp `tools_list`): `ffmpeg` source `missing`, tried `SEMIO_TOOL_FFMPEG`, `PATH ffmpeg`. ffprobe is not registered.
- `repo tools verify` (mcp): exit 1 on 15 of 21 pins (bun, uv, cmake, ninja, tectonic, bazel, cargo-nextest, ... missing or drifted). ffmpeg is not in its table because it has no pin, so verify does not check it.
- `repo doctor --only=tools`: verdict degraded, no ffmpeg finding.
- `repo tools exec --light ffmpeg -- -version` fails with:
  `ToolErrorV1: Tool ffmpeg not found (tried SEMIO_TOOL_FFMPEG, PATH ffmpeg). Provision it with: winget install Gyan.FFmpeg.` (`code: "tool-missing"`).
- `ffmpeg` and `ffprobe` are not on the Git Bash PATH. `SEMIO_TOOL_FFMPEG` is unset.

## 3. Setup attempt

- `repo tools setup` cannot provision a `system` tool. Its only fix is the hint above.
- `winget install Gyan.FFmpeg` was NOT run. It downloads and installs a system-wide binary, which needs the user's explicit go-ahead. Once approved, install it and either put it on PATH or set `SEMIO_TOOL_FFMPEG` to the exe. That would make `repo tools exec --light ffmpeg -- -version` answer.

## 4. Frame-timing test lookup

- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧰️.ts:41-50`: `hasTool("ffmpeg")` runs `repo tools exec --light ffmpeg -- -version`, and `ffprobe` runs `ffprobe -version` from PATH. The lookup matches repo's registration: it goes through the repo tool, so it sees `SEMIO_TOOL_FFMPEG` and PATH the same way. `ffprobe` is not registered in repo, so it is only probed from PATH.
- `🧪️tests/🎞️frame-timing/🟦️.ts:33-37`: `encoders = hasTool("ffmpeg") && hasTool("ffprobe")`. The describe throws `ffmpeg and ffprobe are required: ffmpeg is a repo-registered tool (...)` when false. This is the fail-loudly change from r2-sk.
- Line 69 and 82 are the MP4 decode case (`ffmpeg -v error -i file -f null -`).

## 5. Vitest run with the ticket alias config

Command, from `🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript`:
`repo tools exec bun -- x vitest run --config <T>/q2-vitest-alias.config.ts frame-timing`

Result: exit 1. `Test Files 1 failed (1)`, `Tests 2 passed | 3 skipped (5)`.
- FAIL `|@teaching/staffage| ../../🧪️tests/🎞️frame-timing/🟦️.ts > 🎬️ encoded clips (requires the repo-provisioned ffmpeg and ffprobe)`
- `Error: ffmpeg and ffprobe are required: ffmpeg is a repo-registered tool (repo tools exec ffmpeg -- -version)`

The 2 passing tests are the non-encoder frame schedule cases. The 3 skipped cases are the encoder cases behind `hasTool`.

## 6. Open

- Install ffmpeg (needs user approval, see section 3). Then re-run the vitest command above. Expect the encoded-clip group to run, 5 of 5 cases, instead of failing.
- `ffprobe` is not registered in repo. It ships in the same Gyan.FFmpeg package. Consider registering it, or accept the PATH probe.
- Output of this run is kept in the report only. The generated log at `T/🗑️generated/ff/frame-timing.txt` was deleted.
