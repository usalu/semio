# Kernel Turn Patch Owner R1 Wrong-Target Non-Execution

The selected `@semio-tech/framework-os-kernel:test --args='ui_turn_patch_owner_ -- --nocapture'` was the **wrong host crate** for the new tests. It exited 1 with **30 real OS Store/SPR fixture errors**, but those are not demonstrated dependencies of the requested two laws. The common `🎠️kernel` module is included by the manifest module in `semio-framework`; the correct existing route is `@semio-tech/framework-rs:test-wire-retirement-native --args='--lib ui_turn_patch_owner_ -- --nocapture'`. Root independently located this ownership join. No requested semantic contention RED occurred in R1, and no production trait or assertion may be weakened.

Full raw log: `🧪️member-kernel-turn-patch-red-r1-native-2026-08-27.txt`. Actual diagnostic excerpts and final error count:

```text
844-22191 -             let owner = Arc::new(std::sync::Mutex::new(crate::os_vcs::ArtifactGroupVisibilityOwner::new()));
845-22191 +             let owner = Arc::new(Mutex::new(crate::os_vcs::ArtifactGroupVisibilityOwner::new()));
846-      |
847-
848:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
849-     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:24294:5
850-      |
851-24294 |     impl Mutation<DemoSnapshot> for TimestampedMutation {
852-      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
853-      |
854-      = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
855-      = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
856-
857:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
858-     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:25464:5
859-      |
860-25464 |     impl Mutation<DemoSnapshot> for ValidatedMutation {
861-      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
862-      |
863-      = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
864-      = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
865-
866:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
867-     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:24080:9
868-      |
869-24080 |         impl Mutation<DemoSnapshot> for LossyMutation {
870-      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
871-      |
872-      = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
873-      = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
874-
875:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
876-     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:24577:5
877-      |
878-24577 |     impl Mutation<DemoSnapshot> for SeverityMutation {
879-      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
880-      |
881-      = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
882-      = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
883-
884:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
885-     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:21202:5
886-      |
887-21202 |     impl Mutation<DemoSnapshot> for DemoMutation {
888-      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
889-      |
890-      = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
891-      = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
892-
893:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
894-   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️component.rs:226:5
895-    |
896-226 |     impl Mutation<Value> for Value {
897-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
898-    |
899-    = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
900-    = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
901-
902:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
903-   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️component.rs:235:5
904-    |
905-235 |     impl Mutation<Value> for Noop {
906-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
907-    |
908-    = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
909-    = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
910-
911:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
912-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🧪️testkit/🦀️component.rs:1156:5
913-     |
914-1156 |     impl crate::os_spr::Mutation<i64> for MissingTargetOp {
915-     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
916-     |
917-     = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
918-     = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
919-
920:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
921-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🧪️testkit/🦀️component.rs:1168:5
922-     |
923-1168 |     impl crate::os_spr::Mutation<i64> for BuggyMissingTargetOp {
924-     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
925-     |
926-     = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
927-     = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
928-
929:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
930-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🧪️testkit/🦀️component.rs:1183:5
931-     |
932-1183 |     impl crate::os_spr::Mutation<i64> for NondeterministicOp {
933-     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
934-     |
935-     = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
936-     = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
937-
938:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
939-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🧪️testkit/🦀️component.rs:1119:5
940-     |
941-1119 |     impl crate::os_spr::Mutation<i64> for AddOp {
942-     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
943-     |
944-     = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
945-     = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
946-
947:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
948-   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:895:5
949-    |
950-895 |     impl Mutation<i64> for AddOp {
951-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
952-    |
953-    = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
954-    = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
955-
956:error[E0046]: not all trait items implemented, missing: `DESCRIPTORS`, `descriptor`
957-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🧪️testkit/🦀️component.rs:1197:5
958-     |
959-1197 |     impl crate::os_spr::Mutation<i64> for RejectedForwardOp {
960-     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `DESCRIPTORS`, `descriptor` in implementation
961-     |
962-     = help: implement the missing item: `const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];`
963-     = help: implement the missing item: `fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { todo!() }`
964-
965:error[E0277]: the trait bound `DoubleAdd: protocol::MutationLeaf` is not satisfied
966-   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:947:48
967-    |
968-947 |     impl CompositeMutationKind<i64, AddOp> for DoubleAdd {
969-    |                                                ^^^^^^^^^ unsatisfied trait bound
--
986-    |
987-750 | pub trait CompositeMutationKind<P, Op: Mutation<P>>: MutationLeaf + Clone + serde::Serialize + serde::de::DeserializeOwned {
988-    |                                                      ^^^^^^^^^^^^ required by this bound in `CompositeMutationKind`
989-
990:error[E0277]: the trait bound `DerivedDoubleAdd: protocol::MutationLeaf` is not satisfied
991-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:1762:12
992-     |
993-1762 |     struct DerivedDoubleAdd {
994-     |            ^^^^^^^^^^^^^^^^ unsatisfied trait bound
--
1011-     |
1012- 205 | pub trait MutationKind<P, Op>: MutationLeaf + Clone + serde::Serialize + serde::de::DeserializeOwned
1013-     |                                ^^^^^^^^^^^^ required by this bound in `MutationKind`
1014-
1015:error[E0277]: the trait bound `DerivedDoubleAdd: protocol::MutationLeaf` is not satisfied
1016-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:1765:48
1017-     |
1018-1765 |     impl CompositeMutationKind<i64, AddOp> for DerivedDoubleAdd {
1019-     |                                                ^^^^^^^^^^^^^^^^ unsatisfied trait bound
--
1036-     |
1037- 750 | pub trait CompositeMutationKind<P, Op: Mutation<P>>: MutationLeaf + Clone + serde::Serialize + serde::de::DeserializeOwned {
1038-     |                                                      ^^^^^^^^^^^^ required by this bound in `CompositeMutationKind`
1039-
1040:error[E0277]: the trait bound `QuadAdd: protocol::MutationLeaf` is not satisfied
1041-   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:966:48
1042-    |
1043-966 |     impl CompositeMutationKind<i64, AddOp> for QuadAdd {
1044-    |                                                ^^^^^^^ unsatisfied trait bound
--
1061-    |
1062-750 | pub trait CompositeMutationKind<P, Op: Mutation<P>>: MutationLeaf + Clone + serde::Serialize + serde::de::DeserializeOwned {
1063-    |                                                      ^^^^^^^^^^^^ required by this bound in `CompositeMutationKind`
1064-
1065:error[E0277]: the trait bound `AddThenNotifyForeign: protocol::MutationLeaf` is not satisfied
1066-   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:995:48
1067-    |
1068-995 |     impl CompositeMutationKind<i64, AddOp> for AddThenNotifyForeign {
1069-    |                                                ^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
--
1086-    |
1087-750 | pub trait CompositeMutationKind<P, Op: Mutation<P>>: MutationLeaf + Clone + serde::Serialize + serde::de::DeserializeOwned {
1088-    |                                                      ^^^^^^^^^^^^ required by this bound in `CompositeMutationKind`
1089-
1090:error[E0277]: the trait bound `DoubleAdd: protocol::MutationLeaf` is not satisfied
1091-   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:969:45
1092-    |
1093-969 |             DoubleAdd { delta: self.delta }.plan(base, planner)?;
1094-    |                                             ^^^^ unsatisfied trait bound
--
1114-751 |     const SEMANTICS: SemanticDescriptor;
1115-752 |     fn plan(&self, base: &P, planner: &mut Planner<P, Op>) -> Result<(), PlanError>;
1116-    |        ---- required by a bound in this associated function
1117-
1118:error[E0277]: the trait bound `DoubleAdd: protocol::MutationLeaf` is not satisfied
1119-   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:971:45
1120-    |
1121-971 |             DoubleAdd { delta: self.delta }.plan(&mid, planner)?;
1122-    |                                             ^^^^ unsatisfied trait bound
--
1142-751 |     const SEMANTICS: SemanticDescriptor;
1143-752 |     fn plan(&self, base: &P, planner: &mut Planner<P, Op>) -> Result<(), PlanError>;
1144-    |        ---- required by a bound in this associated function
1145-
1146:error[E0277]: the trait bound `DerivedDoubleAdd: protocol::MutationLeaf` is not satisfied
1147-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:1762:12
1148-     |
1149-1762 |     struct DerivedDoubleAdd {
1150-     |            ^^^^^^^^^^^^^^^^ unsatisfied trait bound
--
1169-     |                                                      ^^^^^^^^^^^^ required by this bound in `CompositeMutationKind::SEMANTICS`
1170- 751 |     const SEMANTICS: SemanticDescriptor;
1171-     |           --------- required by a bound in this associated constant
1172-
1173:error[E0277]: the trait bound `DerivedDoubleAdd: protocol::MutationLeaf` is not satisfied
1174-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:1760:77
1175-     |
1176-1760 |     #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::CompositeMutation)]
1177-     |                                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
--
1198- 753 |     fn label(&self) -> String;
1199-     |        ----- required by a bound in this associated function
1200-     = note: this error originates in the derive macro `dsl_derive::CompositeMutation` (in Nightly builds, run with -Z macro-backtrace for more info)
1201-
1202:error[E0277]: the trait bound `DerivedDoubleAdd: protocol::MutationLeaf` is not satisfied
1203-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:1760:77
1204-     |
1205-1760 |     #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::CompositeMutation)]
1206-     |                                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
--
1227- 754 |     fn target(&self) -> Vec<String> {
1228-     |        ------ required by a bound in this associated function
1229-     = note: this error originates in the derive macro `dsl_derive::CompositeMutation` (in Nightly builds, run with -Z macro-backtrace for more info)
1230-
1231:error[E0277]: the trait bound `DerivedDoubleAdd: protocol::MutationLeaf` is not satisfied
1232-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:1781:53
1233-     |
1234-1781 |         let diff = MutationKind::<i64, AddOp>::diff(&kind, &base);
1235-     |                    -------------------------------- ^^^^^ unsatisfied trait bound
--
1257-...
1258- 211 |     fn diff(&self, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff>;
1259-     |        ---- required by a bound in this associated function
1260-
1261:error[E0277]: the trait bound `DerivedDoubleAdd: protocol::MutationLeaf` is not satisfied
1262-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:1783:59
1263-     |
1264-1783 |         let inverse = MutationKind::<i64, AddOp>::inverse(&kind, &base);
1265-     |                       ----------------------------------- ^^^^^ unsatisfied trait bound
--
1287-...
1288- 215 |     fn inverse(&self, base: &P) -> Vec<Op>;
1289-     |        ------- required by a bound in this associated function
1290-
1291:error[E0277]: the trait bound `DerivedDoubleAdd: protocol::MutationLeaf` is not satisfied
1292-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:1789:21
1293-     |
1294-1789 |         assert_eq!(<DerivedDoubleAdd as MutationKind<i64, AddOp>>::SEMANTICS.kind, "derived-double-add");
1295-     |                     ^^^^^^^^^^^^^^^^ unsatisfied trait bound
--
1315-...
1316- 209 |     const SEMANTICS: SemanticDescriptor;
1317-     |           --------- required by a bound in this associated constant
1318-
1319:error[E0277]: the trait bound `DerivedDoubleAdd: protocol::MutationLeaf` is not satisfied
1320-    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:1790:59
1321-     |
1322-1790 |         assert!(MutationKind::<i64, AddOp>::foreign_steps(&kind, &base).is_empty());
1323-     |                 ----------------------------------------- ^^^^^ unsatisfied trait bound
--
1345-...
1346- 230 |     fn foreign_steps(&self, _base: &P) -> Vec<ForeignStep> {
1347-     |        ------------- required by a bound in this associated function
1348-
1349:error[E0382]: borrow of partially moved value: `with_checkpoint`
1350-     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:24496:51
1351-      |
1352-24490 |   ...   let with_alternative_active = SpaceHistorySnapshot {
1353-      |  _____________________________________-
--
1364-
1365-Some errors have detailed explanations: E0046, E0277, E0382.
1366-For more information about an error, try `rustc --explain E0046`.
1367-warning: `semio-framework-os-kernel` (lib test) generated 64 warnings
1368:error: could not compile `semio-framework-os-kernel` (lib test) due to 30 previous errors; 64 warnings emitted
1369-1741 |  * throws on non-zero exit, signal, or budget exceed (the `[budget]` line is printed
1370-1742 |  * to stderr first so it survives a caller's try/catch, e.g. [[tryRun]]).
1371-1743 |  */
1372-1744 | export function runCmd(cmd: string, args: string[], opts: RunCmdOpts = {}): void {
--
1383-Bun v1.3.14 (macOS arm64)
1384-Warning: command "bun 📜️script.ts test ui_turn_patch_owner_ -- --nocapture" exited with non-zero status code
1385-
1386-
1387: NX   Running target test for project @semio-tech/framework-os-kernel failed
1388-
1389-Failed tasks:
1390-
1391-- @semio-tech/framework-os-kernel:test
```
