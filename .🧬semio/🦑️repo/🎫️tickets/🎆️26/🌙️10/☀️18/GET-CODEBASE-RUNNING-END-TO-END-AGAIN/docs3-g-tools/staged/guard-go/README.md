# docs3-g doc batch `guard-go`

Lands WITH the code batch `none (guard-g edits are live, outside the library)` (guard-g). The Go test sandbox compares roots by identity and scratch leaves the checkout. Land any time.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
