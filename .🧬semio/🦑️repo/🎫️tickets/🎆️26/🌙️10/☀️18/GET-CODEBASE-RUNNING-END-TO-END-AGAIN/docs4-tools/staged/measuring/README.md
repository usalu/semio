# docs4 doc batch `measuring`

Describes behaviour that is live (code: envleak + hotpath + all of the above (live); measurements of 10-27). A new section on measuring a build (clean environment, the third run, the proof lines) and the 10-27 measurements in the status.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
