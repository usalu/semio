# docs3-g doc batch `bench2`

Lands WITH the code batch `none (measurement only, the host-total patch is already live)` (bench2-g). Replaces the 'Not met' status line with the first honest baseline table. Land any time.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
