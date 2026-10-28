# docs4 doc batch `plan-reuse`

Describes behaviour that is live (code: racy-tools + jobskey + grantenv + ignoredenv + dashplan + universe + planfp 1-2 (all live)). Bazel plan reuse: what keeps a plan, what ends it, the two lines, the troubleshooting rows; the 'not measured yet' sentence becomes the measurement.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
