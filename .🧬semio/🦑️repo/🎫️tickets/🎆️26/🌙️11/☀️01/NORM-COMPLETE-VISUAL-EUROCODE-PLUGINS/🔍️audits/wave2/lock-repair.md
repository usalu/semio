# Lock repair

- Cause: `lib.lock_register` and `d_test_support_lock.py` parsed old deps with `re.findall(r'"([^"]+)"')`; a stray quote misalignment yielded the pseudo-dependency `",\n` which was written back as ` ",` / `",` lines (sorted first).
- Fix scripts: both now use the line-anchored `(?m)^ "([^"\n]+)",$`, which cannot match across lines.
- Repaired by removing the 11 garbage pairs per file in: hub, fixture-sweep, en1990/91/93/97/98/99, norm/bridge (the garbage was an extra entry, never a replacement; remaining deps untouched).
- Validation: all Cargo.lock/Cargo.toml in the repo (excluding node_modules/target/.git) parse with tomllib; 0 garbage lines left.
- Cross-check of lock deps vs Cargo.toml is noisy (other waves are mid-split); not hand-edited. Packages without lock entry (cargo adds them on next non-`--locked` build): en1998 mut-site-ground(-more), mut-members-wall-rows(-more); en1999 7 mut-* crates (mut-collection-rows, mut-fatigue-category, mut-ring-stiffener, mut-scenario-rows, ...).
- check-cargo-authority.py: see its tail run in this session.
