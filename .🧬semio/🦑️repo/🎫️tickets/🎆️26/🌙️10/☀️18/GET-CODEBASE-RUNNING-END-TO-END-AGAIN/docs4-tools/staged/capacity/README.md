# docs4 doc batch `capacity`

Describes behaviour that is live (code: quietfc round 1+2 (live)). The quiet reservation of a hot no-op, the bound of what it grows into, the partial grow line and its troubleshooting row.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
