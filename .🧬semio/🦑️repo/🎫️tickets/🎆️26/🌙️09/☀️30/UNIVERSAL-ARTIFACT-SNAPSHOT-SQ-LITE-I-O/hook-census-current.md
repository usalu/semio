# Actual Native Publication Hooks

Read-only source census on 2026-10-09. Exact `fn native_snapshot_registration`/`fn publish_native_snapshot` enumeration across current Rust source found six production owner implementations plus the Store trait defaults. This is the complete finite set for these explicit hook definitions, not all public artifact bindings or installed runtime rows.

| Actual Snapshot | Authored Coordinate | Actual Schema | Original Hook Lineage |
| --- | --- | --- | --- |
| RunArtifact | os.run@1/* | os.run | Workflow run binary snapshot:7/11; S_RUN_SCHEMA run root:7 |
| DagSnapshot | dag.host_snapshot@1/* | dag.host_snapshot | Infinite dag binary snapshot:7/11; DAG_DOCUMENT_SCHEMA vcs:23 |
| FlowHostSnapshot | flow.host_snapshot@1/* | flow.host_snapshot | Flow binary snapshot:33/37; FLOW_DOCUMENT_SCHEMA vcs:29 |
| CollectionSnapshot | os.collection@1/* | os.collection | Space collection binary snapshot:6/10; SQL snapshot constant:10 explicitly references S_COLLECTION_SCHEMA |
| SpaceSnapshot | os.space@1/* | os.space | Space space binary snapshot:6/10; SQL snapshot constant:10 explicitly references S_SPACE_SCHEMA |
| NativeSocketProbeSnapshot | native.socket-grant.probe@1/* | native.socket-grant.probe/v1 | Renderer parent:19199/19204; sibling sqlite constants:7/8 |

All six hooks explicitly return their original owner codec with SQL capability and publish through real atomic native snapshot registration. Every coordinate above is read from authored source, including explicit schema constant use; none is inferred by splitting a schema. No factory id is authored by these hooks. Native Snapshot TypeId is currently present through SQL capability; independent identity remains a required design frontier.

Store default hooks:12100/12102 return None/Ok. Ordinary creation Store:19720 and hydration document/history/hydration:449 call `P::publish_native_snapshot`; default Ok does not establish publication for owners lacking overrides. Socket additionally publishes at original production run:19407. Static hook ownership is not proof those lifecycle calls executed successfully.

Genuine schema-only production probe: MCP workspace ProbeSnapshot wraps serde_json::Value at:193, owns schema os.agent.probe/v1 at:184, and declares SQL capability in ArtifactPack:220–223. `ensure_probe_codec_registered`:399–403 publishes the actual document codec only. It has no native_snapshot_registration/publish override and no authored dialect found. This is a real typed SQL document owner with absent public native coordinate, not an absent SQL owner. Keep its document channel and absent dialect/factory explicit. Do not invent os.agent.probe@1/* merely because the schema resembles one.

The six-hook census cannot find all default-hook users by counting override absence: original Plugin flat/tree codecs are independently published through composition assembly plans, not necessarily these lifecycle hooks. Complete denominator still needs prefilter original local/hosted/foreign specs and every tree subset. Current plan filters SQL after collecting schema-only document rows; missing capabilities and original channels remain lost after publication. No exact count of genuine absent-SQL rows is established by this hook-only audit.

Retained catalog source counts remain33 rows/32 kinds/33 real factories, and Stdio declaration fixture remains89 coordinates, separately. No sum or universal runtime conclusion. Root reports Source20397 terminal0 strict4/126, Native67298 preparation refusal before compilation, retry7471 pending; this audit did not inspect those terminal receipts and grants no new execution credit.
