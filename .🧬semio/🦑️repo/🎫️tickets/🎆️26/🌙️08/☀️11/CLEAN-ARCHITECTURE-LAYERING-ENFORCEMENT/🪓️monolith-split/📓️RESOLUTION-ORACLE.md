# 🔬 Full-Resolution Differential Oracle

`rustc -Zunpretty=expanded` only covers early resolution: imports, globs and module paths. It misses private items referenced through type or expression paths. The wave-1 audit proved this: `WorldInteractionId` is private in `🧵️interaction/📇️registry` but used in the world root, and the old probe reported "identical".

Use full analysis instead. It discards all output, so it writes nothing to the gated store:

```
rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name <crate_snake> --emit=metadata -o /dev/null [--test] [--target wasm32-unknown-unknown|wasm32-wasip2] [--cfg 'feature="x"'] <CRATE LIB ROOT> 2> out.err
```

- Run it from the real crate lib root (the file named by `[lib] path` in the crate's Cargo.toml), so that `crate::…` paths resolve.
- Build a sandbox copy of the whole crate source tree with the ORIGINAL monolith restored, in the same relative layout so `#[path]` resolves, and run the same command on it.
- Compare the sorted multisets of error lines plus their primary span label (`^error` line plus the next `-->` line, with the file path normalised to be relative to the crate root and line numbers stripped). Errors about external crates and attributes (E0433 for crates, `cannot find attribute`) are expected in both and cancel out.
- Any new E0412, E0425, E0422, E0423, E0603, E0616, E0624 or E0432 in the split tree is a real regression. Fix it by widening the definition to the narrowest visibility that works (`pub(super)`, `pub(in crate::…)` or `pub(crate)`). Never fix it with qualified paths.
- One run takes about 7 seconds per configuration. Run native, `--test` and each wasm target the crate builds for.
- Limit: typeck can be cut short by earlier errors, so field and method privacy (E0616, E0624) may still be under-reported. After this oracle, also run a definition scan: every `fn`, field or struct used outside its defining module must have a visibility.

## Sandbox patches that make the differential effective

These come from the infinite hardening (`🗑️generated/infinite-full-oracle.py`). Apply them identically to the baseline sandbox and the current copy, and never to the repo:

1. Remove `extern crate` lines for missing crates. Otherwise rustc aborts early.
2. Replace every glob import from an external crate with `const _: () = ();`. A failed glob suppresses unresolved-name errors in its scope.

Splitter blind spot to check: free `struct` and `enum` definitions moved into a child stay private even when their fields were widened.
