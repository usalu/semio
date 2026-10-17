# Result: CRATE-ROLLOUT-R2 (prefix `r2`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r2-tools/`. Logs: `F/🗑️generated/r2-*`. Started 2026-10-17 08:44.

## 0. Verdict

DONE for all four packages. Each was carved by `new crates` (verify with oracle exit 0, apply with `--backup`, `--check-applied` exit 0 with `onlyCombined` 0 for native and `--test`, admission/shared empty). The census (cgc7 reader copy `r2-tools/r2-census.ts`, `r2-census-final.tsv`) reports 44 crates in the four subset directories, 0 over the gate, lib and test closure.

| Package (subset dir) | Verdict | Crates produced (plus the existing `-export` / `-import` codec crates) |
| --- | --- | --- |
| 🏛️model | DONE | -snapshot, -schema, -examples, -insert-element (mutation leaf family), -mutations, -io, facade |
| 🔺️mesh | DONE | -snapshot, -schema, -examples, -mut-family, -mutations, -io, -editor, facade |
| 🖊️drawing | DONE | -snapshot, -inferences, -schema, -create-node, -change-stroke-width (leaf families), -editor, -binary, -io, facade |
| 🧊️brep | DONE | -snapshot, -schema, -examples, -mut-family, -viewer, -io, facade |

Order: model, mesh, drawing, brep (the four are mutually independent; each depends only on base, base-examples, contract and framework crates, none depends on another of the four; the already-carved `-export`/`-import` codec crates of each subset stay as they are).

## 1. Baseline census (cgc7 reader, host cfg, scoped copy `r2-tools/r2-census.ts`, `r2-census-base0.tsv`)

| crate | lib files/bytes/lines | test closure files/bytes/lines |
| --- | --- | --- |
| semio-s-artifact-stdio-semio-model | 97 / 388,552 / 5,522 | 123 / 476,878 / 6,444 |
| semio-s-artifact-stdio-semio-mesh | 118 / 449,273 / 6,306 | 236 / 721,935 / 9,318 |
| semio-s-artifact-stdio-semio-drawing | 120 / 517,593 / 6,757 | 241 / 956,401 / 9,889 |
| semio-s-artifact-stdio-semio-brep | 109 / 549,374 / 7,143 | 205 / 838,737 / 10,118 |

## 2. Per package

### 2.0 Common causes found and the source changes made (no compat layers, all callers updated at once)

1. Mirror-layout aggregates (`mesh`, `drawing`, `brep`): the aggregate component file `schema/mutations` imported its leaves with `use super::<leaf>;`. A `super::` escape of a `component` child makes `buildSlices` union every kid of the inline wrapper `pub mod mutations`, so the aggregate and all leaves (and, through the codec coherence, the io codecs) stayed one slice of 60-90 files. Source change: `use super::<leaf>;` -> `use crate::schema::mutations::<leaf>;` (17 / 17 / 13 lines).
2. `model`: the 12 leaves and their `diff`/`inverse` companions and `inverts` tests imported the aggregate's private imports through `use super::*;` / `use super::super::*;` (glob of the host). Source change by codemod `F/r2-tools/r2-glob-leaves.ts` (explicit imports of exactly the names each file uses; private aggregate helpers used by leaves widened to `pub`): 42 files.
3. cross-slice `pub(crate)` / `pub(in crate::io)` items (helpers of `schema::diff`, `schema::mutations`, `schema::snapshot`, brep `io` codecs) pull slices together by visibility coherence. Source change: widened to `pub` (top-level `fn`s only; test-gated fns keep their `#[cfg(test)]`).
4. `--mutation-families=auto` only chooses the converted plan when it has strictly fewer problems; all four need the converted (generic-`Op`) leaves.

Tools: `F/r2-tools/r2-blockers.ts <pkg> [on|off|auto]` (full blocker report, no writes), `r2-slices.ts` (slice union trace), `r2-glob-leaves.ts` (model codemod), `r2-census.ts` (scoped census), `step.sh`.

### 2.1 Result per crate: before -> after (lib files/bytes/lines ; test closure files/bytes/lines)

| original crate | before | after (every produced crate) |
| --- | --- | --- |
| semio-s-artifact-stdio-semio-model | 97/388,552/5,522 ; 123/476,878/6,444 | facade 1/3,641/81 ; 1/3,641/81. snapshot 12/127,311/1,546 ; 12/127,311/1,578. schema 27/100,194/262 ; 28/101,707/295. examples 24/88,416/1,652 ; 28/92,648/1,814. insert-element (leaf family) 27/35,282/792 ; 28/35,845/810. mutations 10/57,178/793 ; 10/57,178/828. io 6/42,366/795 ; 26/126,448/1,474 |
| semio-s-artifact-stdio-semio-mesh | 118/449,273/6,306 ; 236/721,935/9,318 | facade 1/5,382/101 ; 1/5,382/101. snapshot 12/103,519/1,232 ; 12/103,519/1,232. schema 26/127,561/337 ; 28/129,387/385. examples 23/107,335/1,770 ; 27/121,975/2,072. mut-family 28/37,536/806 ; 28/37,536/806. mutations 27/78,520/1,221 ; 28/87,513/1,438. io 6/28,340/551 ; 28/215,040/2,877. editor 5/39,043/638 ; 8/52,897/884 |
| semio-s-artifact-stdio-semio-drawing | 120/517,593/6,757 ; 241/956,401/9,889 | facade 1/5,190/98 ; 4/26,662/108. snapshot 13/128,501/1,415 ; 13/128,501/1,447. inferences 26/106,474/1,738 ; 28/110,600/1,887. schema 26/173,013/661 ; 28/176,881/742. create-node 26/49,236/865 ; 26/49,236/865. change-stroke-width 26/42,776/906 ; 28/54,317/1,103. editor 8/64,155/991 ; 14/181,721/1,375. binary 3/1,523/46 ; 21/185,100/2,279. io 2/25,038/469 ; 6/39,321/654 |
| semio-s-artifact-stdio-semio-brep | 109/549,374/7,143 ; 205/838,737/10,118 | facade 1/5,473/107 ; 2/18,721/339. snapshot 14/194,989/2,206 ; 14/194,989/2,265. schema 22/157,405/744 ; 23/162,631/823. examples 23/96,911/1,576 ; 28/121,733/2,026. mut-family 25/34,214/734 ; 25/34,214/734. viewer 25/75,457/1,255 ; 28/96,067/1,637. io 8/48,856/846 ; 28/204,501/2,965 |

