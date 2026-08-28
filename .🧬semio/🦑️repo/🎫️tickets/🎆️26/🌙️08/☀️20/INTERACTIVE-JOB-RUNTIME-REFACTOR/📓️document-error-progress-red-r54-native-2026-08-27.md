# Document Failure Progress — Native Compile RED R54

Exact canonical duplicate-owner selector failed compilation with one E0609: `UiDocumentAssemblyError` did not carry `compared_bytes`. The schema-first fixture requires 16 operand bytes for an exact u64 ID comparison even when that comparison rejects a duplicate. Source oracle passed 55 checks. No native test executed.

```text
[DEBUG] fixed-list-page-oracle checks=55
error[E0609]: no field `compared_bytes` on type `UiDocumentAssemblyError`
error: could not compile `semio-framework-ui-contract` (lib test) due to 1 previous error; 68 warnings emitted
```

Raw: `🧪️member-ui-document-error-progress-red-r54-native-2026-08-27.txt`. The correction keeps compared work alongside actual allocation and initialized metadata on failure; exact rejected input remains owned by the caller.
