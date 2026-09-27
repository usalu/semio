# WP-Z4 — Cross-Platform Zero-Touch (Linux B1–B4, Devcontainer, Native Windows, Docker Hub Image)

Slice Z4, session 14 (2026-09-27 18:2x), Opus 5.5 executor; successor of Z3 ([`📓️wp-z3.md`](📓️wp-z3.md)) and Z2
([`📓️wp-z2.md`](📓️wp-z2.md)); sources [`📓️audit-s11-cross-platform.md`](📓️audit-s11-cross-platform.md),
[`📓️acceptance-s13.md`](📓️acceptance-s13.md) rows 2.8/2.9/5.1. Ports: hubs 8190–8199, serves 6690–6699. Private target
`.tmp-ticket/wp-z4/target`. Durable data `.🧬semio/🌐hub/s14-z4-*`. Captures `wp-z4/generated/` (expendable). One-off
probes/patches `wp-z4/*`.

## Session 14

| # | Item | Status | Evidence |
|---|---|---|---|
| 1 | Linux B1–B4 re-derived against the current tree; hub-only land now, guest/framework prepared for window 3 | B4 IN TREE (Z3); Linux cross type-check of the 3 winit crates QUEUED in the native lane (pid 16982). B1/B2/B3 + new B5 + 2 taxonomy laws PREPARED: `wp-z4/b123-fresh-clone.py` (23 hunks + 1 move), dry run **23/23 ready** on the live tree; applied to the fresh-clone view: idempotent (23/23 applied), `validateTaxonomy` 0, generator validator 0 (was 2), `assets` render(all) with the tracked snapshot, new law **1/1** (git oracle), browser-entry-identities case **1/1**; law red on the live taxonomy with exactly B1/B2/B3. None is hub-only (taxonomy, discovery, 4 project.json, os-infinite `build.rs`) → window 3 | `generated/b123-dry-run-1.txt`, `b123-law-view-4.txt`, `b123-view-checks-1.txt`, `b123-package-integration-view-3.txt`, `b123-law-red-on-tree-1.txt` |
| 2 | Zero-touch devcontainer for hub + `s` serve (config, post-create via bun/nx only), static review; container proof after the go | IN PROGRESS | |
| 3 | Native Windows audit of the nx targets / launch rows of the four outcomes; fix open files now, prepare frozen ones | IN PROGRESS — paths clean (78 027 checkout paths: 0 forbidden chars/reserved names/trailing dot/non-NFC/case collisions/symlinks, 0 over MAX_PATH below an 18-unit root); nx targets: all 2 428 `run-commands` start with `bun`, 0 shell operators/POSIX expansions/env prefixes; 1 single-quoted argument (process3d `test-diff`), 1 launch row on `npx` | `generated/windows-path-scan-1.txt`, `windows-target-scan-1.txt` |
| 4 | Docker hub image: cold build + run `/healthz` + two-client smoke, permanent nx target + launch row; build after the go | HARNESS LANDED + PROVEN NATIVELY; nx targets/launch rows relayed to R10; image build/run waits for the Docker go | landing row 19:4x; `generated/native-posture-2.txt`, `test-forwarding-proxy-2.txt`, `hub-tsc-2.txt` |

## Log

- 18:2x started; read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no CHAIN LAUNCHED line yet), `📓️wp-z3.md`,
  `📓️wp-z2.md`, `📓️audit-s11-cross-platform.md`, `📓️acceptance-s13.md` 2.8/2.9/5.1, fleet-13 coordinator log.
  State: B4 (winit Linux backends) is in the tree (Z3: 3 manifests + 9 root lock entries, `x11-dl`, `smithay-client-toolkit`,
  `wayland-cursor` present); B1–B3 unchanged in `🔣️taxonomy.json` (frozen input → prepared patches).
