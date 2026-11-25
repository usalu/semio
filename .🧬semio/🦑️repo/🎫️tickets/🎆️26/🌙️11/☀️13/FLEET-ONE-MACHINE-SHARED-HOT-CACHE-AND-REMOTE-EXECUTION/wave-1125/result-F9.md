# F9 result: warden coverage

Already landed 2026-11-24 (verified present, tests green): identity roots (pinned toolchain, hashed deps/ test binaries, repo browser runtime, headless browser kind), cargo kind, disk-bounded swap headroom, native cargo admission through `repo cargo` (ticket scripts of SOURCING-END-TO-END), unit carving. No file in the warden was touched in the last 2 h.

Remaining gap found and closed: scripts that bypass the launcher (`CARGO_BUILD_RUSTC=rustc`, `--config build.rustc=...`, ~dozens of ticket scripts) run a rustup/Homebrew rustc/cargo, which no identity matched. New identity (schema `Identities`): cargo|rustc|rustdoc|clippy-driver of any toolchain whose command line names a path below a workspace root (out-dir, source, manifest, deps). Same tools of unrelated projects stay non-subjects.
Files: warden `🧬️schema/🔣️.json`, `🧪️tests/🧫️fixtures/🔣️.json` (new case, oracle regenerated), `🧪️tests/🟦️.ts` (assertion). Tests: `repo test-file` warden 46 pass/0 fail.

Not verified / open: bypass runs with a target dir and relative sources entirely outside the workspace (e.g. C:\t\..., /tmp) name no workspace path in argv, so they remain invisible (needs cwd; the warden reads no cwd). The existing old ticket scripts still bypass admission; only the warden covers them. Windows stays observe-only. Live warden restart on the new digest not checked. Coordinator `warden/mode` and `protect.json` untouched.
