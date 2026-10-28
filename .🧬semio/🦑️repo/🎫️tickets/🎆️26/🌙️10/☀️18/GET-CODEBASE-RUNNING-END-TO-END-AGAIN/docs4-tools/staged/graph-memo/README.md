# docs4 doc batch `graph-memo`

Describes behaviour that is live (code: boot-g graph-memo + memokey + memokey2 + memonotice + factsound + hotpath clonejournal (all live)). The graph memo: when a repeated request is served without a graph child, the two lines it prints, the switch, the refusal reasons.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
