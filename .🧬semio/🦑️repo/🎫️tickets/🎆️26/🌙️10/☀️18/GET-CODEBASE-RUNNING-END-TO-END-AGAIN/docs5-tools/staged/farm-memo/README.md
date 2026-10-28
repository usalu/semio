# docs5 doc batch `farm-memo`

Describes behaviour of: farmid captured (live since G44), farmid authority + gatefast scopedcut (pending). The graph farm and its lines, why it is refused today, and the scoped verification of a slice with its derived-results line.

Claims that depend on a batch that has not landed yet carry the marker `pending gate` (EN) / `wartet auf das Tor` (DE) in the text and are listed in `edits.json` under `pending`: adoptInferenceAuthorityV1

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 or docs4 batch.
