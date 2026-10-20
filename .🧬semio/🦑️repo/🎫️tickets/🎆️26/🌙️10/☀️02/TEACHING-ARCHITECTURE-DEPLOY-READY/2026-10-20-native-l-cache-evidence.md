# Native l Cache Evidence

Read-only filesystem API observations ended at 2026-10-20T14:28:22.681Z. No command, lease acquisition, database open, signal, detach, retirement, rewrite or deletion ran. The image is preserved.

## Current Metadata

- `.cache/b/l/image.json`: 314 bytes, SHA256 `4a5dde4d9404f89a46eb59dd93173d8994584e80719e370e50f69ab614f56c90`, inode441889631, mtime14:25:09.259Z. Current required fields are present: bootSession `1E4A0CBE-3710-481F-9614-F8F1FCBD61D0`, ancestry `[336215211,329956202,587164,35195,35146,18439,2]`. Current ancestor inode census matches that array. This API observation does not independently authenticate current kernel boot-session identity.
- Backing `fixed.dmg`: ordinary single-link file, dev16777229/inode422334229, size66,060,288B, allocated129024×512=66,060,288B, within maximum67,108,864B. Final mtime14:28:22.663Z is newer than the earlier read14:26:53.418Z.
- Partition directory: dev16777229/inode422334224. `m`: dev16777239/inode2; device differs from backing and its statfs reports4096×16118=66019328B. Its entries are .fseventsd, .metadata_never_index, data, leases.json, storage-events.jsonl, tmp. This is mounted-filesystem evidence; no hdiutil/kernel image-path association was queried.
- `.publication`, `.publication/intent.json`, and `swap.json` were absent (ENOENT). No pending publication intent is visible in this snapshot. Absence alone is not proof that no other process is using the image.
- `m/leases.json`:113B, allocation089633ab175c81617de66e2040091837, owner repository:leases-store, bytes66060288; mtime2026-10-18T02:52:26.893Z. `storage-events.jsonl`:236B with reserved→committed records for that same allocation. These are store funding metadata, not a process liveness verdict.
- `m/data`:423 entries in the initial snapshot:378 queue directories,18 direct SQLite files and27 other entries (including zero-byte `.retired`, claims and temporary/isolated subdirectories).31 directories were nonempty. SQLite databases were not opened. Five bounded direct queue rows still named PID10862 with shared/exclusive modes and mtimes14:25:17.861–14:25:45.731Z. A sixth row disappeared with ENOENT between inventory and read. This proves concurrent metadata change only; PID liveness, current lease holder identity and resource attribution were not queried.

## Comparison With dry1

The retained dry1 refusal recorded backing tuple `[actualDev16777229,receiptDev16777232,actualInode422334229,receiptInode422334229,actualSize66060288,receiptSize66060288,nlink1,allocated66060288]` and schema omissions bootSession/ancestry. The current actual backing retains **the same inode422334229, dev16777229 and size66060288**; the current receipt now records dev16777229 plus the required fields. There is no evidence of replacement of the backing inode. A field-only receipt rebind is consistent with these facts, but receipt presence and mtime do not establish who rewrote it or prove provenance. Old dry1 did not retain an old receipt inode/hash or mount-device identity, so those transitions cannot be reconstructed here.

## Current Policy And Next Admission

Current cleanup implementation `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts` has changed since dry1. `inspectPersistentReceiver` now reads boot session, checks publication settled, validates exact receipt/backing identity, and uses the owned bounded hdiutil inventory port to authenticate mounted association. A prior-boot receipt may rebind only after exact boot-stable identity, empty inactive root and unattached kernel inventory; only the exclusive writer rewrites. A current mounted publication follows exact current admission.

README Recovery requires all relevant repo/cargo/Nx processes stopped before recreation, and no kernel association remaining before removal. Its raw recovery examples are not an authorization for this agent to bypass the current native repo-only rule. This snapshot does not justify recreation, retirement or detach; it also does not justify a blanket halt. Root can use current authoritative native admission to decide whether this now-canonical publication is accepted while preserving peers. The remaining limit is kernel association/current ownership, not missing receipt fields in the current file.
