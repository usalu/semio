# Source Scan Harness Peer Delta

Observed 2026-10-21T01:24:09.010Z. Root compared retained bounded no-follow descriptor-read bodies from01:06:57 with fresh current ordinary single-link observations. This is tooling comparison, not a test or Source snapshot.

The SourceDemand test grew105097B/7b80b9dbdf6f35e8c8547203cfedd6dc55db6dfe9e2cd13a339f81895495be83 to105458B/8cf030c4dde4d14ccf1b50cce09da679bb5cf087fae6156634ed04d8d1b94df2. A single contiguous harness replacement adds strict Ajv validation against SourceInputScanGroup, ordinary-directory lstat semantics, ENOTDIR for fixture paths that are files, and the lstat port. The preceding and following bytes are exact. The existing four-case real project-assembly regression lies outside this replacement and remains byte-identical.

The graph corpus grew180862B/772f21f4254a7c1d21d1911aa34ec080ef6bc3cd83bc9ad4d53cc8e76a136e04 to181615B/71a3cd5f22ab3875fe0fe48d49b659817c4103c4b9abb7a16ee8b2f4d533f1df. The only byte addition is one753-byte source scan case, file-and-absent-glob-roots. It includes a real entry.ts and tsconfig.json file, excludes glob walks rooted at a file or absent directory, and expects only the entry.ts compile0 source. All prior corpus bytes, including the four real assembly consumer cases, are retained.

Independent semantic review is assigned to course_log_audit. No extra test title was selected and no manifest was refreshed. Both new harness behavior and corpus remain unexecuted.
