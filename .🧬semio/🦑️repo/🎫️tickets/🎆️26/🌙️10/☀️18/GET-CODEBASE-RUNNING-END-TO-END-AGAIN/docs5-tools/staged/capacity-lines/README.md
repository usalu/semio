# docs5 doc batch `capacity-lines`

Describes behaviour of: agents16 allrun + behind (pending); the live capacity messages. How to read a capacity line: the reasons, the admission and growth lines, the blocker part; what the agents16 batches change for three agents on 16 GB.

Claims that depend on a batch that has not landed yet carry the marker `pending gate` (EN) / `wartet auf das Tor` (DE) in the text and are listed in `edits.json` under `pending`: hostBlockersV1, the host itself (nothing of ours), the run grows to more jobs once memory and cores free up

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 or docs4 batch.
