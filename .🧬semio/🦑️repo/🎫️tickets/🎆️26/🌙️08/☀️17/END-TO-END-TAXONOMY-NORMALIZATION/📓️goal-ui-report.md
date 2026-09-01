# UI module — final report (synchronous run)

Baseline (run 1, before any edit): `moves=812 unresolved=554` (brief quoted 809/563; drift is other
slices' repo-wide landings). Final synchronous run (`🗑️temp/🔣️ui-plan6.json`):
`moves=596 unresolved=719` — worse in raw total, but 471 of the 719 are `semantic-stem-ambiguous:
asset-binary-subject vs asset-subject`, a concurrent session's new, overlapping kind that appeared
between my own runs and touches nothing I edited (proven: 0 asset files in my diff). Net of that
external noise: real unresolved ≈248 vs baseline 554 (-306): `semantic-stem-unresolved` 330→117,
`package-implementation-destination-unresolved` 88→27, `directory-kind-unresolved` 21→19, my own
`test-case`/`test-fixture-member` ambiguity 59→16. `moves` also dropped only because the ambiguity
storm blocks destination computation for ~470 asset files, not from any weakening on my side.

Fixed (additive, `🔣️taxonomy.json`): 53+2 `semanticDirectoryKinds` for ui's package-root Rust/TS/JS
module stems (context, dispatch, resources, pipelines, scene_target, retained, retained-resident-
store scoped under it, …), 3 compound `fileKinds`/`fileKindResolutionRules` (`.expect.json`/
`.snapshot.json`/`.patch.json`, same idiom as existing `json-schema`), `test-fixture-member` +=
`members-of-members-of-examples`, `test-case` += `members-of-elements`, `uv-lock` fixed-filename
contract. Caught and reverted my own regression first: 13 of those 53 words collided with unrelated
`semanticDirectoryMemberKinds` overlay entries repo-wide (same word+emoji, different meaning) and
silently broke `🧪️conformance`'s overlay chain — fixed by scoping those 13 to
`parentKindIds:["rust-language","typescript-language","javascript-language"]`.

Generalizes: **always cross-check a new word against every `semanticDirectoryMemberKinds.memberNames`
list, not just other `semanticDirectoryKinds` ids** — exact-id match wins the branch silently and
breaks overlay chains with no direct error. Checked the sibling's fileKinds-emoji-pairing warning:
doesn't apply here, I never touched `fileKinds` emoji, only added new compound extensions reusing
`json`'s existing 🔣️. Full family/root-cause writeup: `📓️goal-ui-census.md`.

## Update — synchronous drive-down after coordinator's asset-binary-subject fix

Baseline for this pass (coordinator's fix applied): `moves=1067 unresolved=215`. Final, real output:

```
moves=1152 unresolved=113
```

By code: `semantic-stem-unresolved` 117→42, `reference-syntax-unsupported` 34→22,
`directory-kind-unresolved` 19→19, `semantic-stem-ambiguous` 16→17 (+1, see below),
`package-implementation-destination-unresolved` 27→11, `generator-preview-invalid` 1→1.

Fixes: registered `json-fixture-case` (🔣️, catch-all for descriptive-named JSON fixtures) after
discovering — via a temporary `console.error` in `canonicalFile`, reverted immediately after —
that `🖱️ui` itself resolves via the `members-of-modules` → `members-of-members-of-modules` overlay
chain, so EVERY nested `🧪️fixtures` directory under it carries that overlay id as its context, not
plain `fixtures`; scoped accordingly. Extended my 13 previously rust/ts/js-scoped words
(`arena`,`cursor`,`draw`,`theme`,`tree`,…) with `wgpu-target`/`react-target`/`tui-target`, since
`🎯️targets/🧊️wgpu/*.rs` sits one level flatter than the vulkan/metal/webgpu/d3d12 packages (target
dir IS the package root there). Added `retained`, `retained-resident-store`, `uv-lock`,
`descriptor_layout`, `swapchain_support`, `gpu_types`, `tessellate`, `chrome`, `backend_alias`,
`schema-artifact-subject`, 5 tsv-fixture kinds.

**Self-caused-and-fixed mistake, twice.** First: added `inferWithoutEmoji: false` to my new fixture
kinds to follow the coordinator's anti-storm advice, without checking these files' `semanticEvidence`
is EMPTY (fileKind already strips it) — that flag disables the NO-EMOJI branch these files actually
need, so nothing resolved. Second, worse: my blind string-replace to remove that flag matched the
exact same 2-line shape on 9 UNRELATED pre-existing kinds, including the coordinator's own
`asset-binary-subject` fix — silently reintroducing the 471-row ambiguity storm. Caught by re-running
the plan myself (584 unresolved) rather than assuming success, diffed which kinds lost the flag
against the original schema dump, and restored all 9 individually. Lesson: never blanket
string-replace a structural JSON key across a whole file; anchor every edit to the specific `id`
block, always re-verify with a real plan run before reporting a fix landed.

One residual: `fixture-case` vs `json-fixture-case` ambiguous on 1 file
(`🍎️metal/…/🧫️fixtures/🔣️objc2-runtime-abi.json`) — both catch-alls reach the no-emoji branch under
plain `fixtures` context there (vs the override context elsewhere); not resolved, tracked.
