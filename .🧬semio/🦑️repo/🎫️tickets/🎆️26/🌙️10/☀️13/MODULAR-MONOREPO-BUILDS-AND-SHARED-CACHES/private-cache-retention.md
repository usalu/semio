# Private Cache Retention Audit

The nominated old private compiler stores remain intact. Inactivity alone does not establish disposal authority: the authored BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT ticket inputs explicitly reuse these directories.

| Store | Observed size | Authored reuse evidence |
| --- | --- | --- |
| `l1/fgUvgd0/b` | 1.0 GiB | `l1-library-xor-component/📜️script.ts` exposes `resume <generation>` and selects that generation's `t` and `b` as warm shared storage. |
| `l1/{target,build}` | Earlier inventory approximately 22 GiB | `📓️2026-10-13-wave-l1-library-xor-component.md` describes bounded resumed chunks using these exact private directories. |
| `e4/{b,t,head-b,head-t}` | Approximately 7.8 GiB | `e4-3d-architect-bim/🏗️cargo.sh` uses fixed private storage across resumed slices. |
| `at/{b,t}` | Approximately 5.5 GiB | `artifact-tests/🦀️cargo.sh` explicitly documents deadline exit 124 as resumable on rerun. |
| `tc/{build,target}` | Approximately 2.9 GiB | `trusted-catalog-receipts/🏗️cargo.sh` selects these fixed private directories. |
| `ort/{after,before,check,test}` | Approximately 4.3 GiB | `owned-runtime-crate/🏗️cargo.sh` selects fixed reusable `${ORT_LANE:-check}` storage. |

The completed `l1/fgUvgd0/r/report.md` records one failed wasm-release pass, exit 1, 567.9 seconds. That final execution report does not revoke the explicit resume API. `at/results.jsonl` records completed individual test compilations, including failures and blocked dependencies, without establishing the whole private store as disposable.

The first audit performed no deletion. The previous targeted `fChPzBn/t` deletion remains separately justified in `private-fresh-reclamation.md`; its launcher lacks a resume API. At the first audit, disk availability was 176 MiB. The initial no-reuse restriction was a conservative workstream screen, not a user requirement.

Sources examined were retained original ticket inputs, generated compiler reports and result ledgers. `du` supplied size observations. The process/environment inventory was examined only for references; environment values are not retained in this report. A complete open-descriptor check was unnecessary after authored reuse semantics already excluded these candidates.

## Authorized Inactive Private Output Reclamation

The coordinator clarified that reproducible inactive private compiler outputs are within the authorized architecture/cache refactor. Optional historical private warm intermediates may be discarded to unblock current compilation; authored source/configuration/scripts, reports and provenance must remain. Shared/current canonical Cargo storage must remain warm.

Executed the retained `private-reclaim/📜️script.ts` with Bun through actual Nx exec under its original process envelope. The script held current `.cache/framework/leases` cache-prune exclusion plus all debug, release, prototype, wasm-prototype and wasm-release compiler-profile exclusions for every affected target/build store. It checked remaining original control throughout removal, published progress, yielded continuation and released leases in reverse order.

Before mutation, sanitized process/environment inspection found zero references to either private root. Independent `lsof -nP +D` found zero open descriptors under each root. The script repeated live-reference inspection after descriptor admission and approximately every second while removing compiler outputs, and refused links/recent stores before admission. No external process was terminated.

Removed only these reproducible compiler profile directories:

- `l1/build/{debug,wasm32-wasip2}`
- `l1/target/wasm32-unknown-unknown`
- `l1/fgUvgd0/{b,t}/{release,wasm-release,wasm32-wasip2}`
- `at/b/debug`

Actual execution exited 0 and counted 37,968 files / 30,037,753,825 bytes. Disk availability increased to 30 GiB. Top-level compiler metadata/warning reports, l1 registry log, build telemetry, generation reports/timelines/provenance, compiler resource records, at test results/logs and all authored original ticket inputs remain. `e4`, `tc`, `ort` and the canonical shared Cargo store remain intact. Re-running the old private lane commands remains possible, but recompiles the discarded private warm intermediates.

Raw generated evidence is `🗑️generated/private-reclaim-current.log`; the input script and this audit remain after generated cleanup.

## Secondary Private Compiler Profiles

The retained canonical reclamation input gained an explicit `secondary` plan and a matching launch registration. Actual Nx invocation `private-reclaim-secondary-2.log` completed exit 0 after holding cache-prune and every affected debug/release/prototype/wasm-prototype/wasm-release/host-tool store exclusion. Full private roots were independently admitted through zero live process/environment references and zero open file descriptors. The bounded recursive removal rechecked live references while publishing original-control progress and yielding continuation. All source scripts, reports, compiler metadata, logs, timings, warnings and provenance remain.

Removed only e4 `{b,t}/{debug,wasm-release,wasm32-wasip2}`, e4 `{head-b,head-t}/debug`, tc `{build,target}/debug`, ort `{after,before}/{build,target}/host-tool`, and ort `{check,test}/{build,target}/debug`. Every selected store and output was a real old directory before admission. Actual removal counted 37,691 files / 15,982,428,764 bytes. The current canonical shared Cargo cache remained untouched. Disk was 19 GiB immediately after completion while concurrent compilers remained active. Optional historical private warm reruns rebuild these intermediates.

The first secondary attempt failed before mutation because its relative script path resolved in the selected project working directory. The successful retry used the absolute retained input path, matching its launcher. No process was interrupted or daemon state reset.
