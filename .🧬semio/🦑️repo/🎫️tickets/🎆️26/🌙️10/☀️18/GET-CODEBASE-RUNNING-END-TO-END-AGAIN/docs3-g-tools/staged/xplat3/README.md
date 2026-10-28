# docs3-g doc batch `xplat3`

Lands WITH the code batch `xplat3-g-tools/staged (apply.py)` (xplat3-g). Windows notes and the platform fixes of the Windows audit. Creates the section 'Windows, Linux and macOS'.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
