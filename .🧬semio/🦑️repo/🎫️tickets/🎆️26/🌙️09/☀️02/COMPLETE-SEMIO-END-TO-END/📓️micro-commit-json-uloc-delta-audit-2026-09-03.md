# Micro-Commit JSON ULOC Delta Audit

## Finding

The `📃uloc` total counted JSON recursively by object keys, while `➕️`, `✏️`, and `➖️` used Git's physical-line `--numstat` output. Pretty-printed JSON could therefore report more added “ULOC” than exists in the complete JSON bucket.

## Resolution

JSON deltas now read the compared Git blobs in bounded batches and compare recursive key paths and value fingerprints. All JSON footer, bundle, per-day, and range metrics now use that key-based delta. Other languages continue to use their physical-line Git deltas. The blob-size reader now preserves request-to-response ordering, including staged `:0:` objects.

## Verification

- A temporary real Git index reported 155 physical JSON lines added for a 152-key result; the corrected delta reported 151 key additions and no edits or removals.
- The current staged metric is `📊️metric🧾️json📃uloc💯️1.95M📈️796k➗️69.2➕️809k✏️2.34k➖️12.8k🟰️824k`.
- `git diff --check` passed.
- The package test target is blocked before test discovery by a pre-existing missing normalization import. The package lint target is blocked by unrelated workspace TypeScript errors; neither reports an error in the changed metric implementation.
