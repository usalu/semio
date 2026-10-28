# docs3-g doc batch `freespace`

Lands WITH the code batch `xplat4-g-tools/staged/freespace (after xplat3)` (xplat4-g). One free-space question that answers 'unknown'.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
