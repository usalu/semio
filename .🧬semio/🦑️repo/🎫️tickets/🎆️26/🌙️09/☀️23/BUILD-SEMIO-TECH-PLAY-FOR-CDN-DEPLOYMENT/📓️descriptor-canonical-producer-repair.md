# Canonical Descriptor Producer Repair

## Actual red baseline

AEC post-describe native unit command actually ran nine cases: eight semantic cases passed and descriptor_is_fresh failed. Both compared byte arrays had length 2524 and channel ABI 23. The first difference at byte 1492 was nested topic payload object order: native declaration appId/moduleId/label/iconId/computersJson, emitted pack appId/computersJson/iconId/label/moduleId. Re-running describe did not fix this contract disagreement.

## Confirmed owner boundary

The guest describe owner encoded PackageDescriptor ToValue directly; its payload DslValue retained insertion order. The descriptor emitter already had CanonicalDescriptorValue, recursively sorting every object by UTF-8 key bytes, with stable ordering for equal keys. The emitter's committed language-neutral canonical-descriptor-pack corpus and exact TypeScript pack bytes define this policy. The blank-hash freshness projection decoded/rebuilt typed outer descriptors while retaining intrinsic topic payload ordering, preserving the divergence.

## Source repair

The existing CanonicalDescriptorValue authority was relocated beside PackageDescriptor in the manifest owner. The emitter consumes the relocated authority. Guest/native package encoding and exact blank-hash encoding now consume the same authority; the exact byte freshness assertion remains unchanged. Emitter pack/self-hash behavior remains the existing canonical contract. No generic pack ordering policy, schema member semantics, compatibility facade or test normalization was added.

A native guest regression was authored before the source repair, reusing every committed neutral canonical descriptor vector as a topic payload and comparing the whole package bytes against serde_json's independent object ordering oracle. It additionally checks exact blank-hash byte idempotence. The registered framework-plugin test was launched in the current cold task-native store; native success is pending. Applicable root/product/OS instructions were read.

## Native Compiler Gate Baseline

The first native guest regression command failed before assertions: existing plugin lib tests have ten unresolved canonical imports (SQLite snapshot and InteractiveJobClassification). Descriptor runtime correctness has not yet been claimed.

