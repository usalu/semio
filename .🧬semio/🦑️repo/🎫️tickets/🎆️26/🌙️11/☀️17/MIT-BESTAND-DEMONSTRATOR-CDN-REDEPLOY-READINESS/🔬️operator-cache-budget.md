# Operator cache collection on the release host

The manual collector used the absolute 256 GiB ceiling. The writer and status command already derived their limits from the role and measured partition. The same 43 GiB host store consequently returned zero eligible entries from the manual command. The operator now uses the existing derived policy, including its separate proportional limit for held compiler objects. Pin, lease and holder protections still apply. The three commands continue using the existing host role and disk measurements.

The independent Python capacity fixture reproduced a 48,318,382,080-byte requirement where the operator returned zero. The corrected operator matched the fixture. The full CAS suite also exposed a valid replacement usage hint arriving within the same millisecond as the prior hint. The admission guard now sees that changed byte count. A newer usage record now takes precedence for sizing when a holder retirement leaves an older object census. The regression measured 8 GiB before the correction and the expected 50 GiB afterwards. All 70 CAS tests passed, with 2,266 assertions.

The actual preview identified 1,122 eligible unit entries. The collector rechecked live registrations and removed 1,095 entries and 3,931 blobs, freeing 12,038,417,023 bytes. It protected 8,853 entries. Partition pressure accounted for zero bytes of the holder retirement. Free disk space rose from 13 to 24 GiB. The next Procedural build passed the earlier descriptor lock failure.

At 20:10 Pacific a fresh protected preview estimated 7,836 MiB eligible. The official collector retired 5,924 eligible entries and returned 7,826 MiB. Host free disk space reached 21 GiB and the active release resumed compilation above the image watermark.
