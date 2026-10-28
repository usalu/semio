# docs3-g doc batch `probes`

Lands WITH the code batch `toolsetup-g-tools/staged/probes (or probes-after-conv when conv-g landed first)` (toolsetup-g). The planner skip finally hits. The twin code batch `probes-after-conv` needs the same doc batch.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
