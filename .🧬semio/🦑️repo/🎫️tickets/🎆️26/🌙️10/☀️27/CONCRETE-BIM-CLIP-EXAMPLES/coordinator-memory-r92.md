# Coordinator Memory Release

During shared host pressure the root NodeREPL reported1.400GB physical/total V8heap and1.398GB used after repeated source/diagnostic reads. All edits and evidence are already persisted. The coordinator resets only its own idle persistent evaluation worker to release obsolete parse heaps. Separate repo-managed queued tests20340/16894 are unaffected; no peer process or repository capacity policy is changed.
