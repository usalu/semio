# Mesh Document Source Boundary

Before the authored UV implementation, the independent packaging test measured the Mesh Document crate above its 262,144-byte source limit. The final closure measures 258,960 bytes, 3,423 active lines and 32 files. The repository limits remain 262,144 bytes, 4,000 lines and 32 files.

The only direct production caller of the old tessellation advance method is retained face preparation, which admits phases 0 through 6. The other callers are setup for native emission tests and stop at phase 7. Retained corner, edge, triangle and attribute output routes through separate producers. Removing the unreachable old emission branches and duplicate helpers leaves those call sites intact. The removed private polygon splice and compact-and-rebuild chain had no callers outside their own definitions. The replaced preview index path now releases its scalar vector using the quoted physical backing instead of retaining an unnecessary nested closure owner.

The final independent packaging suite passed workspace-owner, direct-codec, source-closure and exact-lock checks. The production-linked authored UV test also passed full tessellation with raw and indexed channels through the retained output route.
