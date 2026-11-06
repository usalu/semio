# Job Source Census

Read-only rust-source-census.ts against current manifests; caps unchanged.

| Package | Configuration | Bytes | Files | Active lines | Result |
| --- | --- | ---: | ---: | ---: | --- |
| semio-framework-job-context | production | 271245 | 24 | 1942 | Exceeds 262144 bytes by 9101 |
| semio-framework-job-context | test cfg | 271245 | 24 | 2022 | Exceeds 262144 bytes by 9101 |
| semio-framework-job-native-decode | production | 20922 | 1 | 145 | Within |
| semio-framework-job-native-decode | test cfg | 46750 | 8 | 248 | Within |
| semio-framework-job-receiving | production | 12646 | 3 | 85 | Within |
| semio-framework-job-receiving | test cfg | 20193 | 6 | 131 | Within |

These are source-closure observations, not compiler or native-test results. The context owner requires a real domain partition before further growth.

After concurrent canonical job-outcome ownership partition, the current Job context production closure is 258610 bytes, 23 files, 1766 active lines, leaving 3534 bytes under the unchanged byte cap; test cfg is 1846 active lines. This does not establish a compiler verdict.

The source-native encoding domain now has its own package, retains its existing native test target, and depends on the actual Job context via a concrete extension trait. The Job facade reexports that owner. Current census: Job context 236460 bytes / 22 files / 1657 active production lines; new native-encoding owner 22584 bytes / 1 file / 112 active production lines. Caps unchanged. Rustfmt parses the moved source and test; Cargo/native checks have not run.
