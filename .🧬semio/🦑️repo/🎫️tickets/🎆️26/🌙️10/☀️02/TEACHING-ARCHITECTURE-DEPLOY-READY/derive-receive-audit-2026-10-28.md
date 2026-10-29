# Recursive Receive Derive Audit — 2026-10-28

Bounded read-only audit while backend_live owns the E0609 repair and test91568. No source changes, builds or tests performed here. No concrete inconsistency found in the current repair.

All derive paths below are beneath 🧰️framework/🔨️modules/🌱️value/✨️derive/.

## Intended Contract

The existing neutral contract 🛬️receive/🔣️.json:15–20 requires sources, dependencies, recursive owner names and expectedCycles. Its fixtures distinguish acyclic ownership (🧪️tests/🧫️fixtures/🔣️.json:4), an unmarked Node↔Group cycle (15–37), an explicitly marked recursive enum (40–59), and a self-recursive struct (62–74). The latter sources already spell #[value(receive_recursive)] and #[value(receive_recursive,tag=...)] before this parser repair. Therefore this is restoration of the existing intended public attribute, rather than an invented attribute inferred solely from the missing field diagnostic.

The portable implementation 🛬️receive/🟦️.ts:1–2 removes conditional dependency edges only at declared recursive owners, rejecting unknown declared owners/dependencies. The graphlib Tarjan oracle 🛬️receive/🧪️tests/🔮️oracle.mjs:3 validates the corpus with Ajv2020 and independently compares strongly connected components to expectedCycles.

## Parser And Generated AST

Current ⚙️expansion/🦀️.rs:237–258 derives Default for ContainerAttrs and contains receive_recursive:bool at248. parse_value_meta:316–333 preserves bare versus string-valued forms. parse_container_attrs:348 accepts only a bare recursive flag; absent remains false, and a valued recursive flag falls through to the existing unsupported-attribute error at359. The new focused Rust parser test 🛬️receive/🧪️tests/🦀️.rs:5 explicitly checks absent=false, bare=true, and valued rejection.

The struct generator 🛬️receive/🦀️.rs:53–54 uses the attribute only to remove the original type NativeReceive implementation where clause. The generated receiver struct retains field NativeReceive/NativeDefault bounds at63, and the original implementation uses the chosen clause at73. The enum generator 🛬️receive/🔀️enum/🦀️.rs:29,40 likewise removes original implementation predicates while receiver behavior retains typed child requirements at45 and68. No runtime permission, allocation accounting, retirement or field validation bypass is introduced by this parser field.

The existing Rust AST regression 🛬️receive/🧪️tests/🦀️.rs:3 expands every fixture source, parses the output with syn and asserts an unconditional original NativeReceive implementation for marked owners plus a conditional receiver struct. This matches the intended edge cut and avoids removing the receiver constraints indiscriminately.

## Entry And Evidence Boundaries

The proc-macro front door 🦀️.rs:99–101 declares NativeReceive with attributes(value) and delegates to component::expand_native_receive. The expansion component ⚙️expansion/🦀️.rs:762–763 routes to the same receive module. No parallel public macro or adapter API is needed for this repair.

The neutral schema describes the dependency/AST fixture corpus; it does not itself parse Rust attribute syntax. The new parser test supplies that admission evidence. The graph oracle proves the declared dependency cuts; the AST test proves emitted predicate shape. Neither alone compiles and executes a concrete expanded recursive type against the native decoding runtime. backend_live owns actual compilation evidence and the proctor build; this report makes no claim that E0609 or subsequent compiler diagnostics are already resolved in an observed build.

## Follow-Up: Concrete Recursive Runtime Coverage

A bounded Rust-source inventory of the entire value module found no concrete compiled type using receive_recursive outside the generators/parser and source strings of expansion tests. Thus the existing recursive tests demonstrate AST shape and a declared graph, not actual generated recursive receiving. This is a test coverage gap, not a demonstrated production decoding defect.

