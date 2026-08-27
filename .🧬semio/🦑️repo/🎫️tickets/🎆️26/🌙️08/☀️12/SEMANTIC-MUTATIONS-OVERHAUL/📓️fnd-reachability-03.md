# FND-REACHABILITY-03 Evidence

The direct-mutation policy now proves each folder through exactly one top-level `pub` canonical `#[path = "<folder>/<canonical-rust-file>"] mod <folder-name>;` mount and exactly one aggregate tuple variant wrapping `<module>::Mutation` or an explicit top-level public re-export alias. It uses tokenized Rust facts, not substrings; comments, strings, nested/private/wrong-target mounts, and mismatched wrapped types do not prove reachability. Folder/variant proof failures are stable high-severity reachability and bijection breaches.

```text
bun nx run @semio-tech/repo-lib:test-quick -- -t 'proves direct leaf reachability through exact public canonical mounts and wrapped types'
PASS: 1 test, 16 expectations, 0 failures
```

The neutral fixture covers canonical, semantic alias, child-facet reexport, comment/private/wrong-target, primitive/foreign alias, aggregate scope, duplicate mount/variant, orphan, nested-mount, and wrong-type cases. Exactly the three intended valid public shapes are compiled with `rustc --edition=2021 --crate-type lib`; deliberate decoys are policy-only. Sources, rlibs, and compiler logs are retained under `🧪️reachability-compiler-artifacts` when `SEMIO_TEST_ARTIFACT_DIR` is supplied. Cargo was not run.
