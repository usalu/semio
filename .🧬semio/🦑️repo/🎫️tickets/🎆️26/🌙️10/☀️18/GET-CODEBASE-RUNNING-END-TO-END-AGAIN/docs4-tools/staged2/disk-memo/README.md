# docs4 doc batch `disk-memo`

Describes behaviour that is live (code: diskbud (live since G40)). The graph memo's 1 GiB budget and the sweep of abandoned journal state directories.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
