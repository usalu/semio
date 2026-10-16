LAUNCHER REACHES CARGO: no (2026-10-16 15:15)

# Infra Unblock 3 Result (2026-10-16)

Status: the publication refusal is a peer's in-progress Native publication change, so I stopped there as instructed. The preview failure was a real stale-call defect in the headroom script. I fixed it with a one-line change, and the preview now gets past it. Both routes now end at the same single blocker: the live image receipt `image.json` is an old receipt without the new required `mountDirectory` claim. No gate was bypassed, no cache material was deleted or moved, and no retire was run.

Logs: `🪓️monolith-split/🗑️generated/iu3-probe-1.log` (launcher repro), `iu3-preview-1.log` (preview after fix). Tools: `iu3-tools/validate-receipt.ts` (read-only schema check), `iu3-tools/headroom-script.pre-edit.ts` (pre-edit snapshot of the headroom script).

## 1. `Exact canonical persistent Native publication required`

Reproduced under the mutex with `SEMIO_SCRIPT_POLICY=…3600000 bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-hash --lib --manifest-path 🧰️framework/Cargo.toml --message-format=short` (`iu3-probe-1.log`, exit 1). The launcher dies before cargo starts, in the Native storage cleanup command:

```
error: Exact canonical persistent Native publication required
      at inspectPersistentReceiver (🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:111:1190)
```

### Cause (measured, not assumed)

- `inspectPersistentReceiver` validates the live `⚡️cache/rust/fixed/image.json` with `validateJsonSchemaSubset(imageSchema.definitions.ImageReceipt, authored, imageSchema, [mountDirectorySchema])`. A non-empty error list throws this message.
- The `ImageReceipt` schema (`💾️storage/🧬️schema/🔣️.json`, modified today 15:08) now lists `mountDirectory` in `required`. The property refs the new mount contract (`🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json`, created 15:02). A mount directory record is `{dev:string, ino:string, mode:integer}`.
- The live receipt (`⚡️cache/rust/fixed/image.json`, 129 bytes, written 2026-10-14 14:16) is `{"version":1,"filesystemBytes":17178820608,"maximumPhysicalBytes":17178820608,"dev":16777232,"ino":335960199,"size":17178820608}`. It has no `mountDirectory`.
- Independent check: `iu3-tools/validate-receipt.ts` runs the same validator on the live receipt and prints `["/mountDirectory is required"]`.
- Other receipt facts match, so this is the only mismatch. `fixed.dmg` dev, ino and size equal the receipt (`16777232`, `335960199`, `17178820608`) and nlink is 1. Physical bytes are 17,044,619,264, below the 17,178,820,608 bound.

### Why I stopped (peer's in-progress change, not stale data from a dead writer)

- The peer checkpoint `MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/whole-cache-retirement-execution-current.md` (line 11, edited 15:13) states the contract: "Native creators capture the underlying mount directory before activation. Required receipt DATA records decimal device/inode strings and finite mode … Old receipts lacking the claim refuse. An empty unmounted path is never adopted."
- It is live, uncommitted work (`git status`, `MM`/`AM`/`A`):
  - `💾️storage/🧹️cleanup/🎛️command/🟦️.ts` (`MM`, edited 15:10)
  - `💾️storage/🧱️fixed/🟦️.ts` (`MM`, edited 15:10)
  - `💾️storage/🍎️image/🟦️.ts` (`MM`, edited 15:03)
  - `💾️storage/🧬️schema/🔣️.json` (`MM`, +156 lines in the working tree)
  - the new `🌐️platform/📋️contract/📂️mount/**`, `🧾️metadata/**`, `🧱️fixed/📦️vhd/**` and `🧹️cleanup/♻️retirement/**` trees
- Owner/creator code (`🧱️fixed/🟦️.ts` lines ~126, 142–150) now captures the mount claim only when it creates the image (`observeNativeImageMountDirectoryV1(root)` of the unmounted underlying directory). Existing receipts are only read back.
- Consequence for the next step: the claim is the identity of the empty directory UNDER the mount (`m`), which is currently hidden by the mounted image (`/dev/disk4s2` on `…/rust/fixed/m`). It cannot be observed without a detach, and the contract says an empty unmounted path is never adopted. Republishing `image.json` is therefore a physical creator or retire/recreate operation, not a file edit.
- Decision for the coordinator or the peer: how the existing 16 GiB image gets a conforming receipt. I did not touch `image.json`, `fixed.dmg`, the mount, or any schema or validator, and I did not loosen the required list.

No fix was attempted: loosening `required` or skipping the check would bypass the publication gate, and a hand-written `mountDirectory` would be a guess at a physical identity.

## 2. `Live Original resource control issuance required` in `cargo-retirement-historical-preview`

### Cause (a real defect in the headroom script)

