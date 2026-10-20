# Result fnsplit4a-finish (wave 1020e)

READY: staged set `C/fnsplit4a-finish-tools/staged/` (116 ops: 64 modify, 52 create, longest path 161), dry-run against the live tree: pending 116, drift 0. NOTHING WRITTEN to the repo (live-apply freeze); all work ran in the write-isolated overlay `/private/tmp/fnsplit4a-finish/ov` (symlink farm, real copies only of the touched directories), one process at a time, `nice -n 10`, no `bun repo`/cargo/bazel, no repo-wide scan (consumer lookups: `git grep -l -F` of one exported identifier per moved name; reader lookup: leaf directory names in tests/laws).

Apply (after the freeze): `python3 C/fnsplit4a-finish-tools/staged/apply.py` (dry run, default), then `--write` (anchored: every modify needs the live sha256, every create needs the path absent; any drift refuses the whole set; temp files first, backup in `staged/backup/`, rollback on error), `--undo` restores from the backup, `--verify` compares live hashes. Afterwards `zsh C/fnsplit4a-finish-tools/check-live.sh` (anchors + shape law on the live tree). If a peer edits one of the 116 paths first the apply refuses; re-derive with `stage.py` from the overlay only after re-copying that file.

## Part A: shape findings of the 11 directories (21 files, 0 findings after)
Law (`shape.mjs`, stricter than `fnsplit2-tools/newcheck.py`: tsx heads are checked too): path <= 255 units and components <= 255 bytes, module doc head emoji = folder emoji, doc emojis unique inside the file, folder emoji RGI and unique among siblings. Before: 21 findings (LayeredOverview 4, fit-and-presence 1, tutorial-orchestration 1, document-transfer-tabs 1, artifact-opening 3, shell-uri-documents 1, directory-route 1, home-grid 1, live-replay 3, mounted 2, transaction-v2 3). Fix = data-driven `emoji-spec.mjs`/`emojifix.mjs`: 12 module doc heads inserted (folder emoji), 26 duplicate doc emojis reassigned (old emoji asserted per line), plus the LayeredOverview `pane` head. Proof per file: TypeScript AST token stream (JSDoc excluded) equal before/after, head = folder emoji, no duplicate. After: `shape.mjs` over the 11 dirs = 0 problems, taxonomy resolution (`tax.ts`, now runnable) of the 58 new directories = 0 unresolved.

## Part B: candidates 27-49 of plist2 (23 functions): 16 staged, 7 not
Tool fixes (copies in `C/fnsplit4a-finish-tools/`): `psplit2` now rebases relative and absolute `import("...")` specifiers (phase bodies, shared, helpers), hoists local `type`/`interface` declarations of the body to the shared file, keeps `export ... from` statements interleaved between imports, no longer double-exports moved exported declarations (the `/m` regex put `export` into nested lines), refuses `import.meta`/`__dirname` in moved code, names inferred types that are not in scope with their `import()` form, errors instead of crashing on flows from kept statements. `fcons.cjs` (new): for exported declarations that a split moves into the shared file it rewrites every importer/barrel (`import`/`export { x } from`) to the shared file, no re-export left behind, `export *` of the root is an error. `tsdiff` got `TSDIFF_REPO` (overlay), `TSDIFF_TYPES=node,bun` (baseline 0 diagnostics instead of 10-18 environment errors) and `TSDIFF_ALSO` (consumers compared original vs new). Overlay isolation: `mkov.py`; staging: `stage.py` (refuses stale copies: live mtime after the overlay copy), `staged/apply.py`.

Staged (type-check differential with node+bun types, consumers included, `TSDIFF CLEAN` = 0 new distinct diagnostics for all 16; re-run after the last edits; titles of registration functions identical via `titles.cjs`):

