# Bazel Distribution Primary Verification

Actual read-only HTTPS probe93296 completed0 at 2026-10-16T14:46:41.767008+00:00. All six pinned8.4.2 platform/architecture artifacts returned HTTP200 to HEAD with exactly the authored byte length; their official SHA256 text returned HTTP200 and exactly the authored digest. No binary was downloaded, installed or executed. This validates the current pin metadata only, not host provisioning/runtime/privilege behavior. The web rendering tool could not open these endpoints; normal certificate-verified Python HTTPS succeeded without disabling TLS verification or retrying. Each checksum body was bounded4096bytes and request timeout15seconds. Complete machine receipt remains `🗑️generated/bazel-distribution-primary-probe.json`.

| Host | Bytes | Official checksum |
| --- | ---: | --- |
| darwin/arm64 | 58602460 | [SHA256](https://releases.bazel.build/8.4.2/release/bazel-8.4.2-darwin-arm64.sha256) |
| darwin/x64 | 60269974 | [SHA256](https://releases.bazel.build/8.4.2/release/bazel-8.4.2-darwin-x86_64.sha256) |
| linux/arm64 | 62234197 | [SHA256](https://releases.bazel.build/8.4.2/release/bazel-8.4.2-linux-arm64.sha256) |
| linux/x64 | 64058427 | [SHA256](https://releases.bazel.build/8.4.2/release/bazel-8.4.2-linux-x86_64.sha256) |
| win32/arm64 | 232269910 | [SHA256](https://releases.bazel.build/8.4.2/release/bazel-8.4.2-windows-arm64.exe.sha256) |
| win32/x64 | 57130821 | [SHA256](https://releases.bazel.build/8.4.2/release/bazel-8.4.2-windows-x86_64.exe.sha256) |

All six pass; no distribution/source edits were needed. Other-host execution remains pending.
