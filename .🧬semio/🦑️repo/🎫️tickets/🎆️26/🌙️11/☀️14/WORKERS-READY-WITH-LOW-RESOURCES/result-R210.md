# R210: graph questions read what they ask about (living report, 2026-11-18)

## 1. Inventory (read from the dispatch code, nothing run)
Dispatch: `buildInvocationGraphSelectionV1(args)` (`🕸️graph/🚪️admission/🟦️.ts`) names the scope; `loadBuildGraphOnceV1` (`🕸️graph/📥️load/🟦️.ts`): `all` goes to `loadGenerationGraphV1` (the generation of the whole workspace), every other scope is a selected read over its owner closure (`baseline:true`, selected generation exchange of R171).

| Question | Selection before | Read before | Why / after |
|---|---|---|---|
| `show project p` | `all` + option `closure:[p]` | WHOLE: `refreshClosure` needs a whole generation; with none it builds all 4785 owners (R209) | now `owners{p}`: selected read of p's closure (53 owners for repo-mcp) |
| `show projects`, `show targets` | `all`, `lean` | WHOLE; CLI default policy `fresh`; policy `current` still built when no generation existed | now default `current`: sealed answer with age, or typed refusal |
| `graph` | `all` | WHOLE; same | same as above. The catalogue has NO `--focus` option (synopsis is `graph`); the MCP `graph` tool takes none either |
| `affected` | `all` | WHOLE: changed files from git, then `affectedBuildProjectsV1(graph,files)` over the full edge list | see plan (section 4) |
| `run p:t`, MCP `run`, MCP `test` (= run `<p>:test`) | `tasks` | SELECTED (via resident host `loadSlice`) | already right |
| `run-many` literal `--projects`+literal `--targets` (<=64, no pattern chars) | `owners`+targets | SELECTED | already right |
| `run-many` with patterns, `--all`, no `--projects` | `all` | WHOLE | needs the roster of all owners; inherent |
| `exec --projects` literal | `owners` | SELECTED | already right |
| `watch` | `all` | WHOLE | not an MCP tool |
| `inputs p:t` | own selected-inputs module | SELECTED | already right |
| MCP `show_projects`, `show_project`, `show_targets`, `graph` | surface defaults `--graph-policy current` | same code as the CLI; the promise "never wait for a rebuild" was FALSE without a generation: `refresh` with policy `current` and no usable generation falls through to the builder | now true (section 3) |
| MCP `affected`, `run_many` | as CLI | as CLI | |
The CLI has no `test` operation in the catalogue (`repo test` is the MCP name for run `<p>:test`).

## 2. Implemented: `show project p` is a selected read
`buildInvocationGraphSelectionV1(["show","project",p])` = `{kind:"owners",projects:[p],exclude:[]}` (literal names only; a pattern or a missing name stays `all`), the same scope key as `exec --projects=p`, so the selected generation exchange (one machine reads, the others fetch) serves it. The runtime (`⌨️command/⚙️runtime`) takes the selection from that function, checks the name against the owner registry first (`Unknown build project: p` without any read), and keeps the old `closure` option only for the non-literal fallback.

## 3. Implemented: whole questions answer from the sealed generation or refuse
- Engine (`🕰️generations/🔌️load/🟨️.mjs`, `loadGenerationGraphV1`): policy `current` on a whole read never enters the builder custody: no usable sealed generation -> error code `GENERATION_NONE_SEALED` with the typed text `neverSealed` (EN/DE, in the message table and its schema; the existing equality law keeps both equal). The text says: this machine holds no sealed whole generation, a question about every project never starts the inference, how far the whole read of THIS machine is (`host.wholeReadProgress`: holder of the builder custody, phase done/total and held time from the lease table, read-only, 5 s bound), whether the fleet cache holds a sealed generation (`exchange.ahead`, one short request), and the two ways out (`repo show project <name>`; `repo graph --graph-policy fresh` on purpose).
- With a sealed generation: served without verification or inference and says `sealedAge` (EN/DE): generation, project count, age (`generationAgeTextV1`: `3 d 4 h 12 min`; age = mtime of the manifest file, the moment it was published).
- CLI: a whole question without `--graph-policy` now defaults to `current` (was `fresh`); `--graph-policy fresh` is the explicit opt-in to hours of inference.
- NOT done: the refusal cannot say how far the FLEET's whole read is (only this machine's lease table is read cheaply); a fleet-wide progress record would need the fleet status exchange. NOT done: fetching the fleet's whole generation inside a `current` question (restore writes the store and needs the builder custody, which the interactive command must not wait for): the text tells the user the fleet holds one, the whole read fetches it first.

