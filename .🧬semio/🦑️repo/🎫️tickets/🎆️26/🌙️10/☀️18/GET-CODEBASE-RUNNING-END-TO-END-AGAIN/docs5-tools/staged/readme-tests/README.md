# docs5 doc batch `readme-tests`

Describes behaviour of: td-misc + td-leases + td-ucache + td-graph findings; envelope and scratch helpers live since 10-27. README, Test sandbox: running a single file from the root, declaring the birth, placing scratch (EN and DE), pointing at the operator guide.

Claims that depend on a batch that has not landed yet carry the marker `pending gate` (EN) / `wartet auf das Tor` (DE) in the text and are listed in `edits.json` under `pending`: none

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 or docs4 batch.
