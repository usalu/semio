# docs3-g doc batch `fdleak-guard`

Lands WITH the code batch `fdleak-g-tools/staged/guard (after census2)` (fdleak-g). The `bun test ./` rule, the open-files ceiling and the orphan guard. The code batch carries its own bullet in the test-runner paragraph; this batch adds the ENFILE and message rows and sharpens the single-file rule.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
