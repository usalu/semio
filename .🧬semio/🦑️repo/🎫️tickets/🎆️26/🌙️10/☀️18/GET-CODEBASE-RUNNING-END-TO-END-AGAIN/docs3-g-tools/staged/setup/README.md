# docs3-g doc batch `setup`

Lands WITH the code batch `setup-g-tools/staged` (setup-g). Zero-touch native prerequisites: preflight names them, `repo tools setup all` builds the Node native core and the compiler helpers.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
