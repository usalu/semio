# docs3-g doc batch `jobsrv`

Lands WITH the code batch `jobsrv-g-tools/staged` (jobsrv-g). Cargo's job-token pool is handed on to build scripts and the compiler wrapper.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
