# Native Portable Compiler Research

The user's latest instruction requires native setups and forbids WSL, IDEs and similar alternatives. WSL preparation stopped without installing or enabling it. The architecture course must still pass its real native runtime and canonical gates.

The [original portable compiler implementation](https://gist.github.com/mmozeiko/7f3162ec2988e81e56d5c4e22cde9977) provides primary implementation evidence for acquiring genuine Microsoft compiler, CRT and SDK files into a portable directory. It uses Microsoft package manifests and payload digests. Its mutable defaults, archive handling, administrative MSI extraction and generated compatibility stubs make direct execution unsuitable for this task. It is a reference, not a trusted installer or proof of readiness.

The execution owner retained an immutable reference revision and digest, then independently authenticated a Microsoft catalog using a Bazel source pin. Four compiler/CRT archives and eight SDK MSI candidates passed payload hashes. The first-party native Go reader queried Windows Installer databases read-only, without running MSI actions or changing script policy. Its directory graph covers 4,453 files with maximum relative path length 119 UTF-16 units. Cabinet closure verification and bounded native extraction are ongoing.

A working provider still needs safe complete materialization, managed tool-store integration, canonical environment/discovery, real compiler/linker smoke tests, and the course gates. No working compiler is claimed from downloads or metadata. Current evidence and provenance are in [the provider ledger](nov05-native-provider.md).