The existing runtime struct receiving tests are 🛬️decode/🧳️owned/🔗️sealed-shared-receiver/🧪️tests/🦀️.rs:38 (WideRecord),65–79 (GroupName, GroupVolume, OriginalDeclaredDefaults),98–102 (FlattenedRecord), with execution laws at48,82,104. They are all acyclic. The parent receiver includes them at 🛬️decode/🧳️owned/🔗️sealed-shared-receiver/🦀️.rs:80. Helpers at test lines4–14 provide exact grants, original source custody, advance and funded retirement.

The existing runtime enum tests are ✨️derive/🛬️receive/🔀️enum/🧪️tests/🦀️.rs:5–24. Node, Element, Mutation, External and Adjacent contain no cycle. Its test58 actually drives generated receiving, cancellation frontiers and Serde comparisons; 🛬️decode/🧳️owned/🦀️.rs:60 includes this module. These paths are relative to 🧰️framework/🔨️modules/🌱️value/.

The precise missing regression is a concrete receive_recursive Branch with Vec<Branch>, plus a concrete marked Node enum through Group.children:Vec<Node>, compiled through the public derive and driven through begin_receive, funded advance, take, prefix cancellation and closure. The existing runtime locations and helpers are the intended places to extend; another expansion-only AST check would not close this evidence gap. backend_live received these exact locations while its owned test91568 remains the sole focused compiler run. No test or build was duplicated here.

## Owning Workspace For Focused Regression

The expansion package manifest ✨️derive/⚙️expansion/📦️packages/🦀️rust/Cargo.toml names semio-framework-value-derive-expansion and sets package.workspace to ../../../../../.., resolving to 🧰️framework/Cargo.toml. That workspace explicitly lists the package at54 and its workspace dependency at1214. 🎓️teaching/Cargo.toml:5–9 lists only proctor/domain/base as members. Testing this dependency from teaching does not establish its dev-dependency-enabled unit-test execution; terminal91568 was the wrong owning workspace rather than a regression failure.

From cwd /home/deck/semio/🧰️framework, the supported focused command is:

~~~sh
repo --lang=en cargo --store=shared --configuration=release --parallelism=1 -- test -p semio-framework-value-derive-expansion --lib recursive
~~~

Alternatively from repository root, add --manifest-path 🧰️framework/Cargo.toml after test. The expansion manifest lib.path ../../🦀️.rs resolves to its actual source ⚙️expansion/🦀️.rs; that source at762 mounts ../🛬️receive/🦀️.rs, whose142 mounts cfg(test) recursive_tests from 🧪️tests/🦀️.rs. Thus the recursive filter reaches both existing AST regression and new attribute parser test when the owning package unit tests are built. backend_live received these exact paths and owns the corrected invocation. This audit ran no build/test.

## Final Current-Tree Parser Review

Re-read current expansion/parser, Rust tests, existing neutral recursive fixtures and owning Cargo manifest after root reported corrected test22049 admitted. Source remains consistent: ContainerAttrs:248 has the default-false bool; parser:348 accepts only the preexisting bare receive_recursive spelling. No alias, valued compatibility form or migration path was introduced. Existing enum/struct fixture strings and recursive owner arrays still match that syntax; AST test still preserves typed receiver bounds while removing the declared original-type predicate cycle. New parser regression still checks absentfalse, baretrue and valued rejection.

The owning expansion manifest still declares only its existing syn/quote/proc-macro2 runtime expansion dependencies and serde_json as a dev dependency. No new runtime library or exported external type/API was found in this narrow repair; the existing portable graph schema requires no invented legacy branch. Reviewed repair ledger: ⚙️expansion/🦀️.rs commonfield/bareparser, and 🛬️receive/🧪️tests/🦀️.rs explicit attribute admission test. Portable fixture/schema and receiving generators establish the existing intended contract as described above.

No concrete source issue found. Compilation remains pending: root reports22049 compiling without recurrent E0609 so far, which is progress rather than terminal test or proctor-build success. This audit ran no test/build/probe and does not upgrade that claim. The previously recorded absence of concrete recursive runtime execution remains a distinct coverage limit.
