# docs3-g doc batch `purge`

Lands WITH the code batch `purge-g-tools/staged/purge` (purge-g). The one purge law of ticket output (P0 data-loss guard). Go and Rust (`repo ticket purge-artifacts`) are already live; the TypeScript `repo clean` lands with the batch.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
