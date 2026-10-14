# Superseded Compilation Units

Cargo’s shared store had kept every earlier hash of the same package. Each package now keeps the newest hash directory in its profile. No `rustc` or `cargo` process was running. Newest finalized incremental sessions were left in place.

## Removed

- 16,564 superseded unit directories under `cargo/build`, `cargo/target`, and `cargo/canonical-architecture`
- Measured 150.03 GiB

## Volume

| | Available | Cargo store |
| --- | ---: | ---: |
| Before | 11.17 GiB | 221 GB |
| After | 149.92 GiB | 81 GB |

`build` is 46 GB, `canonical-architecture` is 33 GB, and `target` is 1.5 GB. A rescan found zero superseded unit hashes and 4,819 current units still present. That remainder is the live output for each package plus its current incremental session, inside the 80 GiB Cargo budget.
