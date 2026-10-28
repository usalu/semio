# docs3-g doc batch `oracle2`

Lands WITH the code batch `oracle2-g-tools/staged` (oracle2-g). The read oracle proves huge keys, waits for the unified log, and uses no local time stamp.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
