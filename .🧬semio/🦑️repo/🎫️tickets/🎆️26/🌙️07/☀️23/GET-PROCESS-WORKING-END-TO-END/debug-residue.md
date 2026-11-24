# Debug Residue Audit

Read-only scan of `🧰️framework` and `✏️s` (.rs, .ts, .tsx). Excluded: `.cache`, `node_modules`, `target`, `🗑️generated`.

Patterns: `[DEBUG]`, `eprintln!`, `console.log`, `debug_tag`, `crate_dbg`, `recv_dbg`. `debug_tag`, `crate_dbg` and `recv_dbg` have zero hits.

## Summary

- Files with hits: 1324
- Total hits (one per line): 2915
- Hits carrying `[DEBUG]`: 77
- Hits without `[DEBUG]`: 2838
- Per pattern (a line can match more than one): `[DEBUG]` 77, `console.log` 1976, `eprintln!` 901

## Method and limits

- Allocation column is a per-line heuristic only: it flags `format!`, `println!`, `Backtrace`, `Mutex`/`RwLock`, `static`/`OnceLock`/`LazyLock`, `lock(`, `to_string(`, `clone(`, `vec!`, `collect`, `join(`, `JSON.stringify`, template literals `${`, `.map(`/`.filter(`, `new Map/Set/Date`, `serde`, `json!`. It does not prove the line sits inside a test-observed path; a test-path flag is given from the file path only.
- Test path flag: `yes` when the file path looks like a test (`/tests/`, `.test.`, `.spec.`, `_test.rs`, `tests.rs`, `test-hosts`). `no` otherwise.
- Hits without `[DEBUG]` are not automatically temporary: many `eprintln!`/`console.log` calls are CLI output or test harness output. They need a per-file review before removal.

## Hits by file

### `✏️s/🔌️plugins/✒️writer/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/✒️writer/🏭️bridge/🦀️.rs:44` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/✒️writer/🏭️bridge/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/📜️script.ts` (1)

- `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/📜️script.ts:12` [console.log] test-path: no; allocates: no
  - `console.log("writer ts ok"); }`

### `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🔬️window-state/🦀️.rs` (1)

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🔬️window-state/🦀️.rs:148` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Writer exact-window runtime failure before close: {error}");`

### `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🧪️tests/🔬️typescript-package-commands/🟦️.ts:23` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[writer-equation-gis] checked published consumers=${files.length}');`

### `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/🦀️.rs:44` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🦀️.rs` (1)

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🦀️.rs:107` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Equation exact-window runtime failure before close: {error}");`

### `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🟦️.ts` (1)

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🟦️.ts:54` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${owned.length} fixture manifest(s) refreshed in ${CATALOG}');`

### `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📖️reader/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.json> | reader compare <expected.json> <actual.json>");`

### `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/🦀️.rs:58` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/🦀️.rs:63` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏭️bridge/🦀️.rs:23` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mutation-inventory] units={} bytes={}", progress.completed_units, progress.emitted_bytes);`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏭️bridge/🦀️.rs:35` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error) = run() { eprintln!("[mutation-inventory] {error}"); std::process::exit(1); }`

### `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏭️bridge/🦀️.rs:19` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mutation-inventory] units={} bytes={}", progress.completed_units, progress.emitted_bytes);`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏭️bridge/🦀️.rs:31` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error) = run() { eprintln!("[mutation-inventory] {error}"); std::process::exit(1); }`

### `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (1)

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:21` [console.log] test-path: yes; allocates: no
  - `test("unlinked public artifact resolves its own exports in independent Bun runtime",async()=>{const root=fileURLToPath(new URL("../../../../../../../../📦️packages/🟦️typescript",import.meta.url));const code="const arti...`

### `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏭️bridge/🦀️.rs:21` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mutation-inventory] units={} bytes={}", progress.completed_units, progress.emitted_bytes);`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏭️bridge/🦀️.rs:33` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error) = run() { eprintln!("[mutation-inventory] {error}"); std::process::exit(1); }`

### `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏭️bridge/🦀️.rs:23` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mutation-inventory] units={} bytes={}", progress.completed_units, progress.emitted_bytes);`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏭️bridge/🦀️.rs:35` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error) = run() { eprintln!("[mutation-inventory] {error}"); std::process::exit(1); }`

### `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:184` [console.log] test-path: yes; allocates: yes: .map(
  - `let script="import Ajv from 'ajv';const x=JSON.parse(await Bun.stdin.text());const a=new Ajv({strict:true}).addKeyword('x-semio-ui');const c=x.schemas.map(s=>a.compile(s));for(const media of x.media){if(!c[0]({tile:{i...`

### `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏭️bridge/🦀️.rs:19` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mutation-inventory] units={} bytes={}", progress.completed_units, progress.emitted_bytes);`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏭️bridge/🦀️.rs:31` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error) = run() { eprintln!("[mutation-inventory] {error}"); std::process::exit(1); }`

### `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/🦀️.rs:54` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/🦀️.rs:59` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🪟️generation2d-window-camera-ownership/🦀️.rs` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🪟️generation2d-window-camera-ownership/🦀️.rs:329` [eprintln!] test-path: yes; allocates: yes: println!
  - `if let Err(error) = &outcome { eprintln!("[TRACE] Generation2d exact-window runtime failure before primary close: {error}"); }`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🟦️.ts:152` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('generation2d preview-eval-run entityKeys=${fixture.entityKeys.length} targets=${fixture.targets.rows.length} observations=${fixture.observations.length} scheduling=${fixture.scheduling.length} runEffects=...`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🧪️tests/🔬️typescript-package-commands/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[generation-source] checked published consumers=${files.length}');`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🧪️tests/🔬️contract/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🧪️tests/🔬️contract/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('generation3d-preview-window-transient accepted=${fixture.accepted.length} rejected=${fixture.rejected.length} configExcludesPreview=true owner=procedural-preview');`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🟦️.ts` (2)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🟦️.ts:33` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('generation3d contribution-gated arming blocked=${blocked?.owedHops} served=${served?.owedHops} resume=${fixture.resume.command}->${fixture.resume.carriedRunAction} budget=${fixture.tickStepBudget}');`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🟦️.ts:197` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('generation3d preview-eval-run entityKeys=${fixture.entityKeys.length} observations=${fixture.observations.length} scheduling=${fixture.scheduling.length} status=${fixture.status.length} runEffects=${fixtu...`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️play-app-and-retained-command-contracts/🦀️.rs` (2)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️play-app-and-retained-command-contracts/🦀️.rs:69` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] gen3d render body={body_key} generations={} selected={selected_generation_id:?}", document.generation.as_state().generations.len());`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️play-app-and-retained-command-contracts/🦀️.rs:306` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[work-frame-sizes] {frames:?}");`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️generate-interactions/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️generate-interactions/🟦️.ts:118` [console.log] test-path: yes; allocates: no
  - `console.log(`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mode-panels/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mode-panels/🟦️.ts:56` [console.log] test-path: yes; allocates: yes: ${, .join(, .map(
  - `console.log('generation3d mode-panel publication panels=${fixture.panels.length} cases=${fixture.cases.length} staleFocusCases=${stale.map((row) => row.id).join(",")}');`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️tick-addressing/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️tick-addressing/🟦️.ts:38` [console.log] test-path: yes; allocates: no
  - `console.log(`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs:14` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🕹️interaction-scope/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🕹️interaction-scope/🟦️.ts:105` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[SCOPE] ${role.id} hover ${hover.windows.length + hover.panels.length + hover.sections.length} sections vs full ${full.windows.length + full.panels.length + full.sections.length} + catalogue');`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔬️status-contract/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔬️status-contract/🟦️.ts:403` [console.log] test-path: yes; allocates: no
  - `console.log(`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🩹️gesture-rearm/🟦️.ts` (3)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🩹️gesture-rearm/🟦️.ts:227` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('generation3d gestureRearm ${row.id} owes=${gesture.owes} owed=${owedWindows.join(",") || "-"} hops=${hops.join(",") || "-"} carried=${gesture.effects.join(",") || "-"} starts=${starts}');`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🩹️gesture-rearm/🟦️.ts:245` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('generation3d jobSupersession ${row.id} owns=${owns} cancels=${cancels} clearsPort=${clearsPort}');`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🩹️gesture-rearm/🟦️.ts:258` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('generation3d documentMoves ${row.id} polls=${row.polls.length}');`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏱️tick-and-resolve/🦀️.rs` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏱️tick-and-resolve/🦀️.rs:385` [eprintln!] test-path: no; allocates: yes: println!
  - `if !payload.ok { eprintln!("flowTessellateCancelResolve: geometry extension refused the cancel for window {}", payload.window_id); return Vec::new(); }`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧊️preview-geometry/🦀️.rs` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧊️preview-geometry/🦀️.rs:38` [eprintln!] test-path: no; allocates: yes: println!
  - `/// label instead of a per-tick 'eprintln!' nobody reads.`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts` (5)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts:49` [console.log] test-path: no; allocates: yes: ${
  - `console.log('generation3d-mesh-selection checks=${generation3dMeshSelectionSelfTests()} independentAjv=true');`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts:50` [console.log] test-path: no; allocates: yes: ${
  - `console.log('generation3d-widget-creation checks=${generation3dWidgetCreationSelfTests()} independentAjv=true');`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts:51` [console.log] test-path: no; allocates: yes: ${
  - `console.log('generation3d-terminology checks=${generation3dTerminologySelfTests()}');`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts:59` [console.log] test-path: no; allocates: yes: ${
  - `console.log('generation3d-semantic-wire checks=${assertGeneration3dSemanticWire()} independentAjv=true');`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts:90` [console.log] test-path: no; allocates: yes: ${
  - `console.log('generation3d-io-authority checks=${testGeneration3dIoAuthorityFixture()}');`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/🧪️tests/🔬️retained-authority-laws/🦀️.rs` (1)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/🧪️tests/🔬️retained-authority-laws/🦀️.rs:296` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] owned={owned_bytes} aliased={aliased_bytes} T={} arc={} birth={}", std::mem::size_of::<Generation3dSnapshot>(), semio_framework_value::shared_retirement_allocation_bytes::<Generation3dSnapshot>(), s...`

### `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/📈️guest-memory/🦀️.rs` (6)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/📈️guest-memory/🦀️.rs:95` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("turn {turn} result: status={:?} effects={} patches={} presence={} receipt={:?}", result.status, result.effects.len(), result.ui_patches.len(), result.presence.len(), result.lifecycle_receipt.is_some());`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/📈️guest-memory/🦀️.rs:102` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{hot} of {turns} turns answered MoreWork, at {hot_turns:?}");`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/📈️guest-memory/🦀️.rs:145` [eprintln!] test-path: yes; allocates: yes: .map(, println!
  - `eprintln!("guest stderr: disarmed={:?} armed={} bytes", silent.as_ref().map(String::len), armed.as_deref().map(str::len).unwrap_or(0));`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/📈️guest-memory/🦀️.rs:147` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("guest said: {line}");`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/📈️guest-memory/🦀️.rs:203` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("guest linear memory: instantiate={instantiated} B, turn {settled_turn}={settled} B, turn {}={last} B, {per_turn:.1} B/turn", readings.len() - 1);`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/📈️guest-memory/🦀️.rs:205` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("turn {turn}: {reading} B");`

### `✏️s/🔌️plugins/🌊️flow/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🌊️flow/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🌊️flow/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/📜️script.ts` (4)

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/📜️script.ts:67` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Flow child identity native laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed');`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/📜️script.ts:86` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Flow typed child edit native law: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed');`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/📜️script.ts:184` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] Flow retained addWidget oracle: ${fixture.accepted.length} accepted + ${fixture.denials.length} hostile cases; session/publication model is independent; runtimeClaims=0');`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/📜️script.ts:191` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Flow retained addWidget native laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed');`

### `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📜️script.ts` (4)

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📜️script.ts:20` [console.log] test-path: no; allocates: yes: ${
  - `console.log('geometry-inference-oracle cases=${geometryInferenceOracle(import.meta.dir)}');`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📜️script.ts:23` [console.log] test-path: no; allocates: yes: ${
  - `console.log('brep-extension-retirement-oracle cases=${brepExtensionRetirementOracle(import.meta.dir)}');`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📜️script.ts:24` [console.log] test-path: no; allocates: yes: ${
  - `console.log('geometry-inference-oracle cases=${geometryInferenceOracle(import.meta.dir)}');`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📜️script.ts:25` [console.log] test-path: no; allocates: yes: ${
  - `console.log('channel-identity-oracle cases=${channelIdentityOracle(import.meta.dir)}');`

### `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🔬️unit/🦀️.rs:575` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("mesh fixture {}: area={}", case.get("name").unwrap().as_str().unwrap(), read_channel_number(&report, "area").unwrap());`

### `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️rust-package-commands/🟦️.ts` (5)

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️rust-package-commands/🟦️.ts:10` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('brep-extension-retirement-oracle cases=${brepExtensionRetirementOracle(context.root)}');`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️rust-package-commands/🟦️.ts:11` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('geometry-inference-oracle cases=${geometryInferenceOracle(context.root)}');`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️rust-package-commands/🟦️.ts:12` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('channel-identity-oracle cases=${channelIdentityOracle(context.root)}');`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️rust-package-commands/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: ${
  - `] }],progress(event) { console.log('brep-extension-retirement ${event.stage}: ${event.law ?? ""}'); }});`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️rust-package-commands/🟦️.ts:36` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('brep-extension-retirement receipts=${receipts.length}');`

### `✏️s/🔌️plugins/🌍️gis/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🌍️gis/🏭️bridge/🦀️.rs:48` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🌍️gis/🏭️bridge/🦀️.rs:53` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/📜️script.ts` (1)

- `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/📜️script.ts:12` [console.log] test-path: no; allocates: no
  - `console.log("gis ts ok"); }`

### `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧪️tests/🔬️contract/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧪️tests/🔬️contract/🟦️.ts:53` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('gis-terrain-window-config accepted=${fixture.accepted.length} rejected=${fixture.rejected.length} windows=${Object.keys(states).length} owner=${fixture.windowKindId}');`

### `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🔬️window/🦀️.rs` (1)

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🔬️window/🦀️.rs:236` [eprintln!] test-path: yes; allocates: yes: println!
  - `if let Err(error) = &outcome { eprintln!("[TRACE] GIS Map exact-window runtime failure before close: {error}"); }`

### `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts` (9)

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:421` [console.log] test-path: no; allocates: no
  - `console.log(`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:427` [console.log] test-path: no; allocates: no
  - `console.log("gis-map-inference-port-check: no WGPU map rendering, no external model provider and no two-user process journey is run or claimed here.");`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:487` [console.log] test-path: no; allocates: yes: ${
  - `console.log('gis-map-peer-rebootstrap-oracle: AJV=1 deep-equal=${observed.length} source-hostiles=${hostiles.length}');`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:547` [console.log] test-path: no; allocates: yes: ${
  - `console.log('mounted-gis-map-probe-oracle: AJV=1 deep-equal=1 source-hostiles=${hostiles.length}');`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:561` [console.log] test-path: no; allocates: yes: ${
  - `console.log('cold-document-pair-browser-check: peer-checks=${checks}');`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:572` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('gis-mcp-inference-bridge: ${JSON.stringify(report)}');`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:583` [console.log] test-path: no; allocates: yes: ${
  - `console.log('gis-map-tiles: laws=${await proveGisMapTilesV1(root)} passed');`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:586` [console.log] test-path: no; allocates: yes: ${
  - `await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1() },cwd: this.repoRoot,groups: [{ p...`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:599` [console.log] test-path: no; allocates: yes: ${
  - `try { console.log(await prefetchMapTiles({ repoRoot: this.repoRoot, signal: cancellation.signal, progress: (completed, total) => console.log('tiles ${completed}/${total}') })); }`

### `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🟦️.ts:206` [console.log] test-path: no; allocates: no
  - `log = (line) => console.log(line),`

### `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🔬️rust-package-commands/🟦️.ts:18` [console.log] test-path: yes; allocates: no
  - `await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 3_600_0...`

### `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🧩️map-create-region-group/🟦️.ts` (3)

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🧩️map-create-region-group/🟦️.ts:79` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('gis-map-create-region-group-check: checks=${14 + fixture.hostile.length + fixture.membershipCases.length} clean; atomic durable publication not claimed');`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🧩️map-create-region-group/🟦️.ts:92` [console.log] test-path: yes; allocates: yes: ${
  - `const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMillisec...`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🧩️map-create-region-group/🟦️.ts:93` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `for (const receipt of receipts) console.log('gis-map-create-region-group-receipt: ${JSON.stringify(receipt)}');`

### `✏️s/🔌️plugins/🌿️vcs/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🌿️vcs/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🌿️vcs/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🎞️animate/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🎞️animate/🏭️bridge/🦀️.rs:44` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🎞️animate/🏭️bridge/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🎥️shooting/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🎥️shooting/🏭️bridge/🦀️.rs:44` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🎥️shooting/🏭️bridge/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (1)

- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:126` [eprintln!] test-path: yes; allocates: yes: String::from, println!
  - `child.stdin.take().unwrap().write_all(bytes).unwrap();let output=child.wait_with_output().unwrap();let diagnostics=String::from_utf8(output.stderr).unwrap();assert!(output.status.success(),"{diagnostics}");eprintln!("...`

### `✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🧪️tests/🔬️typescript-package-commands/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[playground] checked published consumers=${files.length}');`

### `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/🦀️.rs:41` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts` (1)

- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts:17` [console.log] test-path: no; allocates: no
  - `console.log("Sequence artifact browser, protocol oracle and examples passed");`

### `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️carrier-contract/🦀️.rs` (2)

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️carrier-contract/🦀️.rs:79` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Artifact IO descriptor parity entries={}, registered={}", actual.len(), registered.len());`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️carrier-contract/🦀️.rs:107` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Artifact IO native reset and text route bytes={}, spr={}", effect_pack.len(), spr.len());`

### `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️descriptor-parity/🟦️.ts` (1)

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️descriptor-parity/🟦️.ts:45` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Artifact IO descriptor parity checks=${checks}, entries=${ioEntries.length}');`

### `✏️s/🔌️plugins/🏗️fem/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🏗️fem/🏭️bridge/🦀️.rs:48` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🏗️fem/🏭️bridge/🦀️.rs:53` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs:106` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📖️reader/🦀️.rs:50` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.json> | reader compare <expected.json> <actual.json>");`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs` (2)

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:335` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[DEBUG] fem3d results fields caption={caption:?} values={} current={}", values.len(), fields.current_revision());`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:337` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error) = &built { eprintln!("[DEBUG] fem3d results fields render error {error:?}"); }`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/⚙️mounted-job/🦀️.rs` (2)

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/⚙️mounted-job/🦀️.rs:20` [[DEBUG],eprintln!] test-path: no; allocates: yes: .filter(, println!
  - `let Some(state) = shell.as_mut().filter(|state| state.identity == self.identity) else { eprintln!("[DEBUG] fem3d mounted job {:#x} stale shell", self.identity.job); return Ok(JobStep::Failed(b"fem3d.visual-stale-shell...`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/⚙️mounted-job/🦀️.rs:30` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!
  - `if !matches!(&step, JobStep::Running(None)) { eprintln!("[DEBUG] fem3d mounted job {:#x} step {:?}", self.identity.job, match &step { JobStep::Done(_) => "done", JobStep::Failed(detail) => std::str::from_utf8(detail)....`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🧪️tests/🔬️unit/🦀️.rs` (2)

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🧪️tests/🔬️unit/🦀️.rs:152` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("numerical before={before:?}, after={after:?}, context {stage}, terminal={terminal}, elapsed_us={elapsed_us:?}, admission={admission:?}");`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🧪️tests/🔬️unit/🦀️.rs:173` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("FEM3D numerical terminal={terminal}, ready={ready}, static={has_static}, modal={has_modal}, closed={closed}, close_lane={}, close_started_jobs={}, turns={turns}/{maximum_turns}, progress={progress:?}, recor...`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🚀️session-api/🦀️.rs` (3)

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🚀️session-api/🦀️.rs:258` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!(`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🚀️session-api/🦀️.rs:364` [[DEBUG],eprintln!] test-path: no; allocates: yes: .map(, println!
  - `eprintln!("[DEBUG] fem3d result_stage binding={:?} window={} current={:?}", request_owner.map(|owner| owner.index()), authority.is_some(), request_owner.and_then(|owner| registry.current[owner.index()]).map(|current| ...`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🚀️session-api/🦀️.rs:372` [[DEBUG],eprintln!] test-path: no; allocates: yes: .map(, println!
  - `eprintln!("[DEBUG] fem3d result_stage slot={slot} shell={} current_job={:#x} state_job={:#x} done={} numerical_done={} stage={:?} shell_request={:?} owner={:?}", current.shell, current.identity.job, state.identity.job...`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs:106` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📖️reader/🦀️.rs:50` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.json> | reader compare <expected.json> <actual.json>");`

### `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs` (1)

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs:168` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("concrete-forest sls {} ux={:.6e} uy={:.6e} uz={:.6e}", row.node_id, row.values[0], row.values[1], row.values[2]);`

### `✏️s/🔌️plugins/🏙️bim/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🏙️bim/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🏙️bim/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧰️checks/🦀️.rs` (3)

- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧰️checks/🦀️.rs:75` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("schema validation skipped: no repository .venv with the jsonschema library");`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧰️checks/🦀️.rs:375` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("diagnostic {:?} {:?} {:?}", diagnostic.severity, diagnostic.code, diagnostic.elements);`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧰️checks/🦀️.rs:590` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("replay of {name}: {} mutations applied, create kinds still missing: {:?}", replayed.applied, replayed.seeded);`

### `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️export-bim-1-gltf/🟦️.ts` (2)

- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️export-bim-1-gltf/🟦️.ts:155` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('wrote the table of ${name}: ${table.nodes} nodes, ${table.triangles} triangles, ${Object.keys(table.volumes).length} volumes');`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️export-bim-1-gltf/🟦️.ts:159` [console.log] test-path: yes; allocates: yes: ${
  - `console.log(same ? 'check: oracle agrees on ${name} (three 0.182.0)' : '[FAIL] the committed table of ${name} differs from the measurement of the committed file');`

### `✏️s/🔌️plugins/🏛️architect/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🏛️architect/🏭️bridge/🦀️.rs:44` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🏛️architect/🏭️bridge/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/📜️script.ts` (1)

- `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/📜️script.ts:12` [console.log] test-path: no; allocates: no
  - `console.log("architect ts ok"); }`

### `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts` (1)

- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts:94` [console.log] test-path: yes; allocates: no
  - `console.log("architect-window-ownership ajv=valid json-patch=valid four-owner-isolation=valid reload=valid report-identity=valid");`

### `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️window-ownership/🦀️.rs` (1)

- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️window-ownership/🦀️.rs:436` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Architect exact-window runtime failure before close: {error}");`

### `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` (1)

- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:262` [console.log] test-path: no; allocates: yes: .map(, JSON.stringify
  - `let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));console.log(JSON.stringify({title:d.query('SELECT title FROM architect_meta').get().title,name...`

### `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (1)

- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:100` [console.log] test-path: yes; allocates: yes: .join(, .map(, JSON.stringify
  - `let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});let rows=0,bytes=0;const tables=d.query("SELECT name FROM sqlite_schema WH...`

### `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🧪️tests/🔬️rust-package-commands/🟦️.ts:74` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[register-catalog] ${stale.length ? 'wrote ${stale.length}' : "current"}');`

### `✏️s/🔌️plugins/🏭️process/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🏭️process/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🏭️process/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs` (6)

- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs:82` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("LOWPOLY_FOREST_MESH_JSON_START");`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs:83` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{json}");`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs:84` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("LOWPOLY_FOREST_MESH_JSON_END");`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs:86` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("LOWPOLY_FOREST_DSL_START");`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs:87` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{}", semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::component::text::snapshot::print_dsl(&snapshot));`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️export-concrete-forest-mesh/🦀️.rs:88` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("LOWPOLY_FOREST_DSL_END");`

### `✏️s/🔌️plugins/💠️lowpoly/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (2)

- `✏️s/🔌️plugins/💠️lowpoly/🧪️tests/🔬️typescript-package-commands/🟦️.ts:83` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('lowpoly interactive-job actual source actions agree with examples: ${fixture.routes.length} Migrated, 0 BatchOnlyPendingRewrite');`
- `✏️s/🔌️plugins/💠️lowpoly/🧪️tests/🔬️typescript-package-commands/🟦️.ts:97` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Lowpoly actual publication policies agree with Ajv: ${fixture.routes.length} source actions, four invalid domain policies');`

### `✏️s/🔌️plugins/💡️reasoning/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/💡️reasoning/🏭️bridge/🦀️.rs:44` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/💡️reasoning/🏭️bridge/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/📜️script.ts` (1)

- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/📜️script.ts:7` [console.log] test-path: no; allocates: no
  - `run(): void { console.log("[TRACE] reasoning ts ok"); }`

### `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🧪️tests/🔬️unit/🦀️.rs` (5)

- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🧪️tests/🔬️unit/🦀️.rs:111` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Wires pointer move runtime failure before close: {error}");`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🧪️tests/🔬️unit/🦀️.rs:188` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Wires document replacement runtime failure before close: {error}");`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🧪️tests/🔬️unit/🦀️.rs:233` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Wires cancellation runtime failure before close: {error}");`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🧪️tests/🔬️unit/🦀️.rs:314` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Wires window transient runtime failure before close: {error}");`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🧪️tests/🔬️unit/🦀️.rs:404` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Wires batched gesture runtime failure before close: {error}");`

### `✏️s/🔌️plugins/📋️forms/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/📋️forms/🏭️bridge/🦀️.rs:42` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/📋️forms/🏭️bridge/🦀️.rs:47` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧪️tests/🔬️window/🟦️.ts` (1)

- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧪️tests/🔬️window/🟦️.ts:75` [console.log] test-path: yes; allocates: no
  - `console.log("forms-try-window-ownership ajv=valid json-patch=valid two-window-isolation=valid reload=valid reset=valid");`

### `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧪️tests/🔬️window/🦀️.rs` (1)

- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧪️tests/🔬️window/🦀️.rs:184` [eprintln!] test-path: yes; allocates: yes: println!
  - `if let Err(error) = &outcome { eprintln!("[TRACE] Forms Try exact-window runtime failure before close: {error}"); }`

### `✏️s/🔌️plugins/📏️layout/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/📏️layout/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/📏️layout/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/⚙️layout-engine-and-views/🦀️.rs` (1)

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/⚙️layout-engine-and-views/🦀️.rs:37` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] layout engine built-in text fonts unavailable: {error}; stories shape with the layout font alone");`

### `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config/🧪️tests/🔬️window/🦀️.rs` (1)

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config/🧪️tests/🔬️window/🦀️.rs:109` [eprintln!] test-path: yes; allocates: yes: println!
  - `if let Err(error) = &outcome { eprintln!("[TRACE] Layout exact-window runtime failure before close: {error}"); }`

### `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️layout-play-app/🦀️.rs` (1)

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️layout-play-app/🦀️.rs:30` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("[ERROR] layout: the host's built-in text fonts were refused: {error}"),`

### `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:77` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{text}");`

### `✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/🟦️.ts:38` [console.log] test-path: yes; allocates: no
  - `console.log("layout ts ok");`

### `✏️s/🔌️plugins/📐️cad/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/📐️cad/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/📐️cad/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🧪️tests/🔬️window/🦀️.rs` (1)

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🧪️tests/🔬️window/🦀️.rs:262` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("CAD exact-window runtime failure before close: {error}");`

### `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🧪️tests/🔬️rust-package-commands/🟦️.ts:41` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('cad-window-transient-contract rows=${testCadWorldWindowTransientContract()}');`

### `✏️s/🔌️plugins/📕️norm/⚖️compliance/⚡️incremental-evaluation/🧪️tests/⏱️latency/🦀️.rs` (5)

- `✏️s/🔌️plugins/📕️norm/⚖️compliance/⚡️incremental-evaluation/🧪️tests/⏱️latency/🦀️.rs:16` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] one-entity-commit/cold p50_ms={cold:.3} p95_ms={cold:.3} budget_ms=1000.0");`
- `✏️s/🔌️plugins/📕️norm/⚖️compliance/⚡️incremental-evaluation/🧪️tests/⏱️latency/🦀️.rs:30` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] one-entity-commit/commit p50_ms={p50:.3} p95_ms={p95:.3} budget_ms=400.0");`
- `✏️s/🔌️plugins/📕️norm/⚖️compliance/⚡️incremental-evaluation/🧪️tests/⏱️latency/🦀️.rs:49` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] settled-paint/paint p50_ms={:.3} p95_ms={p95:.3} budget_ms={budget:.1}", samples[50]);`
- `✏️s/🔌️plugins/📕️norm/⚖️compliance/⚡️incremental-evaluation/🧪️tests/⏱️latency/🦀️.rs:69` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] settled-report-read-1000-members/paint p50_ms={:.3} p95_ms={p95:.3} budget_ms={budget:.1}", samples[50]);`
- `✏️s/🔌️plugins/📕️norm/⚖️compliance/⚡️incremental-evaluation/🧪️tests/⏱️latency/🦀️.rs:99` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] shared-value-tree-1000-members/paint p50_ms={:.3} p95_ms={p95:.3} budget_ms={budget:.1}", samples[50]);`

### `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🎯️projection/🛰️live-scene/🧪️tests/🦀️.rs` (3)

- `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🎯️projection/🛰️live-scene/🧪️tests/🦀️.rs:87` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] live-scene-1000-members/cold p50_ms={cold:.3} p95_ms={cold:.3} budget_ms={cold_budget:.1}");`
- `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🎯️projection/🛰️live-scene/🧪️tests/🦀️.rs:100` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] live-scene-1000-members/drag p50_ms={:.3} p95_ms={p95:.3} budget_ms={budget:.1}", samples[samples.len() / 2]);`
- `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🎯️projection/🛰️live-scene/🧪️tests/🦀️.rs:117` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] diagram-memo/{name} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={:.1}", release_ms * scale);`

### `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/🧪️tests/🔬️unit/🦀️.rs:323` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] revision-stamp-20000-rows/commit p50_ms={p50:.1} p95_ms={p95:.1} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/📕️norm/🏭️bridge/🦀️.rs:68` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/📕️norm/🏭️bridge/🦀️.rs:73` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs` (6)

- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs:140` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {variant}/cold-drag p50_ms={cold_drag:.3} p95_ms={cold_drag:.3} budget_ms={COLD_MS:.1}");`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs:141` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {variant}/cold-evaluation p50_ms={cold_evaluation:.3} p95_ms={cold_evaluation:.3} budget_ms={COLD_MS:.1}");`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs:142` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {}/drag p50_ms={:.3} p95_ms={:.3} budget_ms={DRAG_P95_MS:.1}", self.variant, self.drag_preview.p50, self.drag_preview.p95);`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs:143` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {}/commit p50_ms={commit_p50:.3} p95_ms={:.3} budget_ms={COMMIT_P95_MS:.1}", self.variant, self.commit_total_p95());`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs:144` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {}/touch-drag p50_ms={:.3} p95_ms={:.3} budget_ms={:.1}", self.variant, self.touch_drag.p50, self.touch_drag.p95, touch_drag_budget_ms());`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs:145` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {}/touch-nudge p50_ms={:.3} p95_ms={:.3} budget_ms={COMMIT_P95_MS:.1}", self.variant, self.touch_nudge.p50, self.touch_nudge.p95);`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs:129` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs:586` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] en1990/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 2000.0);`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` (2)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:1467` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] en1990-member-row-unit-1000-members/commit p50_ms={p50:.1} p95_ms={p95:.1} budget_ms={budget:.1}");`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:1533` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] en1990-gate-1000-rows-1000-members/cold p50_ms={total:.1} p95_ms={total:.1} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪parse-en1990-artifact.bun.ts` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪parse-en1990-artifact.bun.ts:51` [console.log] test-path: no; allocates: no
  - `console.log("parse-en1990-artifact:ok");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧮️physical/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧮️physical/🦀️.rs:55` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `let mut child=std::process::Command::new("repo").args(["tools","exec","bun","--","-e",r#"import{Database}from'bun:sqlite';import Ajv from'ajv/dist/2020.js';const v=JSON.parse(await Bun.stdin.text());if(!new Ajv({stric...`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs:159` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs:268` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] en1997/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 20000.0);`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:83` [eprintln!] test-path: yes; allocates: yes: String::from, println!
  - `child.stdin.take().unwrap().write_all(bytes).unwrap();let output=child.wait_with_output().unwrap();let diagnostics=String::from_utf8(output.stderr).unwrap();assert!(output.status.success(),"{diagnostics}");eprintln!("...`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs:369` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs:804` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] en1991/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 1000.0);`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:299` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[latency] en1992-member-checks/{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget_ms:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs:154` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:34` [console.log] test-path: yes; allocates: yes: .join(, .map(, JSON.stringify, String::from
  - `let mut child=Command::new("bun").arg("-e").arg(r####"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{if(db.query('PRAGMA integrity...`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs:147` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs:325` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] en1993/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 2000.0);`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🕸️frame-envelope/🦀️.rs` (3)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🕸️frame-envelope/🦀️.rs:485` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[latency] frame-envelope/cold p50_ms={cold:.3} p95_ms={cold:.3} budget_ms={:.1}", 1000.0 * scale);`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🕸️frame-envelope/🦀️.rs:500` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[latency] frame-envelope/commit p50_ms={solve_p50:.3} p95_ms={solve:.3} budget_ms={:.1}", 400.0 * scale);`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🕸️frame-envelope/🦀️.rs:503` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[latency] frame-envelope/repaint p50_ms={repeat_p50:.3} p95_ms={repeat:.3} budget_ms={:.1}", 12.0 * scale);`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs:147` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs:143` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs:498` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] en1996/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 4000.0);`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs:143` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs:530` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] en1995/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 2000.0);`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs` (2)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:32` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("wrote {} ({} bytes)", pack_path.display(), bytes.len());`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:36` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("wrote {} ({} bytes)", dsl_path.display(), dsl.len());`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs:153` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs:456` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] en1999/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, limit as f64);`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/📚️assets/🦀️.rs` (2)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/📚️assets/🦀️.rs:186` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] regenerated {id}");`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/📚️assets/🦀️.rs:195` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] regenerated pack {id}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs` (4)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs:59` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] {:?} len {} {}", unit.id, a.len(), b.len());`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs:62` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] differs {x} -> {y}");`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs:66` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] only before {x}");`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs:69` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] only after {y}");`

### `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️live/🦀️.rs:147` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📕️norm/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/📕️norm/🧪️tests/🔬️typescript-package-commands/🟦️.ts:91` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('norm retained cohort ok: ${fixture.apps.length} editors × ${fixture.routes.length} retained routes, ${classified} descriptor rows carried a classification');`

### `✏️s/🔌️plugins/📕️norm/🧮️mechanics/🧪️tests/⏱️latency/🦀️.rs` (1)

- `✏️s/🔌️plugins/📕️norm/🧮️mechanics/🧪️tests/⏱️latency/🦀️.rs:30` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[latency] {name}/{scenario} p50_ms={p50:.3} p95_ms={measured:.3} budget_ms={budget:.1}");`

### `✏️s/🔌️plugins/📖️playbook/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/📖️playbook/🏭️bridge/🦀️.rs:42` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/📖️playbook/🏭️bridge/🦀️.rs:47` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/📜️script.ts` (1)

- `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/📜️script.ts:7` [console.log] test-path: no; allocates: no
  - `run(): void { console.log("[TRACE] playbook ts ok"); }`

### `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️rust-package-commands/🟦️.ts` (2)

- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️rust-package-commands/🟦️.ts:31` [console.log] test-path: yes; allocates: yes: ${
  - `] }],progress(event) { console.log('playbook-geometry-contract ${event.stage}: ${event.law ?? ""}'); }});`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️rust-package-commands/🟦️.ts:32` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('playbook-geometry-contract receipts=${receipts.length}');`

### `✏️s/🔌️plugins/📜️imperative/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/📜️imperative/🏭️bridge/🦀️.rs:42` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/📜️imperative/🏭️bridge/🦀️.rs:47` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/📜️script.ts` (1)

- `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/📜️script.ts:7` [console.log] test-path: no; allocates: no
  - `run(): void { console.log("[TRACE] imperative ts ok"); }`

### `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (1)

- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:63` [console.log] test-path: yes; allocates: yes: .join(, .map(, JSON.stringify, String::from
  - `let mut child=Command::new("bun").arg("-e").arg(r####"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{if(db.query('PRAGMA integrity...`

### `✏️s/🔌️plugins/📸️remodel/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/📸️remodel/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/📸️remodel/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs` (9)

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs:121` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[RETAIN] params voxel={} truncation={} dense={:?} sources={} texture={}", params.tsdf_voxel_size, params.tsdf_truncation, params.dense, params.dense_source_views, params.texture_enabled);`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs:140` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{}", retention(&engine).line("pushed"));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs:151` [eprintln!] test-path: yes; allocates: yes: format!, println!
  - `eprintln!("{}", snapshot.line(&format!("enter {:?}", engine.stage())));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs:157` [eprintln!] test-path: yes; allocates: yes: format!, println!
  - `eprintln!("{}", snapshot.line(&format!("{stage:?} slot {dense_slot}")));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs:159` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[RETAIN] tsdf blocks={} bytes={}", volume.block_count(), volume.retained_bytes());`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs:163` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{}", snapshot.line("TSDF CEILING — probe stops"));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs:169` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{}", retention(&engine).line("done"));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs:173` [eprintln!] test-path: yes; allocates: yes: format!, println!
  - `eprintln!("{}", retention(&engine).line(&format!("failed: {message}")));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️retention/🦀️.rs:178` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[RETAIN] PEAK {:.2} MiB at {peak_label}", peak as f64 / (1024.0 * 1024.0));`

### `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/♻️dense-replay/🦀️.rs` (3)

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/♻️dense-replay/🦀️.rs:224` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[REPLAY] fusion {fusion_config:?}: {kept} consistent pixels");`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/♻️dense-replay/🦀️.rs:242` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[REPLAY] extrema {extrema_min:.3?}..{extrema_max:.3?} lattice {grid_min:?}..{grid_max:?} volume blocks {}", volume.block_count());`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/♻️dense-replay/🦀️.rs:291` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("[REPLAY] voxel {voxel_size} trunc {truncation}: trail {trail:?}; extracted {extracted:?} -> {outcome}; {} vertices, surface distance {}, completeness {:.1}%", positions.len(), quantiles(positions.iter().map...`

### `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs` (31)

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:57` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("tracks {}", engine.tracks.as_ref().map_or(0, |t| t.tracks.len()));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:58` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("pairs {:?}", engine.pairwise_matches.iter().map(|(a, b, m)| (*a, *b, m.len())).collect::<Vec<_>>());`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:62` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("cameras {:?} points {}", r.cameras.iter().map(|c| c.0).collect::<Vec<_>>(), r.points.len());`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:75` [eprintln!] test-path: yes; allocates: yes: .map(, println!
  - `EngineStatus::Done => { eprintln!("[TRACE] done mesh {:?}", engine.mesh_data.as_ref().map(|m| (m.positions.len()/3, m.indices.len()/3))); break; }`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:77` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("failed {msg} mesh so far {}/{}", engine.mesh_pipeline.as_ref().map_or(0, |p| p.mesh().positions.len()), engine.mesh_pipeline.as_ref().map_or(0, |p| p.mesh().triangles.len()));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:131` [eprintln!] test-path: yes; allocates: yes: println!
  - `EngineObservation::PairMatched { frame_a, frame_b, matches } if (*frame_a, *frame_b) == (0, 1) || (*frame_a, *frame_b) == (11, 12) => eprintln!("[GEO] observed pair ({frame_a},{frame_b}) kept {matches} matches"),`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:132` [eprintln!] test-path: yes; allocates: yes: println!
  - `EngineObservation::PairGeometryRejected { frame_a, frame_b, dropped } if (*frame_a, *frame_b) == (0, 1) || (*frame_a, *frame_b) == (11, 12) => eprintln!("[GEO] observed pair ({frame_a},{frame_b}) dropped {dropped} mat...`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:136` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[GEO] {} observations, {} keypoints in frame 11, {} in frame 12", observations.len(), engine.keypoints_per_frame[11].len(), engine.keypoints_per_frame[12].len());`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:176` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[GEO] pair ({a},{b}): {} matches, {good} true-epipolar inliers (<0.005 normalized), median sampson {median:.4}", matches.len());`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:226` [eprintln!] test-path: yes; allocates: yes: .filter(, println!
  - `eprintln!("[GEO] frame {a}: {} keypoints, {off_cube} off the cube; nearest frame-{b} keypoint to the true transfer: median {:.2} px, 25% {:.2}, 75% {:.2}, within 2 px {}", engine.keypoints_per_frame[a].len(), quantile...`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:227` [eprintln!] test-path: yes; allocates: yes: .filter(, println!
  - `eprintln!("[GEO] matched keypoints vs true transfer: {} matched, median {:.2} px, within 2 px {}, within 5 px {}", matched_errors.len(), quantile(&matched_errors, 0.5), matched_errors.iter().filter(|e| **e < 2.0).coun...`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:229` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[GEO] first frame-{a} keypoints (x, y, octave, response): {first:?}");`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:258` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[GEO] octaves of frame-{a} keypoints: {octaves:?}; {} true base-level pairs: median Hamming of the true pair {} (min {}), median best other {}, median |Δangle| {:.1}°", true_distances.len(), mid(&true_dista...`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:269` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[GEO] registering frame {seen_cursor}: {} 2D-3D correspondences, {} points, {} cameras so far", sfm.correspondence_count(seen_cursor), sfm.point_count(), sfm.registered_count());`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:281` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[GEO]   frame {seen_cursor} tracks by registered span (span: tracks/with point): {by_span:?}");`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:288` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("[GEO] {label}: registered {:?}", registered.iter().map(|(frame, _)| *frame).collect::<Vec<_>>());`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:298` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[GEO] {label}: camera {frame} relative to {first}: rotation error {rotation_error:.2}°, translation direction error {:.2}°, baseline recovered {:.3} true {:.3}", cosine.clamp(-1.0, 1.0).acos().to_degrees(),...`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:300` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[GEO] {label}: worst rotation error {worst:.2}°");`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:377` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DENSE] params dense={:?} voxel={} trunc={} dense_source_views={} max_dense_cameras={}", params.dense, params.tsdf_voxel_size, params.tsdf_truncation, params.dense_source_views, params.max_dense_cameras);`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:404` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DENSE] {last:?} -> {:?}", engine.stage());`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:425` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DENSE] dumped to {path}");`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:431` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DENSE] {} cameras, sim scale {:.4}, centre error {}", recovered.len(), sim.s, quantiles(centre_errors));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:432` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("[DENSE] sparse surface distance {}", quantiles(reconstruction.points.iter().map(|p| surface(sim.act(*p))).collect()));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:466` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:482` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DENSE] fusion tolerance {tolerance} views {minimum}: {} points ({far} > 0.2 half-sizes off), surface distance {}", fused.positions.len(), quantiles(distances));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:485` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DENSE] extrema {extrema_min:.3?}..{extrema_max:.3?} lattice {lattice_min:?}..{lattice_max:?} blocks {}", tsdf.block_count());`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:545` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("[DENSE] grid voxel {voxel_size} trunc {truncation}: {outcome}; {} vertices, surface distance {}, completeness {:.1}%", positions.len(), quantiles(positions.iter().map(|p| surface(*p)).collect()), 100.0 * co...`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:547` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DENSE] tsdf lattice voxels: unobserved {unobserved}, sign agrees with truth {agree}, disagrees {disagree}; disagreeing voxels' surface distance {}", quantiles(disagreeing_depth));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:550` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("[DENSE] pipeline mesh {triangles} triangles / {} vertices: surface distance {}", positions.len(), quantiles(positions.iter().map(|p| surface(sim.act(*p))).collect()));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:552` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("[DENSE] fused cloud surface distance {}", quantiles(cloud.iter().map(|p| surface(sim.act(*p))).collect()));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs:563` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("[DENSE] mesh {} vertices / {} triangles, aligned bounds {lo:.2?}..{hi:.2?}, surface distance {}", vertices.len(), mesh.indices.len() / 3, quantiles(vertices.iter().map(|v| surface(*v)).collect()));`

### `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs` (1)

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:197` [eprintln!] test-path: yes; allocates: yes: println!
  - `if let Err(error) = &outcome { eprintln!("[TRACE] Remodel exact-window runtime failure before close: {error}"); }`

### `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️reconstruction-session/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️reconstruction-session/🧪️tests/🔬️unit/🦀️.rs:810` [eprintln!] test-path: yes; allocates: yes: println!, serde
  - `eprintln!("[FIXTURE] {}", serde_json::to_string_pretty(&case).expect("case json"));`

### `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🦀️.rs` (2)

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🦀️.rs:859` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[ORB] a rendered view encodes to {} PNG bytes", remodeling_image::encode_png(&image_a).map_or(0, |bytes| bytes.len()));`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🦀️.rs:887` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[ORB] step {step_deg:>4}° upright={upright}: {} / {} keypoints, {} true pairs (Hamming q25 {} median {}; impostor median {}), {} matches, {correct} within 2 px", keypoints_a.len(), keypoints_b.len(), true_p...`

### `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧬️owned/🦀️.rs` (2)

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧬️owned/🦀️.rs:89` [eprintln!] test-path: yes; allocates: yes: String::from, println!
  - `fn oracle(script:&str,input:&[u8])->Vec<u8>{use std::io::Write;let mut child=std::process::Command::new("bun").args(["-e",script]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::p...`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧬️owned/🦀️.rs:102` [console.log] test-path: yes; allocates: yes: .join(, .map(, JSON.stringify
  - `let script=r####"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'...`

### `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/📜️script.ts` (1)

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/📜️script.ts:14` [console.log] test-path: no; allocates: yes: ${
  - `console.log('remodeling capabilities: targets=${prepareCargoCapabilityLinksV1(this.repoRoot, familyRoot, "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/📦️packages/🦀️rust/Cargo.toml", "🧩️composition/🔗️capab...`

### `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🧪️tests/🔬️rust-package-commands/🟦️.ts:34` [console.log] test-path: yes; allocates: no
  - `console.log("video-container-providers: " + proveVideoContainerProviderOracleV1() + " portable oracle vectors admitted");`

### `✏️s/🔌️plugins/📸️remodel/🧪️tests/🌐️browser/🧭️journey/🟦️.ts` (3)

- `✏️s/🔌️plugins/📸️remodel/🧪️tests/🌐️browser/🧭️journey/🟦️.ts:19` [console.log] test-path: yes; allocates: no
  - `console.log("[remodel] Inspecting " + item.mode + " window instances");`
- `✏️s/🔌️plugins/📸️remodel/🧪️tests/🌐️browser/🧭️journey/🟦️.ts:289` [console.log] test-path: yes; allocates: yes: Date.now
  - `if (Date.now() - progressAt >= 10_000) { console.log("[remodel] Waiting for " + id); progressAt = Date.now(); }`
- `✏️s/🔌️plugins/📸️remodel/🧪️tests/🌐️browser/🧭️journey/🟦️.ts:367` [console.log] test-path: yes; allocates: yes: Date.now
  - `if (Date.now() - progressAt >= 10_000) { console.log("[remodel] Waiting for reconstruction state"); progressAt = Date.now(); }`

### `✏️s/🔌️plugins/🔋️energy/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🔋️energy/🏭️bridge/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🔋️energy/🏭️bridge/🦀️.rs:51` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts` (2)

- `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts:42` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[oracle] ${tool.tool} ${tool.version} -> ${directory}${existsSync(directory) ? "" : " (missing — run setup)"}');`
- `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🏃️execution/🟦️.ts:46` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[oracle]   ${name}: ${path}${existsSync(path) ? "" : " (missing)"}${probe ? ' -> ${probe.stdout.trim() || probe.stderr.trim()}' : ""}');`

### `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🟦️.ts` (3)

- `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🟦️.ts:62` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[oracle] downloading ${pin.asset} (${(pin.bytes / 1e6).toFixed(1)} MB) from ${url}');`
- `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🟦️.ts:67` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[oracle] verified ${pin.asset} sha256 ${digest}');`
- `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🛠️toolchain/🟦️.ts:69` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[oracle] extracting into ${directory}');`

### `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (1)

- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:95` [console.log] test-path: yes; allocates: yes: .join(, .map(, JSON.stringify
  - `const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{deepStrictEqual(db.query('PRAGMA integrity_check').get(),{integrity_check:'ok'});deepStrictEqual(db.query('PRAGMA fo...`

### `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🧪️tests/🔬️rust-package-commands/🟦️.ts:56` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `for (const row of module.solarPvFixtureCases) console.log(JSON.stringify({id:row.id,expected:module.calculateSolarPvReference(row)}));`

### `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/🦀️.rs:54` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🦀️.rs` (1)

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🦀️.rs:127` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Rewriting window configuration runtime failure before close: {error}");`

### `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🧪️tests/🔬️typescript-package-commands/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[trinity-rewriting] checked published consumers=${files.length}');`

### `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🔬️unit/🦀️.rs:279` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("resumable query completed in {steps} steps with {} document mutations", actual.1.len());`

### `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs` (1)

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs:61` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio_s_plugin_trinity_jack_shell: {err}");`

### `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🏠️host/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🏠️host/🧪️tests/🔬️unit/🦀️.rs:275` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] manifest retirement {manifest_steps} turns {manifest_elapsed:?}; snapshot clone {clone_elapsed:?}; snapshot retirement {snapshot_steps} turns {snapshot_elapsed:?}");`

### `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️typescript-package-commands/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[trinity-jack] checked published consumers=${files.length}');`

### `✏️s/🔌️plugins/🕸️dag/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🕸️dag/🏭️bridge/🦀️.rs:44` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🕸️dag/🏭️bridge/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/📜️script.ts` (1)

- `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/📜️script.ts:7` [console.log] test-path: no; allocates: no
  - `run(): void { console.log("[TRACE] dag ts ok"); }`

### `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🧪️tests/🔬️typescript-package-commands/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[dag-dag] checked published consumers=${files.length}');`

### `✏️s/🔌️plugins/🖍️draw/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🖍️draw/🏭️bridge/🦀️.rs:43` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🖍️draw/🏭️bridge/🦀️.rs:48` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config/🧪️tests/🔬️window/🦀️.rs` (1)

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config/🧪️tests/🔬️window/🦀️.rs:111` [eprintln!] test-path: yes; allocates: yes: println!
  - `if let Err(error) = &outcome { eprintln!("[TRACE] Drawing exact-window runtime failure before close: {error}"); }`

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🟦️.ts` (1)

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🟦️.ts:55` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${refreshed} fixture manifest(s) refreshed');`

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📖️reader/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.json> | reader compare <expected.json> <actual.json>");`

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️descriptor-parity/🟦️.ts` (1)

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️descriptor-parity/🟦️.ts:42` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Artifact IO descriptor parity checks=${checks}, entries=${ioEntries.length}');`

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎨️fill/🦀️.rs` (1)

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎨️fill/🦀️.rs:137` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] typed fill edits matched the shared gradient fixtures");`

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🖊️stroke/🦀️.rs` (2)

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🖊️stroke/🦀️.rs:72` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] stroke cap and join codecs matched all shared enum cases");`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🖊️stroke/🦀️.rs:83` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] stroke dash parsing matched all shared cases");`

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/↔️translation/🦀️.rs` (1)

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/↔️translation/🦀️.rs:23` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] world translation agrees with all nested-parent fixtures");`

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📏️arrangement/🦀️.rs` (1)

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📏️arrangement/🦀️.rs:38` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] document-axis arrangement matches shared alignment and spacing fixtures");`

### `✏️s/🔌️plugins/🖨️raster/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🖨️raster/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🖨️raster/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎞️reserved-import/🧪️structured-wire/image-oracle.ts` (1)

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎞️reserved-import/🧪️structured-wire/image-oracle.ts:15` [console.log] test-path: no; allocates: no
  - `console.log("Raster PNG import oracle: " + fixture.cases.length + " images, " + fixture.errors.length + " failures");`

### `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (1)

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:94` [eprintln!] test-path: yes; allocates: yes: String::from, println!
  - `child.stdin.take().unwrap().write_all(bytes).unwrap();let output=child.wait_with_output().unwrap();let diagnostics=String::from_utf8(output.stderr).unwrap();assert!(output.status.success(),"{diagnostics}");eprintln!("...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🟦️.ts:43` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${outPath}\n${digest}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🟦️.ts:66` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entry, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:33` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("wrote {} bytes to {out_path}", bytes.len());`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🔮️oracles/🧪️tests/🔬️oracles-unit/🦀️.rs` (4)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🔮️oracles/🧪️tests/🔬️oracles-unit/🦀️.rs:105` [console.log] test-path: yes; allocates: no
  - `let input = b"<!doctype html>\n<html><head><style>.a { color: red; }</style><script>if (1 < 2) { console.log(1); }</script></head><body></body></html>";`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🔮️oracles/🧪️tests/🔬️oracles-unit/🦀️.rs:113` [console.log] test-path: yes; allocates: no
  - `assert!(matches!(&script_children[0], HNode::RawText { script: true, text } if text.contains("console.log")));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🔮️oracles/🧪️tests/🔬️oracles-unit/🦀️.rs:115` [console.log] test-path: yes; allocates: yes: vec!
  - `let round_tripped = apply_then_undo(input, "set-raw-text", &obj(vec![("path", Json::Array(vec![Json::Number(0.0), Json::Number(1.0), Json::Number(0.0)])), ("text", Json::String("console.log(2);".into()))])).unwrap();`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🔮️oracles/🧪️tests/🔬️oracles-unit/🦀️.rs:170` [console.log] test-path: yes; allocates: yes: vec!
  - `("set-raw-text", obj(vec![("path", Json::Array(vec![Json::Number(2.0), Json::Number(29.0), Json::Number(0.0)])), ("text", Json::String("console.log('wave7');".into()))])),`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:95` [console.log] test-path: yes; allocates: no
  - `let snap = parse_html_document("<html><!-- hi --><style>.a { color: red; }</style><script>if (1 < 2) { console.log(\"</not-a-tag>\"); }</script></html>").unwrap();`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:325` [console.log] test-path: yes; allocates: yes: vec!
  - `HtmlMutation::SetRawText(set_raw_text::SetRawText { path: vec![0, 1], text: "console.log(1);".into() }),`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🧪️tests/🔬️rust-package-commands/🟦️.ts:33` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('retained-opc-check: parts=${oracle.parts.length} payload=${oracle.parts[0].bytes.byteLength} owners=${oracle.relationships.length}');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🏭️generator/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🏭️generator/🟦️.ts:220` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/🦀️.rs:121` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.gif> | reader compare <expected.gif> <actual.gif>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🟦️.ts` (3)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🟦️.ts:114` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${outPath}\n${digest}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🟦️.ts:137` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entry, null, 2));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🟦️.ts:225` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(manifests, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📖️reader/🦀️.rs` (7)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📖️reader/🦀️.rs:406` [eprintln!] test-path: yes; allocates: yes: .join(, println!
  - `eprintln!("[gif-89a-reader] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📖️reader/🦀️.rs:413` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[gif-89a-reader] {id}: {before_file} + {after_file} -> {}", dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📖️reader/🦀️.rs:421` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[gif-89a-reader] cannot read {path}: {error}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📖️reader/🦀️.rs:431` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[gif-89a-reader] cannot project {path}: {error}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📖️reader/🦀️.rs:442` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("usage: reader build <recipe-id> <out-dir> <fixture-directory> <before-file> <after-file>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📖️reader/🦀️.rs:449` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("usage: reader project <path-to-gif>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📖️reader/🦀️.rs:461` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("usage: reader build <recipe-id> <out-dir> <fixture-directory> <before-file> <after-file> | project <path-to-gif> | list-recipes");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/🦀️.rs:21` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("wrote {} bytes to {out_path}", bytes.len());`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/🦀️.rs:103` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.gif> | reader compare <expected.gif> <actual.gif>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs` (6)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:439` [eprintln!] test-path: no; allocates: yes: .join(, println!
  - `eprintln!("[quick-xml-svg-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:446` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[quick-xml-svg-codec] {id}: {before_file} + {after_file} -> {}", dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:454` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[quick-xml-svg-codec] cannot read {path}: {e}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:468` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: quick-xml-svg-codec build <recipe-id> <out-dir> <physical-dir> <before-file> <after-file>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:475` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: quick-xml-svg-codec project <path-to-svg>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:487` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: quick-xml-svg-codec build <recipe-id> <out-dir> <physical-dir> <before-file> <after-file> | project <path-to-svg> | list-recipes");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️codec/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️codec/🦀️.rs:174` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("native-codec-publication={receipt}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-work.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-work.rs:89` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!, static 
  - `{static N:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);if N.fetch_add(1,std::sync::atomic::Ordering::Relaxed)<12{eprintln!("[DEBUG] guided close_owned demand={:?} grant={:?} retirement_empty={...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/render-work.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/render-work.rs:51` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error)=&result{eprintln!("[DEBUG] render refusal {error:?}");}`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/tests.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/tests.rs:746` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `if !work.terminal_is_empty() { eprintln!("[DEBUG] residue {}", work.debug_residue()); }`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/tree-probe-tests.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/tree-probe-tests.rs:10` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `if !node.rejected_children.is_empty(){eprintln!("[DEBUG] rejected children under {}: {}",node.key.as_str(),node.rejected_children.len());}`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏭️pack-tables.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏭️pack-tables.ts:19` [console.log] test-path: no; allocates: no
  - `for(const[name,files]of Object.entries(tableFiles)){const pack=encodeJsonPack(JSON.parse(await readFile(files.json,"utf8")) as PackJson);await writeFile(files.pack,pack);console.log(name,pack.byteLength);}`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏭️pack-tables.ts:20` [console.log] test-path: no; allocates: no
  - `console.log("program sha256",programDigest(program));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🛠️extract/🏭️pack-all.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🛠️extract/🏭️pack-all.ts:5` [console.log] test-path: no; allocates: yes: ${
  - `for(const standard of standards){const owner=fileURLToPath(new URL('../../🏅️standards/${standard}/🧬️schema/📚️catalog/',import.meta.url));console.log(standard,await packCatalogFile({json:owner+"🔣️.json",pack:owner+"cat...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧭️profile/retained.rs` (3)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧭️profile/retained.rs:107` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!
  - `let polled = match native.receive_effect(demand, maximum, &mut frame, run) { Ok(polled) => polled, Err(error) => { eprintln!("[DEBUG] receive_effect error {error:?} phase={:?} demand={:?} maximum={maximum}", frame.pha...`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧭️profile/retained.rs:115` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!
  - `if matches!(done, Some(NativeReceivingEffectPoll::Pending { work: 0 })) { eprintln!("[DEBUG] receive_effect pending0 phase={:?}", frame_phase); }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧭️profile/retained.rs:122` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!
  - `Some(NativeReceivingEffectPoll::Pending { work }) => { eprintln!("[DEBUG] receive_effect pending work={work} phase={:?} demand={:?} fuel={}", frame_phase, demand, cx.fuel_remaining()); Err(ValueError::invariant("nativ...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧶️guidance/type-guidance.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧶️guidance/type-guidance.rs:1836` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!, static 
  - `{static N:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);if N.fetch_add(1,std::sync::atomic::Ordering::Relaxed)<6{eprintln!("[DEBUG] type-input close idle phase={} demand={:?} grant={:?} body={}...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🏭️generator/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🏭️generator/🟦️.ts:179` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[bcf generator] ${id} -> ${directory}');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/🔭️third-party-generators/🟦️.ts` (3)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/🔭️third-party-generators/🟦️.ts:217` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[xlsx plumbing] ${recipe.id} -> ${directory}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/🔭️third-party-generators/🟦️.ts:237` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('${recipe.id}: ${name} has the same plumbing but other container bytes');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/🔭️third-party-generators/🟦️.ts:244` [console.log] test-path: no; allocates: yes: ${
  - `console.log(failures === 0 ? 'every plumbing pair re-derives (${byteIdentical} files byte-identical)' : '${failures} pair(s) failed');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/🏗️generate/🦀️.rs:61` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📖️reader/🦀️.rs:56` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🟦️.ts:62` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${kind}: before=${await sha256(beforePath)} after=${await sha256(afterPath)}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🟦️.ts:101` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/🏗️generate/🦀️.rs:62` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📖️reader/🦀️.rs:56` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🟦️.ts:61` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${fixtureDirectory}: before=${await sha256(basePath)} after=${await sha256(mutatedPath)}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🟦️.ts:100` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🏗️generate/🦀️.rs:64` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📖️reader/🦀️.rs:56` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🟦️.ts:65` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${kind}: before=${await sha256(beforePath)} after=${await sha256(afterPath)}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🟦️.ts:104` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/🏗️generate/🦀️.rs:71` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📖️reader/🦀️.rs:56` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🟦️.ts:71` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${kind}: before=${await sha256(beforePath)} after=${await sha256(afterPath)}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🟦️.ts:110` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/🏗️generate/🦀️.rs:70` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📖️reader/🦀️.rs:56` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🟦️.ts:70` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${kind}: before=${await sha256(beforePath)} after=${await sha256(afterPath)}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🟦️.ts:109` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/🏗️generate/🦀️.rs:70` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📖️reader/🦀️.rs:56` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🟦️.ts:67` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${kind}: before=${await sha256(beforePath)} after=${await sha256(afterPath)}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🟦️.ts:106` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/🏗️generate/🦀️.rs:72` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📖️reader/🦀️.rs:56` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🟦️.ts:69` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${kind}: before=${await sha256(beforePath)} after=${await sha256(afterPath)}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🟦️.ts:108` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/🏗️generate/🦀️.rs:72` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📖️reader/🦀️.rs:56` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🟦️.ts:69` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${kind}: before=${await sha256(beforePath)} after=${await sha256(afterPath)}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🟦️.ts:108` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/💾️native-save/🧪️tests/🔬️original-save/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/💾️native-save/🧪️tests/🔬️original-save/🦀️.rs:217` [[DEBUG]] test-path: yes; allocates: yes: println!
  - `println!("[DEBUG] segmented native Save wrote and retired {} bytes", actual.len());`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (4)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs:271` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `if retire_turns % 256 == 0 { eprintln!("[DEBUG] shard close turn {retire_turns}"); }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs:318` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] shard turn {} at {:?}: {} / {} spent={} retained={:?}", open.log.len() + 1, started.elapsed(), turn.ingress, turn.status, turn.spent, turn.retained);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs:334` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] shard command loop ended after {} turns, closing", open.log.len());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs:338` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] shard instance closed");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🏗️generate/🦀️.rs:120` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📖️reader/🦀️.rs:56` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🟦️.ts:154` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/🦀️.rs:41` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("wrote {} bytes to {out_path}", bytes.len());`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('scalar-schema-producer fields=${rows.length} independent-sqlite=${fixture.fields.length}');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🌱️value/🧬️octets/🧪️tests/🛫️output/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🌱️value/🧬️octets/🧪️tests/🛫️output/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: ${
  - `}finally{db.close();}console.log('pdf-cos-output tags=${fixture.cases.length} intrinsic-octet-witnesses=2');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/🏗️generate/🦀️.rs:76` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📖️reader/🦀️.rs:56` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.pdf> | reader compare <expected.pdf> <actual.pdf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🟦️.ts:78` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${kind}: before=${await sha256(beforePath)} after=${await sha256(afterPath)}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🟦️.ts:117` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(entries, null, 2));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🧪️tests/🔬️unit/🦀️.rs:71` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("native-codec-publication={receipt}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🏭️generator/📜️script.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🏭️generator/📜️script.ts:348` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[docx generator] ${id} -> ${join(outDir, FIXTURE_DIRECTORY_NAMES[id]!)}');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs` (7)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:609` [eprintln!] test-path: no; allocates: yes: .join(, println!
  - `eprintln!("[quick-xml-oracle-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:616` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[quick-xml-oracle-codec] {id}: {before_file} + {after_file} -> {}", dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:624` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[quick-xml-oracle-codec] cannot read {path}: {e}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:634` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[quick-xml-oracle-codec] cannot parse {path}: {error}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:645` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: quick-xml-oracle-codec build <recipe-id> <out-dir> <directory> <before-file> <after-file>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:652` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: quick-xml-oracle-codec project <path-to-xml>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs:664` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: quick-xml-oracle-codec build <recipe-id> <out-dir> <directory> <before-file> <after-file> | project <path-to-xml> | list-recipes");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs` (6)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:379` [eprintln!] test-path: no; allocates: yes: .join(, println!
  - `eprintln!("[png-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:386` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[png-codec] {id}: ⬅️before.png + ➡️after.png -> {}", dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:394` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[png-codec] cannot read {path}: {e}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:408` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: png-codec build <recipe-id> <out-dir>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:415` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: png-codec project <path-to-png>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:427` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: png-codec build <recipe-id> <out-dir> | project <path-to-png> | list-recipes");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️codec/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️codec/🦀️.rs:166` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `let mut oracle = Command::new("bun").args(["-e", "import {PNG} from 'pngjs';const bytes=Buffer.from(await Bun.stdin.arrayBuffer()),image=PNG.sync.read(bytes);if(Math.abs(image.gamma-0.45455)>1e-9)throw Error('gamma ${...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🧪️tests/🧬️owned-native-oracle/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🧪️tests/🧬️owned-native-oracle/🦀️.rs:27` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("native-codec-publication={receipt}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🚦️audit/🦀️.rs` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🚦️audit/🦀️.rs:15` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify, collect
  - `let input=corpus();for case in input["validPng"].as_array().unwrap().iter().filter(|case|!case["background"].is_null()){for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let snapshot=decode(&png(case),...`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🚦️audit/🦀️.rs:19` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify, clone(
  - `let input=corpus();let case=input["validPng"].as_array().unwrap().iter().find(|case|case["id"]=="latin1-and-itext").unwrap();for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{let snapshot=decode(&png(c...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs` (5)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:193` [eprintln!] test-path: no; allocates: yes: .join(, println!
  - `eprintln!("[jpeg-jfif-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:200` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[jpeg-jfif-codec] {id}: ⬅️before.jpg ({} bytes) + ➡️after.jpg ({} bytes) -> {}", before.len(), after.len(), dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:214` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: jpeg-jfif-codec build <recipe-id> <out-dir> <directory-name>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:221` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: jpeg-jfif-codec project <path-to-jpg>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:233` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: jpeg-jfif-codec build <recipe-id> <out-dir> | project <path-to-jpg> | list-recipes");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🎹️facets/🧪️tests/🔬️conformance/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🎹️facets/🧪️tests/🔬️conformance/🦀️.rs:20` [eprintln!] test-path: yes; allocates: yes: json!, println!, serde
  - `let receipt=serde_json::json!({"schemaVersion":1,"artifactKind":kind.id,"artifactSchema":codec.schema,"factoryId":factory.id,"extension":codec.extension,"packSchemaHash":hex(&codec.pack_schema_hash),"protocolSourceSha...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🟦️.ts:15` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('scalar-schema-producer fields=${rows.length} independent-sqlite=${fixture.fields.length}');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:93` [console.log] test-path: yes; allocates: no
  - `let script=r#"import {Database}from'bun:sqlite';const input=JSON.parse(process.argv[1]);const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const row=db.query('SELECT COUNT(*) AS count,MIN(oct...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/🦀️.rs` (7)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/🦀️.rs:640` [eprintln!] test-path: no; allocates: yes: .join(, println!
  - `eprintln!("[riff-avi-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/🦀️.rs:648` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[riff-avi-codec] {id}: ⬅️before.avi + ➡️after.avi -> {}", dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/🦀️.rs:650` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[riff-avi-codec] {id}: ⬅️before.avi only (rejected) -> {}", dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/🦀️.rs:659` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[riff-avi-codec] cannot read {path}: {e}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/🦀️.rs:673` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: riff-avi-codec build <recipe-id> <fixture-dir>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/🦀️.rs:680` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: riff-avi-codec project <path-to-avi>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/🦀️.rs:692` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: riff-avi-codec build <recipe-id> <out-dir> | project <path-to-avi> | list-recipes");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧭️transform/🧪️tests/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧭️transform/🧪️tests/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const child = spawnSync(process.env.SEMIO_NODE_EXECUTABLE ?? "node", ["--input-type=module", "-e", 'let input="";for await(const chunk of process.stdin)input+=chunk;console.log(JSON.stringify(JSON.parse(input).map(({w...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️codec/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️codec/🦀️.rs:347` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("native-codec-publication={receipt}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🏗️grammar/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🏗️grammar/🟦️.ts:63` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('dwg:grammar-shape cases=${corpus.cases.length} emptyRecordFields=${corpus.record.fieldCount} oracle=Chevrotain');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs` (12)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:500` [eprintln!] test-path: yes; allocates: yes: .join(, println!
  - `eprintln!("[dxf-r12-any-engine] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:509` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dxf-r12-any-engine] {id}: {} -> {}", path.display(), out_dir);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:518` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dxf-r12-any-engine] {id}: before.dxf + after.dxf -> {}", dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:520` [eprintln!] test-path: yes; allocates: yes: println!
  - `None => eprintln!("[dxf-r12-any-engine] {id}: before.dxf only (rejected) -> {}", dir.display()),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:531` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dxf-r12-any-engine] cannot read {path}: {e}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:538` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dxf-r12-any-engine] dxf load failed for {path}: {e:?}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:563` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dxf-r12-any-engine] perturb-radius-debug: no circle entity found in {in_path}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:567` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dxf-r12-any-engine] perturb-radius-debug: {in_path} radius+{delta} -> {out_path}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:576` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("usage: engine build <recipe-id> <out-dir>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:583` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("usage: engine project <path-to-dxf>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:596` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("usage: engine perturb-radius-debug <in.dxf> <out.dxf> <delta>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/🦀️.rs:602` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("usage: engine build <recipe-id> <out-dir> | project <path-to-dxf> | list-recipes | perturb-radius-debug <in> <out> <delta>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('scalar-schema-producer fields=${rows.length} independent-sqlite=${fixture.fields.length}');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs` (6)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:248` [eprintln!] test-path: no; allocates: yes: .join(, println!
  - `eprintln!("[tiff-ifd-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:255` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[tiff-ifd-codec] {id}: before.tiff + after.tiff -> {}", dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:266` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[tiff-ifd-codec] project {path}: {e}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:277` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: tiff-ifd-codec build <recipe-id> <out-dir>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:284` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: tiff-ifd-codec project <path-to-tiff>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/🦀️.rs:296` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: tiff-ifd-codec build <recipe-id> <out-dir> | project <path-to-tiff> | list-recipes");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/🦀️.rs:72` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.tif> | reader compare <expected.tif> <actual.tif>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔮️oracles/🧪️tests/🔬️fixture-derivation/🦀️.rs` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔮️oracles/🧪️tests/🔬️fixture-derivation/🦀️.rs:83` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("wrote {} ({} bytes)", out_path.display(), bytes.len());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔮️oracles/🧪️tests/🔬️fixture-derivation/🦀️.rs:88` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("inline 8x8 real thumbnail hex (insert-ifd pixels): {tiny_hex}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🦀️.rs` (8)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🦀️.rs:115` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[tobj-obj-reader] cannot read {path}: {e}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🦀️.rs:124` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[tobj-obj-reader] tobj rejected {path}: {e}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🦀️.rs:284` [eprintln!] test-path: no; allocates: yes: .join(, .map(, collect, println!
  - `eprintln!("[tobj-obj-reader] unknown recipe {id:?} — known: {}", RECIPES.iter().map(|r| r.id).collect::<Vec<_>>().join(", "));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🦀️.rs:295` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[tobj-obj-reader] {}: before.obj + after.obj -> {}", recipe.id, dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🦀️.rs:298` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[tobj-obj-reader] {}: before.obj only (rejected) -> {}", recipe.id, dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🦀️.rs:309` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: tobj-obj-reader build <recipe-id> <out-dir>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🦀️.rs:316` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: tobj-obj-reader project <path-to-obj>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/🦀️.rs:328` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: tobj-obj-reader build <recipe-id> <out-dir> | project <path-to-obj> | list-recipes");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🟦️.ts` (3)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🟦️.ts:90` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${outPath}\n${digest}');`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🟦️.ts:170` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(only === null ? entries : entries[0], null, 2));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🟦️.ts:190` [console.log] test-path: no; allocates: yes: .join(, .map(
  - `console.log([PATTERN_SHELL_RECIPE, ...READER_RECIPES.map((r) => r.id)].join("\n"));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/🦀️.rs` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/🦀️.rs:77` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("tobj 4 admitted the document: {models} model(s), {vertices} referenced vertex position(s), {indices} triangulated index/indices");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/🦀️.rs:79` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("wrote {} bytes to {out_path}", bytes.len());`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🏭️generator/📜️script.ts` (4)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🏭️generator/📜️script.ts:74` [console.log] test-path: no; allocates: yes: ${, .map(, JSON.stringify
  - `console.log(JSON.stringify([...RECIPES.map((r) => ({ id: '${r.mutationId}-applied', mutationId: r.mutationId, outcome: r.outcome, family: r.family, notes: r.notes })), ...REAL_ROWS.map((row) => ({ id: row.id, mutation...`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🏭️generator/📜️script.ts:77` [console.log] test-path: no; allocates: no
  - `for (const r of RECIPES) console.log(r.id);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🏭️generator/📜️script.ts:78` [console.log] test-path: no; allocates: no
  - `for (const row of REAL_ROWS) console.log(row.id);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🏭️generator/📜️script.ts:124` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[generator] wrote ${count} fixture bundle(s) into ${outRoot ?? "declared catalog paths"}');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/🦀️.rs:192` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <path.gltf> | reader compare <expected.gltf> <actual.gltf>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/📜️script.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/📜️script.ts:108` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${entries.length} fixture manifest(s) written and registered in 🔣️oracle.json');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/🦀️.rs` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/🦀️.rs:43` [eprintln!] test-path: no; allocates: yes: println!
  - `let out = std::env::args().nth(1).unwrap_or_else(|| { eprintln!("usage: generate <out-dir>"); std::process::exit(2) });`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/🦀️.rs:57` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{written} file(s)");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge/🦀️.rs:20` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge/🦀️.rs:24` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge answers only 'list-mutations s.stdio.semio v1 base', not {command} {artifact} {standard} {subset}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/⭕️step-line-circle/📜️script.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/⭕️step-line-circle/📜️script.ts:50` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[${FAMILY}] nothing to do');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🖊️dxf-entities/📜️script.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🖊️dxf-entities/📜️script.ts:54` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[${FAMILY}] nothing to do');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/🏗️generate/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/🦀️.rs` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/🦀️.rs:104` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[probe] unknown command {command:?} — expected dxf-read | dxf-witness | dxf-compare | step-read | step-witness | step-compare | generate");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/🦀️.rs:118` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {recipe}: {e}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📜️script.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📜️script.ts:91` [console.log] test-path: no; allocates: yes: ${, .join(, JSON.stringify
  - `console.log(JSON.stringify(failed(probe, 'oracle probe exited ${run.status}', (run.stderr ?? "").trim().split("\n").slice(-6).join("\n"))));`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/🏗️generate/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📖️reader/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📖️reader/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: reader project <file.json> | reader compare <expected.json> <actual.json>");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/🏗️generate/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/📜️script.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/📜️script.ts:126` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${entries.length} fixture manifest(s) written and registered in 🔮️oracles/🔣️.json');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/🦀️.rs` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/🦀️.rs:223` [eprintln!] test-path: no; allocates: yes: println!
  - `let out = std::env::args().nth(1).unwrap_or_else(|| { eprintln!("usage: generate <out-dir>"); std::process::exit(2) });`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/🦀️.rs:235` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{written} file(s)");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/🏗️generate/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/🏗️generate/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/🏗️generate/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {failure}");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/⏱️tessellation-jobs/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/⏱️tessellation-jobs/🦀️.rs:167` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("lod {label}/{lod} (deflection {tolerance}): {steps} budgeted steps, {triangles} triangles, {} vertices, {} edge floats, {:?}, max chordal {:.6}", mesh.position.len() / 3, mesh.edges.len(), elapsed, report.m...`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/🎨️analytic-blend/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/🎨️analytic-blend/🦀️.rs:266` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("⏱️ attempt {attempt}: fillet {filleted:?}, fillet + tessellation {total:?}, {} triangles", mesh.index.len() / 3);`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/📜️script.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/📜️script.ts:16` [console.log] test-path: no; allocates: yes: ${
  - `if (segments.length === 1 && segments[0] === "check") { console.log('semio conversion definition: checks=${runSemioConversionDefinitionChecks()}'); return; }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/📜️script.ts:18` [console.log] test-path: no; allocates: yes: ${
  - `console.log('semio conversion composition: targets=${prepareSemioConversionDefinitionV1(this.repoRoot, resolve(this.root, "../.."))}');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🧪️tests/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🧪️tests/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[TRACE] bmp-paint-region-twin checks=${checks}');`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs` (7)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:246` [eprintln!] test-path: no; allocates: yes: .join(, println!
  - `eprintln!("[image-bmp-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:253` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[image-bmp-codec] {id}: ⬅️before.bmp + ➡️after.bmp -> {}", dir.display());`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:261` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[image-bmp-codec] cannot read {path}: {e}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:271` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[image-bmp-codec] project {path} failed: {error}");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:282` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: image-bmp-codec build <recipe-id> <out-dir>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:289` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: image-bmp-codec project <path-to-bmp>");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/🦀️.rs:301` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: image-bmp-codec build <recipe-id> <out-dir> | project <path-to-bmp> | list-recipes");`

### `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🔮️oracles/🧪️tests/🔬️fixture-derivation/🦀️.rs` (1)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🔮️oracles/🧪️tests/🔬️fixture-derivation/🦀️.rs:63` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("wrote {} ({} bytes, {}x{}, 240-entry table)", out_path.display(), bytes.len(), frame.width, frame.height);`

### `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts` (2)

- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts:418` [console.log] test-path: yes; allocates: yes: ${, .join(, .map(
  - `console.log('[artifact-acceptance] ${options.entry}/${row.batch}/${row.artifact}: ${entry.pass ? "PASS" : "FAIL"} ${STEPS.map((step) => '${step}=${entry.steps[step]?.status ?? "-"}').join(" ")}');`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts:450` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('[artifact-acceptance] ${report.rows.length - failed.length}/${report.rows.length} artifacts pass${report.fatal ? '; fatal: ${report.fatal}' : ""}${failed.length ? '; failing: ${failed.join(", ")}' : ""}');`

### `✏️s/🔌️plugins/🗒️note/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🗒️note/🏭️bridge/🦀️.rs:49` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🗒️note/🏭️bridge/🦀️.rs:54` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️ownership/🟦️.ts` (1)

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️ownership/🟦️.ts:56` [console.log] test-path: yes; allocates: no
  - `console.log("note-empty-config ajv=valid foreign=reject json-patch=valid window-isolation=valid document=unchanged");`

### `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/⌨️cli/🦀️.rs` (7)

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/⌨️cli/🦀️.rs:124` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] --out <dir> is required");`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/⌨️cli/🦀️.rs:133` [eprintln!] test-path: no; allocates: yes: .join(, .map(, collect, println!
  - `eprintln!("[generate] no recipe matches {only:?} — known: {}", all.iter().map(|r| r.id).collect::<Vec<_>>().join(", "));`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/⌨️cli/🦀️.rs:140` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {}: mkdir failed: {e}", recipe.id);`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/⌨️cli/🦀️.rs:151` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {}: write {path:?} failed: {e}", recipe.id);`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/⌨️cli/🦀️.rs:156` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[generate] {}: encode {carrier} {label} failed: {e}", recipe.id);`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/⌨️cli/🦀️.rs:163` [eprintln!] test-path: no; allocates: yes: .join(, println!
  - `eprintln!("[generate] {} — {} ({})", recipe.id, recipe.mutation, recipe.carriers.join("+"));`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/⌨️cli/🦀️.rs:284` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[note-oracle-codec] unknown command {other:?} — expected generate | dxf-project | svg-project | pdf-project | dxf-compare | svg-compare | pdf-compare");`

### `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📜️script.ts` (1)

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📜️script.ts:79` [console.log] test-path: no; allocates: yes: ${, .join(, JSON.stringify
  - `console.log(JSON.stringify(failed(probe, 'oracle probe exited ${run.status}', (run.stderr ?? "").trim().split("\n").slice(-6).join("\n"))));`

### `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/🦀️.rs:51` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️testkit/🦀️.rs` (3)

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️testkit/🦀️.rs:326` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{action} settled turn={turn} ms={} census={} pages={lanes:?} fault={fault:?}", started.elapsed().as_millis(), semio_plugin::app::typed_operation_unit_census() - base);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️testkit/🦀️.rs:335` [eprintln!] test-path: yes; allocates: yes: String::from, println!
  - `eprintln!("{action} FAULT page operation={} turn={turn} ms={} census={} pages={lanes:?} fault={}", page.token.operation, started.elapsed().as_millis(), semio_plugin::app::typed_operation_unit_census() - base, String::...`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️testkit/🦀️.rs:344` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{action} turn={turn} ms={} total={} window={} pages={lanes:?} pending={}", started.elapsed().as_millis(), now - base, now - last_census, app.has_pending_typed_operations());`

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs` (1)

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs:136` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("Nakagin world-3d surface presented {} nodes (cap {}) {}", census.nodes, semio_ui_contract::UI_DOCUMENT_NODES, census.report());`

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mutation-latency/🦀️.rs` (3)

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mutation-latency/🦀️.rs:17` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("b44.native {row}");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mutation-latency/🦀️.rs:113` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("b44.emit objects={} deleteOperations={} removed={first}", large.objects.len(), delete_operations.len());`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mutation-latency/🦀️.rs:135` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("b44.turns small={small_objects}objects/{small}turns large={large_objects}objects/{large}turns");`

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️selection-scale/🦀️.rs` (1)

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️selection-scale/🦀️.rs:133` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("b46.panelBody objects={objects} firstRowPresent={rows} continuation={paged} stampsTotal={stamped} bytes={}", to_json_string(&body).len());`

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/💡️suggestion-and-scene-laws/🦀️.rs` (2)

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/💡️suggestion-and-scene-laws/🦀️.rs:312` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/💡️suggestion-and-scene-laws/🦀️.rs:322` [eprintln!] test-path: yes; allocates: yes: .join(, .map(, collect, format!
  - `eprintln!("nakagin lane paging: {}", census.lanes.iter().map(|lane| format!("{}={}leaves@depth{}", lane.key.trim_start_matches(semio_plugin::WORLD3D_SCENE_LANE_KEY_PREFIX), lane.leaves, lane.leaf_depth)).collect::<Vec...`

### `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🕹️roster-and-read-laws/🦀️.rs` (1)

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🕹️roster-and-read-laws/🦀️.rs:93` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("local interaction read {request_id} turns={turns} bytes={}", capture.len());`

### `✏️s/🔌️plugins/🧱️block/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🧱️block/🏭️bridge/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🧱️block/🏭️bridge/🦀️.rs:51` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:377` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("block2d {id}: live maintenance settled in {turns} turns");`

### `✏️s/🔌️plugins/🪐️space/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🪐️space/🏭️bridge/🦀️.rs:42` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🪐️space/🏭️bridge/🦀️.rs:47` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/📜️script.ts` (1)

- `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/📜️script.ts:7` [console.log] test-path: no; allocates: no
  - `run(): void { console.log("[TRACE] space ts ok"); }`

### `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏭️bridge/🦀️.rs:18` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mutation-inventory] units={} bytes={}", progress.completed_units, progress.emitted_bytes);`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏭️bridge/🦀️.rs:30` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error) = run() { eprintln!("[mutation-inventory] {error}"); std::process::exit(1); }`

### `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏭️bridge/🦀️.rs:18` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mutation-inventory] units={} bytes={}", progress.completed_units, progress.emitted_bytes);`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏭️bridge/🦀️.rs:30` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error) = run() { eprintln!("[mutation-inventory] {error}"); std::process::exit(1); }`

### `✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/🦀️.rs` (2)

- `✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/🦀️.rs:45` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/📜️script.ts` (1)

- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/📜️script.ts:12` [console.log] test-path: no; allocates: no
  - `console.log("sourcing ts ok"); }`

### `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (2)

- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:414` [[DEBUG]] test-path: yes; allocates: no
  - `app.0.mounted_owner_maintenance_step(&mut turn).expect("[DEBUG] a live-cleanup maintenance unit never faults");`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:441` [[DEBUG]] test-path: yes; allocates: yes: .map(, format!, println!
  - `println!("[DEBUG] min-availability {arguments} -> {:?}", outcome.as_ref().map(|_| ()).map_err(|fault| format!("{fault:?}")));`

### `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/📜️script.ts` (1)

- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/📜️script.ts:20` [console.log] test-path: no; allocates: yes: ${, .join(
  - `console.log('Session bindings preview: ${files.length} files under ${join(this.root, "🕸️bindings")}: ${files.join(", ")}');`

### `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🌐️browser/🟦️.ts` (1)

- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🌐️browser/🟦️.ts:53` [console.log] test-path: yes; allocates: no
  - `console.log("Semio geometry browser: two portable volumes, independent lifetime, bounded close receipts, cancellation/resume, zero-grant authority preservation, repeated close and terminal refusal passed");`

### `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🎒️mesh-session/🦀️.rs` (1)

- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🎒️mesh-session/🦀️.rs:78` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🧪️tests/🔬️unit/🦀️.rs` (2)

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🧪️tests/🔬️unit/🦀️.rs:431` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("PCG canonical {kind}: turns={turns}, pages={}, delivered_again={delivered_again}, closed={closed}", pages.as_ref().map_or(0, Vec::len));`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🧪️tests/🔬️unit/🦀️.rs:676` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("PCG restore cancellation: {turns} cuts, fields={fields:#x}, failures={}", failures.len());`

### `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🔬️unit/🦀️.rs:307` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("numerical page {} complete={complete}, fault={fault:?}, cursor={cursor:?}, pages={pages:?}, released={released}", row["id"]);`

### `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/🧪️tests/🔬️unit/🦀️.rs` (1)

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🗂️model/🧪️tests/🔬️unit/🦀️.rs:106` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("mounted model {kind}/{target_name}: allocations={allocations}, allocated={allocated}, released={released}, terminal={terminal}");`

### `✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🏷️ownership/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🏷️ownership/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Flow composition ownership: ${laws.reduce((sum, group) => sum + group.laws.length, 0)} exact native laws');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/📦️packages/🦀️rust/📜️script.ts` (2)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/📦️packages/🦀️rust/📜️script.ts:24` [console.log] test-path: no; allocates: yes: ${
  - `const laws=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd:this.repoRoot...`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/📦️packages/🦀️rust/📜️script.ts:25` [console.log] test-path: no; allocates: yes: ${
  - `console.log('native-service-runtime: exact=${laws.length} passed');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🌉️mcp-composition/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🌉️mcp-composition/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('services-mcp-composition: physical=${fixture.cases.length} independentAst=true independentAjv=true modules=${fixture.cases.length} neutral-canary=true');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💬️agent-reply/🟦️.ts` (5)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💬️agent-reply/🟦️.ts:55` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('${state.toUpperCase().padEnd(4)} ${step} :: ${detail}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💬️agent-reply/🟦️.ts:160` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('\n=== ${label}\n    ${entry.command} ${entry.args.join(" ")}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💬️agent-reply/🟦️.ts:190` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('  (timed out waiting for ${label} after ${budgetMs}ms)');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💬️agent-reply/🟦️.ts:332` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('\nscreenshot → ${SCREENSHOT}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💬️agent-reply/🟦️.ts:341` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('\nos-mcp-agent-reply: ${outcomes.length - failed - skipped} passed, ${failed} failed, ${skipped} skipped of ${outcomes.length}');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💼️inference-quartet/🟦️.ts` (2)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💼️inference-quartet/🟦️.ts:53` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('${ok ? "PASS" : "FAIL"}  [${at()}] ${step} — ${detail}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💼️inference-quartet/🟦️.ts:258` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('inference-quartet: ${green}/${rows.length} rows green against ${ORIGIN}');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🔌️service-composition/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🔌️service-composition/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('native-service-composition: cargo=1 real-owner=${fixture.services.length} generic-empty=1 passed');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🚶️user-path/🟦️.ts` (3)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🚶️user-path/🟦️.ts:58` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('${ok ? "PASS" : "FAIL"}  [${seconds()} s] ${step} — ${detail}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🚶️user-path/🟦️.ts:193` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('INFO [${seconds()} s] the agent pane is bound to another space at withdrawal time; the user reopens the space from the hub list to manage its agents');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🚶️user-path/🟦️.ts:232` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('user-path (${LOCALE}): ${green}/${rows.length} rows green in ${seconds()} s');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🛡️security/🟦️.ts` (2)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🛡️security/🟦️.ts:59` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('${ok ? "PASS" : "FAIL"}  [${seconds()} s] ${step} — ${detail}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🛡️security/🟦️.ts:207` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('security: ${green}/${rows.length} rows green in ${seconds()} s');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️hub-agent-participant/🟦️.ts` (2)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️hub-agent-participant/🟦️.ts:70` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('${ok ? "PASS" : "FAIL"}  ${step} — ${detail}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️hub-agent-participant/🟦️.ts:98` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('hub-agent-participant-check: ${rows.length - red.length}/${rows.length} rows green against ${ORIGIN}');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts` (7)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts:165` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('${state.toUpperCase().padEnd(4)} ${step} :: ${detail}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts:272` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('\n=== ${label}\n    ${entry.command} ${entry.args.join(" ")}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts:304` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('  (timed out waiting for ${label} after ${budgetMs}ms)');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts:747` [console.log] test-path: yes; allocates: yes: ${, .join(, .map(
  - `console.log('\n  server-initiated requests seen: ${gateway.serverRequests.map((row) => row.method).join(", ") || "(none)"}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts:753` [console.log] test-path: yes; allocates: no
  - `console.log("\n=== transcript");`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts:754` [console.log] test-path: yes; allocates: yes: ${
  - `for (const outcome of outcomes) console.log('  ${outcome.state.toUpperCase().padEnd(4)}  ${outcome.step}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts:757` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('\nos-mcp-live-agent-loop: ${outcomes.length - failed - skipped} passed, ${failed} failed, ${skipped} skipped of ${outcomes.length}');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🧩️plugin-coverage/🟦️.ts` (3)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🧩️plugin-coverage/🟦️.ts:69` [console.log] test-path: yes; allocates: yes: ${, new Date
  - `const log = (line: string): void => console.log('${new Date().toISOString()} ${line}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🧩️plugin-coverage/🟦️.ts:241` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('${header}\n${table.join("\n")}');`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🧩️plugin-coverage/🟦️.ts:395` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('${header}\n${table.join("\n")}');`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🔁️peer-rebootstrap/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🔁️peer-rebootstrap/🟦️.ts:159` [console.log] test-path: yes; allocates: no
  - `console.log("gis-map-peer-rebootstrap: clients=2 production-bootstrap=1 scene=controlled simultaneous=0");`

### `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🗺️document-open/↩️undo-owner-phases/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🗺️document-open/↩️undo-owner-phases/🟦️.ts:190` [console.log] test-path: yes; allocates: no
  - `console.log(`

### `✏️s/🧑‍💻dev/🎭️variants/📕️stdio-xlsx/🧪️tests/🚪️route/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🎭️variants/📕️stdio-xlsx/🧪️tests/🚪️route/🟦️.ts:66` [console.log] test-path: yes; allocates: no
  - `console.log("Selected Stdio XLSX route agrees with TOML and binds every XLSX variant to the owned development contribution");`

### `✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf/🧪️tests/🚪️route/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf/🧪️tests/🚪️route/🟦️.ts:93` [console.log] test-path: yes; allocates: no
  - `console.log("Selected Stdio PDF route agrees with TOML and opens the exact PDF 1.7 artifact through the product catalog");`

### `✏️s/🧑‍💻dev/🏗️fem/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (2)

- `✏️s/🧑‍💻dev/🏗️fem/🧪️tests/🔬️typescript-package-commands/🟦️.ts:35` [console.log] test-path: yes; allocates: no
  - `console.log("[verify interactivity tool-jobs p6i] behavioural visual laws passed.");`
- `✏️s/🧑‍💻dev/🏗️fem/🧪️tests/🔬️typescript-package-commands/🟦️.ts:41` [console.log] test-path: yes; allocates: no
  - `console.log("[verify interactivity tool-jobs p6h] behavioural microcursor laws passed.");`

### `✏️s/🧑‍💻dev/💡️services/📦️packages/🦀️rust/📜️script.ts` (1)

- `✏️s/🧑‍💻dev/💡️services/📦️packages/🦀️rust/📜️script.ts:14` [console.log] test-path: no; allocates: no
  - `await runRepositoryCommand(this.invocation,"cargo",["check", "--locked", "--offline", "--manifest-path", manifest, "--all-targets"],this.root,"s-dev-native-check",nativeCommandPolicyV1({maximumBytes:134217728,maximumL...`

### `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/📜️script.ts` (1)

- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/📜️script.ts:16` [console.log] test-path: no; allocates: yes: ${, .join(
  - `class PreviewGeneratedScript extends BundleScript { async run():Promise<void> { const files=["cad_geometry.js","cad_geometry_bg.wasm","cad_geometry.d.ts"]; console.log('CAD geometry bindings preview: ${files.length} f...`

### `✏️s/🧑‍💻dev/📜️script.ts` (6)

- `✏️s/🧑‍💻dev/📜️script.ts:40` [console.log] test-path: no; allocates: no
  - `console.log("[s-dev] checked retained Semio/Jack graph composition");`
- `✏️s/🧑‍💻dev/📜️script.ts:90` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[mcp-client-e2e] workspace folder ${folder}');`
- `✏️s/🧑‍💻dev/📜️script.ts:92` [console.log] test-path: no; allocates: yes: ${
  - `for(const step of steps)console.log('${step.ok?"PASS":"FAIL"}  ${step.step} — ${step.detail}');`
- `✏️s/🧑‍💻dev/📜️script.ts:94` [console.log] test-path: no; allocates: yes: ${
  - `console.log('mcp-client-e2e: ${steps.length-failed.length}/${steps.length} steps green over the two real .mcp.json servers.');`
- `✏️s/🧑‍💻dev/📜️script.ts:111` [console.log] test-path: no; allocates: no
  - `console.log("[s-dev] stdio format claims are current");`
- `✏️s/🧑‍💻dev/📜️script.ts:116` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[s-dev] generated ${STDIO_FORMAT_CLAIMS_PATH}');`

### `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🏃️artifact-commands/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🏃️artifact-commands/🟦️.ts:22` [console.log] test-path: no; allocates: yes: ${
  - `console.log('semio member store producer source: ${verifySemioMemberStoreProducerSource(this.repoRoot)} self-tests passed.');`

### `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🕸️artifact-graph/🟦️.ts` (2)

- `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🕸️artifact-graph/🟦️.ts:75` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[stdio-package-contract] Cargo metadata packages=${contract.packages.length}');`
- `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🕸️artifact-graph/🟦️.ts:115` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[stdio-package-contract] Build graph rust=${contract.packages.length} typescript=${contract.packages.length}');`

### `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🛂️artifact-contract/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🛂️artifact-contract/🟦️.ts:81` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[stdio-package-contract] AJV accepted=${fixture.accepted.length + 1} rejected=${fixture.rejected.length}');`

### `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts` (3)

- `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts:75` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[stdio-package-contract] source packages=${contract.packages.length} dag=valid');`
- `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts:105` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[stdio-package-contract] immutable pin fixtures=${fixture.cases.length} recordVersion=${manifest.version} graphVersion=${manifest.graph.version}');`
- `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts:155` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[stdio-command-ownership] owners=${fixture.owners.length} routers=${fixture.routers.length} artifacts=${fixture.artifactDefinitions.length} commands=${compositionRouter.commands.length}');`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🎬️sequence-runtime/🪟️windows/🦀️.rs` (1)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🎬️sequence-runtime/🪟️windows/🦀️.rs:137` [eprintln!] test-path: yes; allocates: yes: println!
  - `if let Err(error) = &outcome { eprintln!("[TRACE] Sequence exact-window runtime failure before close: {error}"); }`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️rust-package-commands/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${
  - `await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0), invocation: context.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: build...`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🎚️slider-values/🦀️.rs` (2)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🎚️slider-values/🦀️.rs:272` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🎚️slider-values/🦀️.rs:284` [eprintln!] test-path: yes; allocates: yes: .join(, println!
  - `eprintln!("handed on to {owner}: {}", row_faults.join(" | "));`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs` (1)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs:164` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("interactionSelect published: effects={}", settled.requested_effects.len());`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs` (1)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs:89` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("run hops {}: attached={:?} hops={:?}", case.id, view.window_instances.iter().map(|window| window.id.as_str()).collect::<Vec<_>>(), receipt.hop_windows);`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/📈️hot-path-and-fault-laws/🦀️.rs` (1)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/📈️hot-path-and-fault-laws/🦀️.rs:118` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🔁️extension-round-trip/🦀️.rs` (2)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🔁️extension-round-trip/🦀️.rs:173` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[STATS] shell burst rendering {body_key}");`
- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🔁️extension-round-trip/🦀️.rs:272` [eprintln!] test-path: yes; allocates: yes: println!
  - `/// per-tick 'eprintln!' boot #7 counted 46 times (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs` (1)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs:41` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("interactionSelect reserved work items={} effects={}", ArtifactRetainedWorkCapacity::for_invertible_items(1).work_items(), settled.requested_effects.len());`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🕹️interaction-scope/🦀️.rs` (1)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🕹️interaction-scope/🦀️.rs:106` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[SCOPE] {} {} windows={windows:?} panels={panels:?} measures={measures}", role.id, row.verb);`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️eval-chain/🦀️.rs` (1)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️eval-chain/🦀️.rs:87` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("viewer run hops {}: attached={:?} hops={:?}", case.id, view.window_instances.iter().map(|window| window.id.as_str()).collect::<Vec<_>>(), run.hop_windows);`

### `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs` (1)

- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs:150` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("UnlinkedFlowExtensions: the process-wide registry was not restored: {reason}");`

### `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔗️browser-contribution/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔗️browser-contribution/🟦️.ts:30` [console.log] test-path: yes; allocates: no
  - `console.log("Puzzle browser contribution: actual owner entry, package, engine and independent compiler/schema authority passed");`

### `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔬️typescript-package-commands/🟦️.ts:17` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[owner-policy] Puzzle fill checks=${checks}');`

### `✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🏷️ownership/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🏷️ownership/🟦️.ts:138` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('s-dev-composition-source: neutral entries=${entries.length}, isolated neutral runtimes=2, independent AST/esbuild/Cargo owners, GIS/Puzzle deployment source passed');`

### `✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🔄️lifecycle/🟦️.ts` (1)

- `✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🔄️lifecycle/🟦️.ts:71` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('s-dev-composition: ${fixture.cases.length} complete portable lifecycle vectors passed');`

### `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️fleet-example-sweep/🦀️.rs` (5)

- `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️fleet-example-sweep/🦀️.rs:294` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dsl-fixture-sweep] {} example dir(s), {} .semio fixture file(s) found, {} law-check(s) run across {} registered app kind(s), {} unmapped fixture(s)", dirs.len(), fixture_files.len(), walked, registry.len()...`
- `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️fleet-example-sweep/🦀️.rs:296` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dsl-fixture-sweep] unmapped fixtures (no registered ArtifactDsl app matches this extension — not counted as a failure):");`
- `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️fleet-example-sweep/🦀️.rs:298` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("  {entry}");`
- `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️fleet-example-sweep/🦀️.rs:335` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dsl-fixture-sweep] soft-skip example coverage {}: missing {}/ with ≥1 .semio — mid-migration ({})", slug.display(), ASSETS_DIR_NAME, legacy_hint);`
- `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️fleet-example-sweep/🦀️.rs:340` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dsl-fixture-sweep] example asset coverage: {migrated} slug(s) on new 🖼️assets layout, {soft_skipped} soft-skipped mid-migration");`

### `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️ownership/🟦️.ts` (2)

- `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️ownership/🟦️.ts:62` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('[dsl-fixture-sweep] exact native fleet receipt: ${JSON.stringify(evidence)}; assertions=${receipt.assertions}; executable=${receipt.sha256}; evidence=${receipt.artifactDir}');`
- `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️ownership/🟦️.ts:203` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('[dsl-fixture-sweep] ownership oracle: ${laws.length} fleet laws, ${dependencies.length} Cargo edges, ${fixture.cases.length} hostile ownership cases; ${inventory.directories.length} example directories, $...`

### `✏️s/🧪️tests/🏘️plugin-composition/🌀️procedural-formats/🗿️artifact-surface/🟦️.ts` (1)

- `✏️s/🧪️tests/🏘️plugin-composition/🌀️procedural-formats/🗿️artifact-surface/🟦️.ts:179` [console.log] test-path: yes; allocates: no
  - `console.log(`

### `🧰️framework/📦️packages/🟦️typescript/📜️script.ts` (1)

- `🧰️framework/📦️packages/🟦️typescript/📜️script.ts:360` [console.log] test-path: no; allocates: yes: ${
  - `console.log('retained UI native strip-only oracle: ${result.laws} fixture laws, ${result.grants} grants, ${result.retired} one-time retirements, ${result.cancellations} table cancellation, ${result.validationCancellat...`

### `🧰️framework/📦️packages/🦀️rust/📜️script.ts` (2)

- `🧰️framework/📦️packages/🦀️rust/📜️script.ts:257` [console.log] test-path: no; allocates: yes: ${
  - `console.log('framework typescript mirror refreshed -> ${outPath}');`
- `🧰️framework/📦️packages/🦀️rust/📜️script.ts:285` [console.log] test-path: no; allocates: no
  - `console.log("framework typescript mirror is fresh.");`

### `🧰️framework/🔨️modules/⏪️time-travel/🧪️testing/🧾️report/🟦️.ts` (1)

- `🧰️framework/🔨️modules/⏪️time-travel/🧪️testing/🧾️report/🟦️.ts:23` [console.log] test-path: no; allocates: no
  - `console.log(row);`

### `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧪️tests/🧪️contention/🦀️.rs` (2)

- `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧪️tests/🧪️contention/🦀️.rs:158` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧪️tests/🧪️contention/🦀️.rs:182` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("process overrun counters recorded={recorded} sustained_quarantines={quarantines}");`

### `🧰️framework/🔨️modules/⏳️async/🎟️lease-pool/🟦️.ts` (1)

- `🧰️framework/🔨️modules/⏳️async/🎟️lease-pool/🟦️.ts:58` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] ${label} evicted ${key}');`

### `🧰️framework/🔨️modules/⏳️async/📏️api/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/⏳️async/📏️api/🧪️tests/🟦️.ts:72` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const source = "import * as api from " + JSON.stringify(entries[1]) + ";const scheduler=api.createContinuationScheduler();let calls=0;const cancel=scheduler.schedule(()=>calls++,0);cancel();await scheduler.yieldContin...`

### `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📜️script.ts` (1)

- `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📜️script.ts:19` [console.log] test-path: no; allocates: no
  - `console.log(`

### `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts` (9)

- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts:59` [console.log] test-path: yes; allocates: yes: ${
  - `const receipts = await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: ...`
- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts:60` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `for (const receipt of receipts) console.log('worker-maintenance-native-receipt: ${JSON.stringify(receipt)}');`
- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts:93` [console.log] test-path: yes; allocates: yes: ${
  - `const receipts = await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: ...`
- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts:94` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `for (const receipt of receipts) console.log('worker-parking-native-receipt: ${JSON.stringify(receipt)}');`
- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts:124` [console.log] test-path: yes; allocates: yes: ${
  - `const receipts = await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: ...`
- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts:125` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `for (const receipt of receipts) console.log('worker-deferred-wake-native-receipt: ${JSON.stringify(receipt)}');`
- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts:162` [console.log] test-path: yes; allocates: yes: ${
  - `const receipts = await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: ...`
- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts:163` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `for (const receipt of receipts) console.log('worker-pool-use-native-receipt: ${JSON.stringify(receipt)}');`
- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️rust-package-commands/🟦️.ts:172` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('framework-async typescript mirror refreshed -> ${outPath}');`

### `🧰️framework/🔨️modules/⏳️async/🪃️continuation/🧪️tests/🔬️node-twin/🟦️.ts` (3)

- `🧰️framework/🔨️modules/⏳️async/🪃️continuation/🧪️tests/🔬️node-twin/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('  ✓ ${testCase.name}');`
- `🧰️framework/🔨️modules/⏳️async/🪃️continuation/🧪️tests/🔬️node-twin/🟦️.ts:56` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('  ⏱️ ${CHAIN_HOPS} chained zero-delay hops on the real loop: ${chainMs} ms (a throttled nested-setTimeout chain: ~${throttledMs} ms)');`
- `🧰️framework/🔨️modules/⏳️async/🪃️continuation/🧪️tests/🔬️node-twin/🟦️.ts:63` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('🪞️ node twin: ${suite.cases.length} cases held on the virtual clock AND on the real event loop.');`

### `🧰️framework/🔨️modules/◻️2d/🧮️compute/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/◻️2d/🧮️compute/🧪️tests/🟦️.ts:52` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const script = "const rows=" + JSON.stringify(corpus.keys) + ",invalid="+JSON.stringify(corpus.invalidIdentities)+";console.log(JSON.stringify({keys:rows.map(row=>{const id=Buffer.from(row.engineId,'utf8'),input=Buffe...`
- `🧰️framework/🔨️modules/◻️2d/🧮️compute/🧪️tests/🟦️.ts:124` [console.log] test-path: yes; allocates: no
  - `console.log("[compute-schema] " + row.id + ": " + output.length + " reference turns");`

### `🧰️framework/🔨️modules/⚙️runtime/⚖️law/🟦️.ts` (2)

- `🧰️framework/🔨️modules/⚙️runtime/⚖️law/🟦️.ts:73` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ scanned: report.scanned, findings: report.findings, violations: report.violations.length }));`
- `🧰️framework/🔨️modules/⚙️runtime/⚖️law/🟦️.ts:74` [console.log] test-path: no; allocates: no
  - `for (const row of report.violations.slice(0, 50)) console.log("VIOLATION", row);`

### `🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🛠️prepare/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🛠️prepare/🧪️tests/🟦️.ts:88` [console.log] test-path: yes; allocates: yes: ${, .join(, JSON.stringify
  - `const probe = spawnSync(process.versions.node === undefined ? "node" : process.execPath, ["-e", 'const m={exports:{}};process.dlopen(m,${JSON.stringify(expected.file)});console.log(Object.keys(m.exports).sort().join()...`

### `🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🛑️termination/🟦️.ts` (1)

- `🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🛑️termination/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const child = spawn(process.execPath, ["-e", 'import(${JSON.stringify(module)}).then(({onTerminationV1})=>{onTerminationV1(signal=>{console.log("stopped:"+signal);process.exit(0);});console.log("ready");setInterval(()...`

### `🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🟦️.ts:261` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `writeFileSync(join(root, "probe.ts"), 'import { isMainModuleV1 } from ${JSON.stringify(facade)};\nconsole.log(JSON.stringify(isMainModuleV1(import.meta.url)));\n');`
- `🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🟦️.ts:416` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `writeFileSync(file, 'const f = await import(${JSON.stringify(new URL("../🟦️.ts", import.meta.url).href)});\nconst id = f.runtimeIdentityV1();\nlet before; try { f.selectRuntimeAdapterV1(id.id); before = "loaded"; } ca...`

### `🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🧫️fixtures/🔌️probe-plugin.ts` (1)

- `🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🧫️fixtures/🔌️probe-plugin.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ hello: main.hello, name: main.name, answer: main.answer, call: main.call(), object: main.object }));`

### `🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/💾️resident/🦀️.rs` (2)

- `🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/💾️resident/🦀️.rs:245` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/💾️resident/🦀️.rs:469` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("resident first-access root={after_root} first={} second={} layouts={events:?}", after_first - after_root, after_second - after_first);`

### `🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🔎️traversal/🧪️tests/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🔎️traversal/🧪️tests/🦀️.rs:30` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `let mut child=std::process::Command::new("repo").args(["tools","exec","bun","--","-e",r#"import Ajv from "ajv/dist/2020";import{Database}from"bun:sqlite";const{law,schema}=JSON.parse(await Bun.stdin.text());const v=ne...`

### `🧰️framework/🔨️modules/🌱️value/🦀️.rs` (7)

- `🧰️framework/🔨️modules/🌱️value/🦀️.rs:56` [[DEBUG]] test-path: no; allocates: no
  - `/// 🐛️ [DEBUG] Temporary thread-local unit histogram that names the dominant publication phases of the 3D mutation-latency regression; remove it with its call sites once the ladder is measured.`
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs:59` [[DEBUG]] test-path: no; allocates: no
  - `/// 🏷️ [DEBUG] Every counted label in print order; an unknown label counts as the last one.`
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs:62` [[DEBUG]] test-path: no; allocates: no
  - `/// ➕️ [DEBUG] Adds 'count' under 'label' on this thread without allocating.`
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs:64` [[DEBUG]] test-path: no; allocates: no
  - `/// 🔢️ [DEBUG] Counts one unit under 'label' on this thread.`
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs:66` [[DEBUG]] test-path: no; allocates: no
  - `/// 🧹️ [DEBUG] Clears this thread's histogram.`
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs:68` [[DEBUG]] test-path: no; allocates: no
  - `/// 🖨️ [DEBUG] Renders this thread's nonzero counts as '[DEBUG] label=count ...'.`
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs:69` [[DEBUG]] test-path: no; allocates: yes: .filter(, String::from, format!
  - `pub fn debug_units_report()->String{DEBUG_UNITS.with(|units|DEBUG_UNIT_LABELS.iter().zip(units.iter()).filter(|(_,unit)|unit.get()!=0).fold(String::from("[DEBUG]"),|report,(label,unit)|format!("{report} {label}={}",un...`

### `🧰️framework/🔨️modules/🌱️value/🧪️tests/🧩️neutral-owner/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🌱️value/🧪️tests/🧩️neutral-owner/🟦️.ts:142` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const oracle = spawnSync("node", ["--input-type=module", "-e", 'import {lstatSync} from 'node:fs';import {isAbsolute,dirname} from 'node:path';const rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map...`
- `🧰️framework/🔨️modules/🌱️value/🧪️tests/🧩️neutral-owner/🟦️.ts:165` [console.log] test-path: yes; allocates: no
  - `console.log("value:products-absent fresh-captured-units=value,value-derive thirdparty-artifacts=caller-owned");`

### `🧰️framework/🔨️modules/🌱️value/🧬️bytes/🧪️tests/🧬️base64/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🌱️value/🧬️bytes/🧪️tests/🧬️base64/🟦️.ts:43` [console.log] test-path: yes; allocates: no
  - `console.log("intrinsic-bytes: neutral="+fixture.cases.length+" independent-base64=12 invalid="+fixture.invalid.length+" allocation-limits=2 cancellation=4 strict-source=1");`

### `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🧪️tests/🔬️unit/🦀️.rs:490` [console.log] test-path: yes; allocates: yes: .map(, Array.from, JSON.stringify
  - `let mut oracle=std::process::Command::new("bun").args(["-e",r#"import Ajv from "ajv";const {schema,law}=JSON.parse(await Bun.stdin.text());const validate=new Ajv({strict:true}).compile(schema);if(!validate(law)||valid...`

### `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🧪️tests/🏭️factory/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🧪️tests/🏭️factory/🦀️.rs:8` [console.log] test-path: yes; allocates: yes: JSON.stringify, json!, serde, to_string(
  - `use std::io::Write;let mut oracle=std::process::Command::new("bun").args(["-e","const x=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify([...new TextEncoder().encode(x.fixture.payload)]));"]).stdin(std::p...`

### `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/✏️edit/🧪️tests/birth-native.ts` (1)

- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/✏️edit/🧪️tests/birth-native.ts:6` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(import.meta.main)console.log(JSON.stringify(birthOracle(JSON.parse(await Bun.stdin.text()))));`

### `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/✏️edit/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/✏️edit/🧪️tests/🔬️unit/🦀️.rs:40` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify, json!, serde
  - `let fixture:serde_json::Value=serde_json::from_str(include_str!("../../♻️custody/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut oracle=std::process::Command::new("repo").args(["--service=off","tools","exec","bun","--"...`

### `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🧪️tests/🔬️unit/🦀️.rs:67` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `let mut oracle=std::process::Command::new("repo").args(["--service=off","tools","exec","bun","--","-e","const f=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify(f.cases.map(c=>Object.fromEntries(c.entries...`

### `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🧪️tests/🔬️unit/🦀️.rs:150` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("retained-value partial-roots=0,1,2 subexact-preserved=true logical-frame-bound={} exact-release=true", usize::from(limits.max_depth) * 8);`

### `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-catalog-laws/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-catalog-laws/🦀️.rs:450` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("retained-pack-catalog page-crossing-offset={position} scalar={} independent-pack-file=true allocated-bytes={allocated}", u32::from('€'));`

### `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws/🟦️.ts:147` [console.log] test-path: yes; allocates: no
  - `console.log("Retained Pack physical source, catalog, value, diagnostic and inflater fixtures agree with Ajv 2020, fast-json-patch and platform UTF-8; history=32768 dynamic=318 distances=1,32768 terminal-ledgers=zero; ...`

### `🧰️framework/🔨️modules/🎠️kernel/📤️return/🏠️source/🧪️tests/🔬️standalone/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🎠️kernel/📤️return/🏠️source/🧪️tests/🔬️standalone/🦀️.rs:23` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("return-source native layout census {inventory}");`

### `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/⬇️media-export-encoding/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/⬇️media-export-encoding/🟦️.ts:138` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('media-export-encoding cases=${fixture.cases.length} refusals=${fixture.refusals.length} binary=${binary.length} textual=${textual.length} oracles=Buffer,atob');`

### `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🎞️video-render-program/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🎞️video-render-program/🟦️.ts:49` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('video-render-program valid=${fixture.valid.length} invalid=${fixture.invalid.length}');`

### `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/📤️file-open-import/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/📤️file-open-import/🟦️.ts:148` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('file-open-import wire=${fixture.wireCases.length} chunks=${fixture.chunkCases.length} arguments=${fixture.argumentCases.length} extent=${IMPORT_CHUNK_BYTES} oracle=Buffer.byteLength');`

### `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧵️spawned-job-drive/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧵️spawned-job-drive/🟦️.ts:212` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('spawned-job-drive transcripts=${fixture.transcripts.length} refusals=${fixture.refusals.length} placements=${JOB_PLACEMENTS.length} ceiling=${SPAWNED_JOB_STEP_CEILING} events=${fixture.completedEvent.case...`

### `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧵️video-render-job/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧵️video-render-job/🟦️.ts:42` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('video-render-job streams=${fixture.streams.length} refusals=${fixture.refusals.length}');`

### `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🦀️.rs:48` [eprintln!] test-path: yes; allocates: yes: println!
  - `GUARD.call_once(|| { std::thread::spawn(|| { std::thread::sleep(std::time::Duration::from_secs(100)); eprintln!("actor-retained-context tests stalled for 100 s"); std::process::exit(101) }); });`

### `🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧪️tests/🧪️cold-pair-wit-status-codec-agrees-with-the-neutral-schema-and-rejects-ev/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧪️tests/🧪️cold-pair-wit-status-codec-agrees-with-the-neutral-schema-and-rejects-ev/🟦️.ts:48` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('cold-pair-status-codec: schema=1 valid=${fixture.statusRows.length} structural-hostile=${structurallyRejected} semantic-hostile=${fixture.hostileRows.length} unknown-fields=refused');`

### `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/🏃️execution/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/🏃️execution/🟦️.ts:73` [console.log] test-path: no; allocates: no
  - `console.log("framework-actor typegen export started");`
- `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/🏃️execution/🟦️.ts:76` [console.log] test-path: no; allocates: yes: ${
  - `console.log('framework-actor typescript mirror refreshed -> ${actorTypegenTarget(this.repoRoot)}');`

### `🧰️framework/🔨️modules/🏃️process/⏱️budget/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/⏱️budget/🧪️tests/🟦️.ts:30` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `const program = 'const rows=JSON.parse(require('node:fs').readFileSync(0,'utf8')).cases;const read=${read.toString()};import(require('node:url').pathToFileURL(process.argv[1]).href).then(api=>console.log(JSON.stringif...`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📣️progress/🛑️interruption/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📣️progress/🛑️interruption/🧪️tests/🟦️.ts:6` [console.log] test-path: yes; allocates: yes: ${
  - `const childCode = 'console.log("[TRACE] interruption child PID "+process.pid);setTimeout(()=>console.log("[TRACE] interruption child complete"),${corpus.childMilliseconds});';`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📣️progress/🛑️interruption/🧪️tests/🟦️.ts:7` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const oracleCode = 'const c=JSON.parse(process.argv[1]),r=JSON.parse(process.argv[2]),code=process.argv[3],started=performance.now(),controller=new AbortController(),parent=setTimeout(()=>controller.abort(),c.callerCe...`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📣️progress/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📣️progress/🧪️tests/🟦️.ts:11` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const oracleCode = 'const c=JSON.parse(process.argv[1]),r=JSON.parse(process.argv[2]);let done=false;const publication=new Promise((resolve,reject)=>setTimeout(()=>setTimeout(()=>{done=true;r.rejectPublication?reject(...`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧪️tests/📜️script.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧪️tests/📜️script.ts:8` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({mode,birthMilliseconds:readIssuedScriptProcessEnvelopeV1(original).policy.maximumElapsedMilliseconds,failed,live,ordinary:observed.stdout?JSON.parse(observed.stdout):null,samePrimary:failur...`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📡️channel/🧪️tests/👷️child/📜️script.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📡️channel/🧪️tests/👷️child/📜️script.ts:5` [console.log] test-path: yes; allocates: yes: .repeat(, JSON.stringify
  - `async function runScriptProgram():Promise<void>{const selected=process.env.SEMIO_OWNED_CHANNEL_CASE;if(!selected||selected==="blocked-close"||!corpus.cases.some(row=>row.case===selected))throw Error("Closed channel So...`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📡️channel/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📡️channel/🧪️tests/🟦️.ts:38` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `expect(terminal,"Actual terminal for "+selected+" child="+childPid+" failure="+String(failure)).toBe(true);console.log("actual-owned-channel:"+JSON.stringify({case:selected,outcome,terminal}));`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/📥️input/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/📥️input/🟦️.ts:15` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `else test("neutral Hub binary input early EOF isolated",async()=>{if(selected!=="early-eof")throw Error("Closed owned input Source case required");const acknowledgement=Error("Actual binary child acknowledgement requi...`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: ${, .join(, JSON.stringify
  - `async function failureChild(mode:string,source:string,maximumBytes:number,maximumLines:number):Promise<Record<string,unknown>>{const routing=fileURLToPath(new URL("../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts",imp...`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📪️outcome/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📪️outcome/🧪️tests/🟦️.ts:13` [console.log] test-path: yes; allocates: no
  - `const child = 'const row=JSON.parse(process.argv[1]);console.log("[TRACE] original "+row.id);console.error("[TRACE] original stderr "+row.id);row.signal?process.kill(process.pid,row.signal):process.exit(row.status)';`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📪️outcome/🧪️tests/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const oracleCode = 'const{spawn}=require("node:child_process"),row=JSON.parse(process.argv[1]);const child=spawn(process.execPath,["--eval",process.argv[2],JSON.stringify(row)],{stdio:["ignore","ignore","ignore"]});ch...`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:169` [console.log] test-path: no; allocates: no
  - `const publication = Promise.resolve().then(() => options.onProgress ? options.onProgress(line) : options.stdout === "stderr" ? console.error(line) : console.log(line));`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/📤️stream/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/📤️stream/🟦️.ts:10` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const controller=new AbortController(),observed:{stream:string;line:string}[]=[],program="process.stdout.write("+JSON.stringify(fixture.stdout)+");process.stderr.write("+JSON.stringify(fixture.stderr)+");",node=Bun.sp...`

### `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts` (4)

- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts:61` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const source = row.mode === "lines" ? "const b=Buffer.from('first 🧪\\nlast');process.stdout.write(b.subarray(0,8));setTimeout(()=>process.stdout.write(b.subarray(8)),10)" : row.mode === "pre-abort" ? 'require('node:fs...`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts:72` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const runner = 'const fs=require('node:fs');const root=process.argv[2];const row=JSON.parse(fs.readFileSync(0,'utf8'));const existsSync=fs.existsSync;const resolve=require('node:path').resolve;const fixture=${JSON.str...`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts:131` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const source = "const {spawn}=require('node:child_process');const child=spawn(process.execPath,['-e','setInterval(()=>{},10000)'],{detached:process.platform!=='win32',stdio:'inherit'});console.log('READY '+JSON.string...`
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts:183` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const oracle="const{spawn}=require('node:child_process');const row=JSON.parse(process.argv[1]);const child=spawn(process.execPath,['-e',process.argv[2],JSON.stringify(row)],{stdio:['ignore','pipe','pipe']});let stdout...`

### `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🧪️tests/⏱️overhead/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🧪️tests/⏱️overhead/🟦️.ts:30` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("[overhead] "+JSON.stringify(report));`

### `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🧪️tests/🟦️.ts:139` [console.log] test-path: yes; allocates: no
  - `if(probe.exitCode!==0){console.log("[skip] python jsonschema is not installed; the repository validator alone checked the records");return;}`

### `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/⚓️boot-anchor/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/⚓️boot-anchor/🟦️.ts:42` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const child=Bun.spawnSync([process.execPath,"-e",'const{hostBootAtV1}=await import(${JSON.stringify(fileURLToPath(new URL("../../🎫️gate/🟦️.ts",import.meta.url)))});console.log(hostBootAtV1(${JSON.stringify(directory)}...`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/⚓️boot-anchor/🟦️.ts:45` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const fresh=Bun.spawnSync([process.execPath,"-e",'const{hostBootAtV1}=await import(${JSON.stringify(fileURLToPath(new URL("../../🎫️gate/🟦️.ts",import.meta.url)))});console.log(hostBootAtV1(${JSON.stringify(directory)}...`

### `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔬️footprint/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔬️footprint/🟦️.ts:159` [console.log] test-path: yes; allocates: yes: Array.from
  - `const script="let big=Array.from({length:3000000},(_,i)=>({i,s:'x'+i}));big=null;Bun.gc(true);Bun.shrink?.();console.log('ready');await new Promise(done=>process.stdin.once('end',done));";`

### `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🟦️.ts:39` [console.log] test-path: yes; allocates: yes: ${, Date.now, JSON.stringify
  - `const body='const start=Date.now();let cycles=0;while(Date.now()-start<${fixture.shared.bodyMs}){for(let i=0;i<100000;i++)cycles=(cycles+i)>>>0;}console.log(JSON.stringify({start,end:Date.now(),parallelism:Number(proc...`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🟦️.ts:79` [console.log] test-path: yes; allocates: yes: ${
  - `const child=spawn(process.execPath,["-e",'console.log("ready");setInterval(()=>{},${law.holdMilliseconds});'],{signal:admission.signal,stdio:["ignore","pipe","pipe"]});child.on("error",()=>undefined);const closed=new ...`

### `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/👷️child/🟦️.ts` (8)

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/👷️child/🟦️.ts:13` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(kind==="sampler-birth-close"){try{await receiveScriptProcessInvocation(process.env,async original=>{const control=scriptCompilerAdmissionControlV1(original,"test:sample-closing-births",10000);await withRequestedIss...`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/👷️child/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `else if(kind==="sampler-cancel"){try{await receiveScriptProcessInvocation(process.env,async original=>{const parent=scriptCompilerAdmissionControlV1(original,"test:sample-cancel",10000),abort=new AbortController(),con...`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/👷️child/🟦️.ts:15` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `else if(kind==="sampler-error"||kind==="sampler-undefined"||kind==="consumer-error"||kind==="consumer-undefined"){try{await receiveScriptProcessInvocation(process.env,async original=>{const control=scriptCompilerAdmis...`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/👷️child/🟦️.ts:16` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `else if(kind==="input-error"||kind==="input-undefined"){try{await receiveScriptProcessInvocation(process.env,async original=>{const control=scriptCompilerAdmissionControlV1(original,"test:compiler-input-failure",10000...`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/👷️child/🟦️.ts:17` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `else if(kind==="available-cancel"){try{await receiveScriptProcessInvocation(process.env,async original=>{const control=scriptCompilerAdmissionControlV1(original,"test:available-cancel",10000),held=await acquireCompile...`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/👷️child/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `else if(kind==="root-drain"){await receiveScriptProcessInvocation(process.env,async original=>{const control=scriptCompilerAdmissionControlV1(original,"test:capacity-root-drain",10000);void withCompilerCapacityScopeV1...`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/👷️child/🟦️.ts:19` [console.log] test-path: yes; allocates: no
  - `if(kind==="cancel"){const cancellation=new AbortController(),phase=createRequestedIssuedResourceControlV1(actual,0,cancellation.signal);try{await withOwnedCommandChannelV1(process.execPath,["-e","console.log(process.p...`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/👷️child/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `});});throw new Error("Actual failed Original unexpectedly succeeded");}catch(error){console.log(JSON.stringify({version:1,kind,exact:Object.is(error,primary),late,refused}));process.exitCode=1;}`

### `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/🟦️.ts:33` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const node=await captureOwnedCommandResultV1("node",["-e","const rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(({parent,requested:r})=>r.kind==='whole-host'?parent:r.kind==='available'?Math.min(...`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/🟦️.ts:47` [console.log] test-path: yes; allocates: no
  - `let sampleActive=0,samplePeak=0,sampleCalls=0,terminal=false;let sampleStarted:()=>void=()=>{};const began=new Promise<void>(resolve=>{sampleStarted=resolve;});const sampler=startTelemetrySamplerV1(control,async actua...`

### `🧰️framework/🔨️modules/🏃️process/🏗️build/📤️output/🧪️tests/🎭️fixture/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🏗️build/📤️output/🧪️tests/🎭️fixture/🟦️.ts:10` [console.log] test-path: yes; allocates: no
  - `console.log("[noise] console.log");`

### `🧰️framework/🔨️modules/🏃️process/💾️state/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/💾️state/🧪️tests/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: .push(, Array.from, JSON.stringify
  - `if(process.env.SEMIO_PERSISTENT_STATE_CHILD!==undefined)test("persistent state isolated actual accepted lifetime",async()=>{const row=corpus.lifetime.find(row=>row.name===process.env.SEMIO_PERSISTENT_STATE_CHILD);if(!...`

### `🧰️framework/🔨️modules/🏃️process/📋️context/🧪️tests/🚫️paths/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📋️context/🧪️tests/🚫️paths/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const code="const fs=require('node:fs'),p=require('node:path'),rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(r=>({id:r.id,valid:['cwd','cacheRoot','leaseDirectory'].every(k=>{if(!p.isAbsolute(r....`

### `🧰️framework/🔨️modules/🏃️process/📥️capture/📥️invocation/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📥️capture/📥️invocation/🧪️tests/🟦️.ts:31` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const oracleCode = String.raw'const c=JSON.parse(process.argv[1]),row=JSON.parse(process.argv[2]),controller=new AbortController(),timers=require("node:timers/promises"),spawn=require("node:child_process").spawn;let p...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs` (6)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs:484` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => { eprintln!("semio-build-scheduler: cannot run {}: {error}", rustc.to_string_lossy()); ExitCode::from(127) }`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs:495` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-build-scheduler: cannot run {}: {error}", rustc.to_string_lossy());`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs:567` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-build-scheduler: expected <rustc> <arguments…>");`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs:571` [eprintln!] test-path: no; allocates: yes: println!
  - `let text = match expand_admission_arguments(&args) { Ok(text) => text, Err(error) => { eprintln!("semio-build-scheduler: {error}"); return ExitCode::from(1); } };`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs:574` [eprintln!] test-path: no; allocates: yes: println!
  - `let (args, copies) = match bounded_argfiles(&args, frontend_tokens(&args, tokens)) { Ok((bounded, copies)) => (serial_backend(bounded, tokens), copies), Err(error) => { eprintln!("semio-build-scheduler: {error}"); ret...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs:577` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-build-scheduler: {error}");`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🧢️unit-ceiling/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🧢️unit-ceiling/🟦️.ts:76` [console.log] test-path: yes; allocates: yes: ${
  - `if(!existsSync(path)){console.log('[unit-ceiling] no ledger at ${path}: skipped');return;}`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🧢️unit-ceiling/🟦️.ts:78` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('[unit-ceiling] ${path}: ${JSON.stringify(verdict)}');`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🧪️tests/🟦️.ts:196` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[jobserver] exclusive ${row.tokens} workers; third-party client peak=${row.maxConcurrent}');`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🧪️tests/🟦️.ts:204` [console.log] test-path: yes; allocates: yes: ${, .join(, Array.from
  - `writeFileSync(join(directory, "Makefile"), 'all: ${Array.from({ length: fixture.gnuMake.recipes }, (_, index) => 't${index}').join(" ")}\nflags:\n\t+@node -e "console.log(process.env.MAKEFLAGS)"\nt%:\n\t@${probe}\n');`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🪟️native/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🪟️native/🧪️tests/🟦️.ts:66` [console.log] test-path: yes; allocates: yes: ${
  - `const viaShell=parseNativeBootEpochV1("win32",shell.stdout);console.log('[xplat2-g] windows boot clock: in-process ${nativeMs.toFixed(2)} ms, PowerShell ${shellMs.toFixed(0)} ms, difference ${direct!-viaShell!} ms');`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🟦️.ts:233` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const holder=Bun.spawn([process.execPath,"-e",'const{Database}=require("bun:sqlite");const d=new Database(${JSON.stringify(path)});d.exec("PRAGMA busy_timeout=0;BEGIN EXCLUSIVE");console.log("held");setInterval(()=>{}...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🩺️heal/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🩺️heal/🟦️.ts:97` [console.log] test-path: yes; allocates: no
  - `const holder=Bun.spawn([process.execPath,"-e","const{spawnSync}=require('node:child_process');const r=spawnSync('/usr/bin/hdiutil',['attach','-nobrowse','-noautoopen','-owners','off','-mountpoint',process.argv[1],proc...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🧪️tests/🛑️sigterm/🏃️child.ts` (3)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🧪️tests/🛑️sigterm/🏃️child.ts:6` [console.log] test-path: yes; allocates: no
  - `const marked=(program:string,args:readonly string[])=>{console.log("run "+(args[0]==="image"?"attach":args[0]));return exec(program,args);};`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🧪️tests/🛑️sigterm/🏃️child.ts:11` [console.log] test-path: yes; allocates: no
  - `const outcome=await reclaimNativeImageV1(port,{check:()=>{if(controller.signal.aborted)throw Error("cancelled");},progress:done=>console.log("progress "+done)}).then(()=>"completed",error=>String(error?.message??error...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🧪️tests/🛑️sigterm/🏃️child.ts:12` [console.log] test-path: yes; allocates: no
  - `console.log("end "+outcome);`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🛑️cleanup/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🛑️cleanup/🟦️.ts:14` [console.log] test-path: yes; allocates: no
  - `try{await withScriptInvocationSignalV1(original,controller.signal,async invocation=>{const control=scriptCompilerAdmissionControlV1(invocation,"image:cleanup:body",60000);await withNativeImageStorageV1({...imageFixtur...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🟦️.ts` (5)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🟦️.ts:29` [console.log] test-path: yes; allocates: no
  - `if(row.test){const executable=requireNativeStoragePathV1(image.root,join(owned,"native-laws"));await captureOwnedCommandV1("rustc",["--crate-name",row.name,"--edition=2024","--test","-o",executable,...profile,source],...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🟦️.ts:41` [console.log] test-path: yes; allocates: no
  - `await allocation.recheck();console.log(await captureOwnedCommandV1("rustc",["--crate-name",name,"--edition=2021","--crate-type=rlib","--emit=metadata,link","--out-dir",owned,...profile,source],workspace,{control,env:e...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🟦️.ts:43` [console.log] test-path: yes; allocates: no
  - `console.log(await captureOwnedCommandV1(test,["--nocapture"],workspace,{control,env:originalNativeLibraryEnvironment(environment,directory)}));if(before!==hash())throw Error("Original identity Source changed during co...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🟦️.ts:94` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const output=await captureOwnedCommandV1(process.execPath,["-e","const fs=require('node:fs');const path=require('node:path');let bytes=0;const fd=fs.openSync(path.join(process.argv[1],'fill'),'wx');const data=Buffer.a...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🟦️.ts:95` [console.log] test-path: yes; allocates: no
  - `if(!output.includes('"code":"ENOSPC"'))throw Error("Real filesystem cap-plus-one did not refuse");console.log(output.trim());await image.recheck();`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🍎️events/🍞️bun/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🍎️events/🍞️bun/🟦️.ts:10` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `if(failed){if(cleanupFailed&&!Object.is(primary,cleanupError))throw new AggregateError([primary,cleanupError],"Service preparation and distinct cleanup failed");throw primary;}if(cleanupFailed)throw cleanupError;if(un...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🟦️.ts:76` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `expect(api.artifactDescriptorCustodyV1()).toEqual({active:0,unresolved:0});console.log("[DATA] Descriptor isolated Source worker "+JSON.stringify({version:1,kind:request.kind,bodyCalls,refusal,nativeEffects:0}));`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪟️conditional-retirement/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪟️conditional-retirement/🧪️tests/🟦️.ts:75` [console.log] test-path: yes; allocates: no
  - `const child=runOwnedCommand(process.execPath,["-e",'import fs from"node:fs";const fd=fs.openSync(process.argv[1],"r");try{console.log("[TRACE] conditional-reader-ready");const end=performance.now()+60000;while(!fs.exi...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📈️telemetry/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📈️telemetry/🧪️tests/🟦️.ts:79` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('[telemetry] critical chain ${ours.join(" → ")}: ours ${lengthMs} ms, cargo --timings ${oracle.lengthMs} ms');`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📣️diagnostics/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📣️diagnostics/🟦️.ts:13` [console.log] test-path: no; allocates: no
  - `export function cargoProvenanceConsoleSinkV1(kind:"provenance"|"recipe",environment:Readonly<Record<string,string|undefined>>=process.env,output:(line:string)=>void=line=>console.log(line)):(row:Readonly<{bytes:number...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📣️diagnostics/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📣️diagnostics/🧪️tests/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: .push(
  - `const printed: string[] = [], spy = spyOn(console, "log").mockImplementation(((line: unknown) => { printed.push(String(line)); }) as typeof console.log);`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/⚙️operation/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/⚙️operation/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const message=failure instanceof Error?failure.message:"";const preserved=row.preserved==="value"?!failed:row.preserved==="undefined"?failed&&failure===undefined:row.preserved==="marker"?failure===marker:row.preserved...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔐️custody/🟦️.ts` (6)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔐️custody/🟦️.ts:19` [console.log] test-path: yes; allocates: no
  - `const take=()=>acquireCargoBuildLeaseV1({directory:leases,buildDirectory:build,targetDirectory:target,args:["--profile",profile],signal:controller.signal,onWait:()=>console.log("waiting")});`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔐️custody/🟦️.ts:23` [console.log] test-path: yes; allocates: no
  - `if(word==="stop")for(const child of children.splice(0)){child.stdin.write("release\\n");child.stdin.end();await child.exited;console.log("child-gone");}`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔐️custody/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: .push(
  - `if(word==="again"){owned.push(await take());console.log("again-held");}`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔐️custody/🟦️.ts:25` [console.log] test-path: yes; allocates: no
  - `if(word==="drop"){owned.shift().release();console.log("dropped");}}});`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔐️custody/🟦️.ts:28` [console.log] test-path: yes; allocates: no
  - `try{console.log(mode==="child"?"child-held":"held");await finished;}finally{for(const held of owned.splice(0))held.release();console.log("released");}}`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🔐️custody/🟦️.ts:29` [console.log] test-path: yes; allocates: no
  - `catch(error){if(controller.signal.aborted)console.log("cancelled");else throw error;}finally{clearTimeout(timeout);process.stdin.destroy();}`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🟦️.ts` (4)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🟦️.ts:32` [console.log] test-path: yes; allocates: no
  - `const options={directory,buildDirectory,targetDirectory,args:["--profile",profile],signal:controller.signal,onWait:()=>console.log("waiting")};`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🟦️.ts:34` [console.log] test-path: yes; allocates: no
  - `try { console.log("held"); await finished; } finally { lease.release(); console.log("released"); }`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🟦️.ts:35` [console.log] test-path: yes; allocates: no
  - `} catch (error) { if (controller.signal.aborted) console.log("cancelled"); else throw error; }`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🟦️.ts:41` [console.log] test-path: yes; allocates: no
  - `const argv=access==="oracle"?[runtime,"--input-type=module","-e","import {DatabaseSync} from \"node:sqlite\";const d=new DatabaseSync(process.argv[1]);try{let denied=false;try{d.exec(\"BEGIN EXCLUSIVE\")}catch(e){if(!...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts:129` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[publish] ${tarball}\n[publish] sha256 ${digest}');`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🎟️stage/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🎟️stage/🧪️tests/🟦️.ts:16` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `async function verifyCompilerInputLifetimeV1(directory:string,control:import("../../../../../🏗️build/🎟️capacity/🟦️.ts").CompilerAdmissionControl):Promise<void>{const{withCompilerCapacityConsumptionV1}=await import(".....`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🔁️failure-churn/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🔁️failure-churn/🧪️tests/🟦️.ts:52` [console.log] test-path: yes; allocates: no
  - `const child=spawn(process.execPath,["-e","console.log('ready'); setTimeout(() => process.exit(0), 4000);"],{detached:process.platform!=="win32",stdio:["ignore","pipe","pipe"]});`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts:467` [console.log] test-path: no; allocates: no
  - `const plan = admitNativeBuildPlanV1(options.plan), policy = NATIVE_BUILD_DRIVER_POLICY_V1, log = options.log ?? ((line: string) => console.log(line)), jobs = plan.jobs ?? [], budget = compilerAdmissionBudgetV1(options...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts` (11)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:337` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(process.argv.includes("--unit-graph")){console.log(JSON.stringify({version:1,units,roots:[0,2]}));process.exit(0);}`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:345` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({reason:"compiler-artifact",package_id:unit.pkg_id,manifest_path:path(unit.target.name,"Cargo.toml"),target:unit.target,features:unit.features,profile:{test:false},filenames:[file],executabl...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:353` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({reason:"compiler-message",package_id:units[0].pkg_id,manifest_path:app,target:units[0].target,message:{level:"error",code:{code:first?"E0599":secondCode},message:"selected source changed",s...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:479` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(process.argv.includes("--unit-graph")){console.log(JSON.stringify({version:1,units,roots:[1]}));process.exit(0);}`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:492` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({reason:"compiler-artifact",package_id:unit.pkg_id,manifest_path:packages[index].manifest,target:unit.target,features:[],profile:{test:false},filenames:[file],executable:null,fresh:attempt>0...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:861` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[component-pass] real release: ${units.units.length} unit(s), ${outcome.staged.length} component(s), report ${outcome.report}');`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:906` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[component-pass] reserved ${reserved.parallelism} workers; shared successor and Cargo completed');`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:928` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[component-pass] exclusive jobs=2 stages=2 workers=${fixture.stub.tokens}; shared Cargo completed');`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:1134` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `writeFileSync(script,'import {runNativeBuildDriverV1} from ${JSON.stringify(source)};import {withLawStorageV1,lawStoragePolicyV1} from ${JSON.stringify(new URL("../../🧪️tests/🦀️cargo/🟦️.ts",import.meta.url).href)};imp...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:1306` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(process.argv.includes("--unit-graph")){console.log(JSON.stringify({version:1,units,roots:vector.roots}));process.exit(0);}`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts:1308` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const emit=(message)=>console.log(JSON.stringify(message));`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🟦️.ts:83` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `if(process.env.SEMIO_CARGO_RETENTION_CHILD!==undefined)test("cargo retention isolated actual input continuation",async()=>{const row=corpus.continuations.find(row=>row.name===process.env.SEMIO_CARGO_RETENTION_CHILD);i...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🟦️.ts:105` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `if(process.env.SEMIO_CARGO_RETENTION_GRAPH_CHILD!==undefined)test("cargo retention isolated actual graph drainage",async()=>{const row=corpus.graphOperations.find(row=>row.name===process.env.SEMIO_CARGO_RETENTION_GRAP...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🧪️tests/📥️selection/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🧪️tests/📥️selection/🟦️.ts:33` [console.log] test-path: yes; allocates: no
  - `const program='const fs=require("node:fs"),fd=fs.openSync(process.env.SEMIO_SELECTED_DESCRIPTOR_FILE,"r");console.log("ready");process.stdin.once("data",()=>{fs.closeSync(fd);process.stdin.destroy();});process.stdin.r...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts` (5)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts:13` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `print(json.dumps(out,separators=(',',':')))';const control=scriptCompilerAdmissionControlV1(original,"file-publication:statx-struct",20000),capture=await captureOwnedCommandResultV1(testPythonV1()??"python3",["-c",pro...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts:23` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `}expect((db.query("select count(*) n from trace t where event='transition' and not exists(select 1 from trace s where s.effect=t.effect and s.event='stage-sync' and s.position<t.position) or event='unlink'").get()as{n...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts:29` [console.log] test-path: yes; allocates: yes: .filter(, JSON.stringify
  - `const api=await import("@semio-tech/framework-filesystem/observation/descriptor"),fsp=await import("node:fs/promises"),routing=await import("../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),noble=await import("@nobl...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts:204` [console.log] test-path: yes; allocates: no
  - `async function verifySelectedDescriptorLsofV1(original:import("../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<void>{if(process.platform!=="darwin"&&process.platform!=="linux")return;const...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts:571` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("[DATA] Generic root-family preservation " + JSON.stringify({ effect: row.effect, peak, restored: true, inodeOracle: true }));`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts` (7)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts:17` [console.log] test-path: yes; allocates: yes: .push(, Array.from, JSON.stringify
  - `if(process.env.SEMIO_DESCRIPTOR_CUSTODY_CASE!==undefined)test("compiler cache isolated descriptor custody",async()=>{const name=process.env.SEMIO_DESCRIPTOR_CUSTODY_CASE,row=corpus.descriptors.find(row=>row.name===(na...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts:44` [console.log] test-path: yes; allocates: yes: .push(, .repeat(, JSON.stringify
  - `if(process.env.SEMIO_CACHE_CLEANUP_CASE!==undefined)test("compiler cache isolated actual cleanup lease",async()=>{const name=process.env.SEMIO_CACHE_CLEANUP_CASE,row=corpus.cleanup.find(row=>row.name===name);if(!row||...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts:59` [console.log] test-path: yes; allocates: yes: .map(, .repeat(, JSON.stringify
  - `if(process.env.SEMIO_CREATOR_CLAIM_CASE!==undefined)test("compiler cache isolated creator preclaim",async()=>{const name=process.env.SEMIO_CREATOR_CLAIM_CASE,directory=process.env.SEMIO_CREATOR_CLAIM_DIRECTORY,row=cor...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts:60` [console.log] test-path: yes; allocates: yes: .filter(, .repeat(, JSON.stringify
  - `async function proveCreatorClaims(control:import("../../../../🔒️leases/🟦️.ts").ResourceLeaseControl,original:import("../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<void>{const routing=...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts:72` [console.log] test-path: yes; allocates: yes: .map(, .push(, JSON.stringify
  - `async function withCreatorMetadataObservationV1<T>(directory:string,path:string,body:()=>Promise<T>):Promise<T>{const descriptors=await import("@semio-tech/framework-filesystem/observation/descriptor"),shape=(await im...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts:91` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `if(process.env.SEMIO_PREPARATION_OBSERVATION_CASE!==undefined)test("compiler cache isolated preparation observation",async()=>{const row=corpus.preparationObservations.find(row=>row.name===process.env.SEMIO_PREPARATIO...`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts:92` [console.log] test-path: yes; allocates: yes: .filter(
  - `async function provePreparationObservations(control:import("../../../../🔒️leases/🟦️.ts").ResourceLeaseControl,original:import("../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<void>{cons...`

### `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗂️files/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗂️files/🧪️tests/🟦️.ts:56` [console.log] test-path: yes; allocates: yes: ${, .map(, .push(, JSON.stringify
  - `const program = 'const api = await import(${JSON.stringify(pathToFileURL(bundled).href)}); const rows = JSON.parse(${JSON.stringify(JSON.stringify(corpus.collections))}); const output=[]; for(const row of rows)output....`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️claims/🟦️.ts` (3)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️claims/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: ${, .push(, Array.from, JSON.stringify
  - `registerBuildTestLawV1("default queue claim cleanup passes unfinished prefixes and preserves cancellation","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:claim-frontier",0,async control=>{const sink...`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️claims/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${, .repeat(, Array.from, JSON.stringify
  - `registerBuildTestLawV1("default queue claim cleanup caps actual eligible foreign probes","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:claim-cap",0,async control=>{const sink=process.env.SEMIO_TEST...`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️claims/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: ${, .filter(, Array.from, JSON.stringify
  - `registerBuildTestLawV1("default resource and claim scans bound controlled full name traffic","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:scan-traffic",0,async control=>{const sink=process.env.SEM...`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️frontier/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️frontier/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: ${, .push(, Array.from, JSON.stringify
  - `registerBuildTestLawV1("default lease retirement scans unfinished prefixes without losing useful work or cancellation","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:retirement-frontier",0,async con...`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️frontier/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default lease retirement retains existing ownership closure and primary regression cases","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:retirement-frontier:regression",0,asy...`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🟦️.ts` (5)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `registerBuildTestLawV1("default lease inventory refuses copied controls before actual directory opens","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:inventory:issuance",0,async control=>{const sink...`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${, Array.from, JSON.stringify
  - `registerBuildTestLawV1("default lease inventory preserves actual cancellation read and close primary failures","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:inventory:failures",0,async control=>{co...`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: ${, .join(, Array.from, JSON.stringify
  - `registerBuildTestLawV1("default lease directory inventory closes genuine async reads at full and overflowing bounds","sdk-runtime",original=>withIssuedResourceControlV1(original,"lease:inventory:bounds",0,async contro...`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `registerBuildTestLawV1("default lease inventory returns only frozen immediate names after genuine close","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:inventory:ordinary",0,async control=>{const si...`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: ${, .filter(, .join(, JSON.stringify
  - `registerBuildTestLawV1("default lease inventory full cardinality contract stays within the original transport budget","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:inventory:cardinality-contract",0...`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧭️continuation/🧪️tests/📦️child/📜️script.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧭️continuation/🧪️tests/📦️child/📜️script.ts:30` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({result:result??null,caps,commits,updates,headroomAfter,gaps,closes,restored,failed,exact,failure,mode,foreignExclusive,elapsedMilliseconds,cleanupDenied,cleanupCallbacks,cleanupOpens}));pro...`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/⚖️fairness/🧪️tests/📥️worker/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/⚖️fairness/🧪️tests/📥️worker/🟦️.ts:8` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const say=(value:Record<string,unknown>)=>console.log(JSON.stringify({role:config.role,runtime:process.versions.bun?"bun":"node",...value}));`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️bounded/🧪️tests/♻️preparation/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️bounded/🧪️tests/♻️preparation/🟦️.ts:16` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `registerBuildTestLawV1("default SQLite capacity prepares once and validates every numeric admission without caller effects","sdk-fence",original=>withIssuedResourceControlV1(original,"lease:capacity-preparation",0,asy...`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️bounded/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️bounded/🧪️tests/🟦️.ts:39` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `try{const program="const{DatabaseSync}=require('node:sqlite');const db=new DatabaseSync(process.argv[1]);let refused=false;try{db.exec('PRAGMA page_size=4096;PRAGMA max_page_count='+process.argv[2]+';BEGIN EXCLUSIVE;C...`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️namespace/🧪️tests/📥️pending/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️namespace/🧪️tests/📥️pending/🟦️.ts:12` [console.log] test-path: yes; allocates: yes: Date.now
  - `mkdirSync(root);const targetDirectory=join(root,"uninitialized"),resource="uninitialized-original-target";mkdirSync(targetDirectory);const target=resourceLeasePathV1(targetDirectory,resource),ready=join(root,"ready"),...`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/⚔️concurrency/📥️worker/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/⚔️concurrency/📥️worker/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const wire=process.env.SEMIO_SCRIPT_PROCESS_INVOCATION!;if(wire!==scriptInvocationEnvironment(original,process.env).SEMIO_SCRIPT_PROCESS_INVOCATION)throw new Error("Exact canonical received worker wire required");cons...`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/⚔️concurrency/📥️worker/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `}catch(error){failed=true;failure=error;throw error;}finally{settled=true;let cleanup:unknown,cleanupFailed=false;try{held?.release();}catch(error){cleanupFailed=true;cleanup=error;}try{await monitor;}catch(error){if(...`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts` (5)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts:80` [console.log] test-path: yes; allocates: no
  - `const lease = await (queuedOwner?acquireQueuedResourceLease:acquireResourceLease)({ directory, resource, mode,owner:queuedOwner, signal: controller.signal, onWait: () => console.log("waiting") });`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts:81` [console.log] test-path: yes; allocates: no
  - `try { console.log("held"); await stopped; }`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts:82` [console.log] test-path: yes; allocates: no
  - `finally { lease.release(); console.log("released"); }`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts:83` [console.log] test-path: yes; allocates: no
  - `} catch (error) { if (controller.signal.aborted) console.log("cancelled"); else throw error; }`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts:202` [console.log] test-path: yes; allocates: no
  - `console.log("Resource lease queue: arrival order served, a crashed waiter's ticket swept, a cancelled waiter leaves no ticket PASS");`

### `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🪦️stale-queue/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🪦️stale-queue/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const holder=(directory:string)=>'const {Database}=await import("bun:sqlite");const api=await import(${JSON.stringify(process.env.SEMIO_TEST_LEASES_MODULE??import.meta.resolve("../../🟦️.ts"))});const control=new Abort...`

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🧪️tests/🟦️.ts:32` [console.log] test-path: yes; allocates: no
  - `const program = 'const row=JSON.parse(process.argv[2]),cancellation=new AbortController();if(row.abortMs!==null)setTimeout(()=>cancellation.abort(),row.abortMs);import(require('node:url').pathToFileURL(process.argv[1]...`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🧪️tests/🟦️.ts:67` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const program = 'const api=await import(process.argv[1]);const {spawnSync}=require("node:child_process");const root=api.createTestScratchRootV1(process.argv[2],"scratch-exit-");const {mkdirSync}=require("node:fs");con...`

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧪️tests/🟦️.ts:224` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const program = 'import {createTestSandboxV1} from ${JSON.stringify(modulePath)};import {spawnSync} from "node:child_process";import {mkdirSync} from "node:fs";import {join} from "node:path";const s=createTestSandboxV...`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧪️tests/🟦️.ts:278` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `const modulePath = join(root, "🟦️.ts"), program = 'import {createTestSandboxV1} from ${JSON.stringify(modulePath)};const s=createTestSandboxV1({route:"child",reclaim:false});const ports=await Promise.all([1,2,3,4,5].m...`

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/📜️script.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/📜️script.ts:13` [console.log] test-path: no; allocates: yes: .filter(, JSON.stringify
  - `console.log(JSON.stringify({ dryRun: true, total: rows.length, ours: rows.filter(row => row.kind === "ours").length, leaked: rows.filter(row => row.kind === "leaked").length, foreign: rows.filter(row => row.kind === "...`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/📜️script.ts:16` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(sweepLeakedSandboxImagesV1(workspaceOnly?{under:root}:{parents:[machinePathV1(root,".cache","test-sandbox"),join(root,".test-scratch")]})));`

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/🧪️tests/⏱️bounded/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/🧪️tests/⏱️bounded/🟦️.ts:38` [console.log] test-path: yes; allocates: no
  - `const run=captureOwnedCommandV1(command,["-e",row.mode==="exit"?'console.log("swept");':hung],cwd,options);`

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🧪️tests/🧫️declared/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🧪️tests/🧫️declared/🟦️.ts:10` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(kind!=="unregistered")registerBuildTestGrantIssuerV1("native-build-caller",{prepare:()=>({environment:{SEMIO_DECLARED_FAKE:"prepared"},release(){console.log("[TRACE] Declared release "+JSON.stringify({fake:process....`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🧪️tests/🧫️declared/🟦️.ts:17` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `test("declared birth "+kind,()=>withBuildTestInvocationV1(async original=>{const facts=original.capabilities as{artifacts?:string;command?:string;tool?:string;arguments?:string[]};if(kind?.startsWith("granted"))consol...`

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🧪️tests/🧫️harness/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🧪️tests/🧫️harness/🟦️.ts:13` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const emit=(phase:string,facts:object={})=>console.log("[TRACE] Receiver event "+JSON.stringify({phase,...facts})),sleep=(ms:number)=>new Promise<void>(done=>setTimeout(done,ms));let released=false,firstPolicy:unknown...`

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/📁️files/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/📁️files/🧪️tests/🟦️.ts:48` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `let joined=true;if(selected==="output-limit"||selected==="cohort-limit"||selected==="sibling-drain"){for(const file of selected==="output-limit"?[marker]:[marker,marker+"-sibling"]){if(!existsSync(file)){joined=false;...`

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🧯️ceiling/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🧯️ceiling/🧪️tests/🟦️.ts:67` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("RESULT "+JSON.stringify({ok:true}));`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🧯️ceiling/🧪️tests/🟦️.ts:68` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `}catch(error){console.log("RESULT "+JSON.stringify({ok:false,message:String((error as Error).message)}));}`

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/🧪️tests/🟦️.ts:56` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const raised=spawnSync(process.execPath,["-e",'const{limitOpenFilesV1}=await import(${JSON.stringify(here("../👶️child/🟦️.ts"))});console.log(limitOpenFilesV1(1e9))'],{encoding:"utf8",env:{...process.env}});`

### `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🛠️preparation/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🛠️preparation/🧪️tests/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `}else test("actual exact preparation refusal child",async()=>{let failed=false,primary:unknown,reads=0;try{await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/❄️cold/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/❄️cold/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `writeFileSync(join(output,"cold-terminal.json"),JSON.stringify({version:1,count,bytes,closed,configuredTarget:target,result,rootMarkerFallback:false,specificSourcesAvailable:false}));console.log(result.stdout);console...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🟦️.ts:45` [console.log] test-path: yes; allocates: no
  - `for(const mode of corpus.buildModes)for(const row of corpus.configurations){const args=["--config",configuration,"--cwd",row.cwd,"--manifest",row.manifest,"--",process.execPath,"-e","console.log('variable')"],selectio...`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🟦️.ts:57` [console.log] test-path: yes; allocates: yes: .push(
  - `const phases:string[]=[];await executeCommandV1({...request,args:["-e","console.log('prepared')"],preparation:"required"},policy,{environment:process.env,invocation,onProgress:()=>{},prepare:async value=>{phases.push(...`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🟦️.ts:60` [console.log] test-path: yes; allocates: no
  - `const retired=await executeCommandV1({...request,args:["-e","console.log('retained receipt')"]},{...policy,retainArtifacts:false},{environment:process.env,invocation,onProgress:()=>{}});expect(readFileSync(retired.rec...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/⏳️following/👷️child/📜️script.ts` (3)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/⏳️following/👷️child/📜️script.ts:6` [console.log] test-path: yes; allocates: no
  - `console.log("issued-deadline:" + first.deadlineEpochMilliseconds);`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/⏳️following/👷️child/📜️script.ts:9` [console.log] test-path: yes; allocates: no
  - `console.log("work:" + pulse);`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/⏳️following/👷️child/📜️script.ts:13` [console.log] test-path: yes; allocates: no
  - `console.log("issued-deadline:" + last.deadlineEpochMilliseconds);`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/⏳️progress/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/⏳️progress/🟦️.ts:117` [console.log] test-path: yes; allocates: no
  - `await runOwnedCommand(process.execPath, child("let n=0;const t=setInterval(()=>{console.log('tick',n++);if(n===12){clearInterval(t)}},150)"), process.cwd(), "fixture:chatty", 0, { signal: control.signal, output: { max...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const actual={aborted,inherited,rejected,retired,preserved};console.log("actual-control-expiry:"+JSON.stringify(actual));expect(actual).toEqual({aborted:row.expectedAborted,inherited:row.expectedInherited,rejected:row...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests/🟦️.ts:55` [console.log] test-path: yes; allocates: no
  - `const premise=spawnSync(process.execPath,["-e","let writes=0;const write=process.stdout.write.bind(process.stdout);process.stdout.write=(...args)=>{writes++;return write(...args);};console.log('x');process.stdout.writ...`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests/🟦️.ts:59` [console.log] test-path: yes; allocates: no
  - `try{for(let round=0;round<12;round++){console.log("watchdog progress line "+round);await sleep(30);}expect(stalls,"console output keeps the process alive (runtime passes console through write: "+viaConsole+")").toBe(0...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📤️child/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📤️child/🧪️tests/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `else test("neutral child receives its actual narrowed parent phase",async()=>{if(selected!=="received")throw Error("Closed actual child case required");await receiveScriptProcessInvocation(process.env,async invocation...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📥️issuance/🧪️tests/📥️drain/🧭️modes/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📥️issuance/🧪️tests/📥️drain/🧭️modes/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const result=await proveAcceptedOriginalDrainV1(row.mode,undefined as never);console.log("actual-drain-mode:"+JSON.stringify(result));`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🚪️admission/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🚪️admission/🧪️tests/🟦️.ts:28` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const actual={newEffects,siblingCompletions,refused,preserved:caught&&failure===marker,waited,retired};console.log("actual-failed-admission:"+JSON.stringify(actual));expect(actual).toEqual({newEffects:row.expectedNewE...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📤️settlement/🔎️observation/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📤️settlement/🔎️observation/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({failed,positive,closed,preserved:row.failure==="undefined"?failure===undefined:row.failure==="error"?failure===sentinel:true,message:failure instanceof Error?failure.message:String(failure)...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📥️canonical/🧹️cleanup/⏱️clock/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📥️canonical/🧹️cleanup/⏱️clock/🟦️.ts:11` [console.log] test-path: yes; allocates: yes: Date.now, JSON.stringify
  - `const source=new URL("../../../../🟦️.ts",import.meta.url).href,program='const api=await import(process.argv[1]),f=JSON.parse(process.argv[2]);let deadlineRefused=false;try{await api.withScriptProcessEnvelope(api.creat...`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📥️canonical/🧹️cleanup/⏱️clock/🟦️.ts:16` [console.log] test-path: yes; allocates: yes: .push(, Date.now, JSON.stringify
  - `const source=new URL("../../../../🟦️.ts",import.meta.url).href,program='const api=await import(process.argv[1]),f=JSON.parse(process.argv[2]);const results=[];for(const primary of[false,true]){let refusal;try{await ap...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧹️cleanup/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧹️cleanup/🧪️tests/🟦️.ts:18` [console.log] test-path: yes; allocates: no
  - `await api.withAcceptedPhysicalCleanupV1(control,request(1000),async(token,body)=>{child=api.withIssuedResourceOperationV1(body,()=>captureOwnedCommandV1("node",["--eval","setTimeout(()=>console.log('accepted-child-com...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🪢️lifetime/🧪️tests/📦️child/📜️script.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🪢️lifetime/🧪️tests/📦️child/📜️script.ts:31` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const oracle=db.query<{peak:number;net:number},[]>("select coalesce(max(running),0) peak,(select coalesce(sum(delta),0) from trace) net from(select sum(delta)over(order by n) running from trace)").get(),files=(await r...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🧪️tests/⏱️finite/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🧪️tests/⏱️finite/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const oracle = Bun.spawnSync([registeredToolExecutableV1("node"), "--eval", 'const rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(row=>row.maximumElapsedMilliseconds===0?row.deadlineOffsetMillise...`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🧪️tests/⏱️finite/🟦️.ts:68` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const oracle = Bun.spawnSync([registeredToolExecutableV1("node"), "--eval", 'const r=JSON.parse(process.argv[1]);console.log(JSON.stringify(r.map(v=>!v.abort&&(v.maximum===0?v.remaining===null:typeof v.remaining==="nu...`

### `🧰️framework/🔨️modules/🏃️process/🧭️routing/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🧪️tests/🟦️.ts:151` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(results));';`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🧪️tests/🟦️.ts:438` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const oracle = Bun.spawnSync([registeredToolExecutableV1("node"), "--eval", 'const rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(row=>{const accepted=Number.isSafeInteger(row.parent)&&Number.isS...`

### `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🏃️run/🟦️.ts` (6)

- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🏃️run/🟦️.ts:30` [console.log] test-path: yes; allocates: no
  - `const result = await runToolV1("heavy", program("console.log(process.env.FIXED, process.env.SEMIO_TOOL_RUN, process.env.LEAK_ME)"), { ...options, capture: true, admission: false });`
- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🏃️run/🟦️.ts:77` [console.log] test-path: yes; allocates: yes: .push(
  - `const result = await runToolV1("heavy", program("console.log(process.env.SEMIO_GRANT)"), { ...options, capture: true, admission, onProgress: event => events.push(event.phase) });`
- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🏃️run/🟦️.ts:94` [console.log] test-path: yes; allocates: no
  - `const text = runToolSyncV1("light", program("console.log('sync')"), { ...options, encoding: "utf8" });`
- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🏃️run/🟦️.ts:96` [console.log] test-path: yes; allocates: no
  - `expect(Buffer.isBuffer(runToolSyncV1("light", program("console.log('sync')"), options).stdout)).toBe(true);`
- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🏃️run/🟦️.ts:97` [console.log] test-path: yes; allocates: no
  - `expect(runToolSyncV1("heavy", program("console.log('heavy')"), { ...options, encoding: "utf8" }).stdout.trim()).toBe("heavy");`
- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🏃️run/🟦️.ts:105` [console.log] test-path: yes; allocates: no
  - `const child = startToolV1("light", program("console.log(process.env.FIXED)"), { ...options, stdio: ["ignore", "pipe", "inherit"] });`

### `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🔌️funnel/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🔌️funnel/🟦️.ts:28` [console.log] test-path: yes; allocates: no
  - `const tail = await captureOwnedCommandV1("bun", ["-e", "console.log(process.env.NODE_OPTIONS ?? 'none', process.env.SEMIO_TOOL_RUN)"], process.cwd(), { env: poisoned });`
- `🧰️framework/🔨️modules/🏃️process/🧰️tools/🧪️tests/🔌️funnel/🟦️.ts:40` [console.log] test-path: yes; allocates: no
  - `const child = startToolV1("bun", ["-e", "console.log(process.env.SEMIO_TOOL_RUN)"], { env: poisoned, stdio: ["ignore", "pipe", "inherit"] });`

### `🧰️framework/🔨️modules/🏃️process/🧹️scratch/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🧹️scratch/🧪️tests/🟦️.ts:58` [console.log] test-path: yes; allocates: no
  - `const run=spawnSync(process.execPath,["--preload",script,"-e","console.log(require('node:os').tmpdir());console.log(process.env.TMPDIR===require('node:os').tmpdir())"],{encoding:"utf8",env:environment,cwd:import.meta....`

### `🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions/🧪️tests/📥️worker/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions/🧪️tests/📥️worker/🟦️.ts:23` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const child=Bun.spawnSync([process.execPath,"-e","const {currentAgentSessionV1}=await import("+JSON.stringify(new URL("../../🟦️.ts",import.meta.url).href)+");console.log(JSON.stringify({id:currentAgentSessionV1({ances...`

### `🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions/🧭️ancestry/🪟️native/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions/🧭️ancestry/🪟️native/🧪️tests/🟦️.ts:137` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[winps] windows process listing: kernel ${nativeMs.toFixed(2)} ms (${direct!.length} rows), PowerShell ${shellMs.toFixed(0)} ms (${viaShell.length} rows); first row started kernel ${direct![0]!.started} s...`

### `🧰️framework/🔨️modules/🏗️mesh-engine/🎯️component-references/🧪️tests/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🏗️mesh-engine/🎯️component-references/🧪️tests/🦀️.rs:11` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify, json!, serde
  - `use std::io::Write;let mut oracle=std::process::Command::new("bun").args(["-e","import Ajv from 'ajv';const x=JSON.parse(await Bun.stdin.text());const valid=new Ajv({strict:true}).compile(x.schema);for(const table of ...`

### `🧰️framework/🔨️modules/🏗️mesh-engine/🏛️ownership/🎨️attributes/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🏗️mesh-engine/🏛️ownership/🎨️attributes/🧪️tests/🟦️.ts:43` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const code="const fs=require('node:fs'),p=require('node:path'),c=require('node:crypto');const source=process.argv[1],fixture=process.argv[2],text=fs.readFileSync(source,'utf8'),routes=[...text.matchAll(/test_inputs::t...`

### `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🎛️control/🟦️.ts` (1)

- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🎛️control/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("file-observation-result:"+JSON.stringify({kind,preserved,...(kind==="other"?{detail:String((failure as Error|undefined)?.stack??failure).slice(0,600)}:{})}));expect(kind).toBe(row.expected);expect(preserv...`

### `🧰️framework/🔨️modules/📡️replication/📐️format/🔎️verification/🧪️tests/🔬️unit/🦀️.rs` (3)

- `🧰️framework/🔨️modules/📡️replication/📐️format/🔎️verification/🧪️tests/🔬️unit/🦀️.rs:79` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("SPR resume: 6 verified prefixes survive byte-exactly; chain/sequence/offset continue; 12 wrong sink lengths and exhausted sequence denied");`
- `🧰️framework/🔨️modules/📡️replication/📐️format/🔎️verification/🧪️tests/🔬️unit/🦀️.rs:174` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("retained SPR: 2 real writer commits, 224 LastCommit prefixes at 3 fuel grants; exact EOF required; one span handoff; no typed records");`
- `🧰️framework/🔨️modules/📡️replication/📐️format/🔎️verification/🧪️tests/🔬️unit/🦀️.rs:236` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("retained SPR: 26 hostile header/frame/commit/limit denials and 10 compressed grammar cases at 3 grants; cancellation at all 256 byte boundaries; no input authority or semantic publication");`

### `🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🧪️tests/🔬️unit/🦀️.rs:85` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts` (3)

- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:363` [console.log] test-path: no; allocates: yes: ${
  - `console.log('retained SPR resume oracle: ${fixture.resume.cuts.length} exact prefixes, next sequence/previous offset/hash chain preserved');`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:403` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] retained SPR observation oracle: ${fixture.cases.length} exact rows, ${observed} scalar observations; compressed raw-length/empty payload/clear/error/cancel; zero commit or input authority');`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:453` [console.log] test-path: no; allocates: yes: ${, .filter(
  - `console.log('presence peer codec oracle: ${fixture.cases.length} neutral Rust/TypeScript vectors, ${fixture.cases.filter((row: { accepted: boolean }) => !row.accepted).length} hostile inputs rejected exactly');`

### `🧰️framework/🔨️modules/🔲️pixels/📷️png/📥️decode/🧪️tests/🧫️fixtures/📝️text/🔬️oracle.ts` (1)

- `🧰️framework/🔨️modules/🔲️pixels/📷️png/📥️decode/🧪️tests/🧫️fixtures/📝️text/🔬️oracle.ts:8` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('PNG text oracle: ${corpus.length} cases');`

### `🧰️framework/🔨️modules/🔲️pixels/🧪️testing/🧭️ownership/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🔲️pixels/🧪️testing/🧭️ownership/🧪️tests/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `const source = 'import {mock} from "bun:test"; const calls=[]; const execution=await import(${JSON.stringify(execution)}); mock.module(${JSON.stringify(execution)},()=>({...execution,runOwnedCommand:async(executable,a...`

### `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts:34` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[framework-graph] wrote ${rendered.manifestCount} manifests to ${relative(root, outDir)}');`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts:71` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[framework-graph] ${rendered.manifestCount} generated manifests are fresh');`

### `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📥️admission/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📥️admission/🟦️.ts:104` [console.log] test-path: no; allocates: yes: ${
  - `if (log) console.log('[framework-graph] skip ${relative(root, path)} (${doc.schema})');`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📥️admission/🟦️.ts:107` [console.log] test-path: no; allocates: yes: ${
  - `if (log) console.log('[framework-graph] ${relative(root, path)}');`

### `🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts:32` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const program = "import {parseGraphOutputCatalog} from " + JSON.stringify(resolve(import.meta.dir, "../../🛂️manifest/📇️catalog/🟦️.ts")) + ";import {renderGraphArtifacts} from " + JSON.stringify(resolve(import.meta.dir...`

### `🧰️framework/🔨️modules/🖌️raster/🎥️video/🖋️svg-export/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖌️raster/🎥️video/🖋️svg-export/🧪️tests/🟦️.ts:13` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const program = "import {logoKeyframePaths} from " + JSON.stringify(logo) + ";console.log(JSON.stringify(logoKeyframePaths(" + JSON.stringify("logo") + ").map(path=>path.replaceAll(String.fromCharCode(92),'/'))));";`

### `🧰️framework/🔨️modules/🖌️raster/🎥️video/🧪️tests/🔬️unit/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖌️raster/🎥️video/🧪️tests/🔬️unit/🟦️.ts:105` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('raster-video cases=${fixture.cases.length} refusals=${fixture.refusals.length}');`

### `🧰️framework/🔨️modules/🖱️ui/🌐️locale/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🌐️locale/🧪️tests/🟦️.ts:113` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const script = "const rows=" + JSON.stringify(fixture.locales) + ";console.log(JSON.stringify(rows.map(row=>{let locale=null;try{const candidate=row.mode==='id'?row.input:new Intl.Locale(row.input).language;if(['en','...`

### `🧰️framework/🔨️modules/🖱️ui/🎚️axes/📜️script.ts` (2)

- `🧰️framework/🔨️modules/🖱️ui/🎚️axes/📜️script.ts:23` [console.log] test-path: no; allocates: yes: ${
  - `console.log('ui axes refreshed (${axes.locales.length} locales, ${axes.terminologies.length} terminologies)');`
- `🧰️framework/🔨️modules/🖱️ui/🎚️axes/📜️script.ts:35` [console.log] test-path: no; allocates: yes: ${
  - `console.log('ui axes are fresh (${axes.locales.length} locales, ${axes.terminologies.length} terminologies).');`

### `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts` (2)

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts:23` [console.log] test-path: no; allocates: yes: ${
  - `const lease = await acquireResourceLease({ directory: readProcessOwnerContextV1(process.env,root).leaseDirectory, resource: 'styling-dotnet:${resolve(state)}', mode: "exclusive", signal, control, onWait: ({ elapsedMs ...`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts:49` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Published styling .NET assembly: ${files.size} files');`

### `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts:14` [console.log] test-path: no; allocates: no
  - `console.log("[nx-generate] styling artifacts ready");`

### `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts` (5)

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:18` [console.log] test-path: no; allocates: no
  - `console.log("framework/ui/styling: wrote generated CSS/TS/C#/Rust/Python styling artifacts");`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:31` [console.log] test-path: no; allocates: no
  - `console.log("framework/ui/styling: generated artifacts are fresh");`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:53` [console.log] test-path: no; allocates: yes: ${
  - `console.log('styling-verification-contract: ${proveStylingVerificationContractV1()} vectors passed');`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:61` [console.log] test-path: no; allocates: no
  - `console.log("styling-relative-sizing: " + await proveRelativeStylingSizesV1() + " native assertions passed");`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:70` [console.log] test-path: no; allocates: no
  - `console.log("styling-color-primitives: " + await proveStylingColorPrimitivesV1() + " native assertions passed");`

### `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️fonts/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️fonts/🟦️.ts:148` [console.log] test-path: no; allocates: yes: ${
  - `console.log('framework/ui/styling: catalog fonts ready under framework assets (${wrote} downloaded, ${sources.length} total)');`

### `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️node-twin/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️node-twin/🟦️.ts:40` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[twin] 🔁️animation-scope: ${fixture.cases.length} fixture cases + 🖌️ui.css, ${failures} failure(s)');`

### `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️python-package-commands/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️python-package-commands/🟦️.ts:10` [console.log] test-path: yes; allocates: no
  - `console.log("[nx-generate] styling artifacts ready");`

### `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts:104` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `const program = 'import {tileProxyVitePlugin} from ${JSON.stringify(resolve(directory, "🛰️tile-proxy/🟦️.ts"))};import {resolveAssetDeliveryModeV1} from ${JSON.stringify(resolve(directory, "../../../../🖼️assets/🔍️resol...`

### `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️theme-resolve/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️theme-resolve/🟦️.ts:75` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const program = 'import { parseUiTheme, serializeUiTheme, resolveThemePaint } from ${JSON.stringify(resolve(directory, "🏛️model/🟦️.ts"))}; import fixture from ${JSON.stringify(fixture)} with { type: "json" }; const th...`

### `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️mesh-unit/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️mesh-unit/🦀️.rs:22` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `let mut oracle=std::process::Command::new("bun").args(["-e","const x=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify(x.fixture.copyGrants.map(work=>({work,pageBytes:x.pageBytes,directoryBytes:x.directory...`

### `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🏁️ui-state-and-presented-input/🦀️.rs` (2)

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🏁️ui-state-and-presented-input/🦀️.rs:150` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] seal refuse: existing seals visible={visible_windows:?}");`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🏁️ui-state-and-presented-input/🦀️.rs:154` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] seal refuse: presented without candidate visible={visible_windows:?}");`

### `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🖼️raster/🧪️tests/🖼️raster-residency/🦀️.rs` (2)

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🖼️raster/🧪️tests/🖼️raster-residency/🦀️.rs:421` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no headless WGPU adapter available");`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🖼️raster/🧪️tests/🖼️raster-residency/🦀️.rs:471` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no headless WGPU adapter available");`

### `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🧪️tests/🟦️.ts:41` [console.log] test-path: yes; allocates: yes: .join(, .map(, JSON.stringify
  - `const owner=resolve(import.meta.dir,".."),observe=fixture.sourceAssertions.map(row=>row.symbols.every(symbol=>readFileSync(resolve(owner,row.source),"utf8").includes(symbol))),program=String.raw'const fs=require("node...`

### `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts` (2)

- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts:64` [console.log] test-path: no; allocates: no
  - `console.log("Inspecting ui-render normal dependencies (0/1).");`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts:78` [console.log] test-path: no; allocates: yes: ${
  - `console.log('ui-render dependency boundaries hold (1/1; ${FORBIDDEN_DEPENDENCIES.length} forbidden crates absent).');`

### `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-backend-unit/🦀️.rs` (5)

- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-backend-unit/🦀️.rs:28` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no D3D12 device available on this machine");`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-backend-unit/🦀️.rs:40` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no D3D12 device available on this machine");`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-backend-unit/🦀️.rs:58` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no D3D12 device available on this machine");`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-backend-unit/🦀️.rs:80` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no D3D12 device available on this machine");`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-backend-unit/🦀️.rs:100` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no D3D12 device available on this machine");`

### `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-backend-unit/🦀️.rs` (5)

- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-backend-unit/🦀️.rs:29` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no Metal device available on this machine");`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-backend-unit/🦀️.rs:41` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no Metal device available on this machine");`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-backend-unit/🦀️.rs:59` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no Metal device available on this machine");`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-backend-unit/🦀️.rs:81` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no Metal device available on this machine");`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-backend-unit/🦀️.rs:101` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipping: no Metal device available on this machine");`

### `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/🔬️single-enqueue-publication/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/🔬️single-enqueue-publication/🟦️.ts:63` [console.log] test-path: yes; allocates: no
  - `console.log("single-enqueue schema/Buffer oracle: 3 exact 24-byte tuples, 5 hostiles; queue+scene only, unchanged build input; native interlock is a separate gate");`

### `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/🧪️tests/🔬️input-commit-observer/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/🧪️tests/🔬️input-commit-observer/🟦️.ts:99` [console.log] test-path: yes; allocates: no
  - `console.log("input commit observer format oracle: 3 exact 56-byte tuples, 3 declared phases, 5 schema hostiles; independent updates remain legitimate; single-operation native interlock and funding unexecuted");`

### `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🎟️admission/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🎟️admission/🦀️.rs:22` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("event-queue-constructor capacity={} slot-bytes={} physical={physical}", queue.discrete.capacity(), size_of::<DiscreteEvent>());`

### `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/🧪️tests/🪪️root/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/🧪️tests/🪪️root/🦀️.rs:157` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("input root static={atomic} queue={} rootField={} queueBacking={}", grant(), size_of_val(&last.root), last.discrete.capacity());`

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs:79` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("runtime tree exact owner:18 components*3 grants; occupied node/tree/held sources preserved; bytes matched; foreign untouched");`

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧪️tests/📃️document/🦀️.rs` (2)

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧪️tests/📃️document/🦀️.rs:101` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧪️tests/📃️document/🦀️.rs:148` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("existing-pair-unwind phase={phase} exact-current-unchanged=true retained-close=true");`

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧪️tests/📏️ownership/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧪️tests/📏️ownership/🦀️.rs:302` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("surface-patch-allocation turns={allocation_turns} largest={largest} operation-bytes={}", size_of::<ui_contract::UiPatchOp>());`

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧪️tests/🔒️authority/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧪️tests/🔒️authority/🦀️.rs:46` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("surface-physical-owner owner={owner} bytes={bytes}");`

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/🔬️unit/🦀️.rs:83` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("output-pool busy-refusal-exact=true zero-grant-mutates=false static-bytes={}", SurfaceReconcileOutputs::static_backing_bytes());`

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts:52` [console.log] test-path: no; allocates: yes: ${
  - `console.log('surface-ownership-oracle checks=${surfaceOwnershipSelfTests()}');`

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️authority-unit/🦀️.rs` (4)

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️authority-unit/🦀️.rs:85` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("canonical-owner-layout reconciler={} cursor={} retained={}", size_of::<SurfaceReconciler>(), size_of::<SurfaceReconcileCursor>(), size_of::<SurfaceReconcileRetained>());`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️authority-unit/🦀️.rs:413` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️authority-unit/🦀️.rs:575` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️authority-unit/🦀️.rs:599` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️rust-package-commands/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('runtime-tree exact native laws:${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed');`

### `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🩹️patch/🧪️tests/🩹️patch/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🩹️patch/🧪️tests/🩹️patch/🦀️.rs:112` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("patch-handoff unwind-frontiers=3 exact-payload-pointer=true exact-authority-count=1 publish-bytes={} ack-bytes={}", SurfaceReconcileReadyPatch::required_publish_bytes(), SurfaceReconcilePublishedPatch::requ...`

### `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧭️native-command/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧭️native-command/🟦️.ts:71` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const source = "import { Command } from 'commander'; const oracle = " + oracle.toString() + "; console.log(JSON.stringify(" + JSON.stringify(corpus.cases) + ".map(oracle)));";`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧭️native-command/🟦️.ts:93` [console.log] test-path: yes; allocates: yes: .join(, .map(
  - `console.log("[ui-native-command] " + row.id + ": " + (rejected ? "refused" : calls.map(call => call.kind).join(",")));`

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/📦️whole/🧪️tests/🧩️whole-patch/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/📨️pending/📦️whole/🧪️tests/🧩️whole-patch/🦀️.rs:81` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("pending-whole-patch unwind-frontiers=6 empty-reserved-backing-retired=true exact-handoff=true owner-bytes={}", size_of::<UiPendingPatch>());`

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/📃️document/🧪️tests/📃️document/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/📃️document/🧪️tests/📃️document/🦀️.rs:131` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("document-component-admission required={required} rejected-root-pointer-exact=true foreign-ordinal-denied=true");`

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚙️operations/🪞️copy/🧪️tests/🪞️copy/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚙️operations/🪞️copy/🧪️tests/🪞️copy/🦀️.rs:99` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("component-copy-real-grant inline={} work-max={maximum_work} complete={complete}", size_of::<crate::Component>());`

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🧪️tests/🔬️unit/🦀️.rs:150` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧪️tests/🔄️refresh-conformance/🦀️.rs` (3)

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧪️tests/🔄️refresh-conformance/🦀️.rs:111` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧪️tests/🔄️refresh-conformance/🦀️.rs:142` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧪️tests/🔄️refresh-conformance/🦀️.rs:182` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("resident-refresh-ceiling admitted={count} refusal={refusal:?} surface-bytes={UI_RESIDENT_SURFACE_BYTES} aggregate={UI_RESIDENT_AGGREGATE_BYTES} fixed-backing={}", UiResidentPermit::contract_backing_bytes());`

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧪️tests/🎟️resident/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🧪️tests/🎟️resident/🦀️.rs:107` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("resident-permit small=9 slots={UI_RESIDENT_SLOTS} document={UI_RESIDENT_DOCUMENT_BYTES} aggregate={UI_RESIDENT_AGGREGATE_BYTES} paired-return=0,65536 explicit-close-drop-does-not-return-again=true");`

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts` (11)

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:54` [console.log] test-path: no; allocates: yes: ${
  - `console.log('built-tree exact native laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed');`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:62` [console.log] test-path: no; allocates: yes: ${
  - `console.log('fixed-list-page-oracle checks=${fixedListStorageSelfTests()}');`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:63` [console.log] test-path: no; allocates: yes: ${
  - `console.log('accessibility-projection-twin checks=${accessibilityProjectionSelfTests()}');`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:64` [console.log] test-path: no; allocates: yes: ${
  - `console.log('catalogue-carrier-map-twin checks=${catalogueCarrierMapSelfTests()}');`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:65` [console.log] test-path: no; allocates: yes: ${
  - `console.log('number-controls-twin checks=${numberControlsSelfTests()}');`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:66` [console.log] test-path: no; allocates: yes: ${
  - `console.log('color-input-twin checks=${colorInputSelfTests()}');`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:67` [console.log] test-path: no; allocates: yes: ${
  - `console.log('text-controls-twin checks=${textControlsSelfTests()}');`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:68` [console.log] test-path: no; allocates: yes: ${
  - `console.log('text-field-editing-twin checks=${textFieldEditingSelfTests()}');`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:86` [console.log] test-path: no; allocates: yes: ${
  - `console.log('conformance-corpus-catalog cases=${conformanceCorpusSelfTests()}');`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:122` [console.log] test-path: no; allocates: yes: ${
  - `console.log('ui-contract typescript mirror refreshed -> ${outPath}');`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:148` [console.log] test-path: no; allocates: no
  - `console.log("ui-contract typescript mirror is fresh.");`

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️accessibility-projection/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️accessibility-projection/🟦️.ts:143` [console.log] test-path: yes; allocates: yes: ${
  - `if (import.meta.main) console.log('[TRACE] accessibility-projection-twin checks=${accessibilityProjectionSelfTests()}');`

### `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🔬️graph/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🔬️graph/🟦️.ts:96` [[DEBUG]] test-path: no; allocates: yes: JSON.stringify
  - `if (controlIsUnnamed(record as never, (frame.parent ?? null) as never)) { console.error("[DEBUG] unnamed control", JSON.stringify(record).slice(0, 600), JSON.stringify(frame.parent ?? null).slice(0, 300)); yield* viol...`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🔬️graph/🟦️.ts:155` [[DEBUG]] test-path: no; allocates: yes: JSON.stringify
  - `if (controlIsUnnamed(record as never, (parent ?? null) as never)) { console.error("[DEBUG] unnamed control", JSON.stringify(record).slice(0, 600), JSON.stringify(parent ?? null).slice(0, 300)); yield* violation({ type...`

### `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/📖️stories/🧪️.story.tsx` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/📖️stories/🧪️.story.tsx:160` [console.log] test-path: no; allocates: no
  - `console.log("[story] vfs drag", { active, over });`

### `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/📖️stories/🧪️.story.tsx` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/📖️stories/🧪️.story.tsx:115` [console.log] test-path: no; allocates: yes: new Set
  - `<Table columns={columns} data={sampleData} getRowId={(row) => row.id} selectedRows={new Set(["1", "3"])} onRowClick={(row) => console.log("Clicked:", row.name)} />`

### `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/📖️stories/🧪️.story.tsx` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/📖️stories/🧪️.story.tsx:53` [console.log] test-path: no; allocates: no
  - `onActionClick: () => console.log("Action clicked"),`

### `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕰️HistoryTable/📖️stories/🧪️.story.tsx` (1)

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕰️HistoryTable/📖️stories/🧪️.story.tsx:67` [console.log] test-path: no; allocates: no
  - `console.log("selected checkpoint", checkpointId);`

### `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts` (3)

- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts:58` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[asset] wrote ${artifacts.length} metabolism artifacts');`
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts:77` [console.log] test-path: no; allocates: yes: ${
  - `console.log('@semio-tech/assets wrote ${count} deterministic outputs.');`
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts:90` [console.log] test-path: no; allocates: yes: ${
  - `console.log('@semio-tech/assets ${assetOutputManifest().length} deterministic outputs are fresh.');`

### `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🧭️dispatch/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🧭️dispatch/🧪️tests/🟦️.ts:33` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const program = 'import {createAssetHttpServerV1} from ${JSON.stringify(resolve(import.meta.dir, "../🟦️.ts"))};import {once} from "node:events";const wire=${JSON.stringify(wire)};const provider={kind:wire.kind,plugins...`

### `🧰️framework/🔨️modules/🖼️assets/🗺️tile-proxy/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖼️assets/🗺️tile-proxy/🧪️tests/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `const program = 'import { parseTileProxyAssetSpecV1 } from ${JSON.stringify(resolve(authority, "🟦️.ts"))};const cases=${JSON.stringify(corpus.cases)};console.log(JSON.stringify(cases.map(row=>{try{return{accepted:true...`

### `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🧩️suite/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🧩️suite/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `const program = 'import { parseMeshDeliveryCatalog, meshAssetTransportUrl } from ${JSON.stringify(resolve(import.meta.dir, "../../🟦️.ts"))}; const fixture=${JSON.stringify(fixture)}; const catalog=parseMeshDeliveryCat...`

### `🧰️framework/🔨️modules/🖼️assets/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🖼️assets/🧪️tests/🔬️typescript-package-commands/🟦️.ts:13` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[asset] wrote ${artifacts.length} catalog artifacts → ${target}');`

### `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts` (7)

- `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts:188` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Logo animated SVG created: ${outputPath}');`
- `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts:209` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Parsing ${filePath}...');`
- `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts:212` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Found ${keyframes.length} keyframes');`
- `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts:213` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Will generate ${generateLogoKeyframeSequence(keyframes).length} animation frames');`
- `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts:221` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Exporting ${inputPath} to ${outputPath}...');`
- `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts:230` [console.log] test-path: no; allocates: yes: ${
  - `if (completed % 60 === 0 || completed === total) console.log('[logo] encoded ${completed}/${total} frames');`
- `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🏗️builder/🎞️animation/🟦️.ts:232` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Successfully exported logo to MP4: ${outputPath}');`

### `🧰️framework/🔨️modules/🖼️canvas/🔌️runtime-assets/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🖼️canvas/🔌️runtime-assets/🦀️.rs:210` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[ERROR] guest icon pack ({}) refused: {error}", RuntimeAsset::IconPack.file_name());`

### `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🫳️borrowed-object/🦀️.rs` (3)

- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🫳️borrowed-object/🦀️.rs:125` [console.log] test-path: yes; allocates: yes: JSON.stringify, String::from, serde, to_string(
  - `let output=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const f=JSON.parse(process.argv[1]);if(Buffer.byteLength(f.text,"utf8")!==f.textBytes)throw Error("closed tagged corpus");con...`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🫳️borrowed-object/🦀️.rs:148` [console.log] test-path: yes; allocates: yes: JSON.stringify, String::from, serde, to_string(
  - `let out=std::process::Command::new("bun").args(["-e",r#"import{Database}from'bun:sqlite';const f=JSON.parse(process.argv[1]),word=process.argv[2],actual=process.argv[3];const c=f.cases.find(c=>c.word===word);if(actual...`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🫳️borrowed-object/🦀️.rs:304` [console.log] test-path: yes; allocates: yes: JSON.stringify, to_string(
  - `let oracle=std::process::Command::new("bun").args(["-e",r#"import{Database}from"bun:sqlite";const f=JSON.parse(process.argv[1]);const db=new Database(":memory:");db.run("CREATE TABLE c(n INTEGER, value TEXT, document ...`

### `🧰️framework/🔨️modules/🚪️io/🔤️base64/🧪️tests/🧮️accounting/🔬️budget-oracle.ts` (1)

- `🧰️framework/🔨️modules/🚪️io/🔤️base64/🧪️tests/🧮️accounting/🔬️budget-oracle.ts:9` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Base64 budget oracle: ${cases.length} cases');`

### `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/♻️retirement/🧪️tests/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/♻️retirement/🧪️tests/🦀️.rs:43` [console.log] test-path: yes; allocates: yes: JSON.stringify, String::from, serde, vec!
  - `use std::io::Write;use std::process::{Command,Stdio};let f=fixture();let database=SqliteDatabase{tables:vec![table(&f)]};let limits=&f["sqliteLimits"];let limits=SqliteDatabaseLimits{max_file_bytes:limits["maximumFile...`

### `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🫴️receiving/🧪️tests/🦀️.rs` (3)

- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🫴️receiving/🧪️tests/🦀️.rs:31` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `let script="const {Database}=require('bun:sqlite');(async()=>{const chunks=[];for await(const c of process.stdin)chunks.push(c);const db=Database.deserialize(Buffer.concat(chunks));try{console.log(JSON.stringify(db.qu...`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🫴️receiving/🧪️tests/🦀️.rs:69` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify, String::from, json!
  - `use std::{io::Write,process::{Command,Stdio}};let script="const {Database}=require('bun:sqlite');(async()=>{const chunks=[];for await(const c of process.stdin)chunks.push(c);const db=Database.deserialize(Buffer.concat...`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🫴️receiving/🧪️tests/🦀️.rs:102` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify, String::from, json!
  - `use std::{io::Write,process::{Command,Stdio}};let script="const {Database}=require('bun:sqlite');(async()=>{const chunks=[];for await(const c of process.stdin)chunks.push(c);const db=Database.deserialize(Buffer.concat...`

### `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧪️tests/🫳️reconstruction/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧪️tests/🫳️reconstruction/🦀️.rs:23` [console.log] test-path: yes; allocates: yes: .repeat(
  - `let script=r#"import{Database}from'bun:sqlite';const input=JSON.parse(process.argv[1]);const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const row=db.query('SELECT text_value,blob_value,leng...`

### `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🦀️.rs` (3)

- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🦀️.rs:5` [console.log] test-path: yes; allocates: yes: .map(, .push(, String::from, collect
  - `fn neutral_database_validation_progress_advances_and_cancels_at_the_shared_frontier(){use std::io::Write;use std::process::{Command,Stdio};let f:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧫️fixture...`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🦀️.rs:173` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify, String::from, collect
  - `fn explicit_keyed_projection_completes_in_independent_sqlite_rowid_order_and_cancels_inside_sorting(){use std::io::Write;use std::process::{Command,Stdio};use std::cell::Cell;let f:serde_json::Value=serde_json::from_s...`
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔬️unit/🦀️.rs:203` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `let script="import{Database}from'bun:sqlite';const input=JSON.parse(await Bun.stdin.text()),d=new Database(':memory:');try{d.exec(input.sql);d.exec('INSERT INTO entity DEFAULT VALUES');const value=d.query('SELECT payl...`

### `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🧭️oracle/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🧭️oracle/🦀️.rs:45` [eprintln!] test-path: yes; allocates: yes: println!
  - `fn main() { if let Err(error) = run() { eprintln!("{error}"); std::process::exit(1); } }`

### `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts:44` [console.log] test-path: yes; allocates: yes: ${, .filter(
  - `console.log('resolved-host-context cases=${fixture.invalid.length + 1} guest-rejections=${fixture.guestDecode.length} schema=valid explicit-preferences=required long-fields=${Object.values(schema.properties).filter((f...`

### `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/🔬️unit/🦀️.rs:46` [eprintln!] test-path: yes; allocates: yes: .map(, collect, format!, println!
  - `eprintln!("scoped gate: healthy=ok stranger={:?}", stranger_issues.iter().map(|issue| format!("{}:{}", issue.entity, issue.code)).collect::<Vec<_>>());`

### `🧰️framework/🔨️modules/🧊️3d/🧪️tests/🔬️rust-package-commands/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🧊️3d/🧪️tests/🔬️rust-package-commands/🟦️.ts:34` [console.log] test-path: yes; allocates: yes: ${
  - `const receipts = await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: ...`
- `🧰️framework/🔨️modules/🧊️3d/🧪️tests/🔬️rust-package-commands/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('canonical-mesh-modeling receipts=${receipts.length}');`

### `🧰️framework/🔨️modules/🧬️schema/✅️validator/🏗️compilation/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🧬️schema/✅️validator/🏗️compilation/🧪️tests/🟦️.ts:7` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `test("prepared schema compiler qualifies defining strict types and real Node corpus",async()=>{await receiveScriptProcessInvocation(process.env,async original=>{const control=scriptCompilerAdmissionControlV1(original,...`

### `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust/📜️script.ts` (1)

- `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust/📜️script.ts:20` [console.log] test-path: no; allocates: no
  - `console.log("schema-validator: " + await proveJsonSchemaSubsetContractV1() + " unchanged subset vectors passed");`

### `🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `const program = 'import { compileJsonSchemaSubset, jsonSchemaSubsetValueEquals, validateJsonSchemaSubset } from ${JSON.stringify(validator)}; const corpus=${JSON.stringify(corpus)}; console.log(JSON.stringify({cases:c...`

### `🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🧩️shape/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🧩️shape/🟦️.ts:106` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const runner = ts.transpileModule(helpers + '\nconst rows = JSON.parse(' + JSON.stringify(JSON.stringify(allRows)) + '); console.log(JSON.stringify(rows.map(row => observe(api, row, row.mode ? runtimeValue(row.mode) :...`

### `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/🏃️execution/🟦️.ts` (2)

- `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/🏃️execution/🟦️.ts:32` [console.log] test-path: no; allocates: yes: ${
  - `console.log('entity catalog refreshed (${source.kinds.length} entity kinds, ${shadowed} emoji-shadowed, sha256 ${source.sha256}) -> 🤖️generated/🏷️entity-kinds/🟦️.ts, 🤖️generated/🏷️entity-kinds/🦀️.rs');`
- `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/🏃️execution/🟦️.ts:51` [console.log] test-path: no; allocates: yes: ${
  - `console.log('entity catalog is fresh (${source.kinds.length} entity kinds, sha256 ${source.sha256}).');`

### `🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document/🟦️.ts:98` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] ${spec.name} exact document contracts matched ${snapshots} native snapshots, ${diffs} committed diffs and independent owner/child rejection vectors');`

### `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/📏️ownership/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/📏️ownership/🟦️.ts:51` [console.log] test-path: yes; allocates: no
  - `console.log("[TRACE] independent Node product-removal loader: 58 entries, two planned projections, check and preview executed");', resolveDir: repoRoot, sourcefile: "entity-removal-oracle.ts", loader: "ts" },`

### `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️rust-package-commands/🟦️.ts:27` [console.log] test-path: yes; allocates: no
  - `console.log("schema-subset-contract: " + await proveJsonSchemaSubsetContractV1() + " vectors passed");`

### `🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🪙️return-escrow/🦀️.rs` (1)

- `🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🪙️return-escrow/🦀️.rs:21` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!, static 
  - `fn first_work_refusal(&self,site:&str,delta:u128)->OriginalReturnLedgerIssue{static SEEN:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);if !SEEN.swap(true,std::sync::atomic::Ordering::Relaxed)...`

### `🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧪️tests/🔬️contract/🟦️.ts` (1)

- `🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧪️tests/🔬️contract/🟦️.ts:32` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('job-reconcile-contract: vectors=${fixture.cases.length} ajv+typescript=1 opaque-payload=1');`

### `🧰️framework/🛍️products/🐾️pets/🔨️modules/🎞️animation/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/🐾️pets/🔨️modules/🎞️animation/🧪️tests/🔬️unit/🦀️.rs:422` [eprintln!] test-path: yes; allocates: yes: println!
  - `for banned in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".ln(", ".log", ".powf(", ".powi(", ".mul_add(", ".hypot(", ".cbrt(", ".sqrt(", ".max(", ".min(", ".clamp(", "f64::max", "f64::min", "std::time", "println!...`

### `🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🦀️.rs:49` [eprintln!] test-path: yes; allocates: yes: println!, static 
  - `[".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".mul_add(", ".max(", ".min(", "f64::max", "f64::min", "println!", "eprintln!", "dbg!", "static ", "thread_local!", "HashMap", "BT...`

### `🧰️framework/🛍️products/🐾️pets/🔨️modules/🏞️terrain/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/🐾️pets/🔨️modules/🏞️terrain/🧪️tests/🔬️unit/🦀️.rs:829` [eprintln!] test-path: yes; allocates: yes: println!
  - `for banned in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".ln(", ".log", ".powf(", ".powi(", ".mul_add(", ".hypot(", ".cbrt(", ".max(", ".min(", ".clamp(", "f64::max", "f64::min", "std::time", "println!", "eprint...`

### `🧰️framework/🛍️products/🐾️pets/🔨️modules/👆️gesture/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/🐾️pets/🔨️modules/👆️gesture/🧪️tests/🔬️unit/🦀️.rs:841` [eprintln!] test-path: yes; allocates: yes: println!
  - `"eprintln!",`

### `🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🧪️tests/🏛️world-laws/🦀️.rs` (1)

- `🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🧪️tests/🏛️world-laws/🦀️.rs:58` [eprintln!] test-path: yes; allocates: yes: println!
  - `"eprintln!",`

### `🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🧪️tests/🔬️unit/📜️committed-vectors-and-source/🦀️.rs` (1)

- `🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🧪️tests/🔬️unit/📜️committed-vectors-and-source/🦀️.rs:192` [eprintln!] test-path: yes; allocates: yes: println!, static 
  - `let native = [".max(", ".min(", ".clamp(", "f64::max", "f64::min", "std::time", "println!", "eprintln!", "dbg!", "static ", "thread_local!"];`

### `🧰️framework/🛍️products/🐾️pets/🔨️modules/🧠️behavior/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/🐾️pets/🔨️modules/🧠️behavior/🧪️tests/🔬️unit/🦀️.rs:714` [eprintln!] test-path: yes; allocates: yes: println!
  - `for forbidden in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".mul_add(", ".max(", ".min(", "println!", "eprintln!", "dbg!"] {`

### `🧰️framework/🛍️products/🐾️pets/🔨️modules/🪢️swing/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/🐾️pets/🔨️modules/🪢️swing/🧪️tests/🔬️unit/🦀️.rs:558` [eprintln!] test-path: yes; allocates: yes: println!
  - `for banned in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".ln(", ".log", ".powf(", ".powi(", ".mul_add(", ".hypot(", ".cbrt(", ".max(", ".min(", ".clamp(", "f64::max", "f64::min", "std::time", "println!", "eprint...`

### `🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/🦀️.rs:46` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");`
- `🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/🦀️.rs:51` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("this bridge does not answer {command} {artifact} {standard} {subset} {surface}");`

### `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts` (23)

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:92` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-writer-fence-oracle: behavior-oracles=retained lockKeys=${remoteFixture.postgres.lockKeys.length} laws=${remoteFixture.laws.length}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:280` [console.log] test-path: no; allocates: no
  - `console.log(`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:378` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-writer-authority-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:380` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `for (const receipt of receipts) console.log('wal-writer-authority-native-receipt: ${JSON.stringify(receipt)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:404` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-writer-fence-live ${lane}: ${WAL_WRITER_FENCE_LANES[lane]!.law}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:488` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-committed-transactions-independent-oracle: behavior-oracles=retained u64=1 vectors=${fixture.cases.length}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:533` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-retained-decoder-independent-oracle: behavior-oracles=retained LEB128=1 vectors=${decoder.varints.length}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:567` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-committed-transactions-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:569` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `for (const receipt of receipts) console.log('wal-committed-transactions-native-receipt: ${JSON.stringify(receipt)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:638` [console.log] test-path: no; allocates: no
  - `console.log("wal-committed-compaction-independent-oracle: abort effects excluded, global payloads retained, header-only highest preserved");`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:651` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-committed-compaction-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:653` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('wal-committed-compaction-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:733` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-recovery-independent-oracle: ${fixture.cuts.length} exact CRC/hash-chain prefixes, ${fixture.lifecycle.length} lifecycle rows');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:786` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-recovery ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:788` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('wal-recovery-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:790` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-recovery-check: ${fixture.cuts.length + fixture.lifecycle.length + fixture.fragmentCopies.length + failStop.cases.length} checks clean');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:829` [console.log] test-path: no; allocates: no
  - `console.log("wal-capacity-independent-oracle: Fsync/grouped rotation and exact/one-over capacity confirmed by LEB128");`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:835` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-capacity ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:837` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('wal-capacity-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:839` [console.log] test-path: no; allocates: no
  - `console.log("wal-capacity-check: 6 checks clean");`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:885` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-segment-state ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:887` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('wal-segment-state-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/✍️wal-checks/🟦️.ts:889` [console.log] test-path: no; allocates: yes: ${
  - `console.log('wal-segment-state-check: checks=${checks} clean');`

### `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/👥️member-dialect-checks/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/👥️member-dialect-checks/🟦️.ts:124` [console.log] test-path: no; allocates: yes: ${
  - `console.log('exact member admission laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed across ${receipts.length} verified test executables');`

### `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts` (13)

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:287` [console.log] test-path: no; allocates: yes: ${
  - `console.log('document-mount-single-flight-independent-oracle: behavior-oracles=retained cases=${fixture.cases.length} waiters=${fixture.capacity.waitersPerDocument} owner-futures=${fixture.capacity.ownerFuturesPerDocu...`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:323` [console.log] test-path: no; allocates: yes: ${
  - `console.log('document-mount-single-flight-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:325` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('document-mount-single-flight-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:367` [console.log] test-path: no; allocates: yes: ${
  - `console.log('durable-owned-group-decision-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:369` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `for (const receipt of receipts) console.log('durable-owned-group-decision-native-receipt: ${JSON.stringify(receipt)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:476` [console.log] test-path: no; allocates: no
  - `console.log(`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:481` [console.log] test-path: no; allocates: yes: ${
  - `console.log('durable-group-journal-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:483` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('durable-group-journal-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:498` [console.log] test-path: no; allocates: yes: ${
  - `console.log('document-opening-attempt-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:500` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('document-opening-attempt-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:530` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[reopen-storm] throughput example: storm ${throughput.storm.documents}×${throughput.storm.batches}×${throughput.storm.batchEdits}, storm/serial ≤ ${throughput.bounds.stormToSerialRatioMax}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:541` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[reopen-storm] ${index + 1}/${wanted.length} ${name}: ${law}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts:592` [console.log] test-path: no; allocates: no
  - `console.log("document-http: Ajv vectors, removal/reinstall, scope, bounds and owner decoding passed");`

### `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts` (9)

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts:124` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('directory-session-authority-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts:126` [console.log] test-path: no; allocates: yes: ${
  - `console.log('directory-session-authority-check: checks=${checks} clean');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts:203` [console.log] test-path: no; allocates: yes: ${
  - `console.log('directory-event-page-contract ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts:205` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('directory-event-page-contract-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts:207` [console.log] test-path: no; allocates: yes: ${
  - `console.log('directory-event-page-contract-check: checks=${checks} clean');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts:217` [console.log] test-path: no; allocates: yes: ${
  - `console.log('directory-event-page-client ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts:219` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('directory-event-page-client-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts:221` [console.log] test-path: no; allocates: yes: ${
  - `console.log('directory-event-page-client-check: checks=${checks} clean');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📇️directory-checks/🟦️.ts:263` [console.log] test-path: no; allocates: yes: ${
  - `console.log('directory-event-page-bootstrap-check: checks=${directoryEventPageBootstrapOracle(this.repoRoot)} clean');`

### `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts` (15)

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:48` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log("paged-history-canonical-oracles: " + JSON.stringify(canonical));`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:57` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `for (const receipt of receipts) console.log("paged-history-stack-native-receipt: " + JSON.stringify(receipt));`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:59` [console.log] test-path: no; allocates: yes: ${
  - `console.log('paged-history-stack-check: vectors=${fixture.vectors.length} independent-array/Ajv=passed');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:103` [console.log] test-path: no; allocates: no
  - `console.log("database-history-completion-check: behavior-oracles=retained sqlite-publication=" + fixture.publication.length);`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:117` [console.log] test-path: no; allocates: no
  - `console.log("database-history-completion-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:119` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `for (const receipt of receipts) console.log("database-history-completion-native-receipt: " + JSON.stringify(receipt));`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:197` [console.log] test-path: no; allocates: no
  - `console.log("database-catalog-read-ownership-check: behavior-oracles=retained sqlite-transfers=" + fixture.transfers.length + " sqlite-completion=" + fixture.completion.length + " sqlite-recovery=" + fixture.recovery....`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:216` [console.log] test-path: no; allocates: no
  - `console.log("database-catalog-read-ownership-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:218` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `for (const receipt of receipts) console.log("database-catalog-read-ownership-native-receipt: " + JSON.stringify(receipt));`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:316` [console.log] test-path: no; allocates: no
  - `console.log("database-capability-completion-check: behavior-oracles=retained sqlite-publication=" + fixture.publication.length + " sqlite-retirement=" + fixture.retirement.length + " sqlite-drive-ownership=" + fixture...`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:344` [console.log] test-path: no; allocates: no
  - `console.log("database-capability-completion-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:346` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `for (const receipt of receipts) console.log("database-capability-completion-native-receipt: " + JSON.stringify(receipt));`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:407` [console.log] test-path: no; allocates: yes: ${
  - `console.log('database-shutdown-independent-oracle: behavior-oracles=retained cases=${fixture.cases.length} retained-authority=1 terminal-ack=1');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:419` [console.log] test-path: no; allocates: yes: ${
  - `console.log('database-shutdown-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🗄️database-checks/🟦️.ts:421` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('database-shutdown-native-receipts: ${JSON.stringify(receipts)}');`

### `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🧪️source-and-native-tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🧪️source-and-native-tests/🟦️.ts:87` [console.log] test-path: yes; allocates: no
  - `console.log("canonical-architecture-native " + event.stage + ": " + (event.law ?? ""));`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🧪️source-and-native-tests/🟦️.ts:89` [console.log] test-path: yes; allocates: no
  - `console.log("canonical-architecture-native receipts=" + receipts.length);`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🧪️source-and-native-tests/🟦️.ts:193` [console.log] test-path: yes; allocates: yes: ${
  - `const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMillisecon...`

### `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🧬️retained-clone-check/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🧬️retained-clone-check/🟦️.ts:282` [console.log] test-path: no; allocates: yes: ${
  - `console.log('retained-clone-check: choices=${fixture.source.choices.length} payload=${fixture.payloadByteLength} cancellation=${fixture.cancellationStops.length} orderedMap=${ordered.length} growth=${growth.size} prep...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/🖼️presentation/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/🖼️presentation/🧪️tests/🔬️rust-package-commands/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[directed-presentation] ${corpus.ports.length} portable port vectors admitted by Ajv; native implementation remains separately qualified');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🌱️process-arena/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🌱️process-arena/🦀️.rs:40` [eprintln!] test-path: no; allocates: yes: println!
  - `pub fn exit(mut self)->!{self.with_failures_step(|failure,cx|{let mut writer=FailureWriter{cx,refusal:None};if std::fmt::write(&mut writer,format_args!("{failure:?}\n")).is_err(){return Err(writer.refusal.unwrap_or_el...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🌱️process-arena/🦀️.rs:63` [eprintln!] test-path: no; allocates: yes: String::from, println!, serde
  - `#[cfg(test)]mod tests{use super::*;#[test]fn process_arena_returns_normally_only_after_real_source_empty(){if let Ok(mode)=std::env::var("SEMIO_MCP_PROCESS_ARENA_CHILD"){let policy=serde_json::from_str(include_str!("....`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs` (8)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs:288` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error) = install_credential_exchange_protocols_v1(credentials) { eprintln!("MCP credential protocol inventory refused: {}", error.message); std::process::exit(1); }`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs:289` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error)=install_remote_inference_protocols_v1(services) {eprintln!("MCP installed service inventory refused: {}",error.message);std::process::exit(1);}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs:298` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp compile-component] {message}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs:304` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] protected parent environment was not sealed");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs:309` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] benign direct-child environment was not preserved");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs:315` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] protected MCP credential claim failed");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs:331` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] direct-child descendant seal failed");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs:338` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] {message}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🧪️tests/🧪️canonical-pair-oracle/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🧪️tests/🧪️canonical-pair-oracle/🟦️.ts:154` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('canonical-pair-cache-mount oracle: 3/3; negatives ${rejected}/${fixture.negativeVectors.length}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔬️probe-document/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔬️probe-document/🦀️.rs:302` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[probe-store-close] cannot reach terminal-empty: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔬️probe-document/🦀️.rs:307` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[probe-store-close] did not reach terminal-empty within its bounded turn budget");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/⏱️compile-cancellation-law/🦀️.rs` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/⏱️compile-cancellation-law/🦀️.rs:75` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("compile-law component '{plugin_id}': {} bytes", bytes.len());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/⏱️compile-cancellation-law/🦀️.rs:124` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("compile-law {expected}: requester {index} cancelled, answered in {} µs", waited.as_micros());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/⏱️compile-cancellation-law/🦀️.rs:129` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("compile-law {expected}: workers started {started}, compiled {compiled:?}, scratch {scratch:?}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs` (22)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:187` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:194` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: plugin registry not generated: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:199` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: 'note' not in the plugin registry");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:203` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: the launcher has no retained note component model");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:267` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:271` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: plugin registry not generated");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:275` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: 'note' not in the plugin registry");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:279` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: the launcher has no retained note component model");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:308` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:315` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: plugin registry not generated: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:320` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: 'note' not in the plugin registry");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:324` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: the launcher has no retained note component model");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:420` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:424` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: plugin registry not generated");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:428` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: 'note' not in the plugin registry");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:432` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: the launcher has no retained note component model");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:436` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: the compiled catalog publishes no 'note.….addBlock' capability");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:521` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:525` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: plugin registry not generated");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:529` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: 'note' not in the plugin registry");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:533` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: the launcher has no retained note component model");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs:537` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: the compiled catalog publishes no 'note.….addBlock' capability");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🏁️child-heads-hub-commit-and-probe-pack/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🏁️child-heads-hub-commit-and-probe-pack/🦀️.rs:48` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{error}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🧭️plugin-capability-routing/🦀️.rs` (4)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🧭️plugin-capability-routing/🦀️.rs:123` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🧭️plugin-capability-routing/🦀️.rs:130` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: plugin registry not generated: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🧭️plugin-capability-routing/🦀️.rs:136` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: '{plugin_id}' not in the plugin registry");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🧭️plugin-capability-routing/🦀️.rs:140` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("skipped: the launcher has no retained component model for {plugin_id}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧪️tests/⏱️binding-cancellation-law/🦀️.rs` (4)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧪️tests/⏱️binding-cancellation-law/🦀️.rs:126` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("binding-law case {:?}: reported {:?}", cancel_at.map(ActivationPhase::id), reported.iter().map(|phase| phase.id()).collect::<Vec<_>>());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧪️tests/⏱️binding-cancellation-law/🦀️.rs:133` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("binding-law case {}: cancel answered in {} µs", target.id(), waited.as_micros());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧪️tests/⏱️binding-cancellation-law/🦀️.rs:155` [eprintln!] test-path: yes; allocates: yes: .map(, collect, println!
  - `eprintln!("create-law case {:?}: reported {:?}", cancel_at.map(ActivationPhase::id), reported.iter().map(|phase| phase.id()).collect::<Vec<_>>());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧪️tests/⏱️binding-cancellation-law/🦀️.rs:162` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("create-law case {}: cancel answered in {} µs", target.id(), waited.as_micros());`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🦀️.rs:66` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mcp registry] {diagnostic}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🧰️catalog-source-discovery/🦀️.rs` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🧰️catalog-source-discovery/🦀️.rs:20` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mcp registry] {diagnostic}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🧰️catalog-source-discovery/🦀️.rs:25` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mcp registry] discovery root {} is not a directory — serving gateway-only capabilities", resolved.display());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📇️registry/🧰️catalog-source-discovery/🦀️.rs:29` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mcp registry] could not locate a repo/space root — serving gateway-only capabilities");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/🧪️tests/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const program='const{receiveScriptProcessInvocation,createIssuedResourceControlV1}=await import(${JSON.stringify(routing)});const{withMcpClientSessionV1}=await import(${JSON.stringify(owner)});const mode=${JSON.string...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🚪️gateway/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🚪️gateway/🧪️tests/🟦️.ts:159` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const peer='process.on("SIGTERM",()=>{});require("node:readline").createInterface({input:process.stdin}).on("line",line=>{const m=JSON.parse(line);if(m.method==="initialize")console.log(JSON.stringify({jsonrpc:"2.0",i...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts:23` [console.log] test-path: no; allocates: yes: ${
  - `try{console.log('[test] ${requireMcpBinary(this.repoRoot)}');await runVitest(this.root,rest,"../../🧪️tests/🎚️config/🟦️.ts");}`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts` (11)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:88` [console.log] test-path: no; allocates: no
  - `console.log("mcp-entrypoint-credential-marker: clean=accepted non-fd3=rejected redacted=1");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:113` [console.log] test-path: no; allocates: yes: ${
  - `console.log('mcp-build-release: profile=release staged=${staged}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:203` [console.log] test-path: no; allocates: yes: ${, .join(
  - `console.log('canonical-pair-laws: ${laws.join(" ")}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:244` [console.log] test-path: no; allocates: no
  - `console.log("hub-live-catalog-oracle: AJV=1 states=" + fixture.states.length + " hostile=" + fixture.hostile.length + " execution-protocol=compiled-lease local-fallback=denied");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:263` [console.log] test-path: no; allocates: no
  - `console.log("hub-live-catalog-source: workspace=3 remote=4 inference=no-registry-fallback tools=live-selection-projection");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:282` [console.log] test-path: no; allocates: no
  - `console.log("hub-live-catalog-native: laws=" + suffixes.length + " authenticated-selection=1 revoked-fallback=denied");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:312` [console.log] test-path: no; allocates: yes: ${
  - `console.log('canonical-checkpoint-resource-oracle: AJV=1 parts=2 untrusted=1 hostile=${fixture.hostile.length} lifecycle=${fixture.lifecycle.length}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:329` [console.log] test-path: no; allocates: no
  - `console.log("canonical-checkpoint-resource-source: uri=scope-exact retained-pair=private final-fence=2 raw-limit=4MiB text-limit=6MiB");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:347` [console.log] test-path: no; allocates: yes: ${
  - `console.log('canonical-checkpoint-resource-native: laws=${suffixes.length} retained-pair=1');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:430` [console.log] test-path: no; allocates: yes: ${
  - `console.log('capability-description oracle: rust=${rustPlugin.size} ajv=${oracle.size} rust-only=${onlyRust.length} ajv-only=${onlyOracle.length}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:468` [console.log] test-path: no; allocates: yes: ${
  - `console.log('schema-mirror${check ? "-check" : ""}: exports=${Object.keys(document.$defs).length} ajv-draft07-resolved=${Object.keys(document.$defs).length} json=1 typescript=1');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗒️core-capabilities/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗒️core-capabilities/🦀️.rs:106` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[mcp registry] catalog compile failed ({error}) — falling back to gateway-only capabilities");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts:372` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${step.ok ? "PASS" : "FAIL"}  ${step.step} — ${step.detail}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` (8)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:583` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] hub binding is not settled yet ({}); retry {attempts_made}/{HUB_OPEN_RETRY_ATTEMPTS} in {backoff_ms} ms", error.message);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:617` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] acting as agent principal {} (\"{}\") in space {}", grant.agent_principal_id, grant.agent_label, grant.space_id);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:637` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] real per-capability ArtifactChannel routing bound for {origin_label}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:673` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] the loopback bridge listener could not bind ({}) — continuing over stdio with no live-shell surface", error.message);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:703` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] bridge listening on ws://{}/bridge — offered to {offered_to} via {}", run.local_addr(), published.path().display());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:714` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] the bridge offer could not be published ({}) — cancelling the listener rather than leaving an unreachable socket open", error.message);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:739` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:794` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-os-mcp] bridge listening on ws://{bind_ip}:{}/bridge", options.port);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🚪️canonical/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🚪️canonical/🟦️.ts:124` [console.log] test-path: yes; allocates: no
  - `console.log("[test] Canonical OS MCP protocol passed; actual owned shutdown recorded separately from target whole status");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust/📜️script.ts:31` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Flow browser package preview: ${preview.files.length} files, ${preview.browserBytes} browser bytes, ${preview.declarationBytes} declaration bytes');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/🧪️tests/🔬️rust-package-commands/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/🧪️tests/🔬️rust-package-commands/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('flow extension manifest admission twin: ${flowExtensionManifestAdmissionSelfTests()} assertions');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/🧪️tests/🔬️rust-package-commands/🟦️.ts:44` [console.log] test-path: yes; allocates: no
  - `console.log("Flow packaged browser entry preserves its generated initializer and owned host sibling without external source-tree paths");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/📇️catalogue-owner/🧪️laws/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/📇️catalogue-owner/🧪️laws/🦀️.rs:25` [[DEBUG]] test-path: no; allocates: no
  - `let panicked=std::panic::catch_unwind(||{let _owner=FlowOperatorCatalogueOwner::new(flow_extension_registry(),flow_extension_registry_generation());panic!("[DEBUG] expected test-body failure")});`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs:544` [eprintln!] test-path: no; allocates: yes: println!
  - `/// producer that drops this on the floor (the old 'flowTessellate skipped: …' 'eprintln!') turns a`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧪️tests/🪪️manifest-admission/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧪️tests/🪪️manifest-admission/🟦️.ts:98` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[twin] flow extension manifest admission: ${flowExtensionManifestAdmissionSelfTests()} assertions');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🌱️bootstrap/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🌱️bootstrap/🟦️.ts:27` [console.log] test-path: yes; allocates: no
  - `console.log("Flow source bootstrap: actual synchronous registry load, cold manifest materialization and Ajv fixture parity PASS");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🏷️browser-ownership/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🏷️browser-ownership/🟦️.ts:121` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Flow browser ownership: ${fixture.owners.length} owners, ${fixture.consumers.length} consumers, ${fixture.target.inputs.length} exact Nx inputs, finite manifest DATA, private producer refusal and independ...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🖼️display/🧪️tests/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🖼️display/🧪️tests/🦀️.rs:27` [[DEBUG]] test-path: yes; allocates: no
  - `turns+=1;assert!(turns<100_000,"[DEBUG] display never completed under {items} item(s) per turn");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🖼️display/🧪️tests/🦀️.rs:41` [[DEBUG]] test-path: yes; allocates: yes: println!
  - `println!("[DEBUG] display turns: 1 item={single}, 64 items={mounted}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🔁️extension-bridge-and-chain-census-laws/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🔁️extension-bridge-and-chain-census-laws/🦀️.rs:154` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("wave cancel: retiredTessellations={retired} rearms={rearms}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts:60` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Flow browser declarations: ${fixture.operationMethods} schema methods, runtime prototype and TypeScript parser parity; 3 package exports and 2 TypeScript resolutions; 5 hostile fixtures rejected; strictAj...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🎒️mesh-pack-wire/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🎒️mesh-pack-wire/🦀️.rs:64` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("mesh wire: json={json_bytes} B, pack={pack_bytes} B, base64={} B", encode_base64(&encode_mesh_pack(&mesh).expect("encode")).len());`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/💾️binary/🦀️.rs` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/💾️binary/🦀️.rs:5` [eprintln!] test-path: no; allocates: yes: clone(, println!
  - `let context=match admit_command_transport(semio_os_kernel::os_pack::PackLimits::default().max_total_alloc,cancellation.clone(),print_command_progress){Ok(context)=>context,Err(error)=>{eprintln!("transport admission: ...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/💾️binary/🦀️.rs:6` [eprintln!] test-path: no; allocates: yes: clone(, println!
  - `let command=match CommandContext::try_new(context,semio_os_kernel::os_pack::PackLimits::default().max_total_alloc,cancellation.clone(),print_command_input_progress){Ok(command)=>command,Err(error)=>{eprintln!("input a...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/💾️binary/🦀️.rs:7` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error)=configure_stdin_cancellation(&mut args,command.transport(),cancellation){eprintln!("cancel input: {error}");std::process::exit(1)}`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs` (31)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:209` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: pack inspect <file>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:215` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: cannot open '{path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:248` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: failed to open manifest: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:258` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: recovery scan also failed: {recover_error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:274` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: pack verify <file> [--level=trusted|standard|full]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:280` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:320` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: pack hash <file>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:326` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: cannot read '{path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:336` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:349` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: pack to-dsl <file> --schema <name>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:353` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: to-dsl requires --schema <name> (registry: {})", registry_names());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:357` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: unknown --schema '{schema_name}'; available: {}", registry_names());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:363` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: cannot read '{path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:373` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("note: unknown field ids not in schema '{schema_name}': {:?}", report.unknown_field_ids);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:376` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("note: schema_hash drift between file and registry spec '{schema_name}'");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:381` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: decode failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:394` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: pack from-dsl <file> --schema <name> --out <file>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:398` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: from-dsl requires --schema <name> (registry: {})", registry_names());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:402` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: from-dsl requires --out <file>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:406` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: unknown --schema '{schema_name}'; available: {}", registry_names());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:412` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: cannot read '{path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:419` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: dsl parse failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:426` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: encode failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:436` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: write failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:469` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: pack diff <file-a> <file-b> [--schema <name>]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:477` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: cannot read '{path_a}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:484` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: cannot read '{path_b}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:491` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: unknown --schema '{schema_name}'; available: {}", registry_names());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:498` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: decode '{path_a}' failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:505` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: decode '{path_b}' failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️.rs:578` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("pack: unknown subcommand '{other}'\n");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🚦️control/🦀️.rs` (4)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🚦️control/🦀️.rs:72` [eprintln!] test-path: no; allocates: yes: .map(, println!
  - `loop{let mut byte=[0u8;1];let read=input_context.operation(PackTransportCategory::NativeIo,PackRetryDisposition::Never,1,||input.read(&mut byte).map(|count|(count,count)));match read{Ok(0)=>break,Ok(_)=>{if byte[0]==b...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🚦️control/🦀️.rs:77` [eprintln!] test-path: no; allocates: yes: println!
  - `pub fn print_command_progress(event:PackTransportProgress){if event.phase==semio_os_pack::PackTransportPhase::AfterOperation&&event.bytes>0{eprintln!("Processed {} bytes",event.bytes);}}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🚦️control/🦀️.rs:79` [eprintln!] test-path: no; allocates: yes: println!
  - `pub fn configure_stdin_cancellation(args:&mut Vec<String>,context:&PackTransportContext,cancellation:CancelToken)->Result<(),PackError>{if let Some(index)=args.iter().position(|arg|arg=="--cancel-stdin"){args.remove(i...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🚦️control/🦀️.rs:81` [eprintln!] test-path: no; allocates: yes: println!
  - `pub fn print_command_input_progress(event:CommandInputProgress){if event.requested_bytes>0{eprintln!("Reading {} bytes",event.requested_bytes);}if event.validated_bytes>0{eprintln!("Validated {} bytes",event.validated...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs` (6)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs:205` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[os run] {message}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs:210` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[os run] --watch is not implemented yet — re-run the command instead for now (see HEADLESS-MEDIA-CONTRACT follow-up)");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs:214` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[os run] {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs:327` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[os run] io-router: {io_router_plugins} plugins / {io_router_keys} keys");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs:328` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[os run] merge-policy: {:?}", args.policy);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs:353` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[os run] {}: loading document {}/{}", progress.node_id, progress.completed, progress.total);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs:108` [[DEBUG]] test-path: no; allocates: no
  - `/// 🐛️ [DEBUG] Names the branch 'advance_apply_batch_inner' is about to take, mirroring its dispatch order, for the temporary mutation-latency histogram.`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🦀️.rs:20` [eprintln!] test-path: no; allocates: yes: println!
  - `std::panic::set_hook(Box::new(|panic| eprintln!("[semio-worker panic] {panic}")));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🏁️publication/📚️metadata/🧪️tests/🔬️digest-oracle.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🏁️publication/📚️metadata/🧪️tests/🔬️digest-oracle.ts:7` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('original digest oracle: ${cases.length} cases');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧪️tests/🔗️source-custody/🦀️.rs` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧪️tests/🔗️source-custody/🦀️.rs:150` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `let mut oracle=std::process::Command::new("bun").args(["-e",r#"import {Database} from "bun:sqlite";const law=JSON.parse(await Bun.stdin.text());const db=new Database(":memory:");db.exec("create table nodes(id text pri...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧪️tests/🔗️source-custody/🦀️.rs:169` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `let mut oracle=std::process::Command::new("bun").args(["-e",r#"import {Database} from "bun:sqlite";const law=JSON.parse(await Bun.stdin.text());const db=new Database(":memory:");console.log(JSON.stringify(law.cases.ma...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧪️tests/🔗️source-custody/🦀️.rs:204` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `let mut oracle=std::process::Command::new("bun").args(["-e",r#"import Ajv from "ajv";import {Database} from "bun:sqlite";const {schema,law}=JSON.parse(await Bun.stdin.text());const validate=new Ajv({strict:true}).comp...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧪️tests/🔐️authority/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧪️tests/🔐️authority/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({name:row.name,actual}));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧪️tests/🔐️authority/🟦️.ts:28` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({oracle:"fast-json-patch",cases,status:"passed"}));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌱️initial/🪪️identity/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌱️initial/🪪️identity/🧪️tests/🔬️unit/🦀️.rs:83` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("initial-child identity vectors={} coordinate substitutions={} denials={} scope-agreement=3 full-width frames=2 independent=blake3 authority=none", rows.len(), rows.len() * 9, denied);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🧪️tests/🟦️.ts:38` [console.log] test-path: yes; allocates: no
  - `const result=Bun.spawnSync([productionForest]);writeFileSync(resolve(output,"runtime.log"),new TextDecoder().decode(result.stdout)+new TextDecoder().decode(result.stderr));expect(result.exitCode).toBe(0);console.log(n...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs:69` [[DEBUG],eprintln!] test-path: no; allocates: yes: collect, println!, static 
  - `let receipt=if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,p)}else{cx.consume_retained(p)};cx.consume_fuel(p.copied_items.saturating_add(p.copied_bytes)as u64);{thread_local!{stat...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🍰️companion/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🍰️companion/🔬️unit/🦀️.rs:165` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("retained member sequence={sequence} wire={wire_ptr:p} bytes={byte_count} value={oracle}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗄️durable-owned-group/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗄️durable-owned-group/🟦️.ts:201` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('durable-owned-group-decision-oracle: AJV=1 SHA256=1 roles=3 cases=${fixture.cases.length} event=${fixture.limits.eventBytes} whole=${exactWholeSegmentBytes} member=${fixture.limits.recoveryPackBytes}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:138` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("pack value quote violations: {violations:?}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧷️assembly/⏳️lifetime/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧷️assembly/⏳️lifetime/🧪️tests/🟦️.ts:42` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('[assembly-lifetime] scenario=${id} stage=${stage} exit=${result.exitCode} diagnostics=${codes.join(",")}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧪️tests/🔬️contract/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧪️tests/🔬️contract/🟦️.ts:40` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('local-session-broker-contract: vectors=${fixture.cases.length} ajv+typescript=1 live-exchange=1 stale-run=refused foreign-profile=refused');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧰️wire-helpers-and-credentials/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧰️wire-helpers-and-credentials/🦀️.rs:80` [eprintln!] test-path: no; allocates: yes: .map(, collect, format!, println!
  - `eprintln!("[semio-directory-client] socket-grant-selector-digest {path} {}", digest.iter().map(|byte| format!("{byte:02x}")).collect::<String>());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧰️wire-helpers-and-credentials/🦀️.rs:94` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-directory-client] document-socket-grant {path} {}", EXCHANGES.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/💾️binary/🦀️.rs` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/💾️binary/🦀️.rs:5` [eprintln!] test-path: no; allocates: yes: clone(, println!
  - `let context=match admit_command_transport(semio_os_kernel::os_spr::ProtocolLimits::default().max_total_alloc,cancellation.clone(),print_command_progress){Ok(context)=>context,Err(error)=>{eprintln!("transport admissio...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/💾️binary/🦀️.rs:6` [eprintln!] test-path: no; allocates: yes: clone(, println!
  - `let command=match CommandContext::try_new(context,semio_os_kernel::os_spr::ProtocolLimits::default().max_total_alloc,cancellation.clone(),print_command_input_progress){Ok(command)=>command,Err(error)=>{eprintln!("inpu...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/💾️binary/🦀️.rs:7` [eprintln!] test-path: no; allocates: yes: println!
  - `if let Err(error)=configure_stdin_cancellation(&mut args,command.transport(),cancellation){eprintln!("cancel input: {error}");std::process::exit(1)}`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs` (41)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:217` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: protocol inspect <file>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:223` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: cannot read '{path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:228` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: '{path}' is shorter than the {HEADER_SIZE}-byte header");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:253` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: stopped scanning at a malformed frame: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:302` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: protocol verify <file> [--level=trusted|standard|full]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:308` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:342` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: protocol hash <file>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:348` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: cannot read '{path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:358` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:372` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: protocol log <file> [--limit N] [--actor ID] [--alternative ID] [--reverse]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:380` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: --limit expects a non-negative integer, got '{value}'");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:391` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: cannot read '{path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:399` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:406` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:415` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: unknown --alternative '{alternative_id}'");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:478` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: protocol compile <doc.ops> [--out doc.spr]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:484` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: cannot read '{path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:491` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: compile failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:502` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: write failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:509` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: stdout write failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:523` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: protocol decompile <doc.spr> [--out doc.ops]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:529` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: cannot read '{path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:536` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: decompile failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:547` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: write failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:567` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: protocol diff <a.spr> <b.spr>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:575` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: cannot read '{path_a}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:582` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: cannot read '{path_b}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:590` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: decode '{path_a}' failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:597` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: decode '{path_b}' failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:649` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: protocol compact <file> [--out FIXED]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:654` [eprintln!] test-path: no; allocates: yes: println!
  - `let bytes=match semio_os_pack_tools::control::read_command_file(Path::new(path),semio_os_spr::ProtocolLimits::default().max_file_len,context).await{Ok(bytes)=>bytes,Err(error)=>{eprintln!("protocol: cannot read for co...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:656` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: cannot copy '{path}' to '{out_path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:670` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: compact failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:689` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: protocol repair <file> [--truncate-torn-tail] [--rebuild-indexes]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:696` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: recovery failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:709` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: cannot open '{path}' for truncation: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:714` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: truncate failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:733` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: protocol upgrade <file>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:739` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: cannot read '{path}': {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:750` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: upgrade check failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️.rs:798` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("protocol: unknown subcommand '{other}'\n");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🗣️verb-vocabulary/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🗣️verb-vocabulary/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('mutation-verb-vocabulary: pairs=${ontology.length} vectors=${fixture.cases.length} Ajv/source parity=green');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🗣️verb-vocabulary/🟦️.ts:28` [console.log] test-path: yes; allocates: yes: ${
  - `const laws = await runAdmittedExactCargoLawsV1({ invocation, control, policy: configuredExactCargoLawPolicyV1(), cwd: repoRoot, progress: (event) => console.log('mutation-verb-native ${event.stage} ${event.law ?? even...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🗣️verb-vocabulary/🟦️.ts:29` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('mutation-verb-native: exact=${laws.length} passed');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts:300` [console.log] test-path: no; allocates: no
  - `console.log("framework-renderer-react: region/host-contract lint passed");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/🟦️.ts:52` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Published native ${variant} ${readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode}: ${modules.length} completed components');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs` (7)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs:80` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] native protected credential environment rejected");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs:84` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] native local credential claim failed");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs:132` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("native boot descriptor rejected: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs:137` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] --scale requires --scale-wasm <fixture.wasm>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs:141` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] --scale requires --report <out.json>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs:148` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("scale-bench: original frame publication refused: {refusal}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs:156` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("SEMIO_PLUGIN_MODULES is unset: run through 'repo run @semio-tech/framework-os-dev:run-<variant>-native-<profile>'");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/📜️script.ts:19` [console.log] test-path: no; allocates: no
  - `console.log("framework-renderer-wgpu: generated " + this.entry);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:922` [eprintln!] test-path: no; allocates: yes: println!
  - `/// This is the same class of defect as the 27 dead 'eprintln!' traces`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts:41` [console.log] test-path: no; allocates: no
  - `}, host, port, signal: AbortSignal.any([this.invocation.control.signal, controller.signal]), ready: url => console.log("WGPU browser ready: " + url + "?plugin=" + variant) });`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎭️scene-state/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎭️scene-state/🦀️.rs:272` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] scene retirement map host={} before={before:?} exact-interaction={_interaction}", owner.host_id);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts:61` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `writeFileSync(owned(join(directory,"wasm.json")),JSON.stringify(receipt,null,2)+"\n");completed=receipt;await actual.recheck();await products.recheck();console.log("[renderer-wasm] Published "+outputs.length+" owned b...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏛️host-base/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏛️host-base/🦀️.rs:118` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{message}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts` (13)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:78` [console.log] test-path: no; allocates: no
  - `console.log("native-environment-check: poisoned ordinary runner sanitized and binary fail-closed guard precedes credential/plugin activation");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:187` [console.log] test-path: no; allocates: yes: ${
  - `const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMillisec...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:188` [console.log] test-path: no; allocates: yes: ${
  - `console.log('media-slot-contract: ${receipts.reduce((count, receipt) => count + receipt.laws.length, 0)} exact native laws passed');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:306` [console.log] test-path: no; allocates: yes: ${
  - `console.log('directory-retained-home-bootstrap-source-check: checks=${directoryRetainedHomeBootstrapOracle()} clean');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:327` [console.log] test-path: no; allocates: yes: ${
  - `}],progress(event) { console.log('directory-retained-home-bootstrap ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}'); }});`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:328` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('directory-retained-home-bootstrap-native-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:329` [console.log] test-path: no; allocates: yes: ${
  - `console.log('directory-retained-home-bootstrap-native-check: sourceChecks=${checks} nativeLaws=3 clean');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:351` [console.log] test-path: no; allocates: yes: ${
  - `console.log('normalized-presence-rows-source-check: checks=${normalizedPresenceRowsOracle()} clean');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:365` [console.log] test-path: no; allocates: yes: ${
  - `console.log('normalized-presence-rows-native-check: sourceChecks=${checks} nativeLaws=1 clean');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:450` [console.log] test-path: no; allocates: no
  - `console.log("framework-renderer-wgpu: generated 🎞️frame-worker.js");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:467` [console.log] test-path: no; allocates: no
  - `console.log("framework-renderer-wgpu: 🎞️frame-worker.js is fresh");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:494` [console.log] test-path: no; allocates: yes: ${
  - `console.log('framework-renderer-wgpu: color-literal and artifact-home lint passed (${artifactChecks} checks)');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:516` [console.log] test-path: no; allocates: no
  - `console.log("wgpu-boot-cache-inputs: descriptors=18 refusals=5 Bun+Node+Ajv; ambient-session-reads=0 exhaustive-source-inputs=true");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔤️runtime-assets/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔤️runtime-assets/🦀️.rs:81` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] native runtime assets unavailable: {error}; icons and labels will not render");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📊️scale-bench/🦀️.rs` (5)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📊️scale-bench/🦀️.rs:855` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error)=>{eprintln!("scale-bench: failed to read {}: {error}",registry_path.display());return Ok(1)}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📊️scale-bench/🦀️.rs:862` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("scale-bench: failed to parse {}: {error}", registry_path.display());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📊️scale-bench/🦀️.rs:868` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error)=>{eprintln!("scale-bench: failed to read {}: {error}",wasm_path.display());return Ok(1)}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📊️scale-bench/🦀️.rs:878` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("scale-bench: compile failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📊️scale-bench/🦀️.rs:891` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("scale-bench: refusing to write populated report {} until its retained page encoder is mounted (renderer=native, shards={shard_count}, records={}, wasm={})", report_path.display(), registry.records.len(), wa...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔬️native-socket-probe/🦀️.rs` (4)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔬️native-socket-probe/🦀️.rs:249` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => { eprintln!("smoke: {error}"); return 1; }`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔬️native-socket-probe/🦀️.rs:261` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("smoke: load_wasm_plugins failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔬️native-socket-probe/🦀️.rs:270` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("smoke: shell.boot() failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔬️native-socket-probe/🦀️.rs:299` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("smoke: report encode failed: {error}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/📐️scene-shading-oracle/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/📐️scene-shading-oracle/🟦️.ts:149` [console.log] test-path: no; allocates: no
  - `console.log("Recorded " + result.rows.length + " actual " + command + " pixel samples");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/🔭️reference-visual-oracle/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/🔭️reference-visual-oracle/🟦️.ts:405` [console.log] test-path: no; allocates: no
  - `console.log("Recorded " + result.rows.length + " actual " + (webgpu ? "WebGPU" : "Three") + " reference visual pixel samples");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/🕸️grid-visual-oracle/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/🕸️grid-visual-oracle/🟦️.ts:254` [console.log] test-path: no; allocates: no
  - `console.log("Recorded " + result.rows.length + " actual " + (webgpu ? "production WGSL" : "installed Drei") + " grid pixel samples");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/📇️directory-and-hub-contracts/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🔮️oracles/📇️directory-and-hub-contracts/🟦️.ts:163` [console.log] test-path: no; allocates: no
  - `assert(!shell.includes("console.log(inviteToken)") && !shell.includes("setInviteToken"));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔀️surface-switch/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔀️surface-switch/🟦️.ts:349` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(rows));';`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔮️directory-and-hub-oracles/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔮️directory-and-hub-oracles/🟦️.ts:12` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('${name}: checks=${checks} clean');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔮️surface-retention-oracle/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔮️surface-retention-oracle/🟦️.ts:9` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('surface-host-retention-oracle: checks=${checks} clean');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔮️world3d-gesture-oracles/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔮️world3d-gesture-oracles/🟦️.ts:12` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('${name}: checks=${checks} clean');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts:43` [console.log] test-path: yes; allocates: yes: .map(, .repeat(, JSON.stringify
  - `const consumer = 'const api = require(process.argv[1]); const fixture = JSON.parse(require("node:fs").readFileSync(0,"utf8")); const input = row => ({ defaultVariant: fixture.defaultVariant, search: row.search, hash: ...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const source = 'const input = JSON.parse(require("node:fs").readFileSync(0, "utf8")); console.log(JSON.stringify(await (async () => { ${body} })()));';`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🎥️scene-camera-and-drag-drop/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🎥️scene-camera-and-drag-drop/🦀️.rs:303` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] catalogue workflow drop surface={surface_id} controller={controller_id} program={plugin_id} app={app_id} world=({:.1},{:.1})", world.0, world.1);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/✍️text-editor-delivery-and-diagnostics/🦀️.rs` (4)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/✍️text-editor-delivery-and-diagnostics/🦀️.rs:249` [eprintln!] test-path: no; allocates: yes: println!
  - `/// host could not read. 'eprintln!' is a no-op inside a 'wasm32-unknown-unknown' Worker.`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/✍️text-editor-delivery-and-diagnostics/🦀️.rs:254` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{line}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/✍️text-editor-delivery-and-diagnostics/🦀️.rs:257` [eprintln!] test-path: no; allocates: yes: println!
  - `/// 🩺️ Console line for a screen-path gesture. 'eprintln!' is a no-op inside a`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/✍️text-editor-delivery-and-diagnostics/🦀️.rs:263` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{line}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🖱️engagement-pointer-oracle.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🖱️engagement-pointer-oracle.ts:27` [console.log] test-path: yes; allocates: no
  - `console.log("Verified engagement pointer fixture with Three.js");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🎨️scene-render-entry/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🎨️scene-render-entry/🦀️.rs:584` [eprintln!] test-path: no; allocates: yes: println!
  - `/// only way to tell "no surface" from "a surface with no meshes" on 6118, where 'eprintln!' is a`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🎨️scene-render-entry/🦀️.rs:648` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{line}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎥️VideoRenderHost/🧪️tests/🔬️unit/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎥️VideoRenderHost/🧪️tests/🔬️unit/🟦️.ts:205` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('video-render-host tiers=first-party,platform outcomes=done,refused(capability,program),failed,cancelled events=${events.length}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏳️async/🧩️session-and-topology/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⏳️async/🧩️session-and-topology/🦀️.rs:15` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{line}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🧩️session-and-topology/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🧩️session-and-topology/🦀️.rs:76` [eprintln!] test-path: no; allocates: yes: println!
  - `/// 📣️ 'eprintln!' is a no-op in a 'wasm32-unknown-unknown' Worker — a fault recorded there`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🧩️session-and-topology/🦀️.rs:79` [eprintln!] test-path: no; allocates: yes: println!
  - `/// 'eprintln!' except 'debug_log''s own native arm.`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs:18` [[DEBUG]] test-path: no; allocates: no
  - `/// 🪲 Formats '[DEBUG] shell node-graph fit' with the '{x,y,zoom}' object the port probe parses.`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs:20` [[DEBUG]] test-path: no; allocates: yes: format!
  - `format!("[DEBUG] shell node-graph fit {{\"x\":{x},\"y\":{y},\"zoom\":{zoom}}}")`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️native-keyboard-ring/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️native-keyboard-ring/🦀️.rs:89` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("native accessibility census: painted controls {painted}, chrome nodes {}, {census:?}", chrome.nodes.len());`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs` (4)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:620` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] ShellState {}", std::mem::size_of::<ShellState>());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:621` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] surfaces: w3d {} nodegraph {} tiled {} board {} icon_render {} retire {} retired_q {} status {} fault {}", g(|s| &s.surfaces.world3d_states), g(|s| &s.surfaces.node_graph_states), g(|s| &s.surfaces....`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:622` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] opening: doc_opening {} icon_export {} video {} native_media {} lease {} import {} services {}", g(|s| &s.opening.document_opening), g(|s| &s.opening.icon_export), g(|s| &s.opening.video_render), g(...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:623` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] docs: session {} clip {} ident {} closing {} overlay {} spawned {}", g(|s| &s.docs.session), g(|s| &s.docs.workspace_clipboard), g(|s| &s.docs.app_document_identities), g(|s| &s.docs.closing_documen...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs:215` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] {name} event {event} -> {commands:?}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🪟️window-scope-laws/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🪟️window-scope-laws/🦀️.rs:220` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("window-scope chords: {owners} owner hops, {unowned} hinted no-operations, {} hint locales, code {KEYBINDING_UNOWNED_CODE}", fixture["unownedHint"].as_array().expect("⌨️ fixture hint rows").len());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🪟️window-scope-laws/🦀️.rs:242` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧬️schema/📌️panel-state/🧪️tests/🔬️unit/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧬️schema/📌️panel-state/🧪️tests/🔬️unit/🟦️.ts:121` [console.log] test-path: yes; allocates: no
  - `console.log("host-panel-state ajv=valid independent=valid json-patch=valid json-text-codec=valid no-pack-fallback=valid strict-capacity=valid spawned-preservation=valid");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/📄️paged-payload/🟦️.tsx` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/📄️paged-payload/🟦️.tsx:104` [console.log] test-path: yes; allocates: no
  - `expect(executed).toBe(plan.length + 1); console.log("[TRACE] Authored cancellation prefixes", executed);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🔢️scalar-source-oracles/🟦️.tsx` (4)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🔢️scalar-source-oracles/🟦️.tsx:124` [console.log] test-path: yes; allocates: yes: .map(, Array.from, JSON.stringify, new Set
  - `assert(scalarWordCheck(c.machine.initial.serial), JSON.stringify(scalarWordCheck.errors)); for(const r of c.records)assert.equal(r.bytes,16+8*r.fields.length);assert.equal(new Set(c.records.map(r=>r.name)).size,5);con...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🔢️scalar-source-oracles/🟦️.tsx:135` [console.log] test-path: yes; allocates: no
  - `else throw Error("Unknown effect "+e.op);};const plain: ReceiptChange=(s,fn)=>{const d={...s};fn(d);return d;};const apply=(s: ReceiptState,r: ReceiptRow,change: ReceiptChange): ReceiptResult=>{const t=c.machine.trans...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🔢️scalar-source-oracles/🟦️.tsx:138` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `let suffixes=0;for(const name of (["reader","readerAlias","page"] as const)){const x=c.closePlan[name];const bytes=childSources[x.path];assert(bytes,name+" original source");assert.equal(createHash("sha256").update(by...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🔢️scalar-source-oracles/🟦️.tsx:191` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({exactAliasPrefixes:14,exactAliasRows:exactRows,aliasNegativeRows,aliasFaultRows,aliasShortRows,aliasReplayRows,omittedUnlinkRefusals:7,constructorChargeUnchanged:992,originalParentCharge:88...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts:133` [[DEBUG]] test-path: no; allocates: no
  - `console.error("[DEBUG] job completion turn", String(job), outcome.status, wireTurnStatusTag(settled.status), settled.uiPatches.length, settled.effects.length);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts:201` [[DEBUG]] test-path: no; allocates: no
  - `console.error("[DEBUG] isolated job terminal", String(job), kind, outcome.status, step, live(), outcome.status === "failed" ? new TextDecoder().decode(outcome.value) : "");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts:214` [[DEBUG]] test-path: no; allocates: no
  - `console.error("[DEBUG] isolated job error", String(job), String(error));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧺️handle-ui-intake/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧺️handle-ui-intake/🟦️.ts:108` [[DEBUG]] test-path: no; allocates: no
  - `const installedNow=reads.install(instanceId,surfaceId,surface,intake);console.error("[DEBUG] ui install",instanceId,surfaceId,installedNow);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧺️handle-ui-intake/🟦️.ts:245` [[DEBUG]] test-path: no; allocates: no
  - `const surfaceId=retainedSurfaceId(instanceId,target.key),surface=surfaces.get(surfaceId),view=surface?.view;console.error("[DEBUG] refresh target",surfaceId,target.hash,view?.hash,view?.revision);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧪️tests/🧭️ownership/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧪️tests/🧭️ownership/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `const source = 'import {mock} from "bun:test";const calls=[];const execution=await import(${JSON.stringify(execution)}),fs=await import("node:fs/promises"),access=fs.access;mock.module("node:fs/promises",()=>({...fs,a...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/📄️retained-document-consumer/🦀️.rs` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/📄️retained-document-consumer/🦀️.rs:131` [eprintln!] test-path: no; allocates: yes: println!
  - `/// 🩺️ Console line for a retained-document terminal fault. 'eprintln!' is a no-op inside a`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/📄️retained-document-consumer/🦀️.rs:137` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{line}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/📄️retained-document-consumer/🦀️.rs:160` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] scene retirement forward window={} host={} kind={:?} retained={_retained}", owner.window_id, owner.host_id, owner.kind);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/⏲️tasks/🔗️extension-continuation/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/⏲️tasks/🔗️extension-continuation/🦀️.rs:54` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] extension response dropped the oversized request field {key:?} from the echoed correlation");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs:175` [eprintln!] test-path: no; allocates: yes: .map(, println!
  - `eprintln!("[TRACE] maintenance stage={} elapsed_us={} outcome={:?}", LAST_MAINTENANCE_STAGE.load(Ordering::Relaxed), finished_us - started_us, maintenance.as_ref().map(|_| ()).map_err(|failure| &failure.primary));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📥️staging/🔌️reactor-app/🛠️runtime-helpers/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📥️staging/🔌️reactor-app/🛠️runtime-helpers/🦀️.rs:32` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{line}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs:33` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[DEBUG] live cleanup zero progress {stalled} {progress:?}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs:48` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] cooperative maintenance callback overran the interactive ceiling for instance {} (elapsed {elapsed_us}us) — recorded, not fatal: the callback clock measures wall time including descheduling and the ...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🪪️descriptor/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🪪️descriptor/🦀️.rs:32` [eprintln!] test-path: no; allocates: yes: println!
  - `std::panic::set_hook(Box::new(|panic| eprintln!("[semio-plugin panic] {panic}")));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs:206` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!
  - `if chained > 0 && chained % 128 == 0 { eprintln!("[DEBUG] media-in chained {chained} moves in one turn"); }`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs:74` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[TRACE] typed-operation slots instance={instance} live={live}/{} peak={peak}", semio_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🔦️more-work-sources-and-tracing/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🔦️more-work-sources-and-tracing/🦀️.rs:151` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{line}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🔦️more-work-sources-and-tracing/🦀️.rs:170` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️reconcile-spin/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️reconcile-spin/🦀️.rs:335` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("reconcile-spin cancellation quiesced in {turns} turns");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🚚️more-work-drive/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🚚️more-work-drive/🦀️.rs:446` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("more-work-drive readiness ladder turns={turns} published={published} overlapped={overlapped} surfaces={}", revisions.len());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🚚️more-work-drive/🦀️.rs:465` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🏋️real-size-surfaces-and-producer-shell-denials/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🏋️real-size-surfaces-and-producer-shell-denials/🦀️.rs:91` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🔔️obsolete-feedback-acks-and-reservation-cap/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🔔️obsolete-feedback-acks-and-reservation-cap/🦀️.rs:61` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("obsolete real reconcile feedback rejection={} old={} current={} exact old owner retired", rejection, first, second);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🔔️obsolete-feedback-acks-and-reservation-cap/🦀️.rs:228` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("mounted-resident-capacity fixed={fixed_bytes} floor={floor} ceiling={} ceiling-sized-accepted={capacity} full={} cap-plus-one=false exact-refusal=true restored={fixed_bytes}", limits.max_bytes, full.bytes);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs:518` [eprintln!] test-path: no; allocates: yes: println!
  - `/// target) fall back to 'eprintln!', mirroring 'host_port::host_now_ms''s own`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs:526` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[{level}] {message}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:55` [console.log] test-path: no; allocates: no
  - `console.log("Browser support: canonical selected-mode vendor shims and shard worker published");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/📜️script.ts:34` [console.log] test-path: no; allocates: no
  - `console.log(destination);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/🦀️.rs:181` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-native-transpile: {error:#}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧪️tests/🪞️two-checkout/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧪️tests/🪞️two-checkout/🟦️.ts:119` [console.log] test-path: yes; allocates: no
  - `if(!binaryen)console.log("[two-checkout] no Binaryen found (SEMIO_WASM_OPT_BIN or node_modules/binaryen): the optimize step was not run");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/📨️message-handler/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/📨️message-handler/🟦️.ts:243` [console.log] test-path: no; allocates: yes: ${
  - `if (retryableLifecycle) console.log(\'shard worker: retryable lifecycle deadline on \${faultPhase} for actor \${actorId}; receipt retained, replaying next tick\');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🩹️preview2-shim-patches/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🩹️preview2-shim-patches/🟦️.ts:89` [eprintln!] test-path: no; allocates: yes: println!
  - `* 🐛️ Rust's stderr is unbuffered, so 'core::fmt::write' turns one 'eprintln!("[TRACE] … {a} … {b}")'`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts:46` [console.log] test-path: no; allocates: yes: ${
  - `else console.log(\'[actor] \${message}\');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/⚡️vite/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/⚡️vite/🟦️.ts:14` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Staged ${count} browser runtime files from ${sources.length} completed artifacts');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/🟦️.ts:209` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ observations, component: fixture.component, canonicalInterface: fixture.canonicalInterface, imports: fixture.importInterfaces, mode: fixture.mode, jco: 1, wasm: 1 }));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/🟦️.ts:433` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ observations, pending: pending.progress(), aborted: aborted.progress(), laws: 15, hostCloseLaws }));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/🟦️.ts:449` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('browser-actor-import: AJV=1 JCO=1 Wasm=1 JSPI=${fixture.identity.jco.asyncImports.length} emitted=${fixture.identity.emittedAsyncBindings.length} actors=${fixture.actors.length} pack-ok=2 pack-err=2 strea...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts:26` [console.log] test-path: yes; allocates: no
  - `const original = { log: console.log, debug: console.debug, error: console.error };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts:208` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ writes, ready: [...ready], preview2: 1 }));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts:230` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('browser-wasi-activation: AJV=1 TypeScript=1 Preview2=1 actors=2 laws=${fixture.laws.length} resources=${fixture.limits.resources} waiters=${fixture.limits.waiters} evidence=${evidence}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts:37` [console.log] test-path: no; allocates: no
  - `if (bytes.byteLength) (channel === "stdout" ? console.log : console.warn)("[browser actor " + channel + "] " + new TextDecoder().decode(bytes));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🏃️execution/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🏃️execution/🟦️.ts:78` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Plugin catalog: ${targets.length} selected component(s); report ${outcome.report}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/👁️watch/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/👁️watch/🟦️.ts:80` [console.log] test-path: no; allocates: no
  - `console.log("watching plugin crates for hot-swap rebuilds");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/👁️watch/🧪️tests/🔬️source-plan/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/👁️watch/🧪️tests/🔬️source-plan/🟦️.ts:74` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Native watch source plan: ${fixture.required.length} captured owner/dependency files and ${fixture.events.length} independent minimatch vectors passed');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📦️materialization/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📦️materialization/🟦️.ts:9` [console.log] test-path: no; allocates: yes: ${, new Set
  - `const outcome=await buildComponentClosureV1(workspace,new Set([pluginId]),{facet:"browser",invocation,control,network});console.log('Iterated ${pluginId}; report ${outcome.report}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts:18` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[component] ${metadata}: built, described and materialized; report ${outcome.report}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts:116` [console.log] test-path: no; allocates: no
  - `const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,say=(line:string)=>(options.log??console.log)(line);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts:117` [console.log] test-path: no; allocates: yes: .map(, Date.now
  - `return retryAfterOutOfSpaceV1({store:store.root,language,report:say},()=>withUnitPressureRecheckV1(()=>relieveUnitStorageV1(store.root,{environment:store.environment,language,report:say}),()=>withRepositoryCargoPassIn...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts:179` [console.log] test-path: no; allocates: yes: ${
  - `console.log('components describe: ${outcome.staged.length} component(s) described from ${profile} bytes; report ${outcome.report}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🎛️commands/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🎛️commands/🟦️.ts:52` [console.log] test-path: no; allocates: yes: ${
  - `progress(completed, total, path) { console.log('catalog-complete source ${completed}/${total}: ${path}'); },`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🎛️commands/🟦️.ts:68` [console.log] test-path: no; allocates: yes: ${
  - `progress(event) { console.log('catalog-complete artifact ${event.completed}/${event.total}: ${event.pluginId} ${event.status}'); },`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🎛️commands/🟦️.ts:74` [console.log] test-path: no; allocates: yes: ${
  - `console.log('plugin catalog is complete: ${audit.manifestCount} source identities and fresh raw/descriptor/receipt artifacts verified from ${verifier.root}.');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🎯️native-selection/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🎯️native-selection/🟦️.ts:189` [console.log] test-path: no; allocates: yes: ${
  - `console.log('native-catalog-selection-oracle cases=${fixture.cases.length} positive=${positive} denied=${fixture.cases.length - positive} authority=planning-only published=0');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold/🟦️.ts:146` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${label}: ${verb} ${result.created.length} file(s), ${result.skipped.length} already present');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold/🟦️.ts:147` [console.log] test-path: no; allocates: yes: ${
  - `for (const path of result.created) console.log('  ${dryRun ? "+ (dry-run)" : "+"} ${path}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold/🟦️.ts:169` [console.log] test-path: no; allocates: yes: ${
  - `console.log('new surface --all: ${verb} ${surfacesTouched}/${totalSurfaces} surface(s) across ${subsetRels.length} owned subset(s) (${filesCreated} file(s) ${dryRun ? "would be created" : "created"}, ${filesSkipped} a...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts:29` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Playground session ${variant}: ${session.plugins.length} plugins staged');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:22` [console.log] test-path: no; allocates: yes: ${
  - `console.log('playground-asset-contract: cases=${proveTileProxyAssetCorpusV1()} passed');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:35` [console.log] test-path: no; allocates: yes: ${
  - `console.log('playground-default-contract: ${await provePlaygroundDefaultContractV1()} vectors passed');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📥️runtime/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📥️runtime/🧪️tests/🟦️.ts:18` [console.log] test-path: yes; allocates: no
  - `const timer=setTimeout(()=>child.kill("SIGTERM"),120000);try{const [stdout,stderr,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);expect(status,stderr).toB...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🧪️tests/📇️inventory/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🧪️tests/📇️inventory/🟦️.ts:57` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const assertion = '\nif(JSON.stringify(COMPONENT_MODULE_DIRECTORIES)!==${JSON.stringify(JSON.stringify(row.input.modules))})throw Error("Inventory drift");\nlet actual=null;try{actual=pluginModuleUrl(${JSON.stringify(...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts:382` [console.log] test-path: no; allocates: yes: ${
  - `console.log('plugin registry catalog refreshed (${entries.length} plugin crates, ${playgrounds.length} playgrounds, ${frameworkPackages.length} framework packages) -> ${outDir}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts:460` [console.log] test-path: no; allocates: no
  - `console.log("plugin registry generated catalog bytes are fresh.");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts:523` [console.log] test-path: no; allocates: yes: ${
  - `console.log('plugin registry catalog is fresh (${entries.length} plugin crates, ${playgrounds.length} playgrounds, ${frameworkPackages.length} framework packages).');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts` (7)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts:73` [console.log] test-path: no; allocates: yes: ${, .join(
  - `console.log('guest-framework-check ${check.target} (${check.workspace}): ${check.packages.join(", ")}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts:75` [console.log] test-path: no; allocates: yes: ${, Date.now
  - `console.log('guest-framework-check ${check.target} done in ${Math.round((Date.now() - started) / 1000)} s');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts:113` [console.log] test-path: no; allocates: yes: ${, Date.now
  - `console.log('rebuild-all holds ${WASM_BUILD_LEASE} after ${Math.round((Date.now() - waited) / 1000)} s');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts:117` [console.log] test-path: no; allocates: yes: ${
  - `console.log('rebuild-all ${index + 1}/${chain.length} ${step.id} (${step.stage}) started');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts:119` [console.log] test-path: no; allocates: yes: ${, Date.now
  - `console.log('rebuild-all ${index + 1}/${chain.length} ${step.id} done in ${Math.round((Date.now() - started) / 1000)} s');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts:128` [console.log] test-path: no; allocates: yes: ${, .join(
  - `for (const step of admitted.steps) console.log('rebuild-plan ${step.id} (${step.stage}): ${step.command.join(" ")}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts:129` [console.log] test-path: no; allocates: yes: ${
  - `console.log('rebuild-plan steps=${admitted.steps.length} guestChecks=${admitted.guestFrameworkChecks.length}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🧪️tests/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `const program = 'import * as api from ${JSON.stringify(resolve(import.meta.dirname, "../🧬️schema/🟦️.ts"))}; const rows=${JSON.stringify(corpus.cases)}; const parsers={source:api.parseComponentSourceRowV1,target:api.pa...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🧪️tests/🟦️.ts:194` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const node=spawnSync("node",["--input-type=module"],{input:bundle.outputFiles[0]!.text+'\nconsole.log(JSON.stringify({modules:COMPONENT_MODULE_DIRECTORIES,plugins:PLUGIN_BUILD_TARGETS}))',encoding:"utf8"});`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts:716` [console.log] test-path: no; allocates: no
  - `console.log("registry-rust-mounts-oracle cases=" + fixture.cases.length + " ajv=1 compiler=" + fixture.cases.length);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts:759` [console.log] test-path: no; allocates: no
  - `console.log("registry-plugin-root-ownership cases=" + fixture.cases.length + " ajv=1 sqlite=" + fixture.cases.length + " capture=" + capture);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts:151` [console.log] test-path: no; allocates: yes: ${
  - `console.log('descriptor gate: ${described}/${entries.length} crates own a verified 🔣️.json + 🛂️.descriptor.semio pair.');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts:653` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[surface-schema] ${done}/${total} ${label}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts:659` [console.log] test-path: no; allocates: yes: ${
  - `for (const outcome of counted("blocked")) console.log('  blocked ${outcome.owner.pluginId} ${outcome.owner.ownerLabel} ${outcome.owner.lane}: ${(outcome as { reason: string }).reason}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts:661` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[surface-schema] ${outcomes.length} lane(s): ${counted("written").length} written, ${counted("current").length} current, ${counted("authored").length} authored, ${counted("blocked").length} blocked, ${cou...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📊️size/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📊️size/🟦️.ts:33` [console.log] test-path: no; allocates: yes: ${, .join(, new Map
  - `rows.sort((a,b)=>b.totalBytes-a.totalBytes);const content="["+encoded.join(",")+"]";await artifacts.recheck();await withArtifactScratchV1({workspace:this.repoRoot,invocation:this.invocation,storage:store.storage,contr...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/⚖️comparison/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/⚖️comparison/🟦️.ts:11` [console.log] test-path: no; allocates: yes: ${
  - `for(const target of selected){const intent={version:1 as const,pluginId:target.pluginId,cratePath:target.cratePath};await withComponentModelV1(repoRoot,invocation,{...intent,mode:"prototype"},control,prototype=>withCo...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts` (40)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:80` [console.log] test-path: no; allocates: yes: ${
  - `console.log('artifact-admission-oracle cases=${artifactAdmissionOracle(this.repoRoot)} firstParty=39');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:81` [console.log] test-path: no; allocates: yes: ${
  - `console.log('completion-rejection-oracle assertions=${completionRejectionOracle(this.repoRoot)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:82` [console.log] test-path: no; allocates: yes: ${
  - `console.log('declared-verb-verdict-oracle cases=${declaredVerbVerdictOracle()}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:83` [console.log] test-path: no; allocates: yes: ${
  - `console.log('declared-bridge-argument-oracle assertions=${declaredBridgeArgumentOracle()}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:84` [console.log] test-path: no; allocates: yes: ${
  - `console.log('media-owner-context-oracle assertions=${mediaOwnerContextOracle()}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:85` [console.log] test-path: no; allocates: yes: ${
  - `console.log('agent-lane-preview-verdict-oracle cases=${agentLanePreviewVerdictOracle()}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:86` [console.log] test-path: no; allocates: yes: ${
  - `console.log('agent-lane-carriage-oracle cases=${agentLaneCarriageOracle()}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:87` [console.log] test-path: no; allocates: yes: ${
  - `console.log('time-travel-scenario-oracle cases=${timeTravelScenarioOracle(this.repoRoot)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:88` [console.log] test-path: no; allocates: yes: ${
  - `console.log('supersede-ledger-oracle cases=${supersedeLedgerOracle(this.repoRoot)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:89` [console.log] test-path: no; allocates: yes: ${
  - `console.log('history-alternatives-oracle cases=${historyAlternativesOracle(this.repoRoot)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:90` [console.log] test-path: no; allocates: yes: ${
  - `console.log('history-label-reload-oracle cases=${historyLabelReloadOracle(this.repoRoot)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:91` [console.log] test-path: no; allocates: yes: ${
  - `console.log('folder-reload-route-oracle steps=${folderReloadRouteOracle(this.repoRoot)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:92` [console.log] test-path: no; allocates: yes: ${
  - `console.log('composed-child-history-oracle cases=${composedChildHistoryOracle(this.repoRoot)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:93` [console.log] test-path: no; allocates: yes: ${
  - `console.log('checkpoint-actor-oracle cases=${checkpointActorOracle(this.repoRoot)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:99` [console.log] test-path: no; allocates: yes: ${
  - `console.log('retained-child-close ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:101` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('retained-child-close-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:206` [console.log] test-path: no; allocates: yes: ${
  - `console.log('schema-document-authority oracle=${schemaDocumentAuthorityOracle()}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:208` [console.log] test-path: no; allocates: yes: ${
  - `const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.r...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:209` [console.log] test-path: no; allocates: yes: ${
  - `console.log('schema-document-authority native-laws=${receipts.length}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:220` [console.log] test-path: no; allocates: yes: ${
  - `console.log('artifact-admission-oracle cases=${artifactAdmissionOracle(this.repoRoot)} firstParty=39');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:221` [console.log] test-path: no; allocates: yes: ${
  - `console.log('completion-rejection-oracle assertions=${completionRejectionOracle(this.repoRoot)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:230` [console.log] test-path: no; allocates: yes: ${
  - `console.log('artifact-admission ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:232` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('artifact-admission-laws: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:244` [console.log] test-path: no; allocates: yes: ${
  - `console.log('guest-lifecycle-oracle cases=${guestLifecycleOracle()} patch-cases=${issuedPatchOracle()}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:254` [console.log] test-path: no; allocates: yes: ${
  - `console.log('guest-lifecycle ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:256` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('guest-lifecycle-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:266` [console.log] test-path: no; allocates: yes: ${
  - `console.log('cold-document-pair-ingress-oracle: ajv=1 sha256=3 webcrypto=3 hostile=${hostile} limits=64KiB/64/4MiB');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:280` [console.log] test-path: no; allocates: yes: ${
  - `], progress(event) { console.log('cold-document-pair-ingress ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}'); } });`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:281` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('cold-document-pair-ingress-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:291` [console.log] test-path: no; allocates: yes: ${
  - `console.log('document-backbone-binding-oracle: ajv=1 rows=${rows} hot=256KiB genesis=4MiB pending=64/1MiB');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:317` [console.log] test-path: no; allocates: yes: ${
  - `], progress(event) { console.log('document-backbone-binding ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}'); } });`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:318` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `console.log('document-backbone-binding-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:330` [console.log] test-path: no; allocates: yes: ${
  - `console.log('declared-bridge-argument-oracle assertions=${declaredBridgeArgumentOracle()}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:331` [console.log] test-path: no; allocates: yes: ${
  - `console.log('media-owner-context-oracle assertions=${mediaOwnerContextOracle()}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:332` [console.log] test-path: no; allocates: yes: ${
  - `console.log('extension-retirement-oracle cases=${extensionRetirementOracle(this.repoRoot)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:333` [console.log] test-path: no; allocates: yes: ${
  - `console.log('source-freshness-oracle cases=${await sourceFreshnessOracle(this.repoRoot,nextestArtifactLocation(this.repoRoot).directory)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:343` [console.log] test-path: no; allocates: yes: ${
  - `}],progress(event) { console.log('canonical-architecture-command ${event.stage}: ${event.law ?? ""}'); }});`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:344` [console.log] test-path: no; allocates: yes: ${
  - `console.log('canonical-architecture-command receipts=${receipts.length}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:417` [console.log] test-path: no; allocates: yes: ${
  - `const receipts=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:418` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] retained-structural-refusal receipts=${receipts.length}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧭️topology/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧭️topology/🦀️.rs:448` [eprintln!] test-path: no; allocates: yes: println!
  - `/// 'debug_runtime_line' records in 4 000 console lines, against thousands of plain 'eprintln!'`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧭️topology/🦀️.rs:455` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("interaction selection lost reason={reason} dispatched={dispatched} validated={validated} readback={readback} domains={domains}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🏠️wasmtime-host-state-and-wit-hosts/🦀️.rs` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🏠️wasmtime-host-state-and-wit-hosts/🦀️.rs:65` [eprintln!] test-path: no; allocates: yes: println!
  - `/// 'eprintln!' has never been readable the way the browser console reads it.`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🏠️wasmtime-host-state-and-wit-hosts/🦀️.rs:135` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[actor:{}:{level}] {message}", self.plugin_id);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🏠️wasmtime-host-state-and-wit-hosts/🦀️.rs:145` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[actor:{}:trace] {name}", self.plugin_id);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs:584` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[actor-async:{}:{level}] {message}", self.actor);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs:594` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[actor-async:{}:trace] {name}", self.actor);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs:610` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("[actor-async:{}] emit: {error}", self.actor),`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts:79` [console.log] test-path: no; allocates: yes: ${
  - `await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: ...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts:102` [console.log] test-path: no; allocates: yes: ${
  - `}], progress:event=>console.log('count-component ${event.stage} ${event.law??""}') });`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧪️tests/🔬️rust-package-commands/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧪️tests/🔬️rust-package-commands/🟦️.ts:15` [console.log] test-path: yes; allocates: no
  - `console.log("neutral-host-fixture: independent Ajv valid=3 hostile=3");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/♻️relay-lifecycle/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/♻️relay-lifecycle/🟦️.ts:104` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('relay-lifecycle oracle: ${fixture.traces.length + fixture.ingressCases.length}/${fixture.traces.length + fixture.ingressCases.length}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts` (11)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:161` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('plugin-host-service-operation-source: effects=${fixture.effects.length} wit=1 sync=1 async=1 mutation=1 passed');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:181` [console.log] test-path: yes; allocates: yes: ${
  - `], progress: (event) => console.log('service-operation ${event.stage} ${event.package} ${event.law ?? ""}') });`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:182` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('plugin-host-service-operation-native: groups=${laws.length} laws=${laws.reduce((count, group) => count + group.laws.length, 0)} passed');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:236` [console.log] test-path: yes; allocates: yes: ${, .join(, .map(
  - `console.log('plugin-host-lifecycle-laws: ${laws.map(([pkg, law]) => '${pkg}:${law}').join(" ")}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:248` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('guest-fault-oracle cases=${guestFaultOracle()} retries=${retainedLifecycleOracle()} activations=${activationOwnershipOracle()} reservations=${kernelReservationOracle()}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:309` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('guest-fault ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:311` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('guest-fault-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:354` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('plugin-host-ui-patch-marshalling-source: ajv=1 cases=${fixture.cases.length} component=${fixture.componentCases.length} ops=${fixture.operationKinds.length} originalAsyncRuntime=1 passed');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:380` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('ui-patch-marshalling ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:382` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('ui-patch-marshalling-receipts: ${JSON.stringify(receipts)}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️rust-package-commands/🟦️.ts:465` [console.log] test-path: yes; allocates: yes: ${
  - `await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0), invocation: context.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 86_40...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs` (10)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs:59` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-shard] usage: semio-shard <component.wasm> <package-id> <actor-id> <identity-policy.json>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs:63` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-shard] actor-id must be a u64: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs:70` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-shard] read {wasm_path}: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs:76` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-shard] compile failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs:80` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-shard] instantiate failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs:85` [eprintln!] test-path: no; allocates: yes: println!, to_string(
  - `let policy_text=std::fs::read_to_string(identity_policy_arg).unwrap_or_else(|error|{eprintln!("[semio-shard] identity policy read failed: {error}");std::process::exit(2)});`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs:86` [eprintln!] test-path: no; allocates: yes: println!, serde
  - `let policy=serde_json::from_str(&policy_text).unwrap_or_else(|error|{eprintln!("[semio-shard] identity policy rejected: {error}");std::process::exit(2)});`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs:92` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-shard] registration refused: {:?}", rejected.reason);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs:95` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-shard] pid={} package={package_id} actor={actor_id} ready", std::process::id());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs:100` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio-shard] pump error: {error}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🧪️tests/🔬️unit/🦀️.rs:247` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[skip] SEMIO_SCALE_FIXTURE_WASM not set — see this test's doc comment for the build command");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts:67` [console.log] test-path: no; allocates: no
  - `if (retained) console.log("fresh-component-process: stage=" + stage + " evidence=" + evidence);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:23` [console.log] test-path: no; allocates: yes: ${
  - `console.log('canonical-descriptor-pack-oracle cases=${canonicalDescriptorPackOracle(this.repoRoot)}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs` (22)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:57` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[describe:{level}] {message}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:67` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[describe:trace] {name}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:147` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[describe] ignoring host-async emit(): describe() must be pure");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:151` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[describe] ignoring host-async emit-patch(): describe() must be pure");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:435` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[describe] owned phase=compile bytes={} elapsed_ms=0", wasm_bytes.len());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:438` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[describe] owned phase=execute fuel=0 elapsed_ms={}", started.elapsed().as_millis());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:443` [eprintln!] test-path: no; allocates: yes: println!
  - `|fuel, elapsed| eprintln!("[describe] owned phase=execute fuel={fuel} elapsed_ms={}", elapsed.as_millis()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:522` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[describe] owned phase=codec schema={schema:?} fuel=0 elapsed_ms=0");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:524` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[describe] owned phase=codec schema={schema:?} fuel={fuel} elapsed_ms={}", elapsed.as_millis());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:591` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[describe] owned phase=prepare fuel={fuel} elapsed_ms={}", began.elapsed().as_millis());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:688` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: semio-plugin-describe capabilities");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:694` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-plugin-describe: unknown command {other:?} (expected \"capabilities\", \"describe\", or \"codecs\")");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:698` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: semio-plugin-describe capabilities | describe <component.wasm> --out <dir> | codecs <component.wasm> --kinds <kind=schema,…> --out <file.json>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:726` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-plugin-describe codecs: unexpected argument {other:?}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:732` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: semio-plugin-describe codecs <component.wasm> --kinds <kind=schema,…> --out <file.json>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:736` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-plugin-describe codecs: --kinds must name at least one <kind>=<schema> pair");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:751` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-plugin-describe codecs: creating {}: {error}", parent.display());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:761` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-plugin-describe codecs: writing {}: {error}", out_path.display());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:767` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-plugin-describe codecs: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:784` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-plugin-describe describe: unexpected argument {other:?}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:790` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: semio-plugin-describe describe <component.wasm> --out <dir>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs:799` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("semio-plugin-describe describe: {error}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts:57` [console.log] test-path: no; allocates: no
  - `export function ensureFreshDescriptorEmitterV1(workspace:string,invocation:ScriptInvocation,control:CompilerAdmissionControl,report:(line:string)=>void=console.log):Promise<"fresh"|"built">{`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts` (5)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts:150` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('fresh-source-epoch: AJV=1 stable-stringify=1 WebCrypto=1 physical-laws=${fixture.cases.length} bounded-capture=2 unsafe-paths=7 dep-info=${depInfo.cases.length}+3 evidence=${evidence}; Cargo resolver/dep-...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts:361` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('fresh-component-staging: WebCrypto=1 Pack=1 BLAKE3=1 laws=${fixture.laws.length} evidence=${evidence}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts:488` [console.log] test-path: yes; allocates: no
  - `console.log("fresh-component-preparation-control: actual-controlled-cancellation=1 actual-controlled-deadline=1 live-progress=2 retired-child-checks=2 zero-owned-queue-successors=2 evidence="+ownerRoot);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts:500` [console.log] test-path: yes; allocates: no
  - `console.log("fresh-component-package-ownership: actual-Cargo=2 missing-lock-refusal=1 fresh-lock-pairs=2 current-source-SHA=1 metadata-oracle=1 unknown-refusal=1 cross-workspace-refusal=1 evidence="+ownerRoot);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts:531` [console.log] test-path: yes; allocates: no
  - `console.log("fresh-component-process: fast-deep-equal=3 runtime-laws=" + (fixture.cases.length + fixture.queuedCargoCases.length) + " evidence=" + evidence);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🔬️unit/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🔬️unit/🦀️.rs:105` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("owned describe matches Wasmtime for {} ({} descriptor bytes)", path.display(), owned.len());`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🧾️store-replacement-vocabulary/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🗃️document/🧾️store-replacement-vocabulary/🦀️.rs:26` [eprintln!] test-path: no; allocates: yes: println!
  - `/// 'faulted: bool' whose only explanation was a '#[cfg(test)]' 'eprintln!'. A headless host`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🦀️.rs:71` [eprintln!] test-path: no; allocates: yes: .map(, collect, lock(, println!
  - `eprintln!("[TRACE] exact native close fault={fault:?} origin={} phases={:?} detail={:?}", state.last_fault_origin.load(Ordering::SeqCst), state.callback_phase_us.iter().map(|phase| phase.load(Ordering::SeqCst)).collec...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs:72` [[DEBUG],eprintln!] test-path: no; allocates: yes: println!
  - `if cx.original_work_units_spent().saturating_sub(lifetime.work_start)>u128::from(HostLifetimeStock::FUEL){eprintln!("[DEBUG] lifetime spent {}",cx.original_work_units_spent().saturating_sub(lifetime.work_start));}`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs:741` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("window transient retired a rejected authority over {turns} Ok turns, refused a stale re-begin, and admitted the refreshed one at generation {}", app.stores.window_transient_store.capture(Some(&view)).unwrap...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts:109` [console.log] test-path: yes; allocates: yes: ${
  - `if (import.meta.main) console.log('declared-verb-verdict-oracle cases=${declaredVerbVerdictOracle()}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🏭️component-factory-test/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🏭️component-factory-test/🟦️.ts:79` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ actual, oracle, cancelledBeforeCompile, cancelledDuringCompile }));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🏭️component-factory-test/🟦️.ts:102` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('browser-component-factory: AJV=1 JCO=1 native-Wasm-oracle=1 actors=2 hostile=${hostile.length} cancellation=3 bytes=${factory.length} evidence=${evidence}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🛡️actor-bundle-tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🛡️actor-bundle-tests/🟦️.ts:231` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ actual, oracle, afterCloseB, snapshots, aborted: aborted.progress(), queuedGuestCalls, exports: Object.keys(module) }));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🛡️actor-bundle-tests/🟦️.ts:236` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('browser-actor-factory: AJV=1 JCO=1 native-Wasm-oracle=1 SHA256=node+webcrypto actors=2 laws=${fixture.laws.length} artifact-laws=${fixture.artifactLaws.length} bytes=${artifactBytes.byteLength} evidence=$...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🧬️browser-codegen-tests/🟦️.ts` (5)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🧬️browser-codegen-tests/🟦️.ts:49` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('browser-compiler-capsule: AJV=1 native-Wasm-oracle=1 valid=1 denied=${hostile.length} evidence=${evidence}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🧬️browser-codegen-tests/🟦️.ts:97` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('browser-compiler-sources: AJV=1 Node-oracle=1 WebCrypto=1 laws=${fixture.laws.length} inputs=${baseline.inputs.length} evidence=${evidence}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🧬️browser-codegen-tests/🟦️.ts:149` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('browser-codegen-policy: AJV=1 stable-stringify=1 WebCrypto=1 laws=${fixture.laws.length} inputs=${targets.length + input.packages.length * 2}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🧬️browser-codegen-tests/🟦️.ts:158` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('async-task-return-lift: TypeScript=1 admitted=${fixture.admitted.length} bindings=${fixture.bindings} denied=${fixture.denied.length}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🧬️browser-codegen-tests/🟦️.ts:173` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('browser-actor-codegen-manifest: AJV=1 canonical-arrays=1 valid=1 denied=${fixture.denied.length}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🪢️host-activation-tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🪢️host-activation-tests/🟦️.ts:190` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('browser-host-activation: AJV=1 TypeScript=1 laws=${fixture.laws.length} actors=2 pending-bound=${fixture.limits.pendingEffects} close-streams=2');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📖️read-only-run/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📖️read-only-run/🟦️.ts:55` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('toolRun readOnlyRun tool=${expected.toolId} stateAfterJobCompletes=${slot!.state} parkedStartRejection=toolRun.busy');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔢️action-arg-carriers/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔢️action-arg-carriers/🟦️.ts:59` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('toolRun actionArgCarriers ${row.id} carrier=${row.carrier} read=${read ?? "-"}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔣️codec-caller-source/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔣️codec-caller-source/🟦️.ts:274` [console.log] test-path: yes; allocates: no
  - `console.log("codec caller source hostiles: 12 exact local omissions; await/global-trait/A/PA/Diff/impl/erased-slot guards; in-memory candidate only");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/📌️document-cancellation-and-latest-wins-keys/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/📌️document-cancellation-and-latest-wins-keys/🦀️.rs:167` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("real mounted Document publication {boundary}, delayed ACK={delayed_ack}: count/revision/root retained and close terminal");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/📌️document-cancellation-and-latest-wins-keys/🦀️.rs:263` [eprintln!] test-path: yes; allocates: yes: println!, serde
  - `eprintln!("exact latest-wins key {} matched independent serde scope equality and retired its 8192-byte identity", case["id"]);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📌️pins/📜️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📌️pins/📜️.ts:33` [eprintln!] test-path: yes; allocates: yes: println!
  - `const debugStatements=[...lawFiles].reduce((n,path)=>n+(read(resolve(root,path)).match(/\beprintln!|\[DEBUG\]/g)??[]).length,0);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📌️pins/📜️.ts:40` [console.log] test-path: yes; allocates: no
  - `if(mode==="write"){writeFileSync(pinsFile,next);console.log("wrote "+rel(pinsFile));}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📌️pins/📜️.ts:41` [console.log] test-path: yes; allocates: no
  - `else if(mode==="check"){if(!existsSync(pinsFile)||read(pinsFile)!==next){console.error("Child-group pins drifted: run 'repo run @semio-tech/framework-plugin:child-group-pins'");process.exit(1);}console.log("child-grou...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🏁️replacement-close-cancellation-and-nested-window/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🏁️replacement-close-cancellation-and-nested-window/🦀️.rs:201` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🔧️replacement-fixtures-and-recursive-documents/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🔧️replacement-fixtures-and-recursive-documents/🦀️.rs:71` [[DEBUG],eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[DEBUG] maint demand {demand:?} cxgrant {:?} fuel {} yield {}",cx.retained_grant(),cx.fuel_remaining(),cx.retirement_should_yield());`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-2/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-2/🦀️.rs:245` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("retained Child runtime published every host lane and retired after {turn} turns");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-6-x/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-6-x/🦀️.rs:342` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("settled reactor turn retention: settled={settled} after={after} per_turn={per_turn} B");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-7-x/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-7-x/🦀️.rs:107` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("reactor turn future size_of_val = {bytes} B");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-x/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-x/🦀️.rs:389` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("retained lifecycle exact-event deadline retry={} events={:?}", attempt + 1, events);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛑️cancel-job-reply/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛑️cancel-job-reply/🟦️.ts:60` [console.log] test-path: yes; allocates: yes: ${, .join(, .map(
  - `console.log('shard-worker.cancel-job-reply posts=${posted.map((message) => message.kind).join(",")}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛑️terminal-quiet/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛑️terminal-quiet/🟦️.ts:69` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('toolRun terminalQuiet turns=${expected.turns} rows=${expected.rows.length} terminalStates=${terminal.join(",")} admitted=${admitted.join(" ")}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🤖️agent-lane-preview/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🤖️agent-lane-preview/🟦️.ts:164` [console.log] test-path: yes; allocates: yes: ${
  - `if (import.meta.main) console.log('agent-lane-preview-verdict-oracle cases=${agentLanePreviewVerdictOracle()} agent-lane-carriage-oracle cases=${agentLaneCarriageOracle()}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🚀️worker-bootstrap/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🚀️worker-bootstrap/🟦️.ts:193` [console.log] test-path: yes; allocates: yes: ${, .map(
  - `expect(completed).toEqual(fixture.gateCases.map(row => row.id)); console.log('[TRACE] ShardWorkerBootstrap actual original-cell endings=${completed.length}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🚀️worker-bootstrap/🟦️.ts:217` [console.log] test-path: yes; allocates: yes: ${
  - `expect(observed).toBe(fixture.faults.stages.length * fixture.faults.values.length); console.log('[TRACE] ShardWorkerBootstrap real post-call first-fault vectors=${observed}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🪟️surface-carriers/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🪟️surface-carriers/🦀️.rs:51` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("section carrier packed {} bytes into {} text leaves at depth {depth} ({} nodes)", payload.len(), chunks.len(), count_projected_nodes(&projected));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🪟️surface-carriers/🦀️.rs:196` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("one {}-byte instances lane packed into {leaves} text leaves at depth {depth}", instances.len());`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs:66` [[DEBUG]] test-path: no; allocates: no
  - `/// 🐛️ [DEBUG] Names the owner 'granted_retirement_step' retires next, mirroring its order, for the temporary mutation-latency histogram.`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs` (9)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:92` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:119` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[TRACE] actual registered keyed dispatch {} preserved exact target supersession, rebased its worker, published count{expected}, and closed every retained owner", case["id"]);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:181` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[TRACE] actual registered keyed dispatch {} preserved foreign reservation, retired seven UTF-8 bytes exactly, and delivered rejection after vacancy", case["id"]);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:248` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[TRACE] actual typed ingress pre-admitted slot {VACANT} under 63 live foreign reservations and retained its funded input without minting an operation at true saturation");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:271` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[TRACE] actual {ACTIONS} storm actions each released their typed-operation slot within {HOST_CONTINUATIONS_PER_ACTION} host continuations, peak occupancy {highest}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:296` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[TRACE] actual settled, latest-wins-replaced and cancelled typed operations each released their exact slot within {CONTINUATIONS} host continuations");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:440` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[TRACE] actual one mutating typed operation kept its turn runnable through all {spent} continuations of the call that admitted it");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:462` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[TRACE] actual a status-only host call finished its typed operation in {spent} continuations, with {drained} drain continuations after it");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:496` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[TRACE] actual the admitting call handed back the {lane:?} lane ({body:?}), {} completion witness(es) and {} history patch(es) within {spent} continuations", census.completions, census.history_patches);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🔬️rust-package-commands/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🔬️rust-package-commands/🟦️.ts:70` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('framework-os-shell typescript mirror refreshed from 🧬️schema -> ${outPath}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🔬️rust-package-commands/🟦️.ts:101` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('framework-os-shell schema-check: ${declared.length} 🔣️.json \'$defs\' exports agree with the Rust registry, the rendered mirror and 🧬️schema/🟦️.ts.');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📜️script.ts:39` [console.log] test-path: no; allocates: no
  - `console.log("schema-document-index: fresh");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️rust-package-commands/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️rust-package-commands/🟦️.ts:65` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('mutation-authority: ${writeProjectionIfChangedV1(MUTATION_AUTHORITY_PATH, mutationAuthorityProjection(this.repoRoot))}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️rust-package-commands/🟦️.ts:82` [console.log] test-path: yes; allocates: no
  - `console.log("mutation-authority: fresh");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🔬️rust-package-commands/🟦️.ts:97` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('schema-document-index: ${writeProjectionIfChangedV1(SCHEMA_INDEX_PATH, mutationSchemaDocumentIndexProjection(this.repoRoot))}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔢️checked-integers/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔢️checked-integers/🦀️.rs:101` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("checked integer vectors={checked} pointerBits={}", usize::BITS);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-handcrafted-grammar-conformance/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-handcrafted-grammar-conformance/🦀️.rs:66` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dsl-fixture-sweep] m5 grammar auto-discovery: {} facet(s) found, {} checked, {} soft-skipped, {} stdio-exempt soft failure(s), {} hard failure(s)", facets.len(), checked, soft_skipped, soft_failures.len(),...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-handcrafted-protocol-conformance/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-handcrafted-protocol-conformance/🦀️.rs:13` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("soft-skip {label}: unwrap failed: {error}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-handcrafted-protocol-conformance/🦀️.rs:74` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-production-coverage/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-production-coverage/🦀️.rs:41` [eprintln!] test-path: yes; allocates: yes: .join(, println!
  - `eprintln!("{}: uncovered productions ({}) = {}", facet.label, uncovered.len(), uncovered.join(", "));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-production-coverage/🦀️.rs:53` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("[dsl-fixture-sweep] m5 production coverage auto-discovery: {} facet(s) found, {} checked, {} stdio-exempt soft failure(s), {} hard failure(s)", facets.len(), checked, soft_failures.len(), hard_failures.len());`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🪶️transfer/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🪶️transfer/🦀️.rs:120` [console.log] test-path: yes; allocates: yes: JSON.stringify, String::from, serde
  - `fn independent(bytes:&[u8]){let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok')th...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/✂️args-format-and-inspect/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/✂️args-format-and-inspect/🦀️.rs:136` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("db: {context}: {err}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/✂️args-format-and-inspect/🦀️.rs:176` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("db: {message}");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs` (21)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:135` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("db: wal migration batch close after record close failure: {batch_err}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:149` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("db: wal migration close after submission failure: {close_err}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:172` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("db: {context}: {err}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:291` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: db <command> [args...]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:292` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:293` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("commands:");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:294` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  inspect <root> [--profile test|dev|prod]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:295` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  doc <root> <document-id> [--profile ...]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:296` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  wal-inspect <root> <document-id> [--limit N]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:297` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  snapshot-inspect <root> <document-id> [--generation N] [--verify]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:298` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  verify <root> [document-id] [--profile ...]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:299` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  query <root> <document-id> <path> [more-paths...] [--profile ...]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:300` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  replay <root> <document-id>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:301` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  repair <root> <document-id>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:302` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  compact <root> <document-id> [--holder H] [--consolidate] [--profile ...]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:303` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  health <root> [--profile ...]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:304` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  conflict-simulate --touch-a p1,p2 --touch-b p2,p3 [--kind-a K] [--kind-b K]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:305` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  replica-simulate <leader-root> <follower-root> <document-id>");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:306` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  migrate <root> <document-id> <name> [--payload TEXT]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:307` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("  profile <root> <document-id> [--commands N] [--durability memory|os|fsync|quorum:N] [--policy laissez-faire|normal|vigilant] [--profile ...]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🚀️simulate-migrate-profile-and-main/🦀️.rs:335` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("db: unknown subcommand '{other}'\n");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🔭️capability-open/🧪️tests/🔬️unit/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🔭️capability-open/🧪️tests/🔬️unit/🦀️.rs:353` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("capability-completion: {} first-ready={first_ready} wakes={} waiter-empty={waiter_empty} abandoned={abandoned}", row["name"], wake.0.load(std::sync::atomic::Ordering::Acquire));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🔭️capability-open/🧪️tests/🔬️unit/🦀️.rs:416` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("capability-completion: {} terminal-before-wake={terminal_before_wake} wakes={}", row["name"], wake.0.load(std::sync::atomic::Ordering::Acquire));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/⏱️throughput/🦀️.rs` (6)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/⏱️throughput/🦀️.rs:188` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("throughput[{label}] commit {index}: tasks {} steps {} turns {} :: {}", commit.total_tasks(), commit.total_steps(), commit.total_turns(), commit.describe());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/⏱️throughput/🦀️.rs:238` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("throughput[{label}] reopen storm of {documents} documents begins");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/⏱️throughput/🦀️.rs:258` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/⏱️throughput/🦀️.rs:270` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/⏱️throughput/🦀️.rs:281` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/⏱️throughput/🦀️.rs:296` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("throughput[{label}] the same {documents} greeted one after another in {:.0} ms ({} tasks, {} turns): storm/serial {:.2}", ms(serial), serialized.total_tasks(), serialized.total_turns(), ms(storm) / ms(seria...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🏊️worker-pool-use-and-mount-owner-laws/🦀️.rs` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🏊️worker-pool-use-and-mount-owner-laws/🦀️.rs:121` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🏊️worker-pool-use-and-mount-owner-laws/🦀️.rs:134` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🏊️worker-pool-use-and-mount-owner-laws/🦀️.rs:179` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/📚️history-wake-and-terminal-close-laws/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/📚️history-wake-and-terminal-close-laws/🦀️.rs:136` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/📣️mount-fanout-shutdown-and-mount-failure-laws/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/📣️mount-fanout-shutdown-and-mount-failure-laws/🦀️.rs:250` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🧭️long-module-and-source-laws/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🧭️long-module-and-source-laws/🦀️.rs:54` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("greeting storm of {DOCUMENTS} documents begins after growing them in {grown:?}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🧭️long-module-and-source-laws/🦀️.rs:79` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🧸️fixtures-batch-and-concurrent-ensure/🦀️.rs` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️unit/🧸️fixtures-batch-and-concurrent-ensure/🦀️.rs:216` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧪️tests/🔬️unit/🛑️recovery-abort-laws/🦀️.rs` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧪️tests/🔬️unit/🛑️recovery-abort-laws/🦀️.rs:97` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("one durable WAL abort survived independent filesystem retirement and reopen, preserving the next transaction id: {name}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧪️tests/🔬️unit/🛑️recovery-abort-laws/🦀️.rs:169` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("WAL abort fault retired owners and reopened idempotently: {}, tail={tail}, fail_tail_sync={fail_tail_sync}", case["name"]);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧪️tests/🔬️unit/🛑️recovery-abort-laws/🦀️.rs:325` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("WAL abort recovery exact retained capacity plus {extra} had the expected byte-preserving disposition");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️fence-conformance/🦀️.rs` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️fence-conformance/🦀️.rs:280` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("{backend}: exited holder released after {:?} (contract {wait} ms)", exited.elapsed());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️fence-conformance/🦀️.rs:307` [eprintln!] test-path: yes; allocates: yes: .join(, println!
  - `eprintln!("wal-writer-fence {backend}: {}", proven.join(", "));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🧮️memory-backing/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🧮️memory-backing/🟦️.ts:52` [console.log] test-path: yes; allocates: no
  - `console.log(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts:107` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Activated ${variant} ${renderer} ${profile}: ${receipt.plugins.length} completed components (${changed ? "changed" : "unchanged"})');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/📥️installation/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/📥️installation/🟦️.ts:63` [console.log] test-path: no; allocates: yes: ${
  - `await withRepositoryArtifactDestinationV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:sink.control,directory:output},destination=>stageRepositoryArtifacts(destination,"development-ex...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔍️freshness/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔍️freshness/🟦️.ts:101` [console.log] test-path: no; allocates: yes: ${
  - `if (lines.length === 0) console.log('[fresh] ${facts.length} staged components match their sources and the activation receipt');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🚀️launch/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🚀️launch/🟦️.ts:17` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[dev] activating ${variant} ${renderer} ${profile} via ${target}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts:80` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Prepared ${variant} ${renderer} ${profile}: ${healthy.prepared.length} of ${session.plugins.length} components (${healthy.excluded.length} excluded), session, browser support and ${fonts} fonts');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts:155` [console.log] test-path: no; allocates: yes: ${
  - `for (const row of verdicts) console.log('${row.kind === "prepared" && activated.has(row.pluginId) ? "[staged]  " : "[missing] "}${row.pluginId}: ${row.kind}${activated.has(row.pluginId) ? ", in receipt" : ", absent fr...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts:159` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Cold boot ${variant} ${renderer} ${profile}: ${healthy.prepared.length} of ${session.plugins.length} components staged and activated, ${healthy.excluded.length} excluded');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts` (9)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:180` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(report, null, 2));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:184` [console.log] test-path: no; allocates: yes: ${
  - `console.log('parity smoke PASS for ${variant}: structural=${report.structural?.status} pixel=${report.pixel?.status} behavioral=${report.behavioral?.status} (${report.durationMs}ms)');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:204` [console.log] test-path: no; allocates: yes: ${
  - `console.log('triage ${variant}: react=${reactBoot.status}${reactBoot.detail ? ' (${reactBoot.detail})' : ""}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:205` [console.log] test-path: no; allocates: yes: ${
  - `console.log('triage ${variant}: wgpu=${wgpuBoot.status}${wgpuBoot.detail ? ' (${wgpuBoot.detail})' : ""}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:241` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(result, null, 2));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:242` [console.log] test-path: no; allocates: yes: ${
  - `console.log('probe ${variant}/${suiteName}: ${result.status} (${result.steps.length} step(s))');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:262` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${variant}: boot=${report.boot.react}/${report.boot.wgpu} structural=${report.structural?.status ?? "-"} pixel=${report.pixel?.status ?? "-"} behavioral=${report.behavioral?.status ?? "-"}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:284` [console.log] test-path: no; allocates: yes: ${
  - `console.log('sweep ${variant}: boot=${report.boot.react}/${report.boot.wgpu} structural=${report.structural?.status ?? "-"} pixel=${report.pixel?.status ?? "-"} behavioral=${report.behavioral?.status ?? "-"}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:288` [console.log] test-path: no; allocates: yes: ${
  - `console.log('parity sweep complete: ${reports.length - unresolved.length}/${reports.length} PASS');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts:48` [console.log] test-path: no; allocates: yes: ${
  - `console.log('bench: running web scale-bench (renderer=${renderer}, shards=${shardCount}) via headless Chromium — see 📊️bench-web-harness/🟦️.ts for real-vs-stub scope');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts:57` [console.log] test-path: no; allocates: yes: ${
  - `console.log('bench: wrote report -> ${outPath}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts:58` [console.log] test-path: no; allocates: yes: ${, .join(, .map(
  - `console.log('bench summary: ${rows.map((r) => '${(r as { id: number }).id}:${(r as { status: string }).status}').join(" ")}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🖥️host/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🖥️host/🟦️.ts:16` [console.log] test-path: no; allocates: yes: ${
  - `console.log('bench: running native scale-bench (shards=${shardCount})');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:109` [console.log] test-path: no; allocates: no
  - `const serve = await ensureDevServe({ repoRoot: this.repoRoot, port: devServePortV1(serveUrl), hubUrl: flag("--hub"), variant: flag("--variant"), signal: cancel.signal, onProgress: (_status, line) => console.log(line) });`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🚀️activation-vite-plugin/🏭️runtime/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🚀️activation-vite-plugin/🏭️runtime/🟦️.ts:154` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[lazy-activate] build ${pluginId} through the build driver');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️convergence/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️convergence/🟦️.ts:51` [console.log] test-path: no; allocates: yes: ${, .join(
  - `for (const row of rows) console.log('${row.ok ? "OK  " : "DIFF"} ${row.pluginId.padEnd(34)} committed=${row.committed.slice(0, 12)} dist=${row.dist.slice(0, 12)} staged=${row.staged.slice(0, 12)} activated=${row.activ...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️convergence/🟦️.ts:53` [console.log] test-path: no; allocates: yes: ${
  - `console.log('components=${rows.length} consistent=${rows.length - diverged} diverged=${diverged} receiptPlugins=${receiptPlugins}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️publication/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️publication/🧪️tests/🟦️.ts:47` [console.log] test-path: yes; allocates: no
  - `console.log("protected-pending-response-neutral: waits=8; locked browser oracle in existing containment owner; idle waits=2; expired complete=1; late originals wiped; closed observation has no response listener");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️publication/🧪️tests/🟦️.ts:65` [console.log] test-path: yes; allocates: no
  - `console.log("protected-actor-publication-neutral: 1 join; 13 semantic refusals; actual-size preallocation; latest-group retirement; URLs=2; normal-owner=3; Ajv=5 WebCrypto=3; normal wrapper consumer imported; actual c...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️publication/🧪️tests/🟦️.ts:99` [console.log] test-path: yes; allocates: no
  - `console.log("protected-actor-child-neutral: actual Chromium load/describe/detach/abort/capacity/postcancel; foreign-success forgery refused; Ajv=1 locked-Playwright/Chromium-wait-oracles=8");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🏃️execution/🟦️.ts` (6)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🏃️execution/🟦️.ts:19` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[channel-version] outward contributions: ${proveChannelVersionContributionsV1() + proveChannelVersionContributionCensusV1()} vectors passed');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🏃️execution/🟦️.ts:25` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[channel-version] describe channel: ${proveStaleChannelDescribeOutputsV1()} vectors passed');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🏃️execution/🟦️.ts:34` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[channel-version] pin ${pin}: ${listed.length} committed component descriptors, ${stale.length} stale');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🏃️execution/🟦️.ts:41` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[channel-version] describe ownership: ${proveChannelVersionDescribeOwnershipV1()} vectors passed');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🏃️execution/🟦️.ts:49` [console.log] test-path: no; allocates: yes: ${
  - `for (const path of written) console.log('[channel-version] wrote ${path}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔖️channel-version/🏃️execution/🟦️.ts:56` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[channel-version] pin ${pin}: ${source.consumers.length} registered consumers, ${findings.length} finding(s)');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🎫️session/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🎫️session/🧪️tests/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: ${, .join(, .map(, .push(
  - `test("neutral OS development actual local-only child carries genuine control and rejects copied authority before DATA",()=>receiveScriptProcessInvocation(process.env,async invocation=>{const control=createIssuedResour...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏁completion/📜️script.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏁completion/📜️script.ts:9` [console.log] test-path: no; allocates: no
  - `console.log("Production build complete: s/react/release");`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏃️execution/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏃️execution/🟦️.ts:50` [console.log] test-path: no; allocates: no
  - `console.log("distribution check: fresh");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏃️execution/🟦️.ts:57` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ outputs: plan.manifest.outputs.length, retired: result.retired, recovery: result.recovery }));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📍️output/🧪️tests/🔬️unit/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📍️output/🧪️tests/🔬️unit/🟦️.ts:54` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Distribution output: ${corpus.cases.length} portable vectors, 2 owner layouts, independent Ajv/URL/Node producer passed');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts:21` [console.log] test-path: no; allocates: no
  - `console.log(await productionBrowserInputHash(resolve(import.meta.dirname, "..")));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts:31` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Browser distribution complete: ${mode}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️testing/🚶️journey/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️testing/🚶️journey/🟦️.ts:324` [console.log] test-path: no; allocates: yes: ${, .join(, .map(, JSON.stringify
  - `const flush = (): void => { writeFileSync(resolve(out, "steps.json"), JSON.stringify({ schemaVersion: 1, renderer, locale, viewport: definition.viewport, receipts }, null, 2)); writeFileSync(resolve(out, "parity.md"),...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⏯️tool-run-matrix/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⏯️tool-run-matrix/🟦️.ts:309` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[tool-run] ${index + 1}/${programs.length} ${row.pass ? "PASS" : "FAIL"} ${row.key} tool=${row.toolId} A: start=${String(row.a.started ?? false)} progress=${(row.a.progressSeen as string[] | undefined)?.l...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⏯️tool-run-matrix/🟦️.ts:361` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[tool-run] === ${tag}: PASS ${passed}/${total} → ${outDir} ===');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⏱️interaction-latency/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⏱️interaction-latency/🟦️.ts:347` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('[interaction-latency] ${row.status.toUpperCase()} ${row.id}: input → frame p50 ${row.frames.p50} / p95 ${row.frames.p95} ms (n ${row.frames.n}), ${row.paints.perInput} paints/input (paint p50 ${row.paints...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/♨️hot-update/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/♨️hot-update/🟦️.ts:214` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `console.log('hot update delivery (${runtime}): ${JSON.stringify(Object.fromEntries(rows.map(({ style, ms }) => [style, ms])))} ms');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/♨️hot-update/🟦️.ts:239` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `console.log('dev serve restart under a connected page (${runtime}): ${restartMs} ms, close events ${closeEvents}, then ${JSON.stringify(Object.fromEntries(rows.map(({ style, ms }) => [style, ms])))} ms');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts:91` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('s studio e2e verify passed (${studioUrl})');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts:100` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('s catalog smoke passed (${report.totals.pass} programs, ${studioUrl})');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts:110` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('s studio verify passed (${studioUrl})');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts` (15)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:108` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('navigating to ${baseUrl}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:114` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('home loaded (${booted.children} nodes)');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:119` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('studio loaded at ${pathAfterCreate}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:128` [console.log] test-path: yes; allocates: no
  - `console.log("three studio windows rendered");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:133` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('spawn via ${spawnMode}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:137` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('spawn via ${spawnMode}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:142` [console.log] test-path: yes; allocates: no
  - `console.log("undo issued");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:158` [console.log] test-path: yes; allocates: no
  - `console.log("studio commands in palette");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:164` [console.log] test-path: yes; allocates: no
  - `console.log("find palette available");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:169` [console.log] test-path: yes; allocates: no
  - `console.log("studio home bar navigation works");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:176` [console.log] test-path: yes; allocates: no
  - `console.log("home vfs open studio works");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:181` [console.log] test-path: yes; allocates: yes: ${, .filter(, .join(
  - `console.log('ignored headless gpu errors: ${pageErrors.filter(isIgnorableStudioE2ePageError).join(" | ")}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:186` [console.log] test-path: yes; allocates: no
  - `console.log("PASS: S studio end-to-end workflows verified");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:286` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('[home-e2e] ${name}: ${pass ? "PASS" : "FAIL"} ${JSON.stringify(report.steps.at(-1)!.detail)}'.slice(0, 1600));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:426` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[home-e2e] === ${tag}: PASS ${passed}/${total} → ${join(outDir, tag)} ===');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts:61` [console.log] test-path: yes; allocates: yes: ${
  - `const log = (line: string): void => console.log('[two-human] ${line}'.slice(0, 1600));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts:355` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[two-human] === ${tag}: PASS ${passed}/${total} → ${join(outDir, tag)} ===');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/💤️idle-budget/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/💤️idle-budget/🟦️.ts:409` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[memory-soak] minute ${sample.minute}: heap ${sample.jsHeapMB} MB, workers ${sample.workersHeapMB} MB, nodes ${sample.nodes}, listeners ${sample.listeners}, frames/s ${sample.idle.framesPerSec}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/💤️idle-budget/🟦️.ts:434` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[idle-budget] ${row.pass ? "PASS" : "FAIL"} ${row.key}#${row.role} frames/s ${row.open?.framesPerSec ?? "-"} rAF/s ${row.open?.rafPerSec ?? "-"} draws/s ${row.open?.drawsPerSec ?? "-"} busy ${row.open?.ma...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/💤️idle-budget/🟦️.ts:450` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[idle-budget] === ${tag}: ${status.toUpperCase()} ${passed}/${rows.length} → ${outDir} ===');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/📇️canonical-bootstrap-folder-mirror/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/📇️canonical-bootstrap-folder-mirror/🟦️.ts:104` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('canonical-bootstrap-folder-mirror: phase=source payloads=canonical-pair markers=${markers.length}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/📇️canonical-bootstrap-folder-mirror/🟦️.ts:190` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('canonical-bootstrap-folder-mirror: phase=process payloads=canonical-pair sqlite=1 sha256=2 frontiers=${canonicalPairCorpus.frontierCases.length} stale=7 hostile=${corpus.hostile.length} mixed=1 monotonic-...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔀️connection-budget/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔀️connection-budget/🟦️.ts:145` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[connection-budget] ${options.tag}: shell ${beacon ?? "never ready"}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔀️connection-budget/🟦️.ts:280` [console.log] test-path: yes; allocates: yes: ${
  - `for (const violation of report.violations) console.log('[connection-budget] VIOLATION ${violation.en}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔀️connection-budget/🟦️.ts:281` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[connection-budget] === ${tag}: ${status.toUpperCase()} → ${join(outDir, tag)} ===');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔬️catalog-smoke/🟦️.ts` (6)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔬️catalog-smoke/🟦️.ts:216` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[catalog-smoke] navigating to ${baseUrl}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔬️catalog-smoke/🟦️.ts:223` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[catalog-smoke] document committed after ${phaseMs.commit} ms');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔬️catalog-smoke/🟦️.ts:239` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[catalog-smoke] readiness beacon: ${beacon ?? "none"} (commit ${phaseMs.commit} ms, first module ${phaseMs.firstModule ?? "never"} ms, beacon ${phaseMs.beacon ?? "never"} ms)');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔬️catalog-smoke/🟦️.ts:263` [console.log] test-path: yes; allocates: yes: ${
  - `else console.log('[catalog-smoke] ${probe!.programs.length} spawnable programs, ${probe!.plugins.length} registry rows');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔬️catalog-smoke/🟦️.ts:297` [console.log] test-path: yes; allocates: no
  - `console.log(catalogSmokeMarkdown(report));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔬️catalog-smoke/🟦️.ts:298` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[catalog-smoke] report written to ${opts.outDir}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🚀️local-hub/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🚀️local-hub/🟦️.ts:419` [console.log] test-path: yes; allocates: no
  - `console.log("ready");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🚀️local-hub/🟦️.ts:422` [console.log] test-path: yes; allocates: no
  - `console.log("claim:" + claim.kind);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🥾️boot-budget/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🥾️boot-budget/🟦️.ts:260` [console.log] test-path: yes; allocates: no
  - `console.log(line);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🥾️boot-budget/🟦️.ts:276` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[boot-budget] ${verdict.status.toUpperCase()} ${timings}; cold payload ${payloadLine(measured.cold.payload)}${verdictLine}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧩️contribution/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧩️contribution/🟦️.ts:44` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Selected browser factories: ${corpus.valid.length + corpus.invalid.length} portable vectors with independent Ajv and real Node process');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧩️contribution/🟦️.ts:55` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Development contribution oracle cases=${fixture.invalid.length + fixture.validPaths.length + fixture.invalidPaths.length + 2}');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🌉️fixtures-and-pixels/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🌉️fixtures-and-pixels/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify
  - `const oracle = JSON.parse(execFileSync("node", ["--input-type=module", "--eval", 'import { readFileSync } from "node:fs"; const { registry, catalog } = JSON.parse(readFileSync(0, "utf8")); console.log(JSON.stringify({...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🗃️handles-and-output/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🗃️handles-and-output/🟦️.ts:8` [console.log] test-path: yes; allocates: yes: .map(, .push(, JSON.stringify
  - `const handleCache = (...paths: readonly string[]): { readonly same: boolean[] } => underBun('const { backboneDbHandleFor } = await import(process.argv[1]); const paths = JSON.parse(process.argv[2]); const handles = []...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🚚️vendor-transport/🟦️.ts` (5)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🚚️vendor-transport/🟦️.ts:34` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `const closure = underBun<{ paths: string[]; rejected: boolean[] }>('const { distributionStaticSourcePaths } = await import(process.argv[1]); const [sandbox, entry, rejectedEntries] = JSON.parse(process.argv[2]); const...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🚚️vendor-transport/🟦️.ts:215` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(rows));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🚚️vendor-transport/🟦️.ts:276` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log('[BUILD_INSPECTION]' + JSON.stringify(rows));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🚚️vendor-transport/🟦️.ts:278` [console.log] test-path: yes; allocates: no
  - `child.stderr?.on("data", (chunk) => { if (String(chunk).includes("[TRACE]")) console.log(String(chunk).trim()); });`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🚚️vendor-transport/🟦️.ts:320` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(results));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🧬️jco-rewrites/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🧬️jco-rewrites/🟦️.ts:149` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ actual, independent }));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🧬️jco-rewrites/🟦️.ts:184` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(results));`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🎪️run/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🎪️run/🟦️.ts:202` [console.log] test-path: yes; allocates: no
  - `console.log(summary);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🎪️run/🟦️.ts:203` [console.log] test-path: yes; allocates: no
  - `for (const line of stepLines) console.log(line);`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts:67` [console.log] test-path: yes; allocates: no
  - `return ensureDevServe({ ...request, hubUrl: request.hubUrl ?? undefined, repoRoot, signal: invocation.control.signal, beforeSpawn: async () => (await import("../../♻️activation/🚀️launch/🟦️.ts")).activatePlaygroundRunt...`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts:585` [console.log] test-path: yes; allocates: yes: ${
  - `const log = (line: string): void => console.log('[matrix] ${line}');`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts:890` [console.log] test-path: yes; allocates: no
  - `const serve = await ensureDevServe({ repoRoot, port: devServePortV1(options.serveUrl), hubUrl: options.hubUrl, locale: options.locale === "de" ? "de" : "en", signal: options.signal, onProgress: (_status, line) => cons...`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts:957` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[matrix] === ${tag}: PASS ${passed}/${total} → ${join(outDir, tag)} ===');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️capability-policy/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️capability-policy/🟦️.ts:183` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('program capability lint passed (${checkedPackageCount} plugin package(s) evaluated, ${grandfathered.length} grandfathered warning(s))');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️export-path-policy/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️export-path-policy/🟦️.ts:64` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('plugin index export path lint: ${totalDead}/${totalAll} dead relative export path(s) across ${pluginsWithDeadPaths} plugin(s) — REPORT ONLY, does not gate (26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE)');`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️host-handle-policy/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️host-handle-policy/🟦️.ts:143` [console.log] test-path: yes; allocates: no
  - `console.log(`

### `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs` (8)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:471` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("usage: semio <inspect|verify|open|convert> <path> [...]");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:478` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio] missing path argument");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:485` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio] read {path}: {err}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:497` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio] inspect failed: {err}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:507` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio] verify failed: {err}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:515` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio] {cmd} sniff failed: {err}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:522` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio] handler: {detail}");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:529` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio] unknown command {cmd}");`

### `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🎞️media-persistence-and-retirement/🟦️.ts` (5)

- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🎞️media-persistence-and-retirement/🟦️.ts:47` [console.log] test-path: no; allocates: yes: ${
  - `console.log('media contract presentation oracle: ${fixture.cases.length} document/binary/conversion vectors, ${denied} strict hostile denials; no native claim');`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🎞️media-persistence-and-retirement/🟦️.ts:50` [console.log] test-path: no; allocates: yes: ${, .join(, .map(
  - `console.log('media contract presentation native assertions=${receipts.reduce((total, receipt) => total + receipt.assertions, 0)}; executables=${receipts.map((receipt) => receipt.sha256).join(",")}');`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🎞️media-persistence-and-retirement/🟦️.ts:135` [console.log] test-path: no; allocates: yes: ${
  - `console.log('persistence contract oracle: requests=${requests.size}, hostileRequests=${fixture.requestNegatives.length}, hostileEvents=${eventDenials}, traces=${fixture.traces.length}, accepted=${accepted}, rejected=$...`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🎞️media-persistence-and-retirement/🟦️.ts:139` [console.log] test-path: no; allocates: yes: ${
  - `console.log('persistence contract native assertions=${receipts[0]!.assertions}; executable=${receipts[0]!.sha256}');`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🎞️media-persistence-and-retirement/🟦️.ts:173` [console.log] test-path: no; allocates: yes: ${
  - `console.log('owned retirement oracle: cases=${fixture.cases.length}, budgets=${fixture.budgets.length}, exactByteGrants=${grants}, declaredHostile=${fixture.hostile.length}; native pending');`

### `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🏭️member-factory-identity/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🏭️member-factory-identity/🟦️.ts:136` [console.log] test-path: no; allocates: no
  - `console.log('[TRACE] selected MemberFactory identity oracle: exact18 source-owned declarations,21 selection rows +7 selection/5 semantic lifecycle traces x3 grants,9 persisted schema/dialect/owner cases with literal i...`

### `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/📚️member-history-dictionary/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/📚️member-history-dictionary/🟦️.ts:93` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] dictionary payload cursor oracle: ${accepted} accepted / ${fixture.cases.length - accepted} denied exact wires ×3 grants; exact ordered Begin/base/count and Entry/ranges, UTF8 scratch0..4, event f...`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/📚️member-history-dictionary/🟦️.ts:130` [console.log] test-path: no; allocates: no
  - `console.log("[TRACE] retained history foundation: exactly5 native assertions; framing2 + private input2 + tagged-ID1; no dictionary/member factory/publication claim");`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/📚️member-history-dictionary/🟦️.ts:381` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] retained dictionary owner oracle: ${accepted} accepted / ${fixture.cases.length - accepted} denied committed SPR histories, ${fixture.lifecycle.length} owner traces + ${fixture.recordRetirement.le...`

### `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🚪️member-open/🟦️.ts` (2)

- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🚪️member-open/🟦️.ts:77` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] member open request oracle: ${fixture.cases.length} admission cases, ${fixture.framing.length} framing cases, ${fixture.retention.length} declared retained-stage cases; rejected input bytes retain...`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🚪️member-open/🟦️.ts:154` [console.log] test-path: no; allocates: no
  - `console.log("public member-open handoff oracle:20 projection/input/capacity cases + exact18 operation arms +8 Flow lifecycle traces; one bounded Flow decoder,17 explicit retained denials, schema+owner derived, publica...`

### `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🪪️member-history-identity/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🪪️member-history-identity/🟦️.ts:94` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] retained member history identity schema: ${accepted} accepted / ${ids.size - accepted} denied neutral records; independent LEB128/UTF-8/ID model; no Rust semantic decoder or typed hydration claime...`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🪪️member-history-identity/🟦️.ts:188` [console.log] test-path: no; allocates: yes: ${
  - `console.log('retained history owner oracle: ${accepted} accepted / ${fixture.inputs.length - accepted} denied derived SPR inputs, ${fixture.lifecycle.length} owner-state traces, 3 fuel grants, exact 4531-byte paged re...`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/🎛️commands/🪪️member-history-identity/🟦️.ts:255` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] retained semantic ID oracle: ${accepted} accepted / ${fixture.cases.length - accepted} denied exact tagged wires +${fixture.completion.length} non-mutating completion boundaries; independent LEB12...`

### `🧰️framework/🛍️products/💻️os/🧪️testing/⚖️scale/📤️publication/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🧪️testing/⚖️scale/📤️publication/🟦️.ts:55` [console.log] test-path: no; allocates: no
  - `console.log("scale-fixture check: fresh");`

### `🧰️framework/🛍️products/💻️os/🧪️testing/⚖️scale/📽️projection/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🧪️testing/⚖️scale/📽️projection/🟦️.ts:204` [console.log] test-path: no; allocates: yes: ${
  - `console.log('scale-fixture generate: ${registry.recordCount} records (plugins=${pluginCount} extensions=${extensionsPerPlugin} seed=${seed}) -> ${dir}');`

### `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-host/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-host/🟦️.ts:38` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[jco-probe] serving ${ROOT} on http://localhost:${server.port}');`

### `🧰️framework/🛍️products/💻️os/🧪️tests/🎭️browser-actor-reservation/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🧪️tests/🎭️browser-actor-reservation/🟦️.ts:464` [console.log] test-path: yes; allocates: no
  - `console.log("document-session-activation: socket-path=1 neutral=" + corpus.cases.length + " worker=mocked final-capacity=0");`

### `🧰️framework/🛍️products/💻️os/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (3)

- `🧰️framework/🛍️products/💻️os/🧪️tests/🔬️typescript-package-commands/🟦️.ts:13` [console.log] test-path: yes; allocates: no
  - `console.log("installed-service-check", verifyInstalledServiceLawsV1());`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🔬️typescript-package-commands/🟦️.ts:111` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('document-opening-attempt-oracle: AJV=1 opening=${observed.length} rebootstrap=${rebootstrapObserved.length} source-hostiles=${hostiles.length}');`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🔬️typescript-package-commands/🟦️.ts:120` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('document-opening-attempt-check: checks=${checks}');`

### `🧰️framework/🛍️products/💻️os/🧪️tests/🛡️access-policy/🟦️.ts` (1)

- `🧰️framework/🛍️products/💻️os/🧪️tests/🛡️access-policy/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[access-policy] policies=${fixture.policies.length + 1} decisions=${fixture.vectors.length}');`

### `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️.ts` (4)

- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️.ts:10` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[jcoprobe-callback] ${id}: ${ok ? "PASS" : "FAIL"} — ${detail}');`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️.ts:69` [console.log] test-path: yes; allocates: no
  - `console.log("[jcoprobe-callback] ==== VERDICTS ====");`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️.ts:71` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[jcoprobe-callback] ${v.id}: ${v.ok ? "PASS" : "FAIL"} — ${v.detail}');`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧩️jcoprobe-callback/🟦️.ts:74` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[jcoprobe-callback] overall: ${allPass ? "ALL PASS" : "SOME FAILED"}');`

### `🧰️framework/🛍️products/📓️print/🎮️commands/📊️viz-catalog-generation/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🎮️commands/📊️viz-catalog-generation/🟦️.ts:9` [console.log] test-path: no; allocates: yes: ${
  - `console.log('print: wrote ${written.length} visualization catalogue artifacts');`

### `🧰️framework/🛍️products/📓️print/🎮️commands/🔤print-font-provisioning/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🎮️commands/🔤print-font-provisioning/🟦️.ts:11` [console.log] test-path: no; allocates: yes: ${
  - `console.log('print: staged ${provisioning.files} authored fonts');`

### `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🟦️.ts:43` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[gallery] evidence holds ${Object.keys(evidence.variants).length} variants');`

### `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/📄️macro-and-pdf-verification/🟦️.ts` (4)

- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/📄️macro-and-pdf-verification/🟦️.ts:66` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[print] Published glyph policy: ${policy.accepted.length} accepted cases, ${policy.rejectedCodePoints.length} rejected scalars PASS');`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/📄️macro-and-pdf-verification/🟦️.ts:107` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[print] API visible titles ${name}: ${contents} contents pages, ${captions} measured captions PASS');`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/📄️macro-and-pdf-verification/🟦️.ts:116` [console.log] test-path: yes; allocates: yes: ${, collect
  - `console.log('[print] Consumed ${collection} ${template.id}/${name}: ${pdf.numPages} pages PASS (${count}/${expected[collection]})');`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/📄️macro-and-pdf-verification/🟦️.ts:127` [console.log] test-path: yes; allocates: yes: ${, collect
  - `console.log('[print] PDF.js consumption ${collection}: ${count} PDFs PASS');`

### `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🖼️gallery-carrier-and-kind-paint/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🖼️gallery-carrier-and-kind-paint/🟦️.ts:93` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[print] Gallery carrier ${language}/${appearance}: ${control.cases.length} actual frames, marked path/text extents, explicit-height control, ${pdf.numPages} PDF.js pages measured');`

### `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🧾️api-freshness-and-titles/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🧾️api-freshness-and-titles/🟦️.ts:198` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[print] API localized title rendering ${language}: ${visual.headings.length} headings, ${captions} measured captions, ${pdf.numPages} PDF.js pages PASS');`

### `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/📜️script.ts` (1)

- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/📜️script.ts:13` [console.log] test-path: no; allocates: no
  - `console.log(changed?"[print] Wrote LaTeX design tokens":"[print] LaTeX design tokens are current");`

### `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🧪️testing/📏️measurement/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🧪️testing/📏️measurement/🟦️.ts:55` [console.log] test-path: no; allocates: yes: ${, .map(, JSON.stringify, new Map
  - `export function writePrintGalleryEvidence(execution:PrintExecutionV1,sections:readonly string[]):Promise<GalleryRenderEvidence>{if(sections.length>measurementPolicy.maximumSections)throw Error("Finite gallery section ...`

### `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🩺️coverage-verification/🟦️.ts` (3)

- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🩺️coverage-verification/🟦️.ts:201` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] print: viz coverage ${report.leaves}/${report.leaves} leaves through ${report.kinds} kinds, API ${VIZ_API_COMMANDS.length}/${VIZ_API_COMMANDS.length}');`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🩺️coverage-verification/🟦️.ts:202` [console.log] test-path: no; allocates: yes: ${, .join(
  - `if (report.unknownFamilies.length > 0) console.log('[TRACE] print: ${report.unknownFamilies.length} families still awaiting a \\SemioVizFamily registration: ${report.unknownFamilies.slice(0, 8).join(" ")}…');`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🩺️coverage-verification/🟦️.ts:209` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[TRACE] print: every viz family of ${report.kinds} kinds is registered');`

### `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📜️script.ts` (2)

- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📜️script.ts:11` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Print fonts staged: ${result.files} authored OpenType assets');`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📜️script.ts:113` [console.log] test-path: no; allocates: no
  - `console.log("[print-fonts] source-bound primary/CJK metrics derived");`

### `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts` (1)

- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts:5` [console.log] test-path: no; allocates: yes: ${
  - `class PrepareScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Print bundle preparation accepts no arguments");const phaseControl=createIssuedResourceControlV1(this.inv...`

### `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📜️script.ts` (2)

- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📜️script.ts:10` [console.log] test-path: no; allocates: yes: ${, .filter(, collect
  - `class CompleteScript extends BundleScript{async run(args:string[]):Promise<void>{const[collection]=args;if(args.length!==1||!['templates','visualizations'].includes(collection??""))throw new Error("Print completion re...`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📜️script.ts:11` [console.log] test-path: no; allocates: no
  - `class WatchScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Print watch accepts no compiler arguments");const phase=createIssuedResourceControlV1(this.invocation,"prin...`

### `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts` (1)

- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts:6` [console.log] test-path: no; allocates: yes: ${
  - `class PrepareScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Tectonic preparation accepts no arguments");const phaseControl=createIssuedResourceControlV1(this.invocat...`

### `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🟦️.ts:11` [console.log] test-path: no; allocates: yes: ${
  - `export async function runVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {if(args.length)throw new Error("Tectonic verification accepts n...`

### `🧰️framework/🛍️products/📓️print/🚪️io/🧪️tests/🏛️ownership/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🚪️io/🧪️tests/🏛️ownership/🟦️.ts:17` [console.log] test-path: yes; allocates: no
  - `console.log("print-io-ownership representations=text,binary,sqlite oracle=minimatch");`

### `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📥️native-fixtures/🦀️.rs` (1)

- `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📥️native-fixtures/🦀️.rs:11` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify, format!, serde
  - `for(index,case)in cases.iter().enumerate(){let expected=owned(case);let text=expected.print_dsl();let binary=expected.encode_pack_with(&Default::default()).unwrap();let from_text=ChartSnapshot::parse_dsl(&text).unwrap...`

### `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (4)

- `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:25` [console.log] test-path: yes; allocates: yes: JSON.stringify, serde
  - `let c=corpus();for case in c["cases"].as_array().unwrap(){let expected=owned(case);let bytes=file(&expected);assert_eq!(&bytes[..16],b"SQLite format 3\0");let database=import_sqlite_database(&bytes,Default::default(),...`
- `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:34` [console.log] test-path: yes; allocates: yes: JSON.stringify, serde, to_string(
  - `let bytes=export_sqlite_database(&database,Default::default(),&mut |_|true).unwrap();let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));console.l...`
- `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:39` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify, serde
  - `let c=corpus();let expected=owned(&c["cases"][0]);let bytes=file(&expected);let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));console.log(JSON.s...`
- `🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:73` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify, serde, vec!
  - `fn sqlite_snapshot_chart_neutral_ieee_words_and_producer_companions_are_stable(){let c=corpus();let mut expected=owned(&c["cases"][0]);let mut columns=Vec::new();let mut row=Vec::new();for case in c["numericWords"].as...`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🎟️birth/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🎟️birth/🟦️.ts:23` [console.log] test-path: yes; allocates: no
  - `console.log("[print] Prepared thesis input " + kind);`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/📤️publication/🟦️.ts` (2)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/📤️publication/🟦️.ts:47` [console.log] test-path: yes; allocates: no
  - `console.log("[print] Actual compilation entry built " + row.id);`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/📤️publication/🟦️.ts:109` [console.log] test-path: yes; allocates: no
  - `console.log("[print] Privately published, consumed and exported " + name + " PASS");`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🖨️rendering/🟦️.ts` (4)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🖨️rendering/🟦️.ts:114` [console.log] test-path: yes; allocates: no
  - `console.log("[print] Thesis rendering " + row.language + "/" + row.theme + "/" + row.degree + (phd ? "/phd" : "") + " PASS");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🖨️rendering/🟦️.ts:125` [console.log] test-path: yes; allocates: no
  - `console.log("[print] Thesis refusal " + row.id + " PASS");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🖨️rendering/🟦️.ts:156` [console.log] test-path: yes; allocates: no
  - `console.log("[print] Thesis customization " + row.language + " PASS");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🖨️rendering/🟦️.ts:197` [console.log] test-path: yes; allocates: no
  - `console.log("[print] Minimal thesis and Babel scope " + row.language + " PASS");`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🧫️workspace/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🎓️thesis/🧫️workspace/🟦️.ts:84` [console.log] test-path: yes; allocates: no
  - `console.log("[print] Private source snapshot " + roster.length + " files/" + bytes + " original bytes");`

### `🧰️framework/🛍️products/📓️print/🧪️tests/📚️catalog-coverage/🟦️.ts` (2)

- `🧰️framework/🛍️products/📓️print/🧪️tests/📚️catalog-coverage/🟦️.ts:142` [console.log] test-path: yes; allocates: no
  - `console.log("print: catalog-coverage — 8 fundamental scenarios passed");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📚️catalog-coverage/🟦️.ts:148` [console.log] test-path: yes; allocates: no
  - `console.log("print: catalog-coverage — family registration complete");`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🔬️script-commands/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🔬️script-commands/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${
  - `for (const path of generateVizArtifacts(this.repoRoot)) console.log('[print-generate] ${path}');`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/⚛️physics/🟦️.ts` (2)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/⚛️physics/🟦️.ts:117` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-physics] ${theme}: ${cases.length} compiled cases, ${referenceCount} D3 physical projections, ${potentialReferences} D3 PDF potential points, ${potentialCaptions} actual path/caption checks, ${pag...`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/⚛️physics/🟦️.ts:121` [console.log] test-path: yes; allocates: no
  - `console.log("[native-physics] stock/authored forces, motion, levels, bands, optics and Stokes containment PASS");`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🌳️tree/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🌳️tree/🟦️.ts:51` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-grammar] ${theme}: ${count} D3 hierarchy frame projections, ${cases.length} actual contained families, ${labelPairs} separated PDF label pairs, ${pages} PDF.js pages matched');`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts` (14)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:120` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-families] ${theme}: ${fixture.cases.length} compiled family cases, ${pages} PDF.js pages consumed');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:130` [console.log] test-path: yes; allocates: yes: ${
  - `else console.log('[native-families] ${entry.id}: actual compiler admission rejection PASS');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:133` [console.log] test-path: yes; allocates: no
  - `console.log("[native-families] scientific containment, reaction glyphs, board cells and process labels PASS");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:170` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-queueing] ${theme}: ${fixture.queueingCases.length} cases, ${captions} actual captions, ${projections} D3 station projections, ${pages} PDF.js pages consumed');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:173` [console.log] test-path: yes; allocates: no
  - `console.log("[native-queueing] stock and authored mathematical captions PASS");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:204` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-math-records] ${theme}: ${cases.length} cases, ${glyphs} actual glyphs, ${references} D3 values, ${texts.length} PDF.js pages consumed');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:207` [console.log] test-path: yes; allocates: no
  - `console.log("[native-math-records] protected captions and shared record consumers PASS");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:252` [console.log] test-path: yes; allocates: no
  - `console.log("[native-families] " + theme + ": 18 native categorical paints match D3 and legend group domains, " + pages + " actual PDF.js pages consumed PASS");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:280` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-sunpath] ${theme}: ${sunpaths.cases.length} cases, ${points} actual D3/SunCalc points, ${pages} PDF.js pages PASS');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:308` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-families] ${theme}: 128 evaluation cells match D3, ${pages} actual PDF.js pages consumed PASS');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:338` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-complex] ${theme}: ${points} sampled points match D3, ${pages} actual PDF.js pages consumed PASS');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:364` [console.log] test-path: yes; allocates: no
  - `console.log("[native-grids] "+theme+": "+points+" actual D3 square positions and "+lines+" D3 scale grid lines, "+pages+" PDF.js pages PASS");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:397` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-bifurcation] ${theme}: ${points} retained points, ${fills} actual PDF.js fills, ${pages} pages PASS');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🟦️.ts:427` [console.log] test-path: yes; allocates: no
  - `console.log("[native-orbital] " + theme + ": " + chemistry.cases.length + " cases, 7 actual D3 column centers, mathematical glyphs and horizontal bounds, " + pages + " PDF.js pages PASS");`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🫧️sets/🟦️.ts` (2)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🫧️sets/🟦️.ts:142` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-sets] ${theme}: ${cases.length} cases, ${values} independent values, ${paths} actual PDF circle strokes, ${pages} PDF.js pages consumed');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🖼️family-containment/🫧️sets/🟦️.ts:145` [console.log] test-path: yes; allocates: no
  - `console.log("[native-sets] D3 radii, polygon-clipping overlap/distance and actual PDF containment PASS");`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🧩️frame-junctions/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🧩️frame-junctions/🟦️.ts:104` [[DEBUG]] test-path: yes; allocates: no
  - `assert.ok(!core.includes("[DEBUG]") && !window.includes("[DEBUG]"));`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️chart-mutations/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️chart-mutations/🟦️.ts:52` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const source = '(async()=>{const {chromium}=await import("playwright"),browser=await chromium.launch({headless:true,timeout:30000});try{const page=await browser.newPage();await page.goto(${JSON.stringify(server.url.hr...`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🎛️family-and-scientific/🟦️.ts` (1)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🎛️family-and-scientific/🟦️.ts:137` [console.log] test-path: yes; allocates: no
  - `console.log("[native-customization] "+controls.cases.length+" mutation/inference cases, "+checks+" independent D3/PDF.js checks, "+pages+" actual pages and "+visible+" visible PDF shapes across both themes");`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/💹️financial/🟦️.ts` (2)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/💹️financial/🟦️.ts:31` [console.log] test-path: yes; allocates: no
  - `console.log("[native-grammar] "+checks+" actual financial mean/deviation/band records across "+fixture.cases.length+" neutral cases and 2 themes matched independent D3");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/💹️financial/🟦️.ts:39` [console.log] test-path: yes; allocates: no
  - `console.log("[native-grammar] "+fixture.stockKinds.length+" authored financial/economic stock kinds compiled in 2 themes");`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts` (12)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:146` [console.log] test-path: yes; allocates: no
  - `console.log("[native-grammar] actual native-emitted TikZ compiled");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:188` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-grammar] ${fixture.marks.length} canonical marks, named scales and row styles matched D3');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:197` [console.log] test-path: yes; allocates: no
  - `console.log("[native-grammar] ordered detail paths, clipping, continuous color and numeric temporal scales passed");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:205` [console.log] test-path: yes; allocates: no
  - `console.log('[native-grammar] all seven authored coordinate frames matched independent D3 scales and coordinate oracles');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:242` [console.log] test-path: yes; allocates: no
  - `console.log('[native-grammar] authored reverse, piecewise, Lab/HCL, ISOZ, ticks and independent paint alpha passed');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:257` [console.log] test-path: yes; allocates: no
  - `console.log('[native-grammar] custom palette colors, wrapping, expandable accessors, reset and brand controls matched D3');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:279` [console.log] test-path: yes; allocates: no
  - `console.log('[native-grammar] 12 authored numeric, color, ordinal and temporal unknown fallbacks matched D3');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:289` [console.log] test-path: yes; allocates: no
  - `console.log('[native-grammar] nullable line gaps and authored fallback positions matched D3');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:299` [console.log] test-path: yes; allocates: no
  - `console.log('[native-grammar] stock and authored graph family layouts preserved named node order and matched D3');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:305` [console.log] test-path: yes; allocates: no
  - `console.log('[native-grammar] six authored minor tick positions matched independent D3 scale');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:318` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-grammar] ${transforms.length} native transform columns matched independent D3');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts:321` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-grammar] ${layouts.length} native layout geometry checks matched independent D3 and dagre');`

### `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts` (10)

- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts:79` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-grammar] ${expected.size} shared typed unknown scale inputs matched independent D3');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts:112` [console.log] test-path: yes; allocates: no
  - `console.log('[native-grammar] 10 shared missing-position and nullable-paint cases matched D3');`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts:177` [console.log] test-path: yes; allocates: yes: .filter(
  - `console.log('[native-grammar] '+(cases.length*rows.length)+' directed graph boundaries including '+(cases.length*rows.filter(row=>row[0]===row[1]).length)+' loops and '+(loopOnly?0:a.anchors.length)+' angular title an...`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts:195` [console.log] test-path: yes; allocates: no
  - `console.log("[native-grammar] "+(fontVectors.sizes.length*fontVectors.cases.length)+" actual SemioSans advances matched independent Canvas and tracked Latin shaping");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts:229` [console.log] test-path: yes; allocates: no
  - `console.log("[native-grammar] "+count+" native legend geometry records across "+specifications.length+" neutral controls matched canonical inference and independent D3/Canvas; 3 canonical font-selected TikZ plans comp...`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts:246` [console.log] test-path: yes; allocates: no
  - `console.log("[native-grammar] "+titles+" title and "+labels+" label PDF text faces matched tracked native font roles");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts:325` [console.log] test-path: yes; allocates: no
  - `console.log("[spatial-data] "+control.cases.length*4+" replayed spatial table views compiled across EN/de and light/dark with independent D3 output checks");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts:379` [console.log] test-path: yes; allocates: no
  - `console.log("[geo-palette] "+control.cases.length+" mutation/inference controls, "+control.cases.length*control.inputs.length+" D3 full hue/mix/RGB checks, exact palette classes and native integration counts");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts:396` [console.log] test-path: yes; allocates: no
  - `console.log("[geo-palette] "+control.legendCases.length+" actual one-level diverging legends compiled in en/de");`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🧱️basics/🟦️.ts:431` [console.log] test-path: yes; allocates: no
  - `console.log("[native-scale-options] "+control.cases.length+" mutation/inference controls and "+control.cases.length*(control.inputs.length+1)+" independent D3 native checks");`

### `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧪️tests/🔬️typescript-package-commands/🟦️.ts` (3)

- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧪️tests/🔬️typescript-package-commands/🟦️.ts:65` [console.log] test-path: yes; allocates: yes: ${
  - `for (const [module, tally] of [...byModule].sort((a, b) => a[0].localeCompare(b[0]))) console.log('[viz-inference] ${module.padEnd(11)} passed=${tally.passed} failed=${tally.failed}');`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧪️tests/🔬️typescript-package-commands/🟦️.ts:67` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[viz-inference] level=${level} checks=${checks.length} passed=${passed} failed=${failed.length} errored=${errored.length}');`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🧪️tests/🔬️typescript-package-commands/🟦️.ts:79` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[viz-inference] barrel exports ${Object.keys(barrel).length} symbols');`

### `🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs` (1)

- `🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs:37` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[ERROR] {context}: {cause}");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/🦀️.rs` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🔌️mcp-verb/🦀️.rs:159` [eprintln!] test-path: no; allocates: yes: println!
  - `for warning in &outcome.warnings { eprintln!("Warning: {warning}"); }`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🚪️entrypoint/🦀️.rs` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🚪️entrypoint/🦀️.rs:7` [eprintln!] test-path: no; allocates: yes: println!, to_string(
  - `let root = match std::env::current_dir().map_err(|error| error.to_string()).and_then(|current| semio_framework_repo_cli::repo_cli::entrypoint::resolve_native_mcp_root(&plan.arguments, &current)) { Ok(root) => root, Er...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🚪️entrypoint/🦀️.rs:11` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{}", semio_framework_repo_cli::repo_cli::entrypoint::LAUNCHER_REFUSAL);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⌨️usage/🦀️.rs` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⌨️usage/🦀️.rs:36` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{text}");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🦀️.rs` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🦀️.rs:285` [eprintln!] test-path: no; allocates: yes: println!, serde, to_string(
  - `match result { Ok(preferences) => { println!("{}", serde_json::to_string(&preferences).unwrap_or_default()); 0 }, Err(error) => { eprintln!("[dashboard] {error}"); 2 } }`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/⌨️command/🦀️.rs` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/⌨️command/🦀️.rs:21` [eprintln!] test-path: no; allocates: yes: println!, to_string(
  - `return match lifecycle::serve(&root,&running){Ok(())=>0,Err(error)=>{let selected=dashboard_preferences::cli_locale(&root,parsed);for choice in dashboard_preferences::language::declared().choices(){let locale=dashboar...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/⌨️command/🦀️.rs:23` [eprintln!] test-path: no; allocates: yes: println!
  - `let Some(locale)=dashboard_preferences::cli_locale(&root, parsed) else { eprintln!("{}",dashboard_preferences::language::declared().message("required")); return 2; };`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/⌨️command/🦀️.rs:37` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{}", text.cli_daemon_usage.as_str());`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🚦️lifecycle/🦀️.rs` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🚦️lifecycle/🦀️.rs:66` [eprintln!] test-path: no; allocates: yes: println!, to_string(
  - `Err(error) => { eprintln!("{}", text.cli_daemon_start_failed.fill(&[("error", &error.to_string())]).into_string()); 1 }`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/🚦️lifecycle/🦀️.rs:92` [eprintln!] test-path: no; allocates: yes: println!, to_string(
  - `Err(error) => { eprintln!("{}", text.cli_daemon_stop_failed.fill(&[("error", &error.to_string())]).into_string()); 1 }`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🏛️repo-domain/🦀️.rs` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🏛️repo-domain/🦀️.rs:556` [eprintln!] test-path: no; allocates: yes: println!
  - `Some(action) => match action.execute_result(root) { Ok(output) => { println!("{output}"); 0 }, Err(error) => { eprintln!("[dashboard] {error}"); 1 } },`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🏛️repo-domain/🦀️.rs:557` [eprintln!] test-path: no; allocates: yes: println!
  - `_ => { eprintln!("[dashboard] invalid repo action"); 2 }`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📚️inventory/🦀️.rs` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📚️inventory/🦀️.rs:370` [eprintln!] test-path: no; allocates: yes: println!
  - `Some(snapshot) => { if let Err(error) = write_snapshot(snapshot_path, &snapshot) { eprintln!("[semio] command snapshot: {error}"); } snapshot.registry() }`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🔌️plugin-registry/🦀️.rs` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🔌️plugin-registry/🦀️.rs:18` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{problem}");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🔌️plugin-registry/🦀️.rs:23` [eprintln!] test-path: no; allocates: yes: format!, println!
  - `_ => {let project=std::env::var("SEMIO_CATALOG_PUBLISHER_PROJECT").unwrap_or_else(|_|"@semio-tech/plugin-registry".into());if project.is_empty()||project.starts_with("-")||project.contains(char::is_whitespace){eprintl...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs:101` [eprintln!] test-path: no; allocates: yes: .join(, println!, to_string(
  - `if !parsed.segments.is_empty() { for choice in dashboard_preferences::language::declared().choices(){let locale=ui_locale::Locale::parse(&choice.language).expect("validated choice");if dashboard_preferences::cli_local...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs:104` [eprintln!] test-path: no; allocates: yes: println!
  - `let (mut preferences, problems) = match loaded { Ok(loaded) => loaded, Err(error) => { let failure=dashboard_feedback::TerminalFailure{primary:dashboard_feedback::Failure::io(dashboard_feedback::Operation::Preferences...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs:106` [eprintln!] test-path: no; allocates: yes: println!, to_string(
  - `let mut term=match NativeTerminal::new(){Ok(term)=>term,Err(error)=>{let failure=dashboard_feedback::TerminalFailure{primary:dashboard_feedback::Failure::text(dashboard_feedback::Operation::Enter,error.to_string()),cl...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🦀️.rs:123` [eprintln!] test-path: no; allocates: yes: println!
  - `match result{Ok(())=>0,Err(error)=>{eprintln!("{}",error.display(preferences_locale));1}}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🛤️task-routes/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🛤️task-routes/🟦️.ts:30` [console.log] test-path: no; allocates: yes: ${
  - `async run(): Promise<void> { console.log('[dashboard] Installed ${await installDashboardArtifactV1(this.root,this.repoRoot,dashboardOptions(this.invocation,this.repoRoot))}'); }`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🛤️task-routes/🟦️.ts:158` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[dashboard] Rebuilding ${mode} because ${reason}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🦀️.rs` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🦀️.rs:52` [eprintln!] test-path: no; allocates: yes: println!
  - `if locale.is_none() && rest.iter().any(|word| word.as_str() == "--check") { eprintln!("{}", dashboard_preferences::language::declared().message("required")); return 2; }`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🦀️.rs:93` [eprintln!] test-path: no; allocates: yes: println!
  - `if selected.is_none_or(|selected| selected == locale) { eprintln!("{}", render(dashboard_labels::labels(locale))); }`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/⌨️controls/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/⌨️controls/🟦️.ts:121` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `const source = "const {PassThrough}=require('node:stream'),{emitKeypressEvents}=require('node:readline'),rows=JSON.parse(process.argv[1]),out=[];(async()=>{for(const row of rows){const input=new PassThrough(),events=[...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🌀️control-plane/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🌀️control-plane/🟦️.ts:11` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `const source="const {PassThrough}=require('node:stream'),{emitKeypressEvents}=require('node:readline'),fixture=JSON.parse(process.argv[1]),input=new PassThrough(),events=[];emitKeypressEvents(input);input.on('keypress...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🔎️source-discovery/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🔎️source-discovery/🟦️.ts:52` [console.log] test-path: yes; allocates: no
  - `const run = spawnSync(binary, [], { encoding: "utf8", timeout: 120_000, maxBuffer: 65_536 }); expect(run.status, run.stderr).toBe(0); console.log(run.stdout.trim());`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🔎️source-discovery/🟦️.ts:128` [console.log] test-path: yes; allocates: no
  - `expect(run.status, run.stderr).toBe(0); console.log(run.stdout.trim());`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🗺️coverage/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🗺️coverage/🟦️.ts:153` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `if (kinds.size) console.log('[coverage] differences by kind ${JSON.stringify(Object.fromEntries([...kinds].sort((a, b) => b[1] - a[1]).slice(0, 15)))}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🗺️coverage/🟦️.ts:154` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('[coverage] dry-ran ${ids.length} of ${wanted.size} ids (${special.length} declared, ${ids.length - special.length} of ${plain.length} plain); accepted equivalences ${JSON.stringify(Object.fromEntries(note...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧊️execution/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧊️execution/🟦️.ts:99` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `writeFileSync(join(root,"daemon"),'import {writeFileSync} from "node:fs";writeFileSync("observed.json",JSON.stringify({arguments:process.argv.slice(2),mode:process.env.SEMIO_BUILD_MODE,envelope:JSON.parse(process.env....`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧫️fixtures/🌀️control-plane/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧫️fixtures/🌀️control-plane/🟦️.ts:9` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('dev listening ${server.port}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧫️fixtures/🌀️control-plane/🟦️.ts:11` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('${command} completed ${process.env.DASHBOARD_TEST_VALUE}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️cli/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️cli/🟦️.ts:42` [console.log] test-path: yes; allocates: no
  - `const server = "const s=Bun.serve({port:Number(process.env.JOURNEY_PORT),fetch:()=>new Response('x')});console.log('listening on http://127.0.0.1:'+s.port);setInterval(()=>{},1000)";`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️cli/🟦️.ts:45` [console.log] test-path: yes; allocates: no
  - `{ id: "hello", command: ["bun", "-e", "console.log('hello from the journey')"] },`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️cli/🟦️.ts:46` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `{ id: "authority", command: ["bun", "-e", "console.log(JSON.stringify(Object.fromEntries(['SEMIO_SCRIPT_PROCESS_INVOCATION','SEMIO_SCRIPT_CAPABILITIES','SEMIO_SCRIPT_POLICY'].map(key=>[key,process.env[key]??null]))))"...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🏋️load/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🏋️load/🟦️.ts:41` [console.log] test-path: yes; allocates: yes: ${, Date.now
  - `console.log('[load] 128 sessions started in ${Date.now() - started} ms');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🏋️load/🟦️.ts:68` [console.log] test-path: yes; allocates: yes: ${, Date.now
  - `console.log('[load] check of ${/(\d+) commands/.exec(check)?.[1]} commands in ${Date.now() - first} ms');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🏋️load/🟦️.ts:80` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('[load] search timings ${JSON.stringify(timings)} ms (budget ${budget})');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🏋️load/🟦️.ts:98` [console.log] test-path: yes; allocates: yes: ${, Date.now, JSON.stringify
  - `console.log('[load] ${(run.stdout.length / 1048576).toFixed(1)} MiB through an attached client in ${Date.now() - began} ms; listing answered in ${JSON.stringify(answers)} ms');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🦀️.rs` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🦀️.rs:548` [console.log] test-path: yes; allocates: no
  - `let mut term = terminal::Terminal::spawn(&workspace::which("bun"), &["-e", "process.stdin.setRawMode(true);process.stdin.on('data',data=>process.stdout.write(data));console.log('ready');"], &ws.root, &ws.env(), 20, 120);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/🦀️.rs` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/🦀️.rs:99` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("[semio] {message}");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/🦀️.rs:121` [eprintln!] test-path: no; allocates: yes: println!
  - `for problem in &problems { eprintln!("{}", problem.render(locale)); }`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/🦀️.rs:122` [eprintln!] test-path: no; allocates: yes: println!, to_string(
  - `if registry.graph_unpublished() { eprintln!("{}", voice.say(|text| text.cli_graph_absent.as_str().to_string())); }`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/🦀️.rs:348` [eprintln!] test-path: no; allocates: yes: println!
  - `if line != last { eprintln!("[semio] {line}"); last = line; }`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/🦀️.rs:365` [eprintln!] test-path: no; allocates: yes: println!, to_string(
  - `if wait_ready && urls.is_empty() { eprintln!("[semio] {}", voice.say(|text| text.cli_no_ready_declared.as_str().to_string())); }`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts:53` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Repo CLI entity kinds refreshed (${source.kinds.length} entries, sha256 ${source.sha256}).');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts:67` [console.log] test-path: no; allocates: no
  - `console.log("Repo CLI entity kinds are fresh.");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🃏️glob/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🃏️glob/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Fixture glob reference: ${fixture.cases.length} vectors verified');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🃏️glob/🟦️.ts:42` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('Ticket retention filesystem reference: ${entries.length} files verified with glob');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📥️installation/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📥️installation/🟦️.ts:69` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `await run("repo",["--help"],environment);publishNativeRepoFileV1(receiptPath,Buffer.from(JSON.stringify({version:1,platform:plan.platform,digest,executable:plan.executable,directory:plan.directory,source,mode:process....`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🏃️process/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🏃️process/🟦️.ts:60` [console.log] test-path: no; allocates: yes: ${
  - `async run(segments:string[]):Promise<void>{parseDeveloperArguments(["setup-debugger",...segments]);try{const result=await setupNativeDebugger(this.repoRoot,this.invocation,this.host);console.log('[developer-tools] ${d...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📜️script.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📜️script.ts:21` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[developer-tools] ${developerMessageV1("running")}: ${command}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📜️script.ts:30` [console.log] test-path: no; allocates: yes: ${
  - `if(segments.length)throw new Error(developerMessageV1("invalid"));console.log('[developer-tools] ${developerMessageV1("inventory")}');const nativePath=await probeNativeDebuggerV1(this.repoRoot,this.invocation,createNa...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📜️script.ts:31` [console.log] test-path: no; allocates: yes: ${, .join(, .map(
  - `for(const row of developerDebuggers(process.platform,program=>program.toLowerCase().includes("lldb")?nativePath??null:runtimeWhichV1(program,{PATH:process.env.PATH??process.env.Path??""}),process.env)){const title=not...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🛣️routes/🧬️original-host/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🛣️routes/🧬️original-host/🧪️tests/🟦️.ts:49` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({puzzleSourceCases:checked,neutralHostCases:fixture.cases.length}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/☁️stage-fleet/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/☁️stage-fleet/🧪️tests/🟦️.ts:65` [console.log] test-path: yes; allocates: no
  - `else console.log("[stage-fleet] no Python found: the independent derivation was not run");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🌐️service/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🌐️service/🟦️.ts:41` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[dev] Port ${preferredPort} is already in use — dev server appears to be running at ${url}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts:70` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Bundling ${options.owner}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/📊️report/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/📊️report/🟦️.ts:17` [console.log] test-path: no; allocates: yes: ${
  - `const result = await measureArtifactRegistry(this.repoRoot, registry, { signal: controller.signal, onProgress: (progress) => console.log('[nx-disk] measured ${progress.files} files; ${progress.path}') });`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/📊️report/🟦️.ts:24` [console.log] test-path: no; allocates: yes: ${, .filter(
  - `for (const entry of result.entries.filter((entry) => entry.present && entry.files > 0)) console.log('[nx-disk] ${entry.owner}: ${allocation(entry.allocated)}; ${entry.path}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/📊️report/🟦️.ts:25` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[nx-disk] ${result.complete ? "Complete" : "Incomplete"}: ${result.totals.files} unique files; ${allocation(result.totals.allocated)}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📜️script.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📜️script.ts:15` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `export function runInactiveRepositoryCacheResetV1(context:Readonly<{root:string;repoRoot:string;invocation:ScriptInvocation;control:ResourceLeaseControl}>,profile:RepositoryCacheResetProfileV1):Promise<void>{readInact...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📜️script.ts:32` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({version:1,kind:"repository-cache-regrow",partition:profile,outcome}));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📜️script.ts:50` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(result));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📜️script.ts:65` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(result));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts:141` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `await requireKernelAbsenceV1(directory,backing,control,check);await requireNativeInactiveCapsuleReadersV1(directory,slots,control);await progress("preflight-complete");console.log(JSON.stringify({version:1,kind:"inact...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/⌛️wait/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/⌛️wait/🟦️.ts:40` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const source='import{acquireQueuedResourceLease}from${JSON.stringify(entry)};const [directory,resource]=process.argv.slice(1);try{const lease=await acquireQueuedResourceLease({directory,resource,mode:"shared",sharedCo...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/📦️child/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/📦️child/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `});}catch(error){failed=true;same=Object.is(error,primary);events.push("scope-failed");}console.log(JSON.stringify({mode,failed,same,completed,settledBeforeRelease,bodyCalled,events}));process.exitCode=failed?1:0;`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts:29` [console.log] test-path: yes; allocates: yes: .join(, .map(, .push(, JSON.stringify
  - `test("neutral loaded observation Source and schema refuse before and after await",()=>repository.invoke(async original=>{const url=new URL("../../../../../../../../🏗️build/👁️observation/🟨️.mjs",import.meta.url),source...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `test("neutral managed Graph actual cold entry imports preserve defining async observation instance",()=>repository.invoke(async original=>{const base=new URL("../../../../../../../../",import.meta.url),db=new Database...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🟦️.ts:55` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `void withRepositoryArtifactV1({workspace:repository.root,invocation:root,storage:store.storage,control:store.control,request:{version:1,path,owner:"fixture:store-reader",maximumBytes:4096,maximumFiles:1}},async token=...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🟦️.ts:76` [console.log] test-path: yes; allocates: yes: .map(, .push(, JSON.stringify
  - `void withRepositoryArtifactGroupV1({workspace:repository.root,invocation:root,storage:store.storage,control:store.control,requests},async token=>{expect(order).toContain("fixture-body-return");order.push("group-body")...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📂️parents/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📂️parents/🧪️tests/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: .join(, .map(, JSON.stringify
  - `const output=await captureOwnedCommandV1("node",["--eval","const fs=require('node:fs'),path=require('node:path'),root=process.argv[1];const rows=['.cache','.cache/b'].map(name=>{try{const s=fs.lstatSync(path.join(root...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts` (21)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:39` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `function appendPublicationStageV1(samples:StageSample[],id:RepositoryCachePartitionIdV1,stage:string,started:number,observation:StageObservation|null,refusal:string|null):void{const protocol=schema.definitions.StagePr...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:43` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `function persistPublicationStagesV1(id:RepositoryCachePartitionIdV1,samples:readonly StageSample[],origin:"canonical-callback"|"generated-source-model"):void{const trace={version:1,origin,scope:"declared macOS publica...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:49` [console.log] test-path: yes; allocates: yes: .repeat(, JSON.stringify
  - `async function identityModel(control:ResourceLeaseControl):Promise<void>{const require=createRequire(join(root,"package.json")),Ajv=require("ajv"),validate=new Ajv({strict:true}).compile(schema.definitions.IdentityCas...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:61` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `async function publish(recorded:Census,id:RepositoryCachePartitionIdV1,original:routing.ScriptInvocation,control:ResourceLeaseControl,purpose:"current-publication"|"current-prepared-publication"="current-publication")...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:63` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const request=readNativeImagePublicationRequestV1({version:1,purpose,initialState:last.mounted?"mounted":"inactive",receiptSha256:last.receipt.sha256,directory:{dev:last.folder.dev,ino:last.folder.ino},backing:{dev:la...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:65` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `for(const guard of[target,bootstrap])if(guard){if(uncertain)retained.add(guard);else try{guard.release();}catch(error){unsafe();retained.add(guard);errors.push(error);}}if(leases){if(uncertain)retained.add(leases);els...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:67` [console.log] test-path: yes; allocates: yes: .map(, .push(, JSON.stringify
  - `async function model(control:ResourceLeaseControl,original:routing.ScriptInvocation):Promise<void>{const require=createRequire(join(root,"package.json")),Ajv=require("ajv"),oracle=new(await import("bun:sqlite")).Datab...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:69` [console.log] test-path: yes; allocates: yes: .filter(, .push(, .repeat(, JSON.stringify
  - `async function logicalEntryDiagnosticModelV1(control:ResourceLeaseControl):Promise<void>{const require=createRequire(join(root,"package.json")),Ajv=require("ajv"),db=new(await import("bun:sqlite")).Database(":memory:"...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:72` [console.log] test-path: yes; allocates: yes: .map(, .push(, .repeat(, JSON.stringify
  - `function freshAssetSnapshotModelV1():void{const require=createRequire(join(root,"package.json")),Ajv=require("ajv"),ajv=new Ajv({strict:true}),db=new(require("bun:sqlite").Database)(":memory:"),directory=join(output,"...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:75` [console.log] test-path: yes; allocates: yes: .push(, .repeat(, JSON.stringify
  - `function diagnosticModel():void{const require=createRequire(join(root,"package.json")),Ajv=require("ajv"),validate=new Ajv({strict:true}).compile(schema.definitions.DiagnosticCase),db=new(require("bun:sqlite").Databas...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:79` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const fs=await import("node:fs"),path=join(output,"stage-receipt-"+process.pid),candidate=path+".candidate",samples:StageSample[]=[],started=performance.now(),payload=Buffer.from("Source receipt stage DATA\n");try{fs....`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:81` [console.log] test-path: yes; allocates: yes: JSON.stringify, new Set
  - `async function readerModel(control:ResourceLeaseControl):Promise<void>{const require=createRequire(join(root,"package.json")),ts:typeof import("typescript")=require("typescript"),fs:typeof import("node:fs")=require("n...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:84` [console.log] test-path: yes; allocates: no
  - `if(process.platform==="win32"){console.log("[TRACE] Same-host POSIX directory sync experiment is outside Windows scope");return;}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:88` [console.log] test-path: yes; allocates: yes: .map(, .push(, JSON.stringify
  - `const observe=(async()=>{for(let n=0;n<4;n++){checkResourceLeaseControl(control);if(n===1){fs.chmodSync(paths[1]!,0o400);fs.chmodSync(paths[1]!,0o600);}rows.push(paths.map(describe));if(n<3){await control.publish({res...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:94` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `async function ordinaryWorkerV1():Promise<void>{await routing.receiveScriptProcessInvocation(process.env,async original=>{const before=routing.readIssuedScriptProcessEnvelopeV1(original),phase=routing.createIssuedReso...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:100` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .map(, .push(
  - `async function preparedPublicationModelV1(original:routing.ScriptInvocation,control:ResourceLeaseControl):Promise<void>{const require=createRequire(join(root,"package.json")),Ajv=require("ajv"),ts=(await import("types...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:108` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const launchArgvVectors=await ordinaryLaunchModelV1(control),finiteBody=routing.createRequestedIssuedResourceControlV1(control,protocol.finiteParentMilliseconds);await routing.withAcceptedPhysicalCleanupV1(finiteBody,...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:112` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `async function root0FailureWorkerV1(row:typeof root0FailureCases[number]):Promise<void>{const{withNativeOpenLifetimeV1}=await import("../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:114` [console.log] test-path: yes; allocates: yes: .filter(, .push(, Date.now, JSON.stringify
  - `async function root0FailureModelV1(original:routing.ScriptInvocation,control:ResourceLeaseControl):Promise<void>{const require=createRequire(join(root,"package.json")),Ajv=require("ajv"),ajv=new Ajv({strict:true}),db=...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:118` [console.log] test-path: yes; allocates: yes: .join(, .push(, JSON.stringify
  - `try{await fs.mkdir(rootPath);const real=lstatSync(rootPath,{bigint:true});try{await routing.receiveScriptProcessInvocation(process.env,async original=>{birth=routing.readIssuedScriptProcessEnvelopeV1(original);assert(...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️asset-publication/📜️script.ts:125` [console.log] test-path: yes; allocates: yes: .map(, Date.now, JSON.stringify
  - `await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope(policy,capabilities,Date.now()),async original=>{if(entry==="--retained-inactive-source"||entry==="--retained-inactive-strict"||entry==="--re...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️graph-purpose/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/👷️graph-purpose/📜️script.ts:67` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `async function runGraphMaintenancePhysicalV1(entry:ReturnType<typeof readGraphMaintenanceEntryV1>,original:ScriptInvocation,control:ResourceLeaseControl){if(entry.mode==="--maintain-prepared"&&process.platform!=="darw...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/📐️layout/🍎️physical/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/📐️layout/🍎️physical/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const writer=await open(),nextPolicy=policy(writer,fixture.currentBytes),changed=await writer.withExclusiveMaintenance({version:1,wait:false},async(port,custody)=>{if(custody.kind!=="kernel-ex")throw Error("Actual pri...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🟦️.ts:29` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `mkdirSync(directory);const lease=await acquireResourceLease({directory,resource:"sqlite-page-cap-oracle",mode:"exclusive",signal:control.signal,control,maximumDatabaseBytes:16384});lease.release();const database=resou...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🟦️.ts:38` [console.log] test-path: no; allocates: yes: ${, .join(, JSON.stringify
  - `const authorityProgramV1='const fs=require('node:fs'),workerData=JSON.parse(process.argv[1]),file=fs.openSync(workerData.path,fs.constants.O_RDONLY|(fs.constants.O_NOFOLLOW||0)),stamp=row=>[row.dev,row.ino,row.size,ro...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🔀️selected-state-recovery/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🔀️selected-state-recovery/🟦️.ts:32` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const probe="const{DatabaseSync}=require('node:sqlite'),db=new DatabaseSync(process.argv[1]);let blocked=false;try{db.exec('pragma busy_timeout=0;begin exclusive;rollback');}catch(e){if(e.code!=='ERR_SQLITE_ERROR'||!S...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🔁️revalidation/🍎️withdrawn-cleanup-recheck/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🔁️revalidation/🍎️withdrawn-cleanup-recheck/🟦️.ts:36` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const probe="const{DatabaseSync}=require('node:sqlite'),db=new DatabaseSync(process.argv[1]);let blocked=false;try{db.exec('pragma busy_timeout=0;begin exclusive;rollback');}catch(e){if(e.code!=='ERR_SQLITE_ERROR'||!S...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🟦️.ts:68` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(scenario==="reader"||scenario==="queued"||scenario==="queued-cancel"||scenario==="optional-queued"){reader=await openNativeImageStorageV1(options);const probe="const{DatabaseSync}=require('node:sqlite'),db=new Data...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🟦️.ts:63` [console.log] test-path: yes; allocates: no
  - `console.log("[members] proof of "+physical.length+" leaves from "+(text.length/1048576).toFixed(1)+" MiB of index in "+elapsed.toFixed(0)+" ms");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🧱️resolve/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/🧱️resolve/🟦️.ts:106` [console.log] test-path: yes; allocates: no
  - `console.log("[members] resolved "+leaves.length+" leaves in "+elapsed.toFixed(0)+" ms");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🟦️.ts:32` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `else test("isolated closure worker retains its first refusal",async()=>receiveScriptProcessInvocation(process.env,async original=>{await verifyAdmittedNegative(workerCase,createIssuedResourceControlV1(original,"test:i...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🟦️.ts:13` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .map(, JSON.stringify
  - `export async function runAdmittedClosureWorkerTestV1(request:unknown,control:ResourceLeaseControl){if(validateJsonSchemaSubset(schema,request,schema).length)throw Error("Closed admitted worker fixture request required...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts` (7)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts:63` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .map(, .push(
  - `const row=corpus.maintenanceNestedCases.find(row=>row.mode===maintenanceNestedCase);if(!row)throw Error("Closed nested Source case required");const fs=await import("node:fs/promises"),physical=await import("node:fs"),...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts:86` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const provider=async(options:{directory:string;control:typeof phase;maximumPhysicalBytes:number},_value:unknown,owner:Parameters<typeof borrower.withNativeImageBorrowerOperationV1>[0],proof:Parameters<typeof leases.re...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts:183` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .map(, .repeat(
  - `try{await fs.mkdir(directory);const payloadRoot=contract.nativeImageRootV1(directory);await fs.mkdir(payloadRoot);await fs.mkdir(leaseDirectory,{recursive:true});await fs.writeFile(join(payloadRoot,"storage-events.jso...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts:213` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `}catch(error){failed=true;failure=error;}finally{held?.release();await fs.rm(base,{recursive:true,force:true});}if(row.effect!=="ordinary")expect(parentRetired).toBe(false);if(row.effect==="close")expect(retained.size...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts:297` [console.log] test-path: yes; allocates: yes: .join(, .repeat(, JSON.stringify
  - `try{try{await api.withNativePlatformProtectedMaintenanceParentV1!({directory,leaseDirectory,filesystemBytes:33554432,maximumPhysicalBytes:34603008,maximumLeaseDatabaseBytes:16384,control:phase},corpus.maintenanceReque...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts:307` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `async function emitArtifactMaintenanceSourceReceiptV1(original:Parameters<Parameters<typeof withBuildTestInvocationV1>[0]>[0],mode:string,failed:boolean,reads:number):Promise<void>{const descriptor=await import("@semi...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts:332` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .map(, .repeat(
  - `try{try{await api.withNativePlatformProtectedMaintenanceParentV1!({directory,leaseDirectory,filesystemBytes:33554432,maximumPhysicalBytes:34603008,maximumLeaseDatabaseBytes:16384,control:phase},corpus.maintenanceReque...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts:338` [[DEBUG]] test-path: no; allocates: yes: .filter(, .map(, new Set
  - `if(selection.kind==="products-graph-fixture"){if(!row.producer||!row.fixture)throw Error("Current exact fixture and known producer selector required");stable(row.producer,schema.definitions.CurrentMaintenanceProtocol....`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts:60` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await issuance.receiveScriptProcessInvocation(process.env,async root=>{const control=issuance.createIssuedResourceControlV1(root,"fixture:graph-root-child",90000),directory=join(repository.root,".cache/b/l/m/data"),im...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/📜️script.ts:13` [console.log] test-path: yes; allocates: yes: .map(, .repeat(, JSON.stringify
  - `});},()=>rm(root,{recursive:true,force:true}));}else{const root=await mkdtemp(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"wc-"));await withPurposeCleanupV1(async()=>{await withNxGraphFixtureV1(root,original,async(direc...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts:29` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `try{expect(schema.definitions.NativePolicy.const.maximumPhysicalBytes).toBe(row.maximumPhysicalBytes);expect(typeof owner.nxGraphNativeDirectoryV1).toBe("function");expect(typeof platform.retireNativePlatformStorageV1...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const probe=await captureOwnedCommandResultV1("node",["--input-type=module","--eval",'import{readFileSync,statfsSync,lstatSync}from"node:fs";import{createHash}from"node:crypto";const root=process.argv[1],file=process....`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts:56` [console.log] test-path: yes; allocates: no
  - `await withGraphPublicationLeaseV1(directory,fixture.native.namespace.toolingIdentity,store.control,async()=>owner.withNxGraphPoolAdmissionV1(original,store.storage,store.control,store.recheck,async()=>owner.withNxGrap...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts:66` [console.log] test-path: yes; allocates: yes: .repeat(
  - `const row=fixture.disabledMarker;expect(typeof(owner as any).nxGraphDaemonDisabledMarkerV1).toBe("function");const encode=(owner as any).nxGraphDaemonDisabledMarkerV1 as(value:unknown)=>Uint8Array;for(const text of[ro...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts:86` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const first=fixture.currentMaintenanceIntents[0]!;let getters=0;expect(()=>read(first.root,Object.defineProperty({},"selection",{enumerable:true,get(){getters++;return first.input.selection;}}))).toThrow();expect(gett...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts:118` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const validate=new(require('ajv').default)({strict:true}).addSchema(schema).compile({$ref:schema.$id+'#/definitions/CurrentMaintenanceFile'});try{for(const[index,row]of fixture.currentMaintenanceFiles.entries()){const...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/📥️purpose/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/📥️purpose/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(mode!=="actual"&&mode!=="ledger-model")throw Error("Closed output negative purpose mode required");checkScriptInvocation(original);console.log(ackPrefix+JSON.stringify(acknowledgement(original,mode)));if(mode==="le...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/📥️purpose/🟦️.ts:31` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `});}catch(error){primary={error};throw error;}finally{try{await store.recheck();const current=lstatSync(requireNativeStoragePathV1(store.root,output),{bigint:true});if(!current.isDirectory()||current.isSymbolicLink()|...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/📥️purpose/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const run=new Function(...Object.keys(ports),javascript+";return withOutputCustodyV1;")(...Object.values(ports));try{await run(scope,["/model/products/m/output"],"shared",async()=>{facts.waitEntered=true;});}catch(err...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts:53` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `const program='const vm=require('node:vm'),row=JSON.parse(process.argv[1]),sources=JSON.parse(process.argv[2]),events=[];let active=true,directory=row.directories[0],checks=0;const environment={},port={requireNxGraphN...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts:59` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `const program='const vm=require('node:vm'),main=process.argv[1],entryArguments=JSON.parse(process.argv[2]),events=[],environment={SEMIO_NX_NATIVE_SOURCE_SNAPSHOT:"selected"},cause=Error('Original entry refusal');let a...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts:66` [console.log] test-path: yes; allocates: yes: .join(, .map(, .push(, .repeat(
  - `const program=String.raw'const vm=require('node:vm'),path=require('node:path'),row=JSON.parse(process.argv[1]),source=JSON.parse(process.argv[2]),writes=[],environment={};let active=true;const port={requireNxGraphDiag...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/📋️orchestration/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/📋️orchestration/🟦️.ts:16` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[nx-audit] projects=${result.projects.length} commands=${result.commands.length} artifacts=${result.artifacts.length} violations=${result.violations.length}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/📋️orchestration/🟦️.ts:25` [console.log] test-path: no; allocates: no
  - `console.log("[nx-policy] all target and command contracts passed");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏃️contract-capture/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏃️contract-capture/🟦️.ts:41` [console.log] test-path: no; allocates: yes: ${
  - `const stopProgress = startNativeProgress('artifact-package-contract:${command}', 10_000, line => console.log(line));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts:59` [console.log] test-path: no; allocates: no
  - `log: (line) => console.log(line),`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🐳️devcontainer-context/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🐳️devcontainer-context/🟦️.ts:18` [console.log] test-path: yes; allocates: no
  - `console.log("✅️ Devcontainer build uses only its environment directory and excludes all ordinary context files; YAML/minimatch oracle PASS");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🟦️.ts:128` [console.log] test-path: yes; allocates: no
  - `console.log("✅️ Devcontainer lifecycle: fontconfig (jsdom XML), tool selection (lodash), dashboard-only attach, safe.directory, Claude auth storage, start without destructive commands, IDE controls retired PASS");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🟦️.ts:165` [console.log] test-path: yes; allocates: no
  - `console.log("✅️ Devcontainer dashboard ownership: retired controls absent; attach installs no automatic Git hooks or editor controls (lodash/SQLite oracles) PASS");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/🧪️tests/📥️physical/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/🧪️tests/📥️physical/🟦️.ts:16` [console.log] test-path: yes; allocates: no
  - `await publish(7);let entered!:()=>void,release!:()=>void,escaped!:RepositoryArtifactReaderV1,context:RepositoryArtifactV1|undefined,settled=false;const ready=new Promise<void>(done=>{entered=done;}),gate=new Promise<v...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts:34` [console.log] test-path: yes; allocates: no
  - `await withBuildTestInvocationV1(original=>fixture(original,async(storage,control,workspace,_image,_directory,input)=>{const{bundleJavaScript}=await import("../../../../📦️bundle/🟦️.ts"),source=join(input,"entry.ts"),ou...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts:40` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await withBuildTestInvocationV1(async original=>{const workspace=repository.root,control=scriptCompilerAdmissionControlV1(original,"artifact:original-cancel-child",90000),options={directory:data.directory,leaseDirecto...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts:49` [console.log] test-path: yes; allocates: no
  - `await withBuildTestInvocationV1(original=>fixture(original,async(storage,control,workspace,_image,_directory,input)=>{const{bundleJavaScript}=await import("../../../../📦️bundle/🟦️.ts"),mode=process.env.SEMIO_BUILD_MOD...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts:54` [console.log] test-path: yes; allocates: yes: new Map
  - `await withBuildTestInvocationV1(original=>fixture(original,async(storage,control,workspace,_image,_directory,input)=>{const{stageRepositoryArtifacts}=await import("../../../🟦️.ts"),source=join(input,"publication-sourc...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts:59` [console.log] test-path: yes; allocates: yes: .filter(, new Map
  - `await withBuildTestInvocationV1(original=>fixture(original,async(storage,control,workspace,_image,_directory,input)=>{const{stageRepositoryArtifacts,withRepositoryArtifactDestinationV1}=await import("../../../🟦️.ts"),...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts:69` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify, new Map
  - `await repository.invoke(original=>fixture(original,async(storage,control,workspace,_image,_directory,input)=>{const{realpathSync}=await import("node:fs"),binding=await import("../../../🟦️typescript/🔗️binding/🟦️.ts"),p...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts:85` [console.log] test-path: no; allocates: no
  - `console.log("[cargo:locked-metadata] dependency lock validated");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const child = 'await Bun.write(process.env.SEMIO_OWNER_POLICY_RECEIPT, JSON.stringify({policy: process.env.SEMIO_CARGO_TEST_POLICY ? JSON.parse(process.env.SEMIO_CARGO_TEST_POLICY) : null, context: JSON.parse(process....`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts:71` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[native-owner-command-policy] ${row.id}: ${row.policy}; oracle=${classification}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts:89` [console.log] test-path: yes; allocates: yes: ${
  - `const compiled=await build({stdin:{contents:'setTimeout(()=>console.log("interactive-ready"),${corpus.durationMs});',loader:"js"},write:false,platform:"node",format:"esm"});`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts:140` [console.log] test-path: yes; allocates: no
  - `const manifest = row.manifest!, cwd = row.cwd!, requestedManifest = relative(root, join(output, "unacquired", "Cargo.toml")), code = 'console.log("foreign acquisition occurred")';`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts:141` [console.log] test-path: yes; allocates: no
  - `const wire = ownerInvocation(manifest, cwd, process.execPath, ["--eval", 'console.log("original command")'], output, 15000, started,"prototype");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts:142` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const oracle = Bun.spawnSync([registeredToolExecutableV1("node"), "--eval", 'console.log(JSON.stringify(JSON.stringify(JSON.parse(process.argv[1]).capabilities.command.arguments) === JSON.stringify(["--eval",process.a...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `const oracle=Bun.spawnSync([registeredToolExecutableV1("node"),"--eval",'const cases=${JSON.stringify(fixture.cases)};console.log(JSON.stringify(cases.map(row=>row.route==='owner-command'?(row.consume?['owned','prepar...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts:86` [console.log] test-path: yes; allocates: yes: .join(, .map(, JSON.stringify
  - `const oracle=Bun.spawnSync([registeredToolExecutableV1("node"),"--eval",'const fs=require('node:fs'),path=require('node:path');const root=process.argv[1],rows=JSON.parse(process.argv[2]);console.log(JSON.stringify(row...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts:125` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const oracle=Bun.spawnSync([registeredToolExecutableV1("node"),"--eval",'console.log(JSON.stringify(JSON.parse(process.argv[1]).map(row=>row.command==='cargo'?(row.refused?['plan','prepare']:['plan','prepare','lease',...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/🗺️owner-test-manifests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/🗺️owner-test-manifests/🟦️.ts:34` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `writeFileSync(providerFile,'const cargo=await import(${JSON.stringify(cargo)});const emit=value=>console.log("[TRACE] route-receipt="+JSON.stringify(value));export const provider={runTests:async(request,policy)=>emit(...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/🗺️owner-test-manifests/🟦️.ts:36` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code='const{mock}=await import("bun:test");const cargo=await import(${JSON.stringify(cargo)});const emit=value=>console.log("[TRACE] route-receipt="+JSON.stringify(value));const planning=await import(${JSON.stri...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🍎️image/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🍎️image/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const source='(async()=>{const fs=require('node:fs'),path=require('node:path'),sdk=require('node:module').createRequire(process.env.SEMIO_TEST_SDK_PACKAGE),{DbCache}=sdk('nx/src/tasks-runner/cache'),native=sdk('nx/src...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🍎️image/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .push(, JSON.stringify
  - `const refusalSource='(async()=>{const fs=require('node:fs'),path=require('node:path'),sdk=require('node:module').createRequire(process.env.SEMIO_TEST_SDK_PACKAGE),{DbCache}=sdk('nx/src/tasks-runner/cache'),frozen=awai...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🍎️image/🟦️.ts:42` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const source='(async()=>{const fs=require('node:fs'),path=require('node:path'),sdk=require('node:module').createRequire(process.env.SEMIO_TEST_SDK_PACKAGE),{DbCache}=sdk('nx/src/tasks-runner/cache'),native=sdk('nx/src...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🍎️image/🟦️.ts:55` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const source="const fs=require('node:fs'),path=require('node:path'),sdk=require('node:module').createRequire(process.env.SEMIO_TEST_SDK_PACKAGE),native=sdk('nx/src/native'),workspace=process.env.SEMIO_TEST_BOUND_WORKS...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🍎️image/🟦️.ts:74` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .push(, JSON.stringify
  - `const source='(async()=>{const fs=require('node:fs'),path=require('node:path'),sdk=require('node:module').createRequire(process.env.SEMIO_TEST_SDK_PACKAGE),{DbCache}=sdk('nx/src/tasks-runner/cache'),native=sdk('nx/src...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🟦️.ts:78` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const source='try{require(${JSON.stringify(sdk.resolve("nx/src/utils/cache-directory"))});console.log(JSON.stringify({refused:false}));}catch(error){console.log(JSON.stringify({refused:String(error).includes('Original...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🧪️tests/🧭️commands/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🧪️tests/🧭️commands/📜️script.ts:8` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `run(args:string[]):void{console.log(JSON.stringify({selected:process.argv[2],args}));}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:66` [console.log] test-path: no; allocates: yes: ${
  - `for (const twin of options.twins ?? []) console.log('[TRACE] ${twin.name}-twin checks=${twin.run()}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:72` [console.log] test-path: no; allocates: yes: ${
  - `for (const twin of options.twins ?? []) console.log('artifact-contribution: ${twin.name} checks=${twin.run()}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔁️verification/📋️orchestration/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔁️verification/📋️orchestration/🟦️.ts:70` [console.log] test-path: no; allocates: no
  - `writeFileSync("dist/consumer.cjs", "#!/usr/bin/env node\\nconsole.log(" + source.value + ");\\n");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔁️verification/📋️orchestration/🟦️.ts:73` [console.log] test-path: no; allocates: no
  - `console.log("[cache-probe] " + command + " executed " + count);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔁️verification/📋️orchestration/🟦️.ts:99` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[cache-verify] ${scenario}: builds=${count("build")} tests=${count("test")}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧪️tests/🔏️receipt/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧪️tests/🔏️receipt/🟦️.ts:56` [console.log] test-path: yes; allocates: no
  - `console.log("🔏️ Digest publication matches stable JSON/Ajv, preserves unchanged bytes and mtime, rejects invalid writes, re-digests on every run and keys its cached consumers PASS");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧪️tests/🔬️script-commands/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔏️inputs/🧪️tests/🔬️script-commands/🟦️.ts:29` [console.log] test-path: yes; allocates: no
  - `console.log(digest);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/✅️verification/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/✅️verification/🟦️.ts:32` [console.log] test-path: no; allocates: yes: ${
  - `checkResourceLeaseControl(actual);console.log('[build-graph] projects=${Object.keys(graph.projects).length} edges=${graph.dependencies.length}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚦️service/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚦️service/🧪️tests/🟦️.ts:42` [console.log] test-path: yes; allocates: yes: .filter(, .join(
  - `const closed=new Promise<void>(accept=>child!.on("message",message=>{if(message&&typeof message==="object"&&"type"in message&&message.type==="SOURCE_DAEMON_SERVER_CLOSED")accept();}));child!.send({type:"stop"});await ...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚦️service/🧪️tests/🧪️tests/🔬️script-commands/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚦️service/🧪️tests/🧪️tests/🔬️script-commands/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(sdkRequire("nx/src/daemon/socket-utils").getFullOsSocketPath()!==socketPath)throw new Error("Actual installed Nx socket must equal held funded namespace");console.log("[TRACE] Actual installed Nx socket utility sel...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚦️service/🧪️tests/🧪️tests/🔬️script-commands/🟦️.ts:17` [console.log] test-path: yes; allocates: no
  - `process.on("message",message=>{if(!message||typeof message!=="object"||!("type"in message))return;if(message.type==="worker-stop")finishWorker();if(message.type==="stop")void withNxDaemonRequestV1(async()=>{requestNxD...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚦️service/🧪️tests/🧪️tests/🔬️script-commands/🟦️.ts:20` [console.log] test-path: yes; allocates: no
  - `console.log("[TRACE] Actual Node service callback returns while its original server remains live");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏇️waiter/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏇️waiter/🧪️tests/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `for(const row of rows){fs.rmSync(cache.nxProjectGraph,{force:true});if(row.present){fs.mkdirSync(require('node:path').dirname(cache.nxProjectGraph),{recursive:true});fs.writeFileSync(cache.nxProjectGraph,JSON.stringif...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/🧪️tests/📦️transaction/🧪️tests/🔬️script-commands/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/🧪️tests/📦️transaction/🧪️tests/🔬️script-commands/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: .map(, .push(, JSON.stringify
  - `await withGraphPublicationLeaseV1(root,tooling,rowControl,async token=>{(fs as any).openSync=function(path:any,...args:any[]){const fd=open(path,...args);files.set(fd,String(path));return fd;};(fs as any).writeSync=fu...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/🧪️tests/📦️transaction/🧪️tests/🔬️script-commands/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `(fs as any).openSync=function(path:any,...args:any[]){const fd=open(path,...args);files.set(fd,String(path));return fd;};(fs as any).writeSync=function(fd:number,...args:any[]){if(files.get(fd)?.includes("publication....`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/🧪️tests/📦️transaction/🧪️tests/🔬️script-commands/🟦️.ts:29` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `(fs as any).openSync=function(path:any,...args:any[]){const fd=open(path,...args);files.set(fd,String(path));return fd;};(fs as any).closeSync=function(fd:number){const path=files.get(fd);files.delete(fd);const result...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/🧪️tests/📦️transaction/🧪️tests/🔬️script-commands/🟦️.ts:30` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(mode==="foreign"){const target=join(directory,"project-graph.json"),foreign=join(directory,fixture.foreignLink),before=fs.readFileSync(target,"utf8");fs.linkSync(target,foreign);let refused=false;try{await publishG...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/🧪️tests/📦️transaction/🧪️tests/🔬️script-commands/🟦️.ts:31` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `if(mode==="recover"){recoverGraphPublicationV1(lease);console.log(JSON.stringify({files:fixture.files.map(row=>JSON.parse(fs.readFileSync(join(directory,row.name),"utf8"))),journal:fs.existsSync(join(directory,"public...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/🧪️tests/📦️transaction/🧪️tests/🔬️script-commands/🟦️.ts:34` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `try{await publishGraphTransactionV1(lease,admittedFiles.map(row=>({...row,value:mode==="seed"?{accepted:row.name}:row.value})),()=>true);const snapshot=readGraphPublicationSnapshotV1(directory,fixture.files.map(row=>r...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code='const routing=await import(${JSON.stringify(new URL("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",import.meta.url).href)}),capacity=await import(${JSON.stringi...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts:65` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `try{for(const row of controlCorpus.refusals){const code='const routing=await import(${JSON.stringify(new URL("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",import.meta.url)...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts:132` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `try{const code='const routing=await import(${JSON.stringify(new URL("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",import.meta.url).href)}),capacity=await import(${JSON.str...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts:50` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .map(, JSON.stringify
  - `async function actualWatcherCaseV1(kind:"empty"|"directory"):Promise<void>{return withBuildTestInvocationV1(original=>withRepositoryNxGraphNativeStorageV1(repository.root,original,scriptCompilerAdmissionControlV1(orig...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts:67` [console.log] test-path: yes; allocates: yes: .filter(, .map(, .push(, Date.now
  - `const received=[],watcher=sdk('nx/src/daemon/server/watcher'),nativeWatcher=await watcher.watchWorkspace({},(error,events)=>{if(error)throw error;received.push(...events);graph.scheduleProjectGraphRecomputation(events...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/📥️membership/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/📥️membership/🟦️.ts:21` [console.log] test-path: no; allocates: yes: ${
  - `const finish=(error?:unknown)=>{if(settled)return;settled=true;if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1")console.log('[nx] Git membership ${args.includes("check-ignore")?"ignore-filter":args.includes("--deleted"...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/📥️membership/🟦️.ts:31` [console.log] test-path: no; allocates: yes: ${
  - `const digest=createHash("sha256");for(const file of files)digest.update(file+"\0");const identity=digest.digest("hex");if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1")console.log('[nx] Graph membership files=${files.l...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts:283` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[dashboard] Rebuilding ${options.mode} because ${reason}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts:465` [console.log] test-path: no; allocates: yes: ${
  - `if(env.NX_DAEMON===undefined&&!invocation.watch&&invocation.args[0]!=="watch"){const admission=await import("../🕸️graph/🏁️startup/🚏️admission/🟦️.ts").then(owner=>owner.nxDaemonAdmissionV1(this.root,env,actualPhase.sig...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📥️admission/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📥️admission/🟦️.ts:100` [console.log] test-path: no; allocates: yes: ${
  - `if (++inspected % 1024 === 0) console.log('[nx] Admitting structural source membership (${inspected} directories)');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts:57` [console.log] test-path: no; allocates: no
  - `if(channel===undefined)await install();else await fleetCargoToolV1(fleetFetchContextV1(process.env,workspace),{crate,version,channel,home:boundedEnvironment.CARGO_HOME??join(homedir(),".cargo")},install,{verify:ready,...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts:70` [console.log] test-path: no; allocates: no
  - `const uvFleet = fleetUvPortV1(fleetFetchContextV1(process.env, workspace), readFileSync(join(workspace, plan.root, "uv.lock"), "utf8"), line => console.log(line));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts:80` [console.log] test-path: no; allocates: no
  - `console.log(base === undefined ? setupTextV1("goUpstream", {}) : setupTextV1("goMirror", { base }));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts:86` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[deps-dotnet] Pinned .NET SDK ${sdk} is ready');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts:88` [console.log] test-path: no; allocates: no
  - `else if (kind === "cpp") console.log("[deps-cpp] Build-tool native tooling prerequisite completed");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts:100` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[deps-${kind}] Pinned tools and Rust WebAssembly target are ready');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts:135` [console.log] test-path: yes; allocates: yes: .push(, .repeat(, JSON.stringify
  - `for(const row of dependencyDiagnosticCasesV1.outcomes)test.skipIf(process.env.SEMIO_DEPENDENCY_PLAN_SOURCE_CASE!==row.mode)("dependency current plan diagnostic worker "+row.mode,()=>withBuildTestInvocationV1(async ori...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts:177` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `const path=join(process.cwd(),".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/tooling-diagnostic/📜️script.ts"),text=readFileSync(path,"utf8"),tree=ts.createSourceFile(path,text,ts.Scri...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🔗️pair/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🔗️pair/🟦️.ts:72` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await descriptor.withArtifactRelativeDirectoryV1(root,check,async directory=>{const entry=await directory.entry("link");expect(entry?.kind).toBe("link");const publicTarget=fs.readlinkSync(join(root,"link")),python=spa...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `test("dependency Source owner exports preserves authored conditions and provider-only scope",async()=>withBuildTestInvocationV1(async original=>{const ts=await import("typescript"),util=await import("node:util"),modul...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts:143` [console.log] test-path: yes; allocates: yes: .push(, .repeat(, Array.from, JSON.stringify
  - `for(const row of producerDiagnosticFixtureV1.producerOutcomes)test.skipIf(process.env.SEMIO_DEPENDENCY_PLAN_SOURCE_CASE!=="producer-"+row.mode)("dependency producer plan diagnostic worker "+row.mode,()=>withBuildTestI...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts:171` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `for(const mode of["success","undefined","frozen"])test.skipIf(process.env.SEMIO_DEPENDENCY_PLAN_SOURCE_CASE!=="probe-scope-"+mode)("dependency producer fixed probe scope worker "+mode,()=>withBuildTestInvocationV1(asy...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts:36` [console.log] test-path: no; allocates: yes: ${
  - `const progress = setInterval(() => (capture ? console.error : console.log)('${command} ${args[0]} is still running…'), 10000);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts` (7)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts:58` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const oracleRoot=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"q")),entry='import {mkdirSync,writeFileSync,renameSync,rmSync} from "node:fs";import {join,dirname} from "node:path";import {createRequire} from ...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts:103` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(events.length!==1)throw Error('startup blocked');watcher.stdout.emit('data',Buffer.from('watch process waiting...'));await new Promise(setImmediate);if(events.length!==1)throw Error('overlapping initial preparation...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts:165` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `writeFileSync(executable, "console.log(JSON.stringify({ready:true}))");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts:191` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `const reference="import {EventEmitter} from 'node:events';import {waitForNxTargetStart} from "+JSON.stringify(pathToFileURL(module).href)+";const child=Object.assign(new EventEmitter(),{stdout:new EventEmitter()}),sta...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts:208` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `const reference="import {startNxWatcherAfterPreparation} from "+JSON.stringify(pathToFileURL(module).href)+";import {ChildProcess} from 'node:child_process';let complete;const registered=new ChildProcess(),prepared=ne...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts:220` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const oracle=Bun.spawn([registeredToolExecutableV1("node"),"--input-type=module","-e",'import {pinnedNxRuntimeEnvironment} from '+JSON.stringify(pathToFileURL(module).href)+';console.log(JSON.stringify('+JSON.stringif...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts:271` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const child=Bun.spawn([registeredToolExecutableV1("node"),"--input-type=module","-e",'import{posix}from"node:path";console.log(JSON.stringify('+JSON.stringify(rows.cases)+'.map(row=>posix.dirname(row.modulePath))));']...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts:267` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `let scopes:unknown;await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>{scopes=custody.scopes;});console.log(JSON.stringify({membership:{...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟦️.ts:21` [console.log] test-path: no; allocates: yes: ${
  - `if(watch)void watch.ready.then(address=>console.log('${readinessMarkerV1} ${address}'));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/🟦️.ts:59` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const result:{success:boolean;terminalOutput:string}=await api.default({command:'node -e "console.log(JSON.stringify(JSON.parse(process.env.SEMIO_OWNER_ARGUMENTS).arguments))"',cwd:".",usePty:false,streamOutput:false,...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧊️engine-references/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧊️engine-references/🧪️tests/🟦️.ts:54` [console.log] test-path: yes; allocates: yes: ${, Date.now, JSON.stringify
  - `const directory=join(area,"real"),library=join(import.meta.dir,"../../../../../"),run=()=>{const began=Date.now(),child=Bun.spawnSync([process.execPath,"-e",'const api=await import(${JSON.stringify(join(library,"🟨️.mj...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests/🟦️.ts:178` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const library=resolve(import.meta.dir,"../../../../../🟨️.mjs"),run=Bun.spawnSync([process.execPath,"-e",'const m=await import(${JSON.stringify(library)});console.log(JSON.stringify(m.nxBuildEngineClosureV1()));'],{env...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📬️task-child/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📬️task-child/🟦️.ts:17` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `try{mkdirSync(join(root,".nx"),{recursive:true});symlinkSync(dirname(sdkPackage),join(root,".nx/installation"),process.platform==="win32"?"junction":"dir");expect(realpathSync(createRequire(join(root,".nx/installation...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📬️task-child/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify
  - `try{mkdirSync(join(root,".nx"),{recursive:true});symlinkSync(dirname(sdkPackage),join(root,".nx/installation"),process.platform==="win32"?"junction":"dir");expect(realpathSync(createRequire(join(root,".nx/installation...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative/🟦️.ts:55` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const moduleProof="const{createRequire}=require('node:module'),p=require('node:path'),f=require('node:fs'),root=process.argv[1],base=process.argv[2],r=createRequire(p.join(root,'package.json')),file=r.resolve('semver'...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/🟦️.ts:81` [console.log] test-path: yes; allocates: yes: .repeat(, JSON.stringify
  - `await mkdir(temporary,{recursive:true});try{await writeFile(source,"export const value: number = 7;\n"+"// bounded runtime cache probe\n".repeat(400));const env=nxSdkCompilerEnvironmentV1({...store.environment,BUN_RUN...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🧊️binary/🧪️tests/📥️runtime/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🧊️binary/🧪️tests/📥️runtime/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `export async function proveNxSdkBinaryPublicationV1(store:RepositoryToolingStoreV1,control:ResourceLeaseControl):Promise<void>{await store.recheck();const directory=await mkdtemp(join(store.root,"tmp/sb-")),source=joi...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📥️loading/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📥️loading/🟦️.ts:14` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `const path=join(directory,"semio-authorizers.mjs"),before=observeNxAuthorizerBundleV1(directory,check),probe='const[url,encoded]=process.argv.slice(1),required=JSON.parse(encoded),body=await import(url);for(const name...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️workspace/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️workspace/🧪️tests/🟦️.ts:10` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const snapshot=snapshotNxAuthorizerSourcesV1(captured);await expect(compileNxAuthorizerSnapshotV1(root,snapshot,{...control},"node","module")).rejects.toThrow("issuance");let accessorReads=0;expect(()=>readNxAuthorize...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📦️compiler-init/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📦️compiler-init/🟦️.ts:17` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `try{const file=join(directory,"bundle.mjs");await writeFile(file,sealed.bytes);return spawnSync(registeredToolExecutableV1("node"),["--input-type=module","--eval","await import("+JSON.stringify(pathToFileURL(file).hre...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🚪️entry-law/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🚪️entry-law/🟦️.ts:36` [console.log] test-path: yes; allocates: no
  - `console.log("[headroom] "+report+" "+text("de","budget",{files:census.files,sourceBytes:census.sourceBytes,budgetBytes:law.maximumSourceBytes,headroom,limitBytes:snapshotPolicy.maximumSourceBytes}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify, new Set
  - `const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original fixture artifact directory required");await mkdir(artifacts,{recursive:true});for(const target of["runtime","node"]as const)for(c...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts:78` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await captureOwnedCommandV1(process.execPath,[program,"compile",root,snapshot,sha,output],checkout,{control,env});const bundle=pathToFileURL(join(output,"semio-authorizers.mjs")).href,node=await captureOwnedCommandV1(...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts:92` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await writeFile(join(root,fixture.mutation.path),fixture.mutation.bytes);await rm(join(root,fixture.lazyMutation.path));const result=await compileNxAuthorizerSnapshotV1(root,snapshot,control,"node","module");expect(re...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts:105` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify, collect, new Set
  - `const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original workspace artifact directory required");await mkdir(artifacts,{recursive:true});const workspaceOwner=paths.join(root,"🧰️framework...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts:146` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const {relative,resolve}=await import("node:path"),{transform}=await import("esbuild"),database=new Database(":memory:"),validate=new Ajv({strict:true}).compile({$defs:snapshotSchema.$defs,$ref:"#/$defs/EntryRoleCase"...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧲️node-native/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧲️node-native/🟦️.ts:28` [console.log] test-path: yes; allocates: yes: .join(, .push(, JSON.stringify
  - `const program='const fs=require("node:fs"),cp=require("node:child_process"),module_=require("node:module"),url=require("node:url"),saved=[],calls=[];for(const[object,names]of [[fs,["readFileSync","existsSync","mkdirSy...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🟦️.ts:93` [console.log] test-path: no; allocates: yes: ${
  - `const progress = setInterval(() => console.log('Preparing Binaryen ${manifest.version}…'), 10000);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🟦️.ts:95` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Downloading pinned Binaryen ${manifest.version} for ${row.platform}/${row.architecture}…');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🟦️.ts:155` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(versions));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🟦️.ts:89` [console.log] test-path: no; allocates: yes: ${
  - `const staging=mkdtempSync(join(base,".prepare-")),deadline=new AbortController(),watch=setInterval(()=>{try{checkResourceLeaseControl(control);}catch(error){deadline.abort(error);}},25),progress = setInterval(() => co...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🟦️.ts:91` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Acquiring pinned Bun ${version} for ${row.platform}/${row.arch}…');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/🧰️adoption/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/🧰️adoption/🧪️tests/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const module=new URL("../🟦️.ts",import.meta.url).href,ask=(workspace:string)=>spawnSync(process.execPath,["-e","const m=await import("+JSON.stringify(module)+");const row=m.adoptedRustToolchainV1("+JSON.stringify(work...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/📜️script.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/📜️script.ts:40` [console.log] test-path: no; allocates: no
  - `console.log(toolchainIdentityV1(kind as ToolchainKindV1, {host, ...(project === undefined ? {} : {project})}).digest);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/📜️script.ts:43` [console.log] test-path: no; allocates: yes: .map(, JSON.stringify
  - `console.log(JSON.stringify(Object.fromEntries(kinds.map(name => [name, toolchainIdentityV1(name, {host}).digest]))));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/📜️script.ts:53` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(pythonEnvironmentOfProjectV1(workspace, project, roots)));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🟦️.ts:20` [console.log] test-path: no; allocates: no
  - `log: (text: string) => console.log(text),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🟦️.ts:36` [console.log] test-path: no; allocates: no
  - `log: (text: string) => console.log(text),`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🧪️tests/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🧪️tests/📜️script.ts:24` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({scenario,rootSucceeded,attempts,recoveries,effects,refusalShape,oracle,events}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/📥️negative/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/📥️negative/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const purpose=requirePurpose(value);requireWorkspace(workspace);checkScriptInvocation(original);console.log(ackPrefix+JSON.stringify(acknowledgement(original,workspace,purpose)));let waited=0,entered=false;const abort...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/📥️negative/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `}catch(error){const reason=error instanceof Error?error.message:String(error);console.log(failurePrefix+JSON.stringify({version:1,purpose,reason,waited,entered}));throw error;}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🟦️.ts:58` [console.log] test-path: yes; allocates: yes: .join(, .push(, Date.now, JSON.stringify
  - `try{try{await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope(readDefaultBuildCliPolicyV1(),{workspaceRoot:process.cwd()},Date.now()),async original=>{birth=routing.readIssuedScriptProcessEnvelop...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🟦️.ts:199` [console.log] test-path: yes; allocates: yes: .join(, .push(, .repeat(, JSON.stringify
  - `registerIsolatedBuildTestV1(import.meta.path,"selected tools copied read inventory binds complete actual material",{bodyMilliseconds:120000,scope:{kind:"body"}},()=>withBuildTestInvocationV1(async original=>{const fs=...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🟦️.ts:216` [console.log] test-path: yes; allocates: yes: .join(, .push(, .repeat(, JSON.stringify
  - `test.skipIf(selectedDerivativeReadCase===undefined)("selected tools derivative read proof preserves authentic material and completion",()=>withBuildTestInvocationV1(async original=>{const fs=await import('node:fs/prom...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🟦️.ts:35` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(result));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🎟️funding/📦️child/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🎟️funding/📦️child/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const effects=(db.query("select count(*) n from effects").get()as{n:number}).n;db.close();console.log(JSON.stringify({failure,effects}));process.exitCode=failure===null?0:1;`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/📚️dependencies/🧪️tests/📦️child/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/📚️dependencies/🧪️tests/📦️child/📜️script.ts:22` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(result.readers)result.blockedAfter=probe();console.log(JSON.stringify(result));process.exitCode=result.failed?1:0;`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/📚️dependencies/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/📚️dependencies/🧪️tests/🟦️.ts:43` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify, new Map
  - `registerBuildTestLawV1("default Rust dependency readers retain actual SH through accepted child settlement","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:dependency-reader-cases",0,async control=>{c...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📤️receipt/🧪️tests/📦️child/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📤️receipt/🧪️tests/📦️child/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `});}catch(error){failed=true;message=error instanceof Error?error.message:String(error);}finally{await rm(parent,{recursive:true,force:true});}console.log(JSON.stringify({failed,message,receipt,oracle}));process.exitC...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/📦️batch/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/📦️batch/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket scope sink required");const root=await mkdtemp(sink+sep),db=new Database(":memory:");try{const program=setup(root)+'const{mock}=await ...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/📦️batch/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket scope sink required");const root=await mkdtemp(sink+sep);try{for(const row of corpus.changes){const program=setup(root)+'const owner=a...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/📦️batch/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: ${, .filter(, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket scope sink required");const root=await mkdtemp(sink+sep),db=new Database(":memory:");try{expect(db.query("select ?+?<=67108864 accepte...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/📦️batch/🟦️.ts:30` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket scope sink required");const root=await mkdtemp(sink+sep);try{for(const row of corpus.epochs){const program=setup(root)+'const{default:...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/📦️batch/🟦️.ts:33` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket scope sink required");const root=await mkdtemp(sink+sep);try{for(const row of corpus.guards){const program=setup(root)+'const{mock}=aw...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts:32` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket guard sink required");const root=await mkdtemp(join(sink,"cl-")),routing=fileURLToPath(new URL("../../../../../../../../../../../🔨️mod...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: ${, .map(, .push(, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket request sink required");const root=await mkdtemp(join(sink,"cl-")),routing=fileURLToPath(new URL("../../../../../../../../../../../🔨️m...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts:55` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket planning-limit sink required");const root=await mkdtemp(join(sink,"cl-")),routing=fileURLToPath(new URL("../../../../../../../../../.....`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts:59` [console.log] test-path: yes; allocates: yes: ${, .map(, .push(, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket read-limit sink required");const root=await mkdtemp(join(sink,"cl-")),db=new Database(":memory:"),routing=fileURLToPath(new URL("../.....`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts:63` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket aggregate sink required");const root=await mkdtemp(join(sink,"cl-")),routing=fileURLToPath(new URL("../../../../../../../../../../../🔨...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts:67` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual ticket known-epoch sink required");const root=await mkdtemp(join(sink,"cl-")),routing=fileURLToPath(new URL("../../../../../../../../../../.....`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/🧪️tests/🧫️fixture/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/🧪️tests/🧫️fixture/🧪️tests/🟦️.ts:17` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `registerBuildTestLawV1("default Rust fixture consumer empty preparation retains actual Original children and primaries","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:fixture-consumer:lifetime",0,asy...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🚧️staging/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🚧️staging/🧪️tests/🟦️.ts:23` [console.log] test-path: yes; allocates: yes: ${, .map(, .push(, JSON.stringify
  - `for(const row of corpus.residual)registerBuildTestLawV1("default repeated Originals cannot accumulate abandoned descriptor stages "+row.name,"sdk-fence",original=>withIssuedResourceControlV1(original,"rust:stage-resid...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🚧️staging/🧪️tests/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default fixed staging requires actual writer and exclusive retirement custody","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:stage-guards:parent",0,async control=>{const sink...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📤️publication/🧪️tests/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📤️publication/🧪️tests/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `registerBuildTestLawV1("default Rust publication flow performs actual admission maintenance and reader handoff","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:publication-flow:states",0,async control...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📤️publication/🧪️tests/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `registerBuildTestLawV1("default Rust publication flow preserves arbitrary phase failures without maintenance retry","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:publication-flow:primaries",0,async ...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📤️publication/🧪️tests/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default Rust publication flow checks loaded Source before context hooks and final delivery","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:publication-flow:epochs",0,async con...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📤️publication/🧪️tests/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default Rust publication consumer retains real exclusion and rechecks after accepted child drainage","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:publication-flow:child-drai...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧪️tests/📦️child/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧪️tests/📦️child/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const command=process.argv[2];if(command==="catalog-page-pressure"){const root=process.argv[3],bytes=Number(process.argv[4]);if(!root||bytes!==33554432)throw new Error("Exact page-pressure input required");const file=...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧪️tests/📦️child/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `let freed=true;for(const resource of row.operation==="set"?[a,b]:[catalog]){let db:Database|undefined;try{db=new Database(resourceLeasePathV1(directory,resource));db.exec("begin exclusive");db.exec("rollback");}catch(...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧪️tests/📦️child/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `}else if(command==="waiting-maintenance"){const root=process.argv[3],directory=process.argv[4],begin=process.argv[5];if(!root||!directory||!begin)throw new Error("Exact waiting maintenance coordinates required");await...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧪️tests/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧪️tests/🟦️.ts:30` [console.log] test-path: yes; allocates: yes: ${, .filter(, .map(, JSON.stringify
  - `registerBuildTestLawV1("default interrupted selective retirement overaccounts absent victims and preserves wanted artifacts","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:retirement-interruption:par...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧪️tests/🟦️.ts:31` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `for(const row of corpus.concurrent)registerBuildTestLawV1("default concurrent complete descriptor sets publish atomically "+row.name,"sdk-fence",original=>withIssuedResourceControlV1(original,"rust:working-set-concurr...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧪️tests/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default actual descriptor staging preserves primary errors and undefined across cleanup","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:files:cleanup-parent",0,async control=>...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧪️tests/🟦️.ts:41` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify, collect
  - `registerBuildTestLawV1("default incremental collection refuses foreign expired and failed controls before DATA access","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:collection-guards:parent",0,async...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/📖️read/🧪️tests/📦️child/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/📖️read/🧪️tests/📦️child/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: .map(, .push(, .repeat(, Array.from
  - `try{await receiveScriptProcessInvocation(process.env,async original=>{let values=structuredClone(fixture.units.slice(0,2));if(name==="deep"||name==="depth-limit")values=Array.from({length:name==="deep"?65:64},(_,index...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/📖️read/🧪️tests/🧺️batch/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/📖️read/🧪️tests/🧺️batch/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `registerBuildTestLawV1("default Rust descriptor batch publisher reuses real reads and avoids warm directory writes","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:descriptor-match-publication",0,asyn...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/📖️read/🧪️tests/🧺️batch/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${, .push(, Array.from, JSON.stringify
  - `registerBuildTestLawV1("default Rust descriptor batch matches standalone immutable files within its complete bounds","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:descriptor-match-bounds",0,async co...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/📖️read/🧪️tests/🧺️batch/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default Rust descriptor batch rejects copied duplicate and executable node DATA before IO","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:descriptor-match-invalid",0,async con...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/📖️read/🧪️tests/🧺️batch/🟦️.ts:23` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default Rust descriptor batch closes every read before any repair write on failure","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:descriptor-match-failures",0,async control=>...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/📖️read/🧪️tests/🧺️batch/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default Rust descriptor batch retains source and policy epochs before hooks and delivery","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:descriptor-match-epochs",0,async contr...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️nodes/🧪️tests/♻️composition/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️nodes/🧪️tests/♻️composition/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${, .map(, .push(, .repeat(
  - `const program='const {mock}=await import('bun:test'),validator=await import('@semio-tech/schema-validator-ts'),actualCompile=validator.compileJsonSchemaSubset,corpus=${JSON.stringify(corpus)},fixture=${JSON.stringify(...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️nodes/🧪️tests/♻️composition/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default Rust descriptor composition refuses changed plan-branch constraints at preparation","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:descriptor-composition:schema",0,asy...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️nodes/🧪️tests/📦️child/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️nodes/🧪️tests/📦️child/🟦️.ts:10` [console.log] test-path: yes; allocates: yes: Array.from, JSON.stringify
  - `try{await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"rust:graph:failure",0,signal.signal),input=Array.from({length:32},(_,index)=>{const unit=read...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️inputs/🔏️snapshot/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️inputs/🔏️snapshot/🧪️tests/🟦️.ts:29` [console.log] test-path: yes; allocates: yes: ${, .map(, JSON.stringify
  - `for(const row of corpus.runtimeCases)test("neutral Rust real Source rediscovery "+row.id,()=>withBuildTestInvocationV1(async original=>{const{root,directory,control,boundary,observed,inputs,configuration,plan}=await o...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/📐️profile/♻️preparation/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/📐️profile/♻️preparation/🟦️.ts:15` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default Cargo profile validation prepares fixed contracts once and observes every supplied value","sdk-fence",original=>withIssuedResourceControlV1(original,"cargo:profile-preparation",0,async ...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🔬️script-commands/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🔬️script-commands/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await image.recheck();await withRustUnitDescriptorV2(request,control,async(descriptor,control)=>{const selected=descriptor.node.plan;if(args[0]==="plan"){console.log(JSON.stringify({version:2,node:descriptor.node,pred...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts:51` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await withBuildTestInvocationV1(original=>withRepositoryNxStorageV1(repository.root,original,scriptCompilerAdmissionControlV1(original,"rust:nx-fixture",0),async(bind,nxPhase)=>{const identity=process.env.SEMIO_NX_TOO...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts:172` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `test("sealed Rust compiler entry agrees with actual Bun CLI planning and retains Original code custody",async()=>{const workspace=process.cwd();await withBuildTestInvocationV1(original=>withRepositoryRustUnitCodeV1(re...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🏗️buildscript/☁️offload/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🏗️buildscript/☁️offload/🧪️tests/🟦️.ts:34` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("warning: generated in {}",out.display());`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts:278` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `if(argv[0]==="--semio-unit-cache"){console.log(JSON.stringify(await manageV1(argv.slice(1))));return 0;}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🧪️tests/🟦️.ts:213` [console.log] test-path: yes; allocates: yes: ${, .repeat(, JSON.stringify
  - `const program='const m=await import(${JSON.stringify(client)});let calls=0;const settings={endpoint:"http://127.0.0.1:1",instance:"main",token:undefined,upload:false,stateDirectory:${JSON.stringify(state)},signedRoot:...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/📥️worker/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/📥️worker/🟦️.ts:5` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const print=(row:object)=>console.log(JSON.stringify(row));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🔐️pool-claims/📥️worker/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧪️tests/🔐️pool-claims/📥️worker/🟦️.ts:27` [console.log] test-path: yes; allocates: no
  - `console.log("ready");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🔐️retention-io/📥️worker/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🧪️tests/🔐️retention-io/📥️worker/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({index,lane:lane.paths.id,conflict}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🧾️provenance/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🧾️provenance/🟦️.ts:171` [console.log] test-path: no; allocates: yes: ${
  - `report = scanBuildDirProvenance(buildDir, trustedBuildRoots(this.repoRoot), process.platform, controller.signal, json ? undefined : (target, done, total) => done % 250 === 0 || done === total ? console.log('[cargo-pro...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🧾️provenance/🟦️.ts:177` [console.log] test-path: no; allocates: yes: ${
  - `for (const [target, counts] of Object.entries(perTarget)) console.log('[cargo-provenance] ${target}: ${counts.depInfos} dep-info files, ${counts.foreign} foreign');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🧾️provenance/🟦️.ts:178` [console.log] test-path: no; allocates: yes: ${
  - `for (const root of roots) console.log('[cargo-provenance] foreign source root: ${root}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🧾️provenance/🟦️.ts:179` [console.log] test-path: no; allocates: yes: ${
  - `for (const unit of report.undecodable) console.log('[cargo-provenance] undecodable ${unit.depInfo}: ${unit.reason}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🧾️provenance/🟦️.ts:180` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[cargo-provenance] ${report.foreign.length} foreign units${mode === "repair" ? '; invalidated ${removed}' : ""}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:32` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[cache-policy] ${vectors.policies.length} portable owner-cache laws agree with installed Nx');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:603` [console.log] test-path: yes; allocates: no
  - `console.log("[cache-contract] schema, graph ownership, source-byte discovery, native dependency oracles and materializer cancellation passed");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts:291` [console.log] test-path: yes; allocates: no
  - `if (args[0] === "nextest" && args[1] === "list") console.log("{}");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌍️caller-environment/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌍️caller-environment/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify
  - `await withBuildTestInvocationV1(async original=>{const source='const owner=require(process.env.SEMIO_ENVIRONMENT_SDK),row=JSON.parse(process.env.SEMIO_ENVIRONMENT_CASE),before=JSON.stringify(process.env),actual=owner....`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌍️caller-environment/🟦️.ts:28` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const source='(async()=>{const fs=require('node:fs'),path=require('node:path'),sdk=require('node:module').createRequire(process.env.SEMIO_STARTUP_SDK),client=sdk('nx/src/daemon/client/client').daemonClient,ready=path....`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌍️caller-environment/🟦️.ts:41` [console.log] test-path: yes; allocates: yes: .repeat(, JSON.stringify
  - `await withBuildTestInvocationV1(async original=>{const source='const {daemonClient:client}=require(process.env.SEMIO_ENVIRONMENT_SDK);client.startDaemonIfNecessary=async()=>{};client.socketMessenger={sendMessage(messa...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌍️caller-environment/🟦️.ts:53` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .map(, .push(
  - `await withBuildTestInvocationV1(async original=>{const source='const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),sdk=require('node:module').createRequire(process.env.SEMIO_SCOPE_SDK),...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌍️caller-environment/🟦️.ts:59` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `await withBuildTestInvocationV1(async original=>{for(const row of fixture.daemonFailures){const root=mkdtempSync(join(artifacts,"df-"));try{writeFileSync(join(root,"nx.json"),"{}");writeFileSync(join(root,"package.jso...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌍️caller-environment/🟦️.ts:65` [console.log] test-path: yes; allocates: yes: .join(, .map(, .push(, JSON.stringify
  - `try{mkdirSync(join(root,"patches"));await withBuildTestInvocationV1(async original=>{const source='const fs=require('node:fs'),path=require('node:path'),url=require('node:url'),Module=require('node:module'),load=Modul...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🔐️custody/🟦️.ts:41` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `mkdirSync(${JSON.stringify(productOutput)},{recursive:true});writeFileSync(${JSON.stringify(join(productOutput,"result.json"))},readFileSync("source.json"));console.log("[TRACE] fixture compiler executed");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts:180` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const workerResultCapture=await captureOwnedCommandResultV1(process.execPath,[resolve(workspace,workerCase.entry),"owner-command","--cwd",workspace,"--","node","-e","console.log(JSON.stringify({arguments:process.argv....`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️artifact-packages/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️artifact-packages/🟦️.ts:117` [console.log] test-path: yes; allocates: no
  - `console.log(`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/📥️purpose/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/📥️purpose/📜️script.ts:16` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `try{await receiveScriptProcessInvocation(process.env,async original=>{const parent=createIssuedResourceControlV1(original,"native-fixture:negative",0),phase=nativeCallerPhaseV1(original,workspace,parent),storage={vers...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/📥️purpose/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/📥️purpose/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: .filter(, Date.now, JSON.stringify
  - `export async function runNativeDependencyFixtureNegativeV1(workspace:string,output:string,root:string,invocation:ScriptInvocation,parent:ResourceLeaseControl,purpose:"mutation"|"changed",id:string,calls:number,reason:...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️native-dependencies/🧪️tests/🟦️.ts:40` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const child=await captureOwnedCommandResultV1(process.execPath,["-e",'console.log(JSON.stringify({target:process.env.CARGO_TARGET_DIR,build:process.env.CARGO_BUILD_BUILD_DIR}))'],root,{...phase.options({...process.env...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔁️graph-revision/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔁️graph-revision/🟦️.ts:29` [console.log] test-path: yes; allocates: no
  - `writeFileSync(join(directory, "package/📦️packages/🦀️rust/📜️script.ts"), 'console.log("fixture");');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔁️graph-revision/🟦️.ts:42` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `assert.deepEqual(results,fixture.expected); console.log(JSON.stringify(results));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🟦️.ts:120` [console.log] test-path: yes; allocates: no
  - `const progress = setInterval(() => console.log("[TRACE] Native Trunk watch lockfile probe running"), 10000);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔗️command-imports/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔗️command-imports/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const result = spawnSync(registeredToolExecutableV1("node"), ["--input-type=module", "-e", 'const api = await import(${JSON.stringify(url)}); console.log(JSON.stringify(${expression}));'], { cwd: root, encoding: "utf8...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔗️import-edges/🔁️context/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔗️import-edges/🔁️context/🟦️.ts:86` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify, new Map
  - `const resolved = spawnSync(registeredToolExecutableV1("node"), ["-e", 'const fs=require("node:fs"),{TargetProjectLocator}=require(process.argv[1]),x=JSON.parse(fs.readFileSync(0,"utf8")),l=new TargetProjectLocator(x.n...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/bindgen.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/bindgen.ts:20` [console.log] test-path: yes; allocates: yes: .push(
  - `const staleLines:string[]=[];await runOwnedCommand("wasm-bindgen",["-e",'console.log("wasm-bindgen '+fixture.stale+'");'],workspace,"wasm-bindgen:ambient-fixture",10000,{control,env:environment,onLine:(line,stream)=>{...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/bindgen.ts:22` [console.log] test-path: yes; allocates: no
  - `const port:WasmBindgenToolPortV1={lock:()=>lock,prepared:()=>owned,lookup:()=>ambient,exists:()=>true,probe:command=>({status:0,stdout:"wasm-bindgen "+(command===owned?fixture.locked:fixture.stale)})},selected=resolve...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🗿️artifact-source/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🗿️artifact-source/🟦️.ts:31` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({execution:count,value:provider.value,label:fixture.label,limit:control.limit,sql:readFileSync("../../domain/schema.sql","utf8")}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-runtime/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-runtime/🟦️.ts:73` [console.log] test-path: yes; allocates: no
  - `else{const manifest=JSON.parse(readFileSync(join(mirror,"🔣️runtime.json"),"utf8"));for(const module of manifest.modules){const bytes=readFileSync(join(mirror,module.wasmPath));assert.equal(bytes.subarray(0,8).toString...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️wasm-outputs/📥️purpose/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️wasm-outputs/📥️purpose/📜️script.ts:16` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `catch(error){if(!observed||!onlyWasmOutputRefusalV1(error,observed.error))throw error;const receipt=JSON.stringify({version:1,complete:true,id:fixture.negative.id,mode:observed.mode,reason:observed.reason,wire,wireSha...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️wasm-outputs/📥️purpose/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️wasm-outputs/📥️purpose/🟦️.ts:26` [console.log] test-path: yes; allocates: no
  - `const after=lstatSync(root,{bigint:true});if(!after.isDirectory()||after.isSymbolicLink()||after.dev!==folder.dev||after.ino!==folder.ino||readFileSync(join(root,"🦀️.rs"),"utf8")!==source)throw Error("Original Wasm re...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️wasm-outputs/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️wasm-outputs/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const value=async(path:string)=>{const code='import {readFileSync} from "node:fs";import {pathToFileURL} from "node:url";const module=await import(pathToFileURL(process.argv[1].replace(/_bg.wasm$/,".js")).href);const ...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts:149` [console.log] test-path: yes; allocates: no
  - `put(entry, readFileSync(join(root, entry), "utf8") + '\nconsole.log("wgpu-publication-source-change");\n');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧱️command-source/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧱️command-source/🟦️.ts:187` [console.log] test-path: yes; allocates: yes: .join(
  - `const parent = ["const {spawn}=require('node:child_process')", "const child=spawn(process.execPath,['-e','setTimeout(() => {}, 10000)'],{stdio:'ignore'})", "console.log('descendant='+child.pid)", "setTimeout(() => {},...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/⏱️measurement/🧪️tests/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/⏱️measurement/🧪️tests/🟦️.ts:22` [console.log] test-path: yes; allocates: no
  - `const workspace=(original.capabilities as{workspaceRoot?:unknown}).workspaceRoot;if(typeof workspace!=="string"||!isAbsolute(workspace))throw Error("Actual finite scanner fixture root required");const control=createIs...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/⏱️measurement/🧪️tests/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: .push(
  - `console.log=(...values:unknown[])=>{const text=String(values[0]);if(text.startsWith(owner.cacheScanTimingPrefixV1)){const json=text.slice(owner.cacheScanTimingPrefixV1.length);logBytes+=Buffer.byteLength(text)+1;recor...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/⏱️measurement/🧪️tests/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: .map(
  - `const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");try{for(const record of records){expect(validate(record)).toBe(true);expect(record.root).toBe(root);expect(Reflect.has(record,"files"))...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/⏱️measurement/🧪️tests/🟦️.ts:28` [console.log] test-path: yes; allocates: no
  - `}finally{console.log=log;await rm(root,{recursive:true});}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️script.ts:30` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `try{const declaration=repositoryStrictStorageV1(this.repoRoot),storage=await repositoryCompilerReservationV1(image,{version:1,root:image.root,leaseDirectory,capacityBytes:declaration.capacityBytes,maximumWorkspaceByte...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts` (8)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts:20` [console.log] test-path: no; allocates: yes: ${
  - `const areas = await scanCacheAreas(this.repoRoot, control, json ? undefined : (area, unit) => console.log('[cache-report] scanned ${area}/${unit.path}'));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts:29` [console.log] test-path: no; allocates: no
  - `console.log(`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts:33` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[cache-report] tests: ${await pruneTestEvidence(this.repoRoot, true, this.invocation, control, resolveLanguageV1(CATALOG_LANGUAGES_V1,args,process.env).language)}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts:50` [console.log] test-path: no; allocates: yes: ${
  - `const areas = await scanCacheAreas(this.repoRoot, control, (area, unit) => console.log('[cache-prune] scanned ${area}/${unit.path}'));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts:57` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[cache-prune] ${dryRun ? "would delete" : "deleting"} ${area.name}/${unit.path} (${formatBytes(unit.bytes)})');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts:62` [console.log] test-path: no; allocates: yes: ${
  - `if (area.guardedOverBudgetBytes) console.log('[cache-prune] ${area.name}: ${formatBytes(area.guardedOverBudgetBytes)} over budget but touched within the last ${Math.round(CACHE_POLICY.storage.guardAgeMs / 3600000)}h; ...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts:64` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[cache-prune] ${dryRun ? "would delete" : "deleted"} ${deletedCount} units; ${formatBytes(deletedBytes)}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts:66` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[cache-prune] tests: ${await pruneTestEvidence(this.repoRoot, dryRun, this.invocation, control, resolveLanguageV1(CATALOG_LANGUAGES_V1,args,process.env).language)}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📦️metadata/🧪️tests/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📦️metadata/🧪️tests/🟦️.ts:35` [console.log] test-path: yes; allocates: no
  - `const workspace=(original.capabilities as{workspaceRoot?:unknown}).workspaceRoot;if(typeof workspace!=="string"||!isAbsolute(workspace))throw Error("Actual finite metadata fixture root required");const control=createI...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📦️metadata/🧪️tests/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: .push(
  - `console.log=(...values:unknown[])=>{const text=String(values[0]),prefix="[TRACE] Cargo scanner timing ";if(text.startsWith(prefix))summaries.push(JSON.parse(text.slice(prefix.length)));else log(...values);};const actu...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📦️metadata/🧪️tests/🟦️.ts:99` [console.log] test-path: yes; allocates: yes: .push(, Array.from, JSON.stringify, new Map
  - `const abort=new AbortController(),cancel=Error("actual accepted metadata cancellation"),cancelled=createIssuedResourceControlV1(original,"test:accepted-metadata-drain",120000,abort.signal),releases=new Map<number,()=>...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📦️metadata/🧪️tests/🟦️.ts:100` [console.log] test-path: yes; allocates: no
  - `}finally{console.log=log;await rm(root,{recursive:true});await rm(external,{recursive:true});}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🟦️.ts:116` [console.log] test-path: no; allocates: no
  - `report(complete:boolean,units:number):void{if(this.timing)try{console.log(cacheScanTimingPrefixV1+formatCacheScanTimingV1(this.timing.finish(complete?"complete":"failed",units)));}catch{}}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/🧪️tests/🔭️loader/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/🧪️tests/🔭️loader/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const runner=join(scratch,"runner.cjs");writeFileSync(runner,"require(process.argv[2]).loadSemioGraphAuthorizersV1().then(value=>console.log(JSON.stringify(value===null?null:{identity:value.toolingIdentity,entry:value...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩺️environment/📋️inspection/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩺️environment/📋️inspection/🟦️.ts:10` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[nx-doctor] ${command}: ${result.exitCode === 0 ? result.stdout.toString().split("\n")[0] : "unavailable"}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩺️environment/📋️inspection/🟦️.ts:12` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[nx-doctor] ${command}: unavailable');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🎯️tool-job-lane/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🎯️tool-job-lane/🟦️.ts:24` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(toolJobSharedFrameworkActionFixtureRun(this.root)));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🎯️tool-job-lane/🟦️.ts:38` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(fixture));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🎯️tool-job-lane/🟦️.ts:95` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `if (format === "json" && !output) console.log(JSON.stringify(report, null, 2));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts` (7)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:30` [console.log] test-path: no; allocates: yes: ${
  - `for (const failure of apps.failures.slice(0, 25)) console.log('  ${failure}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:35` [console.log] test-path: no; allocates: yes: ${
  - `for (const category of Object.keys(report.byCategory).sort()) console.log('  ${category}: ${report.byCategory[category]}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:38` [console.log] test-path: no; allocates: yes: ${
  - `for (const f of report.blockingBridgeUnlisted) console.log('  ${f.file}:${f.line}: ${f.text}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:44` [console.log] test-path: no; allocates: yes: ${
  - `for (const f of inCategory) console.log('  ${f.file}:${f.line}: ${f.text}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:48` [console.log] test-path: no; allocates: yes: ${
  - `for (const e of report.staleAllowlistEntries) console.log('  ${e.file} (${e.pattern}, ${e.phase})');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:52` [console.log] test-path: no; allocates: yes: ${
  - `for (const e of report.expectedNeverToMatchEntries) console.log('  ${e.file} (${e.pattern}, ${e.phase})');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:56` [console.log] test-path: no; allocates: yes: ${
  - `for (const e of report.preDeclaredOutOfScope) console.log('  ${e.file} (${e.pattern}, ${e.phase})');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts` (7)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts:341` [console.log] test-path: no; allocates: yes: ${
  - `for (const { first, count } of files.slice(0, 50)) console.log('${TYPESCRIPT_TAG}   ${first}${count > 1 ? ' (+${count - 1} more)' : ""}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts:351` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${TYPESCRIPT_TAG} project ${path} roots=${roots.length}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts:353` [console.log] test-path: no; allocates: no
  - `if (parsed.list) for (const rootPath of roots) console.log(rootPath);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts:356` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${TYPESCRIPT_TAG} node --max-old-space-size=${heap} ${relative(root, fileURLToPath(import.meta.url))} ${TYPESCRIPT_CHECK_FLAG} ${relative(root, path)} > ${relative(root, logPath)}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts:361` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${TYPESCRIPT_TAG} roots=${roots.length} diagnostics=${summary.total} files=${summary.files} top: ${codes || "none"}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts:372` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${TYPESCRIPT_TAG} app ${app.project} diagnostics=${appSummary.total} files=${appSummary.files}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts:378` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${TYPESCRIPT_TAG} passed.');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🚦️gates/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🚦️gates/🟦️.ts:51` [console.log] test-path: no; allocates: yes: ${, .join(
  - `for (const [owner, row] of [...perOwner].sort((left, right) => right[1].unreachable.length - left[1].unreachable.length)) console.log('| ${owner} | ${row.migrated} | ${row.unreachable.length} | ${row.other} | ${row.un...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts:256` [console.log] test-path: no; allocates: no
  - `console.log(breaches.length);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts:257` [console.log] test-path: no; allocates: no
  - `for (const b of breaches) console.log(b.kind, "|", b.scope, "|", b.summary);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts:41` [console.log] test-path: no; allocates: yes: ${
  - `for (const path of changed) console.log('[generate config] wrote ${path}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts:45` [console.log] test-path: no; allocates: no
  - `console.log("[generate config] derived configuration is current");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts:71` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[generate taxonomy ${operation}] ${census.records.length} components, ${census.problems.length} findings -> ${ticketDir}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts:99` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[generate plugin-glue] dry ${plugin}: ${owned.length} subsets');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts:103` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[generate plugin-glue] wrote ${inv}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts:106` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[generate plugin-glue] ${dry ? "dry-run" : "wrote"} inventory for ${emitted} subset refs');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📚️examples/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📚️examples/🟦️.ts:40` [console.log] test-path: no; allocates: no
  - `console.log("[examples.list] (none)");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📚️examples/🟦️.ts:49` [console.log] test-path: no; allocates: yes: ${
  - `console.log('${root}  leaf=${hasLeaf ? "yes" : "no"} assets=${hasAssets ? "yes" : "no"} tests=${hasTests ? "yes" : "no"} slug=${slug}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📚️examples/🟦️.ts:51` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[examples.list] ${roots.length} example unit(s)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/📚️examples/🟦️.ts:79` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[examples.verify] ok${pluginFilter ? ' (${pluginFilter})' : ""}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts:95` [console.log] test-path: no; allocates: no
  - `prepare: () => console.log("[prepare] Build prerequisites completed"),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts:160` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[setup] readiness LongPathsEnabled=${enabled === undefined ? "unknown" : enabled ? 1 : 0} core.longpaths=true');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts:165` [console.log] test-path: no; allocates: no
  - `console.log("[setup] locked dependency environments are ready through the repository build graph");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts:166` [console.log] test-path: no; allocates: no
  - `console.log("[setup] staging semio-os-mcp so '.mcp.json' initialize never waits on a cold build");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🕸️dependency-lint/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🕸️dependency-lint/🟦️.ts:19` [console.log] test-path: no; allocates: yes: ${
  - `const snapshot = loadDependencyDirectionPolicy(repoRoot, { onProgress: progress => console.log('[canonical-architecture] policy ${progress.phase}; sources=${progress.sources}') });`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🕸️dependency-lint/🟦️.ts:41` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[canonical-architecture] resolving present-owner TypeScript/JavaScript dependencies; scope=${sourceRole ?? "all"}; inventoriedSources=${expectedSources.length}; strictRules=${rules.length}; ownershipPatte...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🕸️dependency-lint/🟦️.ts:45` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[canonical-architecture] scope=${sourceRole ?? "all"} present-owner TypeScript/JavaScript dependency direction passed');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🕸️dependency-lint/🟦️.ts:58` [console.log] test-path: no; allocates: yes: ${
  - `const breaches = artifactIoArchitectureBreaches(this.repoRoot, undefined, undefined, { progress: event => { if (++checked % 1000 === 0 || event.phase === "complete") console.log('[artifact-io-ownership] ${event.phase}...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🕸️dependency-lint/🟦️.ts:60` [console.log] test-path: no; allocates: no
  - `console.log("[artifact-io-ownership] all artifact schema and native codec ownership boundaries passed");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🕸️dependency-lint/🟦️.ts:69` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[workspace-styling] ${kind}: current source ownership passed');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🧪️tests/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🧪️tests/🟦️.ts:228` [console.log] test-path: yes; allocates: no
  - `const written = { out: [] as string[], err: [] as string[] }, stdout = process.stdout.write.bind(process.stdout), stderr = process.stderr.write.bind(process.stderr), log = console.log, error = console.error;`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🧪️tests/🟦️.ts:230` [console.log] test-path: yes; allocates: yes: .join(, .push(
  - `process.stdout.write = capture(written.out); process.stderr.write = capture(written.err); console.log = (...values: unknown[]) => { written.out.push(values.join(" ")); }; console.error = (...values: unknown[]) => { wr...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🧪️tests/🟦️.ts:233` [console.log] test-path: yes; allocates: no
  - `await dependencyProvisionPortsV1.run(process.execPath, ["-e", "console.log('child-line')"], root, "dependencies", { PATH: process.env.PATH }, new AbortController().signal);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🧪️tests/🟦️.ts:235` [console.log] test-path: yes; allocates: no
  - `process.stdout.write = stdout; process.stderr.write = stderr; console.log = log; console.error = error;`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️schema-read-inventory/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️schema-read-inventory/🟦️.ts:12` [console.log] test-path: no; allocates: yes: ${
  - `const index = await inventorySchemaValidationReads(this.repoRoot, undefined, { checkCancellation: () => { if (controller.signal.aborted) throw controller.signal.reason; }, onProgress: row => { if (row.completed === 0 ...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗄️stdio/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗄️stdio/🟦️.ts:20` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[stdio] ${gate} passed (${ledger.counts.artifacts} artifacts, ${ledger.counts.dialects} dialects, ${ledger.counts.codecs} codecs).');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🌱️birth/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🌱️birth/🧪️tests/🟦️.ts:128` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const source = 'console.log(JSON.stringify({ profile: process.env.SEMIO_BUILD_MODE, capabilities: JSON.parse(process.env[${JSON.stringify(SCRIPT_PROCESS_INVOCATION_ENV)}]).capabilities }));';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🎮️playground/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🎮️playground/🟦️.ts:289` [console.log] test-path: no; allocates: yes: ${
  - `console.log('Playground session ${request.variant}: ${session.plugins.length} plugins staged');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🟦️.ts:106` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[test] ${level} build prerequisites completed');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts` (17)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:106` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[schema generate] ${rel} is current (${Object.keys(inventory.catalog.scopes).length} scopes).');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:110` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[schema generate] ${rel}: ${Object.keys(inventory.catalog.scopes).length} scopes, ${inventory.diagnostics.length} diagnostics.');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:154` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[schema twins] annotate: ${changed.length} schema files now carry the Rust annotations of their committed structs.');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:195` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[schema check] wrote ${sorted.length} findings to ${target}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:196` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `} else for (const finding of sorted) console.log(JSON.stringify(finding));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:199` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[schema check] modules=${inventory.modules.length} scopes=${Object.keys(inventory.catalog.scopes).length} findings=${sorted.length}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:200` [console.log] test-path: no; allocates: yes: ${
  - `for (const [code, count] of [...counts].sort(([left], [right]) => left.localeCompare(right))) console.log('[schema check] ${code}=${count}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:203` [console.log] test-path: no; allocates: yes: ${, .join(
  - `console.log('[schema check] shared-code-table=${shared.size} unshared-codes=${unshared.length}${unshared.length === 0 ? "" : ': ${unshared.join(", ")}'}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:235` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('[schema verify] catalog and index are current (${Object.keys(inventory.catalog.scopes).length} scopes, generator ${inventory.catalog.generator}).');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:238` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `for (const finding of findings) console.log(JSON.stringify(finding));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:241` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[schema verify] rust entries=${dump.entries.length} scopes=${registered.size} of ${Object.keys(inventory.catalog.scopes).length} catalogued, findings=${findings.length}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:242` [console.log] test-path: no; allocates: yes: ${
  - `for (const [code, count] of [...counts].sort(([left], [right]) => left.localeCompare(right))) console.log('[schema verify] ${code}=${count}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:255` [console.log] test-path: no; allocates: no
  - `if(name){const {withRepositoryBuildReportV1}=await import("../../🏗️build/📊️reporting/🟦️.ts");await withRepositoryBuildReportV1(this.root,name,JSON.parse(text),this.invocation,async artifact=>{console.log(artifact.path...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:279` [console.log] test-path: no; allocates: no
  - `await withRepositoryBuildReportV1(this.root,name,report,this.invocation,async artifact=>{console.log("[schema compile] native module report: "+artifact.path);});`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:302` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[schema test] harness exit=${harness}, draft-07 oracle exit=${oracle}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:341` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[schema audit] ${inventory.modules.length} modules, ${findings.length} findings -> ${target}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts:349` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[schema docs] ${rel}: ${Object.keys(inventory.catalog.scopes).length} scopes.');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧱️fixture-ownership/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧱️fixture-ownership/🟦️.ts:19` [console.log] test-path: no; allocates: yes: ${
  - `const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMillisecon...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧱️fixture-ownership/🟦️.ts:48` [console.log] test-path: no; allocates: yes: ${
  - `console.log('fixture-ownership: ${receipts.reduce((count, receipt) => count + receipt.assertions, 0)} exact native laws passed');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧹️code-style/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧹️code-style/🟦️.ts:11` [console.log] test-path: no; allocates: no
  - `console.log("[lint] Completed the repository lint graph.");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎨️styling/🏗️builder/🐍️python/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎨️styling/🏗️builder/🐍️python/🧪️tests/🟦️.ts:54` [console.log] test-path: yes; allocates: no
  - `await withResourceLeases({directory:namespace,control,signal:control.signal,resources:[{resource:"python:test:input",mode:"shared"},{resource:"python:test:output",mode:"shared"}]},()=>withCompilerCapacityConsumptionV1...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🖥️native-host/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🖥️native-host/🧪️tests/🟦️.ts:75` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('native-host-contract: vectors=${fixture.cases.length} ajv=1 toml=1 absent-read=0 malformed=3 producer=3 source=5 actual=2 profiles=4 outputs=4 input-closure=3 passed');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🧪️tests/📥️original-control/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🧪️tests/📥️original-control/🟦️.ts:14` [console.log] test-path: yes; allocates: no
  - `const oracle=spawnSync(registeredToolExecutableV1("node"),["-e",String.raw'const {spawnSync}=require("node:child_process");const child=spawnSync(process.execPath,["-e",'console.log("ready")'],{encoding:"utf8"});if(chi...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🧪️tests/📥️original-control/🟦️.ts:18` [console.log] test-path: yes; allocates: no
  - `await execute(invocation,"node",["-e",'console.log("ready")'],process.cwd(),"ordinary-repository-law",corpus.ordinary.child);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🧪️tests/📥️original-control/🟦️.ts:23` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const oracle=spawnSync(registeredToolExecutableV1("node"),["-e",String.raw'const {spawn}=require("node:child_process"),original=new AbortController();let ready=false;const child=spawn(process.execPath,["-e",'console.l...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🧪️tests/📥️original-control/🟦️.ts:26` [console.log] test-path: yes; allocates: no
  - `let ready=false,refusal:unknown;try{await execute(invocation,"node",["-e",'console.log("ready");setInterval(()=>{},1000)'],process.cwd(),"repository-live-cancel",corpus.cancel.child,{onLine:line=>{if(line==="ready"){r...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts:28` [console.log] test-path: yes; allocates: yes: .filter(
  - `const oracle=spawnSync(registeredToolExecutableV1("node"),["-e",String.raw'const {spawnSync}=require("node:child_process");const mode=process.argv[1];if(["cancel","reject","zero"].includes(mode)){process.stdout.write(...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🧪️tests/🎟️phase/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🧪️tests/🎟️phase/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const text=failure instanceof Error?failure.message:"",kind=row.control==="capture-once"&&invocationReads===1&&(failure===boundary||row.owner==="core"&&text.includes("native storage reservation"))?"capture":failure===...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🧪️tests/📥️invocation/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🧪️tests/📥️invocation/🟦️.ts:14` [console.log] test-path: yes; allocates: no
  - `}else test("actual exact protocol refusal child",async()=>{let failed=false,primary:unknown;try{await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"e...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏭️inventory/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏭️inventory/🧪️tests/🟦️.ts:29` [eprintln!] test-path: yes; allocates: yes: collect, println!, vec!
  - `const workspace=process.cwd(),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket output root required");const source=join(artifacts,"mb-"+randomUUID().slice(0,8));mkdirSync(source...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/🟦️.ts:41` [[DEBUG]] test-path: yes; allocates: no
  - `expect(text).toContain('row.args.includes("--offline")&&scope.selections.has(');expect(text).toContain("offline:false");expect(source).toContain("options.requests.find(value=>!registryCoveredV1(scope,value))");expect(...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/📥️capture/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/📥️capture/🧪️tests/🟦️.ts:17` [eprintln!] test-path: yes; allocates: yes: println!
  - `const root=repository.root,artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket inputs required");const source=join(artifacts,"cc-"+randomUUID().slice(0,8)),mode=process.env.SEMIO_B...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🧰️native-helpers/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🧰️native-helpers/🟦️.ts:41` [console.log] test-path: no; allocates: no
  - `const built=(await shareSchedulerBuildV1({workspace,targetDirectory:store.environment.CARGO_TARGET_DIR!,toolchain:store.toolchain.digest,environment:input.environment,mode:readBuildMode(store.environment.SEMIO_BUILD_M...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts` (15)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:89` [console.log] test-path: no; allocates: yes: ${
  - `for (const edit of result.updated) console.log('  ${dryRun ? "~ (dry-run)" : "~"} ${edit}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:97` [console.log] test-path: no; allocates: yes: ${
  - `if (!result) { console.log('new leaves: ${positional[0]} has no ArtifactSchemaDescriptor companion to serve.'); return; }`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:99` [console.log] test-path: no; allocates: yes: ${
  - `console.log('new leaves: ${positional[0]} — ${dryRun ? "would serve" : "serving"} ${result.leaves.length} companion(s) from their documents; install a SchemaLeafReader (FileSystemSchemaLeaves) in every host that resol...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:100` [console.log] test-path: no; allocates: yes: ${
  - `for (const leaf of result.leaves) console.log('  ${leaf.scope}/${leaf.export}/${leaf.format} <- ${leaf.path}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:148` [console.log] test-path: no; allocates: yes: ${, .join(, .map(
  - `for (const crate of plan.crates) console.log('crate ${crate.name}  ${crate.packageDir}  lib ${crate.lib?.files}/${crate.lib?.bytes}/${crate.lib?.lines}  test ${crate.test?.files}/${crate.test?.bytes}/${crate.test?.lin...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:152` [console.log] test-path: no; allocates: yes: ${, .filter(
  - `console.log('re-carve: ${kept} carved crate(s) unchanged, ${plan.crates.filter((crate) => plan.carved!.crates.some((known) => known.signed && known.name === crate.name)).length - kept} changed, ${plan.crates.filter((c...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:153` [console.log] test-path: no; allocates: yes: ${
  - `for (const crate of supersededCrates(plan)) console.log('  superseded ${crate.name}  ${crate.packageDir}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:155` [console.log] test-path: no; allocates: yes: ${
  - `for (const note of plan.overlay.notes) console.log('note: ${note}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:176` [console.log] test-path: no; allocates: yes: ${
  - `for (const write of writes) console.log('  ${dryRun ? "(dry-run) " : ""}${write.action === "create" ? "+" : write.action === "delete" ? "-" : "~"} ${write.path}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:182` [console.log] test-path: no; allocates: yes: ${, .join(
  - `console.log('owner rows: ${owners.length ? owners.join(", ") : "none added"}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:183` [console.log] test-path: no; allocates: yes: ${
  - `console.log('new crates: ${plan.artifactCrate} — wrote ${writes.length} file(s) for ${plan.crates.length} crate(s).');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:189` [console.log] test-path: no; allocates: yes: ${
  - `const result = runUnmirror({ repoRoot: this.root, artifacts: [dirname(dirname(packageDir))], apply, log: (line) => console.log('  ${line}') });`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:191` [console.log] test-path: no; allocates: yes: ${
  - `console.log('new crates --unmirror: ${report.edits.size} file(s), ${result.manifests.size} manifest(s), ${report.removedLeaves} mirror re-export(s) removed, ${report.repointedLeaves + report.repointedChains} path(s) r...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:198` [console.log] test-path: no; allocates: yes: ${
  - `console.log('new: ${rel} — ${verb} ${created.length} file(s), ${skipped.length} already present.');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:199` [console.log] test-path: no; allocates: yes: ${
  - `for (const p of created) console.log('  ${dryRun ? "+ (dry-run)" : "+"} ${p}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/💾️cache/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/💾️cache/🟦️.ts:14` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `if(outcome.result!==undefined)console.log(JSON.stringify(outcome.result,null,2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/💾️cache/🟦️.ts:22` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(await runBuildCacheOperationV1(selected.tool,resolved.args.slice(1),{root,invocation,phaseControl},resolveCacheTool),null,2));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/📦️dependencies/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/📦️dependencies/🟦️.ts:15` [console.log] test-path: no; allocates: no
  - `export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{if(answerKernelCheckV1(args,proof))return;await runCapturedRepositoryKernelV1(args,proof,import.me...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🩺️preflight/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🩺️preflight/🟦️.ts:8` [console.log] test-path: no; allocates: no
  - `export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{if(answerKernelCheckV1(args,proof))return;await runCapturedRepositoryKernelV1(args,proof,import.me...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🧪️tests/🔁️restart/🚀️serve.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🧪️tests/🔁️restart/🚀️serve.ts:5` [console.log] test-path: yes; allocates: no
  - `console.log("listening "+server.url);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🏃️entry/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🏃️entry/🟦️.ts:6` [console.log] test-path: no; allocates: no
  - `const result=await runRemoteExecutionCommandV1(["workers",...process.argv.slice(2)],{root:process.cwd(),environment:process.env,signal:controller.signal,progress:async()=>{},write:line=>{console.log(line);},spawnWorke...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🌐️fleet/🔁️restart/🚀️serve.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🌐️fleet/🔁️restart/🚀️serve.ts:10` [console.log] test-path: yes; allocates: no
  - `console.log("listening "+server.grpcUrl);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🔧️bazel/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🔧️bazel/🟦️.ts:23` [console.log] test-path: yes; allocates: no
  - `"neg/leak.ts":'import{readFileSync}from"node:fs";console.log(readFileSync(process.env.SEMIO_WORKSPACE_ROOT+"/lib/secret.txt","utf8"));',`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🧰️cluster/🚀️serve.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🧰️cluster/🚀️serve.ts:5` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({grpc:cluster.server.grpcUrl,worker:cluster.server.workerUrl,cache:cluster.cache.url}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🔓️sealed-read/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🔓️sealed-read/🟦️.ts:47` [[DEBUG]] test-path: yes; allocates: yes: .push(
  - `const written:string[]=[],granted=await withBuildCapacityV1(args,{},async value=>value,{write:line=>written.push(line),disk:{partition:()=>{throw new Error("[DEBUG] disk admission must not run");}}});`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🖼️image-slack/🧪️tests/♻️reclaim/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🖼️image-slack/🧪️tests/♻️reclaim/🟦️.ts:235` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({state:env.SEMIO_MACHINE_STATE_DIRECTORY,rows,local}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🗃️replica-store/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🗃️replica-store/🧪️tests/🟦️.ts:109` [console.log] test-path: yes; allocates: yes: ${, Date.now, JSON.stringify
  - `writeFileSync(script,'import {fleetCompactV1} from ${JSON.stringify(replica)};import topology from ${JSON.stringify(join(sharedWorktree,"🗺️topology/🔣️.json"))} with {type:"json"};await fleetCompactV1(process.argv[2],"...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/👥️processes/📜️child.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/👥️processes/📜️child.ts:11` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({pid:process.pid,status:result.status,label:labelOfTextV1(result.text),captures:result.captures}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🗂️store/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🗂️store/🟦️.ts:172` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const program='import{readFileSync}from"node:fs";import{createHash}from"node:crypto";import{relative,join}from"node:path";import{v}from"./helper.ts";import{sealedTreeDriftV1}from'+JSON.stringify(core)+';export async f...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🧫️fixtures/📜️trampoline.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🧫️fixtures/📜️trampoline.ts:10` [console.log] test-path: yes; allocates: yes: .join(
  - `await startImplementationV1(root,argv,process.env,async()=>{console.log("LIVE "+argv.join(","));},{entry:"tool/entry.ts",promote,...(rule==="explicit"||rule==="drift"?{policy:{...sealedPolicyV1,promotion:{...sealedPol...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🧫️fixtures/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🧪️tests/🧫️fixtures/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `writeFileSync(join(root,"tool/entry.ts"),'import{writeSync}from"node:fs";import{v}from"./helper.ts";import d from"./data.json"with{type:"json"};export async function runCapturedBuildSourceV1(args:string[],proof:{loade...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🧱️hermetic/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🧱️hermetic/🧪️tests/🟦️.ts:119` [console.log] test-path: yes; allocates: no
  - `const TS={"src/index.ts":'import {answer} from "./lib.ts";\nconsole.log(answer());\n',"src/lib.ts":"export const answer=():number=>42;\n","extra/undeclared.ts":"export const secret=1;\n"};`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🧪️tests/🌊️burst/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🧪️tests/🌊️burst/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({result,peakBytes:peak}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🧪️tests/📊️bench/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🧪️tests/📊️bench/🟦️.ts:34` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({files:paths.length,dirs,cursorMs:Math.round(cursorMs),statWalk:brief(before),journalQuiet:{...brief(after),via:after[0]!.value.via,statted:after[0]!.value.statted,journalled:after[0]!.value...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🏊️delegate/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🏊️delegate/🟦️.ts:15` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `else{console.log(JSON.stringify({argv:process.argv.slice(2),cwd:process.cwd(),env:process.env.TEST_VAR??null,removed:process.env.TEST_GONE??null}));console.error("to stderr");if(process.argv.includes("--fail"))process...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/💓️keepalive/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/💓️keepalive/🟦️.ts:143` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(outcomes));';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/📐️booking/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/📐️booking/🟦️.ts:93` [console.log] test-path: yes; allocates: yes: .filter(, JSON.stringify
  - `console.log(JSON.stringify(frames.filter(row=>row.phase==="slice")));';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🚪️client/🔇️silence/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🚪️client/🔇️silence/🟦️.ts:106` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `for(const row of cases)console.log(JSON.stringify(await observe(row)));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🚪️client/🔇️silence/🟦️.ts:107` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(await observeHeldPlan()));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🚪️client/🔇️silence/🟦️.ts:108` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(await observeReattachedPlan()));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🚪️client/🔇️silence/🟦️.ts:109` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `for(const row of handoff.interruptions)console.log(JSON.stringify(await observe(cases.find(row=>row.name==="exact slice progress")!,row.mode)));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🧽️plan-answered/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🧽️plan-answered/🟦️.ts:33` [console.log] test-path: yes; allocates: no
  - `console.log("[R2 measure] start "+start.toFixed(1)+" MB, plan footprint "+footprint.toFixed(1)+" MB, after plan 1 "+afterOne.toFixed(1)+" MB, after plan 2 "+afterTwo.toFixed(1)+" MB");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/⚡️incremental/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/⚡️incremental/🟦️.ts:39` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(process.env.SEMIO_G1_MODE==='oracle'){console.log(JSON.stringify({result:await run(fixture.concurrency.serial),counts:counts()}));return;}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/⚡️incremental/🟦️.ts:48` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `out.halfWritten.persisted={records:records.length,foreignLibraryFacts,foreignPaths:[...foreignPaths],authorityRows};console.log(JSON.stringify(out));}));})().catch(error=>{console.error(error);process.exit(1)});';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🌊️owner-scale/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🌊️owner-scale/🟦️.ts:16` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({rows:rows.length,ordered:rows.every((row,at)=>row.path===paths[at]),grewMegabytes:Math.round((peak-before)/1048576)}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📦️projects/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📦️projects/🟦️.ts:28` [console.log] test-path: yes; allocates: yes: .filter(, .map(, .push(, JSON.stringify
  - `const source=nodeTypeScriptLoaderV1(sdk.resolve("typescript"))+'const{readFileSync,writeFileSync,mkdirSync,renameSync,rmSync}=require('node:fs'),{join,dirname}=require('node:path');(async()=>{const routing=await impor...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📦️projects/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify
  - `const source=nodeTypeScriptLoaderV1(sdk.resolve("typescript"))+'const fs=require('node:fs'),fsp=require('node:fs/promises'),{join}=require('node:path');(async()=>{const routing=await import(process.env.SEMIO_PROJECT_R...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🔒️loaded-implementation/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🔒️loaded-implementation/🟦️.ts:54` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({hash,fresh,old}));');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🔒️loaded-implementation/🟦️.ts:70` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({loaded,before,untouched,finished,current:hash(),other,error}));');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🟦️.ts:340` [console.log] test-path: yes; allocates: no
  - `const oracle=Bun.spawnSync([registeredToolExecutableV1("node"),"--input-type=module","-e",'const c=new AbortController();try{await Promise.resolve();c.abort(Error(process.argv[1]));c.signal.throwIfAborted();console.lo...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧺️presence-batch/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧺️presence-batch/🟦️.ts:249` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("[bench] presence sessions for 100 batches at 16 s open+close, 1 s per batch: before "+JSON.stringify(before)+" after "+JSON.stringify(after));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧺️scope-rows/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧺️scope-rows/🟦️.ts:42` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("[bench]",JSON.stringify({owners:sizes.owners,rowsPerOwner:sizes.rows,distinct,observed:(outer.observations as Row[]).length,scopeEntries:entries,heapMB:Math.round((held-before)/1048576),ms:Math.round(elap...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🪪️parser-identity/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🪪️parser-identity/🟦️.ts:30` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({before:said,verdict:last.verdict,why:last.staleRow?.why}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👷️workers/📨️protocol/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👷️workers/📨️protocol/🟦️.ts:113` [console.log] test-path: no; allocates: no
  - `const savedConsole={log:console.log,info:console.info,debug:console.debug,warn:console.warn,error:console.error};`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👷️workers/📨️protocol/🟦️.ts:114` [console.log] test-path: no; allocates: yes: ${
  - `console.log=console.info=console.debug=(...values:unknown[])=>{stdout.write('${formatWorkerLineV1(values)}\n');};console.warn=console.error=(...values:unknown[])=>{stderr.write('${formatWorkerLineV1(values)}\n');};`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🛂️acceptance/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🛂️acceptance/🧪️tests/🟦️.ts:369` [console.log] test-path: yes; allocates: yes: ${, Date.now
  - `writeFileSync(script,'const keep=Buffer.alloc(300*1024*1024,1);console.log("graph: 3 projects served from the memo of 2 s ago");const begun=Date.now();await new Promise(accept=>setTimeout(accept,2200));console.log("IN...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🛂️acceptance/🧪️tests/🟦️.ts:389` [console.log] test-path: yes; allocates: no
  - `writeFileSync(stub,'import {readFileSync} from "node:fs";const [,,root,file]=process.argv;console.log("graph: 3 projects served from the memo of 2 s ago");await new Promise(accept=>setTimeout(accept,500));console.log(...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🎭️stub/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🎭️stub/🟦️.ts:7` [console.log] test-path: yes; allocates: no
  - `if(words[0]==="--help"){await sleep(number("BENCH_STUB_HELP_MS",20));console.log("usage: repo <command>");process.exit(0);}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🎭️stub/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${
  - `console.log(keep?'done ${keep.length}':"done");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🟦️.ts:121` [console.log] test-path: yes; allocates: no
  - `if(numpy===null)console.log("numpy is not installed: third-party percentile check skipped");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🟦️.ts:795` [console.log] test-path: yes; allocates: no
  - `const script='const w=ms=>new Promise(r=>setTimeout(r,ms));console.log("[capacity] waiting 0s for run: not enough free memory");await w(200);console.log("[1 / 3] checking cached actions");await w(150);console.log("INF...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧭️planning/🏃️runner/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧭️planning/🏃️runner/🟦️.ts:130` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(report));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📐️conformance/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📐️conformance/🧪️tests/🟦️.ts:240` [console.log] test-path: yes; allocates: no
  - `console.log(renderConformanceReportV1(report,set,"en"));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/🟦️.ts:43` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[storybook] UI discovery guard passed (${stories} stories, ${docs} docs, ${inputs.size} TS/TSX inputs, 0 MDX).');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts:38` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[build] Completed the ${mode} workspace build graph.');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🧪️tests/📥️birth/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🧪️tests/📥️birth/📜️script.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const program="const x=JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION);console.log(JSON.stringify({policy:x.policy,deadline:x.deadlineEpochMilliseconds,capabilities:x.capabilities,argv:process.argv.slice(1)}))...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/📦️snapshot/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/📦️snapshot/🧪️tests/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: .join(, .map(, .push(, JSON.stringify
  - `await withRepositoryNxStorageV1(workspace,original,snapshotPhase,async(bind,nxPhase)=>{const namespace=await bind(identity,null,(await import("../../../../../🟨️.mjs")).nxBuildDependencyImplementationV1()),child="(asyn...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/🧪️tests/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: .join(
  - `const source='(async()=>{const fs=require('node:fs'),path=require('node:path'),sdk=require('node:module').createRequire(process.env.SEMIO_TEST_SDK_PACKAGE),native=sdk('nx/src/native'),fixture=JSON.parse(process.env.SE...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/🧪️tests/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .map(, .push(
  - `const observations=[];for(const row of fixture.cases){const root=path.join(directory,row.id),physical=path.join(root,'source'),empty=path.join(root,'empty');fs.mkdirSync(physical,{recursive:true});fs.mkdirSync(empty);...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️selected/🧪️tests/🟦️.ts:23` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `try{const result=await captureOwnedCommandV1("node",["-e",source],workspace,{control,env:{...process.env,...namespace.environment,SEMIO_TEST_SDK_PACKAGE:join(testNxToolkitDirectoryV1(identity),"package.json"),SEMIO_TE...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts:69` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const nodeSource=crashSource.slice(0,crashSource.indexOf("const mutate="))+'const{DatabaseSync}=await import('node:sqlite');let committedDirty;const read=()=>{const db=new DatabaseSync(${JSON.stringify(path)},{readOnl...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `try{parent=await openNativeImageStorageV1(options);const image=await openNativeImageStorageV1(options);try{expect(await maintainBuildGraphOptionalCachesV1(image,control)).toBe(false);await parent.recheck();}finally{aw...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📋️data/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📋️data/🧪️tests/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify
  - `test("actual shared inventory DATA projection agrees with ordinary Node",()=>withBuildTestInvocationV1(async original=>{const control=createIssuedResourceControlV1(original,"inventory:node-oracle",30000),rows=fixture....`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📋️data/🧪️tests/🟦️.ts:51` [console.log] test-path: yes; allocates: yes: .join(, .map(, .push(, JSON.stringify
  - `test("actual loaded inventory helper and private policy refuse same-process Source drift",()=>withBuildTestInvocationV1(async original=>{const source=readFileSync(new URL("../📥️load/🟨️.mjs",import.meta.url),"utf8"),tr...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: ${, .repeat(, JSON.stringify
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual revision sink required");const root=await mkdtemp(join(sink,"rv-")),routing=fileURLToPath(new URL("../../../../../../../../🔨️modules/🏃️proces...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: ${, Date.now, JSON.stringify, new Date
  - `const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual physical revision sink required");const root=await mkdtemp(join(sink,"rv-")),routing=fileURLToPath(new URL("../../../../../../../../🔨️modules...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🧊️own-stamp/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧪️tests/🧊️own-stamp/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `const program='const fs=require("node:fs");const m=await import(${JSON.stringify(pathToFileURL(made.own).href)});const out=[];\nconst run=file=>{const before=globalThis.__lstat;try{out.push({sha:m.captureBoundedSource...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts:43` [console.log] test-path: no; allocates: no
  - `else console.log(text);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🪟️windows/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🧪️tests/🪟️windows/🟦️.ts:29` [console.log] test-path: yes; allocates: no
  - `const real = await captureV1([process.execPath, "-e", "console.log('ok')"], {cwd: process.cwd(), env: {PATH: process.env.PATH}, timeoutMs: 20000});`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/📏️windows/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/📏️windows/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: .map(, .push(
  - `test("exact Source window and JSON reduction children consume actual FD operations",()=>withBuildTestInvocationV1(async original=>{const checks:Array<readonly[unknown,unknown]>=[],oracle=await captureOwnedCommandResul...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🧪️tests/🟦️.ts:30` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const contract={version:1,files:corpus.paths},actual=await sourceClosureDigest(root,contract,control),oracleCode=String.raw'const fs=require('node:fs'),c=require('node:crypto'),p=require('node:path'),root=process.argv...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🧪️tests/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: ${, .push(, Array.from, JSON.stringify
  - `const program='const{receiveScriptProcessInvocation,createIssuedResourceControlV1}=await import(${JSON.stringify(routing)});const{sourceClosureDigest}=await import(${JSON.stringify(owner)});const{admitExactSourceExten...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts:98` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const keyed=[file("entry.ts",'import {k} from "./keyed.ts";import {s} from "./${prefix}probe.ts";export async function runCapturedBuildSourceV1(args:string[],proof:{sha256:string}){console.log(JSON.stringify([k,s,args...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/✅️conformance/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/✅️conformance/📜️script.ts:57` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({version:1,backend:expected,completed:true}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/🎚️capacity/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/🎚️capacity/📜️script.ts:18` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({version:1,phase,completed:true}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👁️existing/📜️script.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👁️existing/📜️script.ts:16` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(mode==="writer"){const directory=process.argv[4]!,resource=process.argv[5]!,expectation=process.argv[6]!,lease=await acquireResourceLease({directory,resource,mode:"exclusive",control,signal:control.signal,timeoutMs...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👁️existing/📜️script.ts:22` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(mode!=="positive"){const messages={missing:"Existing initialized Native journal required",partial:"Complete existing Native journal required",foreign:"Exact existing Native observation policy required"};try{await o...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👁️existing/📜️script.ts:28` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({version:1,readerShared,writerExcluded,unchanged,usedBytes:nativeStorageUsedBytesV1(result!.state),snapshot:result!}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👓️reader-lifetime/📜️script.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👓️reader-lifetime/📜️script.ts:20` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const retired=await retireNativePlatformStorageV1(options,{version:1,wait:true});console.log(JSON.stringify({phase,id,used,retired}));return;`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👓️reader-lifetime/📜️script.ts:24` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const charge=await acquirePreparedNativeStorageOwnerV1({policy,control:consumer,port,owner:"test:native-reader",kind:"compiler-workspace",bytes:fixture.chargeBytes},preparation);const committed=(await observeNativeSto...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👓️reader-lifetime/📜️script.ts:26` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if("preparationExpired" in row&&row.preparationExpired){if(!preparation.signal.aborted)await once(preparation.signal,"abort",{signal:consumer.signal});console.log(JSON.stringify({phase:"expired",id,aborted:preparation...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👓️reader-lifetime/📜️script.ts:27` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `try{await image.port.withOperation(consumer,()=>charge.recheck());console.log(JSON.stringify({phase:"result",id,usable:true,bytes:charge.allocation.bytes}));}catch(error){console.log(JSON.stringify({phase:"result",id,...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👶️lifecycle/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👶️lifecycle/📜️script.ts:18` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(!childRemoved||!parentRetired)throw Error("Actual complete child and parent retirement required");console.log(JSON.stringify({version:1,platform:process.platform,childAttached,childRemoved,parentRetired,bodyClosed,...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📁️directory/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📁️directory/📜️script.ts:20` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `});console.log(JSON.stringify({version:1,completed:true}));}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/⏱️tools/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/⏱️tools/📜️script.ts:25` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `console.log(JSON.stringify({version:1,mode:"tools-read-observation",timings,usage,backing:{maximumPhysicalBytes:canonical.maximumPhysicalBytes,effectiveCapacityBytes:snapshot.effectiveCapacityBytes},journal:snapshot.j...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/⏱️tools/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/⏱️tools/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const text=Buffer.concat(chunks).toString("utf8"),lines=text.trim().split("\n"),result=JSON.parse(lines.at(-1)!);expect(result.version).toBe(1);expect(result.mode).toBe("tools-read-observation");expect(result.timings....`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/📜️script.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/📜️script.ts:25` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(!facts)throw Error("Actual owned plan was never entered");if(mode==="pending"&&!pendingFinished)throw Error("Plan operation escaped input/storage scope");console.log(JSON.stringify({mode,...facts,receiverPid:proces...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/📜️script.ts:26` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `});}catch(error){console.log(JSON.stringify({mode,refused:true,consumed,message:error instanceof Error?error.message:String(error)}));throw error;}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📜️script.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📜️script.ts:31` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `});console.log(JSON.stringify(outcome));return;}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📜️script.ts:34` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const entry=await captureRuntimeEntryV1(declaration,body),retained=retainRuntimeEntryCleanupV1(entry,cleanup,body);await beginAcceptedPhysicalCleanupV1(cleanup);try{await fault.terminal(cleanup,()=>{requireAcceptedPhy...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📜️script.ts:39` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `}).catch(error=>{if(!deadlineRefused||!/deadline exhausted/u.test(String(error)))throw error;const state=observeNativeDirectoryStateV1(directory);if(state!=="inactive")throw Error("Failed child must preserve inactive ...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📡️request/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📡️request/📜️script.ts:17` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `if(import.meta.main)await receiveScriptProcessInvocation(process.env,async original=>{const workspace=process.cwd(),control=createIssuedResourceControlV1(original,"test:nx-request-runtime-entry",90000),mode=process.ar...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/🧹️whole/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/🧹️whole/📜️script.ts:9` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(import.meta.main)await receiveScriptProcessInvocation(process.env,async original=>{const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original test artifact directory required");const ro...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🛂️readmission/🧪️tests/🏃️tasks/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🛂️readmission/🧪️tests/🏃️tasks/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `await writeFile(entry,'import{value}from"./target.ts";void value;import{buildSourceClosureIdentityV1,captureBuildSourceClosureV1}from'+specifier(closure)+';import{receiveScriptProcessInvocation}from'+specifier(routing...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/📥️stdin/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/📥️stdin/🟦️.ts:30` [console.log] test-path: yes; allocates: yes: .repeat(, JSON.stringify
  - `try{for(const row of fixture.cases){const source='import{answer}from"./answer.ts";export async function runCapturedBuildSourceV1(args,proof){proof.recheck();console.log(JSON.stringify({identity:import.meta.semioCaptur...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/📥️stdin/🟦️.ts:44` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const root=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!,"node-interactive"),control=createIssuedResourceControlV1(original,"test:node-interactive",60000),source='export async function runCapturedBuildSourceV1(){consol...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: .join(, .map(, .push(, JSON.stringify
  - `async function capturedParentNodeArgumentsV1(root:string,snapshot:unknown,control:Parameters<typeof captureOwnedCommandResultV1>[3]["control"]):Promise<ReturnType<typeof buildSourceClosureNodeArgumentsV1>>{const works...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🧪️tests/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: .filter(, .map(, .repeat(, JSON.stringify
  - `let failed=false,primary:unknown;try{const bootstrapImports=(text:string)=>new Bun.Transpiler({loader:"js"}).scan(text).imports.map(row=>row.path).filter(path=>path.startsWith("node:")||fixture.bootstrapContract.forbi...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts:28` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts||!isAbsolute(artifacts)||join(artifacts,"al-xxxxxx","helper.ts").length>256)throw Error("Bounded issued auxiliary fixture artifacts required");const ar...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: .map(, .push(, JSON.stringify
  - `try{const files=fixture.commonJsCases.map(row=>({path:row.owner+".cjs",text:"module.exports={"+row.member+":value=>value};",imports:{}})),entryText='import ownership from"./bun.cjs";import payload from"./payload.cjs";...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts:44` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `try{const specifier=(url:URL)=>{const local=relative(area,fileURLToPath(url)).replaceAll("\\","/");return local.startsWith(".")?local:"./"+local;},entryText='import{parseBunWorkspaceDeclaration}from'+JSON.stringify(sp...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts:124` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const area=makeTemporaryV1(join(temporaryDirectoryV1(),"lazy-")),files=[{path:"entry.ts",body:'import{h}from"./h.ts";export async function runCapturedBuildSourceV1(args,proof){console.log(JSON.stringify({args,h,sha:pr...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🔏️selection/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🔏️selection/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: .map(, .push(, .repeat(, JSON.stringify
  - `test("actual admitted compiler binds child Source JSON and helper images under its Original",async()=>{await withBuildTestInvocationV1(async original=>withRepositoryTestArtifactsV1(repository.root,original,createIssue...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify
  - `test("actual admitted child captures a nested Source query without directory resolution and keeps its images immutable under a physical rewrite",()=>withBuildTestInvocationV1(async original=>{const workspace=process.c...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts:31` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const workspace=process.cwd(),artifacts=await closureFixtureRootV1(workspace),origin=physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL);if(!isAb...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🚫️cache-root-refusal/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🚫️cache-root-refusal/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const entry='import{captureBuildSourceClosureV1}from'+JSON.stringify(owner)+';import{receiveScriptProcessInvocation}from'+JSON.stringify(routing)+';import"./worker.ts";export async function runCapturedBuildSourceV1(ar...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("RESULT "+JSON.stringify(out));}';`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts:63` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `put("node-entry.mjs",'import fake from"fake-pkg";import{createRequire}from"node:module";export async function runCapturedBuildSourceV1(){console.log("NODE "+JSON.stringify({fake:fake.value}));}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🧾️mismatch/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🧾️mismatch/🟦️.ts:23` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await writeFile(target,corpus.before);await writeFile(entry,'import{value}from"./target.ts";void value;import{captureBuildSourceClosureV1}from'+JSON.stringify(specifier(fileURLToPath(sourceURL)))+';import{receiveScrip...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🌌️whole/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🌌️whole/🟦️.ts:108` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log(results.join("\n"));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🕳️excluded-names/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🕳️excluded-names/🟦️.ts:78` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log(results.join("\n"));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts:114` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log(results.join("\n"));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts:69` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('[ignore-sources] ${scenario.name}: universe changed ${after!==before}, witness ${JSON.stringify(verdict)}, tainted ${JSON.stringify(witness.tainted)}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts:165` [console.log] test-path: no; allocates: yes: ${
  - `if(task.definition.continuous)console.log('[bazel] task-started ${task.id}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/👷️worker/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/👷️worker/🟦️.ts:39` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("REPLY " + JSON.stringify({ a, root, reply: JSON.parse(reply), mode }));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️classes/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️classes/🟦️.ts:41` [console.log] test-path: yes; allocates: no
  - `if(verb==="test"){const{answer}=await import("@fx/lib"),pad=(await import("leftpad-fx")).default;console.log(pad(readFileSync(join(here,"data/in.txt"),"utf8").trim(),6)+answer);return;}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️classes/🟦️.ts:45` [console.log] test-path: yes; allocates: no
  - `if(verb==="check-leak"){console.log(readFileSync(join(root,"secret.txt"),"utf8"));return;}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️classes/🟦️.ts:148` [console.log] test-path: yes; allocates: no
  - `delete process.env.SEMIO_TEST_LEVEL;if(process.env.SEMIO_TEST_KEEP)console.log("kept "+directory);else{makeTestTreeWritableV1(directory);rmSync(directory,{recursive:true,force:true});}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts:23` [console.log] test-path: yes; allocates: no
  - `"neg/leak.ts":'import{readFileSync}from"node:fs";console.log(readFileSync(process.env.SEMIO_WORKSPACE_ROOT+"/lib/secret.txt","utf8"));',`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️shim/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🪟️shim/🟦️.ts:36` [console.log] test-path: yes; allocates: no
  - `writeFileSync(join(directory,"tool","bin","tool.js"),"console.log(1)\n");writeFileSync(join(directory,"tool","bin","tool.exe"),"MZ");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/📥️runtime/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/📥️runtime/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const source='const{existsSync,readFileSync,writeFileSync}=require("node:fs");const[path,counter,payload]=process.argv.slice(1);writeFileSync(counter,String(existsSync(counter)?Number(readFileSync(counter,"utf8"))+1:1...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🔒️real-bazel/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🔒️real-bazel/🟦️.ts:25` [console.log] test-path: yes; allocates: no
  - `"neg/leak.ts":'import{readFileSync}from"node:fs";console.log(readFileSync(process.env.SEMIO_WORKSPACE_ROOT+"/lib/secret.txt","utf8"));',`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🔒️real-bazel/🟦️.ts:65` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify
  - `const digests=snapshot.entries.filter((entry:any)=>entry.kind==="file"&&entry.path!=="app/dist/where.txt").map((entry:any)=>[entry.path,entry.sha256]);console.log("DIGESTS "+JSON.stringify(digests));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts:104` [console.log] test-path: no; allocates: yes: ${, .filter(, JSON.stringify
  - `const lock=await readFile(lockPath,"utf8"),files=(await readdir(blobs)).filter(value=>/^[a-f0-9]{64}$/u.test(value)).sort();await writeChanged(receiptPath,JSON.stringify({version:1,identity,lockSha256:hash(lock),blobs...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts:651` [console.log] test-path: no; allocates: no
  - `if(verbose||snapshot.vanished.length)console.log(renderBazelSourceSnapshotV1(snapshot,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts:803` [console.log] test-path: no; allocates: no
  - `const mode=readWorldWitnessSwitchV1(process.env),language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,say=(table:"Messages"|"Reasons",name:string,values:Record<string,string|number>={})=>console.lo...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts:833` [console.log] test-path: no; allocates: yes: .filter(, .join(, .map(, Date.now
  - `await profilePhaseV1("bazel.planner",async()=>{if(!reuse.record){await runPlanner(false);return;}try{await runPlanner(true);}catch(error){if(phaseControl.signal.aborted)throw error;sayBuildV1("bazel.plannerRetry",{rea...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts:928` [console.log] test-path: no; allocates: no
  - `await reapBazelServersV1(directories,port,servers,phaseControl.signal,base=>stopBazelServerV1(base,phaseControl.signal),{budget:bazelBaseServersV1(host).slots,progress:line=>console.log(line),language}).catch(error=>{...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts:939` [console.log] test-path: no; allocates: yes: ${, collect
  - `await collectBazelSessionsV1(hold.directories,hold.port,hold.servers,phaseControl.signal,base=>stopBazelServerV1(base,phaseControl.signal),{holding:hold.session.slot,keep:hold.session.key,progress:line=>console.log(li...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔌️backend/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔌️backend/🟦️.ts:15` [console.log] test-path: no; allocates: no
  - `let last=0;const bazel=await bootstrap.prepareBazel(root,storage.tools,actualPhase,event=>{publish(event.phase);const step=bazelProgressLineV1(event,last);last=step.lastPercent;if(step.line)console.log(step.line);},of...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔌️backend/🟦️.ts:17` [console.log] test-path: no; allocates: yes: ${, JSON.stringify
  - `publish("ready");console.log(report.line);console.log('[bazel] readiness ${JSON.stringify(report.readiness)}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔌️backend/🟦️.ts:44` [console.log] test-path: no; allocates: no
  - `const {withRepositoryBuildReportV1}=await import("../../../📊️reporting/🟦️.ts");await withRepositoryBuildReportV1(root,output,plan,invocation,async artifact=>{console.log(artifact.path);});`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔌️backend/🟦️.ts:45` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `}else console.log(JSON.stringify(plan,null,2));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️control/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️control/🟦️.ts:23` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `try{await ready;if(row.name==="nested"){const path=join(directory,"📜️script.ts"),routingPath=resolve(import.meta.dir,"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");await writ...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🔏️implementation/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🔏️implementation/🟦️.ts:23` [console.log] test-path: yes; allocates: yes: .filter(, JSON.stringify
  - `async function admittedBazelFixtureV1(root:string,entry:string,original:ScriptInvocation):Promise<{capture:BuildSourceClosureV1;identity:string}>{const workspace=process.cwd(),driver=join(root,"capture-driver.ts"),con...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/📜️script.ts:10` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `checkScriptInvocation(original);console.log(JSON.stringify(Object.fromEntries(Object.entries(oracle.dependencies).map(([id,edges])=>[id,(edges as string[]).sort()]))));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🟦️.ts:105` [console.log] test-path: yes; allocates: yes: ${, Date.now, JSON.stringify
  - `await writeFile(script,'const root=import.meta.dir;if(process.argv[2]==="service"){await Bun.write(root+"/service-pid.json",JSON.stringify({pid:process.pid}));console.log("[TRACE] fixture service ready");setInterval((...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🟦️.ts:110` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const negative=async(variant:"cancel"|"parallel-failure"|"refusal-early"|"refusal-ended",reason:string)=>{const context=join(project,"purpose-"+variant+".json"),receipt=join(project,"purpose-"+variant+"-receipt.json")...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🟦️.ts:138` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await mkdir(project,{recursive:true});await writeFile(join(project,"📜️script.ts"),'await Bun.write(process.env.SEMIO_ARGV_OUTPUT,JSON.stringify(process.argv.slice(2)));console.log("[TRACE] Literal argv receiver execut...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🔌️port/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🔌️port/🟦️.ts:52` [console.log] test-path: no; allocates: no
  - `progress:line=>{(options.write??console.log)(line);},`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts:284` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code='import {bazelSlotPortV1} from ${JSON.stringify(join(import.meta.dir,"../🔌️port/🟦️.ts"))};const port=bazelSlotPortV1(${JSON.stringify(directories)});const claim=await port.claim(${JSON.stringify(slot)},new ...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🟦️.ts:391` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code='import {bazelSlotPortV1} from ${JSON.stringify(join(import.meta.dir,"../🔌️port/🟦️.ts"))};import {acquireBazelSessionV1} from ${JSON.stringify(join(import.meta.dir,"../🟦️.ts"))};const port=bazelSlotPortV1($...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🔌️backend/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🔌️backend/🟦️.ts:115` [console.log] test-path: no; allocates: yes: ${
  - `if(event.kind==="end")console.log('[semio] ${event.status} ${event.task} (${event.done}/${event.total})');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🔌️backend/🟦️.ts:137` [console.log] test-path: no; allocates: no
  - `async setup(context:BackendContextV1){console.log(both("[semio] The in-house build engine needs no external tool to provision.","[semio] Die hauseigene Build-Engine benötigt kein externes Werkzeug."));checkResourceLea...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🔌️backend/🟦️.ts:153` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(buildTaskInputsV1(graph,plan.tasks[plan.selected[0]!]!),null,2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🔌️backend/🟦️.ts:160` [console.log] test-path: no; allocates: no
  - `if(output&&output!=="stdout"&&output!=="true"){const {withRepositoryBuildReportV1}=await import("../../../📊️reporting/🟦️.ts");await withRepositoryBuildReportV1(root,output,plan,invocation,async artifact=>{console.log(...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🔌️backend/🟦️.ts:161` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `else console.log(JSON.stringify(plan,null,2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🔌️backend/🟦️.ts:164` [console.log] test-path: no; allocates: no
  - `if(untouched){console.log("[semio] no affected tasks");return;}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🧪️tests/🟦️.ts:195` [console.log] test-path: yes; allocates: no
  - `console.log(owner?"owner":"waiter");');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🧪️tests/🟦️.ts:265` [console.log] test-path: yes; allocates: no
  - `try{adapter=await import("../../../🔧️tools/🏗️bazel/📜️script.ts");}catch(error){console.log("skipped: the Bazel adapter could not be loaded here",String(error).slice(0,120));return;}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🐙️semio/🧪️tests/🟦️.ts:457` [console.log] test-path: yes; allocates: yes: .push(
  - `const ok=await run(make({program:process.execPath,args:["-e","console.log('hi')"]}),new AbortController().signal,chunk=>output.push(chunk));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/🔌️backend/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/🔌️backend/🟦️.ts:27` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(buildExecPlanV1(selection.selectBuildProjectsV1(graph,listed(["projects","p"]),listed(["exclude"]))),null,2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/🔌️backend/🟦️.ts:45` [console.log] test-path: no; allocates: yes: .join(
  - `if(result.state==="restored"||result.state==="partial")console.log("[store-receipt] "+texts("restored").split("\n").join("\n[store-receipt] "));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/🔌️backend/🟦️.ts:46` [console.log] test-path: no; allocates: yes: .join(
  - `if(result.state==="partial"||result.state==="unavailable")console.log("[store-receipt] "+texts("unavailable").split("\n").join("\n[store-receipt] "));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🪦️reaper/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🪦️reaper/🟦️.ts:68` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code='const {spawn}=require("node:child_process");const c=spawn(process.execPath,[${JSON.stringify(entry)},process.argv[1]],{detached:true,stdio:"ignore",env:{...process.env,SEMIO_OWNER_POOL_CHILD:"1"}});c.unref...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🗃️child-custody/👷️child.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🗃️child-custody/👷️child.ts:11` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("REPORT "+JSON.stringify({report,staging:join(root,"staging","observe")}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🪦️busy-orphan/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🪦️busy-orphan/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code='const {spawn}=require("node:child_process");const c=spawn(process.execPath,[${JSON.stringify(worker)}],{stdio:["ignore","inherit","inherit","ipc"],env:{...process.env,SEMIO_GRAPH_WORKER_INIT:${JSON.stringi...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🔱️warm/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🔱️warm/🧪️tests/🟦️.ts:94` [console.log] test-path: yes; allocates: no
  - `const one=await timed(1),many=await timed(K);console.log("[warm] 48 owners x "+spin+" ms: 1 lane "+one.toFixed(2)+" s, "+K+" lanes "+many.toFixed(2)+" s, speed-up "+(one/many).toFixed(2));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🪘️refs-file/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🪘️refs-file/🟦️.ts:81` [console.log] test-path: yes; allocates: no
  - `console.log("[R121] parent adopt CPU per owner",perOwner.toFixed(2),"ms, rss growth",growth.toFixed(1),"MB over",OWNERS,"owners, children",pids.size);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧮️conformance/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧮️conformance/🧪️tests/🟦️.ts:216` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `writeFileSync(script,'const root=${JSON.stringify(join(target,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"))},base=root+"/"+${JSON.stringify(own)};const core=await import(base+"🟨️.mjs"),{RUNNERS_V1}=await impor...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🧮️conformance/🧪️tests/🟦️.ts:224` [[DEBUG]] test-path: yes; allocates: yes: JSON.stringify
  - `writeFileSync(join(target,file),file.endsWith(".json")?JSON.stringify(JSON.parse(original),null,4)+"\n":"// [DEBUG] body-only edit\n"+original+"\n// trailing comment\n");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/♻️owner-reuse/🏭️build.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/♻️owner-reuse/🏭️build.ts:28` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("owner-reuse "+JSON.stringify({inferred,closures:scriptInputs.closureEntryCountsV1(),rosters}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🌊️layer-stream/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🌊️layer-stream/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({text,digests:saved.digests}));';`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🌊️layer-stream/🟦️.ts:47` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `Bun.gc(true);console.log(JSON.stringify({growth:held()-before,read}));';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🎓️course-inputs/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🎓️course-inputs/🟦️.ts:111` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({name,oracle:row.reads.size,python:python.size,onlyPython:onlyPython.length,onlyOracle:onlyOracle.length}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🎯️target-fanout/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🎯️target-fanout/🟦️.ts:60` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("[DATA] target fan-out "+JSON.stringify({targets:fixture.targets,stored,growthMegabytes:Math.round(growth*10)/10,slopeKilobytesPerTarget:Math.round(slope),last:Math.round(heaps[heaps.length-1]!/MEGABYTE)}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🟦️.ts:66` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ steps, rosters }));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧩️group-merge/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧩️group-merge/🟦️.ts:46` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({text,grown:(after-before)*1024,groups:Object.keys(merged.namedInputs).length,first:merged.targets.t7.inputs[0],last:merged.targets.t1499.inputs[0]}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧬️read-verdicts/👷️child/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧬️read-verdicts/👷️child/🟦️.ts:9` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("CHILD "+JSON.stringify({probes:OB.observationProbeCountsV1().readdir-before,layers:SI.layerVerdictCountsV1(),entries:SI.closureEntryCountsV1(),verdicts:SI.closureVerdictCountsV1?.()}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🟦️.ts:102` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const script="const{receiveScriptProcessInvocation}=await import("+JSON.stringify(import.meta.resolve("@semio-tech/process/routing"))+");await receiveScriptProcessInvocation(process.env,async()=>{const m=await import(...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🫧️cargo-yield/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🫧️cargo-yield/🟦️.ts:50` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `expect([...answer].map((entry:[string,string])=>entry[0]).sort()).toEqual(crates);console.log("[DATA] cargo walk "+JSON.stringify({directories:fixture.branches*fixture.directoriesPerBranch,turnMs:Math.round(longest)})...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🫧️flat-heap/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🫧️flat-heap/🟦️.ts:54` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("[DATA] flat heap "+JSON.stringify({owners:fixture.owners,warmup:fixture.warmup,growthMegabytes:Math.round(growth*10)/10,slopeKilobytesPerOwner:Math.round(slope),first:Math.round(heaps[0]!/MEGABYTE),last:M...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests/🟦️.ts:31` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `writeFileSync(join(area,"run.ts"),'await import("./router.ts");console.log(JSON.stringify(globalThis.__touched??[]));\n');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🧱️layer-lost/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🧱️layer-lost/🟦️.ts:98` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({count,removed}));';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🫧️take-release/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🫧️take-release/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({peak:mb(peak),nodes:scripts.walkNodes.size}));';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts:145` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `writeFileSync(parent,'const{spawn}=require("node:child_process");const c=spawn(process.execPath,[process.argv[2],process.argv[3],process.argv[4]],{stdio:["pipe","ignore","pipe"]});c.stdin.end(JSON.stringify({preflight...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🧪️tests/🟦️.ts:31` [console.log] test-path: yes; allocates: yes: ${, .filter(, .map(, JSON.stringify
  - `writeFileSync(script,'const stamps=await import(${JSON.stringify(join(library,"🏗️build/🕸️graph/📥️inference/🔖️stamps/🟨️.mjs"))}),{observeDetachedV1}=await import(${JSON.stringify(join(library,"🏗️build/👁️observation/🟨️....`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔭️coverage/🧪️tests/🤝️queue/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔭️coverage/🧪️tests/🤝️queue/🟦️.ts:95` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const child=Bun.spawn([process.execPath,"-e",'const{createFileLeasePortV1}=await import(${JSON.stringify(module)});await createFileLeasePortV1(${JSON.stringify(leases)}).exclusive("builder",async()=>{console.log("held...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/♻️refine/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/♻️refine/🟦️.ts:34` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({growth:(process.resourceUsage().maxRSS-before)/1024,incidences,classes:refined.classes.length}));';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🌊️flat-memory/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🌊️flat-memory/🟦️.ts:16` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log(JSON.stringify({growth:(peak-before)/1048576,seconds,rows:${owners}*${per},bytes:out.length}));log.close();rmSync(directory,{recursive:true,force:true});`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🌊️stream-patch/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🌊️stream-patch/🟦️.ts:104` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log(JSON.stringify({growth:(process.resourceUsage().maxRSS-before)/1024,incidences:${owners}*${per},bytes:out.length}));';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🔖️journal-memory/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🔖️journal-memory/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({growth:(peak-before)/1048576,files:carry.files.length,largest:largest/1048576,bytes:bytes/1048576,exact,entries:seen.size}));log.close();rmSync(directory,{recursive:true,force:true});`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🗃️row-log/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🗃️row-log/🟦️.ts:101` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({growth,bytes:stats.bytes,rows:stats.rows,incidences:total,read}));log.close();rmSync(directory,{recursive:true,force:true});';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️host/🧪️tests/🌊️wave-prime/🧫️fixture.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️host/🧪️tests/🌊️wave-prime/🧫️fixture.ts:30` [[DEBUG]] test-path: yes; allocates: yes: JSON.stringify
  - `process.stdout.write("[DEBUG] "+JSON.stringify({outcomes,trace})+"\n");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🧪️tests/📥️child/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🧪️tests/📥️child/🟦️.ts:6` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await receiveScriptProcessInvocation(process.env,async original=>{const value=readIssuedScriptProcessEnvelopeV1(original),caller=readOwnedBuildInvocationV1(process.cwd(),original,["run","child:compile"]);let refused=f...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️source/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️source/🧪️tests/🟦️.ts:24` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `test("neutral selected Rust bounded Source ordinary files bind private observations and refuse failed children",async()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);await withBuildTestInvocation...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🔒️lifetime/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🔒️lifetime/🧪️tests/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `test("actual selected Rust failure child retains its first Original outcome",async()=>{if(kind!=="error"&&kind!=="undefined")throw Error("Closed selected Rust failure required");const primary=kind==="error"?Error("Exa...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🔒️lifetime/🧪️tests/🟦️.ts:38` [console.log] test-path: yes; allocates: no
  - `await withResourceLeases({directory:root,control,signal:control.signal,resources:[{resource:"selected:compiler-reader",mode:"shared"}]},()=>{observation.compilerReaders++;return withSelectedRustGraphConsumerV1(control...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts:35` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default indexed selected Rust graph refuses actual malformed DAGs without healing Originals","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:graph-invalid",0,async control=>{fo...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts:36` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default indexed selected Rust graph refuses executable DATA before hooks","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:graph-hooks",0,async control=>{for(const kind of corpu...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default indexed selected Rust graph retains primary body failures through accepted work","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:graph-primary",0,async control=>{for(co...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts:38` [console.log] test-path: yes; allocates: yes: Array.from, JSON.stringify
  - `registerBuildTestLawV1("default indexed selected Rust graph cancellation retains the genuine Original signal","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:graph-cancel",0,async control=>{const prog...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts:40` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default indexed selected Rust graph meters exact reducing byte extents","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:graph-extents",0,async control=>{const db=new Database("...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🧩️assembly/🧪️tests/🟦️.ts:41` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `registerBuildTestLawV1("default indexed selected Rust graph refuses increasing and invalid DATA limits before hooks","sdk-fence",original=>withIssuedResourceControlV1(original,"rust:graph-limit-guards",0,async control...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/🧪️tests/🪦️busy-orphan/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/🧪️tests/🪦️busy-orphan/🟦️.ts:9` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code='const {spawn}=require("node:child_process");const c=spawn(process.execPath,[${JSON.stringify(child)},"/","${busy}"],{stdio:["pipe","ignore","ignore"]});console.log(c.pid);setInterval(()=>{},1000);';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🧪️tests/🟦️.ts:592` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(await guard.admit(0)));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/📥️resolution/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/📥️resolution/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .map(, .push(
  - `try{const positives=fixture.cases.filter(row=>row.accepted);for(const[index,row]of positives.entries()){const scope=join(area,"r"+index),directory=join(scope,"node_modules",...row.parser.split("/")),entry=join(directo...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🔷️typescript-compiler/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🔷️typescript-compiler/🧪️tests/🟦️.ts:87` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const child=Bun.spawnSync([process.execPath,"-e",'import {typescriptCompilerV1} from ${JSON.stringify(join(place,"🟨️.mjs"))};try{console.log("loaded "+typescriptCompilerV1().version)}catch(error){console.log("refused ...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/📤️output/🎭️noise/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/📤️output/🎭️noise/🟦️.ts:1` [console.log] test-path: yes; allocates: no
  - `const key = Symbol.for("semio.output.contract.v1"), emit = (): void => { console.log("[noise] console.log"); process.stdout.write("[noise] process.stdout.write\n"); };`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/📤️output/🎭️wait/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/📤️output/🎭️wait/🟦️.ts:11` [console.log] test-path: yes; allocates: no
  - `console.log("[noise] console.log while admitted");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/📤️output/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/📤️output/🟦️.ts:33` [console.log] test-path: yes; allocates: no
  - `expect(result.stderr, "the noise went to the diagnostic stream").toContain("[noise] console.log");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/📤️output/🟦️.ts:37` [console.log] test-path: yes; allocates: no
  - `expect(result.stdout, "without the flag nothing is diverted").toContain("[noise] console.log");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🟦️.ts:11` [console.log] test-path: no; allocates: no
  - `export class CppScript extends Script{async run(segments:string[]):Promise<void>{const[command="all",...args]=segments;if(args.length)throw new Error("CMake accepts exact host preset and Original mode only");const mod...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🧪️tests/👷️child/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🧪️tests/👷️child/🟦️.ts:10` [console.log] test-path: yes; allocates: yes: JSON.stringify, new Map
  - `try{await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"cmake:failure",10000);await withRequestedIssuedResourceControlV1(control,0,phase=>withCmakeIn...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/🧪️tests/🟦️.ts:25` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const probeCode="const{DatabaseSync}=require('node:sqlite'),fs=require('node:fs'),crypto=require('node:crypto');const busy=p=>{const d=new DatabaseSync(p);try{d.exec('begin exclusive');d.exec('rollback');return false}...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/🧪️tests/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/🧪️tests/🟦️.ts:192` [console.log] test-path: yes; allocates: yes: ${
  - `const program='const delay=ms=>new Promise(accept=>setTimeout(accept,ms));await delay(150);const server=Bun.serve({hostname:"127.0.0.1",port:${port},fetch:()=>new Response("")});console.log("Local: http://localhost:${...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/🧪️tests/🟦️.ts:193` [console.log] test-path: yes; allocates: no
  - `const written:string[]=[],write=process.stdout.write.bind(process.stdout),log=console.log.bind(console);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/🧪️tests/🟦️.ts:206` [console.log] test-path: yes; allocates: yes: .join(, .push(
  - `console.log=((...values:unknown[])=>{written.push(values.join(" ")+"\n");log(...values);}) as typeof console.log;`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/🧪️tests/🟦️.ts:208` [console.log] test-path: yes; allocates: no
  - `process.stdout.write=write;console.log=log;`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/🧪️tests/🟦️.ts:214` [console.log] test-path: yes; allocates: no
  - `process.stdout.write=write;console.log=log;`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/📋️slice-report/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/📋️slice-report/🧪️tests/🟦️.ts:93` [console.log] test-path: yes; allocates: yes: ${
  - `import{readFileSync}from"node:fs";console.log(JSON.parse(readFileSync(process.argv[2]+"/${P.directory}/${P.name}","utf8")).reason);r.stop();');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/📋️slice-report/🧪️tests/🟦️.ts:239` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const usage=process.memoryUsage();console.log(JSON.stringify({guard:Math.max(guard,(usage.heapUsed+usage.external)/MB),rss:Math.max(rss,usage.rss/MB),kept:keep.length}));');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🦶️footprint/🧪️tests/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🦶️footprint/🧪️tests/🟦️.ts:301` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log("[R35] settled footprint MB per wave:",settled.footprints.join(" "));console.log("[R35] settled managed heap MB per wave:",settled.heaps.join(" "));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🦶️footprint/🧪️tests/🟦️.ts:310` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log("[R35] retained footprint MB per wave:",retained.footprints.join(" "));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🦶️footprint/🧪️tests/🟦️.ts:312` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log("[R35] retained managed heap MB per wave:",retained.heaps.join(" "));expect(retained.heaps[19]).toBeGreaterThan(100);expect(retained.heaps[19]).toBeGreaterThan(retained.heaps[2]*3);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🦶️footprint/🧪️tests/🟦️.ts:569` [console.log] test-path: yes; allocates: no
  - `fs.rmSync(root,{recursive:true,force:true});console.log(Math.round(peak/1048576));})();';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧪️tests/🧹️parent-budget/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧪️tests/🧹️parent-budget/🟦️.ts:119` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({most,peakManagedMegabytes:Math.round(peakManaged/MB),peakFootprintMegabytes:peakFootprint}));';`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧩️units/⌨️command/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧩️units/⌨️command/📜️script.ts:12` [console.log] test-path: no; allocates: no
  - `write: line => console.log(line),`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧩️units/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧩️units/🧪️tests/🟦️.ts:489` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[real repo, canonical overlay] modules ${modules.length}, packages planned ${packages.length}, compared with go list ${compared}, refused by go list ${skipped}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🌐️source-discovery/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🌐️source-discovery/🟦️.ts:52` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `for(const value of row.cases){const source="import resource from "+JSON.stringify(value.specifier)+"; console.log(resource);\n"+("secondSpecifier"in value?"import executable from "+JSON.stringify(value.secondSpecifier...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts:177` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const oracle=Bun.spawn(["node","--input-type=module","-e",'import{createHash}from"node:crypto";import{readFileSync}from"node:fs";import{join}from"node:path";const [root,row]=JSON.parse(process.argv[1]),hash=value=>cre...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts:86` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `try{await expect(withScriptInvocationSignalV1(issued,controller.signal,async original=>{try{await expect(api.withRepositoryBuildGraphFactStoreV1(root,original,createIssuedResourceControlV1(original,"graph:fixture-writ...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts:28` [console.log] test-path: yes; allocates: no
  - `const before = process.env.SEMIO_BUILD_MODE, messages: string[] = [], log = console.log;`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts:29` [console.log] test-path: yes; allocates: yes: .join(, .push(
  - `console.log = (...args: unknown[]) => { messages.push(args.join(" ")); };`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts:44` [console.log] test-path: yes; allocates: no
  - `console.log = log;`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🟦️.ts:214` [console.log] test-path: yes; allocates: no
  - `if(process.env.MEMO_LAW_TABLE)console.log("law",mutation.name.padEnd(54),"journal rows",String(changes.length).padStart(2),"relevant",String(firstRelevantChangeV1(relevance,changes)?.[2]??"-").padEnd(16),"memo",memo?"...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/🟦️.ts:120` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const entry='import"./m0.ts";export async function runCapturedBuildSourceV1(args,proof){proof.recheck();const inside=Math.round(process.memoryUsage().rss/1048576),outside=Math.round(Number(Bun.spawnSync(["ps","-o","rs...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/🧫️fixtures/🪆️relaunch.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/🧫️fixtures/🪆️relaunch.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("CHAIN "+JSON.stringify({first:report,secondFile:file,error}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: .filter(, .join(, .map(, JSON.stringify
  - `const fs=require("node:fs"),path=require("node:path"),url=require("node:url"),ts=require("typescript"),{spawnSync}=require("node:child_process"),sourcePath=url.fileURLToPath(new URL("../🟦️.ts",import.meta.url)),source...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/🎯️scope/🧪️tests/🧶️nx/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/🎯️scope/🧪️tests/🧶️nx/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const namespace=await bind(identity,null,(await import("../../../../../🟨️.mjs")).nxBuildDependencyImplementationV1()),source="const sdk=require('node:module').createRequire(process.env.SDK),{createTaskGraph}=sdk('nx/s...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/📥️closure/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/📥️closure/🧪️tests/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const namespace=await bind(identity,null,(await import("../../../../🟨️.mjs")).nxBuildDependencyImplementationV1()),source="const sdk=require('node:module').createRequire(process.env.SDK),{createTaskGraph}=sdk('nx/src/...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/📥️closure/🧪️tests/🟦️.ts:34` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `await withRepositoryNxStorageV1(repository.root,original,scriptCompilerAdmissionControlV1(original,"nx:fixture",0),async(bind,nxPhase)=>{const identity=process.env.SEMIO_NX_TOOLING_IDENTITY;if(!identity)throw Error("O...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧮️task-model/⚖️cache-law/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧮️task-model/⚖️cache-law/🧪️tests/🟦️.ts:44` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(counts));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/⌨️command/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/⌨️command/🟦️.ts:157` [console.log] test-path: no; allocates: no
  - `console.log(toolSetupMessageV1("starting", {count: rows.length}, context.environment));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/⌨️command/🟦️.ts:188` [console.log] test-path: no; allocates: no
  - `const reports = await setupToolsV1(rows, context, provision, control.signal, line => console.log(line));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/⌨️command/🟦️.ts:195` [console.log] test-path: no; allocates: no
  - `await warmToolStepsV1(toolStepsV1(process.platform), prepare, control.signal, line => console.log(line), context.environment);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/🧪️tests/🔍️hook-reason/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/🧪️tests/🔍️hook-reason/🟦️.ts:38` [console.log] test-path: yes; allocates: no
  - `expect(source).toContain('console.log(toolSetupMessageV1("starting", {count: rows.length}, context.environment));');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧱️job-limits/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧱️job-limits/🧪️tests/🟦️.ts:214` [console.log] test-path: yes; allocates: no
  - `const dir=mkdtempSync(join(tmpdir(),"launcher-")),procs=join(dir,"cgroup.procs"),argv=serviceLaunchArgumentsV1(procs,[process.execPath,"-e","console.log(process.pid)"]);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧱️job-limits/🧪️tests/🟦️.ts:219` [console.log] test-path: yes; allocates: no
  - `const refused=serviceLaunchArgumentsV1(join(dir,"missing","cgroup.procs"),[process.execPath,"-e","console.log('started')"]),started=spawnSync(refused[0]!,refused.slice(1),{encoding:"utf8"});`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🔄️reexec/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🔄️reexec/🧪️tests/🟦️.ts:7` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `test("restartable process birth defers only to an actual outer restart scope",()=>{const validate=new Ajv({strict:true}).compile(schema.$defs.RestartBirthCase),db=new Database(":memory:"),stamps=new URL("../../🟨️.mjs"...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/🚀️launch/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🧪️tests/🚀️launch/🟦️.ts:97` [console.log] test-path: yes; allocates: yes: .join(, collect
  - `writeFileSync(script,'console.log("collected "+process.argv.slice(2).join(" "));');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🧪️tests/🟦️.ts:296` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(process.env.SEMIO_CI_RENDER==="1")console.log(JSON.stringify(rendered,null,1));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🟦️.ts:37` [console.log] test-path: no; allocates: no
  - `const report=options.report??((line:string)=>console.log("[store-receipt] "+line)),execute=async():Promise<RunOutcomeV1>=>{try{await options.run();return {failed:false};}catch(error){return {failed:true,error};}};`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪞️parity/🧪️tests/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪞️parity/🧪️tests/🟦️.ts:360` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(args[0]==="show"){console.log(JSON.stringify({name:args[2],root:args[2],targets:{build:{execution:{kind:"command"},outputs:["{projectRoot}/dist"]},gen:{execution:{kind:"command"},outputs:["{workspaceRoot}/gen.json"...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪞️parity/🧪️tests/🟦️.ts:361` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(args.includes("inputs")){console.log("[nx] inputs");console.log(JSON.stringify({project:"p",target:"build",files:["p/a.ts","bun.lock"],runtime:[],environment:["PARITY_FLAG"],depOutputs:[],external:[]},null,2));proc...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪞️parity/🧪️tests/🟦️.ts:362` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `if(args.includes("--dry-run")){console.log("[log] planning");console.log(JSON.stringify(plan));process.exit(0);}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪞️parity/🧪️tests/🟦️.ts:366` [console.log] test-path: yes; allocates: no
  - `if(tool==="nx"){if(fail){console.log("   Failed tasks:\n\n   - p:test\n");process.exit(1);}console.log("Successfully ran target build");process.exit(0);}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧪️tests/🌊️seed-stream/👶️child.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧪️tests/🌊️seed-stream/👶️child.ts:23` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log("RESULT "+JSON.stringify({out,peakMegabytes:Math.round(process.resourceUsage().maxRSS*1024/1048576)}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/✅️verification/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/✅️verification/🟦️.ts:33` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[verify abstraction-ownership] ${fixture.cases.length} ownership vectors agree with Ajv.');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/✅️verification/🟦️.ts:40` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[verify abstraction-ownership] ${fixture.schemaCases.length} nested schema ownership vectors agree with Ajv.');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/✅️verification/🟦️.ts:47` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[verify abstraction-ownership] ${fixture.sourceCases.length} command source ownership vectors agree with Ajv.');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/✅️verification/🟦️.ts:78` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[verify abstraction-ownership] ${fixture.artifactSchemas.length} artifact contracts expose document state only across five schema formats.');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/💾️storage/🧪️tests/📥️runtime/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/💾️storage/🧪️tests/📥️runtime/🟦️.ts:9` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `await withRuntimeVerificationStorageV1(workspace,["--report",report],invocation,async scope=>{const mode=(invocation.capabilities as {buildMode:"prototype"|"release"}).buildMode,directory=mkdtempSync(join(scope.diagno...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts` (7)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts:388` [console.log] test-path: no; allocates: yes: ${, Date.now
  - `const observationStarted=Date.now(),observationBudget=cmdBudgetMs(),observation:FileObservationControlV1={maxBytes:128*1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>signal.aborted,remainingMs:()=>Math.min...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts:389` [console.log] test-path: no; allocates: yes: ${, Date.now
  - `const cargoObservation = cargoProvenancePhysicalControlV1({ cancelled: () => signal.aborted, deadlineMs: 86_400_000, remainingMs: () => Math.min(phaseControl.remainingMilliseconds()??Infinity,(observationBudget > 0 ? ...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts:414` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[runtime graph] acquiring current exact guest compiler/resource witnesses through ${producer.command[2]}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts:467` [console.log] test-path: no; allocates: yes: ${
  - `if(!native&&!provenance&&!artifacts){console.log('[runtime graph] acquiring actual native browser mount through ${config.nativeBrowserProject}:wasm:${completedProfile}');await runRuntimeBuildInvocationV1(root,["run",c...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts:474` [console.log] test-path: no; allocates: yes: ${, .map(
  - `if(mountedOutputs.some(artifact=>!artifact.current)&&!provenance&&!artifacts){console.log('[runtime graph] acquiring current browser mount through ${config.nativeBrowserProject}:generate-frame-worker');await runRuntim...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts:498` [console.log] test-path: no; allocates: yes: ${, .join(
  - `console.log('[runtime graph] actual Cargo metadata ${index + 1}/${checks.length} ${check.target}: ${check.packages.join(",")}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🟦️.ts:591` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[runtime graph] graphs=${graphs.length} sources/resources=${identities.size} findings=${findings.length} report=${report}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧪️tests/🏗️build-producer/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧪️tests/🏗️build-producer/🟦️.ts:11` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `await writeFile(join(directory,"📜️script.ts"),'const original=JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION);await Bun.write(import.meta.dir+"/result.json",JSON.stringify({policy:original.policy,deadline:orig...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts` (16)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:227` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[verify dependencies testkit] manifests=${manifests.length} clean.');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:239` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `if (format === "json") console.log(JSON.stringify(audit, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:240` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('[verify dependencies locks] scopes=${audit.scopes} manifests=${audit.workspaces} frozen-refusals=${refused.length} lock-noise=${audit.mismatches.length - refused.length} fixtures=${audit.lockFixtureC...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:245` [console.log] test-path: no; allocates: no
  - `console.log("[verify dependencies locks] clean.");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:254` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `if (format === "json") console.log(JSON.stringify(report, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:255` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('[verify dependencies parity js] manifests=${report.manifests} external-rows=${report.externalRows} evidenced=${report.evidencedRows} unowned=${report.unownedRows.length} undeclared-imports=${report.u...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:268` [console.log] test-path: no; allocates: no
  - `console.log("[verify dependencies parity js] clean.");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:272` [console.log] test-path: no; allocates: yes: ${
  - `if (args[0] === "self-test") { console.log('[verify dependencies self-test] hostile-mutations=${selfTests.truth()} clean.'); return; }`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:278` [console.log] test-path: no; allocates: yes: JSON.stringify
  - `console.log(format === "json" ? JSON.stringify(report, null, 2) : dependencyTruthSummaryText(report));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:293` [console.log] test-path: no; allocates: yes: .filter(, JSON.stringify
  - `console.log(JSON.stringify(entries.filter((entry) => !ecosystem || entry.ecosystem === ecosystem), null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:298` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[verify dependencies] wrote ${DEPENDENCY_BASELINE_REL_PATH}: ${baseline.entries.length} third-party dependenc(y/ies) at commit ${baseline.commit}.');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:302` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[verify dependencies] baseline: ${result.baseline.entries.length} third-party dependenc(y/ies) (commit ${result.baseline.commit || "none — run write-baseline first"}); current: ${result.current.length}.');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:303` [console.log] test-path: no; allocates: no
  - `console.log(dependencyTruthSummaryText(dependencyTruthReport(repoRoot, result.current)));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:305` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[verify dependencies] ${result.removedDeps.length} dependenc(y/ies) removed since baseline (always passes — ratchet only tightens):');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:306` [console.log] test-path: no; allocates: yes: ${
  - `for (const dependency of result.removedDeps) console.log('  ${dependency.ecosystem}:${dependency.name}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts:314` [console.log] test-path: no; allocates: no
  - `console.log("[verify dependencies] clean — no new third-party dependencies.");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/⚙️configured/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/⚙️configured/🧪️tests/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify, collect
  - `test("configured Source resolves genuine aliases, package owners and lazy inputs against esbuild",async()=>withBuildTestInvocationV1(original=>withRepositoryTestArtifactsV1(repository.root,original,artifactEntryContro...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/⚙️configured/🧪️tests/🟦️.ts:40` [console.log] test-path: yes; allocates: yes: .filter(, .map(, .repeat(, JSON.stringify
  - `const directory=mkdtempSync(join(repositoryTestArtifactRootV1(repository.root),"cr-")),compiler=createRequire(import.meta.url)("typescript"),readFile=compiler.sys.readFile;let oversized=0;try{mkdirSync(join(directory,...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/⚙️configured/🧪️tests/🟦️.ts:44` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const directory=mkdtempSync(join(repositoryTestArtifactRootV1(repository.root),"rt-")),row=fixture.declarationRuntime,require=createRequire(import.meta.url);try{for(const[path,body]of Object.entries(row.files)){mkdirS...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🟦️.ts:48` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const output = await captureOwnedCommandV1("node", ["-e", '${source}\nconsole.log(JSON.stringify(module.exports.result))'], root, { control: scriptCompilerAdmissionControlV1(original, "build:declared-source:oracle", 1...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/🔁️verifier/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/🔁️verifier/🟦️.ts:16` [console.log] test-path: yes; allocates: yes: JSON.stringify, new Set
  - `console.log(JSON.stringify({holds:await observationsHold(rows,64,new Set(),true),plain}));');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/📥️admission/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/📥️admission/🧪️tests/🟦️.ts:7` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `test("portable runtime source admission agrees with Ajv and independent Node execution",()=>{for(const row of fixture.vectors){const actual=runtimeInputAdmissionV1(row.components,row.roots,row.sources,(path:string)=>p...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️cargo/🏃️execution/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️cargo/🏃️execution/🟦️.ts:18` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[cargo-dependency-direction] resolving all declaration kinds; inventoriedPackages=${inventory.length}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️cargo/🏃️execution/🟦️.ts:32` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[cargo-dependency-direction] passed; localDeclarations=${report.localDependencies}; packages=${report.packages}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:202` [console.log] test-path: no; allocates: yes: ${
  - `if (files % 250 === 0) console.log('[rust-source-direction] progress; files=${files}; references=${references}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:217` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[rust-source-direction] passed; files=${report.files}; authoredReferences=${report.references}; scope=all-configurations-and-macro-templates; runtimeReferences=${report.runtime.references}; runtimeScope=$...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🧪️tests/🟦️.ts:33` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const oracle=String.raw'const fs=require("node:fs"),p=require("node:path"),toml=require("@iarna/toml");const root=process.argv[1],w=JSON.parse(process.argv[2]);const d=toml.parse(fs.readFileSync(p.join(root,w.manifest...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/🟦️.ts:249` [console.log] test-path: no; allocates: yes: ${
  - `console.log('framework/ui/js/react: no UI primitive violations (${allowlist.size} allowlisted files)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/🟦️.ts:283` [console.log] test-path: no; allocates: yes: ${
  - `console.log('framework/ui/js/react: no chrome i18n violations (${allowlist.size} allowlisted files)');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🏃️execution/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🏃️execution/🟦️.ts:17` [console.log] test-path: no; allocates: yes: ${
  - `if (directoriesScanned % 256 === 0) console.log('[workspaces] scanned ${directoriesScanned} directories');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📣️publication/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📣️publication/🟦️.ts:32` [console.log] test-path: no; allocates: no
  - `report: (stream, message) => (stream === "stderr" ? console.error(message) : console.log(message)),`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧪️tests/📥️original-control/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧪️tests/📥️original-control/🟦️.ts:22` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const oracle='const [row,root,corpus,schema,callerSchema,cargoExecutionSchema]=JSON.parse(process.argv[1]),fs=require("node:fs"),p=require("node:path"),cp=require("node:child_process"),Ajv=require("ajv"),controller=ne...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧪️tests/📥️original-control/🟦️.ts:56` [console.log] test-path: yes; allocates: yes: .join(, JSON.stringify
  - `const node=Bun.spawn([registeredToolExecutableV1("node"),"--eval",'const fs=require("node:fs"),p=require("node:path"),cp=require("node:child_process"),TOML=require("smol-toml"),[root,row]=JSON.parse(process.argv[1]),c...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧪️tests/📥️original-control/🟦️.ts:71` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const timer=setTimeout(()=>controller.abort(Error("Original identity caller timeout")),corpus.callerCeilingMilliseconds),node=Bun.spawn([registeredToolExecutableV1("node"),"--eval",'const cp=require("node:child_proces...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/📥️original-control/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/📥️original-control/🟦️.ts:73` [console.log] test-path: yes; allocates: yes: .push(
  - `try{await withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",lease,async(args,cwd,signal)=>{steps.push(args[0]);expect(signal).toBe(lease.signal);const budget=control.childBudget(1000);budgets.push(budget);co...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/📥️original-control/🟦️.ts:89` [console.log] test-path: yes; allocates: no
  - `const output=process.env.SEMIO_TEST_ARTIFACT_DIR!,root=mkdtempSync(join(output,"rc-")),entry=join(root,"📜️script.ts"),source='import {value as a} from "./ä.ts";import {value as b} from "./\\u00e4.ts";export {value} fr...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/📥️original-control/🟦️.ts:95` [console.log] test-path: yes; allocates: no
  - `const root=mkdtempSync(join(output(),"rx-")),entry=join(root,"📜️script.ts");writeFileSync(entry,'import {value} from "./ä.ts";console.log(value);');writeFileSync(join(root,"ä.ts"),"export const value=1;\n");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🔬️script-commands/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🔬️script-commands/🟦️.ts:15` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[cargo-members] ${owner.manifest} ${changed ? "published" : "current"}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🔬️script-commands/🟦️.ts:29` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[runtime-input] ${request.component} ${admission.selected.length} admitted components');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🔬️script-commands/🟦️.ts:36` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('cargo capability contract: ${runCargoCapabilityContributionChecks() + runCargoCapabilityPhysicalChecks()} laws');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🕰️queued-preparation/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🕰️queued-preparation/🟦️.ts:34` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `put("proof/📜️script.ts", 'import {prepareCargoFixtureV1} from ${JSON.stringify(fileURLToPath(new URL("../🧰️runtime/🟦️.ts",import.meta.url)))};await prepareCargoFixtureV1(${JSON.stringify(storage)},process.env.SEMIO_WO...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🛰️fixture-children/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🛰️fixture-children/🟦️.ts:57` [console.log] test-path: yes; allocates: no
  - `put(root,"📜️script.ts",observedCargoPreparationV1(root,'console.log("[preparation] owned input refreshed");'));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧪️tests/🛰️fixture-children/🟦️.ts:59` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `put(root,"proof/📜️script.ts",'import {prepareCargoFixtureV1,captureCargoFixtureV1} from ${JSON.stringify(fileURLToPath(new URL("../🧰️runtime/🟦️.ts",import.meta.url)))};await prepareCargoFixtureV1(${JSON.stringify(stor...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🏃️runner/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🏃️runner/🟦️.ts:161` [console.log] test-path: no; allocates: yes: ${
  - `console.log('policy: no high-priority breaches (${breachs.length} total recorded at ${cachePath})');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🖥️cli/🧪️tests/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🖥️cli/🧪️tests/🟦️.ts:20` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const executable=(root:string,spec:{lines:string[];exit:number;stderr?:string}):string=>{const path=join(root,"fake-repo");writeFileSync(path,"#!/usr/bin/env node\n"+"const lines="+JSON.stringify(spec.lines)+";for(con...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚨️policy/🖥️cli/🧪️tests/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const root=scratch(),path=join(root,"echo-repo");writeFileSync(path,"#!/usr/bin/env node\nconsole.log(JSON.stringify({data:{argv:process.argv.slice(2),cwd:process.cwd()}}));\n");chmodSync(path,0o755);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🎮️command/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🎮️command/🟦️.ts:11` [console.log] test-path: no; allocates: yes: ${, .join(, .map(
  - `console.log(changed.length === 0 ? "[agents write] every client file is current" : changed.map((path) => '[agents write] wrote ${path}').join("\n"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🎮️command/🟦️.ts:18` [console.log] test-path: no; allocates: no
  - `console.log("[agents check] every client file matches the root declaration");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📊️coverage/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📊️coverage/🟦️.ts:119` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[coverage] ${summary.linesHit}/${summary.linesFound} lines (${summary.pct.toFixed(2)}%), threshold ${thresholdPct}%.');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📊️coverage/🟦️.ts:121` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[coverage] worst ${Math.min(worstCount, summary.perFile.length)} files:');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📊️coverage/🟦️.ts:123` [console.log] test-path: no; allocates: yes: ${
  - `console.log('  ${file.pct.toFixed(1)}% (${file.linesHit}/${file.linesFound})  ${file.path}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📊️coverage/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📊️coverage/🧪️tests/🟦️.ts:18` [console.log] test-path: yes; allocates: no
  - `const lines:string[]=[],errors:string[]=[],log=console.log,error=console.error,exit=process.exit;let code:number|null=null;`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📊️coverage/🧪️tests/🟦️.ts:19` [console.log] test-path: yes; allocates: yes: .join(, .push(
  - `console.log=(...parts:unknown[])=>{lines.push(parts.join(" "));};console.error=(...parts:unknown[])=>{errors.push(parts.join(" "));};(process as unknown as {exit:(c?:number)=>never}).exit=(value?:number)=>{throw new E...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📊️coverage/🧪️tests/🟦️.ts:20` [console.log] test-path: yes; allocates: no
  - `try{action();}catch(thrown){if(thrown instanceof Exit)code=thrown.code;else throw thrown;}finally{console.log=log;console.error=error;process.exit=exit;}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🟦️.ts:59` [console.log] test-path: no; allocates: yes: ${, Date.now
  - `else if (Date.now() - reported >= 2000) { console.log('[test-resolution] ${progress.stage}: ${progress.completedOperations} source operations'); reported = Date.now(); }`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts` (8)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:31` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code='const{mock}=await import("bun:test");const{requireIssuedScriptInvocationV1,createIssuedResourceControlV1,requireIssuedResourceControlInvocationV1}=await import(${JSON.stringify(processInvocationPath)});con...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:43` [console.log] test-path: yes; allocates: yes: ${, .map(, .push(, JSON.stringify
  - `const code='const{mock}=await import("bun:test");const library=await import(${JSON.stringify(commandPath)});let calls=[];mock.module(${JSON.stringify(commandPath)},()=>({...library,runRepositoryTestCommand:async(file,...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:77` [console.log] test-path: yes; allocates: yes: ${
  - `const child = 'await new Promise(resolve => setTimeout(resolve, ${row.durationMs})); console.log("build-complete");';`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:78` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code = 'const { runProbe } = await import(${JSON.stringify(probePath)}); try { const result = runProbe(process.execPath, ["-e", ${JSON.stringify(child)}], { budgetMs: ${row.budgetMs} }); console.log(JSON.stringi...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:95` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code = 'const { ${runner} } = await import(${JSON.stringify(runner === "runRepositoryTestCommand" ? commandPath : processPath)}); const { buildBudgetMs } = await import(${JSON.stringify(budgetPath)}); await ${ru...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:122` [console.log] test-path: yes; allocates: no
  - `spawn: (cmd, args, opts) => cmd === "cargo" ? spawn(process.execPath, ["-e", args.includes("list") ? "await Bun.sleep(120); console.log('{}')" : "console.log('assertions-complete')"], opts) : spawn(cmd, args, opts),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:148` [console.log] test-path: yes; allocates: no
  - `return spawn(process.execPath, ["-e", building ? "await Bun.sleep(300); console.log('coverage-built')" : reporting ? "console.log('coverage-reported')" : "console.log('assertions-complete')"], opts);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts:182` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code='const{mock}=await import("bun:test");const preparePath=${JSON.stringify(fileURLToPath(new URL("../../🗂️workspaces/🦀️cargo/📜️script.ts",import.meta.url)))};const originalBunSpawnSync=Bun.spawnSync.bind(Bun)...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚡️workspace-negation-closure/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚡️workspace-negation-closure/🟦️.ts:61` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const script = 'const { createProjectGraphAsync } = await import(${graph}); const { HashPlanInspector } = await import(${inspector}); const planner = new HashPlanInspector(await createProjectGraphAsync({ exitOnError: ...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts:136` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `const started = performance.now(), checkpoint = (phase: string) => console.log("[TRACE] Historical coordinate fixture phase", JSON.stringify({ phase, milliseconds: performance.now() - started }));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts:121` [console.log] test-path: yes; allocates: no
  - `'else if(command==="generate"){mkdirSync(root,{recursive:true,mode:0o755});writeFileSync(file,bytes,{mode:0o644});console.log("support fixture generated");}',`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts:122` [console.log] test-path: yes; allocates: no
  - `'else if(command==="check-generated"){if(!existsSync(file)||!readFileSync(file).equals(bytes))throw new Error("stale generated fixture");console.log("support fixture checked");}',`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts:131` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `'console.log('${JSON.stringify({events:JSON.parse(source0).events.length,dsl:source1.split("\\n")[0],payload:JSON.parse(source2).title})}');',`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts:110` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `'console.log(${JSON.stringify(marker)} + JSON.stringify({ calls, exports: Object.keys(family) }));',`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🟦️.ts:161` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `"console.log(JSON.stringify({ pid: process.pid, elapsedMs: performance.now() - started, rows }));",`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️typescript-package-commands/🛤️runtime-and-path-routes/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️typescript-package-commands/🛤️runtime-and-path-routes/🟦️.ts:68` [console.log] test-path: yes; allocates: no
  - `console.log("[nx-project-inference] original Node/Bun graph revision laws passed");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/📝️commit-and-budgets/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/📝️commit-and-budgets/🟦️.ts:440` [console.log] test-path: yes; allocates: no
  - `const result = runProbe("bun", ["-e", "console.log('probe-ok')"]);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts:210` [console.log] test-path: yes; allocates: no
  - `const log = console.log;`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts:212` [console.log] test-path: yes; allocates: yes: .join(, .push(
  - `console.log = (...args: unknown[]) => messages.push(args.join(" "));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts:241` [console.log] test-path: yes; allocates: no
  - `} finally { console.log = log; rmSync(root, { recursive: true, force: true }); }`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🪵️root/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🪵️root/🟦️.ts:32` [console.log] test-path: yes; allocates: yes: .map(, .push(, JSON.stringify
  - `const script = "const fs=require(\"node:fs\"),path=require(\"node:path\");const rows=JSON.parse(process.argv[1]);console.log(JSON.stringify(rows.map(row=>{const value=row.input;if(!value||value.includes(\"\\0\")||/^[A...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🪵️root/🟦️.ts:38` [console.log] test-path: yes; allocates: yes: .map(, JSON.stringify
  - `const markerOracle = Bun.spawn([registeredToolExecutableV1("node"), "-e", 'const fs=require("node:fs");console.log(JSON.stringify(JSON.parse(process.argv[1]).map(path=>fs.readFileSync(path+"/leaf.rs","utf8").match(/OW...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️wasm-package-wrappers/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️wasm-package-wrappers/🟦️.ts:99` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `writeFileSync(probe, 'for (const specifier of ${JSON.stringify(specifiers)}) console.log(new URL(import.meta.resolve(specifier)).pathname);\n');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:397` [console.log] test-path: yes; allocates: no
  - `console.log("Rust join compiler oracle confirmed six string/path runtime cases.");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:430` [console.log] test-path: yes; allocates: no
  - `console.log("Rust finite receiver compiler oracle confirmed five exact-target cases and one missing-target rejection.");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:451` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('Rust adversarial ${row.id}: ${JSON.stringify(observed)}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:493` [console.log] test-path: yes; allocates: no
  - `console.log("Rust manifest PathBuf compiler oracle confirmed five constructor/capture runtime cases.");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-command-paths/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-command-paths/🟦️.ts:60` [console.log] test-path: yes; allocates: no
  - `if (!powershell) return console.log("[owner-only] no pwsh/powershell.exe on PATH: .NET oracle skipped");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts:57` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('[clean taxonomy inventory] entries=${inventory.entries.length} source=${inventory.sourceTreeDigest}${ticketDir ? ' -> ${ticketDir}' : ""}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts:74` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('[clean taxonomy plan] ${taxonomyCliPlanOperationConsoleFields(counts)} unresolved=${plan.unresolved.length} digest=${plan.planDigest} -> ${planPath}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts:113` [console.log] test-path: no; allocates: no
  - `console.log(`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts:120` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[clean taxonomy apply] build owners removed=${owners.removed.length} added=${owners.added.length} renamed=${owners.renamed.length} relocated=${owners.relocated.length} repackaged=${owners.repackaged.lengt...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts:133` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('[clean taxonomy verify] clean=${verification.clean} errors=${errors} warnings=${warnings}${ticketDir ? ' -> ${ticketDir}' : ""}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🔁️workflow/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🔁️workflow/🟦️.ts:78` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('[clean taxonomy inventory --kind mutation] roots=${inventory.roots.length} records=${inventory.records.length} violations=${inventory.violations.length}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🔁️workflow/🟦️.ts:87` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('[clean taxonomy plan --kind mutation] moves=${plan.moves.length} unresolved=${plan.unresolved.length} digest=${plan.planDigest} -> ${path}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🔁️workflow/🟦️.ts:109` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('[clean taxonomy apply --kind mutation] state=committed moves=0 digest=${plan.planDigest}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🔁️workflow/🟦️.ts:116` [console.log] test-path: no; allocates: yes: ${
  - `else console.log('[clean taxonomy verify --kind mutation] clean=${verification.clean} errors=${verification.violations.length}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts` (15)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:48` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[clean marker-only-folders] ${dry ? "dry-run" : "applied"} removals=${removals.length} bytes=${bytes}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:49` [console.log] test-path: no; allocates: yes: ${
  - `for (const row of removals) console.log('[clean marker-only-folders] ${dry ? "would-remove" : "removed"} ${row.path} (${row.bytes})');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:50` [console.log] test-path: no; allocates: yes: ${
  - `for (const path of blocked) console.log('[clean marker-only-folders] protected ${path}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:68` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[clean empty-folders] ${dry ? "would-remove" : "removed"} ${row.path} (${row.bytes})');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:75` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[clean empty-folders] ${dry ? "dry-run" : "applied"} removals=${totalRemovals} bytes=${totalBytes}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:76` [console.log] test-path: no; allocates: yes: ${
  - `for (const path of blocked) console.log('[clean empty-folders] protected ${path}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:96` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[clean coverage] ${dry ? "dry-run" : "applied"} removals=${removed.length}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:97` [console.log] test-path: no; allocates: yes: ${
  - `for (const path of removed) console.log('[clean coverage] ${dry ? "would-remove" : "removed"} ${path}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:111` [console.log] test-path: no; allocates: no
  - `for (const line of lines) console.log(line);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:120` [console.log] test-path: no; allocates: no
  - `const presence=await readJavascriptDependencyPreservationPresenceV1(this.repoRoot,this.invocation,control);if(presence.state==="empty"){console.log("[clean] dependency-preservations retired=0 retained=0 entries=0");re...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:123` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[clean] dependency-preservations retired=${report.retired.length} retained=${report.retained.length} entries=${report.entries}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:124` [console.log] test-path: no; allocates: yes: ${
  - `for(const row of report.retained)console.log('[clean] retained ${row.path}: ${row.reason}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:131` [console.log] test-path: no; allocates: no
  - `console.log('[clean] cache-prune skipped: the process table is unreadable, so an active Semio Tech build cannot be ruled out');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:135` [console.log] test-path: no; allocates: no
  - `console.log('[clean] cache-prune skipped active Semio Tech build');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts:140` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[clean] cache-prune ok ${repoCacheDirectory(this.root)}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🚀️entrypoint/🦀️.rs` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🚀️entrypoint/🦀️.rs:110` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("repo MCP accepts no command arguments");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🚀️entrypoint/🦀️.rs:116` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{error}");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🚀️entrypoint/🦀️.rs:123` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("{error}");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️types/🏃️execution/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️types/🏃️execution/🟦️.ts:30` [console.log] test-path: no; allocates: yes: ${
  - `console.log('graphql types refreshed (${source.operations.length} operations, schema sha256 ${source.schemaSha256}) -> 🤖️generated/🔗️graphql-types/🟦️.ts');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️types/🏃️execution/🟦️.ts:47` [console.log] test-path: no; allocates: yes: ${
  - `console.log('graphql types are fresh (${source.operations.length} operations, schema sha256 ${source.schemaSha256}).');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🎯️goal-and-file-support/🦀️.rs` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🎯️goal-and-file-support/🦀️.rs:16` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to update GitHub issue title: {}", error.message);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs` (12)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:20` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to remove labels from GitHub issue {issue}: {}", error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:47` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("Warning: Failed to ensure milestone for root goal {}: {error}", goal.id),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:51` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to update milestone for root goal {}: {}", goal.id, error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:62` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to delete milestone {number} for child goal {}: {}", goal.id, error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:77` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to create issue for child goal {}: {}", goal.id, error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:91` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("Warning: Failed to link sub-issue {} to parent {parent_id}: {}", goal.id, error.message),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:96` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to save goal {}: {error}", goal.id);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:104` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to get issue for goal {}: {}", goal.id, error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:123` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to clear milestone for goal {}: {}", goal.id, error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:129` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("Warning: Failed to resolve parent for goal {}: {}", goal.id, error.message),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:132` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("Warning: Failed to link sub-issue {} to parent {parent_id}: {}", goal.id, error.message),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔄️management-sync/🦀️.rs:140` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to add goal label for {}: {}", goal.id, error.message);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs` (14)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:222` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: {warning}");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:236` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("Warning: Failed to close ticket {}: {}", ticket.slug, error.message),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:246` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to close GitHub issue {issue}: {}", error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:250` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("Warning: Failed to list GitHub issues with 'ticket' label: {}", error.message),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:259` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: {warning}");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:286` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: {warning}");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:294` [eprintln!] test-path: no; allocates: yes: println!
  - `for warning in &outcome.warnings { eprintln!("Warning: {warning}"); }`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:436` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to sync GitHub label catalog: {}", error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:439` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("Warning: Failed to list goals: {error}"),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:453` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to get GitHub issue {issue}: {}", error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:460` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to close GitHub issue {issue}: {}", error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:466` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("Warning: Failed to resolve goal milestone for issue {issue}: {error}"),`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:470` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("Warning: Failed to update milestone for GitHub issue {issue}: {}", error.message);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🗄️fs-context/🔌️repo-context-port/🦀️.rs:480` [eprintln!] test-path: no; allocates: yes: println!
  - `Err(error) => eprintln!("Warning: Failed to list GitHub issues for label sync: {}", error.message),`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/🏃️execution/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/🏃️execution/🟦️.ts:41` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[next-build] mode=${mode} workers=${compilerEnvironment.SEMIO_COMPILER_PARALLELISM} outputs=${staged.size}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📥️inputs/🔌️nx/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📥️inputs/🔌️nx/🧪️tests/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify
  - `test("actual frozen Nx resolves original mode output placeholders",async()=>withBuildTestInvocationV1(original=>withRepositoryNxStorageV1(repository.root,original,scriptCompilerAdmissionControlV1(original,"next:projec...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📦️publication/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📦️publication/🟦️.ts:34` [console.log] test-path: no; allocates: no
  - `console.log("[next-build] tracing standalone startup runtime");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🚪️entrypoint/🦀️.rs` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🚪️entrypoint/🦀️.rs:12` [eprintln!] test-path: no; allocates: yes: println!
  - `eprintln!("repo coordinator: {failure}");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️library/👷️worker/🏃️execution/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️library/👷️worker/🏃️execution/🟦️.ts:40` [console.log] test-path: no; allocates: yes: ${
  - `console.log('[worker] reindex job for ${job.data.repoRoot}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️library/👷️worker/🏃️execution/🟦️.ts:53` [console.log] test-path: no; allocates: no
  - `console.log("[worker] pg-boss started");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️library/👷️worker/🏃️execution/🟦️.ts:58` [console.log] test-path: no; allocates: no
  - `console.log("[worker] listening for jobs");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️library/👷️worker/🏃️execution/🟦️.ts:61` [console.log] test-path: no; allocates: no
  - `console.log("[worker] shutting down...");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️library/👷️worker/🏃️execution/🟦️.ts:67` [console.log] test-path: no; allocates: no
  - `console.log("[worker] shutting down...");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📌️runtime-pin-contract/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📌️runtime-pin-contract/🟦️.ts:164` [console.log] test-path: yes; allocates: no
  - `const script=spawnSync(other!,["-e","console.log(process.versions.bun)"],{cwd:root,encoding:"utf8"});`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🧱️cache-isolation/🧮️runner/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🧱️cache-isolation/🧮️runner/🟦️.ts:9` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({out:(await import(join(root,"main.ts"))).out}));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/📍️relocate/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/📍️relocate/🧪️tests/🟦️.ts:137` [[DEBUG]] test-path: yes; allocates: yes: ${
  - `await expect(run(fleet,{fault:reached=>{if(reached===step&&!crashed){crashed=true;throw Error('[DEBUG] injected crash after ${reached}');}}})).rejects.toThrow("injected crash");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌪️chaos.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌪️chaos.ts:75` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[chaos soak] base seed ${base}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌪️chaos.ts:81` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[chaos soak] ${runs} runs, ${failures.length} failures');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌪️sweep.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌪️sweep.ts:21` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `console.log('CHAOS_RESULT ${JSON.stringify(report)}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌪️sweep.ts:31` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('shrunk ${saved.ops.length} -> ${shrunk.length}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌪️sweep.ts:52` [console.log] test-path: yes; allocates: yes: ${, .push(, JSON.stringify
  - `if(line===undefined){timedOut++;failing.push(seed);writeFileSync(join(out,'seed-${seed}.json'),JSON.stringify({seed,machines,violations:["timeout: the run printed no result"],ops:[],notes:[]}));console.log('seed ${see...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌪️sweep.ts:59` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('seed ${seed}: FAIL ${report.violations[0]!.slice(0,200)}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌪️sweep.ts:62` [console.log] test-path: yes; allocates: yes: ${
  - `if(flag("shrink"))for(const seed of failing.sort((a,b)=>a-b).slice(0,Number(arg("shrink-max","6")))){console.log((await child(["--shrink-file",join(out,'seed-${seed}.json'),...common],1_800_000)).trim());}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🌪️sweep.ts:65` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(summary));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🛡️resilience.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🛡️resilience.ts:112` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const script=(ok:boolean):string=>'import {codeDigestV1} from ${JSON.stringify(attest)};\nconst digest=await codeDigestV1(${JSON.stringify(module)});\nconsole.log(JSON.stringify({version:1,ok:${ok},digest,modules:1,sc...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🧭️agree.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🧭️agree.ts:95` [[DEBUG],console.log] test-path: yes; allocates: yes: ${, .map(, .push(, JSON.stringify
  - `for(const other of report.rows)if(other.originId!==who.name&&other.verdict==="diverged"){named.push('wrong:${other.originId}');console.log("[DEBUG]",JSON.stringify(report.differing),JSON.stringify(report.rows.map(r=>[...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/🧭️agree.ts:116` [[DEBUG],console.log] test-path: yes; allocates: yes: ${, .filter(, .map(, .push(
  - `for(const other of report.rows)if(other.originId!==who.name&&other.verdict==="diverged"){named.push('wrong:${other.originId}');if(!(globalThis as any).__dbg){(globalThis as any).__dbg=1;for(const m of churn.fleet.mach...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧲️adopt/🧪️tests/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧲️adopt/🧪️tests/🟦️.ts:225` [[DEBUG]] test-path: yes; allocates: yes: ${
  - `await expect(run(fleet.adopter,{fault:reached=>{if(reached===step&&!crashed){crashed=true;throw Error('[DEBUG] injected crash after ${reached}');}}})).rejects.toThrow("injected crash");`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧷️sidecars/🧪️tests/🧬️child.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧷️sidecars/🧪️tests/🧬️child.ts:6` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('child ${mode} started');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🥾️provision/🧪️tests/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🥾️provision/🧪️tests/🟦️.ts:29` [console.log] test-path: yes; allocates: no
  - `put(base,"🗺️topology/📜️script.ts",'import {variant} from "./🟦️.ts";\nconsole.log(variant);\n');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🥾️provision/🧪️tests/🟦️.ts:33` [console.log] test-path: yes; allocates: no
  - `put(base,"🔁️replica/📜️script.ts",'import {replica} from "./🟦️.ts";\nconsole.log(replica);\n');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🥾️provision/🧪️tests/🟦️.ts:34` [console.log] test-path: yes; allocates: yes: ${
  - `put(base,"🔁️replica/🥾️entry/🟨️.mjs",'console.log("boot ${scripts}");\n');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🥾️provision/🧪️tests/🟦️.ts:115` [console.log] test-path: yes; allocates: no
  - `put(base,"🗺️topology/📜️script.ts",'import {variant} from "./🟦️.ts";\nawait import("./📚️extra/🟦️.ts");\nconsole.log(variant);\n');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🥾️provision/🧪️tests/🟦️.ts:119` [console.log] test-path: yes; allocates: no
  - `put(base,"🗺️topology/📜️script.ts",'import {variant} from "./🟦️.ts";\nawait import("./🚫️unlisted/🧰️helper.mjs");\nconsole.log(variant);\n');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🚧️boundary/🔤️fold/📜️script.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🚧️boundary/🔤️fold/📜️script.ts:32` [console.log] test-path: no; allocates: no
  - `console.log("fold table equals this runtime's ICU");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🚧️boundary/🔤️fold/📜️script.ts:33` [console.log] test-path: no; allocates: yes: ${
  - `}else{writeFileSync(file,text);console.log('wrote ${file}');}`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🛡️resilience/🧪️tests/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🛡️resilience/🧪️tests/🟦️.ts:115` [console.log] test-path: yes; allocates: yes: ${, .repeat(, JSON.stringify
  - `expect(await run('console.log(${JSON.stringify(ok)})')).toEqual({ok:true,reason:"",digest:"a".repeat(64)});`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🛡️resilience/🧪️tests/🟦️.ts:116` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const failed=await run('console.log(${JSON.stringify(JSON.stringify({version:1,ok:false,digest:null,modules:0,schemas:0,texts:0,error:"module x does not load"}))});process.exit(1)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🛡️resilience/🧪️tests/🟦️.ts:120` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `expect((await run('console.log(${JSON.stringify(ok)});process.exit(2)')).ok).toBe(false);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️parity/📋️orchestration/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️parity/📋️orchestration/🟦️.ts:172` [console.log] test-path: yes; allocates: no
  - `console.log(`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️parity/📋️orchestration/🟦️.ts:175` [console.log] test-path: yes; allocates: no
  - `if (segments.includes("--metrics")) console.log(formatMetrics(metrics, readImplementationCoverage(repoRoot)));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:152` [[DEBUG]] test-path: yes; allocates: no
  - `/** 🐞️ One line carrying the '[DEBUG]' tag, which AGENTS.md reserves for temporary logs removed before a change lands. */`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:155` [[DEBUG]] test-path: yes; allocates: no
  - `/** 📖️ Every line of one source that carries the '[DEBUG]' tag. */`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:157` [[DEBUG]] test-path: yes; allocates: no
  - `return text.split("\n").flatMap((line, index) => (line.includes("[DEBUG]") ? [{ path, line: index + 1, text: line.trim().slice(0, 120) }] : []));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:163` [[DEBUG]] test-path: yes; allocates: no
  - `/** ▶️ The '[DEBUG]' census over every tracked text source outside the ticket tree, Markdown prose and`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:168` [[DEBUG]] test-path: yes; allocates: no
  - `const listed = runToolSyncV1("git", ["grep", "-l", "-z", "-I", "-F", "[DEBUG]", ...scope], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 });`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:182` [[DEBUG]] test-path: yes; allocates: yes: .filter(, .map(, new Map
  - `const oracle = new Map(runToolSyncV1("git", ["grep", "-c", "-I", "-F", "[DEBUG]", ...scope], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 }).stdout.split("\n").filter(Boolean).map((row) => [row.slice(0, row.l...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts` (16)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:48` [console.log] test-path: yes; allocates: yes: ${, Date.now
  - `const ticker = setInterval(() => console.log('[goal-gate] … ${check.id} running ${Math.round((Date.now() - started) / 1000)} s'), 60_000);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:172` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('[goal-gate] ▶ ${check.id}: repo run ${check.project}:${check.target}${args.length ? ' -- ${args.join(" ")}' : ""}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:280` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('[goal-gate] ▶ provider ${requirement}: repo run ${provider.project}:${provider.target}${args.length ? ' -- ${args.join(" ")}' : ""} → ${provider.url}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:282` [console.log] test-path: yes; allocates: yes: ${, .push(
  - `holders.push(await (options.provide ?? provisionGoalGateRequirement)(repoRoot, requirement, provider, args, env, logPath, options.signal, (line) => console.log('[goal-gate] … ${line}')));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:287` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[goal-gate] provider ${requirement} FAILED: ${reason}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:299` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[goal-gate] hubAdmin: the hub provider's admin capability ${path}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:329` [console.log] test-path: yes; allocates: no
  - `console.log("[goal-gate] ▶ backends: repo run os-hub-ts:backend-up -- all (the shared postgres + neo4j servers, zero-touch)");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:331` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[goal-gate] backends ${backendsReady ? "ready" : "NOT ready"} (${backendsLog})');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:334` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[goal-gate] step ${index + 1}/${selected.length} ${step.id} (${step.mode}, ${step.checks.length} checks): ${step.title.en}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:348` [console.log] test-path: yes; allocates: yes: ${
  - `for (const check of checks) console.log('[goal-gate] ${check.status.toUpperCase().padEnd(7)} ${check.check} (${Math.round(check.durationMs / 1000)} s) — ${check.summary.en}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:392` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('${step.id} (${step.mode}${step.gatesRest ? ", gates the rest" : ""}${step.optional ? ", optional" : ""}) — ${step.title.en} / ${step.title.de}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:393` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `for (const check of step.checks) console.log('  ${check.id}: repo run ${check.project}:${check.target}${check.args.length ? ' -- ${check.args.join(" ")}' : ""} [criteria ${check.criteria.join(",")}; needs ${check.requ...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:420` [console.log] test-path: yes; allocates: no
  - `console.log(goalSummaryMarkdown(summary, plan, "en"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:421` [console.log] test-path: yes; allocates: no
  - `console.log(goalSummaryMarkdown(summary, plan, "de"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:422` [console.log] test-path: yes; allocates: yes: ${
  - `for (const outcome of summary.outcomes) console.log('[goal-gate] outcome ${outcome.id} ${outcome.status.toUpperCase()} (pass ${outcome.totals.pass}, fail ${outcome.totals.fail}, blocked ${outcome.totals.blocked}, skip...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🚦️goal-gate/🟦️.ts:423` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[goal-gate] records: ${outDir}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts:213` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[acceptance] ${result.check} ${result.status.toUpperCase()} — ${result.summary.en}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts:214` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[acceptance] ${result.check} ${result.status.toUpperCase()} — ${result.summary.de}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️execution/🎬️scenario/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️execution/🎬️scenario/🟦️.ts:68` [console.log] test-path: yes; allocates: no
  - `if (probe.stdout.trim() !== "") console.log(probe.stdout.trimEnd());`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs:266` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("usage: host --plan <plan.json> --out <results.jsonl>");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs:273` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("usage: host --plan <plan.json> --out <results.jsonl>");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs:280` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("cannot read plan {}: {}", plan_path, error);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs:287` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("malformed plan {}: {}", plan_path, error);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs:293` [eprintln!] test-path: yes; allocates: yes: println!
  - `Err(error) => { eprintln!("{error}"); return std::process::ExitCode::from(2); }`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs:339` [eprintln!] test-path: yes; allocates: yes: println!
  - `eprintln!("cannot write results {}: {}", out_path, error);`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts:89` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('[inventory] ${coordinate}: ${inventory.mutations.length} runtime mutation(s), ${manifest.mutations.length} declared, ${drift} difference(s) → ${relative(this.repoRoot, path).split(sep).join("/")}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts` (8)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts:27` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(probes, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts:32` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[probe] ${probe.id.padEnd(28)} ${probe.kind.padEnd(17)} engine=${engineFamilyId(probe.engine)}@${probe.engine?.version ?? "*"} deterministic=${probe.deterministic} qualification=${status}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts:33` [console.log] test-path: yes; allocates: yes: ${
  - `for (const criterion of probe.qualification?.criteria ?? []) console.log('[probe]   ${criterion.met ? "✔" : "✘"} ${criterion.id}${criterion.detail === undefined ? "" : ' — ${criterion.detail}'}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts:36` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[probe] ${probes.length} probe(s), ${unqualified.length} not yet qualified');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts:37` [console.log] test-path: yes; allocates: yes: ${
  - `for (const probe of unqualified) console.log('[probe] ${probe.id}: RUNS and REPORTS; no release gate may claim its strongest guarantee until the qualification spike passes');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts:52` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ baselineSha, rows, measurements }, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts:55` [console.log] test-path: yes; allocates: yes: ${
  - `for (const measurement of measurements) console.log('[matrix] ${measurement.dimension.padEnd(32)} ${(measurement.ratio * 100).toFixed(2).padStart(6)}%  ${measurement.covered}/${measurement.total}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️coverage/📋️orchestration/🟦️.ts:56` [console.log] test-path: yes; allocates: no
  - `console.log(formatCoverageQuestions(registry, rows, measurements));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️reporting/📋️orchestration/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️reporting/📋️orchestration/🟦️.ts:70` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('[report] ${results.length} result(s) from ${relative(this.repoRoot, stream).split(sep).join("/")}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📊️reporting/📋️orchestration/🟦️.ts:84` [console.log] test-path: yes; allocates: no
  - `console.log(formatMetrics(metrics, perImplementation));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts:103` [console.log] test-path: yes; allocates: yes: ${
  - `.register("inventory-provider-check", class extends Script { run(args: string[]): void { if (args.length) throw new Error("inventory-provider-check accepts no overrides"); console.log('mutation inventory provider: ${r...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/📋️orchestration/🟦️.ts` (6)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/📋️orchestration/🟦️.ts:81` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[dependency] baseline rewritten with ${sorted.length} classified entries');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/📋️orchestration/🟦️.ts:88` [console.log] test-path: yes; allocates: yes: ${, .filter(
  - `console.log('[dependency] live scan: ${scanned.length} declared external dependenc(ies), ${scanned.filter((entry) => entry.productionReachable).length} production-reachable');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/📋️orchestration/🟦️.ts:90` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('[dependency] scan-only ${entry.ecosystem}:${entry.name}@${entry.version} kinds=${entry.kinds.join(",")} users=${entry.users.slice(0, 2).join(",")}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/📋️orchestration/🟦️.ts:95` [console.log] test-path: yes; allocates: yes: ${, .map(, new Set
  - `console.log('[dependency] ecosystems=${new Set(sorted.map((entry) => entry.ecosystem)).size} entries=${sorted.length} production-reachable=${production.length} test-oracle=${oracleDeps.length}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/📋️orchestration/🟦️.ts:96` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `for (const entry of oracleDeps) console.log('[dependency] test-oracle ${entry.ecosystem}:${entry.name}@${entry.version} (${(entry.oracleIds ?? []).join(",")})');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/📋️orchestration/🟦️.ts:98` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `for (const [, entry] of recorded) console.log('[dependency] production-debt ${entry.package} (oracle ${entry.id}) reachable from ${entry.productionDebt!.reachableFrom.join(", ")} — owner ${entry.productionDebt!.owner}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/⚖️comparison-profiles/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/⚖️comparison-profiles/🟦️.ts:211` [console.log] test-path: yes; allocates: yes: .filter(, .map(, JSON.stringify
  - `writeFileSync(probeScript, 'const [probe, ...rest] = process.argv.slice(2);\nconst inputs = rest.filter((_, index) => rest[index - 1] === "--input");\nif (probe === "broken") { console.log("not json"); process.exit(1)...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🚷️discovery-boundaries/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🚷️discovery-boundaries/🟦️.ts:101` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const code='import{receiveScriptProcessInvocation,createIssuedResourceControlV1,withScriptInvocationSignalV1}from ${JSON.stringify(pathToFileURL(join(repo,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️rou...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🚷️discovery-boundaries/🟦️.ts:144` [console.log] test-path: yes; allocates: yes: .push(, JSON.stringify
  - `const oracle=spawnSync(registeredToolExecutableV1("node"),["--input-type=module","--eval",'const events=[];let ready=false;const bootstrap={then(accept){ready=true;events.push("ready");accept();}};await bootstrap;if(!...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧱️command-composition-source/🟦️.ts` (1)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧱️command-composition-source/🟦️.ts:21` [console.log] test-path: yes; allocates: yes: ${, JSON.stringify
  - `const witness = 'const {spawn}=require('node:child_process'),{createHash}=require('node:crypto');const child=spawn(${JSON.stringify(runtime)},['--eval',${JSON.stringify(program)}],{stdio:['ignore','pipe','pipe']});con...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/⚖️mutation-payload-parity/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/⚖️mutation-payload-parity/🟦️.ts:588` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(census ? report.census : report, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/⚖️mutation-payload-parity/🟦️.ts:594` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log(["owner", "leaves", "witnessed", "fixtures", "negatives", "invariants", "rows", "clean", ...classes].join("\t"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/⚖️mutation-payload-parity/🟦️.ts:595` [console.log] test-path: yes; allocates: yes: .join(, .map(
  - `for (const row of report.census) console.log([row.owner, row.leaves, row.witnessed, row.fixtures, row.negatives, row.invariants, row.rows, row.clean, ...classes.map((name) => row.findings[name] ?? 0)].join("\t"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/⚖️mutation-payload-parity/🟦️.ts:596` [console.log] test-path: yes; allocates: yes: ${
  - `} else for (const entry of report.diagnostics.slice(0, 40)) console.log('[schema mutation-payloads]   ${entry.path} — ${entry.detail}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/⚖️mutation-payload-parity/🟦️.ts:597` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[schema mutation-payloads] ${total.clean}/${total.fixtures} fixture payload(s) and wire-form feature row(s) meet their leaf schema (${total.negatives} negative witness(es) it must reject, ${total.invarian...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/✏️mutation-editability/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/✏️mutation-editability/🟦️.ts:377` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(census ? report.census : report, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/✏️mutation-editability/🟦️.ts:383` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log(["owner", "aggregates", "leaves", "editable", "foreign", "inert", "withdrawOnly", "handwritten", "findings", ...classes].join("\t"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/✏️mutation-editability/🟦️.ts:384` [console.log] test-path: yes; allocates: yes: .join(, .map(
  - `for (const row of report.census) console.log([row.owner, row.aggregates, row.leaves, row.editable, row.foreign, row.inert, row.withdrawOnly, row.handwritten, row.findings, ...classes.map((name) => row.refused[name] ??...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/✏️mutation-editability/🟦️.ts:385` [console.log] test-path: yes; allocates: yes: ${
  - `} else for (const entry of report.diagnostics.slice(0, 40)) console.log('[schema mutation-editability]   ${entry.path} — ${entry.detail}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/✏️mutation-editability/🟦️.ts:386` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[schema mutation-editability] ${total.editable}/${total.leaves} leaves of ${total.aggregates} aggregates editable (${total.foreign} composite with foreign-step capability, ${total.withdrawOnly} withdraw-o...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🎛️mutation-input-ui/🟦️.ts` (7)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🎛️mutation-input-ui/🟦️.ts:485` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(census ? report.census : listed ? report.inputs : { diagnostics: report.diagnostics, census: report.census, multiline: report.multiline }, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🎛️mutation-input-ui/🟦️.ts:491` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log(["owner", "leaves", "withdrawOnly", "inputs", "declared", "inferred", "refused", "labelInferred", "findings", ...codes].join("\t"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🎛️mutation-input-ui/🟦️.ts:492` [console.log] test-path: yes; allocates: yes: .join(, .map(
  - `for (const row of report.census) console.log([row.owner, row.leaves, row.withdrawOnly, row.inputs, row.declared, row.inferred, row.inputs - row.declared - row.inferred, row.labelInferred, row.findings, ...codes.map((c...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🎛️mutation-input-ui/🟦️.ts:494` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log(["owner", "verdict", "pointer", "type", "widget", "role", "label", "step", "snaps", "path"].join("\t"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🎛️mutation-input-ui/🟦️.ts:495` [console.log] test-path: yes; allocates: yes: ${, .join(, .map(
  - `for (const input of report.inputs) console.log([input.owner, input.verdict, input.pointer, input.type ?? "", input.widget ?? "", input.role ?? "", input.label === null ? "" : Object.entries(input.label).map(([locale, ...`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🎛️mutation-input-ui/🟦️.ts:496` [console.log] test-path: yes; allocates: yes: ${
  - `} else for (const entry of report.diagnostics.slice(0, 40)) console.log('[schema mutation-inputs]   ${entry.scope} — ${entry.detail}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🎛️mutation-input-ui/🟦️.ts:497` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[schema mutation-inputs] ${total.declared} declared + ${total.inferred} inferred of ${total.inputs} input(s) of ${total.leaves} leaves (${total.inputs - total.declared - total.inferred} refused, ${total.w...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🏷️mutation-labels/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🏷️mutation-labels/🟦️.ts:175` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(census ? report.census : { diagnostics: report.diagnostics, census: report.census }, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🏷️mutation-labels/🟦️.ts:181` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log(["owner", "labels", "native", "forward", "findings", ...classes].join("\t"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🏷️mutation-labels/🟦️.ts:182` [console.log] test-path: yes; allocates: yes: .join(, .map(
  - `for (const row of report.census) console.log([row.owner, row.labels, row.native, row.forward, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\t"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🏷️mutation-labels/🟦️.ts:183` [console.log] test-path: yes; allocates: yes: ${
  - `} else for (const entry of report.diagnostics.slice(0, 40)) console.log('[schema mutation-labels]   ${entry.path} — ${entry.detail}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🏷️mutation-labels/🟦️.ts:184` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[schema mutation-labels] ${total.native} native and ${total.forward} forwarding of ${total.labels} leaf label(s); ${report.diagnostics.length} schema-mutation-label finding(s)${under === "" ? "" : ' under...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/📢️fault-notices/🟦️.ts` (5)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/📢️fault-notices/🟦️.ts:443` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(census ? report.census : { diagnostics: report.diagnostics, census: report.census }, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/📢️fault-notices/🟦️.ts:449` [console.log] test-path: yes; allocates: yes: .join(
  - `console.log(["owner", "codes", "labelled", "declared", "anonymous", "findings", ...classes].join("\t"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/📢️fault-notices/🟦️.ts:450` [console.log] test-path: yes; allocates: yes: .join(, .map(
  - `for (const row of report.census) console.log([row.owner, row.codes, row.labelled, row.declared, row.anonymous, row.findings, ...classes.map((name) => row.refused[name] ?? 0)].join("\t"));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/📢️fault-notices/🟦️.ts:451` [console.log] test-path: yes; allocates: yes: ${
  - `} else for (const entry of report.diagnostics.slice(0, 40)) console.log('[schema fault-notices]   ${entry.path} — ${entry.detail}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/📢️fault-notices/🟦️.ts:452` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[schema fault-notices] ${total.labelled} labelled of ${total.codes} guest fault code(s), ${total.declared} declared notice(s), ${total.anonymous} anonymous fault(s); ${report.diagnostics.length} schema-fa...`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts` (8)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:49` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify({ diagnostics, fixtures: reports }, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:54` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[test schema] ${diagnostics.length} invariant finding(s) over ${scope.length === 0 ? "the repository" : scope}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:55` [console.log] test-path: yes; allocates: yes: ${
  - `for (const [code, count] of [...byCode].sort((a, b) => b[1] - a[1])) console.log('[test schema]   ${String(count).padStart(5)} × ${code}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:56` [console.log] test-path: yes; allocates: yes: ${
  - `for (const entry of diagnostics.slice(0, 40)) console.log('[test schema]   ${entry.code} ${entry.path ?? entry.scope ?? ""} — ${entry.detail}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:57` [console.log] test-path: yes; allocates: yes: ${
  - `if (diagnostics.length > 40) console.log('[test schema]   … and ${diagnostics.length - 40} more');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:58` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[test schema] ${reports.length - failed.length}/${reports.length} schema-bound fixture(s) reached their declared stage');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:61` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[test schema]   ${report.outcome === "passed" ? "✔" : "✘"} ${report.fixture} ${report.uri} — ${line}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:62` [console.log] test-path: yes; allocates: yes: ${
  - `if (report.outcome === "failed") console.log('[test schema]     ${report.detail}');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧹️retention/📋️orchestration/🟦️.ts` (2)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧹️retention/📋️orchestration/🟦️.ts:17` [console.log] test-path: yes; allocates: no
  - `console.log(formatCleanReport(this.repoRoot, report));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧹️retention/📋️orchestration/🟦️.ts:43` [console.log] test-path: yes; allocates: no
  - `console.log(formatGcReport(report));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️contracts/📋️orchestration/🟦️.ts` (4)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️contracts/📋️orchestration/🟦️.ts:14` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(cases, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️contracts/📋️orchestration/🟦️.ts:17` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `for (const entry of cases) console.log('${entry.projectName}\t${entry.caseDir}\t[${Object.keys(entry.adapters).join(",") || "no-adapter"}]');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️contracts/📋️orchestration/🟦️.ts:18` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[discover] ${cases.length} test case(s)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️contracts/📋️orchestration/🟦️.ts:30` [console.log] test-path: yes; allocates: no
  - `console.log(formatBreachReport(breaches, cachePath));`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts` (26)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:63` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[manifest payload-schema] ${declared}/${leaves} leaves declare a payload contract at the taxonomy location (${((declared / Math.max(leaves, 1)) * 100).toFixed(1)}%)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:64` [console.log] test-path: yes; allocates: yes: ${
  - `for (const [why, count] of [...refusals].sort((a, b) => b[1] - a[1]).slice(0, 15)) console.log('[manifest payload-schema]   ${String(count).padStart(4)} × ${why.slice(0, 96)}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:65` [console.log] test-path: yes; allocates: yes: ${
  - `for (const leaf of undeclared.slice(0, 20)) console.log('[manifest payload-schema]   undeclared: ${leaf}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:66` [console.log] test-path: yes; allocates: yes: ${
  - `if (undeclared.length > 20) console.log('[manifest payload-schema]   … and ${undeclared.length - 20} more');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:95` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[manifest scaffold] ${derived}/${leaves} leaves derivable with full evidence (${((derived / Math.max(leaves, 1)) * 100).toFixed(1)}%)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:96` [console.log] test-path: yes; allocates: yes: ${
  - `for (const [field, count] of [...refusals].sort((a, b) => b[1] - a[1])) console.log('[manifest scaffold] refused ${String(count).padStart(5)} × ${field}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:97` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[manifest scaffold] ${ready.length} owner(s) fully derivable:');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:98` [console.log] test-path: yes; allocates: yes: ${
  - `for (const owner of ready) console.log('[manifest scaffold]   ${owner}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:99` [console.log] test-path: yes; allocates: yes: ${
  - `console.log(write ? '[manifest scaffold] ${written} descriptor(s) written' : "[manifest scaffold] dry run — pass --write to emit descriptors for the fully derivable owners");`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:136` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(rows, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:145` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('\n[gap] ${state.toUpperCase()} — ${group.length} catalog(s), ${mutationsIn(state)} mutation kind(s)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:147` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[gap]   ${row.capability.padEnd(34)} ${String(row.kinds).padStart(4)} kinds  ${String(row.fixtures).padStart(4)} fixtures  ${row.subset.padEnd(10)} ${row.owed}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:149` [console.log] test-path: yes; allocates: yes: ${
  - `if (!segments.includes("--all") && group.length > 12) console.log('[gap]   … and ${group.length - 12} more (--all to list, --json for the full record)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:154` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('\n[gap] ${covered}/${total} mutation kind(s) are externally oracled and manifested — ${((covered / Math.max(total, 1)) * 100).toFixed(1)}%');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:155` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[gap] ${mutationsIn("manifestable")} kind(s) have a qualifying oracle and still need a manifest, a runtime inventory and fixtures');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:156` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[gap] ${mutationsIn("supplemental-only") + mutationsIn("un-oracled")} kind(s) need a qualifying third-party oracle BEFORE any of that');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:158` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[gap] ${inventories}/${registry.mutationManifests.length} manifest(s) have a runtime inventory — a manifest's outcome classes cannot be stated honestly without one');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:160` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[gap] ${semioDerived} of ${registry.oracles.length} registered oracles are second implementations written inside this repository, and none of them discharges a mutation's requirement');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:208` [console.log] test-path: yes; allocates: no
  - `console.log(`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:223` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[manifest] ${totalDescribed}/${totalLeaves} mutation leaves carry a descriptor across ${rows.length} catalog(s)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:224` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[manifest] ${described.length} owner(s) fully described; ${ready.length} of those also have a qualifying oracle and are READY');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:225` [console.log] test-path: yes; allocates: yes: ${
  - `for (const row of ready) console.log('[manifest] READY   ${row.capability.padEnd(32)} ${String(row.manifest?.mutations.length).padStart(4)} mutations  ${row.owner}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:228` [console.log] test-path: yes; allocates: yes: ${
  - `for (const [reason, count] of [...blocked].sort((a, b) => b[1] - a[1])) console.log('[manifest] BLOCKED ${String(count).padStart(4)} catalog(s): ${reason}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:231` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[manifest] dry run — pass --write to emit manifests for the ${ready.length} ready owner(s)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:262` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[manifest] wrote ${row.manifest.mutations.length} mutation(s) into ${contribution.manifestPath}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/🏗️authoring/🟦️.ts:264` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[manifest] ${written} manifest(s) written');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts` (7)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts:26` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[fixture verify] ${fixtures.length} fixture(s), ${bad} file problem(s)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts:44` [console.log] test-path: yes; allocates: yes: JSON.stringify
  - `console.log(JSON.stringify(rows, null, 2));`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts:48` [console.log] test-path: yes; allocates: no
  - `console.log(`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts:52` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[fixture audit] ${rows.length} fixture(s), ${bad} with contract problems');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts:88` [console.log] test-path: yes; allocates: yes: ${, .filter(
  - `console.log('[fixture reproduce] ${fixtures.filter((entry) => entry.class === "third-party-generated").length} generated fixture(s), ${failed} problem(s)');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts:110` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[fixture generate] ${fixture.id}: ${fixture.files.length} file(s) into the content-addressed store');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧾️provenance/📋️orchestration/🟦️.ts:112` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[fixture generate] ${generated} fixture bundle(s) generated — commit review is a separate, human step');`

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🩺️environment/📋️inspection/🟦️.ts` (3)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🩺️environment/📋️inspection/🟦️.ts:12` [console.log] test-path: yes; allocates: no
  - `console.log(`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🩺️environment/📋️inspection/🟦️.ts:39` [console.log] test-path: yes; allocates: yes: ${
  - `console.log('[doctor] ${implementation}: ${ok ? probe.stdout.trim().split("\n")[0] : "MISSING"}${required ? " (required by a discovered case)" : ""}');`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🩺️environment/📋️inspection/🟦️.ts:42` [console.log] test-path: yes; allocates: yes: ${, .join(
  - `console.log('[doctor] cache root: ${relative(this.repoRoot, testCacheDir(this.repoRoot, taxonomy.testOutputChildDirs[0]!)).split(sep).join("/")}');`

