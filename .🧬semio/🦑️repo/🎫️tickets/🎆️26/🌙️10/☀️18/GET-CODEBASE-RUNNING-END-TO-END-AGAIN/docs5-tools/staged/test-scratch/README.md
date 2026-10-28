# docs5 doc batch `test-scratch`

Describes behaviour of: td-ucache scratch + td-graph scratch + td-misc artdir findings. Where scratch and artifact directories must live and why: Cargo ancestors, observation ancestry, the 256-unit bound, sockets, images.

Claims that depend on a batch that has not landed yet carry the marker `pending gate` (EN) / `wartet auf das Tor` (DE) in the text and are listed in `edits.json` under `pending`: none

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 or docs4 batch.
