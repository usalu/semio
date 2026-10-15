# Current Fixed Backing and Retained Owner Census

This is a read-only macOS observation on2026-10-15. It does not acquire deletion authority. System hdiutil mapping, exact backing lstat identity/size and mounted statvfs/device agree for all seven authored partitions. Every backing remains inside its closed envelope; each has one regular backing link and exactly the authored mount on a different filesystem device. Inode/size comparisons stayed equal. Existing scopes/mounts/processes were preserved.

|Owner|Declared Envelope Bytes|Backing Logical Bytes|Backing Allocated Bytes|Filesystem Bytes|Current Available Bytes|Exact Mapping|
|---|---:|---:|---:|---:|---:|---|
|rust|17179869184|17178820608|11970822144|16834846720|5745811456|True|
|tools|8589934592|8588886016|8588886016|8244912128|1576960000|True|
|graph|4294967296|4293918720|4293918720|3949944832|2664189952|True|
|dependencies|8589934592|8588886016|8588886016|8244912128|5762600960|True|
|products|25769803776|25768755200|25768755200|25424781312|24854970368|True|
|diagnostics|2147483648|2146435072|2146435072|2012176384|1922998272|True|
|leases|67108864|66060288|66060288|66019328|14970880|True|

Declared partition envelopes sum66,639,101,952 bytes. Fixed backing files sum66,631,761,920 logical bytes and61,423,763,456 allocated bytes. Mounted descendants are excluded, avoiding counting each backing twice. Current control file allocations are16,384 bytes versus its1,048,576-byte allowance. Control is individually bounded issued host writers, not an enclosing quota filesystem; host directory/APFS/snapshot metadata is not inferred from the file sum. The managed maximum stays68,719,476,736 bytes, including declared control and2,079,326,208 bytes policy headroom. These observations are not a promise that currently free blocks stay free.

The SDK reduction now shares40 immutable binary members/81,008,715 logical bytes between their real object and snapshot/generation names. Physical identity checks and actual consumer proofs are recorded in [the material report](nx-sdk-material-sharing-current.md). Existing old snapshots/generations are retained unless exact Original EX allows their own retirement.

## Retained Ordinary Owners

Read-only system du -sk completed with these allocated-byte observations. They are not deletion authority or a logical file-byte census. The first five roots are disjoint; whole Hub/extensions rows include authored material and must not be treated as generated-only. One mistyped optional Source root was absent and contributed no value.

|Observed Root|Allocated Bytes|
|---|---:|
|.git|15240855552|
|.cache/held-javascript|4674502656|
|ordinary cache/tools|26155294720|
|.nx|23508721664|
|Framework generated/process/cargo-build|838438912|
|Hub whole tree|1571418112|
|extensions whole tree|4435652608|
|ticket generated observation|119332864|

No prior whole-tree Original reader/writer catalog authenticates these retained roots. New fixed-image leases cannot retrospectively prove their readers absent. Safe retirement needs routing each actual reader/writer through its original protocol or an enclosing admitted filesystem, then exact EX retirement. No unknown data was deleted.

## Remaining Ownership Frontier

The old ordinary SDK/cache trees and held JavaScript roots have no retrospective whole-owner custody merely because new launchers use fixed images. Source/Git/coeditor writes remain a separate class. Raw Cargo fetch/update/lock/metadata can write a package registry before a compiler guard. Generic supported Cargo now retains real private Rustup/tools/Rust state, but arbitrary custom compiler/wrapper programs remain outside a complete OS write sandbox. Agent shell redirects and other direct producer scripts are also outside that admitted envelope. No unknown tree is deleted.

The compiler environment gap observed in this census was: repositoryCompilerLaunchEnvironmentV1 spreads caller RUSTFLAGS/RUSTDOCFLAGS/encoded flags; bounded dependency admission also preserves that environment. The new compiler-forwarding argument guard only receives argv. Thus known compiler output/dump selectors supplied through environment do not traverse the tail guard. This is a Source audit finding, not an observed external write or a claimed successful escape. The compiler launch and Cargo-input boundaries now qualify this receiver in both modes; see [actual environment admission](cargo-compiler-environment-current.md). A remaining separate receiver is effective compiler flags from admitted Cargo configuration: authenticated Source identity does not itself route compiler output selectors. This still requires selected/compiler and generic configuration admission; no configuration-based external write has been observed.

The generated64GiB ceiling therefore remains an admitted-writer ceiling. A perpetual maximum for every checkout byte additionally needs authentic Source/Git writer custody or an enclosing finite filesystem, plus original retirement of all retained unbounded owners. An observed baseline plus64GiB is not that proof.
