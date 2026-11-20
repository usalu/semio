# Verdict directory lifetime and disk collection

At 03:44 UTC the coordinator’s ordinary temporary verdict root contained 23 directories at least two hours old with 65,579 entries in total. The Windows workspace had separately reported a read-verdicts collector timeout. These facts do not establish the cause of the remote timeout or the old graph process’s exit delay.

The graph read used to enumerate the shared per-user verdict root every time a new round needed its own name. It recursively removed old entries without knowing whether a different process still owned them. A real child process deleted an older verdict belonging to the test parent. Allocating a round now creates only its own directory when needed. Private owned exit cleanup continues to remove the current process’s own directories.

The machine disk collector previously walked every verdict directory before applying its age check. It now checks the authored name format, directory type, top-level age and owner liveness before reading descendants. Before retirement it checks again using the newest observed timestamp. Private filesystem fixtures preserved live, recent and unrelated entries and removed the expired dead-reader entry. SQLite independently produced the expected retained set. All 24 disk-recovery and shared-verdict tests passed with 215 assertions.
