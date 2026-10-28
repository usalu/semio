# docs4 doc batch `action-env`

Describes behaviour that is live (code: actenv v3 + envleak (live)). The action environment allowlist, how a task declares a variable, runtime rows keyed by output only, the warm-cache row and the troubleshooting row.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
