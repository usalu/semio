# docs3-g doc batch `fleet`

Lands WITH the code batch `obs-g-tools/staged/fleet` (obs-g). `repo fleet status` and the MCP tool `fleet_status`. Rows 1-4 are byte-identical to obs-g's own `apply_guide.py`, so either may land first.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
