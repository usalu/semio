# W1 result

Cause: `catalogNativeManifestCandidatesV1` crashed on `lstat` ENOENT (log 20:41). A peer edit (uncommitted, before my work) already made the candidate loop treat ENOENT/ENOTDIR as absent (implicit candidate dropped; declared one raises BUILD_OWNER_MANIFEST_ABSENT, entering registry repair) and covered it in the existing test "a vanished declared catalog Manifest...". The observation `probe` records `!ENOENT` before rethrowing, so the next read refreshes.

Remaining race fixed: `existsSync(package.json)` then `readFileSync` in owner discovery. New helper `readVanishableTextV1` (owner-discovery `🟨️.mjs`, next to its users) returns undefined on ENOENT/ENOTDIR, rethrows EACCES/EIO. Test added (temp dir + injected fs). Other lstat sites (ancestry, directory walk, kind) already tolerate ENOENT/ENOTDIR or are deliberately fatal (workspace ancestry).

Tests: owner-discovery observation 4 pass; source-discovery 5 pass (SEMIO_TEST_ARTIFACT_DIR set).