## 4. Plan for questions that need a reverse index (not implemented)
- What is cheap and exact: the owner registry (`readBuildOwnerRegistrySourceV1`: owners, roots, native roots) maps changed files to owners without inference; `graph --focus p` (closure of p, forward edges) is the selected read of `owners{p}`.
- What is not cheap: dependents of p. The edges are INFERRED per owner (source imports, tool inputs); a declared-only scan (project.json implicit/dependsOn, package.json, Cargo.toml) misses inferred edges and would under-approximate.
- Design for `affected`: (1) changed files -> owners by registry (exact, seconds); (2) dependents from the reverse edge index of the last sealed whole generation (`policy current`, age stated, per-owner fragments hold the edges: `engine.readGlobal`/`readFragment`) by a fixpoint over `{source,target}` rows without assembling projects; (3) with no sealed whole generation: the same typed refusal, plus an explicit `--declared-only` lower bound built from a new per-owner declared-edge reader, labelled as a lower bound in the answer; (4) owners changed since the sealed generation (membership journal) are re-inferred through the owner-record exchange (R156), which are few, instead of the whole workspace. Open: the per-owner declared-edge reader and a published reverse index in the generation exchange.

## 5. Laws
- `🕸️graph/🚪️admission/🧪️tests/🔎️show-project-scope` (3): show project plans scope {p} (same key as exec); whole questions and non-literal names stay `all`; the runtime source takes selection and default policy from the functions (source law).
- `🕰️generations/🔌️load/🧪️tests/🔒️current-never-builds` (4): no generation + `current` refuses with the typed text in EN and DE, names 89 of 4785, no custody event, no inference; a sealed generation (made stale by an edit) answers with no inference, no custody, age line EN/DE, same project set as the Python model; closure reads keep their way; age/progress wording. Red on the base: import fails (new exports), and the old behaviour built.
- Changed existing law: `🔌️load/🧪️tests` "a current policy ... without one it builds" now expects the refusal; three said-line expectations ignore the extra `sealedAge` line.
- Staged battery (preload map to staged files, transpiler cache off): load tests 100 pass 1 fail (the same baseline red as on the base: "progress counters..."), admission 7/2 (base 7/2), generations 30/0, slice-share 6/0, load subdirs equal to base.

## 6. Landing and verification
- Landing 1: 15:00:22-15:02:01 UTC (landings.md "R210 landing"). Staged and live batteries equal to base (baseline rereds unchanged).
- First real `repo show project @semio-tech/repo-mcp-rs` (15:04-15:07): selected read, NO whole read (2.5 min), then died on a second closure-proof gap: `np/owner-peaks.json` unknown as a closure role (same class as R209's row log). Landing 2 (15:30:05, 3 s): `spill()` knows it; law `closures/tests/owner-peaks-spill` (3).
- Second attempt 15:32:50 hit a transient `Export named 'machineLocalRowV1' not found` from the pinned source snapshot (R208's landing of 15:00 moving the observation module under a running pin; gone on retry, not mine, not reproduced).
- Third run 15:33:03-15:33:48: `repo show project @semio-tech/repo-mcp-rs --json` = 45 s, exit 0, 33.9 KB project JSON, selected read of 1 owner (this project's closure is 1 owner; the 53-owner closure of R136/R141 belongs to another project), published to the fleet (selected-exchange `published:true`). Load 31.
- `repo show projects` (German, no sealed whole generation on this machine): 6 s, exit 1, typed refusal text in German (no inference, no custody). The generic wrapper prefixes it with "Interner Fehler"/"Internal error"; a command-level presentation of `GENERATION_NONE_SEALED` as a plain refusal (exit code, no prefix) is open.
- NOT verified: a whole question WITH a sealed generation on a real tree (only the model/mock law); the MCP tools end to end (same code path, not run); the 53-owner project.