| function | file dir | before | root after | largest piece after | pieces |
| --- | --- | ---: | ---: | ---: | ---: |
| `toolJobCoverageRun` | `🎯️tool-job-coverage/🏁️run` | 324 | 42 | 153 | 3 |
| `useAgentBridge` | `🧱️elements/🔗️AgentBridge` | 325 | 6 | 223 | 4 |
| `registerTests1` | `🧪️tests/🧪️actorreturn-codecs-load-natively-in-node-strip-only-mode-and-preserve-ev` | 327 | 5 | 149 | 4 |
| `inspectRustNonRepoJoinBaseSpans` | `🔍️discovery/🔀️rust-non-repo-join-bases` | 348 | 7 | 157 | 4 |
| `registerTests1` | `🧪️tests/🧪️artifact-bootstrap-protocol` | 336 | 5 | 194 | 4 |
| `toolJobLatestWinsSelfTests` | `🧪️tests/🔬️tool-job-latest-wins` | 314 | 6 | 153 | 3 |
| `useCheckInAndSync` | `🏛️ShellHost/✅️check-in-sync` | 313 | 10 | 191 | 2 |
| `useCommandBar` | `🏛️ShellHost/🔎️command-bar` | 330 | 8 | 171 | 2 |
| `FrameworkOsShellInner` | `🧱️elements/🏛️ShellHost` | 344 | 298 | 298 | 2 |
| `useInkPointer` | `🖋️InkCanvasHost/👆️ink-pointer` | 308 | 6 | 143 | 3 |
| `Canvas2dHost` | `🧱️elements/📐️Canvas2dHost` | 356 | 50 | 147 | 4 |
| `registerPlaygroundActionAndPluginSourceTests` | `🧪️docklayoutstore/🧭️playground-actions-and-plugin-sources` | 306 | 5 | 135 | 3 |
| `registerSessionBrokerPortTests` | `🧪️tests/🔑️session-broker-port` | 325 | 5 | 167 | 4 |
| `useWorldHoverAndPlacement` | `🌐️World3dHost/👇️hover-and-placement` | 305 | 5 | 159 | 2 |
| `registerTests1` | `🧪️tests/🧪️actor-instance-close-fault-publication-fixture-preserves-watchdog-and-te` | 303 | 5 | 141 | 4 |
| `transformTestClosure` | `📦️crate-tree/🧪️test-closure` | 315 | 6 | 148 | 4 |

Exported-declaration moves with consumer rewrites: `Canvas2dHost` (`CAMERA_SYNC_DEBOUNCE_MS`: meshes-ref consumer + the package barrel `Canvas2dHost/🟦️.tsx`), `useAgentBridge` (12 declarations, 13 consumers incl. AgentChatPanel, AgentPresence, AgentApprovals, TaskManager, ShellHost identity ports and 4 AgentBridge tests), `toolJobCoverageRun` (1 test), `transformTestClosure` (11 crate-tree consumers). `FrameworkOsShellInner` only goes from 344 to 298 lines (two pieces, root still 298): kept because it is below 300 and clean, a second pass would need a design step.

Not staged (reverted in the overlay, reason):
- `registerDocumentOpenTests` (357) and `registerDocumentPackWireAdapterTests` (327): the generated pieces stay at 353 and 325 lines (one giant `describe`), a rename only.
- `runNativeBuildDriverV1` (346): the body is one minified statement run inside an inner `async () =>` closure of `withIssuedResourceOperationV1` (statements share lines): needs a closure extraction design (`bx`), not a phase split.
- `PixelEditingOverlay` (346): a moved module `let` (`strokePress`) is assigned from a phase (TS2632); needs a box/accessor.
- `startShow` (341): `let` locals cross phases and phases use values declared later (`pacer`, `grasp`, `announce`, `direction`, `told`).
- `validateTaxonomyTree` (318): phases need values produced by a kept (nested-return) statement.
- `inspectRustCompileReferences` (302): the split loses the contextual callback type of `onGeneratedTokens` (TS2322 in the rust phase), differential not clean.

## Verification and limits
- Type check: tsc differential per split (original text vs new tree) with `types: node,bun`, React types for tsx; sanity-checked that an injected error is reported. Not run: any test suite (live-apply freeze, load), taxonomy engine beyond `tax.ts`. Source-text readers: git grep of the 16 leaf directory names in `🧪️tests`, `⚖️laws`, `📜️script.ts` found only readers through `readModuleTreeSource` (reads the module tree) or imports; no reader of a moved function body. The repo-wide reader lookup beyond those pathspecs was not made.
- `apply.py` was proven on a scratch root (`/private/tmp`): dry run, write, verify, refused second write, undo, drift refusal (one file edited) all behave as described; scratch deleted.
- Residue left for the owners: `owned-locale-detector-retirement` test family (structurally broken generators), pre-existing shape findings at deeper levels (`LayeredOverview/🧪️tests/🧩️component` duplicate emoji and head, `mounted/*/🧪️tests` missing head and a duplicate folder emoji `🔬`, ShellHost sibling folder emoji collisions) are outside the law as applied before and were not touched.

## Files
Tools, specs and the staged set: `C/fnsplit4a-finish-tools/` (`mkov.py`, `fgo.sh`, `frun.sh`, `frevert*.sh`, `psplit2.cjs`, `psplit2.orig.cjs`, `fcons.cjs`, `tsdiff.cjs`, `fverify.sh`, `shape.mjs`, `shape-rows.txt`, `dirs11.txt`, `emoji-spec.mjs/.json`, `emojifix.mjs`, `dedupe-docs.mjs`, `stage.py`, `check-live.sh`, `cands.txt`, `specs/`, `size-table.txt`, `staged/{apply.py,manifest.json,blobs/}`). Overlay and logs: `/private/tmp/fnsplit4a-finish/` (delete after the apply).
