# docs4 doc batch `rust-path`

Describes behaviour that is live (code: cargofix synchronize + leaseorder-c + rustedit testmemo + rustfast (all live)). The Rust test and build path: the preparation witness, the lease order of the compiler writer, the admission memos.

```sh
python3 -I stage.py check|apply|revert [--root DIR]
```

Idempotent, anchored (see `stage.py`), refuses on drift (exit 2), writes nothing then. Independent of every other doc batch: no `old` text is shared with a docs3 batch.
