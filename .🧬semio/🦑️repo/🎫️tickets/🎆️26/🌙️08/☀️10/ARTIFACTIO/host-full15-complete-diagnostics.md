# Complete Host15 Flow and Infinite Receiving Diagnostics

Actual unchanged complete Flow and Infinite native Host15 terminated 1 during compilation. Both original packages, all targets, unfiltered roster, original long assertion deadlines and 600000ms build ceiling were retained. No runtime test started. All compiler primaries appear below; source readiness is not acceptance.

Raw `🗑️generated/fd/host-full15.log`; current row/env `🗑️generated/fd/host-full15.launch.json`.

## Canonical Source Family Counts

| Canonical Source | Diagnostics |
|---|---:|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | 203 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` | 16 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs` | 15 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs` | 14 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs` | 10 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs` | 10 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs` | 9 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs` | 6 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs` | 4 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🦀️.rs` | 3 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🦀️.rs` | 3 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📸️checkpoint/🦀️.rs` | 2 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs` | 2 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs` | 2 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs` | 2 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs` | 1 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | 1 |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🦀️.rs` | 1 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs` | 1 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs` | 1 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs` | 1 |

## Every Primary Diagnostic

### 1. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12336:28`. Raw line 23667.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12336:28
      |
12336 | ...   store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_by...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
12336 -                     store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
12336 +                     store::SnapshotRetirementFactory::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
      |

```

### 2. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12337:28`. Raw line 23679.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12337:28
      |
12337 | ...   store::SnapshotRetirementStep::Pending { .. } => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.pee...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
12337 -                     store::SnapshotRetirementStep::Pending { .. } => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-rejected-over-budget"), "rejected app-typed presence disposer exceeded its exact grant")),
12337 +                     store::SnapshotRetirementFactory::Pending { .. } => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-rejected-over-budget"), "rejected app-typed presence disposer exceeded its exact grant")),
      |

```

### 3. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12338:28`. Raw line 23691.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12338:28
      |
12338 | ...   store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "rejected app-typed presence remains extern...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
12338 -                     store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "rejected app-typed presence remains externally owned" }),
12338 +                     store::SnapshotRetirementFactory::Blocked => Ok(PluginCloseStep::Blocked { reason: "rejected app-typed presence remains externally owned" }),
      |

```

### 4. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12339:28`. Raw line 23703.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12339:28
      |
