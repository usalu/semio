# Artifact Directory Removal Observation

During genuine Generated Source retirement, requiring every removed inode to have descriptor nlink0 refused a successfully removed directory. Independent Node and Bun probes on separately created Generated files/directories both observed file nlink1→0 but directory nlink2→2 after rmdir, with unchanged dev/ino. No Native/protected paths were read. Exact observations are saved under Generated/bazel-planner/artifact-unlink-node.json and artifact-unlink-bun.json.

The correct removal proof is unchanged open descriptor identity plus actual named-path absence, with nlink0 required specifically for regular files. A removed directory need not report a zero link count on this filesystem. New neutral Source vectors cover both directory link-count forms, surviving names and foreign identities. Production correction waits for Root’s brief physical module-load freeze to end.
