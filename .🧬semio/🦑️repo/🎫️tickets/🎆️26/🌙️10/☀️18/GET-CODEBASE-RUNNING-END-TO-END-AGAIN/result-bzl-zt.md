# result-bzl-zt: zero-touch cross-platform Bazel bootstrap (G13, G14, G4/G15, WP5)

Abbreviations: `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `BS` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel`.

## Changes
- `B/📜️script.ts` (only the `tools setup` Bazel branch): `bun repo tools setup bazel|all` now (1) runs `requireBazelPlatformV1` before any image is attached, (2) opens the full Bazel storage (`withBazelStorageV1`: tools, dependencies, products), (3) provisions the pinned binary with throttled progress lines (phase + every 10 percent) and per-event `invocation.control.publish`, (4) runs `prepareBazelModulesV1` (module catalog and repository cache, offline-aware, cancellable through the phase signal), (5) prints `[bazel] ready binary=... modules=N blobs lock=...` and a schema-validated `[bazel] readiness {json}` line.
- `BS/📜️script.ts`: `prepareBazel` calls `requireBazelPlatformV1` too, so a plain `--tool=bazel run` also refuses non-ASCII Windows ancestry before download or launch.
- New `BS/🩺️readiness/🟦️.ts`: pure `bazelPlatformProblemsV1` (facts to refusals and warnings), `bazelNonAsciiV1`, `bazelMountedPartitionsV1` (mountinfo), `collectBazelPlatformFactsV1` (read-only observation, cancellable), `requireBazelPlatformV1`, `bazelProgressLineV1`, `describeBazelReadinessV1`.
- `BS/🧬️schema/🔣️.json`: added `PlatformPolicy` (const), `PlatformFacts`, `Readiness` definitions (schema-first; the `Release` manifest is untouched).
- Tests: `BS/🩺️readiness/🧪️tests/🟦️.ts` plus `🧫️fixtures/🔣️.json` (11 platform cases, invalid facts, mountinfo, progress, non-ASCII corpus). Third-party cross-checks: Ajv closed-schema admission and an ECMAScript `^[\x00-\x7F]*$` pattern oracle for ASCII detection, `awk` for mountinfo, `Bun.CryptoHasher` for the lock hash, `yaml` for the compose file.
- `.devcontainer/docker-compose.yml`: `privileged: true` on the `semio` service (needed by `mount -o loop`); `.devcontainer/README.md` documents it.
- Not touched: the adapter `B/🔧️tools/🏗️bazel/📜️script.ts` (bzl-act; `prepareBazelModulesV1` stays there and is called from the branch), `💾️storage/🟦️.ts` (bzl-cache).

## Decisions
- Windows (G14): refuse, do not relocate. Evidence from the codex report: SUBST drives and 8.3 names did not help because Bazel 8.4.2's native launcher canonicalizes back to the real Unicode path (`ClassNotFoundException com.google.devtools.build.lib.bazel.Bazel`); a junction is resolved the same way, and I cannot validate either on this Mac. A silent relocation would also split the fixed-image cache identity per machine. The refusal names the offending path, code point and index, and the remedy (ASCII checkout such as `C:\semio`). Blocking paths: workspace and the three partition roots; a non-ASCII `%TEMP%` only warns (not proven to matter). Not elevated and partitions not mounted: warning about `Mount-DiskImage` HRESULT `0x80070522` (cannot be refused safely because already-mounted images need no admin).
- Linux and devcontainer (G4/G15): no unprivileged mount exists for ext4 loop images; a FUSE backend would be a new image backend (out of scope). Implemented a precise preflight (CAP_SYS_ADMIN bit, writable `/dev/loop-control`, at least 3 free writable loop nodes, `fallocate mkfs.ext4 mount umount losetup` in the same PATH the fixed backend uses), skipped when all three partitions are already mount points. Devcontainer made privileged.

## Verification
- `bun test ./BS/🩺️readiness/🧪️tests/🟦️.ts`: 8 pass, 0 fail, 97 expects.
- `bun test ./BS/🧪️tests/🟦️.ts` (existing distribution test): 1 pass.
- `bun build --target=bun B/📜️script.ts`: bundled 612 modules, resolves the new imports. Root `bun build --target=bun 📜️script.ts` fails on an unrelated peer move (`⚖️laws/🎯️tool-job-coverage/…/🖼️caller-retained-media` imports a missing `🔍️discovery`).
- `tsc --noEmit --strict` over `B/📜️script.ts`, `BS/📜️script.ts` and the new test: only an unrelated error in `📦️dependencies/💾️storage/🟦️.ts(510)` (`chunkBytes`, deps owner).
- `perl -e 'alarm 590; exec @ARGV' -- bun repo tools setup bazel`: NOT RUN. `ps` showed `bun repo dependencies lock --all` (pids 41418 tree and 45509) running.
- Windows and Linux paths: unit-tested through fixtures only; never executed on those OSes.

## Open items
- Run `bun repo tools setup bazel` once the dependency lock finishes; expect it to open the products image (24 GiB fixed) because `BazelStorageV1` requires all three stores.
- Early generic Linux privilege check: the `dependencies` image is attached by the entry before the `tools` branch for every command (also nx), so an unprivileged Linux box fails there with the native error; extending the existing `repo preflight` entry (outside my ownership) would cover it.
- `LANG`/`LC_ALL` UTF-8 is set only on darwin in the adapter; a `C` locale container may need it for Bazel (bzl-act).
- Windows junction/subst root remains unproven; only a Windows run can overturn the refusal.
