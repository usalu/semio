# docs3-g doc batch `mirror`

Lands WITH the code batch `none (ticket tooling mirror-tools/, exists)` (mirror-g). Names the shared mirror tool in the staging rule. Land any time.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
