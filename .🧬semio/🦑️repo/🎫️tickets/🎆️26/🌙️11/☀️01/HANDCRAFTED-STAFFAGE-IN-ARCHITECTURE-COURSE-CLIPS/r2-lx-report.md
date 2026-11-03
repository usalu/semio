# r2-lx report: cast-export cancellation on real Linux (WSL2 Ubuntu-24.04)

## Zero-touch setup on Linux (what ran)
- `bash 🚀️launcher/📥️install/🐚️.sh` (the launcher installer the bootstrap itself calls; user space, needs only curl/tar/sha256): fetched the pinned Go 1.26.9 (SHA-256 checked) and built `~/.local/share/semio/launcher/bin/repo`. Worked with no sudo.
- `repo --help`: the launcher provisioned the pinned **Bun 1.4.2 linux/x64** itself (`~/.cache/semio/runtimes/bun/...`). No manual curl install. Needs `SEMIO_LANGUAGE=en` (no default language, by design).
- `repo preflight` from `/mnt/c/git/semio` (about 14 min, because source admission walks the tree over 9p) **refuses**, correctly and with remedies in en/de:
  1. `/mnt/c` is 9p: no atomic directory exchange (errno 22) and not case-sensitive. The remedy text says to put the repository or its `.cache` on ext4/xfs/btrfs. No Linux-side checkout location is documented, so none was created.
  2. missing `cc` and `pkg-config`, and no Rust/rustup.
- The full bootstrap (`🔩️native/🥾️bootstrap/🐚️.sh setup`) was NOT run: its first step is `apt-get install build-essential pkg-config ...` through `sudo`, and the WSL user has no passwordless sudo (`sudo: a password is required`). Supplying a password or switching to `wsl -u root` is a decision for the owner, not an agent.
- Consequence: `repo tools setup`, `repo dependencies sync` and `repo tools exec` cannot run on this WSL as is. Without a Linux `node_modules`, the **vitest** cancellation cases (including the POSIX-only SIGTERM case) could not be run. They remain unexecuted under vitest.

Real finding, not in this ticket's area (build infra, README already states the ext4 requirement): a zero-touch Linux run on a Windows-hosted checkout through WSL is impossible by design (9p). The documented route is a Linux-side ext4 checkout plus `sudo` for the package step. Nothing to fix in staffage or visuals.

## What could be proven on Linux with the repo-provisioned Bun
Substitute used for the unavailable `repo tools exec --light bun --`: `T/🧰️lx/repo-shim.sh` (maps that command line to the repo-provisioned Bun, runs it as a child, does NOT exec, so shim and Bun are two processes like the real chain).

1. **Python `Cancellation` tests, the real `ensure_casts` / `_run` / `_Tree` against the real Bun exporter: 5 of 5 OK on Linux** (`T/🧰️lx/cancellation.py`, run by `py.sh`; stubs only `manim` and `numpy`, which these tests do not use; the real module and the real test class are loaded):
   - progress reaches the caller and the cast lands in the cache;
   - `KeyboardInterrupt` mid-export kills shim and Bun (two pids), cache snapshot unchanged, no `.tmp-*`, under 60 s;
   - a failing progress callback cancels the same way;
   - a Python-parent grandchild dies with the group;
   - the exporter is told `--parent <python pid>`.
   No survivor (`pgrep`) afterwards.
2. **Exporter CLI on Linux, direct** (`sigterm.sh`, the same fixture as the vitest case): SIGTERM after the first frames gives exit **130**, `[cancelled]` on stderr, empty `out`. Hard `kill -KILL` of the watched parent pid gives exit **130** with `[cancelled]`, only the complete cast `slow-a` in `out`, no `.tmp-*`. No survivor.
3. Earlier px result (T/r2-px-report.md) for `_Tree` alone stays valid; this run adds the end-to-end Python to Bun case.

## Notes
- A first run showed `JSON Parse error` in 3 tests while my `repo preflight` was running concurrently and republishing generated files in the shared checkout. The rerun with preflight finished was clean (5/5, twice). Treat concurrent `repo` runs against the same checkout as a transient cause. This is not a code defect found.
- The first 5/5 attempt failed one assertion (`>= 2 pids`) only because my shim used `exec`; fixed in the shim.
- Windows `.cache` was untouched (preflight refused before store allocation). No file in the repo outside this ticket changed.

## Open
- Run vitest `cast-export` (SIGTERM case) and the real `repo tools exec` chain on an ext4 checkout after `sudo bash ...🥾️bootstrap/🐚️.sh setup` (owner action; or in the devcontainer once Docker runs).
- WSL distro and all processes: none left running (checked with `pgrep`; distro terminated).

Scripts kept in `T/🧰️lx/`; `T/🗑️generated/lx/` deleted.