```text
error[E0432]: unresolved import `semio_framework::io::sqlite_snapshot`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs:591:34
    |
591 | ...   use semio_framework::io::sqlite_snapshot::{SqliteDatabaseLimits, SqliteSnapshotPhase, export_sqlite_database, import_sqlite_d...
    |                                ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `io`

[cargo:build] running elapsedMs=590102
error[E0433]: cannot find type `InteractiveJobClassification` in this scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-app-builder/🦀️.rs:18:61
   |
18 | ...tial"] == "migrated" { InteractiveJobClassification::Migrated } else { InteractiveJobClassification::Unclassified };
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `InteractiveJobClassification`
   |
help: consider importing one of these enums
   |
 2 +     use crate::InteractiveJobClassification;
   |
 2 +     use semio_framework::InteractiveJobClassification;
   |

error[E0433]: cannot find type `InteractiveJobClassification` in this scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-app-builder/🦀️.rs:18:61
   |
18 | ...tial"] == "migrated" { InteractiveJobClassification::Migrated } else { InteractiveJobClassification::Unclassified };
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `InteractiveJobClassification`
   |
help: consider importing one of these enums
   |
 2 +     use crate::InteractiveJobClassification;
   |
 2 +     use semio_framework::InteractiveJobClassification;
   |

error[E0433]: cannot find type `InteractiveJobClassification` in this scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-app-builder/🦀️.rs:18:109
   |
18 | ...ion::Migrated } else { InteractiveJobClassification::Unclassified };
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `InteractiveJobClassification`
   |
help: consider importing one of these enums
error[E0433]: cannot find type `InteractiveJobClassification` in this scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-app-builder/🦀️.rs:18:109
   |
18 | ...ion::Migrated } else { InteractiveJobClassification::Unclassified };
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `InteractiveJobClassification`
   |
help: consider importing one of these enums
   |
 2 +     use crate::InteractiveJobClassification;
   |
 2 +     use semio_framework::InteractiveJobClassification;
   |

error[E0433]: cannot find `sqlite_snapshot` in `io`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs:88:81
    |
 88 |                 fn to_sqlite_database(&self, control: &mut semio_framework::io::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteD...
    |                                                                                 ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `io`
...
160 |     fixture_channel!(Std1AnySnapshot, Std1AnyDiff, Std1AnyMutation, std1_any_mutations, Std1AnyCommand, STD1_ANY_DIALECT, "semio.testkit.w1c-fixture.std1-any/v1");
error[E0433]: cannot find `sqlite_snapshot` in `io`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs:88:81
    |
 88 |                 fn to_sqlite_database(&self, control: &mut semio_framework::io::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteD...
    |                                                                                 ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `io`
...
160 |     fixture_channel!(Std1AnySnapshot, Std1AnyDiff, Std1AnyMutation, std1_any_mutations, Std1AnyCommand, STD1_ANY_DIALECT, "semio.testkit.w1c-fixture.std1-any/v1");
    |     -------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `fixture_channel` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these crates
    |
  7 +     use crate::sqlite_snapshot;
    |
  7 +     use semio_framework::sqlite_snapshot;
    |
  7 +     use semio_framework_os_kernel::sqlite_snapshot;
    |

error[E0433]: cannot find `sqlite_snapshot` in `io`
error[E0433]: cannot find `sqlite_snapshot` in `io`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs:96:127
    |
 96 |                 fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut semio_framework::io::sqlite_snapshot::SqliteSnapshotControl<...
    |                                                                                                                               ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `io`
...
160 |     fixture_channel!(Std1AnySnapshot, Std1AnyDiff, Std1AnyMutation, std1_any_mutations, Std1AnyCommand, STD1_ANY_DIALECT, "semio.testkit.w1c-fixture.std1-any/v1");
    |     -------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `fixture_channel` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these crates
    |
  7 +     use crate::sqlite_snapshot;
    |
  7 +     use semio_framework::sqlite_snapshot;
    |
  7 +     use semio_framework_os_kernel::sqlite_snapshot;
    |

error[E0433]: cannot find `sqlite_snapshot` in `io`
error[E0433]: cannot find `sqlite_snapshot` in `io`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs:88:81
    |
 88 |                 fn to_sqlite_database(&self, control: &mut semio_framework::io::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, semio_framew...
    |                                                                                 ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `io`
...
161 |     fixture_channel!(Std1StrictSnapshot, Std1StrictDiff, Std1StrictMutation, std1_strict_mutations, Std1StrictCommand, STD1_STRICT_DIALECT, "semio.testkit.w1c-fixture.std1-strict/v1");
    |     ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `fixture_channel` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these crates
    |
  7 +     use crate::sqlite_snapshot;
    |
  7 +     use semio_framework::sqlite_snapshot;
    |
  7 +     use semio_framework_os_kernel::sqlite_snapshot;
    |

error[E0433]: cannot find `sqlite_snapshot` in `io`
error[E0433]: cannot find `sqlite_snapshot` in `io`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs:96:127
    |
 96 |                 fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut semio_framework::io::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, ...
    |                                                                                                                               ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `io`
...
161 |     fixture_channel!(Std1StrictSnapshot, Std1StrictDiff, Std1StrictMutation, std1_strict_mutations, Std1StrictCommand, STD1_STRICT_DIALECT, "semio.testkit.w1c-fixture.std1-strict/v1");
    |     ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `fixture_channel` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these crates
    |
  7 +     use crate::sqlite_snapshot;
    |
  7 +     use semio_framework::sqlite_snapshot;
    |
  7 +     use semio_framework_os_kernel::sqlite_snapshot;
    |

error[E0433]: cannot find `sqlite_snapshot` in `io`
error[E0433]: cannot find `sqlite_snapshot` in `io`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs:88:81
    |
 88 |                 fn to_sqlite_database(&self, control: &mut semio_framework::io::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteD...
    |                                                                                 ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `io`
...
162 |     fixture_channel!(Std2AnySnapshot, Std2AnyDiff, Std2AnyMutation, std2_any_mutations, Std2AnyCommand, STD2_ANY_DIALECT, "semio.testkit.w1c-fixture.std2-any/v1");
    |     -------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `fixture_channel` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these crates
    |
  7 +     use crate::sqlite_snapshot;
    |
  7 +     use semio_framework::sqlite_snapshot;
    |
  7 +     use semio_framework_os_kernel::sqlite_snapshot;
    |

error[E0433]: cannot find `sqlite_snapshot` in `io`
error[E0433]: cannot find `sqlite_snapshot` in `io`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs:96:127
    |
 96 |                 fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut semio_framework::io::sqlite_snapshot::SqliteSnapshotControl<...
    |                                                                                                                               ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `io`
...
162 |     fixture_channel!(Std2AnySnapshot, Std2AnyDiff, Std2AnyMutation, std2_any_mutations, Std2AnyCommand, STD2_ANY_DIALECT, "semio.testkit.w1c-fixture.std2-any/v1");
    |     -------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `fixture_channel` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these crates
    |
  7 +     use crate::sqlite_snapshot;
    |
  7 +     use semio_framework::sqlite_snapshot;
    |
  7 +     use semio_framework_os_kernel::sqlite_snapshot;
    |

error[E0433]: cannot find `sqlite_snapshot` in `io`
error[E0433]: cannot find `sqlite_snapshot` in `io`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️app-declarations-fixture/🦀️.rs:620:131
    |
620 | ...[0].values[1], semio_framework::io::sqlite_snapshot::SqliteValue::Integer(serde_json::from_str::<serde_json::Value>(text).expect...
    |                                        ^^^^^^^^^^^^^^^ could not find `sqlite_snapshot` in `io`
    |
help: consider importing this enum
    |
  7 +     use crate::sqlite_snapshot::SqliteValue;
    |
help: if you import `SqliteValue`, refer to it directly
    |
620 -                 assert_eq!(database.table("fixture_value").expect("semantic value table").rows[0].values[1], semio_framework::io::sqlite_snapshot::SqliteValue::Integer(serde_json::from_str::<serde_json::Value>(text).expect("independent snapshot oracle")["value"].as_i64().expect("value")));
620 +                 assert_eq!(database.table("fixture_value").expect("semantic value table").rows[0].values[1], SqliteValue::Integer(serde_json::from_str::<serde_json::Value>(text).expect("independent snapshot oracle")["value"].as_i64().expect("value")));
    |

warning: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🧪️tests/🔬️schema-stamping/🦀️.rs:2:5
      |
    2 | use crate::__semio_dispatch_PluginApp;
```


The ten first-run lib-test compilation errors are repaired at their canonical test owners: declaration fixture SQLite types use the existing `semio_framework::sqlite_snapshot` reexport, and app-builder tests import the manifest-owned `InteractiveJobClassification` from the existing framework surface. No production compatibility export or dependency was added. Native retry session 72031 uses the same private task generation, skips Nx caches, and preserves the exact neutral corpus plus serde_json canonical byte oracle. It is pending; no assertion success is claimed yet.


Actual native retry 72031 completed with Nx exit 0: one regression passed, zero failed, 1058 unrelated cases skipped. It printed both neutral witnesses (nested reversed members and UTF-8/UTF-16 order divergence) and matched serde_json exact canonical descriptor bytes, including blank-hash freshness idempotence. Full AEC descriptor freshness remains a separate required nine-case package check.


Publication actual full AEC retry52806 completed exit0:9pass0skip, Nx26m27s. Canonical exact descriptor freshness now passes alongside all semantic/oracle cases. Current shared source compile included the coordinated Cad test DslValue qualification.
