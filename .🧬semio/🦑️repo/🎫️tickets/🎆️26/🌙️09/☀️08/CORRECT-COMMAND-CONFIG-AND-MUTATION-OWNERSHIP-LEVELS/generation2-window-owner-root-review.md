# Generation2d Window Owner Root Review

Root reviewed the source before native acceptance. Main graph, edit preview and generation preview now have distinct owner types, distinct schema/envelope identities, and the shared Viewport2d value. Their defaults preserve the previous0/0/1 navigation values. All three concrete render routes read their exact window config; the global app camera and SetCamera are absent from the migrated surfaces.

The actual retained NodeGraph reducer reads the captured trusted ViewModel and WindowConfigSnapshot, updates the main viewport, and emits only the exact WindowConfig mutation. Its addressed helper rejects missing, stale and wrong-kind concrete contexts. The raw NodeGraph handler is explicitly empty, as are the four existing Canvas handlers; the test must establish which retained/HostOnly route actually runs rather than infer behavior from those raw handlers.

The native source constructs two instances for each of the three kinds, supplies six independent configs, checks renderer scenes, exact result lanes, app/document Pack and SPR conservation, reopen, invalid contexts and bounded close on2MiB. Native execution is pending.

Root requested explicit deny_unknown_fields on all three Snapshot mutation enums and hostile mutation/state/Pack-envelope codec cases. The state schema already rejects unknown fields and the Pack codec checks artifact/component/version identity. The generic framework WindowConfigPack inner concrete-ID rejection remains a separate open counterexample; the successful two-instance reopen law does not replace that admission law.
