# Composition Baseline Correction R184

R180 supplied the actual RED: full734/735, sole stale composition final baseline. The released language-neutral composition fixture explicitly retains its shared controller through per-pool release and disposeAll. The parent authorized this narrow authored assertion correction only.

R184 actual focused `OwnedResidentComposition`: **1 PASS /734 skipped /735**, one selected file of five,5.38s,start05:54:36,Nx0. Selected107 before/after hashes equal. Exact command/log/hash manifest: `🧪️renderer-composition-r184-2026-08-28.{txt,json}`.

```sh
bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedResidentComposition'
```

## Exact Authored Diff

Only UiDocumentStore's final per-pool baseline line changed. No runtime/schema/price/limit change. It uses the imported current `composition.poolPreparation.controllerTotal`, not hardcoded784 or zero. The existing private-pool admission, hostile constructor, witness, rejection and release-grant assertions remain unchanged.

```diff
-      expect(native.client.ownsUiResidentPool(pool)).toBe(false); expect(native.residentLedger.usage.data).toEqual(before); native.client.disposeAll();
+      const retained = produce(before, draft => { draft.bytes += composition.poolPreparation.controllerTotal.bytes; draft.slots += composition.poolPreparation.controllerTotal.slots; draft.owners += composition.poolPreparation.controllerTotal.owners; });
+      expect(native.client.ownsUiResidentPool(pool)).toBe(false); expect(native.residentLedger.usage.data).toEqual(retained); native.client.disposeAll(); expect(native.residentLedger.usage.data).toEqual(retained);
```

Removed152 UTF8 bytes, added468, net+316; all text in this hunk is ASCII. Resulting UiDocumentStore594573 bytes, SHA256 `100dae341b4112c50683105637c043a14c0149158366fc61bd1320343e34f435`. Pre-change selected source was `a407815789afbc945a7f5f9615893486888710ed8ad89dfd058f4c517f6dc7ab`.

An additional read-only exact-hunk inversion reconstructed the original source hash a407815789afbc945a7f5f9615893486888710ed8ad89dfd058f4c517f6dc7ab and original594257-byte extent. This verifies the reported +316-byte change without a git rollback or any file rewrite.

R185 full renderer and R186 strict were then dispatched against a fresh selected107 prehash. R186 is terminal with the same seven known tutorial diagnostics and all107 hashes stable. R185 is still running at this report checkpoint. No full735 green is claimed here. The retained controller is intentionally not a whole-Shard retirement witness.