- 18:3x **fresh-clone view** (`wp-z4/fresh-view.py`): tracked + untracked-not-ignored files (tickets and `.tmp*` excluded),
  78 015 files as APFS clones in the scratchpad (never hard links). The repository's own gate
  `validateGeneratorContractsAgainstWorkspace` (every `loadTaxonomy()` caller) run on it (`wp-z4/fresh-contracts.ts`):
  **2 problems** on the view, 0 on the tree — B2 `wgpu-frame-worker` tracked output `🎞️frame-worker/🤖️generated/🟨️.js` missing,
  B3 `scale-fixture` tracked output `🧫️fixtures/⚖️scale/🤖️generated` missing — also with the real git index visible
  (`GIT_DIR`+`GIT_WORK_TREE`, read-only), so the `semanticPackageSourceOutputPhase` escape does not cover a clone.
  B1: `external-emoji-shortcodes` is `ownership: external` + `inclusion: ignored` → a clone never has
  `🔣️icons/🤖️generated/🔣️shortcodes.json`; readers: assets builder (`renderShortcodes` throws "missing external
  🔣️shortcodes.json snapshot") AND os-infinite `build.rs:47` (panics) — os-infinite is on the hub's path, so B1 also breaks
  `cargo check -p semio-hub` and the Docker build from a clean context. Stale taxonomy input rows found on the way
  (absent even in the tree, harmless for clones): `dev-distribution-bundle` 3 `🚚️distribution/*.schema.json`,
  `wgpu-frame-worker` `Trunk.toml`, `🌐️.html`, 3 `🧪️tests/*.ts`, `🪢️kernel-seam/🦀️.rs`.
- 18:4x **Windows scans**: `wp-z4/windows-path-scan.py` (NTFS/Win32 naming over every checkout path) → clean;
  `wp-z4/windows-target-scan.py` (every `📋️project.json` `run-commands` target + every launch row, statically; the 351
  outcome launch rows resolve to 290 closure targets, 71 of them inferred by Nx plugins) → no shell-only construct in any
  target; `.gitattributes` = `* text=auto eol=lf` (LF on every checkout, `.bat`/`.cmd` CRLF, fixtures `-text`) → no CRLF
  drift for byte-fresh generators.
- 19:0x–19:4x **item 4 harness** (hub-only, rule 17): see the landing row. Design: the image's production posture needs
  `X-Forwarded-Proto: https` on every request (transport layer outermost) → `hubForwardingProxy` in the shared probe
  harness is a loopback stand-in for the TLS terminator (HTTP + WebSocket, subprotocol negotiated upstream first), so the
  existing probe client drives a container unchanged. Docker Desktop on this Mac cannot read `~/Documents` (Z2) → the
  check never bind-mounts: catalog via `docker cp` into a stopped seed container, credential via stdin. Native proof of the
  whole drill (os-hub 13:56 + B3 catalog, production mode, allowlist, proxy trust, loopback 8190):
  `bun wp-z4/z4-native-posture-proof.ts …` → ready 42 s, `/healthz` 200 through the proxy, 403 `insecure-transport` without,
  two-client relay A→B 65 ms / B→A 40 ms, 0 undecodable frames, SIGTERM → exit 0 in 1.0 s, trace `server.shutdown ok`.
  First run found the default kind id is `s.note.note` (not `note`) → default fixed. Law 3/3; hub `tsc` 0 errors.
  Found on the way (not mine): `🚧️hostile-input` law red in the os-hub-ts suite ("every declared route body schema refuses
  the wrong-schema and malformed vectors"); the default `test` target's 15 s budget kills the whole os-hub-ts vitest run
  (64 s, transform/import-bound) — `generated/test-hub-ts-all-1.txt`.
- 19:4x–20:1x **item 1 prepared set** `wp-z4/b123-fresh-clone.py <root> [--apply]` (dry run by default, idempotent):
  - B1: snapshot moves to the tracked `🔣️icons/🔣️shortcodes.json` (taxonomy input + external output → tracked; builder
    `renderShortcodes(icons, iconsDir, generatedDir)`; os-infinite `build.rs` path + message; os-infinite `nativeAssets` input —
    also ends the Nx hasher never seeing the ignored snapshot).
  - B2: frame worker declared `ignored` (taxonomy entry + output root, `parseSemanticPackageBrowserProfile` expected row,
    both fixtures, package-integration oracle); wgpu `serve`/`dev` gain `generate-frame-worker`.
  - B3: scale fixture declared `ignored` (+ reason); plugin-host `test` + `ui-patch-marshalling-native-check` (the
    `include_str!` reader) and renderer `native-scale`/`native-scale-release` gain `generate-scale-fixture`.
  - B5 (NEW, found by the law's git oracle): `flow-browser-package` declares `🫀️core/🕸️bindings` tracked; `**/🕸️bindings/**`
    is ignored and its one indexed file `package.json` is a live indexed-generated-output breach (the `flow-core-package`
    bootstrap source publishes it on every clone). Declared `ignored`. **Needs a human/coordinator `git rm --cached
    🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/package.json`** (I run no modifying git).
  - LAW: `validateTaxonomy` refuses a tracked output inside `🤖️generated` and an untracked external input; law test
    `fresh-clone generator outputs` in `🔬️workspace-contract` (mutants for both shapes + `git check-ignore --no-index` as the
    third-party oracle over every declared-tracked output, probing inside directory roots).
  - Proof (scratch fresh-clone view, then with the tree's 18 `🤖️generated` dirs cloned in for the renderer test): see row 1.
    Not provable before window 3: os-infinite `build.rs` compile (`cargo check -p semio-framework-os-infinite`, native lane)
    and the plugin-host `include_str!` edge (needs `generate-scale-fixture` through nx).
  - Pre-existing reds met on the way (same on the unpatched tree, not caused by the set): `🧱️root-artifact-dependency-source`
    3/5 fail (its fixture schema refuses the `producer` key the taxonomy now carries); package-integration
    "browser entry identities" times out at the 5 s default under load (passes with `--testTimeout`).
- 20:1x **B4 Linux proof without a container**: the `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-msvc` std targets are
  installed, so `cargo check --target x86_64-unknown-linux-gnu -p semio-framework-ui --features wgpu-engine -p
  semio-framework-ui-host -p semio-framework-os-renderer-wgpu --lib` is queued in the native lane (9 ahead), detached
  (`wp-z4/z4-cargo.sh`, pid 16982, capture `.🧬semio/🌐hub/s14-z4-logs/check-linux-winit-1.txt`). Only C in that closure:
  `wayland-backend`'s build script (`cc`) and `wayland-sys` (`pkg-config`, dlopen mode).
