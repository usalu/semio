# Shared IFC Binary Primitive

The IFC4x3 fleet found `TYPE IfcBinary = BINARY` in official IFC4X3_ADD2. The shared IFC graph requires an additional scalar tag `binary` (Rust `Binary(String)`), represented as uppercase STEP literal content without double quotes. The first character is unused bit count 0–3, followed by hexadecimal data; the empty value is `0`. Padding is on the left and the first nibble must be less than `2^(4-unused)`.

The IFC4x3 coordinator owns the shared TypeScript snapshot variant and new TypeScript central diff validation. IFC4x3 exchange worker owns STEP TypeScript parsing/writing and independent binary fixtures. Catalog worker owns declaration validation and new transport schema. A subsequent worker will own shared Rust Part21/IFC codecs and SQLite integration. Existing IFC4 work in this ticket remains independently owned; preserve its changes and reconcile exhaustive scalar matches with the new variant.

The schema cannot claim complete primitive support while BINARY is absent. Initial edit test failed on unknown kind before implementation; TypeScript exact binary editing/undo now passes, including invalid-padding refusals.
