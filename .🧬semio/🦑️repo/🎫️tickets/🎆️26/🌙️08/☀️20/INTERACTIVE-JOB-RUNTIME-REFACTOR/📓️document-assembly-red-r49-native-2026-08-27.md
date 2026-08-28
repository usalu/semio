# Canonical Document Assembly — Native Compile RED R49

Canonical UI contract route: `bun x nx run @semio-tech/ui-contract-rs:test --skip-nx-cache --args='--lib retained_document_assembly_ -- --nocapture'`.

Actual exit 1 before tests: 18 missing-API errors for `UiDocumentAssembly`, its exact error kind, and nonblocking lease read/alias/close methods. All three authored native laws are unexecuted. Independent Node Buffer/Ajv oracle passed 54 checks, including exact ID byte order, Unicode surface bytes, and four hostile ownership projections.

```text
[DEBUG] fixed-list-page-oracle checks=54
error: could not compile `semio-framework-ui-contract` (lib test) due to 18 previous errors; 66 warnings emitted
NX Running target test for project @semio-tech/ui-contract-rs failed
```

Raw output: `🧪️member-ui-document-assembly-red-r49-native-2026-08-27.txt`. The implementation will reuse the existing document arena and its shared paged node storage, not create another document or retained-record representation. Active runtime R30/R31 remain semantic RED until the authority cutover is integrated.
