# Remodel asset identity oracle

The Python mutation oracle now derives asset identities from the payload. It hashes the UTF-8 bytes of the MIME type, U+001F, and the exact Base64 text using `hashlib.sha256`. The first 16 lowercase hex digits follow the `remodeling-asset-` prefix. The same derived ID keys the handle and durable content. The oracle also writes the decoded bytes as 4 KiB octet arrays, matching the committed JSON wire format.

All three successful create-asset vectors already contain the correct IDs in their after snapshots and diff rows.

| Case | Computed child ID |
| --- | --- |
| Stores a new asset | `remodeling-asset-0097f564e9451b76` |
| Stores an orbit frame | `remodeling-asset-8ee41804fb96ac80` |
| Overwrites an orbit frame | `remodeling-asset-0605a8d410402e0f` |

The existing overwrite diff removes `remodeling-asset-b5e857b335c98470` and inserts the new leaf. The refused create vector already preserves its original snapshot. The fixture files needed no edits.

The focused Python harness uses the repository's actual Python test host and the ticket's existing feature expansion helper. Before the fix, three forward create-asset cases failed due to the previous Base64 chunk representation. After the fix, all 18 create/delete asset forward, refusal, and inverse scenarios passed. The harness also replaced a valid expected handle ID and corresponding durable key with a forged ID in memory. The new oracle rejected the forged expectation.

`repo tools exec python` failed because the host-capacity queue lives in a read-only cache in this environment. The same harness passed under `python3 -B`. No Rust build ran. A broader baseline probe also exposed unrelated wire drift in other mutation kinds, including floating-point bit wrappers and decimal-string unsigned values. Those cases fall outside this asset identity check.
