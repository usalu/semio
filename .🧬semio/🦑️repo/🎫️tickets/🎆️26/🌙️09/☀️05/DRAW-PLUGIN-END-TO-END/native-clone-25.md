# Independent Native Component Fallback

The managed component pass stalled on resource admission. An APFS clone of the earlier physical `paefc3c` Cargo lane is isolated under `🗑️generated/w25`, with separate build and target directories. The first attempt exposed managed build-script wrappers that embed the original physical lane. `unwrap-cloned-buildscripts-25.py` replaces only wrappers in the clone with their accompanying cloned real executables; it restored 14.

`native-clone25.ts` renders the actual repository profile policy for the Draw component, including all package overrides, and launches the pinned native Cargo. The invocation sets one Cargo job, the repository Cargo home, the cloned directories, the WASI target, and normal target flags. It uses the pinned nightly compiler and does not disable the OS resource warden. Cargo is running in `🗑️generated/native-clone25b.log`; it passed the wrapper failure and began compiling actual dependencies. The available filesystem space was 6.3 GiB at the last check. This is a fallback build, not a managed component receipt; the output requires the genuine emitter and browser materialization before it can support browser acceptance.

The first successful compiling attempt used `package` feature unification; the driver schema specifies `selected`. It was stopped and restarted with `selected`; the current log is `native-clone25c.log`. No output from the earlier attempt is accepted as the component.

The PDF exporter manifest changed while that Cargo graph was running. The process was stopped and restarted on the same target with the latest graph, preserving compiled artifacts; the active log is `native-clone25d.log`.
