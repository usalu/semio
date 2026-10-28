# docs4 doc batch `memo-serve`

Describes behaviour that is live (code: printhot carry + reseal, memoedit + derived, hotpath2 memoout, audit1 verified (all live)). The graph memo across generations, edits and derived facts: the three info lines, what is served and what still plans, output directories.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
