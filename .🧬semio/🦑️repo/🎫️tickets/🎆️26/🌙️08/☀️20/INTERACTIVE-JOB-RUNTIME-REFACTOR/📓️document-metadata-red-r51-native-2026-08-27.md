# Document Metadata Initialization — Native RED R51

Exact canonical UI selector `retained_document_assembly_reports_metadata_initialization` executed: 0 passed, 1 failed, 146 skipped, 0.041s nextest; exit 1. The first assembly implementation preserved physical allocation but failed to report initialized metadata bytes. The test closes its exact document before asserting.

```text
[DEBUG] document-metadata allocated=7568 initialized=0 expected-initialized=1152
Summary [0.041s] 1 test run: 0 passed, 1 failed, 146 skipped
```

Raw output: `🧪️member-ui-document-metadata-red-r51-native-2026-08-27.txt`. The correction separates initialized metadata from reserved, uninitialized record capacity and retains actual allocation/error ownership. No resident grant or semantic work grant is increased.
