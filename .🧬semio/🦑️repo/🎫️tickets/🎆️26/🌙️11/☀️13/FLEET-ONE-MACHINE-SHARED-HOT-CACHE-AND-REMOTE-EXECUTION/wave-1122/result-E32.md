# Result E32 (adoption-wiring)

## Verdict
The read-only adoption of the installed Rust toolchain is WIRED into the sealed Rust generation and proven. The Nx SDK part is only half done: the identity and the read-only fence exemption are wired, but the SDK clone publication refuses a source outside the private store, so the SDK-bound files stay red. Of the 111 red files of E24 a fresh `repo test-file` pass (3 parallel, 280 s cap, partly while I was still editing) gave 6 green (units, verification, published, authorizers capacity, entry-law, activation), 5 timeouts, the rest red for reasons that are NOT the toolchain (listing at the end). After the batch I fixed further files: `📏️recensus` 1/0, `📏️capacity` 9/11 (see below), `🎟️capacity` sdk 4/8.

## What changed (files, all anchored edits under locks caching-bootstrap, caching-storage, testing, process-leases; landing recorded in `T/landings.md`)
- NEW `⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/🧰️adoption/` (schema first `🧬️schema/🔣️.json`: Protocol texts en/de, closed `AdoptedToolchain` row; `🟦️.ts`: `adoptRustToolchainReadOnlyV1(workspace,{storeRoot,directory,identity})` reads the live generation's `g/<gen>/intent.json` by plain reads, refuses a generation inside the private repository, registers it per workspace and admits the directory in the fence as read-only; `adoptedRustToolchainV1`, `adoptedRustCompilerDigestV1`). Tests `🧪️tests/🟦️.ts` 3/0: closed row + identity equals an independent `shasum -a 256` of the installed rustc; inside-repository refusal en/de; real sealed generation of a private repository (`withRepositoryCompilerStoreV1`) is the adopted one (digest, root, rustc path, RUSTUP_HOME in the live store, CARGO_HOME in the private store, `requireRepositoryRustToolchainV1` and `repositoryRustCompilerIdentityV1` accept it), live directory and store mtime unchanged, write/lease/mount/remove still refused.
- `🦀️rustup/📥️installation/🟦️.ts`: `withRepositoryRustToolchainOwnedV1` branches right after `sourceRequest` (before bootstrap, cargo tools, leases, publication): an adopted workspace goes to the new `withAdoptedRepositoryRustToolchainV1`: no download, no `rustup:home` lease (shared or exclusive), no mount, no receipt or seal publication; `RepositoryRustToolchainV1` row built from the adopted directory (digest = sha256 of rustc, request/host/channel from the intent row), same custody registrations (`readers`, `readerControls`, `programDigests`, `compilers`) so every compiler-identity consumer works; `recheck` re-hashes rustc, re-reads the pin file and re-verifies the native tool receipts (read from the live store, `provision=false`). The `rust-toolchain.toml` pin is still read through `sourceRequest` from the private code root and its channel must equal the adopted channel. A non-adopted workspace runs the old path unchanged (rustup protocol tests 20/0).
- `🧪️testing/🧫️private-repository/🟦️.ts`: opening a private repository now writes `rust-toolchain.toml` at its code root (copy of the checkout's pin) and adopts the installed toolchain; also names `SEMIO_NX_TOOLING_IDENTITY` (newest complete Nx toolkit) when no runner granted one. `🧰️adoption/🟦️.ts`: `storeRoot` in the row (+schema), optional `wanted` identity, newest-complete-install rule for Nx, registers every adopted directory in the fence, `adoptInstalledRustToolchainForV1`, `adoptInstalledNxToolkitIdentityV1`. Adoption tests 5/0.
- `🔨️modules/🏃️process/🔒️leases/🎛️control/🟦️.ts`: `adoptMaterializedReadOnlyDirectoriesV1`; `requireMaterializedPathV1` lets a path inside an adopted directory pass the live-store fence. The fence for images, lease databases and partitions of the live store is unchanged (adoption test "live store fence still refuses an image path of the live store" stays green); a writer of a private store cannot address an adopted directory because every store path is bound to the store root (`requireNativeStoragePathV1`), which is what made the SDK source read fail (below).
- Tests repaired: `📦️artifacts/🏗️native-build/📥️published/🧪️tests` (scratch inside the private repository; every expected refusal in its own `repository.invoke`, because a refusal closes the Original) 1/0; `💾️storage/.../🏗️compiler/🧪️tests/🕸️units` (`repository.invoke` instead of the checkout's Original; stale refusal texts) 11/0; `🧹️pruning/.../📏️recensus` + `📦️compact` law (isolated build test with declared primary rejection, `checkpoint` after the refusal assertions) 1/0 (125 s); `📏️capacity/🧪️tests` (below); `🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests` (store helper on the private repository + adopted SDK source) 4 pass of 8.

## `📏️capacity` (split done, 9 of 11 pass)
The three laws that end in an expected refusal now run as isolated build tests (`registerIsolatedBuildTestV1`, each its own process): second writer refused by headroom (declared rejection `/initial headroom/`), real ENOSPC (`/ENOSPC|space/`; the follow-up positive write after ENOSPC cannot run in a closed Original and was dropped, it is the law that only a fresh process can prove), retired lifetime (plain isolated pass) and the inactive-store retirement. Phase bodies end by `checkpoint()`, detach their image by `detachDiskImagesUnderV1` and skip admitted cleanup (a closed Original refuses it). Also: `repo.prepare("diagnostics")` ahead, realpath scratch, the control of the law 45 s (the wait after a refusal runs to the control deadline), and the two places that copied the issued control with `{...control, publish}` (the product requires the issued object; the copy gave "Live Original resource control issuance required") now use the control itself and a 1.5 s observation window instead of the publish hook. Last run: 9 pass, 2 fail: `one received Original retains ... image exclusion and repeatable compact recovery laws` fails with "Native borrower live frame required" after waiting 120 s: the non-awaited `future=openNativeImageStorageV1(...)` started inside `withExclusiveMaintenance` leaves the borrower frame. That is a product/test contract question of the image-exclusion law (owner of `💾️storage/.../🏗️native-build`), not adoption; NOT fixed. The whole file takes about 325 s under load 30, above the 300 s plain budget of `test-file` when the combined law hangs (it is the one that runs 120 s).

## NOT done / NOT verified
- Nx SDK reader: `testNxToolkitDirectoryV1` was already a sanctioned read; the harness now provides the identity and the fence admits adopted `nx`/`sdk` directories. The files that need a real SDK stay red because (a) `withNxSdkClonePublicationV1` reads the source below the store root only ("Native storage path escapes enforcing boundary": the clone source must be a path of the private store; an adopted foreign source needs a product seam like the Rust one), (b) the files in `🧪️tests/🌍️caller-environment`, `🔌️nx`, `🕸️graph/🔏️seal/*` additionally need the full bounded Nx namespace environment that the Nx task wrapper builds (`SEMIO_NX_FIXED_STORAGE`, `SEMIO_NX_GRAPH_STORAGE`, watcher identity, daemon sockets, `SEMIO_STARTUP_SDK`): `repo test-file` does not provide it (E26/E28 area; I did not touch `🔬️test-file`). With only the identity set, `caller-environment` went from 0 pass to 3 pass 8 fail on those namespace errors.
- The batch of 111 files ran while I edited, so single numbers may differ; no whole `⚡️caching` rerun. `🏗️build/📥️caller/🧪️tests/📥️birth` has 6 structural failures that are not from adoption (not in my list, not touched).
- The adopted toolchain's `rustc` hash is computed on every `recheck` (a few hundred KB read); a toolchain replaced in the live store while a test runs is refused, not followed.
- Other lanes: a private repository now always has `rust-toolchain.toml` at its root and an adopted toolchain whenever the checkout has an installed one; tests that provision or assert a fresh private toolchain must not rely on a private repository for that (rustup protocol/installation tests do not).

## Remaining red among the 111 (suite id order in file; error head)
Counts by first error: expect-mismatch/stale expectation about 25 (toBe/toEqual/toContain/toThrow drift), "Original ... required" (SDK/Nx namespace/caller/artifact repository/captured runtime entry provider/native caller capabilities/native platform image/setup-all asset purpose) about 25, Build test law body deadline exhausted 3 (sdk source derivative, at load 30+), 5 file timeouts at 280 s (`closures/tests`, `seal/selection`, `nx-plugin/arguments`, `authorizers/tests`, `authorizers/closure-law`), the rest single causes. The per-file line list (rc pass/fail file :: error) follows.

- 1 0/3 💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/⌛️wait/🟦️.ts :: host-pressure-does-not-drop-ratio
- 1 4/2 💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/⚡️recovery/🧪️tests/🟦️.ts :: rust/generic-macos-receipt-without-platform
- 1 1/1 💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🎟️reservation/🧪️tests/🟦️.ts :: 
- 1 2/3 💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️queued/🟦️.ts :: expect(received).toEqual(expected)
- 1 3/6 💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️required/🟦️.ts :: Actual required cache consumer entry missing
- 1 4/6 💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📦️child/🟦️.ts :: expect(received).toBeInstanceOf(expected)
- 1 0/2 💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/⚓️anchor/🟦️.ts :: Original captured runtime entry provider required
- 1 1/11 💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build/📦️owner/🟦️.ts :: success
- 1 3/4 💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build/🟦️.ts :: Explicit original setup-all asset purpose required
- 1 1/2 💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker/🟦️.ts :: 125 |  const work=Promise.resolve().then(async()=>{await ready;if(!startFixture)throw Error("Actual 
- 1 2/1 💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/📐️layout/🟦️.ts :: expect(received).toBe(expected)
- 1 2/2 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📇️formats/🟦️.ts :: 
- 1 0/1 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🔄️cohort-controller/🟦️.ts :: small-unknown-reuse
- 1 4/1 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/📡️session/🟦️.ts :: closure-admitted-birth:{"version":1,"ingress":"birth-owning-program","owner":"test-file","maximumEla
- 1 1/2 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🧪️admitted-worker-harness/🟦️.ts :: compileNxAuthorizerSnapshotV1
- 1 0/1 💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🟦️.ts :: expect(received).toBe(expected)
- 0 11/0 💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🕸️units/🟦️.ts :: 
- 1 0/2 💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts :: publish:/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%9A%EF%B8%8Flibrary/%E2%9A%A1%EF%B8%8Fcaching/%F0%9F%
- 1 10/1 💾️storage/🔒️enforcement/🔌️nx/📦️graph/📥️inputs/📋️data/🧪️tests/🟦️.ts :: Required original cache consumer unavailable
- 1 2/4 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/⏱️deadline/🟦️.ts :: expect(received).toContain(expected)
- 1 17/2 💾️storage/🔒️enforcement/📦️budget/🧪️tests/🟦️.ts :: Actual Native platform image required
- 1 0/2 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/📜️script.ts :: Exact graph purpose context required
- 1 2/2 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/🧪️tests/🟦️.ts :: Exact carried graph purpose receipt required
- 1 0/1 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts :: expect(received).toBe(expected)
- 1 13/34 💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts :: expect(received).toBeGreaterThanOrEqual(expected)
- 1 1/2 💾️storage/🔒️enforcement/🔌️nx/📦️outputs/👁️observation/🧪️tests/🔑️kernel/🟦️.ts :: expect(received).toBe(expected)
- 1 1/2 💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/📥️purpose/🧪️tests/🟦️.ts :: 
- 1 7/8 💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts :: expect(received).toEqual(expected)
- 1 3/1 📥️inference/🧪️tests/🔍️imports/🟦️.ts :: 
- 1 3/1 💾️storage/🧾️ledger/🧪️tests/⚖️law/🌐️native/🟦️.ts :: stray cache roots (delete them and run the command from the checkout root)
- 1 5/1 📦️artifacts/🏗️native-build/⏱️observation/🧪️tests/🟦️.ts :: expect(received).toBe(expected)
- 1 2/1 📥️inference/🧪️tests/🟦️.ts :: Reducing finite Source revision request required
- TIMEOUT None/None 💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🟦️.ts :: 
- 1 2/2 📦️artifacts/📋️native-orchestration/📥️invocation/🧪️tests/🟦️.ts :: Original Repo build caller required
- 1 5/6 📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts :: [script] {"owner":"native-owner-command-original-law","command":"native","stage":"loading"}
- 1 0/9 📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts :: Original native caller capabilities required
- 1 1/15 📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts :: Actual artifact repository authority required
- 1 0/1 📦️artifacts/🔒️bounded/🧪️tests/🍎️image/🟦️.ts :: 
- 1 6/1 📦️artifacts/🔒️bounded/🧪️tests/🟦️.ts :: expect(received).toBe(expected)
- 1 0/2 📦️artifacts/🧪️tests/🟦️.ts :: Isolated build test did not end with its declared primary rejection: repository artifact publisher p
- 1 1/2 🔍️discovery/📂️source/🧪️tests/🟦️.ts :: 
- 0 2/0 🕸️graph/✅️verification/🧪️tests/🟦️.ts :: 
- 1 0/2 🕸️graph/🏁️startup/🚦️service/🧪️tests/👁️outputs/🧊️provider/🟦️.ts :: Isolated build test did not end with its declared primary rejection: admitted consumer refuses missi
- 0 1/0 📦️artifacts/🏗️native-build/📥️published/🧪️tests/🟦️.ts :: 
- 1 0/1 🕸️graph/🏇️waiter/🧪️tests/🟦️.ts :: Original installed tooling identity and ticket artifact directory required
- 1 0/2 🕸️graph/🏁️startup/🚦️service/🧪️tests/🟦️.ts :: Original loaded SDK root required
- 1 1/1 🕸️graph/🏁️startup/🚦️service/🧪️tests/🔭️startup-journal/🟦️.ts :: Workspace journal cannot vouch: No change journal service is running; the tree is walked.
- 1 6/12 🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests/🟦️.ts :: Required original cache consumer unavailable
- 1 1/2 🕸️graph/📤️publication/🧪️tests/📦️transaction/🟦️.ts :: Build test receiver closed with active callback
- 1 1/5 🕸️graph/📤️publication/🧪️tests/🟦️.ts :: Exact canonical persistent Native publication required: /Users/ueli/Documents/semio/.cache/test-sand
- 1 None/None 🕸️graph/🔏️seal/📥️membership/📥️journal/🧪️tests/🟦️.ts :: 
- 1 0/3 🚀️bootstrap/📥️admission/💾️storage/🧪️tests/🟦️.ts :: Original canonical collection graph backing required
- 1 0/4 🕸️graph/🔏️seal/🧪️tests/🟦️.ts :: Nested persistent image requires actual parent scope
- 1 1/2 🚀️bootstrap/📦️dependencies/💾️storage/📥️read-context/🧪️tests/🟦️.ts :: 
- 1 1/1 🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🆔️identity/🟦️.ts :: expect(received).toEqual(expected)
- 1 0/2 🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/📥️recheck/🟦️.ts :: 
- 1 44/41 🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts :: 
- 1 0/3 🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🔭️providers/pressure.test.ts :: 
- 1 28/20 🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts :: expect(received).toThrow(expected)
- 1 5/1 🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/📄️generation/🧪️tests/🟦️.ts :: alias-drift: undefined
- 1 1/1 🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🗃️unready/🧪️tests/♻️roles.ts :: unready-generation:none
- 1 3/1 🚀️bootstrap/📥️invocation/🧪️tests/🟦️.ts :: expect(received).toThrow(expected)
- 1 14/1 🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts :: expect(received).toEqual(expected)
- 1 90/4 🚀️bootstrap/📦️dependencies/🔌️managers/🧪️tests/🟦️.ts :: expect(received).toBe(expected)
- 1 7/1 🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🛡️followup/🟦️.ts :: expect(received).rejects.toThrow(expected)
- 1 0/1 🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/📥️native-caller/🟦️.ts :: Original bounded Nx namespace required
- 1 8/11 🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts :: Managed Python test interpreter binding required
- 1 0/1 🚀️bootstrap/🔌️nx-plugin/🧪️tests/🦀️native-dependency-closures/🟦️.ts :: crates/a tests=false
- TIMEOUT None/None 🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts :: 
- 1 1/3 🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📥️purpose/🧪️tests/🟦️.ts :: 
- 1 0/2 🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📬️task-child/🟦️.ts :: Original SDK and artifacts required
- 1 0/1 🚀️bootstrap/🛠️tools/📥️use/🧪️tests/🟦️.ts :: Original artifacts and immutable SDK identity required
- 1 0/1 🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative/⚠️failure/🟦️.ts :: expect(received).toBe(expected)
- 1 3/5 🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative/🟦️.ts :: A test reached the live repository store of the checkout /Users/ueli/Documents/semio; give it a priv
- 1 2/7 🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/🟦️.ts :: A test reached the live repository store of the checkout /Users/ueli/Documents/semio; give it a priv
- 1 None/None 🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🧪️tests/🟦️.ts :: 
- 1 10/1 🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests/🟦️.ts :: cut 🏗️build/📜️script.ts
- 1 0/11 🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🧪️tests/🟦️.ts :: Build test law body deadline exhausted
- 1 1/6 🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/🛂️sdk-fences/🟦️.ts :: Build test law body deadline exhausted
- 1 8/17 🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/🟦️.ts :: Build test law body deadline exhausted
- 1 2/2 🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📥️scope/🟦️.ts :: expect(received).toBe(expected)
- 0 10/0 🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📏️capacity/🟦️.ts :: 
- 0 3/0 🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🚪️entry-law/🟦️.ts :: 
- TIMEOUT None/None 🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/🟦️.ts :: 
- 0 4/0 🚀️bootstrap/🛠️tools/🔗️activation/🧪️tests/🟦️.ts :: 
- TIMEOUT None/None 🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts :: 
- TIMEOUT None/None 🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️closure-law/🟦️.ts :: 
- 1 1/2 🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🧪️tests/🟦️.ts :: /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e86
- 1 1/3 🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/📥️negative/🧪️tests/🟦️.ts :: Exact private retention workspace required
- 1 1/2 🦀️cargo/📦️units/🏗️compiler/🔏️code/🧪️tests/📖️reader/🟦️.ts :: expect(received).toBeLessThanOrEqual(expected)
- 1 5/13 🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts :: expect(received).toBe(expected)
- 1 17/7 🦀️cargo/📦️units/📥️descriptor/💾️cache/🧺️set/🧪️tests/🟦️.ts :: expect(received).toHaveLength(expected)
- 1 0/1 🦀️cargo/📦️units/🧪️tests/📥️source/🟦️.ts :: A test reached the live repository store of the checkout /Users/ueli/Documents/semio; give it a priv
- 1 0/2 🚀️bootstrap/🛠️tools/🧪️tests/🟦️.ts :: expect(received).toBe(expected)
- 1 2/9 🧪️tests/🌍️caller-environment/🟦️.ts :: Original immutable SDK required
- 1 11/17 🦀️cargo/📦️units/🧪️tests/🟦️.ts :: expect(received).toThrow(expected)
- 1 3/1 🧪️tests/📋️values/🧪️tests/🟦️.ts :: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:projectBuildTargetForNxV1
- 1 0/1 🧪️tests/📐️phases/🟦️.ts :: 
- 1 2/6 🧪️tests/📦️native-dependencies/🧪️tests/🟦️.ts :: 
- 1 0/1 🧪️tests/🔗️import-edges/🏛️graph/🔑️authority/🟦️.ts :: Original bounded Nx namespace required
- 1 0/2 🧪️tests/🔗️import-edges/🏛️graph/🟦️.ts :: 
- 1 1/8 🧪️tests/🔗️import-edges/📁️paths/🟦️.ts :: Original bounded graph writer custody required
- 1 2/11 🧪️tests/🔗️import-edges/🔁️context/🟦️.ts :: Original bounded graph writer custody required
- 1 15/2 🧪️tests/🗃️compiler-state/🟦️.ts :: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/💾️storage/🧪️tests/🟦️.ts
- 1 0/1 🧪️tests/🗿️artifact-source/🟦️.ts :: Select the active ticket with --ticket <directory> or SEMIO_TICKET_DIR
- 1 0/1 🧪️tests/🛑️coordinator/🔮️oracle/🟦️.ts :: 
- 1 3/1 🧪️tests/🎮️playground-preparation/🧪️tests/🟦️.ts :: Original Script deadline refused
- 1 1/2 🧹️pruning/🏪️budget/📜️history/🧪️tests/🟦️.ts :: 
- 1 11/1 🧹️pruning/🏪️budget/🧪️tests/🟦️.ts :: 
- 1 1/0 🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests/🟦️.ts :: Isolated build test failed: managed tooling crash witness cannot block authenticated retirement afte
- 1 0/1 🧹️pruning/🏪️budget/📜️history/🧪️tests/📏️recensus/🟦️.ts :: Inactive Cargo unit changed under physical custody