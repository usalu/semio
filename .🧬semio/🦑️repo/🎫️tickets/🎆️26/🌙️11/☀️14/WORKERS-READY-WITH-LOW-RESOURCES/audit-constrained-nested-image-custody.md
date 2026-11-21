# Nested Image Custody at Cold Lab Cleanup

The ticket launcher checked the native image inventory for backing paths below the outer mount before detaching its sparse image. A separate image could have a backing file elsewhere and a mounted entity inside the lab. The previous check also missed paths spelled with the Darwin `/tmp` alias. The focused regression reproduced the missed external-backing case with a private in-memory inventory.

Before outer detach, the launcher now checks the backing path and every mount point in the inventory for descendants of the private lab root. The check recognizes the fixed Darwin temporary-directory alias and uses POSIX path normalization. The launcher refreshes the inventory after any receipt export and requires the exact backing and mounted device association to still match. It preserves the outer image if a nested image is present or the association has changed. The cleanup path continues to require the private backing inode and mounted directory identity, and uses a non-forced detach for that outer mount only.

The neutral fixture covers a partially attached nested backing, an external backing mounted into the lab, an aliased backing, an aliased mount among several mounted entities, a sibling path with the same prefix, and a normalized descendant. SQLite independently evaluates path containment against the fixture's canonical paths. The focused test failed on the old backing-only guard. The full wrapper suite passed 18 tests with 186 assertions through `repo tools exec --light bun -- test`. Verification used fixture inventories and private ledgers.

At 13:36:29 UTC on 2026-11-21, the actual generated backing directory and `/private/tmp` both reported device 16777234 and 26,462,887,936 available bytes. The 41 GiB profile requires 60,129,542,144 available bytes, a gap of 33,666,654,208 bytes. A prior standalone control also established a host StorageKit stall even without the cold-lab mount. Cleanup protection applies once the lab owns an image. The physical admission and cold all-stage verdict are still pending. No live image tool operation or process signal was issued in this check.

- [Native attach investigation](audit-constrained-native-attach.md)
- [Constrained launcher](constrained.ts)
- [Language-neutral inventory fixture](constrained.cases.json)
