# docs3-g doc batch `quietfc`

Lands WITH the code batch `quietfc-g-tools (apply.sh)` (quietfc-g). Needs the code batch applied first: its own guide edit adds the three bullets (quiet follow-up, envelopes, declared machine) with the word 'staged'; this batch turns that word into 'live', closes the 4b status, and adds the waiting row.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2) or missing code prerequisite (exit 3), writes nothing then. Revert the doc batch whenever the code batch is reverted.
