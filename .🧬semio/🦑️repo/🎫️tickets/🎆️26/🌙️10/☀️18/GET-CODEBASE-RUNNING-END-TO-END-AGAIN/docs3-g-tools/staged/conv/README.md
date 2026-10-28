# docs3-g doc batch `conv`

Lands WITH the code batch `conv-g-tools/staged (after oracle2-g)` (conv-g). Policy numbers in schema JSON, bilingual refusals, audit rules.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
