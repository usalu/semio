# S1: SDK identity

Cause: `sdkIdentityV1` returned the basename of the realpath of SDKROOT. Xcode's `MacOSX27.2.sdk` is a symlink to the real directory `MacOSX.sdk`, so the coordinator got `MacOSX.sdk` while CLT workers (real dir `MacOSX27.2.sdk`) advertised `MacOSX27.2.sdk`.

Fix (one function, `LIB/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts` `sdkIdentityV1`): identity is the `CanonicalName` of `SDKSettings.json` (`macosx27.2`), else the version parsed from the directory name (realpath, then given name; `MacOSX15.4.sdk` gives `macosx15.4`), else the basename. Used by the unit key (`machineValueV1`, SDKROOT), the worker property (`offloadSdkIdentityV1` in `offload/🧰️toolchain/🟦️.ts`, called by `detectWorkerRustcV1`) and the requester's required property (`offload/🟦️.ts:118`): all go through it, no second implementation.

Live check: Xcode `MacOSX.sdk`, Xcode `MacOSX27.2.sdk` link, CLT `MacOSX27.2.sdk`, CLT `MacOSX.sdk` all give `macosx27.2`; CLT `MacOSX26.5.sdk` gives `macosx26.5`.

Tests: new portable test (symlinked unversioned dir, CLT layout, other version: same identity / same key environment vs different); updated expectations. portable 36/0, offload 40/0, cross-root 2/0.

Effects: linked-unit keys re-key once (SDK component changed). Workers advertise `semio-sdk=macosx27.2` after their next natural restart; until then a requester with the new value is declined to local (safe).
Open: build number not part of identity (same version, different build would collide).
