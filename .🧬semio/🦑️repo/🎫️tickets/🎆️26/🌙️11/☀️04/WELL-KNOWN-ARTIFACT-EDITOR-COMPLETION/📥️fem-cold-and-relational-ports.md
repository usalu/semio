# FEM Cold And Relational Ports

Read-only; no source edits or jobs.

FEM Node actual owner: ✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/🦀️.rs. Macro produces public FemNode{id:SealedShared<String>, coordinates}. It exposes validate(), no direct cold constructor/From<FemNodeValue>. Material owner ⚙️engine/🧱️material/🦀️.rs similarly seals id+name and exposes validate/from_parts through NativeMaterialRow. The from_parts trait accepts already sealed text owners; it does not construct them from authoring Strings.

Value 📋️list/🦀️.rs:877/881 provides PagedList FromIterator and From<Vec>. This solves collection composition only. SealedShared owner value/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs provides funded admit(value,SharedIssuer,grant), prepare_birth/publication, not plain new/From. No existing clean grant-free cold text-sealing constructor was found.

Therefore current FEM from_reference's fresh maximum decoder path cannot simply be replaced with existing direct node/material constructors. A clean explicit cold boundary needs a properly owned cold construction API at node/material/shared ownership layer, while interactive receive_authoring remains rich and original-funded. Never flatten NativeSnapshotFailure into FromValue's ValueError. Do not use projected authoring conversion inside interactive intake.

## Actual relational override anchors

Nine artifact owners provide paired named project_receiving/reconstruct_receiving functions. Exact paths/lines:

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:50
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:51
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:57
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:58
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:59
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:60
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:60
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:61
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:48
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:49
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:56
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:61
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:79
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:80
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:49
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:55
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:52
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:53

TSV/CSV/Binary own scoped_native+receive and subsequent settle. Their rich migration must preserve both producer failure and settlement failure, actual work already performed, and owned output on post-effect refusal. LAS/EPW/SemioText delegate shared receive_sqlite_snapshot; JSON delegates receive_decoding/receive_encoding; STL/TXT have their explicit wrappers. All current functions return Result aliases, so changing only lower callbacks leaves owner trait boundaries stale. Editor received the census. Existing genuine Port.work calls must feed actual counters before fallible domain actions, not physical copied_items or constant callback counts.
