# docs3-g doc batch `dashlock`

Lands WITH the code batch `dashlock-g-tools/staged/dashlock` (dashlock-g). A running dashboard never fails or blocks a build; old executable digests are reclaimed when unheld.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
