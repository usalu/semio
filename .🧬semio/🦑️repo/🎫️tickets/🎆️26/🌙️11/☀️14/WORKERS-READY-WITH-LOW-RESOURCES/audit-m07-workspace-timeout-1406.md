# m07 workspace exit at 13:52 UTC

A fresh physical timing receipt at 14:06 UTC distinguishes the m07 attempt beginning at 13:02:46 UTC from its predecessor. The newer stopped attempt ran 2,995 seconds and ended at 13:52:41 UTC with SIGTERM and exit receipt evidence. The timeout recorded 2,976 seconds in the graph step against its 600-second bound and 606,812 ms of quiet. The retained stage output resumes from workspace process 80197, reports the initial 512 MiB run-many admission and planning selected targets, and records its terminal signal. It does not contain a verified plan reply.

The same tail records cleanup of process 79743. The retained ownership record ties that process to the earlier stopped 11:37:42 UTC attempt and says no live process of the current stage was its ancestor. Cleanup is a separate event from process 80197's termination. The subsequent containment inventory still identifies a separate service as shared. These receipts do not prove a retry or cleanup caused the newer attempt to stall.

The later m07 receipt reports a running workspace retry around 14:06 UTC, and its recent log records 252,564 source observations and 31 completed owner checks. No full Ready or terminal success is recorded for that recent physical attempt. The receipt does not identify the loaded resident client source. No process or shared storage state was changed by this check.
