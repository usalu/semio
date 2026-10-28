# docs5 doc batch `providers`

Describes behaviour of: peeredit2 providers + tools (live since G44); consumers batch pending. Script providers: what a command-closure edit re-infers, how to read a refresh, how a consumer expands a provider reference.

Claims that depend on a batch that has not landed yet carry the marker `pending gate` (EN) / `wartet auf das Tor` (DE) in the text and are listed in `edits.json` under `pending`: none

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 or docs4 batch.
