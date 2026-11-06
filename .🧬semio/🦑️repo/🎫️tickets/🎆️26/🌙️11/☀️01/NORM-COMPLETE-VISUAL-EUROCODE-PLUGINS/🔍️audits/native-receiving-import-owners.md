# Native receiving import owners

Source review of the declared UI receiving roots, during concurrent edits. NativeReceive, NativeReceiver and the owned receiving tree belong to the canonical native receiving owner, while NativeDecodeControl and NativeDecodeConstructionRecipient remain in the lower native-owned decoder.

Remaining observed references at review time:

- UI locale label receiving Rust source line 3 imports NativeReceive and owned mapping/tree types from the lower native-owned decoder.
- UI locale axis-value test Rust source line 28 mixes those receiving imports with the legitimate lower decoder control and recipient imports.
- UI viewport 2d schema Rust source lines 54–58 qualifies NativeDefaultValue, NativeDefault, NativeDefaultValueCursor and tree fixed/receipt through the lower decoder.

Other UI contract receiving files changed concurrently and no longer matched the old lower-owner pattern at the final scoped read. No build or runtime validation was performed.

## Correction

The locale and viewport references above are valid: `semio_framework_value_native_owned::native_decoding` reexports the receiving API. The actual text failure used the different lower crate `semio_framework_value_native_decoding`. Disregard the alleged mismatches above.
