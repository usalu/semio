# Doctor storage-images repair: reported success without attaching

## Cause
Repair `storage-images` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔎️probes/💾️storage/🟦️.ts`:
1. `repairImage` refused a non-empty mount point (`readdirSync(mount).length>0` threw). After a reboot, writers had created `browser`, `modules`, `sessions` in the bare `m`, so every detached image with such residue failed. Nothing moved the residue aside, although `quarantineStrandedNativeRootEntriesV1` (the `m-stranded-<stamp>` mechanism of the cleanup shell) existed and was not used here.
2. `repairImages` pushed per-image failures into `notes` and only threw when nothing was repaired. If any other image (e.g. leases) was repaired, the run returned `removed:N` with status done (the failure survived only as a `reason` note), so the CLI printed "repaired storage-images: N item(s)" while the products image stayed detached.
3. The success check was only a device compare of the mount directory, not the kernel inventory.

## Fix
- Strand the content of a bare, unmounted mount point to the sibling `m-stranded-<stamp>` (never delete), named in the repair notes (en+de), before attaching.
- Verify after attaching with the kernel inventory (`attachedAtMount`) and the device change; otherwise throw.
- Any per-image failure now fails the whole repair ("N of M images repaired, not repaired: ..." en+de); the runner reports it as `failed`, and the next doctor run is the real verification.

## Test
`🩺️doctor/🧪️tests/🔎️probes/🟦️.ts`: "a bare mount point that writers filled is stranded to a sibling first, and a failed attach is never reported as repaired" (fake command layer in a temp dir, runs on every platform). The existing darwin hdiutil test still passes.
`repo test-file` on that file: 22 pass, 0 fail.
