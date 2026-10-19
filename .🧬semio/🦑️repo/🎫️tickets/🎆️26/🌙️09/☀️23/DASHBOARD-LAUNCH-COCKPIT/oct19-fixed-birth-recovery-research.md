# Fixed Image Failed-Birth Recovery Research

Read-only review requested by the parent after the native dashboard build left `.cache/b/l/fixed.vhd` without `image.json`. No storage path was modified and no alternate deletion tool/API was called. The parent's reported automatic approval rejection of exact removal remains binding.

## Finding

The existing fixed-image APIs do not provide restart recovery of an initial host directory containing a backing file without its creator receipt. The normal receiver and ordinary retirement paths both require that receipt before they can classify or retire the directory. A newly invoked API cannot reconstruct the original in-memory creator proof from detached file facts.

## Source Evidence

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:120–121`: openNativeFixedPlatformV1 opens the receiver, takes its shared probe and calls fixedPersistentState before entering a preparation.
- Same file, line 187: fixedPersistentState returns absent only when the whole directory is absent. For an existing directory it immediately reads `image.json` through readNativeImageRetirementFileV1; a missing receipt therefore throws rather than producing a fresh/inactive state.
- Same file, lines 148–157: both retireNativeFixedPlatformV1 and retireNativeFixedPlatformWithWriterV1 read and validate the exact creator receipt before observing/detaching/removing. Neither is a receipt-free recovery path.
- Same file, lines 164 and 191: preparation custody is kept in a WeakMap plus unresolvedFixedPreparations. Failed fresh creation can retire only inside the original attempt using original directory/root/backing identities and the accepted cleanup operation. Unresolved creator state is deliberately retained. These capabilities cannot be recovered in a later process from path/size alone.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts:163–166`: settleEnteredNativeStorageFundedChildV1 requires an actual original enrollment/request/source and unforgeable NativeStorageFailedBirthRetirementV1. It addresses entered funded children, not an unattested initial host image after process exit.
- `🧰️framework/🔨️modules/🏃️process/💾️state/📥️recovery/🟦️.ts:9` and `💾️state/🟦️.ts:32–34`: persistent-state charge-before-intent recovery handles exact charged absent targets and refuses incomplete/unbound backing. It does not adopt the initial host image.

## Consequence

The parent's observed detached image, empty mount directory, exact file size and known creation time are useful diagnostics but do not satisfy the original creator capability required by the code. Existing APIs cannot safely complete this recovery without either the still-live original preparation scope or a new explicit schema-first failed-birth recovery mechanism backed by durable creator identity. Forging `image.json`, altering the classifier to treat an existing orphan as absent, disabling enforcement, or calling a different deletion API would bypass the current guarantees and was not attempted.

Native runtime verification remains blocked by the existing orphan and the parent's rejected removal. Source/gate tests can still run independently.
