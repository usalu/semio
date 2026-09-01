# Store Rejected Page Close Test Mount 68

## Source-Only Result

Mounted only the approved top-level cfg(test) child after `owned_schema_record_tests` and before the inline Store tests module, at [Store line 19607](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:19607):

```rust
#[cfg(test)]
#[path = "🧪️tests/🧬️rejected-page-close/🦀️.rs"]
mod owned_field_rejected_page_tests;
```

This is a 114-byte insertion. No native compiler or test ran. In particular, the two new rejected-page laws have not yet reached their intended semantic RED; the separate known OS compiler errors must first be repaired. No bounded-close or runtime success is claimed.

## Authority And Preservation

Runtime approved the exact private-owner sibling mount, conditioned on taxonomy membership. Root fully read taxonomy's [released one-member report](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️store-rejected-page-close-vocabulary/📝️.md), then runtime explicitly released this mount. The shipped taxonomy member is `🧬️rejected-page-close` under tests; catalog SHA256 `7800b09ba8644260ba818e0aff7c51bbe9e6271a0bb374b3595790baa3b577d7`.

Whole Store before: 1541032 bytes / `ed1d6b93b36a07f3c2aa914350c97f993613bc1a779e3b019a8f1329c7e19a37`.
Whole Store after: 1541146 bytes / `7c71a7bf09b8bac3fbfd8b420b98f3a82ae89d62ebd0c868f5e6e97d8bffc2c4`.

An independently executed in-memory removal of the one unique inserted block reproduces the exact whole-file preimage hash. No file was restored. Therefore this patch preserved both production rejected-page wrappers, current Retained codec changes, and every other Store byte.

The three test leaves, preparation controller, and taxonomy file all retained exact pre/post hash, byte count, device, inode and mtime values. Preparation controller `source` deliberately checks an **unmounted** state; its registered unmounted-preparation command is historical preparation, not a mounted-source or native acceptance gate. It was not rerun or reinterpreted after this mount.

## Exact Readback

```json
{
  "proof": {
    "count": 1,
    "includeBytes": 114,
    "postSha256": "7c71a7bf09b8bac3fbfd8b420b98f3a82ae89d62ebd0c868f5e6e97d8bffc2c4",
    "inverseSha256": "ed1d6b93b36a07f3c2aa914350c97f993613bc1a779e3b019a8f1329c7e19a37",
    "wholeSourceExactInverse": true,
    "includeLine": 19607
  },
  "before": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs",
      "bytes": 1541032,
      "sha256": "ed1d6b93b36a07f3c2aa914350c97f993613bc1a779e3b019a8f1329c7e19a37",
      "dev": 16777230,
      "ino": 122164261,
      "mtimeMs": 1787884550463.1267
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️rejected-page-close/🦀️.rs",
      "bytes": 22610,
      "sha256": "ce76c55dbfa74756365226a8be5bcc0c7155853c40336bdf9df4cea583f8cd4f",
      "dev": 16777230,
      "ino": 134945298,
      "mtimeMs": 1787882503509.6143
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️rejected-page-close/🔣️.json",
      "bytes": 10143,
      "sha256": "efe7c7d8de5e99f140b606c58134afab3e4d375dbb8a0489b543a92aab0524bb",
      "dev": 16777230,
      "ino": 134942164,
      "mtimeMs": 1787880927428.4202
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️rejected-page-close/🧬️schema/🔣️.json",
      "bytes": 5454,
      "sha256": "b26e851b5cd1317b4ca799dbbfc117ed33df010ad0178bcf5a2e5db3820bb9a1",
      "dev": 16777230,
      "ino": 134942163,
      "mtimeMs": 1787880927428.0603
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-rejected-page-close-66/📜️script.ts",
      "bytes": 14931,
      "sha256": "5fb860042a37e7a511a127f814aa349d33dbd0d67063ea07d036b5545604306e",
      "dev": 16777230,
      "ino": 134943802,
      "mtimeMs": 1787881133581.8208
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
      "bytes": 386265,
      "sha256": "7800b09ba8644260ba818e0aff7c51bbe9e6271a0bb374b3595790baa3b577d7",
      "dev": 16777230,
      "ino": 129218887,
      "mtimeMs": 1787884600808.8196
    }
  ],
  "after": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs",
      "bytes": 1541146,
      "sha256": "7c71a7bf09b8bac3fbfd8b420b98f3a82ae89d62ebd0c868f5e6e97d8bffc2c4",
      "dev": 16777230,
      "ino": 122164261,
      "mtimeMs": 1787885080596.9277
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️rejected-page-close/🦀️.rs",
      "bytes": 22610,
      "sha256": "ce76c55dbfa74756365226a8be5bcc0c7155853c40336bdf9df4cea583f8cd4f",
      "dev": 16777230,
      "ino": 134945298,
      "mtimeMs": 1787882503509.6143
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️rejected-page-close/🔣️.json",
      "bytes": 10143,
      "sha256": "efe7c7d8de5e99f140b606c58134afab3e4d375dbb8a0489b543a92aab0524bb",
      "dev": 16777230,
      "ino": 134942164,
      "mtimeMs": 1787880927428.4202
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️rejected-page-close/🧬️schema/🔣️.json",
      "bytes": 5454,
      "sha256": "b26e851b5cd1317b4ca799dbbfc117ed33df010ad0178bcf5a2e5db3820bb9a1",
      "dev": 16777230,
      "ino": 134942163,
      "mtimeMs": 1787880927428.0603
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-rejected-page-close-66/📜️script.ts",
      "bytes": 14931,
      "sha256": "5fb860042a37e7a511a127f814aa349d33dbd0d67063ea07d036b5545604306e",
      "dev": 16777230,
      "ino": 134943802,
      "mtimeMs": 1787881133581.8208
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
      "bytes": 386265,
      "sha256": "7800b09ba8644260ba818e0aff7c51bbe9e6271a0bb374b3595790baa3b577d7",
      "dev": 16777230,
      "ino": 129218887,
      "mtimeMs": 1787884600808.8196
    }
  ]
}
```

## Separate Work

The 84 outer-sync-test diagnostic join and mandatory base Mutation metadata restoration remain separately planned; neither is part of this insertion. Fresh decoders, poison recovery, registry behavior, R17 backbone ownership, Interaction and lifecycle remain excluded. All prior evidence remains preserved.

