# Input Closure Publication Identity Race

The farm four-adopter regression failed during immutable closure verification in the memory compatibility run. Atomic rename can replace the directory entry while a metadata lookup retains the old inode; the old inode can report zero links after replacement. A deterministic actual-filesystem test holds the old descriptor, publishes a replacement, and supplies the actual detached inode status at that lookup boundary. It failed before implementation. This reproduces a valid race; the earlier unspecialized error did not record its exact stat fields.

The publisher now retries only a differing device/inode after an invalid metadata observation, using the existing closed filesystem read retry profile (six attempts and declared progress/backoff). Stable hard links, wrong size, nonregular objects and corrupt bytes still fail. It never accepts a multilink leaf. The unchanged verified-stamp fast path remains.

Five closed neutral cases validate through Ajv and an independent SQLite oracle: standing, identical replacement, stable hard link, corrupt bytes and corrupt replacement. Fresh test38800 passed the neutral regression plus the existing real four-process/three-round adoption test:2tests35assertions,5.28seconds. The first combined attempt exposed only the test artifact path exceeding the existing256-unit limit; scratch now requires caller-owned ticket artifacts with a short f- prefix. Maximum staged path in this ticket is227UTF16 units/221codepoints; no guard was raised.

The new test is imported by the existing registered build-graph suite. Full memory/farm/publication compatibility verification follows before fresh graph integration.

Final compatibility35568 passed38tests/1791assertions across existingmemory,actualreservation/shrink,farm andpublication suites in11.94seconds. Source frozen for fresh registeredworkspace graph86952.
