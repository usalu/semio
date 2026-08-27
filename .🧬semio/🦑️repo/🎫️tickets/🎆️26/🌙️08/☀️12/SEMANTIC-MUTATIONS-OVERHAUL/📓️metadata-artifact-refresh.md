# Metadata Artifact Refresh

The previously retained `🧪️derive-contract-target` and `🧪️pdf14-contract-target` no longer exist. Root confirmed their absence; no removal cause is established and no old binary is reused as current acceptance.

A fresh scoped Bun/Nx Cargo build compiled the genuine kernel and schema libraries, with one build job, into `🧪️metadata-refresh-target`. It completed in 3m31s with exit 0 and 33 kernel warnings. Transcript: `🧪️metadata-artifacts-refresh.log`. The async test macro is being built separately into that same target before the current-source runtime harnesses run.

Fresh library pairs under `debug/deps` are kernel `libsemio_framework_os_kernel.{rlib,rmeta}`, schema `libsemio_framework_schema-529c308cc4e44832.{rlib,rmeta}`, lower protocol `libprotocol-6c1330d456d23eb4.{rlib,rmeta}`, serde `libserde-73de109b1e55818a.{rlib,rmeta}`, serde_json `libserde_json-9a8518baf918989a.{rlib,rmeta}` and serde_core `libserde_core-b3bb434f385c2fed.{rlib,rmeta}`. The macro library is `libdsl_derive-f44e247812382dc4.dylib`. The serde and serde_json fingerprint records both depend on serde_core identity `281446630473711399`; the other serde_json/serde_core artifacts are not interchangeable with that pair.

Standalone clients must supply both artifact formats for each ordinary library and retain source/artifact fingerprints before and after execution. This build is artifact preparation, not TXT or glTF runtime acceptance. No STDIO registered gate is claimed at this checkpoint.

The async macro build completed separately in 8.32s, exit 0: `🧪️metadata-async-macro-refresh.log`. Its fresh artifact is `libsemio_framework_async_macros-80123402fdf704c6.dylib` in the same dependency directory. The previous deleted target's macro filename is not reused.
