# Native image reclaim cleanup

A private fault injection reproduced two failure paths in the reclaim cycle. An attach that failed after creating a mapping skipped the cleanup block. Direct Darwin scratch disposal also recursively removed files in a modeled mount directory. A real reclaim may encounter partial attachment or failure to detach when the host is under pressure.

The cleanup path now runs after every attempted attach. It rechecks for the exact attached device before detaching. It disposes of scratch only after discovery and detachment settle, and removes only an empty directory. When work and cleanup both fail, the returned error retains both failures in order. The Linux loop adapter also preserves the original mount failure if its initial loop release fails. The Darwin port can use a separately supplied, retained cleanup runner for the final exact lookup and detach.

The SQLite-backed fault fixture checked successful cleanup and a refused detach. Other private cases verified the supplied cleanup runner and proved that a nonempty scratch directory keeps its payload. The 18 focused tests passed with 74 assertions. The ordinary 2,048 MiB command grant admitted the real Darwin image fixture. It returned at least 150 MiB from its private image, preserved the image size and inode, and verified the mapping had detached. All 19 tests passed with 82 assertions.
