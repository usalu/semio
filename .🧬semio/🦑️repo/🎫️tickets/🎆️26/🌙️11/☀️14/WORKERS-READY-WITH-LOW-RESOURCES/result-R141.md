# R141: a small selection has a small closure (living report)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R141/` (`probe.ts` = R140 `probe5.ts` with per-observation file lists, `prof.sh`, `restore.py`).

## 1. Cause (measured with the in-process probe, load 40 to 60)

`norm-din4108` alone admitted **4058 of 4772 owners** (the three readiness probes: 4407). Two independent mechanisms, both "a file of another owner that is only read as an input fact admits the owner that holds it" (`closeBuildOwnerAdmissionV1`: every `value.files` entry admits `sourceOwner(file)`):

1. The catalog consumer rule: `catalogNativeManifestCandidatesV1` adds the Cargo manifest of every catalog crate (3227 files, 3226 owners) to the files of a consumer under a `consumerRoots` entry of the catalog provider.
2. Input globs: the catalog producer `@semio-tech/s-dev-composition` (a real dependency edge of every plugin, one owner) is observed with its `production` named input (`{workspaceRoot}/✏️s/**/*`): 81156 matched files in 3488 owners (74659 in the workspace owner), each admitting its owner.

Not inflating (checked): the provider block of the closure (`catalogProviders` with `consumerRoots`) admits exactly one owner (the owner of the provider source); native dependency roots, imports and generated producers add real `dependencies` edges.

## 2. Fix (schema first)

- `🧬️schema/🚪️admission.json`: `ownerFacts.observedFiles` (file facts of foreign owners an owner only reads: members, content observed, invalidating, never admitting) and admission `observedFiles` (the members of `files` that admit no owner). `files` stays the complete membership (membership digest, hash inputs, invalidation); a real dependency edge or an entry of `files` admits.
- `closeBuildOwnerAdmissionV1`: `observedFiles` are added to `files`, not resolved to owners; a file that is also an admitting file of another owner stays admitting and is not reported as observed.
- Owner discovery: the catalog manifests and the matched input-glob files are `observedFiles`.
- Graph read (`🧭️discovery`, `📦️project-nodes`): project and Cargo manifests listed as observed make no project node; the plugin catalog, the session/preparation targets and the component roots are still derived from the admitted manifests plus the observed ones (`catalogFiles`), so the nodes of the selected owners are the ones a full-closure read computes.
- Left as it was: files of command sources, imports and compile closures still admit the owner that holds them (53 owners for `repo-mcp`), since those are executable inputs of the task.

## 3. Proof

Tests (new, `🕸️graph/🚪️admission/🧪️tests/🔭️observed-files`, 4 pass; red before: schema refused the field): 200 seeded worlds against an independent fixpoint, observed file also admitting, dependency edge wins, 200 foreign manifests give a one-owner closure. `🧪️tests/🕸️graph/🛂️admission/🔭️observed-catalog` (2 pass): real owner discovery on a synthetic workspace with a catalog provider, 30 crates and a foreign glob: closure `app`, `catalog-owner`; editing one crate manifest changes its observation row (invalidation law); 120 seeded splits: catalog and component roots from admitted plus observed manifests equal those from all manifests, and admitted-only differs in more than 100 splits (why `catalogFiles` is needed).
Existing `🛂️admission` file: 19 pass, 1 fail (`selected Source discovery yields original control`: "Original resource loan deadline exhausted" on the loaded machine, a cancellation timing law, no catalog involved).

Closure sizes (probe): `norm-din4108` 4058 -> **54** owners, wall 88 s -> 37 s; the three probes 4407 -> **57** owners (`repo-mcp` 53 of them), wall 226 s (R140 before) / 115 s (R140 after) -> 94 s, 66 to 102 owner observations instead of 4070.

## 4. Landing

Entry in `landings.md` ("R141 landing", 20:34 UTC). Deviation disclosed there: the source edits were live before the lock (probe and lab read the live tree); the lock covered the checks. Restore: `restore.py` (tested on a scratch copy).

## 5. Slice lab, `--slice=@semio-tech/norm-din4108` (16384 MB / 8 cores, load 47, 557 s, no verdict)

- The read admitted **54 owners** (was 4058 for this slice; the lab of R140 run 5 admitted 4455 for the probes). 0 owners inferred twice, peak footprint 3165 MB, 0 leftover processes.
- Phases: 130 s with no phase at all (stage `workspace` started at 2 s, phase `start` at 133 s), `start` 133 to 265 s (178 s of start in the verdict: source admission 64, implementation source 18, tools 86 (that stage is replaced by `repo --help` in the lab but its activity second still counts), command-source imports 56, selected source inputs 40, selected declaration 24), then the take pass: 49 of 54 owners taken at 291 s (26 s). Target of a minute for start plus take: **not met on this loaded machine**.
- Walls found:
  - **W-a (new): 130 s before the `start` phase begins** with the machine at load 47 (no phase, no activity in the timeline until 133 s). Not analysed: suspects are the service start plus capacity admission of the first read.
  - **W-b (R140 wall 18 again): a second pass.** After `taken 49 of 54` at 291 s the timeline falls back to `taken 33 of 54` (344 s), then `start` again (395 s) and a second `taken` (448 s onwards, 49, 30, 30 ...) until the timeout; the take pass of the first round did not reach 54 of 54 either (the last owners wait). Same shape as R140 run 5; not touched here (R140 owns it).
  - **W-c (remaining closure cost): the catalog producer scan.** The 54 owners still pay about 400000 observation rows (probe: 104 MB), because the producer `s-dev-composition` (the single catalog owner, a real edge of every plugin) is asked for its `production` input (`{workspaceRoot}/✏️s/**/*`, 81156 matched files) through `^production`. Narrowing that request to the provider's declared digest input would cut the closure scan (about 20 of the 37 s of the probe) but changes what a consumer's `^production` hash covers; needs a declared catalog digest input and a decision, so it is NOT done here.
- Not exercised: assembly/seal/publish of a slice generation, target runs.

## 6. Unverified

- The graph read's node equality against a full-closure read is proven only for the pure parts (catalog and component roots derived from admitted plus observed manifests, 120 seeded splits) and the closure; a real `discoverBuildGraphV1` of a synthetic workspace needs native caller capabilities of a checkout and was not run (it would touch the live state).
- The three-probe lab (`repo-mcp`, `norm-din4108-rs`) was not run; `norm-din4108-rs` still fails in the in-process probe on "graph writer custody" (probe limitation).
- The main `🚪️admission/🧪️tests/🟦️.ts` file needs a script-process invocation and was not run; `🛂️admission` main file: 19 pass, 1 timing fail.
