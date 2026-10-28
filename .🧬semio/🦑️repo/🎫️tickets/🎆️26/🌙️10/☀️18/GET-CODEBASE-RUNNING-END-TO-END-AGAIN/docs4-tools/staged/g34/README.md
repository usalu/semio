# docs4 doc batch `g34`

Describes behaviour that is live (code: planfp staged3 + hotpath2 rowid, jdir, memolist (live since 10:35, gate verdict g34)). The G34 group: memo stamps for unread siblings, the journal's new place, runtime rows without a child, the plan walk skipping ignored directories.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
