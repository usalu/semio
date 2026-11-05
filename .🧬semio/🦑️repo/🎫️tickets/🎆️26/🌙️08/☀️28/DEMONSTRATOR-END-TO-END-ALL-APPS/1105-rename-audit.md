# Current OS and Plugin Namespace Audit

Two actual compiler captures observed dependency-name edits during their lifetime: fonts at 23:01:29Z and OS/kernel references at 23:50:02Z. Both current renamed declarations are preserved. No compatibility alias, reverse rename, cap change or Source-guard bypass was introduced.

Current Flow editor dependency keys map to semio_plugin, semio_os_kernel and semio_os_infinite_directed_presentation. Its named aliases flow, infinite_canvas and dsl_derive remain those exact aliases despite their package names now being semio-os-flow, semio-os-infinite and semio-os-kernel-dsl-derive. Current P3 editor keys map to semio_plugin, semio_plugin_retained_command and semio_os_kernel. The existing semio-framework-artifact-flow-flow dependency remains unchanged, so a blind removal of framework prefixes would be wrong.

A read-only scan of 34 Rust files under the runtime-owned publication/session/preview/host-laws, declaration emission and SemioFlow IO child scopes found zero obsolete semio_framework_os or semio_framework_plugin namespace tokens. The concurrent rename already updated these files; no source edit was needed. Other lane owners audit their newly mounted source against the actual keys before readiness.

The scope observer's former trailing literal backslash-n was a research-output bug. Its writer now appends String.fromCharCode(10), and an independent Python json.loads successfully reads all 559 entries. The actual @iarna/toml observer rerun still finds zero unresolved dependencies and zero Flow host/session/neural-engine packages in the conservative SemioFlow IO dependency closure. This output correction changes no production behavior or dependency.
