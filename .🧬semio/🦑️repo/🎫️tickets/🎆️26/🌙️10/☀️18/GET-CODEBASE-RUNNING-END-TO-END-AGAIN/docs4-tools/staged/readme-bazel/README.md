# docs4 doc batch `readme-bazel`

Describes behaviour that is live (code: racy-tools + jobskey + ignoredenv + dashplan + planfp 1-2 + actenv v3 + envleak (all live)). README, Bazel implementation: plan reuse and the action environment allowlist, EN and DE, pointing at the operator guide.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
