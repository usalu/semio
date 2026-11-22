# Result E49 (nx-watcher-tests) - halted by coordinator

## Done
1. Watcher option for the `nx-namespace` birth: new birth `nx-watcher-namespace` in `🧪️testing/🌱️genuine-birth` (`🔣️.json`, schema field `watcher`, `🟦️.ts`: binds the namespace with `bind(identity, graphId, impl, root, watcherId)`, watcherId = 24 hex of sha256(root, birth id), passed as SEMIO_NX_WATCHER_IDENTITY). The watcher birth uses its own derived graph identity, so it does not share the product's null-graph namespace.
   Suites: `⚡️caching/🗂️suites/🔣️.json` bootstrap-processes got `fileLaunchers` for `🔌️nx-plugin/📤️arguments/🧪️tests/📥️native-caller/🟦️.ts` and `📤️arguments/🧪️tests/🟦️.ts` -> `nx-watcher-namespace`. Debug code removed; locks released; landing recorded.
2. Watcher-gated resident tests re-run with delivered FSEvents (all via `repo test-file`):

| file | before (E13) | after |
|---|---|---|
| 🔭️watch | 6 pass / 8 skip | 14 pass / 0 fail / 0 skip |
| 🏛️service | 32 pass / 5 skip | 37 pass / 0 fail |
| 🏗️provision | gated | 17 pass |
| journal tests | gated | 9 pass |
| journal host | gated | 1 pass |
| journal fuzz | gated | 3 pass |
| journal world | gated | 1 pass |
| graph-hooks direct | gated | 1 pass |
| resident tests helper | - | 0 tests |

## Findings
- native-caller: with the watcher birth the birth now succeeds and the file runs, but 10 tests fail on fixture drift: the caller schema now requires `codeRoot`, the test's `inputs()` capabilities (lines 17, 107, 142, 173 of the test) lack it. Fix NOT landed (area lock `bootstrap` held by E47 for 20+ min). Add `codeRoot:"/repo"` where `native` is present, `codeRoot:capabilities.codeRoot` at line 173.
- `📤️arguments/🧪️tests/🟦️.ts` with the new launcher: 3 pass / 3 fail (static owner command `toBe`, dispatcher "capacity sized at 46%" refusal, nextest filter test timed out at 180 s). Not analysed.
- Live wedge (product): the plain `nx-namespace` birth (graph identity null, impl `acd636dd...`) fails with "Original committed Nx native funding required" because the live journal `.cache/b/n/m/storage-events.jsonl` holds an orphan `reserved` allocation 516cc48fac9a... (10:11, directory never created, no birth journal) that no recovery retires. `caller-environment` hits this too. Recovery owner: native-build storage (no code path retires a reserved row without namespace-created).

## Not verified
Full `native-caller` green; `📤️arguments` file; the other ~11 E43 table files were not given the launcher; `🔗️import-edges` needs another custody (not routed).
