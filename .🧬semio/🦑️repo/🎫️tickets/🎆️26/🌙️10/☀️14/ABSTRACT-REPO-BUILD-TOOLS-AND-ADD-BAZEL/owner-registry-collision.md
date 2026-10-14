# Current Owner Registry Collision Diagnosis

Selected16 reported `Canonical build owner identity collision` before Source inventory. The current disk declaration was checked read-only using the exact sequential conditions in `readBuildOwnerRegistryV1`: repeated names, canonical roots including previously declared native aliases, manifest containment and canonical direct-child filename, repeated native-root claims and duplicate package names.

For registry SHA-256 `8bb367c94932b84575309fee49bfaa468fed6225324ccc765976685755124ec6`, the independent census found **1,622 owners and zero issues**. The exact production reader then accepted those same bytes, exit 0, with the same SHA and owner count. Evidence is retained in `🗑️generated/owner-registry-collision.json` and `owner-registry-current-reader.txt`.

The current Source therefore does not reproduce the earlier failure, and there is no responsible owner to identify from these current bytes. Core needs the failed attempt's consumed registry digest or declaration snapshot to distinguish an earlier declaration from the current accepted one. No owner was invented, ignored or rewritten, and no validation condition, store or grant was changed.
