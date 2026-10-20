# 🧷️ Handoff: `tools setup bazel` Fails On Mounted Child Allocation

- When: 2026-10-20 15:13–15:20 (started 15:13, 7 min of it queued behind codex pid 22428 on `native-fixed-image:.cache/b/d`).
- Command: `bun repo --tool=bazel tools setup bazel` (log: `🗑️generated/setup-bazel.log`).
- Failure: `Exact original parent child allocation required` at `readNativeStorageMountedChildV1` (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts:203`). The cleanup journal of the parent has either zero or more than one allocation row whose `child.directory` equals the requested child directory.
- Note: the file was modified on disk at 15:11, during the run (owner: peer storage/native-image layer, frozen for us).
- Mounted children at the time: `.cache/b/p/m/persistent/development/m` (disk21s1).
- Impact on the course: none so far, because the pinned Bazel binary already exists under `.cache/b/t/m/bazel/bin/…`. `run --tool=bazel` is tested separately.
