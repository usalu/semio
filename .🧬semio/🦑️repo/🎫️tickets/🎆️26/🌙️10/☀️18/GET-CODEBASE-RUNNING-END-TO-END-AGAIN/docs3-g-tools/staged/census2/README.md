# docs3-g doc batch `census2`

Lands WITH the code batch `census2-g-tools/staged (supersedes testmem2-g-tools/staged: land exactly one)` (census2-g). Per-file memory ceilings for every owned `bun test` process. The code batch's own bullet is in the test-runner paragraph; this batch adds the message row and the budget-table row.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
