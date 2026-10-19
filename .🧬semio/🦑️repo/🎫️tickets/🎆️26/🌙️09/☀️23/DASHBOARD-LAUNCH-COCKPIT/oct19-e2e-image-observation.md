# Native Image Observation Runtime Failure

The actual dashboard developer-tools owner target failed during repository startup, before its test body. `observeNativeImageVolumeSilenceV1` enumerated `.cache/b/d/m/.fseventsd/000000006ed33f9a`, then its `lstatSync` failed with `ENOENT`. The macOS service can rotate these logs independently of the repository lease.

The schema-first regression describes 64 discovered regular event logs of 1000 bytes, removing one through the existing cancellation checkpoint after discovery. The census must report 63 files and 63000 bytes and agree with independent fast-glob discovery. A second assertion makes the checkpoint throw a cancellation error whose code is `ENOENT`, proving only filesystem inspection races are tolerated.

The correction catches `ENOENT` only around inspection of each already discovered log, and tolerates it only when a no-follow metadata recheck proves the real service directory retains its original device/inode identity. Shared negative vectors remove and replace the entire directory after discovery; both must refuse rather than reporting an empty census. Directory enumeration failures, cancellation callbacks, unsafe markers and existing service-directory policy retain their established behavior. The correction creates no retry loop or dependency. The owner exposes a localized source execution target, `test-native-image-observation`, through its existing `📜️script.ts`.

The regression is queued through `bun repo`; no passing test body is claimed yet. Generated runtime output remains temporary under this ticket.
