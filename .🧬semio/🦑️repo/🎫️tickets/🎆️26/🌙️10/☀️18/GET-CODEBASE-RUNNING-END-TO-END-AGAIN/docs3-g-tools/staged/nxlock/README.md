# docs3-g doc batch `nxlock`

Lands WITH the code batch `nxlock-g-tools/staged/nxlock (after nxfix-g core)` (nxlock-g). Nx daemon start order and the inert lease classes.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