- `withNxToolingUseV1` gained a required `phaseControl` parameter: now `(storage, authority, phaseControl, run:(witness, actualPhase)=>…)`. Source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🟦️.ts:25`.
- The headroom script (`MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/headroom/📜️script.ts:223`, last changed 00:23) still used the old 3-argument form. It passed its `async()=>{…}` callback in the `phaseControl` slot.
- `requireIssuedResourceControlInvocationV1(control, invocation)` then called `requireIssuedResourceControlV1(<function>)`. `controls.get(<function>)` is undefined, so it threw `Live Original resource control issuance required`. The coordinator's trace (`coord-retire-preview.log`: `withNxToolingUseV1 … 📥️use/🟦️.ts:26` called from `📜️script.ts:223:140`) matches.
- The authority check itself is intact and correct. The script was calling the API outside the issued-control context it needs, because the launcher contract changed under it.
- The current contract is shown by every other caller. For example `🚀️bootstrap/🛠️tools/📥️use/🟦️.ts:52` (`withRepositoryToolingStoreV1(root, original, createIssuedResourceControlV1(original,"cache:entry",0), (_store, actualPhase) => withNxToolingUseV1(storage, authority, actualPhase, …))`). The script already passed the same issued control to `withRepositoryToolingStoreV1`.

### Fix

One call-site change in `📜️script.ts:223`. The `withRepositoryToolingStoreV1` callback now takes `(_store, actualPhase)`, and `actualPhase` is passed as the third argument of `withNxToolingUseV1`. The control stays the one issued by the store, so the issuance, signal and invocation checks are not weakened:

```
async()=>{bindSdk();return withNxToolingUseV1(storage,{…},async()=>{
→ async(_store,actualPhase)=>{bindSdk();return withNxToolingUseV1(storage,{…},actualPhase,async()=>{
```

- Test-first: there is no unit-test harness for this script. RED was the failing preview in `coord-retire-preview.log`. GREEN is `iu3-preview-1.log` (below).
- No `[DEBUG]` lines added. A pre-existing `[DEBUG] Actual immutable SDK activated` log at `📜️script.ts:49` (the `sdk-provision` path) belongs to the peer, and I left it.

### Preview result after the fix (`iu3-preview-1.log`, exit 1)

I ran only `cargo-retirement-historical-preview prototype` (under the mutex). I did not run any `…-retire`.

- `Live Original resource control issuance required` is gone. It got through `withNxToolingUseV1`, the SDK binding and the store opening.
- It now stops at the same receipt refusal as section 1: `error: Exact canonical persistent Native publication required` at `inspectPersistentReceiver (…/🎛️command/🟦️.ts:111:1190)`.
- So the preview cannot print retirement candidates until `image.json` conforms.

## 3. Files changed

```
 .../☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/headroom/📜️script.ts | 2 +-
 1 file changed, 1 insertion(+), 1 deletion(-)
```

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/headroom/📜️script.ts` (peer-owned, one line)
- New (mine): `iu3-tools/validate-receipt.ts`, `iu3-tools/headroom-script.pre-edit.ts`, this file, the two logs.
- No other source, schema, launch.json or cache file was changed.

## 4. Commands run and real results

| Command | Result |
| --- | --- |
| `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-hash --lib --manifest-path 🧰️framework/Cargo.toml --message-format=short` (with finite `SEMIO_SCRIPT_POLICY`, mutex held) | exit 1, `Exact canonical persistent Native publication required` before cargo starts (`iu3-probe-1.log`) |
| `bun iu3-tools/validate-receipt.ts` (from `🧰️framework`, read-only) | `["/mountDirectory is required"]` |
| `bun …/headroom/📜️script.ts cargo-retirement-historical-preview prototype` (mutex held) | exit 1, passes the control issuance check, stops at `Exact canonical persistent Native publication required` (`iu3-preview-1.log`) |

The mutex `🗑️generated/cargo-mutex.lock` was released after each call. It was free when I started.

## 5. Next blocker

1. Receipt: `⚡️cache/rust/fixed/image.json` lacks the new required `mountDirectory` claim, so every Rust launcher call and the retirement preview refuse before they do any work. The owner of the Native storage contract change (the peer ticket `MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES`, files listed in section 1) or Root must decide how the existing image is republished. The claim describes the empty directory under the mount and cannot be read while the image is mounted. Options: (a) let the peer finish and land the creator and retire path that produces a conforming receipt, then run the sanctioned retire (Root authorizes the retire); (b) have the peer provide a sanctioned receipt-republish operation. A hand edit of `image.json` is not recommended, because the value would be guessed.
2. Headroom: still unchanged and measured at 15:15. `⚡️cache/rust/fixed/m` has 4,262,842,368 B available against 4,294,967,296 B required, so it is short by 32,124,928 B. Once the receipt is conforming, the headroom gate will be the next refusal. The preview exists to plan the sanctioned retirement that closes this gap.
3. Until both are resolved, Rust slices keep verifying statically.
