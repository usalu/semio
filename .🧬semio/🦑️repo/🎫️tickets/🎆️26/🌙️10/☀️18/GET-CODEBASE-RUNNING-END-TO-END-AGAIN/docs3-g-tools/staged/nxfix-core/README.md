# docs3-g doc batch `nxfix-core`

Lands WITH the code batch `nxfix-g-tools/staged/core (its manifest entry for the guide)` (nxfix-g). nxfix-g's `core` replaces the WHOLE guide file (base sha256 must equal the live file) and refuses after any other guide edit. Land this batch instead and remove the guide entry from core's manifest, or land core before every other doc batch. The two blocks are byte-identical to core's, so either order ends in the same guide.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
