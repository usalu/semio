# Direct Typed PDF Image Samples

The stage selects `image.body.values` within the original format-1 SetImage JSON and uses the repository's `JsonNumberCursor` directly. It counts the samples while validating the integer grammar, reserves exactly that many `u32` elements in one admitted allocation, and fills the backing by replaying the original immutable numeric span. The surrounding grammar retains all other ordered data and its duplicate-member checks. The typed handoff places the vector directly in `PdfImageBody::Samples` alongside the residual metadata and both exact source extents.

The positive tests compile the unchanged production `PdfImageBody` declaration with its original derives and the actual `ArtifactRef` definitions. They compare both cold production leaf decodes and all residual ordered fields. Serde JSON independently checks every fixture sample. Both fixture frame digests match the preceding mixed projection inputs.

| Format-1 frame | Samples | Copied items | Cumulative copy bytes | Cumulative capacity bytes | Largest allocation | Work |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 44,368-byte RGB | 12,288 | 182,556 | 22,301,186 | 106,610 | 49,152 | 256,249 |
| 14,835-byte alpha | 4,096 | 60,989 | 7,447,410 | 41,007 | 16,384 | 85,530 |

The checks use mounted per-turn loans of 64 items, 32,768 copy bytes, 16,777,216 capacity bytes, 1,048,576 release bytes and depth 4,096. Both leave copy stock under the archive fixture’s 134,217,728 cumulative limit. RGB retains 51,450 bytes across the typed output and residual metadata after the cursor closes. Alpha retains 17,239 bytes. The proof measures the production typed leaf and enclosing grammar, with aggregate binding and history replay still outside the scope.

Cold `FromValue` for u32 requires an exact integer number and refuses decimal or exponent lexemes, including `4.0` and `17e1`. The fast path rejects those forms and integers longer than ten unsigned digits without invoking floating conversion. It accepts negative zero and the unsigned maximum, and rejects overflow, negative values, leading zeros, wrong element types, missing and trailing separators, duplicate JSON members and mismatched tags or discriminators. The neutral cases compare the optimized refusal category to the actual production leaf. The array grant tests verify that one-byte-short reservation and release loans have zero effect. Cancellation at the RGB reservation frontier returned Canceled at zero work and retired all 77,740 bytes recorded by the enclosing cursor.

The current concrete destination is one contiguous vector, so the stage admits at most 262,144 samples under the mounted 1 MiB release policy. The 16 MiB allocation limit alone could hold 4,194,304 samples, but that backing cannot be canceled with the current atomic release ceiling. This stage returns OwnershipLimit before allocating a vector that cannot be retired. It preserves byte-level wire representation and has no product file writes.
