# docs4 doc batch `plan-walk`

Describes behaviour that is live (code: printhot gitrow + audit3 excluded (live)). Bazel plan witness: the git row digests only admitted files, the source walk asks git about excluded names.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
