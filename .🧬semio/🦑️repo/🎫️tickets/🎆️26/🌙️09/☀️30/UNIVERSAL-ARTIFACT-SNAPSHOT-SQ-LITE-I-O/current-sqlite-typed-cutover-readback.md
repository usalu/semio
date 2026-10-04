# Current SQLite Typed Cutover Readback

Read-only audit of the authentic TIFF19 log and current shared source. No Cargo run or production edit performed. The log failed compilation before assertions: **84 errors**, E0277: 81, E0308: 1, E0631: 2.

## Exact Error Groups

### 11 errors: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🦀️.rs`

Locations (repeated line numbers denote distinct diagnostics): E0277@2416, E0277@2432, E0277@2191, E0277@2196, E0277@2198, E0277@2205, E0277@2208, E0277@2210, E0277@2214, E0631@2441, E0631@2452.

### 42 errors: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`

Locations (repeated line numbers denote distinct diagnostics): E0277@26, E0277@27, E0277@27, E0277@29, E0277@29, E0277@31, E0277@31, E0277@32, E0277@32, E0277@33, E0277@33, E0277@35, E0277@35, E0277@36, E0277@36, E0308@37, E0277@40, E0277@41, E0277@41, E0277@41, E0277@44, E0277@44, E0277@45, E0277@45, E0277@46, E0277@46, E0277@46, E0277@46, E0277@53, E0277@53, E0277@53, E0277@53, E0277@60, E0277@14, E0277@15, E0277@15, E0277@16, E0277@16, E0277@17, E0277@17, E0277@19, E0277@19.

### 18 errors: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🪶️sqlite/🦀️.rs`

Locations (repeated line numbers denote distinct diagnostics): E0277@6, E0277@11, E0277@14, E0277@14, E0277@18, E0277@18, E0277@18, E0277@18, E0277@22, E0277@22, E0277@23, E0277@23, E0277@29, E0277@29, E0277@31, E0277@31, E0277@31, E0277@31.

### 2 errors: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🚦️native/🦀️.rs`

Locations (repeated line numbers denote distinct diagnostics): E0277@44, E0277@48.

### 1 errors: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs`

Locations (repeated line numbers denote distinct diagnostics): E0277@21.

### 1 errors: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛫️native-encoding/🦀️.rs`

Locations (repeated line numbers denote distinct diagnostics): E0277@19.

### 9 errors: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`

Locations (repeated line numbers denote distinct diagnostics): E0277@10896, E0277@10901, E0277@10905, E0277@10911, E0277@10922, E0277@10924, E0277@10926, E0277@10934, E0277@10936.

## Current Canonical Signatures

`🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs`: `checkpoint`, `check_rows`, `check_value_bytes`, `check_database`, `admit_allocation_bytes`, and `admit_reconstruction_bytes` return `Result<(), ValueError>`; `reconstruction_remaining_bytes` returns `Result<usize, ValueError>`; `allocation_stage<T,E>` returns `Result<Result<T,E>, ValueError>` and settles the child allocation ledger even on child refusal. `allocation_remaining_bytes` returns `usize`. Physical export/import and relational artifact producers return `ValueError`, including Projection/Reconstruction/NativeEncodingBound and text/blob/owned-vector helpers.

`ArtifactSqliteSnapshot` at store/🦀️.rs:10884 still defines relational projection/reconstruction, native encode/decode, and encoding preflight as `Result<..., String>`. Subset validation and erased codec import/export use `IoResult`. Generic record native encode/decode helpers and space-history native dispatch also currently return `String`. These are actual terminal contracts, not typed producer contracts.

## Minimal Prerequisite Consumer Repair

Keep SQLite and physical producer errors typed. At an existing declared `String` terminal only, explicitly map `ValueError::into_message` (or equivalent local closure); replace `map_err(String::from)` and untyped `?` at these terminals. For a direct tail returning `Result<_,ValueError>` into `Result<_,String>`, map the tail error explicitly. Do not add `From<ValueError> for String`.

At `IoError` terminals, inspect refusal kind before converting its message; preserve the kind in a structured diagnostic/code where the current IoError schema permits. Its current fields are only `message` and `diagnostics`, so a naked `IoError::from(error.to_string())` loses typed classification. A local diagnostic-bearing conversion is cleaner than global implicit conversion. Canceled/OwnershipLimit/WorkLimit/AllocationFailed/InvalidValue/InvariantViolated must retain distinct classification up to that boundary.

Retain checkpoint positions, one shared control, bounded fallible allocations, explicit native owner construction, cumulative allocation/reconstruction charging, and retirement guards. No test disabling, no fresh limits/control resetting, and no unbounded clone as a compile workaround.

Root scope: store capability defaults/erased terminals (9), space-history relational (42), space relational (18), space-history native dispatcher checkpoints (2). I/O scope: I/O metadata/public/hop boundaries (11) and generic native encoding/decoding entry checkpoints (1 each). Total 84.

## Additional HTML and TIFF Consumers Hidden Behind Kernel Failure

These paths still call typed SQLite control from String-returning functions and need explicit boundary mapping after the kernel compiles. They are not counted in the 84 kernel errors:

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: lines 27, 34, 36, 42, 73, 77, 82, 87, 92, 102, 105, 119, 120, 125, 133.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🛫️encoding/🦀️.rs`: lines 65.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🛬️decoding/🦀️.rs`: lines 85.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: lines 12, 31, 32, 57, 82, 85, 92.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🪶️sqlite/🧮️construction/🦀️.rs`: lines 11, 12, 13, 20, 25, 28, 30.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🚦️cohort/🦀️.rs`: lines 93.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🦀️.rs`: lines 181, 183, 188, 189, 190.

HTML relational tick/measure/projection/reconstruction and TIFF relational tick/semantic-budget/construction index walkers remain String terminals. TIFF document native decoding line 85 and encoding line 65 also have direct typed checkpoints; baseline conformance returns a String error while calling typed control. Subset IoResult methods additionally need the refusal-preserving IoError conversion. NativeDecodeControl/NativeEncodeControl zero-argument checkpoints are separate already-typed APIs, not SQLite String boundary sites.
