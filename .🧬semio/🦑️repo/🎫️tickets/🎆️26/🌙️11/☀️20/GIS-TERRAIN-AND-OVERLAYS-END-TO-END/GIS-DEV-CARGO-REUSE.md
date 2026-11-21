# Cargo Reuse in the GIS Dev Build

The native component progress counter counts Cargo `compiler-artifact` messages with `fresh: true` as reused. The separate compiled counter includes every broker unit. The shared unit-cache wrapper also reports replayed units to the broker for provenance, so the compiled counter includes those hits even when no real compiler ran.

A snapshot of the narrower build’s Cargo JSON contained 698 non-fresh artifacts and zero fresh ones. The shared component unit-event ledger over the same interval recorded 499 compiler misses and 199 local cache hits replayed to the broker. The two counts add up to the 698 artifacts reported by Cargo.

The predecessor used Cargo lane `pacfecf/t-ce555540` and recorded 2,041 fresh units. The narrower build selected `p07afd0/t-ce555540`. The predecessor lane was absent on disk during the narrower build, so its lane-local Cargo fingerprints were unavailable. Both runs show compiler-image cleanup. The predecessor requested 12.1 GiB of cleanup and the narrower build requested 1.0 GiB. Those summary lines do not name the individual removals.

The two runs produced the same relative artifact path for the `proc-macro2` build script, including Cargo fingerprint directory `0396e82ec9363e1c`. The predecessor retry reported that artifact as fresh. The new lane reported it as stale. The product lane key hashes the workspace, manifest, profile, command, producer, toolchain digest, and Cargo arguments. A change to the transitive dependency graph alone does not change that key. The logs do not establish which product-key input differed.

The existing warm retry path is the same standard playground invocation on the same compiler product. It reacquires the same lane while the lane remains on disk and uses Cargo fingerprints from the last run. The default compiler chain can also restore shared unit artifacts across lanes. Keeping the current command alive preserves its lane custody while the build is running.

Later retries stayed on the narrower build’s lane. The compiler-evidence verifier reported disagreement on `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs` once and `🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🦀️.rs` twice. None of the four Cargo JSON logs contained a compiler error. The verifier recompiled in the held lane after the evidence changed, and the retries restored ordinary Cargo freshness.
