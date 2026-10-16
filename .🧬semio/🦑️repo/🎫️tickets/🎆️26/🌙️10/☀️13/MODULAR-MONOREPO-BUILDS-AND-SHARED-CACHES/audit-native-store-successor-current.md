# Native and Cache Store Successor Audit

Read-only source audit, 2026-10-16. No source changes, tests, compiler, native operations, downloads, publication, cache execution, or retirement were performed. Runtime and physical proof remain pending. Earlier test counts are historical report claims and are not independently verified here.

## Temporal Conclusions

- Actual store lines 61–62 now have the required nesting: requested image control encloses open, a distinct requested body control, canonical child settlement, private consumer settlement, and finally image.close. The inner requested wrapper calls canonical settle before resolving; image.close runs while the outer image control is still live. Owners are invalidated after close. This corrects the earlier close-after-control-settlement concern for the actual store path.
- Private consumer frames close births and drain pending descendants on both return and error (settleConsumers); enrollConsumer creates an actual requested child, validates refinement and store ownership, and registers its promise before returning. The store bridge verifies exact invocation, reservation issuance, control identity, and retained frame. These are executable checks, not a conclusion from structural types.
- Actual maintenance similarly nests its requested body beneath the requested image phase; its private pending tasks drain before returning through exclusive maintenance and close. This is source reasoning only.
- Borrower operations synchronously validate actual SH or writer proof, select genuine issued refinements, create requested children, and drain nested private frames. Close rejects self-close, closes top births, waits accepted work, attempts custody verification and release, and reports failures.

## Failed Open and Transfer Boundaries

- Both macOS and fixed openers now special-case failed shared/ready receivers and await the borrower close before final guard release and receiver deletion. This covers failure of the final image.recheck after accepting a real SH owner without returning the image. Close errors are retained with the primary failure. Initial failures before a borrower exists release the acquired shared guard directly.
- macOS preparation acknowledges a concrete receiver while retaining the original canonical EX. Accepted physical cleanup then verifies that receiver, drains owned work, and settles creator responsibility. Only after settlement does completePersistentReceiver release EX and enter the real gap before obtaining SH. Fixed preparation follows the same distinction. Post-ack gap failure is retained unresolved; it does not pretend that pre-ack funded cleanup remains valid after EX release.
- Both platforms compare original receipt, backing, mounted root, and mapping across the guard gap. macOS compares immutable captured facts; fixed compares the previous captured values before replacing its facts object. Receipt reads bind bounded bytes to stable descriptor/named-file identity. Fresh receipt creation uses exclusive creation, fsync, descriptor checks, and directory flush. No reader receipt rewrite is present in the inspected path.
- Fixed command birth requires a private operation in its WeakMap, exact original options, writer proof, accepted cleanup identity and funding, captured ancestry/root/backing, and closed command admission. The backend and command constructors are private. Cleanup commands use the genuine accepted cleanup signal and deadline checks, terminate on cancellation/deadline/output failure, and await child close. This establishes intended source ownership and deadline plumbing only.

## Limits and Residual Risk

- No new actionable source defect was established in the inspected boundaries. In particular, no image close outside its outer requested image phase remains in actual withRepositoryCacheStoreV1 at the captured hash.
- Native receiver/fixed state machines intentionally retain unresolved responsibility when transfer or physical cleanup cannot be proved. Source retention is not evidence of a physical recovery path or runtime safety.
- Source audit cannot establish platform command availability, kernel/mount behavior, queue fairness, cancellation timing, termination completion, identity race resistance against noncooperating actors, or runtime error aggregation behavior. All platform runs remain pending.
- The moving CPU/WASM code and composite typing were not compiled or tested. No test count, typecheck success, or runtime success is asserted.

## Captured SHA-256


- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts
  - 3854269ba9896f7018b449b6f43ea58769965db475db74ac8f214ce2ab6cb9c2

- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts
  - d17d831d310166ba42abc97a9167224f04008d35e8e7ef4d02c4f6d4a2f9a04b

- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts
  - c33422c71cd83fc388488268351308b7714829cdee5a0dd25cef293c3f3422b2

- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🎛️command/🟦️.ts
  - 40557f3636269e5b00d22c4eca7b978b959617d5f61d8fd4edd6c27f7eede0fe

- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts
  - dd712fcd016453334fdd17f169b870d33d19531f70b3a10af3732f88ea51bf50

- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts
  - b8c6793983619f4907c53fff3a9c1dcd4d8dfcbe1166e1876f86cfecbe44deea

- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🐧️image/🟦️.ts
  - 73eeac87fb10da753b10a654fb5c62622900706996ad1ebe49858f6f34d3fa7b

- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪟️image/🟦️.ts
  - 2fdbd5642a5c5fb9e0cbb74469e3436b505b93473d57c19d372ad72eeebd8309

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts
  - 97a964ff0d09424960ba6c74783f2b13e39cfbcdbb8c12b10d33c45f7b188b5c

- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts
  - c761793e52597335f72ca9cab339847eedc10cc9dc9e8ff80b5d0d618af3117e
