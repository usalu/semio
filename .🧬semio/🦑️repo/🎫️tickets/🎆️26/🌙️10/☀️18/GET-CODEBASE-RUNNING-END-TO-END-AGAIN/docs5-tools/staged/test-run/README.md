# docs5 doc batch `test-run`

Describes behaviour of: td-misc + td-graph findings (live): the root bunfig, the a1 recipe, what a bare red means. How to run one test file correctly: from the checkout root, why, how to read a bare red, and the troubleshooting row for the missing process invocation.

Claims that depend on a batch that has not landed yet carry the marker `pending gate` (EN) / `wartet auf das Tor` (DE) in the text and are listed in `edits.json` under `pending`: none

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 or docs4 batch.
