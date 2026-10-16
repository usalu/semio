# Retirement Birth Successor Source Audit

Read-only Source review on 2026-10-16. No Native/image/mount/compiler/cache/network/runtime operation or Source edit was performed; no test was run. All 28 rows in the latest successor roster matched at the observation below (the operation schema is included). These hashes are observations rather than locks.

## Actionable Findings

### P2 — Teardown bypasses the new underlying-directory mode identity

In Native storage `🧹️cleanup/🎛️command/🟦️.ts:57`, the no-matching-image branch of `observe` calls `exact(scope.root, original, true)`. That helper at line 43 checks device, inode, no-link and directory kind but ignores mode. `removeSettledNativeImageV1` at line 62 also uses that weaker helper immediately before removal, and its persistent-existing branch can declare settlement without checking `scope.mountDirectory` at all. A chmod of the original underlying directory after activation therefore passes the cleanup identity checks although the new mount-directory contract deliberately includes mode and rejects this change. The creator captures `scope.mountDirectory` and activation/inactive opening use `requireNativeImageMountDirectoryV1`; final detached observation and teardown should use the same exact contract before effects/settlement. This is a static branch finding; no physical reproduction was attempted.

### P2 — Validate empty underlying root before unlinking the backing

At `🧹️cleanup/🎛️command/🟦️.ts:62`, created-directory inventory is checked, then receipt and backing are unlinked, and only afterward `names(scope.root,0)` checks for unknown entries. If the exact underlying root contains an unexpected ordinary entry, cleanup rejects and retains the owner after destroying its backing/receipt. The private scope still retains the original backing identity, so subsequent `check` also fails on the already deleted backing. Move the exact root and empty inventory preflight before the first unlink; retain the whole creator unchanged on unknown-entry refusal. This concerns the already implemented private creator cleanup, not missing whole-cache teardown. Static inspection only.

## Verified Source Boundaries

- Borrower cleanup capture refuses `scope.cleanup` before a second loan effect; the cross-token overwrite path is closed.
- `closeNativeImageBorrowerCleanupV1` drains actual accepted work, verifies the exact retained loan, releases the actual old guard, and returns the original body failure array as DATA. `undefined` is preserved by array membership rather than truthiness. Verification/release failure rejects and retains the actual scope in the finite unresolved set.
- Lease cleanup loan identity is held privately with original lease issuance, database and directory device/inode. Writer acquisition refuses a live original, consumes once, acquires real SQLite exclusive custody under the cleanup signal, and joins accepted cleanup children before release. The retirement variant retains the writer on failure. This does not establish funding or ownership of a child allocation.
- Captured persistent-image cleanup readers retain the receiver and loan until accepted physical cleanup settlement; image close awaits their release and pending reads before releasing the original borrower. Ordinary close is refused from its cleanup reader and from its accepted cleanup context.
- The mount-directory schema/creator and inactive reader use the same copied DATA contract; the mismatch is specifically the final private creator cleanup checks identified above.

## Precise Missing Constructor Transaction Boundary

The issued allocator reservation and child creator need one real private transaction before allocation effects: own reservation before journal append can fail; retain child request/settlement before child creation; carry the same preaccepted cleanup enrollment and incoming phase into the real canonical parent. Only after actual child EX, allocation/payload recensus, descriptor closure, closed detach/inactive/removal may parent charge and binding retire. Current loan/borrower APIs prove custody of existing leases; path DATA or later attaching child completion cannot prove this funding relationship. Whole-cache teardown is explicitly incomplete and no qualified teardown or new cache birth is claimed.

## Observed SHA-256

