# Archive loading and local memory

The 152 source documents total 198 MiB on disk. In compact JSON their volumes alone use 104.3 MiB and their drawings use 28.4 MiB. The archive manifest checker reads drawing availability, dimensions, poses, thermal declarations and export metadata. It can finish before a single digital scale is opened. The four scale records can therefore remain encoded in the shipped payload and be decoded on first use while preserving the original JSON values and revisions.

The local validation pool reserves 3,584 MiB across ticket processes, including native descendants. Catalog sweeps use at most 2,816 MiB of that pool. Separate jobs check expensive rule groups when running the groups together exceeds their per-process ceiling.