Gate: 32 files / 262,144 bytes / 4,000 lines. The largest produced crates are at 28 files (target limit), 215,040 bytes (mesh-io test) and 2,965 lines. The existing -export/-import codec crates are unchanged and inside the gate.

### 2.2 Commands run and results (all from the repo root, niced, `ulimit -n 65536`)

| step | model | mesh | drawing | brep |
| --- | --- | --- | --- | --- |
| `new crates <pkg> --verify --oracle --sandbox=<tmp>` | exit 0 (oracle onlyCandidate 0 native/test, layout onlyCombined 0) | exit 0 (same) | exit 0 (same) | exit 0 (same) |
| `new crates <pkg> --backup=F/🗑️generated/r2-backup-<pkg>` | wrote 59 files, 6 crates, 6 owner rows | 85 files, 7 crates | 92 files, 8 crates | 77 files, 6 crates |
| `new crates <pkg> --check-applied --sandbox=<tmp>` (base HEAD) | exit 0, onlyCombined 0 / 0 | exit 0, 0 / 0 | exit 0, 0 / 0 | exit 0, 0 / 0 |

Repository checks after the last apply: owners scan `owners-tools/scan.ts` 2,744 packages / 2,872 owners, `missing` 18, none under `🗄️stdio` (all other plugins and framework, other agents); taxonomy `pw0-taxcheck.ts ✏️s/🔌️plugins/🗄️stdio` 0 mount findings, 1 tree finding (`gif` standard `sqlite` without subsets, existing before); `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts`: my four roots no longer appear (I removed the three stale ratchet pins: mesh 312, drawing 308, brep 275; model had none); remaining failures name `reasoning/wires`, `forms`, `graph`, `dag`, `jpg`, `energy` roots (other agents). Both launch JSON files parse. Debug logs: none added. Scratch sandboxes removed.

### 2.3 Source changes outside the generated crates (files edited by me)

- `🧬️schema/🧬️mutations/🦀️.rs` of mesh, drawing, brep: `use super::<leaf>;` -> `use crate::schema::mutations::<leaf>;`.
- model: 42 files under `🧬️schema/🧬️mutations` (12 leaves, 9 diff, 9 inverse, 12 inverts tests) via `r2-glob-leaves.ts`; helpers `relative_placement_diff`, `relative_placement_inverse`, `number_label`, `element_count_label`, `vector_label` widened to `pub`.
- top-level `pub(crate) fn` -> `pub fn` in `🧬️schema/{🔺️diff,🧬️mutations,📸️snapshot}/🦀️.rs` of mesh (diff, mutations), drawing, brep, model; brep `🚪️io/{💾️binary,📝️text,🪶️sqlite}` `pub(in crate::io)` and `pub(crate)` items -> `pub`.
- `🧰️framework/.../📏️lib-wiring-ratchet/🔣️.json`: removed pins of mesh, drawing, brep roots.
- Generator library: no edit by me (the planner, `📝️render`, `🧬️mutation-families`, `🌳️module-tree`, `🧪️test-closure`, `🧫️sandbox` were edited by other agents during this run; see 2.4).

### 2.4 Findings, risks, open items

- Planner drift under me: between the model verify (exit 0) and the model apply, another agent changed `🗺️plan/🟦️.ts` (`R8B`/`R5C` debug switches, `lift`, mutation `support` module). The unchanged tree then planned as one 80-file `model-io` crate (`io <-> schema::snapshot` visibility coupling through three `pub(crate)` demo fns that I had widened for mesh/drawing/brep but not yet for model). Widening them restored a clean plan; model was applied from that plan and passed `--check-applied`. Because apply replans, the exact plan applied is the one in `r2-model-apply.log`.
- Crate naming defect (generator, not fixed): the model mutation-leaf family crate is called `semio-s-artifact-stdio-semio-model-insert-element` (named after its first leaf) and drawing has `-create-node` and `-change-stroke-width`; mesh and brep got `-mut-family`. The names are consistent in Cargo, owner rows and lock, but they read as one leaf. A later rename needs one coordinated rewrite (Cargo.toml `name`, dependents, owners, lock).
- Not compiled: `COMPILE ADMITTED` is not in `📓️FLEET.md`, so no `bun repo cargo` run. Static evidence only (rustc analysis oracle and layout differential, both clean). Open risk shared with every carve: items gated `#[cfg(test)] pub fn` (`fixture`, `demo_mutation_cases`, `demo_diff_cases`, `demo_*_snapshot`) are visible to tests in other crates of the union check but not in a real multi-crate test build; the first real `cargo test -p` of the new `-io`/`-mutations` crates will show it. Run list for the compile queue: `bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --tests --message-format=short` for each crate in 2.1.
- The `lift`/`support` planner features and R8B debug switches seen in `🗺️plan/🟦️.ts` belong to other agents; their `[DEBUG]`-marked code is theirs to remove.