12339 |                     store::SnapshotRetirementStep::Complete => {
      |                            ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
12339 -                     store::SnapshotRetirementStep::Complete => {
12339 +                     store::SnapshotRetirementFactory::Complete => {
      |

```

### 5. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12370:35`. Raw line 23715.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12370:35
      |
12370 |                 if step != store::SnapshotRetirementStep::Complete {
      |                                   ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
12370 -                 if step != store::SnapshotRetirementStep::Complete {
12370 +                 if step != store::SnapshotRetirementFactory::Complete {
      |

```

### 6. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12372:32`. Raw line 23727.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12372:32
      |
12372 | ...   store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_by...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
12372 -                         store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => PluginCloseStep::Pending { released_items, released_bytes },
12372 +                         store::SnapshotRetirementFactory::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => PluginCloseStep::Pending { released_items, released_bytes },
      |

```

### 7. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12373:32`. Raw line 23739.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12373:32
      |
12373 |                         store::SnapshotRetirementStep::Pending { .. } => {
      |                                ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
12373 -                         store::SnapshotRetirementStep::Pending { .. } => {
12373 +                         store::SnapshotRetirementFactory::Pending { .. } => {
      |

```

### 8. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12376:32`. Raw line 23751.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12376:32
      |
12376 | ...   store::SnapshotRetirementStep::Blocked => PluginCloseStep::Blocked { reason: "app-typed peer candidate cleanup remains exte...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
12376 -                         store::SnapshotRetirementStep::Blocked => PluginCloseStep::Blocked { reason: "app-typed peer candidate cleanup remains externally blocked" },
12376 +                         store::SnapshotRetirementFactory::Blocked => PluginCloseStep::Blocked { reason: "app-typed peer candidate cleanup remains externally blocked" },
      |

```

### 9. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12377:32`. Raw line 23763.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12377:32
      |
12377 |                         store::SnapshotRetirementStep::Complete => unreachable!(),
      |                                ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
12377 -                         store::SnapshotRetirementStep::Complete => unreachable!(),
12377 +                         store::SnapshotRetirementFactory::Complete => unreachable!(),
      |

```

### 10. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23834:28`. Raw line 23920.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23834:28
      |
23834 | ...   store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maxim...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
23834 -                     store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maximum_bytes => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
23834 +                     store::SnapshotRetirementFactory::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maximum_bytes => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
      |

```

### 11. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23835:28`. Raw line 23932.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23835:28
      |
23835 | ...   store::SnapshotRetirementStep::Pending { .. } => Err(plugin_sdk_fault("rejected member open exceeded its exact close grant")),
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
23835 -                     store::SnapshotRetirementStep::Pending { .. } => Err(plugin_sdk_fault("rejected member open exceeded its exact close grant")),
23835 +                     store::SnapshotRetirementFactory::Pending { .. } => Err(plugin_sdk_fault("rejected member open exceeded its exact close grant")),
      |

```

### 12. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23836:28`. Raw line 23944.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23836:28
      |
23836 | ...   store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "rejected member open is blocked" }),
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
23836 -                     store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "rejected member open is blocked" }),
23836 +                     store::SnapshotRetirementFactory::Blocked => Ok(PluginCloseStep::Blocked { reason: "rejected member open is blocked" }),
      |

```

### 13. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23837:28`. Raw line 23956.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23837:28
      |
23837 |                     store::SnapshotRetirementStep::Complete if store::MemberOpenOperation::terminal_is_empty(open) => {
      |                            ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
23837 -                     store::SnapshotRetirementStep::Complete if store::MemberOpenOperation::terminal_is_empty(open) => {
23837 +                     store::SnapshotRetirementFactory::Complete if store::MemberOpenOperation::terminal_is_empty(open) => {
      |

```

### 14. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23841:28`. Raw line 23968.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23841:28
      |
23841 | ...   store::SnapshotRetirementStep::Complete => Err(plugin_sdk_fault("rejected member open returned false terminal")),
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
23841 -                     store::SnapshotRetirementStep::Complete => Err(plugin_sdk_fault("rejected member open returned false terminal")),
23841 +                     store::SnapshotRetirementFactory::Complete => Err(plugin_sdk_fault("rejected member open returned false terminal")),
      |

```

### 15. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23910:28`. Raw line 23980.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23910:28
      |
23910 | ...   store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maxim...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
23910 -                     store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maximum_bytes => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
23910 +                     store::SnapshotRetirementFactory::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= maximum_bytes => Ok(PluginCloseStep::Pending { released_items, released_bytes }),
      |

```

### 16. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23911:28`. Raw line 23992.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23911:28
      |
23911 | ...   store::SnapshotRetirementStep::Pending { .. } => Err(plugin_sdk_fault("candidate composition retirement exceeded its exact ...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
23911 -                     store::SnapshotRetirementStep::Pending { .. } => Err(plugin_sdk_fault("candidate composition retirement exceeded its exact close grant")),
23911 +                     store::SnapshotRetirementFactory::Pending { .. } => Err(plugin_sdk_fault("candidate composition retirement exceeded its exact close grant")),
      |

```

### 17. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23912:28`. Raw line 24004.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23912:28
      |
23912 | ...   store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "candidate composition retirement is blocke...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
23912 -                     store::SnapshotRetirementStep::Blocked => Ok(PluginCloseStep::Blocked { reason: "candidate composition retirement is blocked" }),
23912 +                     store::SnapshotRetirementFactory::Blocked => Ok(PluginCloseStep::Blocked { reason: "candidate composition retirement is blocked" }),
      |

```

### 18. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23913:28`. Raw line 24016.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23913:28
      |
23913 |                     store::SnapshotRetirementStep::Complete if composition.terminal_is_empty() => {
      |                            ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
23913 -                     store::SnapshotRetirementStep::Complete if composition.terminal_is_empty() => {
23913 +                     store::SnapshotRetirementFactory::Complete if composition.terminal_is_empty() => {
      |

```

### 19. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23917:28`. Raw line 24028.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23917:28
      |
23917 | ...   store::SnapshotRetirementStep::Complete => Err(plugin_sdk_fault("candidate composition retirement returned false terminal")),
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
23917 -                     store::SnapshotRetirementStep::Complete => Err(plugin_sdk_fault("candidate composition retirement returned false terminal")),
23917 +                     store::SnapshotRetirementFactory::Complete => Err(plugin_sdk_fault("candidate composition retirement returned false terminal")),
      |

```

### 20. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24482:31`. Raw line 24040.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24482:31
      |
24482 | ...   Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) => PluginCloseStep::Pending { released_items,...
      |                 ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
24482 -                     Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) => PluginCloseStep::Pending { released_items, released_bytes },
24482 +                     Ok(store::SnapshotRetirementFactory::Pending { released_items, released_bytes }) => PluginCloseStep::Pending { released_items, released_bytes },
      |

```

### 21. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24483:31`. Raw line 24052.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24483:31
      |
24483 | ...   Ok(store::SnapshotRetirementStep::Blocked) => PluginCloseStep::Blocked { reason: "recursive archive partial decoded owner r...
      |                 ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
24483 -                     Ok(store::SnapshotRetirementStep::Blocked) => PluginCloseStep::Blocked { reason: "recursive archive partial decoded owner retirement is blocked" },
24483 +                     Ok(store::SnapshotRetirementFactory::Blocked) => PluginCloseStep::Blocked { reason: "recursive archive partial decoded owner retirement is blocked" },
      |

```

### 22. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24484:31`. Raw line 24064.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24484:31
      |
24484 |                     Ok(store::SnapshotRetirementStep::Complete) if retained.terminal_is_empty() => {
      |                               ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
24484 -                     Ok(store::SnapshotRetirementStep::Complete) if retained.terminal_is_empty() => {
24484 +                     Ok(store::SnapshotRetirementFactory::Complete) if retained.terminal_is_empty() => {
      |

```

### 23. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24493:31`. Raw line 24076.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24493:31
      |
24493 | ...   Ok(store::SnapshotRetirementStep::Complete) if store::ErasedSnapshotRetirement::terminal_is_empty(hydration) => {
      |                 ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
24493 -                     Ok(store::SnapshotRetirementStep::Complete) if store::ErasedSnapshotRetirement::terminal_is_empty(hydration) => {
24493 +                     Ok(store::SnapshotRetirementFactory::Complete) if store::ErasedSnapshotRetirement::terminal_is_empty(hydration) => {
      |

```

### 24. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24497:31`. Raw line 24088.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24497:31
      |
24497 | ...   Ok(store::SnapshotRetirementStep::Complete) => PluginCloseStep::Blocked { reason: "recursive archive parent hydration retur...
      |                 ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
24497 -                     Ok(store::SnapshotRetirementStep::Complete) => PluginCloseStep::Blocked { reason: "recursive archive parent hydration returned false terminal" },
24497 +                     Ok(store::SnapshotRetirementFactory::Complete) => PluginCloseStep::Blocked { reason: "recursive archive parent hydration returned false terminal" },
      |

```

### 25. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24498:31`. Raw line 24100.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24498:31
      |
24498 | ...   Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) => PluginCloseStep::Pending { released_items,...
      |                 ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
24498 -                     Ok(store::SnapshotRetirementStep::Pending { released_items, released_bytes }) => PluginCloseStep::Pending { released_items, released_bytes },
24498 +                     Ok(store::SnapshotRetirementFactory::Pending { released_items, released_bytes }) => PluginCloseStep::Pending { released_items, released_bytes },
      |

```

### 26. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24499:31`. Raw line 24112.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24499:31
      |
24499 | ...   Ok(store::SnapshotRetirementStep::Blocked) => PluginCloseStep::Blocked { reason: "recursive archive parent hydration retire...
      |                 ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
24499 -                     Ok(store::SnapshotRetirementStep::Blocked) => PluginCloseStep::Blocked { reason: "recursive archive parent hydration retirement is blocked" },
24499 +                     Ok(store::SnapshotRetirementFactory::Blocked) => PluginCloseStep::Blocked { reason: "recursive archive parent hydration retirement is blocked" },
      |

```

### 27. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33343:24`. Raw line 24124.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33343:24
      |
33343 | ...   store::SnapshotRetirementStep::Pending { released_items, released_bytes } => return PluginCloseStep::Pending { released_ite...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
33343 -                 store::SnapshotRetirementStep::Pending { released_items, released_bytes } => return PluginCloseStep::Pending { released_items, released_bytes },
33343 +                 store::SnapshotRetirementFactory::Pending { released_items, released_bytes } => return PluginCloseStep::Pending { released_items, released_bytes },
      |

```

### 28. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33344:24`. Raw line 24136.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33344:24
      |
33344 | ...   store::SnapshotRetirementStep::Blocked => return PluginCloseStep::Blocked { reason: "composition graph retirement is extern...
      |              ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
33344 -                 store::SnapshotRetirementStep::Blocked => return PluginCloseStep::Blocked { reason: "composition graph retirement is externally blocked" },
33344 +                 store::SnapshotRetirementFactory::Blocked => return PluginCloseStep::Blocked { reason: "composition graph retirement is externally blocked" },
      |

```

### 29. E0433: cannot find `SnapshotRetirementStep` in `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33345:24`. Raw line 24148.

```text
error[E0433]: cannot find `SnapshotRetirementStep` in `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33345:24
      |
33345 |                 store::SnapshotRetirementStep::Complete => {}
      |                        ^^^^^^^^^^^^^^^^^^^^^^ could not find `SnapshotRetirementStep` in `store`
      |
help: a trait with a similar name exists
      |
33345 -                 store::SnapshotRetirementStep::Complete => {}
33345 +                 store::SnapshotRetirementFactory::Complete => {}
      |

```

### 30. E0425: cannot find function `io_run` in module `semio_framework_os_kernel::io::io_mechanism`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/🦀️.rs:568:64`. Raw line 24160.

```text
error[E0425]: cannot find function `io_run` in module `semio_framework_os_kernel::io::io_mechanism`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/🦀️.rs:568:64
    |
568 |     let outcome = semio_framework_os_kernel::io::io_mechanism::io_run(&route, payload).await.map_err(|error| fault("job.io-run", er...
    |                                                                ^^^^^^ not found in `semio_framework_os_kernel::io::io_mechanism`

```

### 31. E0425: cannot find function `artifact_retirement_box_byte_demand` in crate `store`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24470:76`. Raw line 24166.

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in crate `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24470:76
      |
24470 |             if let Some(retained) = self.retained.as_ref() { return store::artifact_retirement_box_byte_demand(retained); }
      |                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1321:1
      |
 1321 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
      | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
      |
help: a function with a similar name exists
      |
24470 -             if let Some(retained) = self.retained.as_ref() { return store::artifact_retirement_box_byte_demand(retained); }
24470 +             if let Some(retained) = self.retained.as_ref() { return store::artifact_retirement_box_demands(retained); }
      |

```

### 32. E0425: cannot find function `owned_retirement` in module `semio_framework_value::retirement`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24531:73`. Raw line 24183.

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24531:73
      |
24531 |                 self.retained = Some(semio_framework_value::retirement::owned_retirement((history, auxiliary)));
      |                                                                         ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

```

### 33. E0433: cannot find `binary` in `io`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29278:85`. Raw line 24189.

```text
error[E0433]: cannot find `binary` in `io`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29278:85
      |
29278 | ...   let inverse_payload = identity.encode(|control| ::protocol::io::binary::operation_sequence::encode(&inverse_wires, control)...
      |                                                                       ^^^^^^ could not find `binary` in `io`
      |
help: consider importing one of these modules
      |
  432 +     use crate::dsl::os_spr::io::binary::operation_sequence;
      |
  432 +     use replication::io::binary::operation_sequence;
      |
  432 +     use semio_framework_os_kernel::os_spr::io::binary::operation_sequence;
      |
help: if you import `operation_sequence`, refer to it directly
      |
29278 -                     let inverse_payload = identity.encode(|control| ::protocol::io::binary::operation_sequence::encode(&inverse_wires, control)).map_err(|error| plugin_sdk_fault(error.to_string()))?;
29278 +                     let inverse_payload = identity.encode(|control| operation_sequence::encode(&inverse_wires, control)).map_err(|error| plugin_sdk_fault(error.to_string()))?;
      |

```

### 34. E0425: cannot find function `owned_retirement` in module `semio_framework_value::retirement`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:35156:76`. Raw line 24209.

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:35156:76
      |
35156 |                         let mut close = semio_framework_value::retirement::owned_retirement(value);
      |                                                                            ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

```

### 35. E0603: function `arc_bytes` is private

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18801:148`. Raw line 24215.

```text
error[E0603]: function `arc_bytes` is private
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18801:148
      |
18801 | ...== 1 { semio_framework_value::retirement::shared::arc_bytes::<std::sync::atomic::AtomicBool>() } else { 0 };
      |                                                      ^^^^^^^^^ private function
      |
note: the function `arc_bytes` is defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🔗️shared/🦀️.rs:89:1
      |
   89 | pub(crate) fn arc_bytes<T>()->usize {shared_retirement_allocation_bytes::<T>()}
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🚪️member-open Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🌳️graph Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🌳️graph/📏️demand Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🌳️graph/📏️demand/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/♻️root Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📨️emission Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📨️emission/📦️preparation Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📨️emission/🌱️genesis Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📨️emission/🌱️genesis/📄️input Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🔎️projection Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/↩️return Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🪪️triple Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🧹️pruning Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/📄️entry Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🪟️mounted Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📚️members Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪆️child Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/📮️port Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/📮️port/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/📮️port/♻️retirement/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎟️publication Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/♻️retirement/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🎮️decision Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🎮️decision/🗑️discard Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🎟️admission Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🎟️admission/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/♻️metadata Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/♻️metadata/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🪟️mounted Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🪟️mounted/♻️frontier Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🚪️raw-allocation-close Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🚪️raw-close Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪪️operation Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪪️operation/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎚️config Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎚️config/🚫️none Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎚️config/🚫️none/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🎛️control Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🎛️control/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧾️source-epoch Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/📤️result Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/📤️result/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/📤️return Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📝️draft Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📝️draft/🚫️none Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📝️draft/🚫️none/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📮️requests Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📸️checkpoint Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📸️checkpoint/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📥️cold-pair Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️task Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️task/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/♻️wake Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/♻️wake/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testing Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testing/🧬️job-test-mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testing/🧬️job-test-mutations/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testing/🧬️job-test-mutations/🧬️mutations/➕️add-value Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testing/🧬️job-test-mutations/🧬️mutations/➕️add-value/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testing/🧬️job-test-mutations/🔺️diff Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧬️schema/🧬️job-test-mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🔀️migrate Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/💡️infer Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📊️size Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🎭️actor Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🔗️alias Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧬️schema/🧾️context Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧬️schema/⚠️receipt Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧬️schema/🎭️actor Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧬️schema/🔗️alias Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪪️identity Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️extension Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️extension/🚪️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️effect-backbone Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🔁️lifecycle Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🔁️lifecycle/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/📨️slot Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/📨️slot/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/📥️input Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/📤️checkpoint Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/📤️checkpoint/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/🐎️wasmtime Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🪶️snapshot Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🪶️snapshot/📨️metadata Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔮️oracles Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/♻️publication Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/♻️publication/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️lifecycle Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️lifecycle/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🪞️schema-parity Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️cold-pair Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🏛️ownership Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🏛️ownership/🧮️compute Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/📸️snapshot Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/📸️snapshot/🪶️sqlite Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/📸️snapshot/🪶️sqlite/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/🔺️diff Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/🪶️sqlite Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/🪶️sqlite/📸️snapshot Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/💾️binary Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/💾️binary/📸️snapshot Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/💾️binary/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/💾️binary/🔺️diff Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/📝️text Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/📝️text/📸️snapshot Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/📝️text/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🚪️io/📝️text/🔺️diff Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🧬️schema/📸️snapshot Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🧬️schema/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🗿️artifacts/🚫️snapshot-refusal/🧬️schema/🔺️diff Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧬️component-codec Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎠️activation Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎠️activation/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔔️wake Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔔️wake/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️ui-patch Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️ui-patch/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️testing Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️testing/🔗️dependency-contribution Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️testing/🔗️dependency-contribution/🔤️keywords Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️testing/🔗️dependency-contribution/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️testing/🔗️dependency-contribution/🧬️mutations/➕️add-value Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️testing/🔗️dependency-contribution/🧬️mutations/➕️add-value/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️testing/🔗️dependency-contribution/📐️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️testing/🔗️dependency-contribution/🔺️diff Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧬️schema/🔗️dependency-contribution Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧬️schema/🔗️dependency-contribution/🔤️keywords Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/♻️effects Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/⚠️fault Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/⚠️fault/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/♻️vacant Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/♻️retirement/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📤️checkpoint Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📤️checkpoint/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📥️installation Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🛂️descriptor Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🔍️freshness Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🔍️freshness/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/👁️watch Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/👁️watch/📋️plan Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🏃️execution Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📦️materialization Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/📖️body Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖱️context-menu Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎞️media-owner-context Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎞️media-owner-context/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/👥️presence Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/👥️presence/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/🫧️transient Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/🫧️transient/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/2standard Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/2standard/🌐️any Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/2standard/🌐️any/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/2standard/🌐️any/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/2standard/🌐️any/🧬️mutations/📝️set-value Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/2standard/🌐️any/🧬️mutations/📝️set-value/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🔒️strict Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🔒️strict/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🔒️strict/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🔒️strict/🧬️mutations/📝️set-value Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🔒️strict/🧬️mutations/📝️set-value/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🌐️any Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🌐️any/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🌐️any/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🌐️any/🧬️mutations/📝️set-value Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🛰️declaration-channels/1standard/🌐️any/🧬️mutations/📝️set-value/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🪟️surface Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🪟️surface/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🎲️dummy Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🎲️dummy/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🎚️config Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🎚️config/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/⚖️declared-verb-verdicts Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/⚖️declared-verb-verdicts/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📡️contributed-mutation-wire Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📡️contributed-mutation-wire/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📡️contributed-mutation-wire/🧬️mutations/➕️add-value Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📡️contributed-mutation-wire/🔺️diff Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/🧮️compute Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/📥️command-ingress Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏳️operation-progress Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏳️operation-progress/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🔐️authority Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🔐️authority/📖️inputs Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🔺️diff Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/👷️worker Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧭️intent Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🎛️command Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📮️requests Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️testing Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️testing/🌊️actor-import Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️testing/🌊️actor-import/👽️guest Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/⚡️vite Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🫙️empty-state Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🛰️declaration-channels Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/⚠️refusal Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/⚠️refusal/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🧬️schema/📨️metadata Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🧬️schema/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪆️child Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🔣️codec Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🔣️codec/🧵️send Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🧬️contract Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧊️mesh Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🖼️image Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📝️text Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization/♻️retirement/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🖥️hosts Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🏗️framework Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🗿️artifacts Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧩️composition Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧩️composition/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🖼️assets Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🖼️assets/🧩️composition Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/⭐️default Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/⭐️default/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌳️surface-scaffold Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔄️refresh Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧰️framework-catalog Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/👷️service-worker Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🗄️store Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🔍️resolution Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🧾️bytes Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/📬️mutation Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/📬️mutation/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗂️registry Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗂️registry/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preparation Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preparation/♻️custody Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preparation/🧬️schema Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🔁️document-replacement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/🧵️document Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/🧵️document/♻️retirement Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/👁️read Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/👁️read/🧪️test-view Is a directory (os error 21)
 ERROR rustc_interface::passes failed to compute checksum, omitting it from dep-info /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/👁️capture Is a directory (os error 21)
```

### 36. E0050: method `close_step` has 3 parameters but the declaration in trait `semio_framework_job::InteractiveJob::close_step` has 2

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22249:23`. Raw line 36974.

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_job::InteractiveJob::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22249:23
      |
22249 |         fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
      |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, dsl::RetainedCloneGrant) -> InteractiveJobCloseStep`
help: remove the extra parameter to match the trait
      |
22249 -         fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
22249 +         fn close_step(&mut self, maximum_items: usize) -> semio_framework_job::InteractiveJobCloseStep {
      |

```

### 37. E0050: method `close_step` has 3 parameters but the declaration in trait `semio_framework_job::InteractiveJob::close_step` has 2

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19615:23`. Raw line 36987.

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_job::InteractiveJob::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19615:23
      |
19615 |         fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
      |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, dsl::RetainedCloneGrant) -> InteractiveJobCloseStep`
help: remove the extra parameter to match the trait
      |
19615 -         fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
19615 +         fn close_step(&mut self, maximum_items: usize) -> semio_framework_job::InteractiveJobCloseStep {
      |

   Compiling semio-framework-artifact-flow-flow v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust)
   Compiling semio-framework-os-infinite v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust)
```

### 38. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📸️checkpoint/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📸️checkpoint/🦀️.rs:110:125`. Raw line 37002.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📸️checkpoint/🦀️.rs:110:125
    |
110 | ...d, &instance.app_id, crate::protocol::ActorId(instance.actor.clone())).await?;
    |                         ------------------------ ^^^^^^^^^^^^^^^^^^^^^^ expected `SharedUtf8`, found `String`
    |                         |
    |                         arguments to this struct are incorrect
    |
note: tuple struct defined here
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🆔️ids/🦀️.rs:20:12
    |
 20 | pub struct ActorId(pub semio_framework_value::SharedUtf8);
    |            ^^^^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
    |
110 |         let id = plugin_runtime::plugin_create_app_with_id(runtime, instance.id, &instance.app_id, crate::protocol::ActorId(instance.actor.clone().into())).await?;
    |                                                                                                                                                   +++++++

```

### 39. E0061: this function takes 5 arguments but 4 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📸️checkpoint/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📸️checkpoint/🦀️.rs:110:18`. Raw line 37020.

```text
error[E0061]: this function takes 5 arguments but 4 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📸️checkpoint/🦀️.rs:110:18
      |
  110 | ... = plugin_runtime::plugin_create_app_with_id(runtime, instance.id, &instance.app_id, crate::protocol::ActorId(instance.actor.clone())).aw...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------------------------------------------------------------------------------------------ argument #5 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: function defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:40903:18
      |
40903 | ...fn plugin_create_app_with_id<PA: PluginApp>(runtime: &PluginRuntime<PA>, id: u32, app_id: &str, actor: protocol::ActorId, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^                                                                                              -------------------------------------------------------------------------------------------------------------------
help: provide the argument
      |
  110 |         let id = plugin_runtime::plugin_create_app_with_id(runtime, instance.id, &instance.app_id, crate::protocol::ActorId(instance.actor.clone()), /* &mut EntityIdentityAuthority<'_> */).await?;
      |                                                                                                                                                    ++++++++++++++++++++++++++++++++++++++++

```

### 40. E0061: this function takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1029:37`. Raw line 37036.

```text
error[E0061]: this function takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:1029:37
      |
 1029 | ... = crate::plugin_runtime::extension_retirement_turn(usize::from(budget.fuel > 0), if budget.fuel > 0 { budget.max_patch_bytes as usize } else { 0 })?;
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ----------------------------  ----------------------------------------------------------------- unexpected argument #2 of type `usize`
      |                                                        |
      |                                                        expected `RetainedCloneGrant`, found `usize`
      |
note: function defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:45896:19
      |
45896 |     pub(crate) fn extension_retirement_turn(grant: RetainedCloneGrant) -> Result<bool, Fault> {
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^ -------------------------
help: remove the extra argument
      |
 1029 -     let extension_retirement_work = crate::plugin_runtime::extension_retirement_turn(usize::from(budget.fuel > 0), if budget.fuel > 0 { budget.max_patch_bytes as usize } else { 0 })?;
 1029 +     let extension_retirement_work = crate::plugin_runtime::extension_retirement_turn(/* dsl::RetainedCloneGrant */)?;
      |

```

### 41. E0061: this method takes 4 arguments but 3 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🛠️tool-machine/🦀️.rs:533:31`. Raw line 38832.

```text
error[E0061]: this method takes 4 arguments but 3 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🛠️tool-machine/🦀️.rs:533:31
      |
  533 |                 Box::pin(self.dispatch_emit(&verb, Emit::commit_transaction(transaction, mutations), meta)).await?;
      |                               ^^^^^^^^^^^^^--------------------------------------------------------------- argument #4 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28702:18
      |
28702 | ...fn dispatch_emit(&mut self, verb: &str, mut emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -...
      |       ^^^^^^^^^^^^^                                                                                                             -------------------------------------------------------------------------------------------------------------------
help: provide the argument
      |
  533 |                 Box::pin(self.dispatch_emit(&verb, Emit::commit_transaction(transaction, mutations), meta, /* &mut EntityIdentityAuthority<'_> */)).await?;
      |                                                                                                          ++++++++++++++++++++++++++++++++++++++++

```

### 42. E0061: this method takes 3 arguments but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:840:62`. Raw line 38848.

```text
error[E0061]: this method takes 3 arguments but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:840:62
      |
  840 |                 TimeTravelStoreOutput::Committed(match store.commit_finished_replay(finished, finalization).await {
      |                                                              ^^^^^^^^^^^^^^^^^^^^^^------------------------ argument #3 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:23853:18
      |
23853 |     pub async fn commit_finished_replay(&mut self, finished: EditReplayResult<P, Mutation>, finalization: HistoryFinalization, id...
      |                  ^^^^^^^^^^^^^^^^^^^^^^
help: provide the argument
      |
  840 |                 TimeTravelStoreOutput::Committed(match store.commit_finished_replay(finished, finalization, /* &mut EntityIdentityAuthority<'_> */).await {
      |                                                                                                           ++++++++++++++++++++++++++++++++++++++++

```

### 43. E0061: this method takes 1 argument but 0 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:2649:26`. Raw line 38864.

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:2649:26
      |
 2649 |                     self.deliver_base_moved().await?;
      |                          ^^^^^^^^^^^^^^^^^^-- argument #1 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27301:18
      |
27301 | ...fn deliver_base_moved(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -...
      |       ^^^^^^^^^^^^^^^^^^            -------------------------------------------------------------------------------------------------------------------
help: provide the argument
      |
 2649 |                     self.deliver_base_moved(/* &mut EntityIdentityAuthority<'_> */).await?;
      |                                             ++++++++++++++++++++++++++++++++++++++

```

### 44. E0061: this method takes 2 arguments but 1 argument was supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:2976:18`. Raw line 38880.

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:2976:18
      |
 2976 |             self.revalidate_interaction_on_document_change(meta).await?;
      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------ argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29654:18
      |
29654 | ...fn revalidate_interaction_on_document_change(&mut self, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                               -------------------------------------------------------------------------------------------------------------------
help: provide the argument
      |
 2976 |             self.revalidate_interaction_on_document_change(meta, /* &mut EntityIdentityAuthority<'_> */).await?;
      |                                                                ++++++++++++++++++++++++++++++++++++++++

```

### 45. E0061: this method takes 3 arguments but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3069:26`. Raw line 38896.

```text
error[E0061]: this method takes 3 arguments but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3069:26
      |
 3069 |                     self.deliver_host_event_to_every_window(|window_id| HostEvent::TimeTravelFrozen { window_id }, meta).await?;
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------------------------------------------------------------- argument #3 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27290:29
      |
27290 | ...fn deliver_host_event_to_every_window(&mut self, event: impl Fn(String) -> HostEvent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                                    -------------------------------------------------------------------------------------------------------------------
help: provide the argument
      |
 3069 |                     self.deliver_host_event_to_every_window(|window_id| HostEvent::TimeTravelFrozen { window_id }, meta, /* &mut EntityIdentityAuthority<'_> */).await?;
      |                                                                                                                        ++++++++++++++++++++++++++++++++++++++++

```

### 46. E0061: this method takes 3 arguments but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3755:26`. Raw line 38912.

```text
error[E0061]: this method takes 3 arguments but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3755:26
      |
 3755 |         match self.store.commit_finished_replay(result, finalization.clone()).await {
      |                          ^^^^^^^^^^^^^^^^^^^^^^------------------------------ argument #3 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:23853:18
      |
23853 |     pub async fn commit_finished_replay(&mut self, finished: EditReplayResult<P, Mutation>, finalization: HistoryFinalization, id...
      |                  ^^^^^^^^^^^^^^^^^^^^^^
help: provide the argument
      |
 3755 |         match self.store.commit_finished_replay(result, finalization.clone(), /* &mut EntityIdentityAuthority<'_> */).await {
      |                                                                             ++++++++++++++++++++++++++++++++++++++++

```

### 47. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs:120:360`. Raw line 38928.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs:120:360
    |
120 | ...otal:0,owned_bytes:0},terminal_outcome:None,terminal_seen:true,
    |                                           ^^^^ expected `JobOutcomeSlot`, found `Option<_>`
    |
    = note: expected struct `JobOutcomeSlot`
                 found enum `std::option::Option<_>`

```

### 48. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs:123:63`. Raw line 38937.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs:123:63
    |
123 | ...me(std::sync::Arc::clone(&self.child_content_root)),captured_child_content_generation:self.child_content_generation,
    |       --------------------- ^^^^^^^^^^^^^^^^^^^^^^^^ expected `&Arc<ChildContentView>`, found `&ManuallyDrop<ChildContentView>`
    |       |
    |       arguments to this function are incorrect
    |
    = note: expected reference `&std::sync::Arc<component::app::ChildContentView>`
               found reference `&std::mem::ManuallyDrop<component::app::ChildContentView>`
note: method defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/clone.rs:236:8
    |
236 |     fn clone(&self) -> Self;
    |        ^^^^^

```

### 49. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27303:44`. Raw line 38954.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27303:44
      |
27303 |             let meta = ActionMeta { actor: self.store.local_actor_id().0.clone(), instance_id, view_state: None };
      |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `String`, found `SharedUtf8`
      |
help: try using a conversion method
      |
27303 -             let meta = ActionMeta { actor: self.store.local_actor_id().0.clone(), instance_id, view_state: None };
27303 +             let meta = ActionMeta { actor: self.store.local_actor_id().0.to_string(), instance_id, view_state: None };
      |

```

### 50. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28608:118`. Raw line 38966.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28608:118
      |
28608 | ..._id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone()));
      |                                   ------- ^^^^^^^^^^^^^^^^^^ expected `SharedUtf8`, found `String`
      |                                   |
      |                                   arguments to this struct are incorrect
      |
note: tuple struct defined here
     --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🆔️ids/🦀️.rs:20:12
      |
   20 | pub struct ActorId(pub semio_framework_value::SharedUtf8);
      |            ^^^^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
      |
28608 |                     let author = entry.and_then(|entry_meta| entry_meta.author_id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone().into()));
      |                                                                                                                                        +++++++

```

### 51. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29223:54`. Raw line 38984.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29223:54
      |
29223 |             let group_meta = GroupMeta { actor: Some(meta.actor.clone()), group_id, transaction };
      |                                                 ---- ^^^^^^^^^^^^^^^^^^ expected `SharedUtf8`, found `String`
      |                                                 |
      |                                                 arguments to this enum variant are incorrect
      |
help: the type constructed contains `std::string::String` due to the type of the argument passed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29223:49
      |
29223 |             let group_meta = GroupMeta { actor: Some(meta.actor.clone()), group_id, transaction };
      |                                                 ^^^^^------------------^
      |                                                      |
      |                                                      this argument influences the type of `Some`
note: tuple variant defined here
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
      |
  606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
      |
29223 |             let group_meta = GroupMeta { actor: Some(meta.actor.clone().into()), group_id, transaction };
      |                                                                        +++++++

```

### 52. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29286:122`. Raw line 39009.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29286:122
      |
29286 | ..._id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone()));
      |                                   ------- ^^^^^^^^^^^^^^^^^^ expected `SharedUtf8`, found `String`
      |                                   |
      |                                   arguments to this struct are incorrect
      |
note: tuple struct defined here
     --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🆔️ids/🦀️.rs:20:12
      |
   20 | pub struct ActorId(pub semio_framework_value::SharedUtf8);
      |            ^^^^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
      |
29286 |                         let author = entry.and_then(|entry_meta| entry_meta.author_id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone().into()));
      |                                                                                                                                            +++++++

```

### 53. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29331:37`. Raw line 39027.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29331:37
      |
29331 |                     author: ActorId(meta.actor.clone()),
      |                             ------- ^^^^^^^^^^^^^^^^^^ expected `SharedUtf8`, found `String`
      |                             |
      |                             arguments to this struct are incorrect
      |
note: tuple struct defined here
     --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🆔️ids/🦀️.rs:20:12
      |
   20 | pub struct ActorId(pub semio_framework_value::SharedUtf8);
      |            ^^^^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
      |
29331 |                     author: ActorId(meta.actor.clone().into()),
      |                                                       +++++++

```

### 54. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30627:89`. Raw line 39045.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30627:89
      |
30625 | ...    let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cance...
      |                           -------------------------------------------------------------------- the found closure
30626 | ...    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resum...
30627 | ...    let result = match self.run_framework_reserved_commit_unit_admitted(commit, &mut identity).await {
      |                                -------------------------------------------         ^^^^^^^^^^^^^ expected `&mut EntityIdentityAuthority<'_>`, found `&mut _`
      |                                |
      |                                arguments to this method are incorrect
      |
      = note: expected mutable reference `&mut EntityIdentityAuthority<'_, dyn FnMut(NativeEncodeProgress) -> bool + std::marker::Send>`
                 found mutable reference `&mut EntityIdentityAuthority<'_, {closure@🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30625:32: 30625:100}>`
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30637:18
      |
30637 | ...fn run_framework_reserved_commit_unit_admitted(&mut self, commit: &mut FrameworkReservedCommit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                  -------------------------------------------------------------------------------------------------------------------

```

### 55. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30628:62`. Raw line 39064.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30628:62
      |
30625 | ...observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() &...
      |               -------------------------------------------------------------------- the found closure
...
30628 | ...utput) => self.follow_derivable_children(&mut identity).await.map(|_| output),
      |                   ------------------------- ^^^^^^^^^^^^^ expected `&mut EntityIdentityAuthority<'_>`, found `&mut _`
      |                   |
      |                   arguments to this method are incorrect
      |
      = note: expected mutable reference `&mut EntityIdentityAuthority<'_, dyn FnMut(NativeEncodeProgress) -> bool + std::marker::Send>`
                 found mutable reference `&mut EntityIdentityAuthority<'_, {closure@🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30625:32: 30625:100}>`
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26269:18
      |
26269 | ...fn follow_derivable_children(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^            -------------------------------------------------------------------------------------------------------------------

```

### 56. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31682:87`. Raw line 39371.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31682:87
      |
31680 | ...    let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cance...
      |                           -------------------------------------------------------------------- the found closure
31681 | ...    let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resum...
31682 | ...    driver_result = self.revalidate_interaction_on_document_change(&meta, &mut identity).await;
      |                             -----------------------------------------        ^^^^^^^^^^^^^ expected `&mut EntityIdentityAuthority<'_>`, found `&mut _`
      |                             |
      |                             arguments to this method are incorrect
      |
      = note: expected mutable reference `&mut EntityIdentityAuthority<'_, dyn FnMut(NativeEncodeProgress) -> bool + std::marker::Send>`
                 found mutable reference `&mut EntityIdentityAuthority<'_, {closure@🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31680:36: 31680:104}>`
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29654:18
      |
29654 | ...fn revalidate_interaction_on_document_change(&mut self, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                               -------------------------------------------------------------------------------------------------------------------

```

### 57. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31722:91`. Raw line 39534.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31722:91
      |
31720 | ...  let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancell...
      |                         -------------------------------------------------------------------- the found closure
31721 | ...  let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(...
31722 | ...  let result = match self.publish_mounted_typed_operation_run_admitted(mounted, &mut identity).await {
      |                              --------------------------------------------          ^^^^^^^^^^^^^ expected `&mut EntityIdentityAuthority<'_>`, found `&mut _`
      |                              |
      |                              arguments to this method are incorrect
      |
      = note: expected mutable reference `&mut EntityIdentityAuthority<'_, dyn FnMut(NativeEncodeProgress) -> bool + std::marker::Send>`
                 found mutable reference `&mut EntityIdentityAuthority<'_, {closure@🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31720:32: 31720:100}>`
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31732:18
      |
31732 | ...fn publish_mounted_typed_operation_run_admitted(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                               -------------------------------------------------------------------------------------------------------------------

```

### 58. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31723:58`. Raw line 39553.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31723:58
      |
31720 | ...   let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancel...
      |                          -------------------------------------------------------------------- the found closure
...
31723 | ...       Ok(()) => self.follow_derivable_children(&mut identity).await,
      |                          ------------------------- ^^^^^^^^^^^^^ expected `&mut EntityIdentityAuthority<'_>`, found `&mut _`
      |                          |
      |                          arguments to this method are incorrect
      |
      = note: expected mutable reference `&mut EntityIdentityAuthority<'_, dyn FnMut(NativeEncodeProgress) -> bool + std::marker::Send>`
                 found mutable reference `&mut EntityIdentityAuthority<'_, {closure@🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31720:32: 31720:100}>`
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26269:18
      |
26269 | ...fn follow_derivable_children(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^            -------------------------------------------------------------------------------------------------------------------

```

### 59. E0609: no field `0` on type `semio_framework_geometry::Point`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1083:100`. Raw line 39572.

```text
error[E0609]: no field `0` on type `semio_framework_geometry::Point`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1083:100
     |
1083 |         self.interaction = InteractionMode::DragNodes { primary_id, offset: point - primary.center.0 };
     |                                                                                                    ^ unknown field
     |
help: a field with a similar name exists
     |
1083 -         self.interaction = InteractionMode::DragNodes { primary_id, offset: point - primary.center.0 };
1083 +         self.interaction = InteractionMode::DragNodes { primary_id, offset: point - primary.center.x };
     |

```

### 60. E0700: hidden type for `impl Future<Output = Result<InvocationResult, Fault>>` captures lifetime that does not appear in bounds

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33122:13`. Raw line 39584.

```text
error[E0700]: hidden type for `impl Future<Output = Result<InvocationResult, Fault>>` captures lifetime that does not appear in bounds
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33122:13
      |
33120 |           pub fn dispatch_typed<'a, 'b>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> impl Future<Output = Result<InvocationResult, Fault>> + 'a {
      |                                     --                                                                                                                                                                                      ---------------------------------------------------------- opaque type defined here
      |                                     |
      |                                     hidden type `{async block@🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33122:13: 33122:23}` captures the lifetime `'b` as defined here
33121 |               let command = Box::new(command);
33122 | /             async move {
33123 | |                 let log_generation_before = self.log_generation;
33124 | |                 let verb = A::command_id(command.as_ref()).await;
33125 | |                 let payload = ::protocol::OpBinary::encode_op(command.as_ref()).map_err(|error| error.into_fault())?;
...     |
33128 | |                 Ok(self.finish_recorded(log_generation_before, verb, result).await)
33129 | |             }
      | |_____________^
      |
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACTIO/🗑️generated/f/o/semio_framework_plugin-304318389d874f5e.long-type-5616320574671290979.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: add a `use<...>` bound to explicitly capture `'b`
      |
33120 |         pub fn dispatch_typed<'a, 'b>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> impl Future<Output = Result<InvocationResult, Fault>> + 'a + use<'a, 'b, A, M> {
      |                                                                                                                                                                                                                                                                                      +++++++++++++++++++

```

### 61. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33766:55`. Raw line 39608.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33766:55
      |
33766 |             assert_eq!(self.store.local_actor_id().0, actor, "opening actor must own the document store from construction");
      |                                                       ^^^^^ expected `SharedUtf8`, found `&str`
      |
help: call `Into::into` on this expression to convert `&str` into `SharedUtf8`
      |
33766 |             assert_eq!(self.store.local_actor_id().0, actor.into(), "opening actor must own the document store from construction");
      |                                                            +++++++

```

### 62. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33767:62`. Raw line 39619.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33767:62
      |
33767 |             assert_eq!(self.config_store.local_actor_id().0, actor, "opening actor must own the config store from construction");
      |                                                              ^^^^^ expected `SharedUtf8`, found `&str`
      |
help: call `Into::into` on this expression to convert `&str` into `SharedUtf8`
      |
33767 |             assert_eq!(self.config_store.local_actor_id().0, actor.into(), "opening actor must own the config store from construction");
      |                                                                   +++++++

```

### 63. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33768:61`. Raw line 39630.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33768:61
      |
33768 |             assert_eq!(self.draft_store.local_actor_id().0, actor, "opening actor must own the draft store from construction");
      |                                                             ^^^^^ expected `SharedUtf8`, found `&str`
      |
help: call `Into::into` on this expression to convert `&str` into `SharedUtf8`
      |
33768 |             assert_eq!(self.draft_store.local_actor_id().0, actor.into(), "opening actor must own the draft store from construction");
      |                                                                  +++++++

```

### 64. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1083:77`. Raw line 39641.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1083:77
     |
1083 |         self.interaction = InteractionMode::DragNodes { primary_id, offset: point - primary.center.0 };
     |                                                                             ^^^^^^^^^^^^^^^^^^^^^^^^ expected `Vec2`, found `Point`

```

### 65. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33769:67`. Raw line 39647.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33769:67
      |
33769 | ...   assert_eq!(self.interaction_store.local_actor_id().0, actor, "opening actor must own the interaction store from constructio...
      |                                                             ^^^^^ expected `SharedUtf8`, found `&str`
      |
help: call `Into::into` on this expression to convert `&str` into `SharedUtf8`
      |
33769 |             assert_eq!(self.interaction_store.local_actor_id().0, actor.into(), "opening actor must own the interaction store from construction");
      |                                                                        +++++++

```

### 66. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33770:69`. Raw line 39658.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33770:69
      |
33770 | ...   assert_eq!(self.window_config_store.local_actor_id().0, actor, "opening actor must own the window config registry from cons...
      |                                                               ^^^^^ expected `SharedUtf8`, found `&str`
      |
help: call `Into::into` on this expression to convert `&str` into `SharedUtf8`
      |
33770 |             assert_eq!(self.window_config_store.local_actor_id().0, actor.into(), "opening actor must own the window config registry from construction");
      |                                                                          +++++++

```

### 67. E0609: no field `0` on type `semio_framework_geometry::Point`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1108:110`. Raw line 39669.

```text
error[E0609]: no field `0` on type `semio_framework_geometry::Point`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1108:110
     |
1108 |             self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.0 };
     |                                                                                                              ^ unknown field
     |
help: a field with a similar name exists
     |
1108 -             self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.0 };
1108 +             self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.x };
     |

```

### 68. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1108:90`. Raw line 39681.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1108:90
     |
1108 |             self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.0 };
     |                                                                                          ^^^^^^^^^^^^^^^^^^^^^ expected `Vec2`, found `Point`

```

### 69. E0609: no field `0` on type `semio_framework_geometry::Point`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1110:97`. Raw line 39687.

```text
error[E0609]: no field `0` on type `semio_framework_geometry::Point`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1110:97
     |
1110 |             self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.0 };
     |                                                                                                 ^ unknown field
     |
help: a field with a similar name exists
     |
1110 -             self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.0 };
1110 +             self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.x };
     |

```

### 70. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1110:77`. Raw line 39699.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1110:77
     |
1110 |             self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.0 };
     |                                                                             ^^^^^^^^^^^^^^^^^^^^^ expected `Vec2`, found `Point`

```

### 71. E0609: no field `0` on type `semio_framework_geometry::Point`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1137:118`. Raw line 39705.

```text
error[E0609]: no field `0` on type `semio_framework_geometry::Point`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1137:118
     |
1137 |                     self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.0 };
     |                                                                                                                      ^ unknown field
     |
help: a field with a similar name exists
     |
1137 -                     self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.0 };
1137 +                     self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.x };
     |

```

### 72. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1137:98`. Raw line 39717.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1137:98
     |
1137 |                     self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.0 };
     |                                                                                                  ^^^^^^^^^^^^^^^^^^^^^ expected `Vec2`, found `Point`

```

### 73. E0609: no field `0` on type `semio_framework_geometry::Point`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1139:105`. Raw line 39723.

```text
error[E0609]: no field `0` on type `semio_framework_geometry::Point`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1139:105
     |
1139 |                     self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.0 };
     |                                                                                                         ^ unknown field
     |
help: a field with a similar name exists
     |
1139 -                     self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.0 };
1139 +                     self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.x };
     |

```

### 74. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1139:85`. Raw line 39735.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1139:85
     |
1139 |                     self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.0 };
     |                                                                                     ^^^^^^^^^^^^^^^^^^^^^ expected `Vec2`, found `Point`

```

### 75. E0609: no field `0` on type `semio_framework_geometry::Point`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1192:130`. Raw line 39741.

```text
error[E0609]: no field `0` on type `semio_framework_geometry::Point`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1192:130
     |
1192 | ...                   self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.0 };
     |                                                                                                                        ^ unknown field
     |
help: a field with a similar name exists
     |
1192 -                                 self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.0 };
1192 +                                 self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.x };
     |

```

### 76. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1192:110`. Raw line 39753.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1192:110
     |
1192 | ...                   self.interaction = InteractionMode::DragNodes { primary_id: node_id, offset: point - node.center.0 };
     |                                                                                                    ^^^^^^^^^^^^^^^^^^^^^ expected `Vec2`, found `Point`

```

### 77. E0609: no field `0` on type `semio_framework_geometry::Point`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1194:117`. Raw line 39759.

```text
error[E0609]: no field `0` on type `semio_framework_geometry::Point`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1194:117
     |
1194 | ...                   self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.0 };
     |                                                                                                           ^ unknown field
     |
help: a field with a similar name exists
     |
1194 -                                 self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.0 };
1194 +                                 self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.x };
     |

```

### 78. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1194:97`. Raw line 39771.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/./../../🎲️board/🦀️.rs:1194:97
     |
1194 | ...                   self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center.0 };
     |                                                                                       ^^^^^^^^^^^^^^^^^^^^^ expected `Vec2`, found `Point`

[cargo:build] running elapsedMs=60023
```

### 79. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:35192:79`. Raw line 39778.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:35192:79
      |
35192 | ...   self.dispatch_import_media(port, media, &ActionMeta { actor, instance_id, view_state: None }).await.map(|_| MediaConsumptio...
      |                                                             ^^^^^ expected `String`, found `SharedUtf8`
      |
help: try using a conversion method
      |
35192 |                         self.dispatch_import_media(port, media, &ActionMeta { actor: actor.to_string(), instance_id, view_state: None }).await.map(|_| MediaConsumption::Applied).map_err(MediaArtifactError::Import)
      |                                                                               ++++++      ++++++++++++

```

### 80. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:35207:67`. Raw line 39789.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:35207:67
      |
35207 |             self.dispatch_import_media(port, media, &ActionMeta { actor, instance_id, view_state: None })
      |                                                                   ^^^^^ expected `String`, found `SharedUtf8`
      |
help: try using a conversion method
      |
35207 |             self.dispatch_import_media(port, media, &ActionMeta { actor: actor.to_string(), instance_id, view_state: None })
      |                                                                   ++++++      ++++++++++++

```

### 81. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:40885:68`. Raw line 39800.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:40885:68
      |
40885 | ...   let mut app = program.create_app(app_id, protocol::ActorId(actor.to_string()), runtime.mounted_policy, identity).ok_or_else...
      |                                                ----------------- ^^^^^^^^^^^^^^^^^ expected `SharedUtf8`, found `String`
      |                                                |
      |                                                arguments to this struct are incorrect
      |
note: tuple struct defined here
     --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🆔️ids/🦀️.rs:20:12
      |
   20 | pub struct ActorId(pub semio_framework_value::SharedUtf8);
      |            ^^^^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
      |
40885 |         let mut app = program.create_app(app_id, protocol::ActorId(actor.to_string().into()), runtime.mounted_policy, identity).ok_or_else(|| plugin_internal_fault("unknown app"))?;
      |                                                                                     +++++++

```

### 82. E0560: struct `ReactorTaskSlot` has no field named `wake`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🧵️executor/🦀️.rs:133:79`. Raw line 39849.

```text
error[E0560]: struct `ReactorTaskSlot` has no field named `wake`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🧵️executor/🦀️.rs:133:79
    |
133 | ...   slots.extend((0..LOCAL_EXECUTOR_TASK_SLOTS).map(|_| ReactorTaskSlot { wake: None, generation: 0, instance: 0, operation: 0, a...
    |                                                                             ^^^^ `ReactorTaskSlot` does not have this field
    |
    = note: all struct fields are already assigned

```

### 83. E0560: struct `ReactorTaskSlot` has no field named `wake`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🧵️executor/🦀️.rs:153:48`. Raw line 39857.

```text
error[E0560]: struct `ReactorTaskSlot` has no field named `wake`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🧵️executor/🦀️.rs:153:48
    |
153 | ...   inner.slots[index] = ReactorTaskSlot { wake: Some(Arc::new(ExecutorWakeSignal::new())), generation, instance, operation, auth...
    |                                              ^^^^ `ReactorTaskSlot` does not have this field
    |
    = note: all struct fields are already assigned

```

### 84. E0061: this function takes 3 arguments but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:553:93`. Raw line 39865.

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:553:93
    |
553 | ...on(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequen...
    |                         ^^^^^^^^^^^^^^^------------- argument #3 of type `dsl::RetainedCloneGrant` is missing
    |
note: associated function defined here
   --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:394:12
    |
394 |     pub fn new(fuel: u64, deadline_us: u64, retained:RetainedCloneGrant) -> StepBudget {
    |            ^^^
help: provide the argument
    |
553 |             let mut context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX, /* dsl::RetainedCloneGrant */), slot.cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
    |                                                                                                                        +++++++++++++++++++++++++++++++

```

### 85. E0061: this function takes 7 arguments but 6 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:553:31`. Raw line 39881.

```text
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:553:31
    |
553 | ... = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
    |       ^^^^^^^^^^^^^^^^------------------------------------------------------------------------------------------------------------------------------------------------------------ argument #7 of type `&mut dsl::RetainedCloneProgress` is missing
    |
note: associated function defined here
   --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:925:12
    |
925 |     pub fn new(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>...
    |            ^^^
help: provide the argument
    |
553 |             let mut context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, /* &mut dsl::RetainedCloneProgress */);
    |                                                                                                                                                                                                          +++++++++++++++++++++++++++++++++++++++

```

### 86. E0061: this function takes 3 arguments but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1251:89`. Raw line 39915.

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1251:89
     |
1251 | ...on(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut slot.preview_...
     |                         ^^^^^^^^^^^^^^^------------- argument #3 of type `dsl::RetainedCloneGrant` is missing
     |
note: associated function defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:394:12
     |
 394 |     pub fn new(fuel: u64, deadline_us: u64, retained:RetainedCloneGrant) -> StepBudget {
     |            ^^^
help: provide the argument
     |
1251 |         let mut context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX, /* dsl::RetainedCloneGrant */), slot.cancel.clone(), semio_framework_job::default_now_us, &mut slot.preview_sequence);
     |                                                                                                                    +++++++++++++++++++++++++++++++

```

### 87. E0061: this function takes 7 arguments but 6 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1251:27`. Raw line 39931.

```text
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1251:27
     |
1251 | ... = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut slot.preview_sequence);
     |       ^^^^^^^^^^^^^^^^----------------------------------------------------------------------------------------------------------------------------------------------------------------- argument #7 of type `&mut dsl::RetainedCloneProgress` is missing
     |
note: associated function defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:925:12
     |
 925 |     pub fn new(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64...
     |            ^^^
help: provide the argument
     |
1251 |         let mut context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut slot.preview_sequence, /* &mut dsl::RetainedCloneProgress */);
     |                                                                                                                                                                                                           +++++++++++++++++++++++++++++++++++++++

```

### 88. E0061: this function takes 3 arguments but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1276:85`. Raw line 39947.

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1276:85
     |
1276 | ...on(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut slot.preview_...
     |                         ^^^^^^^^^^^^^^^------------- argument #3 of type `dsl::RetainedCloneGrant` is missing
     |
note: associated function defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:394:12
     |
 394 |     pub fn new(fuel: u64, deadline_us: u64, retained:RetainedCloneGrant) -> StepBudget {
     |            ^^^
help: provide the argument
     |
1276 |     let mut context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX, /* dsl::RetainedCloneGrant */), slot.cancel.clone(), semio_framework_job::default_now_us, &mut slot.preview_sequence);
     |                                                                                                                +++++++++++++++++++++++++++++++

```

### 89. E0061: this function takes 7 arguments but 6 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1276:23`. Raw line 39963.

```text
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1276:23
     |
1276 | ... = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut slot.preview_sequence);
     |       ^^^^^^^^^^^^^^^^----------------------------------------------------------------------------------------------------------------------------------------------------------------- argument #7 of type `&mut dsl::RetainedCloneProgress` is missing
     |
note: associated function defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:925:12
     |
 925 |     pub fn new(operation: OperationId, generation: Generation, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64...
     |            ^^^
help: provide the argument
     |
1276 |     let mut context = StepContext::new(slot.operation, Generation(slot.generation), StepBudget::new(1, u64::MAX), slot.cancel.clone(), semio_framework_job::default_now_us, &mut slot.preview_sequence, /* &mut dsl::RetainedCloneProgress */);
     |                                                                                                                                                                                                       +++++++++++++++++++++++++++++++++++++++

```

### 90. E0432: unresolved import `semio_framework_trace`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7633:9`. Raw line 39979.

```text
error[E0432]: unresolved import `semio_framework_trace`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7633:9
     |
7633 |     use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
     |         ^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_trace`
     |
help: there is a crate or module with a similar name
     |
7633 -     use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
7633 +     use semio_framework_hash::observe_heap_allocations_on_this_thread as observe;
     |

```

### 91. E0432: unresolved import `semio_framework_trace`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7681:9`. Raw line 39991.

```text
error[E0432]: unresolved import `semio_framework_trace`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7681:9
     |
7681 |     use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
     |         ^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_trace`
     |
help: there is a crate or module with a similar name
     |
7681 -     use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
7681 +     use semio_framework_hash::observe_heap_allocations_on_this_thread as observe;
     |

```

### 92. E0433: cannot find `JobPayloadCloseStep` in `semio_framework_job`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7424:114`. Raw line 40046.

```text
error[E0433]: cannot find `JobPayloadCloseStep` in `semio_framework_job`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7424:114
     |
7424 | ...= semio_framework_job::JobPayloadCloseStep::Complete {}
     |                           ^^^^^^^^^^^^^^^^^^^ could not find `JobPayloadCloseStep` in `semio_framework_job`
     |
help: a struct with a similar name exists
     |
7424 -                 while outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) != semio_framework_job::JobPayloadCloseStep::Complete {}
7424 +                 while outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) != semio_framework_job::JobPayloadSlot::Complete {}
     |

```

### 93. E0533: expected value, found struct variant `Step::Refused`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:40986:118`. Raw line 40061.

```text
error[E0533]: expected value, found struct variant `Step::Refused`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:40986:118
      |
40986 | ...copy_bytes){Ok(demand)=>demand,Err(error)=>return Step::Refused(error.kind)};
      |                                                      ^^^^^^^^^^^^^ not a value
      |
help: you might have meant to create a new value of the struct
      |
40986 -             let demand=match self.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return Step::Refused(error.kind)};
40986 +             let demand=match self.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return Step::Refused { kind: /* value */, progress: /* value */ }};
      |

```

### 94. E0533: expected value, found struct variant `Step::Refused`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:40992:217`. Raw line 40073.

```text
error[E0533]: expected value, found struct variant `Step::Refused`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:40992:217
      |
40992 | ...e(progress))=>Step::Pending{progress},Err(error)=>Step::Refused(error.kind)};
      |                                                      ^^^^^^^^^^^^^ not a value
      |
help: you might have meant to create a new value of the struct
      |
40992 -                 return match semio_framework_job::close_step_outcome_slot(&mut self.outcome,child){Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>Step::Pending{progress},Err(error)=>Step::Refused(error.kind)};
40992 +                 return match semio_framework_job::close_step_outcome_slot(&mut self.outcome,child){Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>Step::Pending{progress},Err(error)=>Step::Refused { kind: /* value */, progress: /* value */ }};
      |

```

### 95. E0164: expected tuple struct or tuple variant, found struct variant `Worker::Refused`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41003:188`. Raw line 40085.

```text
error[E0164]: expected tuple struct or tuple variant, found struct variant `Worker::Refused`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41003:188
      |
41003 | ...s},Worker::Blocked=>Step::Blocked,Worker::Refused(kind)=>Step::Refused(kind),_=>Step::Refused(semio_framework_value::ValueRefu...
      |                                      ^^^^^^^^^^^^^^^^^^^^^ not a tuple struct or tuple variant
      |
help: add the names to match a struct variant's fields
      |
41003 -                 return match session.close_step(child){Worker::Pending{progress}|Worker::Complete{progress}if progress.fits(child)=>Step::Pending{progress},Worker::Blocked=>Step::Blocked,Worker::Refused(kind)=>Step::Refused(kind),_=>Step::Refused(semio_framework_value::ValueRefusalKind::InvariantViolated)};
41003 +                 return match session.close_step(child){Worker::Pending{progress}|Worker::Complete{progress}if progress.fits(child)=>Step::Pending{progress},Worker::Blocked=>Step::Blocked,Worker::Refused { kind, progress: _ }=>Step::Refused(kind),_=>Step::Refused(semio_framework_value::ValueRefusalKind::InvariantViolated)};
      |

```

### 96. E0533: expected value, found struct variant `Step::Refused`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41003:211`. Raw line 40097.

```text
error[E0533]: expected value, found struct variant `Step::Refused`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41003:211
      |
41003 | ...er::Blocked=>Step::Blocked,Worker::Refused(kind)=>Step::Refused(kind),_=>Step::Refused(semio_framework_value::ValueRefusalKind...
      |                                                      ^^^^^^^^^^^^^ not a value
      |
help: you might have meant to create a new value of the struct
      |
41003 -                 return match session.close_step(child){Worker::Pending{progress}|Worker::Complete{progress}if progress.fits(child)=>Step::Pending{progress},Worker::Blocked=>Step::Blocked,Worker::Refused(kind)=>Step::Refused(kind),_=>Step::Refused(semio_framework_value::ValueRefusalKind::InvariantViolated)};
41003 +                 return match session.close_step(child){Worker::Pending{progress}|Worker::Complete{progress}if progress.fits(child)=>Step::Pending{progress},Worker::Blocked=>Step::Blocked,Worker::Refused(kind)=>Step::Refused { kind: /* value */, progress: /* value */ },_=>Step::Refused(semio_framework_value::ValueRefusalKind::InvariantViolated)};
      |

```

### 97. E0533: expected value, found struct variant `Step::Refused`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41003:234`. Raw line 40109.

```text
error[E0533]: expected value, found struct variant `Step::Refused`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41003:234
      |
41003 | ...ked,Worker::Refused(kind)=>Step::Refused(kind),_=>Step::Refused(semio_framework_value::ValueRefusalKind::InvariantViolated)};
      |                                                      ^^^^^^^^^^^^^ not a value
      |
help: you might have meant to create a new value of the struct
      |
41003 -                 return match session.close_step(child){Worker::Pending{progress}|Worker::Complete{progress}if progress.fits(child)=>Step::Pending{progress},Worker::Blocked=>Step::Blocked,Worker::Refused(kind)=>Step::Refused(kind),_=>Step::Refused(semio_framework_value::ValueRefusalKind::InvariantViolated)};
41003 +                 return match session.close_step(child){Worker::Pending{progress}|Worker::Complete{progress}if progress.fits(child)=>Step::Pending{progress},Worker::Blocked=>Step::Blocked,Worker::Refused(kind)=>Step::Refused(kind),_=>Step::Refused { kind: /* value */, progress: /* value */ }};
      |

```

### 98. E0599: no method named `is_some` found for struct `JobOutcomeSlot` in the current scope

Canonical source: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🦀️.rs`. Exact compiler span: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🚪️lifetime/🦀️.rs:76:96`. Raw line 40121.

```text
error[E0599]: no method named `is_some` found for struct `JobOutcomeSlot` in the current scope
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🚪️lifetime/🦀️.rs:76:96
   |
76 | ...| pump.rejected.is_some() || pump.outcome.is_some() || pump.terminal || !pump.complete {
   |                                              ^^^^^^^ method not found in `JobOutcomeSlot`

```

### 99. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33342:36`. Raw line 40127.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33342:36
      |
33342 |             match self.composition.close_step(maximum_items.min(1), maximum_bytes) {
      |                                    ^^^^^^^^^^ --------------------  ------------- unexpected argument #2 of type `usize`
      |                                               |
      |                                               expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:30166:12
      |
30166 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
      |            ^^^^^^^^^^
help: remove the extra argument
      |
33342 -             match self.composition.close_step(maximum_items.min(1), maximum_bytes) {
33342 +             match self.composition.close_step(/* dsl::RetainedCloneGrant */) {
      |

```

### 100. E0061: this method takes 2 arguments but 3 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27374:81`. Raw line 40146.

```text
error[E0061]: this method takes 2 arguments but 3 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27374:81
      |
27374 | ... self.close_private_child_group_operation_step(group,maximum_items,maximum_bytes); }
      |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^       ------------- ------------- unexpected argument #3 of type `usize`
      |                                                         |
      |                                                         expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs:293:8
      |
  293 |     fn close_private_child_group_operation_step(&mut self,operation:u64,grant:RetainedCloneGrant)->Result<semio_framework_job::In...
      |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                         ------------------------
help: remove the extra argument
      |
27374 -                 if self.private_child_groups.get(group).is_some() { return self.close_private_child_group_operation_step(group,maximum_items,maximum_bytes); }
27374 +                 if self.private_child_groups.get(group).is_some() { return self.close_private_child_group_operation_step(group,/* dsl::RetainedCloneGrant */); }
      |

```

### 101. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27374:76`. Raw line 40165.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27374:76
      |
27372 | ...ion_id: u64, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
      |                                                                ------------------------------ expected `Result<component::app::PluginCloseStep, semio_framework_dsl::Fault>` because of return type
27373 | ...t(operation_id).and_then(|operation| operation.owned_child_group) {
27374 | ...is_some() { return self.close_private_child_group_operation_step(group,maximum_items,maximum_bytes); }
      |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<PluginCloseStep, Fault>`, found `Result<InteractiveJobCloseStep, Fault>`
      |
      = note: expected enum `Result<component::app::PluginCloseStep, _>`
                 found enum `Result<InteractiveJobCloseStep, _>`

```

### 102. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28288:25`. Raw line 40177.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28288:25
      |
28288 |                 author: edit.and_then(|edit| edit.actor.clone()),
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<String>`, found `Option<SharedUtf8>`
      |
      = note: expected enum `Option<std::string::String>`
                 found enum `Option<SharedUtf8>`

```

### 103. E0782: expected a type, found a trait

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24471:71`. Raw line 40186.

```text
error[E0782]: expected a type, found a trait
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24471:71
      |
24471 | ...   if let Some(hydration) = self.hydration.as_ref() { return store::ErasedSnapshotRetirement::next_close_byte_demand(hydration...
      |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: you can add the `dyn` keyword if you want a trait object
      |
24471 |             if let Some(hydration) = self.hydration.as_ref() { return <dyn store::ErasedSnapshotRetirement>::next_close_byte_demand(hydration); }
      |                                                                       ++++                                +
help: you may have misspelled this associated item, causing `ErasedSnapshotRetirement` to be interpreted as a type rather than a trait
      |
24471 -             if let Some(hydration) = self.hydration.as_ref() { return store::ErasedSnapshotRetirement::next_close_byte_demand(hydration); }
24471 +             if let Some(hydration) = self.hydration.as_ref() { return store::ErasedSnapshotRetirement::next_copy_byte_demand(hydration); }
      |

```

### 104. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33364:38`. Raw line 40202.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33364:38
      |
33364 |                     let step = child.close_one(1, maximum_bytes);
      |                                      ^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
      |                                                |
      |                                                expected `RetainedCloneGrant`, found integer
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14084:23
      |
14084 |         pub(crate) fn close_one(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
      |                       ^^^^^^^^^            -------------------------
help: remove the extra argument
      |
33364 -                     let step = child.close_one(1, maximum_bytes);
33364 +                     let step = child.close_one(/* dsl::RetainedCloneGrant */);
      |

```

### 105. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33365:32`. Raw line 40221.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33365:32
      |
33365 |                     if step == PluginCloseStep::Complete {
      |                        ----    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<RetainedCloneStep, ValueError>`, found `PluginCloseStep`
      |                        |
      |                        expected because this is `Result<dsl::RetainedCloneStep, semio_framework_value::ValueError>`
      |
      = note: expected enum `Result<dsl::RetainedCloneStep, semio_framework_value::ValueError>`
                 found enum `component::app::PluginCloseStep`

```

### 106. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33369:28`. Raw line 40232.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33369:28
      |
33324 |         fn close_retained_fields_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
      |                                                                                                 --------------- expected `component::app::PluginCloseStep` because of return type
...
33369 |                     return step;
      |                            ^^^^ expected `PluginCloseStep`, found `Result<RetainedCloneStep, ValueError>`
      |
      = note: expected enum `component::app::PluginCloseStep`
                 found enum `Result<dsl::RetainedCloneStep, semio_framework_value::ValueError>`

```

### 107. E0433: cannot find module or crate `semio_framework_os_infinite` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:1:349`. Raw line 40244.

```text
error[E0433]: cannot find module or crate `semio_framework_os_infinite` in this scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:1:349
   |
 1 | ...! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_infinite::b...
   |                                                                                                                                                                                                                                                                                                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_os_infinite`
...
37 | ... = physical_layout!(force_snapshot_json,&fixture.to_string(), &opts.to_string()).unwrap();
   |       ----------------------------------------------------------------------------- in this macro invocation
   |
   = note: this error originates in the macro `physical_layout` (in Nightly builds, run with -Z macro-backtrace for more info)
help: there is a crate or module with a similar name
   |
 1 - macro_rules! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(500_000_000,&mut progress);semio_framework_os_infinite::board::io::text::layout::$entry($source $(,$options)?,&mut decode,&mut work,&mut encode).map_err(|e|e.to_string())}};}
 1 + macro_rules! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_kernel::board::schema::layout::LayoutControl::new(500_000_000,&mut progress);semio_framework_os_infinite::board::io::text::layout::$entry($source $(,$options)?,&mut decode,&mut work,&mut encode).map_err(|e|e.to_string())}};}
   |
help: consider importing this struct
   |
 3 + use crate::board::schema::layout::LayoutControl;
   |

```

### 108. E0433: cannot find module or crate `semio_framework_os_infinite` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:1:447`. Raw line 40264.

```text
error[E0433]: cannot find module or crate `semio_framework_os_infinite` in this scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:1:447
   |
 1 | ...! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(500_000_000,&mut progress);semio_framework_os_infinite::b...
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_os_infinite`
...
37 | ... = physical_layout!(force_snapshot_json,&fixture.to_string(), &opts.to_string()).unwrap();
   |       ----------------------------------------------------------------------------- in this macro invocation
   |
   = note: this error originates in the macro `physical_layout` (in Nightly builds, run with -Z macro-backtrace for more info)
help: there is a crate or module with a similar name
   |
 1 - macro_rules! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(500_000_000,&mut progress);semio_framework_os_infinite::board::io::text::layout::$entry($source $(,$options)?,&mut decode,&mut work,&mut encode).map_err(|e|e.to_string())}};}
 1 + macro_rules! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(500_000_000,&mut progress);semio_framework_os_kernel::board::io::text::layout::$entry($source $(,$options)?,&mut decode,&mut work,&mut encode).map_err(|e|e.to_string())}};}
   |
help: consider importing this module
   |
 3 + use crate::io::text::layout;
   |

```

### 109. E0433: cannot find module or crate `semio_framework_os_infinite` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:1:349`. Raw line 40284.

```text
error[E0433]: cannot find module or crate `semio_framework_os_infinite` in this scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:1:349
   |
 1 | ...! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_infinite::b...
   |                                                                                                                                                                                                                                                                                                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_os_infinite`
...
65 | ... = physical_layout!(force_snapshot_json,&fixture.to_string(), &opts.to_string()).unwrap();
   |       ----------------------------------------------------------------------------- in this macro invocation
   |
   = note: this error originates in the macro `physical_layout` (in Nightly builds, run with -Z macro-backtrace for more info)
help: there is a crate or module with a similar name
   |
 1 - macro_rules! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(500_000_000,&mut progress);semio_framework_os_infinite::board::io::text::layout::$entry($source $(,$options)?,&mut decode,&mut work,&mut encode).map_err(|e|e.to_string())}};}
 1 + macro_rules! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_kernel::board::schema::layout::LayoutControl::new(500_000_000,&mut progress);semio_framework_os_infinite::board::io::text::layout::$entry($source $(,$options)?,&mut decode,&mut work,&mut encode).map_err(|e|e.to_string())}};}
   |
help: consider importing this struct
   |
 3 + use crate::board::schema::layout::LayoutControl;
   |

```

### 110. E0433: cannot find module or crate `semio_framework_os_infinite` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:1:447`. Raw line 40304.

```text
error[E0433]: cannot find module or crate `semio_framework_os_infinite` in this scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:1:447
   |
 1 | ...! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(500_000_000,&mut progress);semio_framework_os_infinite::b...
   |                                                                                                                                                                                                                                                                                                                                                                                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_os_infinite`
...
65 | ... = physical_layout!(force_snapshot_json,&fixture.to_string(), &opts.to_string()).unwrap();
   |       ----------------------------------------------------------------------------- in this macro invocation
   |
   = note: this error originates in the macro `physical_layout` (in Nightly builds, run with -Z macro-backtrace for more info)
help: there is a crate or module with a similar name
   |
 1 - macro_rules! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(500_000_000,&mut progress);semio_framework_os_infinite::board::io::text::layout::$entry($source $(,$options)?,&mut decode,&mut work,&mut encode).map_err(|e|e.to_string())}};}
 1 + macro_rules! physical_layout {($entry:ident,$source:expr $(,$options:expr)?)=>{{let mut decoding=|_|true;let mut encoding=|_|true;let mut progress=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(8*1024*1024,&mut decoding);let mut encode=semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut encoding);let mut work=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(500_000_000,&mut progress);semio_framework_os_kernel::board::io::text::layout::$entry($source $(,$options)?,&mut decode,&mut work,&mut encode).map_err(|e|e.to_string())}};}
   |
help: consider importing this module
   |
 3 + use crate::io::text::layout;
   |

```

### 111. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33434:47`. Raw line 40324.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33434:47
      |
33434 |             let registry_step = self.registry.close_step(maximum_items.min(1), maximum_bytes);
      |                                               ^^^^^^^^^^ --------------------  ------------- unexpected argument #2 of type `usize`
      |                                                          |
      |                                                          expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:16480:23
      |
16480 |         pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
      |                       ^^^^^^^^^^            -------------------------
help: remove the extra argument
      |
33434 -             let registry_step = self.registry.close_step(maximum_items.min(1), maximum_bytes);
33434 +             let registry_step = self.registry.close_step(/* dsl::RetainedCloneGrant */);
      |

```

### 112. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33435:33`. Raw line 40343.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33435:33
      |
33435 |             if registry_step != PluginCloseStep::Complete {
      |                -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<RetainedCloneStep, Fault>`, found `PluginCloseStep`
      |                |
      |                expected because this is `Result<dsl::RetainedCloneStep, semio_framework_dsl::Fault>`
      |
      = note: expected enum `Result<dsl::RetainedCloneStep, semio_framework_dsl::Fault>`
                 found enum `component::app::PluginCloseStep`

```

### 113. E0599: no method named `private_child_group_operation_close_byte_demand` found for mutable reference `&mut component::app::VcsArtifactApp<A, M>` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27421:175`. Raw line 40368.

```text
error[E0599]: no method named `private_child_group_operation_close_byte_demand` found for mutable reference `&mut component::app::VcsArtifactApp<A, M>` in the current scope
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27421:175
      |
27421 | ...ERATION_RESULT_PAGE_BYTES),|group|self.private_child_group_operation_close_byte_demand(group).map(|bytes|bytes.max(TYPED_OPERA...
      |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: there is a method `private_child_group_operation_close_demands` with a similar name, but with different arguments
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs:274:5
      |
  274 |     fn private_child_group_operation_close_demands(&self, operation:u64, body:usize) -> Result<semio_framework_value::RetirementDemand, Fault> {
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

```

### 114. E0425: cannot find function `mounted_private_child_grant` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27422:17`. Raw line 40380.

```text
error[E0425]: cannot find function `mounted_private_child_grant` in this scope
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27422:17
      |
27422 |                 mounted_private_child_grant(0,bytes)?;
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope

```

### 115. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19619:84`. Raw line 40398.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19619:84
      |
19619 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 116. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19619:103`. Raw line 40406.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19619:103
      |
19619 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 117. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19623:80`. Raw line 40414.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19623:80
      |
19623 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 118. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19623:99`. Raw line 40422.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19623:99
      |
19623 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 119. E0616: field `member` of struct `time_travel::TimeTravelLedger` is private

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27440:43`. Raw line 40430.

```text
error[E0616]: field `member` of struct `time_travel::TimeTravelLedger` is private
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27440:43
      |
27440 |             let member = self.time_travel.member.as_ref().and_then(|member| member.children.as_ref());
      |                                           ^^^^^^ private field

```

### 120. E0616: field `member` of struct `time_travel::TimeTravelLedger` is private

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27441:43`. Raw line 40436.

```text
error[E0616]: field `member` of struct `time_travel::TimeTravelLedger` is private
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27441:43
      |
27441 | ...   let review = self.time_travel.member.as_ref().and_then(|member| member.review_children.as_ref().map(|review|&review.view));
      |                                     ^^^^^^ private field

```

### 121. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19627:84`. Raw line 40562.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19627:84
      |
19627 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 122. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19627:103`. Raw line 40570.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19627:103
      |
19627 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 123. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19630:80`. Raw line 40578.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19630:80
      |
19630 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 124. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19630:99`. Raw line 40910.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19630:99
      |
19630 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 125. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21626:34`. Raw line 41242.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21626:34
      |
21626 |                 let step = child.close_one(maximum_items, maximum_bytes);
      |                                  ^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
      |                                            |
      |                                            expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14084:23
      |
14084 |         pub(crate) fn close_one(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
      |                       ^^^^^^^^^            -------------------------
help: remove the extra argument
      |
21626 -                 let step = child.close_one(maximum_items, maximum_bytes);
21626 +                 let step = child.close_one(/* dsl::RetainedCloneGrant */);
      |

```

### 126. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21627:28`. Raw line 41261.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21627:28
      |
21627 |                 if step == PluginCloseStep::Complete {
      |                    ----    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<RetainedCloneStep, ValueError>`, found `PluginCloseStep`
      |                    |
      |                    expected because this is `Result<dsl::RetainedCloneStep, semio_framework_value::ValueError>`
      |
      = note: expected enum `Result<dsl::RetainedCloneStep, semio_framework_value::ValueError>`
                 found enum `component::app::PluginCloseStep`

```

### 127. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21631:24`. Raw line 41272.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21631:24
      |
21623 |         fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
      |                                                                                 --------------- expected `component::app::PluginCloseStep` because of return type
...
21631 |                 return step;
      |                        ^^^^ expected `PluginCloseStep`, found `Result<RetainedCloneStep, ValueError>`
      |
      = note: expected enum `component::app::PluginCloseStep`
                 found enum `Result<dsl::RetainedCloneStep, semio_framework_value::ValueError>`

```

### 128. E0533: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19633:13`. Raw line 41332.

```text
error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19633:13
      |
19633 |             semio_framework_job::InteractiveJobCloseStep::Complete
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a value
      |
help: you might have meant to create a new value of the struct
      |
19633 |             semio_framework_job::InteractiveJobCloseStep::Complete { progress: /* value */ }
      |                                                                    +++++++++++++++++++++++++

```

### 129. E0533: expected value, found struct variant `Step::Refused`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21181:157`. Raw line 41787.

```text
error[E0533]: expected value, found struct variant `Step::Refused`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21181:157
      |
21181 | ...ogress}}else{Step::Pending{progress}},Err(error)=>Step::Refused(error.kind)}
      |                                                      ^^^^^^^^^^^^^ not a value
      |
help: you might have meant to create a new value of the struct
      |
21181 -             match self.close_owned_turn(grant){Ok(progress)=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Err(error)=>Step::Refused(error.kind)}
21181 +             match self.close_owned_turn(grant){Ok(progress)=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Err(error)=>Step::Refused { kind: /* value */, progress: /* value */ }}
      |

```

### 130. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21699:69`. Raw line 41799.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21699:69
      |
21699 |                 Self::Presence(publication)=>publication.close_step(grant).map_err(|error|plugin_sdk_fault(error.into_message())),
      |                                                          ---------- ^^^^^ expected `ArtifactStoreOneItemGrant`, found `RetainedCloneGrant`
      |                                                          |
      |                                                          arguments to this method are incorrect
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5175:12
      |
 5175 |     pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> where P: Send + Sync ...
      |            ^^^^^^^^^^

```

### 131. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21700:70`. Raw line 41813.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21700:70
      |
21700 |                 Self::Transient(publication)=>publication.close_step(grant).map_err(|error|plugin_sdk_fault(error.into_message())),
      |                                                           ---------- ^^^^^ expected `ArtifactStoreOneItemGrant`, found `RetainedCloneGrant`
      |                                                           |
      |                                                           arguments to this method are incorrect
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5175:12
      |
 5175 |     pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> where P: Send + Sync ...
      |            ^^^^^^^^^^

```

### 132. E0599: no method named `next_close_byte_demand` found for reference `&OwnedDocumentMemberIngress` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24472:84`. Raw line 41827.

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&OwnedDocumentMemberIngress` in the current scope
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24472:84
      |
24472 |             if let Some(ingress) = self.rejected_ingress.as_ref() { return ingress.next_close_byte_demand(); }
      |                                                                                    ^^^^^^^^^^^^^^^^^^^^^^ method not found in `&OwnedDocumentMemberIngress`
      |
      = help: items from traits can only be used if the trait is implemented and in scope
      = note: the following trait defines an item `next_close_byte_demand`, perhaps you need to implement it:
              candidate #1: `RetirementCursor`

```

### 133. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21228:38`. Raw line 41837.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21228:38
      |
21228 |                     let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
      |                                      ^^^^^^^^^^ -  ------------------------------------------- unexpected argument #2 of type `usize`
      |                                                 |
      |                                                 expected `RetainedCloneGrant`, found integer
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2822:12
      |
 2822 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
      |            ^^^^^^^^^^
help: remove the extra argument
      |
21228 -                     let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
21228 +                     let _ = rejected.close_step(/* dsl::RetainedCloneGrant */);
      |

```

### 134. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22253:84`. Raw line 41856.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22253:84
      |
22253 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 135. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22253:103`. Raw line 41864.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22253:103
      |
22253 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 136. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22257:80`. Raw line 41872.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22257:80
      |
22257 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 137. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22257:99`. Raw line 41880.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22257:99
      |
22257 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 138. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22261:84`. Raw line 41888.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22261:84
      |
22261 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 139. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22261:103`. Raw line 41896.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22261:103
      |
22261 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 140. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24481:39`. Raw line 41904.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24481:39
      |
24481 |                 return match retained.close_step(maximum_items, maximum_bytes) {
      |                                       ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
      |                                                  |
      |                                                  expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
      |
   12 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
      |        ^^^^^^^^^^
help: remove the extra argument
      |
24481 -                 return match retained.close_step(maximum_items, maximum_bytes) {
24481 +                 return match retained.close_step(/* dsl::RetainedCloneGrant */) {
      |

```

### 141. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17930:36`. Raw line 41923.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17930:36
      |
17928 |             let mut observer = |next| { observed = next; !token.is_cancelled_now() && !publication.is_cancelled() };
      |                                ------ the found closure
17929 |             let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIde...
17930 |             let result = operation(&mut identity);
      |                          --------- ^^^^^^^^^^^^^ expected `&mut EntityIdentityAuthority<'_>`, found `&mut _`
      |                          |
      |                          arguments to this function are incorrect
      |
      = note: expected mutable reference `&mut EntityIdentityAuthority<'_, dyn FnMut(NativeEncodeProgress) -> bool + std::marker::Send>`
                 found mutable reference `&mut EntityIdentityAuthority<'_, {closure@🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17928:32: 17928:38}>`
note: type parameter defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17924:41
      |
17924 | ...n: impl FnOnce(&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<T, Fault>) -...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

```

### 142. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22264:80`. Raw line 41942.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22264:80
      |
22264 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 143. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22264:99`. Raw line 41950.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22264:99
      |
22264 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 144. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22268:84`. Raw line 41958.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22268:84
      |
22268 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 145. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22268:103`. Raw line 41966.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22268:103
      |
22268 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 146. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22271:80`. Raw line 41974.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22271:80
      |
22271 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 147. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22271:99`. Raw line 41982.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22271:99
      |
22271 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 148. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22275:84`. Raw line 41990.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22275:84
      |
22275 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 149. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22275:103`. Raw line 41998.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22275:103
      |
22275 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 150. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22278:80`. Raw line 42006.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22278:80
      |
22278 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 151. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22278:99`. Raw line 42014.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22278:99
      |
22278 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 152. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22282:84`. Raw line 42022.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22282:84
      |
22282 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 153. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22282:103`. Raw line 42030.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22282:103
      |
22282 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 154. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22285:80`. Raw line 42038.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22285:80
      |
22285 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 155. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22285:99`. Raw line 42046.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22285:99
      |
22285 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 156. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22289:84`. Raw line 42054.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22289:84
      |
22289 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 157. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21260:33`. Raw line 42062.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21260:33
      |
21260 |                 let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
      |                                 ^^^^^^^^^^ -  ------------------------------------------- unexpected argument #2 of type `usize`
      |                                            |
      |                                            expected `RetainedCloneGrant`, found integer
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../♻️retirement/📄️payload/🦀️.rs:122:12
      |
  122 |     pub fn close_step(&mut self, grant: RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_dema...
      |            ^^^^^^^^^^
help: remove the extra argument
      |
21260 -                 let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
21260 +                 let _ = outcome.close_step(/* dsl::RetainedCloneGrant */);
      |

```

### 158. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22289:103`. Raw line 42081.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22289:103
      |
22289 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 159. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22292:80`. Raw line 42089.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22292:80
      |
22292 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 160. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22292:99`. Raw line 42097.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22292:99
      |
22292 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 161. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22296:84`. Raw line 42105.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22296:84
      |
22296 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 162. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22296:103`. Raw line 42113.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22296:103
      |
22296 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 163. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22299:80`. Raw line 42121.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22299:80
      |
22299 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 164. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22299:99`. Raw line 42129.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22299:99
      |
22299 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 165. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22303:84`. Raw line 42137.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22303:84
      |
22303 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 166. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22303:103`. Raw line 42145.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22303:103
      |
22303 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 167. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22306:80`. Raw line 42153.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22306:80
      |
22306 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 168. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22306:99`. Raw line 42161.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22306:99
      |
22306 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 169. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22310:84`. Raw line 42169.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22310:84
      |
22310 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 170. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22310:103`. Raw line 42177.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22310:103
      |
22310 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 171. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22313:80`. Raw line 42185.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22313:80
      |
22313 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 172. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22313:99`. Raw line 42193.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22313:99
      |
22313 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 173. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22317:84`. Raw line 42201.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22317:84
      |
22317 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 174. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22317:103`. Raw line 42209.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22317:103
      |
22317 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 175. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22320:80`. Raw line 42217.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22320:80
      |
22320 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 176. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22320:99`. Raw line 42225.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22320:99
      |
22320 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 177. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22324:84`. Raw line 42233.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22324:84
      |
22324 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 178. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22324:103`. Raw line 42241.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22324:103
      |
22324 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 179. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22327:80`. Raw line 42249.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22327:80
      |
22327 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 180. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22327:99`. Raw line 42257.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22327:99
      |
22327 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 181. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22331:84`. Raw line 42265.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22331:84
      |
22331 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 182. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22331:103`. Raw line 42273.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22331:103
      |
22331 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 183. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22334:80`. Raw line 42281.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22334:80
      |
22334 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 184. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22334:99`. Raw line 42289.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22334:99
      |
22334 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 185. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22338:84`. Raw line 42297.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22338:84
      |
22338 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 186. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22338:103`. Raw line 42305.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22338:103
      |
22338 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 187. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22341:80`. Raw line 42313.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22341:80
      |
22341 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 188. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22341:99`. Raw line 42321.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22341:99
      |
22341 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 189. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22345:84`. Raw line 42329.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22345:84
      |
22345 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 190. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22345:103`. Raw line 42337.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22345:103
      |
22345 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
      |                                                                                                       ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 191. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22348:80`. Raw line 42345.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22348:80
      |
22348 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 192. E0559: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22348:99`. Raw line 42353.

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22348:99
      |
22348 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
      |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
      |
      = note: available fields are: `progress`

```

### 193. E0061: this method takes 2 arguments but 3 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18910:33`. Raw line 42361.

```text
error[E0061]: this method takes 2 arguments but 3 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18910:33
      |
18910 |         let step = disposer_ref.close_step(owner, maximum_items, maximum_bytes)?;
      |                                 ^^^^^^^^^^        -------------  ------------- unexpected argument #3 of type `usize`
      |                                                   |
      |                                                   expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18384:12
      |
18384 |         fn close_step(&mut self, owner: &mut T, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>;
      |            ^^^^^^^^^^                           -----
help: remove the extra argument
      |
18910 -         let step = disposer_ref.close_step(owner, maximum_items, maximum_bytes)?;
18910 +         let step = disposer_ref.close_step(owner, /* dsl::RetainedCloneGrant */)?;
      |

```

### 194. E0533: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22351:13`. Raw line 42380.

```text
error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22351:13
      |
22351 |             semio_framework_job::InteractiveJobCloseStep::Complete
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a value
      |
help: you might have meant to create a new value of the struct
      |
22351 |             semio_framework_job::InteractiveJobCloseStep::Complete { progress: /* value */ }
      |                                                                    +++++++++++++++++++++++++

```

### 195. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18911:20`. Raw line 42391.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18911:20
      |
18911 |         if step != PluginCloseStep::Complete {
      |            ----    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PluginLifecycleStep`, found `PluginCloseStep`
      |            |
      |            expected because this is `PluginLifecycleStep`

```

### 196. E0599: no method named `next_close_byte_demand` found for reference `&OwnedDocumentMemberIngressRegistry` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23742:34`. Raw line 42399.

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&OwnedDocumentMemberIngressRegistry` in the current scope
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23742:34
      |
23742 |             let bytes = registry.next_close_byte_demand();
      |                                  ^^^^^^^^^^^^^^^^^^^^^^ method not found in `&OwnedDocumentMemberIngressRegistry`
      |
      = help: items from traits can only be used if the trait is implemented and in scope
      = note: the following trait defines an item `next_close_byte_demand`, perhaps you need to implement it:
              candidate #1: `RetirementCursor`

```

### 197. E0061: this function takes 2 arguments but 3 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24492:30`. Raw line 42409.

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24492:30
      |
24492 |                 return match store::ErasedSnapshotRetirement::close_step(hydration, maximum_items.min(1), maximum_bytes) {
      |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^            --------------------  ------------- unexpected argument #3 of type `usize`
      |                                                                                     |
      |                                                                                     expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
      |
   12 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
      |        ^^^^^^^^^^
help: remove the extra argument
      |
24492 -                 return match store::ErasedSnapshotRetirement::close_step(hydration, maximum_items.min(1), maximum_bytes) {
24492 +                 return match store::ErasedSnapshotRetirement::close_step(hydration, /* dsl::RetainedCloneGrant */) {
      |

```

### 198. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21274:29`. Raw line 42428.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21274:29
      |
21274 |             let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
      |                             ^^^^^^^^^^ -  ------------------------------------------- unexpected argument #2 of type `usize`
      |                                        |
      |                                        expected `RetainedCloneGrant`, found integer
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2535:12
      |
 2535 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> WorkerJobCloseStep {
      |            ^^^^^^^^^^
help: remove the extra argument
      |
21274 -             let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
21274 +             let _ = session.close_step(/* dsl::RetainedCloneGrant */);
      |

```

### 199. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18912:23`. Raw line 42447.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18912:23
      |
18912 |             return Ok(step);
      |                    -- ^^^^ expected `PluginCloseStep`, found `PluginLifecycleStep`
      |                    |
      |                    arguments to this enum variant are incorrect
      |
help: the type constructed contains `PluginLifecycleStep` due to the type of the argument passed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18912:20
      |
18912 |             return Ok(step);
      |                    ^^^----^
      |                       |
      |                       this argument influences the type of `Ok`
note: tuple variant defined here
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:561:5
      |
  561 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ^^

```

### 200. E0061: this enum variant takes 2 arguments but 1 argument was supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:94:12`. Raw line 42468.

```text
error[E0061]: this enum variant takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:94:12
      |
   94 |         Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
      |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------- argument #2 of type `dsl::RetainedCloneProgress` is missing
      |
note: tuple variant defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18278:5
      |
18278 |     Prepared(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
      |     ^^^^^^^^
help: provide the argument
      |
   94 |         Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, /* dsl::RetainedCloneProgress */))
      |                                                                                ++++++++++++++++++++++++++++++++++

```

### 201. E0277: the trait bound `M: ArtifactCanonicalJsonTree` is not satisfied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18487:9`. Raw line 42484.

```text
error[E0277]: the trait bound `M: ArtifactCanonicalJsonTree` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18487:9
      |
18487 | ...   std::sync::Arc::new(BoundedConfigPreparation...edValueRetirementFactory::<M>::default()) })
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^...^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ArtifactCanonicalJsonTree` is not implemented for `M`
      |
note: required for `BoundedConfigPreparationFactory<C, M>` to implement `ArtifactStoreOneItemPreparationFactory<C, M>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18545:16
      |
18545 |     impl<C, M> store::ArtifactStoreOneItemPreparationFactory<C, M> for BoundedConfigPreparationFactory<C, M>
      |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
18548 |         M: store::ArtifactCanonicalJsonTree + Clone + OpBinary + Mutation<C> + semio_framework_value::retirement::RetireOwned + S...
      |            -------------------------------- unsatisfied trait bound introduced here
      = note: required for the cast from `std::sync::Arc<BoundedConfigPreparationFactory<C, M>>` to `std::sync::Arc<(dyn ArtifactStoreOneItemPreparationFactory<C, M> + 'static)>`
help: consider further restricting type parameter `M` with trait `ArtifactCanonicalJsonTree`
      |
18485 |         M: Clone + OpBinary + Mutation<C> + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static + dsl::ArtifactCanonicalJsonTree,
      |                                                                                                                    ++++++++++++++++++++++++++++++++

```

### 202. E0061: this method takes 2 arguments but 3 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21371:42`. Raw line 42504.

```text
error[E0061]: this method takes 2 arguments but 3 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21371:42
      |
21371 |                 let step = self.disposer.close_step(command, maximum_items.min(1), maximum_bytes)?;
      |                                          ^^^^^^^^^^          --------------------  ------------- unexpected argument #3 of type `usize`
      |                                                              |
      |                                                              expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18384:12
      |
18384 |         fn close_step(&mut self, owner: &mut T, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>;
      |            ^^^^^^^^^^                           -----
help: remove the extra argument
      |
21371 -                 let step = self.disposer.close_step(command, maximum_items.min(1), maximum_bytes)?;
21371 +                 let step = self.disposer.close_step(command, /* dsl::RetainedCloneGrant */)?;
      |

```

### 203. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21372:28`. Raw line 42523.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21372:28
      |
21372 |                 if step == PluginCloseStep::Complete {
      |                    ----    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PluginLifecycleStep`, found `PluginCloseStep`
      |                    |
      |                    expected because this is `PluginLifecycleStep`

```

### 204. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21378:27`. Raw line 42531.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21378:27
      |
21378 |                 return Ok(step);
      |                        -- ^^^^ expected `PluginCloseStep`, found `PluginLifecycleStep`
      |                        |
      |                        arguments to this enum variant are incorrect
      |
help: the type constructed contains `PluginLifecycleStep` due to the type of the argument passed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21378:24
      |
21378 |                 return Ok(step);
      |                        ^^^----^
      |                           |
      |                           this argument influences the type of `Ok`
note: tuple variant defined here
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:561:5
      |
  561 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ^^

```

### 205. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24504:38`. Raw line 42552.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24504:38
      |
24504 |                 return match ingress.close_step(maximum_items, maximum_bytes) {
      |                                      ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
      |                                                 |
      |                                                 expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10651:23
      |
10651 |         pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
      |                       ^^^^^^^^^^            -------------------------
help: remove the extra argument
      |
24504 -                 return match ingress.close_step(maximum_items, maximum_bytes) {
24504 +                 return match ingress.close_step(/* dsl::RetainedCloneGrant */) {
      |

```

### 206. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24505:24`. Raw line 42571.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24505:24
      |
24504 |                 return match ingress.close_step(maximum_items, maximum_bytes) {
      |                              ------------------------------------------------ this expression has type `Result<dsl::RetainedCloneStep, semio_framework_dsl::Fault>`
24505 |                     Ok(PluginCloseStep::Complete) if ingress.terminal_is_empty() => {
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RetainedCloneStep`, found `PluginCloseStep`

```

### 207. E0308: `match` arms have incompatible types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24509:33`. Raw line 42579.

```text
error[E0308]: `match` arms have incompatible types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24509:33
      |
24504 |                   return match ingress.close_step(maximum_items, maximum_bytes) {
      |  ________________________-
24505 | |                     Ok(PluginCloseStep::Complete) if ingress.terminal_is_empty() => {
24506 | |                         self.rejected_ingress = None;
24507 | |                         PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
      | |                         ----------------------------------------------------------------- this is found to be of type `component::app::PluginCloseStep`
24508 | |                     }
24509 | |                     Ok(step) => step,
      | |                                 ^^^^ expected `PluginCloseStep`, found `RetainedCloneStep`
24510 | |                     Err(_) => PluginCloseStep::Blocked { reason: "recursive archive rejected member ingress could not retire" },
24511 | |                 };
      | |_________________- `match` arms have incompatible types

```

### 208. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21386:56`. Raw line 42595.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21386:56
      |
21386 |                     return Ok(match admission.raw_wire.close_step(maximum_items.min(1), maximum_bytes) {
      |                                                        ^^^^^^^^^^ --------------------  ------------- unexpected argument #2 of type `usize`
      |                                                                   |
      |                                                                   expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🎯️action-bus/🦀️.rs:182:12
      |
  182 |     pub fn close_step(&mut self,grant:semio_framework_job::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{
      |            ^^^^^^^^^^
help: remove the extra argument
      |
21386 -                     return Ok(match admission.raw_wire.close_step(maximum_items.min(1), maximum_bytes) {
21386 +                     return Ok(match admission.raw_wire.close_step(/* dsl::RetainedCloneGrant */) {
      |

```

### 209. E0026: variant `semio_framework_job::InteractiveJobCloseStep::Pending` does not have fields named `released_items`, `released_bytes`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21387:81`. Raw line 42614.

```text
error[E0026]: variant `semio_framework_job::InteractiveJobCloseStep::Pending` does not have fields named `released_items`, `released_bytes`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21387:81
      |
21387 | ...ob::InteractiveJobCloseStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released...
      |                                           ^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^ variant `semio_framework_job::InteractiveJobCloseStep::Pending` does not have these fields

```

### 210. E0027: pattern does not mention field `progress`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21387:25`. Raw line 42620.

```text
error[E0027]: pattern does not mention field `progress`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21387:25
      |
21387 | ...   semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { rele...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `progress`
      |
help: include the missing field in the pattern
      |
21387 |                         semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes, progress } => PluginCloseStep::Pending { released_items, released_bytes },
      |                                                                                                               ++++++++++
help: if you don't care about this missing field, you can explicitly ignore it
      |
21387 |                         semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes, progress: _ } => PluginCloseStep::Pending { released_items, released_bytes },
      |                                                                                                               +++++++++++++
help: or always ignore missing fields here
      |
21387 |                         semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes, .. } => PluginCloseStep::Pending { released_items, released_bytes },
      |                                                                                                               ++++

```

### 211. E0533: expected unit struct, unit variant or constant, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21389:25`. Raw line 42639.

```text
error[E0533]: expected unit struct, unit variant or constant, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21389:25
      |
21389 | ...   semio_framework_job::InteractiveJobCloseStep::Complete => PluginCloseStep::Pending { released_items: 1, released_bytes: 0 },
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a unit struct, unit variant or constant
      |
help: add the names to match a struct variant's fields
      |
21389 |                         semio_framework_job::InteractiveJobCloseStep::Complete { progress: _ } => PluginCloseStep::Pending { released_items: 1, released_bytes: 0 },
      |                                                                                +++++++++++++++

```

### 212. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14199:31`. Raw line 42650.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14199:31
      |
14199 |             match preparation.step(maximum_items,maximum_bytes)?{
      |                               ^^^^ ------------- ------------- unexpected argument #2 of type `usize`
      |                                    |
      |                                    expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📨️emission/📦️preparation/🦀️.rs:143:12
      |
  143 |     pub fn step(&mut self,grant:RetainedCloneGrant)->Result<ChildEmitPreparationStep,Fault>{self.owner.step(grant)}
      |            ^^^^           ------------------------
help: remove the extra argument
      |
14199 -             match preparation.step(maximum_items,maximum_bytes)?{
14199 +             match preparation.step(/* dsl::RetainedCloneGrant */)?{
      |

```

### 213. E0283: type annotations needed

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11602:216`. Raw line 42669.

```text
error[E0283]: type annotations needed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11602:216
      |
11602 | ...nt_supported(){return Err(plugin_sdk_fault("original child metadata has no controlled producer".into()))}
      |                              ---------------- required by a bound introduced by this call          ^^^^
      |
      = note: the type must implement `Into<std::string::String>`
note: required by a bound in `component::app::plugin_sdk_fault`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:715:39
      |
  715 |     fn plugin_sdk_fault(message: impl Into<String>) -> Fault {
      |                                       ^^^^^^^^^^^^ required by this bound in `plugin_sdk_fault`
help: try using a fully qualified path to specify the expected types
      |
11602 -                 if !<ChildContentMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(plugin_sdk_fault("original child metadata has no controlled producer".into()))}
11602 +                 if !<ChildContentMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(plugin_sdk_fault(<&str as Into<T>>::into("original child metadata has no controlled producer")))}
      |
help: consider removing this method call, as the receiver has type `&'static str` and `&'static str: Into<std::string::String>` trivially holds
      |
11602 -                 if !<ChildContentMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(plugin_sdk_fault("original child metadata has no controlled producer".into()))}
11602 +                 if !<ChildContentMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(plugin_sdk_fault("original child metadata has no controlled producer"))}
      |

```

### 214. E0599: no method named `next_close_byte_demand` found for reference `&std::boxed::Box<dsl::ArtifactStore<P, Mutation>>` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23819:220`. Raw line 42692.

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&std::boxed::Box<dsl::ArtifactStore<P, Mutation>>` in the current scope
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23819:220
      |
23819 | ... } else { store.next_close_byte_demand() });
      |                    ^^^^^^^^^^^^^^^^^^^^^^ method not found in `&std::boxed::Box<dsl::ArtifactStore<P, Mutation>>`

```

### 215. E0782: expected a type, found a trait

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23823:79`. Raw line 42698.

```text
error[E0782]: expected a type, found a trait
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23823:79
      |
23823 | ...   if let Some(open) = self.active_member_open.as_ref() { return store::MemberOpenOperation::next_close_byte_demand(open); }
      |                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: you can add the `dyn` keyword if you want a trait object
      |
23823 |                 if let Some(open) = self.active_member_open.as_ref() { return <dyn store::MemberOpenOperation>::next_close_byte_demand(open); }
      |                                                                               ++++                           +
help: you may have misspelled this associated item, causing `MemberOpenOperation` to be interpreted as a type rather than a trait
      |
23823 -                 if let Some(open) = self.active_member_open.as_ref() { return store::MemberOpenOperation::next_close_byte_demand(open); }
23823 +                 if let Some(open) = self.active_member_open.as_ref() { return store::MemberOpenOperation::next_copy_byte_demand(open); }
      |

```

### 216. E0283: type annotations needed

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11602:216`. Raw line 42714.

```text
error[E0283]: type annotations needed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11602:216
      |
11602 | ...k_fault("original child metadata has no controlled producer".into()))}
      |                                                                 ^^^^
      |
      = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `naga`, `semio_framework_diagnostic`, `zune_core`, `zune_jpeg`:
              - impl From<&'static str> for naga::front::wgsl::error::DiagnosticAttributeNotSupportedPosition;
              - impl From<&'static str> for semio_framework_dsl::FaultCode;
              - impl From<&'static str> for zune_core::bytestream::reader::ZByteIoError;
              - impl From<&'static str> for zune_jpeg::errors::DecodeErrors;
      = note: required for `&str` to implement `Into<_>`
help: try using a fully qualified path to specify the expected types
      |
11602 -                 if !<ChildContentMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(plugin_sdk_fault("original child metadata has no controlled producer".into()))}
11602 +                 if !<ChildContentMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(plugin_sdk_fault(<&str as Into<T>>::into("original child metadata has no controlled producer")))}
      |

```

### 217. E0599: no method named `next_close_byte_demand` found for mutable reference `&mut ChildEmitPreparation` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14224:29`. Raw line 42732.

```text
error[E0599]: no method named `next_close_byte_demand` found for mutable reference `&mut ChildEmitPreparation` in the current scope
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14224:29
      |
14224 |                 preparation.next_close_byte_demand().max(preparation.owner_cell_bytes().saturating_add(birth)).max(output)
      |                             ^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut ChildEmitPreparation`
      |
      = help: items from traits can only be used if the trait is implemented and in scope
      = note: the following trait defines an item `next_close_byte_demand`, perhaps you need to implement it:
              candidate #1: `RetirementCursor`

```

### 218. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14293:142`. Raw line 42742.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14293:142
      |
14293 | ...mpty(){return Ok(self.close_child_one(maximum_items,maximum_bytes));}
      |                          ^^^^^^^^^^^^^^^ ------------- ------------- unexpected argument #2 of type `usize`
      |                                          |
      |                                          expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14257:16
      |
14257 |         pub fn close_child_one(&mut self, grant: RetainedCloneGrant) -> Result<Option<PluginLifecycleStep>, Fault> {
      |                ^^^^^^^^^^^^^^^            -------------------------
help: remove the extra argument
      |
14293 -             if !self.child_preparations.is_empty()||self.child_preparations.capacity()!=0||!self.owned_child_emits.is_empty(){return Ok(self.close_child_one(maximum_items,maximum_bytes));}
14293 +             if !self.child_preparations.is_empty()||self.child_preparations.capacity()!=0||!self.owned_child_emits.is_empty(){return Ok(self.close_child_one(/* dsl::RetainedCloneGrant */));}
      |

```

### 219. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14293:137`. Raw line 42761.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14293:137
      |
14293 | ...hild_emits.is_empty(){return Ok(self.close_child_one(maximum_items,maximum_bytes));}
      |                                 -- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<PluginCloseStep>`, found `Result<Option<_>, Fault>`
      |                                 |
      |                                 arguments to this enum variant are incorrect
      |
      = note: expected enum `Option<component::app::PluginCloseStep>`
                 found enum `Result<Option<PluginLifecycleStep>, semio_framework_dsl::Fault>`
help: the type constructed contains `Result<std::option::Option<PluginLifecycleStep>, semio_framework_dsl::Fault>` due to the type of the argument passed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14293:134
      |
14293 | ..._empty(){return Ok(self.close_child_one(maximum_items,maximum_bytes));}
      |                    ^^^-------------------------------------------------^
      |                       |
      |                       this argument influences the type of `Ok`
note: tuple variant defined here
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:561:5
      |
  561 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ^^

```

### 220. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:353:497`. Raw line 42784.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:353:497
    |
353 | ....undo_policy }, dependencies, author: ActorId(author), timestamp: self.timestamp }, MutationId(undo_id), InverseMutation { targe...
    |                                          ------- ^^^^^^ expected `SharedUtf8`, found `String`
    |                                          |
    |                                          arguments to this struct are incorrect
    |
note: tuple struct defined here
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🆔️ids/🦀️.rs:20:12
    |
 20 | pub struct ActorId(pub semio_framework_value::SharedUtf8);
    |            ^^^^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
    |
353 |         Some((KernelMutation { id: MutationId(id), document: self.document, base_version: self.base_version, invocation_id: InvocationId(invocation), diff: ArtifactDiff { schema: SchemaId(schema), payload: forward }, inverse: InverseMutation { target_mutation: MutationId(target), inverse_diff: ArtifactDiff { schema: SchemaId(inverse_schema), payload: inverse }, base_version: self.base_version, dependencies: inverse_dependencies, undo_policy: self.undo_policy }, dependencies, author: ActorId(author.into()), timestamp: self.timestamp }, MutationId(undo_id), InverseMutation { target_mutation: MutationId(undo_target), inverse_diff: ArtifactDiff { schema: SchemaId(undo_schema), payload: undo_inverse }, base_version: self.base_version, dependencies: undo_dependencies, undo_policy: self.undo_policy }))
    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       +++++++

```

### 221. E0609: no field `maximum_bytes` on type `ArtifactStoreOneItemGrant`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10928:46`. Raw line 42802.

```text
error[E0609]: no field `maximum_bytes` on type `ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10928:46
      |
10928 |             if !grant.permits_one() || grant.maximum_bytes < store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return Ok(None); }
      |                                              ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
10928 -             if !grant.permits_one() || grant.maximum_bytes < store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return Ok(None); }
10928 +             if !grant.permits_one() || grant.maximum_items < store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return Ok(None); }
      |

```

### 222. E0061: this method takes 1 argument but 0 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12277:56`. Raw line 42814.

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12277:56
      |
12277 |                         *self.rejected = Some(rejected.into_retirement());
      |                                                        ^^^^^^^^^^^^^^^-- argument #1 of type `dsl::RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/🚫️rejection/🦀️.rs:31:12
      |
   31 |     pub fn into_retirement(self, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress),...
      |            ^^^^^^^^^^^^^^^
help: provide the argument
      |
12277 |                         *self.rejected = Some(rejected.into_retirement(/* dsl::RetainedCloneGrant */));
      |                                                                        +++++++++++++++++++++++++++++

```

### 223. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12277:47`. Raw line 42830.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12277:47
      |
12277 |                         *self.rejected = Some(rejected.into_retirement());
      |                                          ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                                          |
      |                                          arguments to this enum variant are incorrect
      |
      = note: expected struct `std::boxed::Box<dyn ErasedSnapshotRetirement>`
                   found enum `Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), _>`
help: the type constructed contains `Result<(std::boxed::Box<(dyn ErasedSnapshotRetirement + 'static)>, dsl::RetainedCloneProgress), (semio_framework_value::ValueError, PresencePeerAdmissionRejected<<A as component::app::ArtifactApp>::Presence>)>` due to the type of the argument passed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12277:42
      |
12277 |                         *self.rejected = Some(rejected.into_retirement());
      |                                          ^^^^^--------------------------^
      |                                               |
      |                                               this argument influences the type of `Some`
note: tuple variant defined here
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
      |
  606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ^^^^
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACTIO/🗑️generated/f/o/semio_framework_plugin-304318389d874f5e.long-type-17902842854607137983.txt'
      = note: consider using `--verbose` to print the full type name to the console

```

### 224. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:624:180`. Raw line 42855.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:624:180
    |
624 | ... row.slot = None; return Ok(group_plugin(step)); }
    |                                ------------ ^^^^ expected `PluginCloseStep`, found `RetainedCloneStep`
    |                                |
    |                                arguments to this function are incorrect
    |
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:687:4
    |
687 | fn group_plugin(step: PluginCloseStep) -> RetainedCloneStep { match step { PluginCloseStep::Pending { released_items, released_byte...
    |    ^^^^^^^^^^^^ ---------------------

```

### 225. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:633:40`. Raw line 42869.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:633:40
    |
633 |                 return Ok(group_plugin(step));
    |                           ------------ ^^^^ expected `PluginCloseStep`, found `RetainedCloneStep`
    |                           |
    |                           arguments to this function are incorrect
    |
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:687:4
    |
687 | fn group_plugin(step: PluginCloseStep) -> RetainedCloneStep { match step { PluginCloseStep::Pending { released_items, released_byte...
    |    ^^^^^^^^^^^^ ---------------------

```

### 226. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:638:36`. Raw line 42883.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:638:36
    |
638 |             return Ok(group_plugin(step));
    |                       ------------ ^^^^ expected `PluginCloseStep`, found `RetainedCloneStep`
    |                       |
    |                       arguments to this function are incorrect
    |
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:687:4
    |
687 | fn group_plugin(step: PluginCloseStep) -> RetainedCloneStep { match step { PluginCloseStep::Pending { released_items, released_byte...
    |    ^^^^^^^^^^^^ ---------------------

```

### 227. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:122:170`. Raw line 42897.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:122:170
    |
122 | ..._revision, actor: std::mem::take(&mut parts.publication_actor), group_id: parts.group_id.take(), transaction: parts.transaction....
    |                      -------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&mut SharedUtf8`, found `&mut String`
    |                      |
    |                      arguments to this function are incorrect
    |
    = note: expected mutable reference `&mut SharedUtf8`
               found mutable reference `&mut std::string::String`
note: function defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/mod.rs:888:14
    |
888 | pub const fn take<T: [const] Default>(dest: &mut T) -> T {
    |              ^^^^

```

### 228. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:39:435`. Raw line 42913.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:39:435
    |
 39 | ...e(&mut self.owner.child_id), Some(&mut self.actor.0), Some(&mut self.key.owner), Some(&mut self.key.slot), Some(&mut self.key.ch...
    |                                 ---- ^^^^^^^^^^^^^^^^^ expected `&mut String`, found `&mut SharedUtf8`
    |                                 |
    |                                 arguments to this enum variant are incorrect
    |
    = note: expected mutable reference `&mut std::string::String`
               found mutable reference `&mut SharedUtf8`
help: the type constructed contains `&mut SharedUtf8` due to the type of the argument passed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:39:430
    |
 39 | ...ome(&mut self.owner.child_id), Some(&mut self.actor.0), Some(&mut self.key.owner), Some(&mut self.key.slot), Some(&mut self.key....
    |                                   ^^^^^-----------------^
    |                                        |
    |                                        this argument influences the type of `Some`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
    |
606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^

```

### 229. E0599: no method named `next_close_byte_demand` found for reference `&OwnedDocumentMemberIngress` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23824:93`. Raw line 42936.

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&OwnedDocumentMemberIngress` in the current scope
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23824:93
      |
23824 |                 if let Some(ingress) = self.active_member_ingress.as_ref() { return ingress.next_close_byte_demand(); }
      |                                                                                             ^^^^^^^^^^^^^^^^^^^^^^ method not found in `&OwnedDocumentMemberIngress`
      |
      = help: items from traits can only be used if the trait is implemented and in scope
      = note: the following trait defines an item `next_close_byte_demand`, perhaps you need to implement it:
              candidate #1: `RetirementCursor`

```

### 230. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12335:39`. Raw line 42946.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12335:39
      |
12335 | ...   return match rejected.close_step(maximum_items, maximum_bytes).map_err(|error| plugin_sdk_fault(error.to_string()))? {
      |                             ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
      |                                        |
      |                                        expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
      |
   12 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
      |        ^^^^^^^^^^
help: remove the extra argument
      |
12335 -                 return match rejected.close_step(maximum_items, maximum_bytes).map_err(|error| plugin_sdk_fault(error.to_string()))? {
12335 +                 return match rejected.close_step(/* dsl::RetainedCloneGrant */).map_err(|error| plugin_sdk_fault(error.to_string()))? {
      |

```

### 231. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:42:395`. Raw line 42965.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:42:395
    |
 42 | ...slot), Some(&self.owner.child_id), Some(&self.actor.0), Some(&self.key.owner), Some(&self.key.slot), Some(&self.key.child_id), S...
    |                                       ---- ^^^^^^^^^^^^^ expected `&String`, found `&SharedUtf8`
    |                                       |
    |                                       arguments to this enum variant are incorrect
    |
    = note: expected reference `&std::string::String`
               found reference `&SharedUtf8`
help: the type constructed contains `&SharedUtf8` due to the type of the argument passed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:42:390
    |
 42 | ...ot), Some(&self.owner.child_id), Some(&self.actor.0), Some(&self.key.owner), Some(&self.key.slot), Some(&self.key.child_id), Som...
    |                                     ^^^^^-------------^
    |                                          |
    |                                          this argument influences the type of `Some`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
    |
606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^

```

### 232. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12369:34`. Raw line 42988.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12369:34
      |
12369 |                 let step = typed.close_step(maximum_items, maximum_bytes).map_err(|error| plugin_sdk_fault(error.to_string()))?;
      |                                  ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
      |                                             |
      |                                             expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5575:12
      |
 5575 |     pub fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_valu...
      |            ^^^^^^^^^^
help: remove the extra argument
      |
12369 -                 let step = typed.close_step(maximum_items, maximum_bytes).map_err(|error| plugin_sdk_fault(error.to_string()))?;
12369 +                 let step = typed.close_step(/* dsl::RetainedCloneGrant */).map_err(|error| plugin_sdk_fault(error.to_string()))?;
      |

```

### 233. E0277: the trait bound `MemberOpenRequest: MemberOpenOperation` is not satisfied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10644:108`. Raw line 43007.

```text
error[E0277]: the trait bound `MemberOpenRequest: MemberOpenOperation` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10644:108
      |
10644 | ...rementDemand { copy_bytes: store::MemberOpenOperation::next_copy_byte_demand(request)?, capacity_bytes: store::MemberOpenOpera...
      |                               ------------------------------------------------- ^^^^^^^ the trait `MemberOpenOperation` is not implemented for `MemberOpenRequest`
      |                               |
      |                               required by a bound introduced by this call
      |
help: the following other types implement trait `MemberOpenOperation`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:270:1
      |
  270 |   impl<M: Send> MemberOpenOperation for UnsupportedMemberFactoryOpen<M> {
      |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsupportedMemberFactoryOpen<M>`
...
  927 | / impl<F, P, M> MemberOpenOperation for InitialMemberStoreOpen<F, P, M>
  928 | | where
  929 | |     F: MemberFactory + 'static,
  930 | |     P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactCompositi...
  931 | |     M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
      | |______________________________________________________________________________________^ `InitialMemberStoreOpen<F, P, M>`

```

### 234. E0277: the trait bound `MemberOpenRequest: MemberOpenOperation` is not satisfied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10644:189`. Raw line 43028.

```text
error[E0277]: the trait bound `MemberOpenRequest: MemberOpenOperation` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10644:189
      |
10644 | ...quest)?, capacity_bytes: store::MemberOpenOperation::next_capacity_byte_demand(request, body)?, release_bytes: store::MemberOp...
      |                             ----------------------------------------------------- ^^^^^^^ the trait `MemberOpenOperation` is not implemented for `MemberOpenRequest`
      |                             |
      |                             required by a bound introduced by this call
      |
help: the following other types implement trait `MemberOpenOperation`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:270:1
      |
  270 |   impl<M: Send> MemberOpenOperation for UnsupportedMemberFactoryOpen<M> {
      |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsupportedMemberFactoryOpen<M>`
...
  927 | / impl<F, P, M> MemberOpenOperation for InitialMemberStoreOpen<F, P, M>
  928 | | where
  929 | |     F: MemberFactory + 'static,
  930 | |     P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactCompositi...
  931 | |     M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
      | |______________________________________________________________________________________^ `InitialMemberStoreOpen<F, P, M>`

```

### 235. E0061: this enum variant takes 2 arguments but 1 argument was supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:46:24`. Raw line 43049.

```text
error[E0061]: this enum variant takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:46:24
      |
   46 |                     Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.progress()))
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------- argument #2 of type `dsl::RetainedCloneProgress` is missing
      |
note: tuple variant defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18277:5
      |
18277 |     Progress(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
      |     ^^^^^^^^
help: provide the argument
      |
   46 |                     Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.progress(), /* dsl::RetainedCloneProgress */))
      |                                                                                            ++++++++++++++++++++++++++++++++++

```

### 236. E0023: this pattern has 1 field, but the corresponding tuple variant has 2 fields

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:50:92`. Raw line 43065.

```text
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:50:92
      |
   50 | ...   if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_)) { self.phase = PrivateOwnedPublicationPhase::Sta...
      |                                                                              ^ expected 2 fields, found 1
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18278:14
      |
18278 |     Prepared(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
      |              ------------------------------  --------------------- tuple variant has 2 fields
      |
help: use `_` to explicitly ignore each field
      |
   50 |                     if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { self.phase = PrivateOwnedPublicationPhase::Staging; return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.progress())); }
      |                                                                                             +++
help: use `..` to ignore all fields
      |
   50 -                     if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_)) { self.phase = PrivateOwnedPublicationPhase::Staging; return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.progress())); }
   50 +                     if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(..)) { self.phase = PrivateOwnedPublicationPhase::Staging; return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.progress())); }
      |

```

### 237. E0061: this enum variant takes 2 arguments but 1 argument was supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:50:160`. Raw line 43086.

```text
error[E0061]: this enum variant takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:50:160
      |
   50 | ...se::Staging; return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.progress())); }
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------- argument #2 of type `dsl::RetainedCloneProgress` is missing
      |
note: tuple variant defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18277:5
      |
18277 |     Progress(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
      |     ^^^^^^^^
help: provide the argument
      |
   50 |                     if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_)) { self.phase = PrivateOwnedPublicationPhase::Staging; return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.progress(), /* dsl::RetainedCloneProgress */)); }
      |                                                                                                                                                                                                                                    ++++++++++++++++++++++++++++++++++

```

### 238. E0277: the trait bound `MemberOpenRequest: MemberOpenOperation` is not satisfied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10644:274`. Raw line 43102.

```text
error[E0277]: the trait bound `MemberOpenRequest: MemberOpenOperation` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10644:274
      |
10644 | ...st, body)?, release_bytes: store::MemberOpenOperation::next_release_byte_demand(request)?, depth: store::MemberOpenOperation::...
      |                               ---------------------------------------------------- ^^^^^^^ the trait `MemberOpenOperation` is not implemented for `MemberOpenRequest`
      |                               |
      |                               required by a bound introduced by this call
      |
help: the following other types implement trait `MemberOpenOperation`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:270:1
      |
  270 |   impl<M: Send> MemberOpenOperation for UnsupportedMemberFactoryOpen<M> {
      |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsupportedMemberFactoryOpen<M>`
...
  927 | / impl<F, P, M> MemberOpenOperation for InitialMemberStoreOpen<F, P, M>
  928 | | where
  929 | |     F: MemberFactory + 'static,
  930 | |     P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactCompositi...
  931 | |     M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
      | |______________________________________________________________________________________^ `InitialMemberStoreOpen<F, P, M>`

```

### 239. E0277: the trait bound `MemberOpenRequest: MemberOpenOperation` is not satisfied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10644:338`. Raw line 43123.

```text
error[E0277]: the trait bound `MemberOpenRequest: MemberOpenOperation` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10644:338
      |
10644 | ...yte_demand(request)?, depth: store::MemberOpenOperation::next_depth_demand(request)?.checked_add(1).ok_or_else(||ValueError::l...
      |                                 --------------------------------------------- ^^^^^^^ the trait `MemberOpenOperation` is not implemented for `MemberOpenRequest`
      |                                 |
      |                                 required by a bound introduced by this call
      |
help: the following other types implement trait `MemberOpenOperation`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:270:1
      |
  270 |   impl<M: Send> MemberOpenOperation for UnsupportedMemberFactoryOpen<M> {
      |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `UnsupportedMemberFactoryOpen<M>`
...
  927 | / impl<F, P, M> MemberOpenOperation for InitialMemberStoreOpen<F, P, M>
  928 | | where
  929 | |     F: MemberFactory + 'static,
  930 | |     P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactCompositi...
  931 | |     M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
      | |______________________________________________________________________________________^ `InitialMemberStoreOpen<F, P, M>`

```

### 240. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:155:452`. Raw line 43144.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:155:452
    |
155 | ... slot, child_id: child }, actor: protocol::ActorId(actor), key: MemberKey { owner: key_owner, slot: key_slot, child_id: key_chil...
    |                                     ----------------- ^^^^^ expected `SharedUtf8`, found `String`
    |                                     |
    |                                     arguments to this struct are incorrect
    |
note: tuple struct defined here
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🆔️ids/🦀️.rs:20:12
    |
 20 | pub struct ActorId(pub semio_framework_value::SharedUtf8);
    |            ^^^^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
    |
155 |             Ok(Some(PrivateChildMemberMetadata { parts: ManuallyDrop::new(Some(PrivateChildMemberMetadataParts { expected: ArtifactRef { artifact_id: id, dialect: ArtifactDialect { artifact_kind: kind, standard, subset } }, owner: store::OwnerRef { parent: ArtifactRef { artifact_id: parent, dialect: ArtifactDialect { artifact_kind: parent_kind, standard: parent_standard, subset: parent_subset } }, slot, child_id: child }, actor: protocol::ActorId(actor.into()), key: MemberKey { owner: key_owner, slot: key_slot, child_id: key_child }, prepared_identity: PreparedChildContentIdentity { key: MemberKey { owner: prepared_owner, slot: prepared_slot, child_id: prepared_child }, reference: ArtifactRef { artifact_id: prepared_id, dialect: ArtifactDialect { artifact_kind: prepared_kind, standard: prepared_standard, subset: prepared_subset } } }, publication_actor, transaction, group_id, registry_owner: store::OwnerRef { parent: ArtifactRef { artifact_id: registry_parent, dialect: ArtifactDialect { artifact_kind: registry_kind, standard: registry_standard, subset: registry_subset } }, slot: registry_slot, child_id: registry_child } })) }))
    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                         +++++++

```

### 241. E0023: this pattern has 1 field, but the corresponding tuple variant has 2 fields

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:56:92`. Raw line 43162.

```text
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:56:92
      |
   56 | ...   if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_)) { self.phase = PrivateOwnedPublicationPhase::Sta...
      |                                                                              ^ expected 2 fields, found 1
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18278:14
      |
18278 |     Prepared(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
      |              ------------------------------  --------------------- tuple variant has 2 fields
      |
help: use `_` to explicitly ignore each field
      |
   56 |                     if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { self.phase = PrivateOwnedPublicationPhase::Staged; }
      |                                                                                             +++
help: use `..` to ignore all fields
      |
   56 -                     if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_)) { self.phase = PrivateOwnedPublicationPhase::Staged; }
   56 +                     if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(..)) { self.phase = PrivateOwnedPublicationPhase::Staged; }
      |

```

### 242. E0061: this enum variant takes 2 arguments but 1 argument was supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:57:43`. Raw line 43183.

```text
error[E0061]: this enum variant takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:57:43
      |
   57 |                     Ok(if self.staged() { store::ArtifactStoreOneItemPreparationStep::Prepared(self.progress()) } else { step })
      |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------- argument #2 of type `dsl::RetainedCloneProgress` is missing
      |
note: tuple variant defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18278:5
      |
18278 |     Prepared(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
      |     ^^^^^^^^
help: provide the argument
      |
   57 |                     Ok(if self.staged() { store::ArtifactStoreOneItemPreparationStep::Prepared(self.progress(), /* dsl::RetainedCloneProgress */) } else { step })
      |                                                                                                               ++++++++++++++++++++++++++++++++++

```

### 243. E0061: this enum variant takes 2 arguments but 1 argument was supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:59:60`. Raw line 43199.

```text
error[E0061]: this enum variant takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:59:60
      |
   59 |                 PrivateOwnedPublicationPhase::Staged => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.progress())),
      |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------- argument #2 of type `dsl::RetainedCloneProgress` is missing
      |
note: tuple variant defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18278:5
      |
18278 |     Prepared(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
      |     ^^^^^^^^
help: provide the argument
      |
   59 |                 PrivateOwnedPublicationPhase::Staged => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.progress(), /* dsl::RetainedCloneProgress */)),
      |                                                                                                                                ++++++++++++++++++++++++++++++++++

```

### 244. E0061: this enum variant takes 2 arguments but 1 argument was supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:65:43`. Raw line 43215.

```text
error[E0061]: this enum variant takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:65:43
      |
   65 |             if self.adopted() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.progress())); }
      |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------- argument #2 of type `dsl::RetainedCloneProgress` is missing
      |
note: tuple variant defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18278:5
      |
18278 |     Prepared(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
      |     ^^^^^^^^
help: provide the argument
      |
   65 |             if self.adopted() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.progress(), /* dsl::RetainedCloneProgress */)); }
      |                                                                                                               ++++++++++++++++++++++++++++++++++

```

### 245. E0023: this pattern has 1 field, but the corresponding tuple variant has 2 fields

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:68:84`. Raw line 43231.

```text
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:68:84
      |
   68 | ...   if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_)) { self.phase = PrivateOwnedPublicationPhase::Ado...
      |                                                                              ^ expected 2 fields, found 1
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18278:14
      |
18278 |     Prepared(ArtifactStoreOneItemCheckpoint, RetainedCloneProgress),
      |              ------------------------------  --------------------- tuple variant has 2 fields
      |
help: use `_` to explicitly ignore each field
      |
   68 |             if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_, _)) { self.phase = PrivateOwnedPublicationPhase::Adopted; self.grouped = false; }
      |                                                                                     +++
help: use `..` to ignore all fields
      |
   68 -             if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(_)) { self.phase = PrivateOwnedPublicationPhase::Adopted; self.grouped = false; }
   68 +             if matches!(step, store::ArtifactStoreOneItemPreparationStep::Prepared(..)) { self.phase = PrivateOwnedPublicationPhase::Adopted; self.grouped = false; }
      |

```

### 246. E0599: no method named `capacity` found for struct `SharedUtf8` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs:154:350`. Raw line 43252.

```text
error[E0599]: no method named `capacity` found for struct `SharedUtf8` in the current scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs:154:350
    |
154 | ...utation.dependencies),9=>mutation.author.0.capacity(),_=>0}}
    |                                               ^^^^^^^^ method not found in `SharedUtf8`

```

### 247. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:97:118`. Raw line 43258.

```text
error[E0308]: mismatched types
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:97:118
   |
97 | ...ivatePublicationMetadataParts { actor: request.actor, transaction: request.transaction, group_id: request.group_id }));
   |                                           ^^^^^^^^^^^^^ expected `String`, found `SharedUtf8`
   |
help: try using a conversion method
   |
97 |                 self.metadata = Some(PrivatePublicationMetadata::from_parts(PrivatePublicationMetadataParts { actor: request.actor.to_string(), transaction: request.transaction, group_id: request.group_id }));
   |                                                                                                                                   ++++++++++++

```

### 248. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs:155:423`. Raw line 43269.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs:155:423
    |
155 | ...on.dependencies),9=>Self::clear_text(&mut mutation.author.0),_=>{}}true}
    |                        ---------------- ^^^^^^^^^^^^^^^^^^^^^^ expected `&mut String`, found `&mut SharedUtf8`
    |                        |
    |                        arguments to this function are incorrect
    |
    = note: expected mutable reference `&mut std::string::String`
               found mutable reference `&mut SharedUtf8`
note: associated function defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs:149:8
    |
149 |     fn clear_text(text:&mut String){drop(std::mem::take(text));}
    |        ^^^^^^^^^^ ----------------

```

### 249. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3420:106`. Raw line 43285.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3420:106
     |
3420 | ...e.mutation_id.0.clone(), actor: envelope.actor.0.clone(), timestamp: envelope.timestamp, scope: supersede.scope, inputs: supers...
     |                                    ^^^^^^^^^^^^^^^^^^^^^^^^ expected `String`, found `SharedUtf8`
     |
help: try using a conversion method
     |
3420 -                     records.push(SupersedeRecord { transition_id: envelope.mutation_id.0.clone(), actor: envelope.actor.0.clone(), timestamp: envelope.timestamp, scope: supersede.scope, inputs: supersede.inputs, role: SupersedeRole::Edit, entry: 0 })
3420 +                     records.push(SupersedeRecord { transition_id: envelope.mutation_id.0.clone(), actor: envelope.actor.0.to_string(), timestamp: envelope.timestamp, scope: supersede.scope, inputs: supersede.inputs, role: SupersedeRole::Edit, entry: 0 })
     |

```

### 250. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3423:48`. Raw line 43297.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3423:48
     |
3423 |                     let newest = reverts.entry(envelope.actor.0.clone()).or_insert(envelope.timestamp);
     |                                          ----- ^^^^^^^^^^^^^^^^^^^^^^^^ expected `String`, found `SharedUtf8`
     |                                          |
     |                                          arguments to this method are incorrect
     |
help: the return type of this call is `SharedUtf8` due to the type of the argument passed
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:3423:34
     |
3423 |                     let newest = reverts.entry(envelope.actor.0.clone()).or_insert(envelope.timestamp);
     |                                  ^^^^^^^^^^^^^^------------------------^
     |                                                |
     |                                                this argument influences the return type of `entry`
note: method defined here
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:1013:12
     |
1013 |     pub fn entry(&mut self, key: K) -> Entry<'_, K, V, A> {
     |            ^^^^^
help: try using a conversion method
     |
3423 -                     let newest = reverts.entry(envelope.actor.0.clone()).or_insert(envelope.timestamp);
3423 +                     let newest = reverts.entry(envelope.actor.0.to_string()).or_insert(envelope.timestamp);
     |

```

### 251. E0599: no method named `next_close_byte_demand` found for reference `&component::app::ChildEmit` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:147:65`. Raw line 43323.

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&component::app::ChildEmit` in the current scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:147:65
    |
147 |         else if let Some(raw) = self.raw.as_ref() { release(raw.next_close_byte_demand()) }
    |                                                                 ^^^^^^^^^^^^^^^^^^^^^^ method not found in `&component::app::ChildEmit`
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following trait defines an item `next_close_byte_demand`, perhaps you need to implement it:
            candidate #1: `RetirementCursor`

```

### 252. E0599: no method named `next_close_byte_demand` found for reference `&OwnedDocumentMemberIngressRegistry` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23825:88`. Raw line 43333.

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&OwnedDocumentMemberIngressRegistry` in the current scope
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23825:88
      |
23825 |                 if let Some(registry) = self.member_ingress.as_ref() { return registry.next_close_byte_demand(); }
      |                                                                                        ^^^^^^^^^^^^^^^^^^^^^^ method not found in `&OwnedDocumentMemberIngressRegistry`
      |
      = help: items from traits can only be used if the trait is implemented and in scope
      = note: the following trait defines an item `next_close_byte_demand`, perhaps you need to implement it:
              candidate #1: `RetirementCursor`

```

### 253. E0061: this function takes 2 arguments but 3 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23833:30`. Raw line 43343.

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23833:30
      |
23833 | ...ch store::MemberOpenOperation::close_step(open, maximum_items.min(1), maximum_bytes).map_err(|error| plugin_sdk_fault(error.to...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^       --------------------  ------------- unexpected argument #3 of type `usize`
      |                                                    |
      |                                                    expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:105:8
      |
  105 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>;
      |        ^^^^^^^^^^
help: remove the extra argument
      |
23833 -                 return match store::MemberOpenOperation::close_step(open, maximum_items.min(1), maximum_bytes).map_err(|error| plugin_sdk_fault(error.to_string()))? {
23833 +                 return match store::MemberOpenOperation::close_step(open, /* dsl::RetainedCloneGrant */).map_err(|error| plugin_sdk_fault(error.to_string()))? {
      |

```

### 254. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23845:36`. Raw line 43362.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23845:36
      |
23845 |                 let step = ingress.close_step(maximum_items.min(1), maximum_bytes)?;
      |                                    ^^^^^^^^^^ --------------------  ------------- unexpected argument #2 of type `usize`
      |                                               |
      |                                               expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10651:23
      |
10651 |         pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
      |                       ^^^^^^^^^^            -------------------------
help: remove the extra argument
      |
23845 -                 let step = ingress.close_step(maximum_items.min(1), maximum_bytes)?;
23845 +                 let step = ingress.close_step(/* dsl::RetainedCloneGrant */)?;
      |

```

### 255. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23846:28`. Raw line 43381.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23846:28
      |
23846 |                 if step == PluginCloseStep::Complete {
      |                    ----    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RetainedCloneStep`, found `PluginCloseStep`
      |                    |
      |                    expected because this is `dsl::RetainedCloneStep`

```

### 256. E0624: method `pending_command_retirement_demands` is private

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:465:25`. Raw line 43389.

```text
error[E0624]: method `pending_command_retirement_demands` is private
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:465:25
    |
465 |         let demand=self.pending_command_retirement_demands(body)?;
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ private method
    |
   ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/♻️retirement/🦀️.rs:12:2
    |
 12 |  fn pending_command_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
    |  -------------------------------------------------------------------------------------------- private method defined here

```

### 257. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:196:63`. Raw line 43400.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:196:63
      |
  196 | ...mut() { let step = raw.close_one(1, child.maximum_release_bytes); if step == PluginCloseStep::Complete { self.raw.take(); } re...
      |                           ^^^^^^^^^ -  --------------------------- unexpected argument #2 of type `usize`
      |                                     |
      |                                     expected `RetainedCloneGrant`, found integer
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14084:23
      |
14084 |         pub(crate) fn close_one(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
      |                       ^^^^^^^^^            -------------------------
help: remove the extra argument
      |
  196 -         if let Some(raw) = self.raw.as_mut() { let step = raw.close_one(1, child.maximum_release_bytes); if step == PluginCloseStep::Complete { self.raw.take(); } return Ok(group_plugin(step)); }
  196 +         if let Some(raw) = self.raw.as_mut() { let step = raw.close_one(/* dsl::RetainedCloneGrant */); if step == PluginCloseStep::Complete { self.raw.take(); } return Ok(group_plugin(step)); }
      |

```

### 258. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:196:117`. Raw line 43419.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:196:117
    |
196 | ...ytes); if step == PluginCloseStep::Complete { self.raw.take(); } return Ok(group_plugin(step)); }
    |              ----    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<RetainedCloneStep, ValueError>`, found `PluginCloseStep`
    |              |
    |              expected because this is `Result<dsl::RetainedCloneStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `Result<dsl::RetainedCloneStep, semio_framework_value::ValueError>`
               found enum `component::app::PluginCloseStep`

```

### 259. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:196:187`. Raw line 43430.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:196:187
    |
196 | ...ke(); } return Ok(group_plugin(step)); }
    |                      ------------ ^^^^ expected `PluginCloseStep`, found `Result<RetainedCloneStep, ValueError>`
    |                      |
    |                      arguments to this function are incorrect
    |
    = note: expected enum `component::app::PluginCloseStep`
               found enum `Result<dsl::RetainedCloneStep, semio_framework_value::ValueError>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:687:4
    |
687 | fn group_plugin(step: PluginCloseStep) -> RetainedCloneStep { match step { PluginCloseStep::Pending { released_items, released_byte...
    |    ^^^^^^^^^^^^ ---------------------

```

### 260. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23853:27`. Raw line 43446.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23853:27
      |
23853 |                 return Ok(step);
      |                        -- ^^^^ expected `PluginCloseStep`, found `RetainedCloneStep`
      |                        |
      |                        arguments to this enum variant are incorrect
      |
help: the type constructed contains `dsl::RetainedCloneStep` due to the type of the argument passed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23853:24
      |
23853 |                 return Ok(step);
      |                        ^^^----^
      |                           |
      |                           this argument influences the type of `Ok`
note: tuple variant defined here
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:561:5
      |
  561 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ^^

```

### 261. E0624: method `session_retirement_demands` is private

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:467:25`. Raw line 43467.

```text
error[E0624]: method `session_retirement_demands` is private
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:467:25
    |
467 |         let demand=self.session_retirement_demands(body)?;
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ private method
    |
   ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/♻️retirement/🦀️.rs:26:2
    |
 26 |  fn session_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
    |  ------------------------------------------------------------------------------------ private method defined here

```

### 262. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23856:37`. Raw line 43478.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23856:37
      |
23856 |                 let step = registry.close_step(maximum_items.min(1), maximum_bytes)?;
      |                                     ^^^^^^^^^^ --------------------  ------------- unexpected argument #2 of type `usize`
      |                                                |
      |                                                expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10777:23
      |
10777 |         pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
      |                       ^^^^^^^^^^            -------------------------
help: remove the extra argument
      |
23856 -                 let step = registry.close_step(maximum_items.min(1), maximum_bytes)?;
23856 +                 let step = registry.close_step(/* dsl::RetainedCloneGrant */)?;
      |

```

### 263. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23857:28`. Raw line 43497.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23857:28
      |
23857 |                 if step == PluginCloseStep::Complete {
      |                    ----    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RetainedCloneStep`, found `PluginCloseStep`
      |                    |
      |                    expected because this is `dsl::RetainedCloneStep`

```

### 264. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23864:27`. Raw line 43505.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23864:27
      |
23864 |                 return Ok(step);
      |                        -- ^^^^ expected `PluginCloseStep`, found `RetainedCloneStep`
      |                        |
      |                        arguments to this enum variant are incorrect
      |
help: the type constructed contains `dsl::RetainedCloneStep` due to the type of the argument passed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23864:24
      |
23864 |                 return Ok(step);
      |                        ^^^----^
      |                           |
      |                           this argument influences the type of `Ok`
note: tuple variant defined here
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:561:5
      |
  561 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ^^

```

### 265. E0624: method `pending_command_retirement_step` is private

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:478:32`. Raw line 43526.

```text
error[E0624]: method `pending_command_retirement_step` is private
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:478:32
    |
478 |         if let Some(step)=self.pending_command_retirement_step(grant).map_err(ValueError::into_fault)?{return Ok(Some(step));}
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ private method
    |
   ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/♻️retirement/🦀️.rs:18:2
    |
 18 |  fn pending_command_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{
    |  -------------------------------------------------------------------------------------------------------------------- private method defined here

```

### 266. E0624: method `session_retirement_step` is private

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:479:32`. Raw line 43537.

```text
error[E0624]: method `session_retirement_step` is private
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:479:32
    |
479 |         if let Some(step)=self.session_retirement_step(grant).map_err(ValueError::into_fault)?{return Ok(Some(step));}
    |                                ^^^^^^^^^^^^^^^^^^^^^^^ private method
    |
   ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/♻️retirement/🦀️.rs:32:2
    |
 32 |  fn session_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{
    |  ------------------------------------------------------------------------------------------------------------ private method defined here

```

### 267. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:418:181`. Raw line 43548.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:418:181
    |
418 | ...pected_revision, actor: std::mem::take(&mut parts.actor), group_id: parts.group_id.take(), transaction: parts.transaction.take()...
    |                            -------------- ^^^^^^^^^^^^^^^^ expected `&mut SharedUtf8`, found `&mut String`
    |                            |
    |                            arguments to this function are incorrect
    |
    = note: expected mutable reference `&mut SharedUtf8`
               found mutable reference `&mut std::string::String`
note: function defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/mod.rs:888:14
    |
888 | pub const fn take<T: [const] Default>(dest: &mut T) -> T {
    |              ^^^^

```

### 268. E0061: this method takes 5 arguments but 6 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23875:157`. Raw line 43564.

```text
error[E0061]: this method takes 5 arguments but 6 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23875:157
      |
23875 | ...)?.close_step(children, None, current, &owners, maximum_items.min(1), maximum_bytes)?;
      |       ^^^^^^^^^^                                   --------------------  ------------- unexpected argument #6 of type `usize`
      |                                                    |
      |                                                    expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11567:23
      |
11567 |         pub(crate) fn close_step<M: SpaceMember>(
      |                       ^^^^^^^^^^
...
11573 |             grant: RetainedCloneGrant,
      |             -------------------------
help: remove the extra argument
      |
23875 -                 let step = retirements.get_mut(generation).ok_or_else(|| plugin_sdk_fault("displaced content retirement changed before one bounded step"))?.close_step(children, None, current, &owners, maximum_items.min(1), maximum_bytes)?;
23875 +                 let step = retirements.get_mut(generation).ok_or_else(|| plugin_sdk_fault("displaced content retirement changed before one bounded step"))?.close_step(children, None, current, &owners, /* dsl::RetainedCloneGrant */)?;
      |

```

### 269. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23876:28`. Raw line 43586.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23876:28
      |
23876 |                 if step != PluginCloseStep::Complete {
      |                    ----    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RetainedCloneStep`, found `PluginCloseStep`
      |                    |
      |                    expected because this is `dsl::RetainedCloneStep`

```

### 270. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23878:31`. Raw line 43594.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23878:31
      |
23878 |                     return Ok(step);
      |                            -- ^^^^ expected `PluginCloseStep`, found `RetainedCloneStep`
      |                            |
      |                            arguments to this enum variant are incorrect
      |
help: the type constructed contains `dsl::RetainedCloneStep` due to the type of the argument passed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23878:28
      |
23878 |                     return Ok(step);
      |                            ^^^----^
      |                               |
      |                               this argument influences the type of `Ok`
note: tuple variant defined here
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:561:5
      |
  561 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ^^

```

### 271. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:515:12`. Raw line 43615.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:515:12
    |
515 |         Ok(step)
    |         -- ^^^^ expected `PluginCloseStep`, found `RetainedCloneStep`
    |         |
    |         arguments to this enum variant are incorrect
    |
help: the type constructed contains `dsl::RetainedCloneStep` due to the type of the argument passed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:515:9
    |
515 |         Ok(step)
    |         ^^^----^
    |            |
    |            this argument influences the type of `Ok`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:561:5
    |
561 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^

```

### 272. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🛠️tool-machine/🦀️.rs:733:52`. Raw line 43636.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🛠️tool-machine/🦀️.rs:733:52
    |
733 | ...   .send(&tag.window, &tag.tool, &ActorId(actor.to_string()), &base, tag.phase.clone().input(leaves), semio_framework_tool_machi...
    |                                      ------- ^^^^^^^^^^^^^^^^^ expected `SharedUtf8`, found `String`
    |                                      |
    |                                      arguments to this struct are incorrect
    |
note: tuple struct defined here
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🆔️ids/🦀️.rs:20:12
    |
 20 | pub struct ActorId(pub semio_framework_value::SharedUtf8);
    |            ^^^^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
    |
733 |             .send(&tag.window, &tag.tool, &ActorId(actor.to_string().into()), &base, tag.phase.clone().input(leaves), semio_framework_tool_machine::authoring_clock(0))
    |                                                                     +++++++

```

### 273. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🛠️tool-machine/🦀️.rs:776:60`. Raw line 43654.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🛠️tool-machine/🦀️.rs:776:60
    |
776 | ...   .send(&tag.window, &tag.tool, &ActorId(mounted.meta.actor.clone()), TypingInput::Edit { buffer: tag.buffer, leaves }, A::typi...
    |                                      ------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `SharedUtf8`, found `String`
    |                                      |
    |                                      arguments to this struct are incorrect
    |
note: tuple struct defined here
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🆔️ids/🦀️.rs:20:12
    |
 20 | pub struct ActorId(pub semio_framework_value::SharedUtf8);
    |            ^^^^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
    |
776 |                     .send(&tag.window, &tag.tool, &ActorId(mounted.meta.actor.clone().into()), TypingInput::Edit { buffer: tag.buffer, leaves }, A::typing_fold, clock)
    |                                                                                      +++++++

```

### 274. E0061: this method takes 5 arguments but 6 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23890:39`. Raw line 43672.

```text
error[E0061]: this method takes 5 arguments but 6 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23890:39
      |
23890 | ...rement.close_step(children, None, current_content, &ChildContentOwners::none(), maximum_items.min(1), maximum_bytes)?;
      |           ^^^^^^^^^^                                                               --------------------  ------------- unexpected argument #6 of type `usize`
      |                                                                                    |
      |                                                                                    expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11567:23
      |
11567 |         pub(crate) fn close_step<M: SpaceMember>(
      |                       ^^^^^^^^^^
...
11573 |             grant: RetainedCloneGrant,
      |             -------------------------
help: remove the extra argument
      |
23890 -                 let step = retirement.close_step(children, None, current_content, &ChildContentOwners::none(), maximum_items.min(1), maximum_bytes)?;
23890 +                 let step = retirement.close_step(children, None, current_content, &ChildContentOwners::none(), /* dsl::RetainedCloneGrant */)?;
      |

```

### 275. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23891:28`. Raw line 43694.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23891:28
      |
23891 |                 if step == PluginCloseStep::Complete {
      |                    ----    ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `RetainedCloneStep`, found `PluginCloseStep`
      |                    |
      |                    expected because this is `dsl::RetainedCloneStep`

```

### 276. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23898:27`. Raw line 43702.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23898:27
      |
23898 |                 return Ok(step);
      |                        -- ^^^^ expected `PluginCloseStep`, found `RetainedCloneStep`
      |                        |
      |                        arguments to this enum variant are incorrect
      |
help: the type constructed contains `dsl::RetainedCloneStep` due to the type of the argument passed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23898:24
      |
23898 |                 return Ok(step);
      |                        ^^^----^
      |                           |
      |                           this argument influences the type of `Ok`
note: tuple variant defined here
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:561:5
      |
  561 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ^^

```

### 277. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23909:42`. Raw line 43723.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23909:42
      |
23909 |                 return match composition.close_step(maximum_items.min(1), maximum_bytes) {
      |                                          ^^^^^^^^^^ --------------------  ------------- unexpected argument #2 of type `usize`
      |                                                     |
      |                                                     expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:30166:12
      |
30166 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
      |            ^^^^^^^^^^
help: remove the extra argument
      |
23909 -                 return match composition.close_step(maximum_items.min(1), maximum_bytes) {
23909 +                 return match composition.close_step(/* dsl::RetainedCloneGrant */) {
      |

```

### 278. E0277: the trait bound `Result<OwnedJsonSchemaValidator, std::string::String>: RetireOwned` is not satisfied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:139:5`. Raw line 43742.

```text
error[E0277]: the trait bound `Result<OwnedJsonSchemaValidator, std::string::String>: RetireOwned` is not satisfied
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:139:5
    |
130 | #[derive(semio_framework_value::RetireOwned)]
    |          ---------------------------------- required by a bound introduced by this call
...
139 |     validator: Result<semio_framework_schema::OwnedJsonSchemaValidator, String>,
    |     ^^^^^^^^^ the trait `RetireOwned` is not implemented for `Result<OwnedJsonSchemaValidator, std::string::String>`
    |
    = help: the following other types implement trait `RetireOwned`:
              &'static str
              ()
              (A, B)
              (A, B, C)
              (A, B, C, D)
              (A, B, C, D, E)
              ActionArgOption
              AlternativeView
            and 374 others
note: required by a bound in `deferred`
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:423:20
    |
423 | pub fn deferred<T: RetireOwned>(value: T) -> Box<dyn RetirementCursor> {
    |                    ^^^^^^^^^^^ required by this bound in `deferred`

```

### 279. E0061: this method takes 2 arguments but 1 argument was supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:730:46`. Raw line 43767.

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:730:46
      |
  730 | ...k(store.retire_snapshot_alias(snapshot.into_snapshot_owner()).map_err(|error| error.into_fault())?);
      |            ^^^^^^^^^^^^^^^^^^^^^-------------------------------- argument #2 of type `dsl::RetainedCloneGrant` is missing
      |
note: expected `&mut Option<Arc<P>>`, found `Arc<P>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:730:68
      |
  730 | ...   self.retirements.push_back(store.retire_snapshot_alias(snapshot.into_snapshot_owner()).map_err(|error| error.into_fault())?);
      |                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      = note: expected mutable reference `&mut std::option::Option<Arc<_>>`
                            found struct `Arc<_>`
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20985:12
      |
20985 |     pub fn retire_snapshot_alias(&self,alias:&mut Option<Arc<P>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshot...
      |            ^^^^^^^^^^^^^^^^^^^^^
help: provide the argument
      |
  730 -             self.retirements.push_back(store.retire_snapshot_alias(snapshot.into_snapshot_owner()).map_err(|error| error.into_fault())?);
  730 +             self.retirements.push_back(store.retire_snapshot_alias(/* &mut std::option::Option<std::sync::Arc<P>> */, /* dsl::RetainedCloneGrant */).map_err(|error| error.into_fault())?);
      |

```

### 280. E0308: `?` operator has incompatible types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:730:40`. Raw line 43791.

```text
error[E0308]: `?` operator has incompatible types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:730:40
    |
730 | ...ts.push_back(store.retire_snapshot_alias(snapshot.into_snapshot_owner()).map_err(|error| error.into_fault())?);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Option<(Box<_>, _)>`
    |
    = note: `?` operator cannot convert from `std::option::Option<(std::boxed::Box<dyn ErasedSnapshotRetirement>, dsl::RetainedCloneProgress)>` to `std::boxed::Box<(dyn ErasedSnapshotRetirement + 'static)>`
    = note: expected struct `std::boxed::Box<(dyn ErasedSnapshotRetirement + 'static)>`
                 found enum `std::option::Option<(std::boxed::Box<dyn ErasedSnapshotRetirement>, dsl::RetainedCloneProgress)>`

```

### 281. E0277: the trait bound `Result<OwnedJsonSchemaValidator, std::string::String>: RetireOwned` is not satisfied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:139:16`. Raw line 43801.

```text
error[E0277]: the trait bound `Result<OwnedJsonSchemaValidator, std::string::String>: RetireOwned` is not satisfied
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏪️time-travel/🦀️.rs:139:16
    |
139 |     validator: Result<semio_framework_schema::OwnedJsonSchemaValidator, String>,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `Result<OwnedJsonSchemaValidator, std::string::String>`
    |
    = help: the following other types implement trait `RetireOwned`:
              &'static str
              ()
              (A, B)
              (A, B, C)
              (A, B, C, D)
              (A, B, C, D, E)
              ActionArgOption
              AlternativeView
            and 374 others
note: required by a bound in `deferred_birth_bytes`
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🦀️.rs:50:38
    |
 50 | pub const fn deferred_birth_bytes<T: RetireOwned>() -> usize { size_of::<Deferred<T>>() }
    |                                      ^^^^^^^^^^^ required by this bound in `deferred_birth_bytes`

```

### 282. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23932:39`. Raw line 43823.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23932:39
      |
23932 |                 let step = retirement.close_step(maximum_items.min(1), maximum_bytes)?;
      |                                       ^^^^^^^^^^ --------------------  ------------- unexpected argument #2 of type `usize`
      |                                                  |
      |                                                  expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11750:12
      |
11750 |         fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
      |            ^^^^^^^^^^           ------------------------
help: remove the extra argument
      |
23932 -                 let step = retirement.close_step(maximum_items.min(1), maximum_bytes)?;
23932 +                 let step = retirement.close_step(/* dsl::RetainedCloneGrant */)?;
      |

```

### 283. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23936:27`. Raw line 43842.

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23936:27
      |
23936 |                 return Ok(step);
      |                        -- ^^^^ expected `PluginCloseStep`, found `RetainedCloneStep`
      |                        |
      |                        arguments to this enum variant are incorrect
      |
help: the type constructed contains `dsl::RetainedCloneStep` due to the type of the argument passed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23936:24
      |
23936 |                 return Ok(step);
      |                        ^^^----^
      |                           |
      |                           this argument influences the type of `Ok`
note: tuple variant defined here
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:561:5
      |
  561 |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ^^

```

### 284. E0061: this function takes 3 arguments but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:125:9`. Raw line 43863.

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:125:9
    |
125 |         semio_framework_job::StepBudget::new(fuel, u64::MAX).with_retained(grant),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------- argument #3 of type `RetainedCloneGrant` is missing
    |
note: associated function defined here
   --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:394:12
    |
394 |     pub fn new(fuel: u64, deadline_us: u64, retained:RetainedCloneGrant) -> StepBudget {
    |            ^^^
help: provide the argument
    |
125 |         semio_framework_job::StepBudget::new(fuel, u64::MAX, /* RetainedCloneGrant */).with_retained(grant),
    |                                                            ++++++++++++++++++++++++++

```

### 285. E0599: no method named `with_retained` found for struct `StepBudget` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:125:62`. Raw line 43879.

```text
error[E0599]: no method named `with_retained` found for struct `StepBudget` in the current scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:125:62
    |
125 |         semio_framework_job::StepBudget::new(fuel, u64::MAX).with_retained(grant),
    |                                                              ^^^^^^^^^^^^^ method not found in `StepBudget`

```

### 286. E0433: cannot find module or crate `semio_framework_trace` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1759:28`. Raw line 43921.

```text
error[E0433]: cannot find module or crate `semio_framework_trace` in this scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1759:28
     |
1759 |     let(mut capture,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||BoardFillSnapshotCapture::new(0.0));
     |                            ^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_trace`
     |
help: there is a crate or module with a similar name
     |
1759 -     let(mut capture,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||BoardFillSnapshotCapture::new(0.0));
1759 +     let(mut capture,birth)=semio_framework_hash::observe_heap_allocations_on_this_thread(||BoardFillSnapshotCapture::new(0.0));
     |

```

### 287. E0433: cannot find module or crate `semio_framework_trace` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1765:28`. Raw line 43933.

```text
error[E0433]: cannot find module or crate `semio_framework_trace` in this scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1765:28
     |
1765 | ...   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||capture.close_step(RetainedCloneGrant{maximu...
     |                      ^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_trace`
     |
help: there is a crate or module with a similar name
     |
1765 -             let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||capture.close_step(RetainedCloneGrant{maximum_release_bytes:release-1,..grant}));
1765 +             let(step,heap)=semio_framework_hash::observe_heap_allocations_on_this_thread(||capture.close_step(RetainedCloneGrant{maximum_release_bytes:release-1,..grant}));
     |

```

### 288. E0433: cannot find module or crate `semio_framework_trace` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1770:24`. Raw line 43945.

```text
error[E0433]: cannot find module or crate `semio_framework_trace` in this scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1770:24
     |
1770 |         let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||capture.close_step(grant));
     |                        ^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_trace`
     |
help: there is a crate or module with a similar name
     |
1770 -         let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||capture.close_step(grant));
1770 +         let(step,heap)=semio_framework_hash::observe_heap_allocations_on_this_thread(||capture.close_step(grant));
     |

```

### 289. E0433: cannot find module or crate `semio_framework_trace` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1779:28`. Raw line 43957.

```text
error[E0433]: cannot find module or crate `semio_framework_trace` in this scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1779:28
     |
1779 |     let(mut ingress,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||BoardFillSnapshotIngress::new(0.0));
     |                            ^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_trace`
     |
help: there is a crate or module with a similar name
     |
1779 -     let(mut ingress,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||BoardFillSnapshotIngress::new(0.0));
1779 +     let(mut ingress,birth)=semio_framework_hash::observe_heap_allocations_on_this_thread(||BoardFillSnapshotIngress::new(0.0));
     |

```

### 290. E0433: cannot find module or crate `semio_framework_trace` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1783:24`. Raw line 43969.

```text
error[E0433]: cannot find module or crate `semio_framework_trace` in this scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1783:24
     |
1783 |         let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.close_step(grant));
     |                        ^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_trace`
     |
help: there is a crate or module with a similar name
     |
1783 -         let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.close_step(grant));
1783 +         let(step,heap)=semio_framework_hash::observe_heap_allocations_on_this_thread(||ingress.close_step(grant));
     |

```

### 291. E0433: cannot find module or crate `semio_framework_trace` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:95:28`. Raw line 43981.

```text
error[E0433]: cannot find module or crate `semio_framework_trace` in this scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:95:28
   |
95 |             let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pages.retire_descriptor(denied).unwrap());
   |                            ^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_trace`
   |
help: there is a crate or module with a similar name
   |
95 -             let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pages.retire_descriptor(denied).unwrap());
95 +             let(step,heap)=semio_framework_hash::observe_heap_allocations_on_this_thread(||pages.retire_descriptor(denied).unwrap());
   |

```

### 292. E0433: cannot find module or crate `semio_framework_trace` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:100:24`. Raw line 43993.

```text
error[E0433]: cannot find module or crate `semio_framework_trace` in this scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:100:24
    |
100 |         let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pages.retire_descriptor(grant).unwrap());
    |                        ^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_trace`
    |
help: there is a crate or module with a similar name
    |
100 -         let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pages.retire_descriptor(grant).unwrap());
100 +         let(step,heap)=semio_framework_hash::observe_heap_allocations_on_this_thread(||pages.retire_descriptor(grant).unwrap());
    |

```

### 293. E0433: cannot find module or crate `semio_framework_trace` in this scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:105:21`. Raw line 44005.

```text
error[E0433]: cannot find module or crate `semio_framework_trace` in this scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:105:21
    |
105 |         let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(reference));
    |                     ^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_trace`
    |
help: there is a crate or module with a similar name
    |
105 -         let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(reference));
105 +         let(_,heap)=semio_framework_hash::observe_heap_allocations_on_this_thread(||drop(reference));
    |

```

### 294. E0061: this function takes 3 arguments but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:119:124`. Raw line 44017.

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:119:124
    |
119 | ...eration(1), semio_framework_job::StepBudget::new(fuel, u64::MAX).with_retained(grant), cancel, semio_framework_job::default_now_...
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------------- argument #3 of type `RetainedCloneGrant` is missing
    |
note: associated function defined here
   --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:394:12
    |
394 |     pub fn new(fuel: u64, deadline_us: u64, retained:RetainedCloneGrant) -> StepBudget {
    |            ^^^
help: provide the argument
    |
119 |             semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, u64::MAX, /* RetainedCloneGrant */).with_retained(grant), cancel, semio_framework_job::default_now_us, &mut sequence,&mut receipt);
    |                                                                                                                                                                               ++++++++++++++++++++++++++

```

### 295. E0061: this function takes 3 arguments but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:1954:9`. Raw line 44033.

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:1954:9
     |
1954 |         semio_framework_job::StepBudget::new(4, u64::MAX),
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------------- argument #3 of type `RetainedCloneGrant` is missing
     |
note: associated function defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:394:12
     |
 394 |     pub fn new(fuel: u64, deadline_us: u64, retained:RetainedCloneGrant) -> StepBudget {
     |            ^^^
help: provide the argument
     |
1954 |         semio_framework_job::StepBudget::new(4, u64::MAX, /* RetainedCloneGrant */),
     |                                                         ++++++++++++++++++++++++++

```

### 296. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:1960:31`. Raw line 44049.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:1960:31
     |
1960 |     assert!(matches!(outcome, semio_framework_job::StepOutcome::Yield), "a bound producer keeps the preparation alive");
     |                      -------  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `(StepOutcome, RetainedCloneProgress)`, found `StepOutcome`
     |                      |
     |                      this expression has type `(StepOutcome, RetainedCloneProgress)`
     |
     = note: expected tuple `(StepOutcome, RetainedCloneProgress)`
                 found enum `StepOutcome`

```

### 297. E0061: this function takes 2 arguments but 3 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:1963:21`. Raw line 44060.

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:1963:21
     |
1963 |     while !matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::Int...
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^           -  ------------------------------------------- unexpected argument #3 of type `usize`
     |                                                          |
     |                                                          expected `RetainedCloneGrant`, found integer
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1241:8
     |
1241 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep;
     |        ^^^^^^^^^^
help: remove the extra argument
     |
1963 -     while !matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {}
1963 +     while !matches!(InteractiveJob::close_step(&mut job, /* RetainedCloneGrant */), semio_framework_job::InteractiveJobCloseStep::Complete) {}
     |

```

### 298. E0533: expected unit struct, unit variant or constant, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:1963:107`. Raw line 44079.

```text
error[E0533]: expected unit struct, unit variant or constant, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:1963:107
     |
1963 | ..._PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {}
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a unit struct, unit variant or constant
     |
help: add the names to match a struct variant's fields
     |
1963 |     while !matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete { progress: _ }) {}
     |                                                                                                                                                                  +++++++++++++++

```

### 299. E0061: this function takes 1 argument but 0 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:2037:16`. Raw line 44090.

```text
error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:2037:16
     |
2037 |             if ui_wgpu::wgpu::PreparedRenderInput::close_abandoned_step() {
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-- argument #1 of type `RetainedCloneGrant` is missing
     |
note: associated function defined here
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:2333:12
     |
2333 |     pub fn close_abandoned_step(grant: RetainedCloneGrant) -> CloseStep {
     |            ^^^^^^^^^^^^^^^^^^^^
help: provide the argument
     |
2037 |             if ui_wgpu::wgpu::PreparedRenderInput::close_abandoned_step(/* RetainedCloneGrant */) {
     |                                                                         ++++++++++++++++++++++++

```

### 300. E0308: mismatched types

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:2037:16`. Raw line 44106.

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:2037:16
     |
2037 |             if ui_wgpu::wgpu::PreparedRenderInput::close_abandoned_step() {
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `InteractiveJobCloseStep`

```

### 301. E0599: no method named `with_retained` found for struct `StepBudget` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:119:177`. Raw line 44112.

```text
error[E0599]: no method named `with_retained` found for struct `StepBudget` in the current scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:119:177
    |
119 | ...ork_job::StepBudget::new(fuel, u64::MAX).with_retained(grant), cancel, semio_framework_job::default_now_us, &mut sequence,&mut r...
    |                                             ^^^^^^^^^^^^^ method not found in `StepBudget`

```

### 302. E0061: this method takes 1 argument but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7424:31`. Raw line 44119.

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7424:31
     |
7424 | ...   while outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) != semio_framework_job::JobPayloadCloseStep::Comple...
     |                     ^^^^^^^^^^ -  ------------------------------------------- unexpected argument #2 of type `usize`
     |                                |
     |                                expected `RetainedCloneGrant`, found integer
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../♻️retirement/📄️payload/🦀️.rs:122:12
     |
 122 |     pub fn close_step(&mut self, grant: RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let demand=self.retirement_deman...
     |            ^^^^^^^^^^
help: remove the extra argument
     |
7424 -                 while outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) != semio_framework_job::JobPayloadCloseStep::Complete {}
7424 +                 while outcome.close_step(/* RetainedCloneGrant */) != semio_framework_job::JobPayloadCloseStep::Complete {}
     |

```

### 303. E0061: this function takes 2 arguments but 1 argument was supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7430:12`. Raw line 44138.

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7430:12
     |
7430 |     while !PreparedRenderJob::close_step(&mut job) {}
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1241:8
     |
1241 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep;
     |        ^^^^^^^^^^
help: provide the argument
     |
7430 |     while !PreparedRenderJob::close_step(&mut job, /* RetainedCloneGrant */) {}
     |                                                  ++++++++++++++++++++++++++

```

### 304. E0600: cannot apply unary operator `!` to type `InteractiveJobCloseStep`

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7430:11`. Raw line 44154.

```text
error[E0600]: cannot apply unary operator `!` to type `InteractiveJobCloseStep`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7430:11
     |
7430 |     while !PreparedRenderJob::close_step(&mut job) {}
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
     |
note: `InteractiveJobCloseStep` does not implement `Not`
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1201:1
     |
1201 | pub enum InteractiveJobCloseStep {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `InteractiveJobCloseStep` is defined in another crate

```

### 305. E0061: this function takes 3 arguments but 2 arguments were supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7695:130`. Raw line 44166.

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7695:130
     |
7695 | ...eneration(1),semio_framework_job::StepBudget::new(1,u64::MAX),semio_framework_job::root_cancel_token(),semio_framework_job::def...
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------------ argument #3 of type `RetainedCloneGrant` is missing
     |
note: associated function defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:394:12
     |
 394 |     pub fn new(fuel: u64, deadline_us: u64, retained:RetainedCloneGrant) -> StepBudget {
     |            ^^^
help: provide the argument
     |
7695 -     let mut context=semio_framework_job::StepContext::new(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX),semio_framework_job::root_cancel_token(),semio_framework_job::default_now_us,&mut sequence,&mut zero);
7695 +     let mut context=semio_framework_job::StepContext::new(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1, u64::MAX, /* RetainedCloneGrant */),semio_framework_job::root_cancel_token(),semio_framework_job::default_now_us,&mut sequence,&mut zero);
     |

```

### 306. E0061: this method takes 2 arguments but 1 argument was supplied

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs:143:23`. Raw line 44183.

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs:143:23
     |
 143 |     while !retirement.step(&mut state) {
     |                       ^^^^------------ argument #2 of type `&mut StepContext<'_>` is missing
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🦀️.rs:2568:8
     |
2568 |     fn step(&mut self, state: &mut World3dState, context:&mut semio_framework_job::StepContext<'_>) -> bool {
     |        ^^^^                                      -------------------------------------------------
help: provide the argument
     |
 143 |     while !retirement.step(&mut state, /* &mut StepContext<'_> */) {
     |                                      ++++++++++++++++++++++++++++

```

### 307. E0599: no method named `retire_step` found for struct `PreparedRenderPacket` in the current scope

Canonical source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. Exact compiler span: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7464:19`. Raw line 44199.

```text
error[E0599]: no method named `retire_step` found for struct `PreparedRenderPacket` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:7464:19
     |
7464 |     while !packet.retire_step() {}
     |                   ^^^^^^^^^^^ method not found in `PreparedRenderPacket`

```
