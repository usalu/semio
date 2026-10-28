# mirror-tools: ONE shared, safe short-root overlay for verifying staged batches

Ticket tooling (not product code), lane `mirror-g`, wave 1026g. Replaces the per-lane "short-root mirror / overlay / workbench" symlink farms
(two write-through incidents on 10-26: testmem2-g 17:10, obs-g into `📓️build-guide.md` 19:01).

`python3 -I mirror.py ...` (stdlib only; macOS, Linux, Windows). Let `MM="python3 -I <C>/mirror-tools/mirror.py"`.

## Commands

```sh
MM create <name> [--batch <stage-dir | command with {root}>] [--only <subtree|alias>...] [--force] [--no-nm] [--no-closure]
MM run    <name> [--cwd REL] [--peers-ok] [--no-seal] [--seal-allow SUBPATH]... -- <cmd...>
MM diff   <name>                 # M changed / A only in mirror / D only in live, vs the live tree now
MM list
MM drop   <name>... [--sync]     # or: drop --prefix <lane>-   |   drop --nm [--force]
```

- Mirror root: `$TMPDIR/sm/<name>` (override: `SEMIO_MIRROR_HOME`; live root: `SEMIO_REPO`, found from the script by default).
- **Name**: `<lane>-<what>`, max 8 characters (the deepest library test path is 252 chars with a 5-character name; the library rejects > 256). `create` prints the longest path and warns.
- **Never `drop` another lane's mirror.** There is deliberately no `drop --all`; `--prefix` only for your own prefix.

### What `create` copies (all real copies, no symlink into the live tree)

1. Root files + `.cargo` + `.config`, and the sibling files of every ancestor directory of a requested subtree (tracked only).
2. The requested subtrees: **tracked + untracked-not-ignored**, taken from the working tree, so uncommitted live edits and new files ride along. Default `--only`: `library process launcher` (aliases: `library process launcher mcp repo framework`; any repo-relative path works). Files > 64 MB are skipped and reported.
3. **Import closure** (default on; `--no-closure` = exactly `--only`): relative imports and workspace package names found in the copied sources are resolved to the nearest enclosing `📋️project.json` project, which is copied whole, transitively. The default set grows from 6.7k files / 76 MB to ~16k files / 173 MB (80 projects). Reads that are not imports (data files of another project, `path.join(__dirname, ...)`) are not found: add them with `--only`.
4. `node_modules` of every copied package directory. The managed one lives on a DMG volume (no clone possible), so the first use builds ONE shared real copy in `$TMPDIR/sm/.nm` (1.6 GB, ~3 min once), later mirrors clone it in ~3 s. Every absolute symlink into the live tree inside it (134 workspace links) and every relative link that escaped to the root `node_modules` is rewritten to point into the mirror; links to projects that were not copied dangle (a loud import error instead of a silent read of live files).

Copy method: macOS `clonefile(2)` (APFS copy-on-write, a few MB of metadata per mirror), Linux `FICLONE`/`cp --reflink=auto`, Windows plain copy.
No `.git` in the mirror (tools that need git must run against live in read-only fashion or skip).

### `--batch`

- A **stage directory** with `manifest.json` (`{rel: {file, base, new}}` + `files/<file>`, the obs-g / xplat style): files named in it are added to the mirror if missing, each is checked against its `base` sha256 (mismatch = exit 3, `--force` overrides) and replaced atomically IN THE MIRROR.
- A **command** (anything else): run through `sh -c` inside the mirror after cloning, under the same seal and guard as `run`; `{root}` is replaced by the quoted mirror root and `SEMIO_MIRROR_ROOT`, `SEMIO_STAGE_ROOT`, `O2_ROOT` are exported. Pass it to the tool's own `--root`, e.g.
  `MM create qfc --batch "python3 -I $T/edit_schema.py {root} && python3 -I $T/edit_code.py {root} && python3 -I $T/install_new.py {root}"`.
  A tool that hard-codes the live path is stopped by the seal (macOS) or reported by the guard.

### Safety: seal + write-through guard (`run` and command batches)

- **Seal** (macOS, `sandbox-exec`, on by default): file writes below the live repo are denied for the command and everything it starts, except `.cache .nx .tmp .repo .venv .pytest_cache` (add more with `--seal-allow SUBPATH`; `--no-seal` turns it off). The write fails with EPERM at the moment of the write. Linux/Windows: no seal, guard only.
- **Guard** (all platforms, after every run): before and after, the live tree is snapshotted for every mirrored file (mtime_ns, size) and every directory listing of the mirrored directories (new/removed names). Any difference prints `GUARD FAIL` with the paths, tagged `also written in the mirror: WRITE-THROUGH SUSPECT` or `mirror untouched: hard-coded live path or peer edit`, and `run` exits **97**. Peers editing the same live files during the run cause the same report: re-check with `git diff`, or pass `--peers-ok` (warning only). Ctrl-C is forwarded to the command; the guard still runs.
- Not covered: writes to ignored locations outside the snapshot (`.cache/b`, `~`, ticket folders) when `--seal` is off.

## Verification

`python3 -I selftest.py` (20 laws on a throw-away git repo, `diff -rq` as the independent oracle): create/closure/untracked, no escaping symlink, link rewrite, mirror write leaves live untouched, `diff`, unsealed write-through exit 97, seal refusal (macOS), touched-file detection, `--peers-ok`, manifest batch + stale base exit 3, command batch with `{root}`, `drop`.

Real batches: `obs-g-tools/staged/fleet` (manifest batch; fleet-status + guard tests 14/14 green in the mirror) and `quietfc-g-tools` (command batch; 28 differences, host-capacity followup test 9/9 green); live tree unchanged (guard clean) in both. See `C/result-mirror-g.md` for numbers.
