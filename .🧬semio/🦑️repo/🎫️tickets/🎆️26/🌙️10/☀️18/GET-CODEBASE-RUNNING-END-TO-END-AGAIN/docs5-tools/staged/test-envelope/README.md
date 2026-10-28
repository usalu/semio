# docs5 doc batch `test-envelope`

Describes behaviour of: td-leases envelope + td-capacity + td-graph + td-misc requirement tables. How a test file declares the process birth its suite grants so that a bare run gives the truth; what cannot be declared yet.

Claims that depend on a batch that has not landed yet carry the marker `pending gate` (EN) / `wartet auf das Tor` (DE) in the text and are listed in `edits.json` under `pending`: none

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 or docs4 batch.