- `e4edb311307c6f081e7faeb6444b06148d65bb718302d259a91030d6ff4bbc9e` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🟦️.ts` (matched)
- `54815db155e46e55fb637da1aa8e9602964c95b2119165e5006b67c44c5dc571` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧬️schema/🔣️.json` (matched)
- `96160822fca8c2809103582b6a8f8a3d7643d6bd885306ec1540417dfc49ac4f` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧾️journal/🟦️.ts` (matched)
- `dd271dd1fb8f2437f1376123b080703fe15b7fceeeb91540920d422e5b82bfb4` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts` (matched)
- `bfbac87f3c58bfbdbb152954e642a38df3c0215159b4abacc8608250b564de2c` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🔣️.json` (matched)
- `76f262bdab0be60cc87314391d9160e849765650601c10e2749ad1ca7ac57645` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts` (matched)
- `d03dd51465cb1e63830b69b2b3186b0a90529952197c2600c66857cc0e15dcef` — `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` (matched)
- `05afde1006332ca3ff978bf066b9b51ff7a4e27bd06cbcea5ac373820f45261e` — `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧹️cleanup/🧬️schema/🔣️.json` (matched)
- `e824abd12449f6c63fc13819f4ea8f8bae85cf604158c535d4985897ce40a90f` — `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` (matched)
- `cdb3bf110c01fb5b1cbbea00c951d5ce5ffaae8af479a23050db9c6dcbf135d2` — `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/🧹️cleanup.json` (matched)
- `f3ffb9a17232449a168fb6af24a99e93208b4173dfe1a8ef7124999ad8619a81` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts` (matched)
- `30cd991b9615da11c5201f6d3ad63db28eafc161b583598b955ee7aa2d447495` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts` (matched)
- `e341956611e400b1b987374fea5e13a862ef3cbdb6191e9f7f33eba3ed0cb5ca` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json` (matched)
- `1141b707933485accfd3358887d322303973f3fb67aa3ab4942b716e5dfde0fb` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts` (matched)
- `34a2618d9973bd5e27bd9fa012b9b0ead2c6424ae4e9d50a9941a25583144281` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧬️schema/🔣️.json` (matched)
- `8cfb2140215f03a1c61751b18842673c88b9bcd090e27825c720046f3f23421b` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🎛️command/🧪️tests/🟦️.ts` (matched)
- `e498c5877f1ed9acfc97b466cf406ada3e9844e21c414c547cfd4cf8e6a441be` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts` (matched)
- `c00c399e435cafdd6cb1a2e916fde47e2aa2b0d80647ef9adb1efd9053069078` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/📋️contract/🟦️.ts` (matched)
- `ced0dd512e7c0895c091c153c554e2b32ac5f1a2e97aaf68f8ab71d1b1f3b369` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/🧬️schema/🔣️.json` (matched)
- `cbcacb916c1a7a249b49e1b84c322544881ca681ed6acee41606b9ab67b87954` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/💾️durability/🟦️.ts` (matched)
- `4fc76a2d93d7ce31a5378f6484834843f96608dfc4ee035275ff08bb1332c947` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts` (matched)
- `b50d8efef3f4d4e79459633996b15f066a56dcd7d62e74893f7e376210807874` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🟦️.ts` (matched)
- `5f80c1b9fcadd3df8f76245afa23445a56254bea7a9280487b0efdd7e65e8e79` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json` (matched)
- `7098baac3a190f61ec9c665f12bd8928f69b1ac3ea7a0ed1fe1f8f2f148c0daa` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/📥️persistent/🟦️.ts` (matched)
- `b352958f4f427180f1d985f3f4d96667802e4dee3c3e2685b7a8af1d578421f8` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts` (matched)
- `ad675f6c626de96b6fb0548991ea99cbd6fdb3473fdb472b8cee0f936eecc0b7` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧪️tests/🟦️.ts` (matched)
- `f1a695410760d17358b0d2c3b571ec35c91591f82d069d9f3b105e14851c6c18` — `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` (matched)
- `0b8600e1d9b041c19a095304e9b296a91bcd410624e1c9a3fc6e50a260c441a1` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧬️schema/🔣️.json` (matched)
