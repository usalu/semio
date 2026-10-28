# docs4 doc batch `status`

Describes behaviour that is live (code: gate runs g23-g42 (measurements)). Replaces the 10-26 status, the G34 note and the 10-26 baseline with today's measured numbers; updates the measured line of 'Measuring a build' and the graph-builder row.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
