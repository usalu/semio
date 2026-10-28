# docs3-g doc batch `vouch`

Lands WITH the code batch `verify2-g-tools/staged/vouch` (verify2-g). The change journal narrows the dependency verification. Land after `journal-fix`.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
