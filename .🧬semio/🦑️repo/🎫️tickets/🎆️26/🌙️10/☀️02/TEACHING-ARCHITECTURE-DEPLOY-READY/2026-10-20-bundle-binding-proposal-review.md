# Bundle Binding Proposal Review

Read bundle-image-binding-plan.md and its current-owner scope. No Source edits, tests or commands. No material design blocker found.

Pin proctor configID before label/stack extraction and use immutableID for those reads; pin authored Caddy digest-resolvedID and tag portablealias from thatID. Saving required proctor+portableCaddy tags preserves RepoTags. Verify actual archive requiredtag→actualconfigbytes SHA against those pinnedIDs; before/aftertagchecks alone do not close ABA. This proposal's archive check does, without new receipt/returnfields or claiming layerbody authentication.

Private domain scanner is justified as narrow plainTAR metadata reader because existing compressedarchiveowner is not suitable. Reuse framing concepts, keep explicit finite archive/header/JSON/PAX limits, skip opaque layers without extraction/buffering. Requiredtag association must tolerate identical config across disjointaliasrows while refusing duplicate requiredtags or conflicting config/layer associations. Safely allow only exactzero-sized rootdirectory headers if Dockerexport includes them. Handle actual OCIhybrid config/layerpaths or refuse explicitly rather than accept unexplainedambiguity. Do not inherit1GiB optionalexporter cap as a product producerrestriction.

Existing-owner regression with tar-stream archiveoracle, noble hash, Ajv closed20casecorpus and actual injectedexporthelper handles test-first/multiimplementation requirement. Author tests before producerchanges; no observed RED/GREEN claimed. Runtime received these constraints; await concrete implementation.
