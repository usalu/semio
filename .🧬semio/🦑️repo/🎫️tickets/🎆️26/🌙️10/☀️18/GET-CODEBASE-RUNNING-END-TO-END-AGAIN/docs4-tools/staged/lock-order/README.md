# docs4 doc batch `lock-order`

Describes behaviour that is live (code: leaseord staged, staged2, staged3 + cargofix leaseorder-c (all live)). The lease classes with their ranks, the [lease-order] line, the switch and the registry; the troubleshooting row.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
