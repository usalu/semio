# docs3-g doc batch `journal-fix`

Lands WITH the code batch `verify2-g-tools/staged/journal-fix` (verify2-g). The change-journal service survives the live tree. Land before `vouch`.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
