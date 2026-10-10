# Renderer Wasm Original Compiler Boundary

The exact registered `⚖️check-wasm📋️` row selected `@semio-tech/framework-renderer-wgpu` with its unchanged original caller resources, offline setting and argv. Session 88900 exited 1, actual Cargo check exited 101 after 7m 33s; four dependency tasks succeeded. Plugin library reports 259 errors and Infinite library five errors. Renderer implementation compilation and runtime tests were not reached.

Unique diagnostic/location pairs from the actual log follow. These are compiler evidence, not acceptance.

```text
25125: error[E0433]: cannot find `extension_invocation_failure` in `crate`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️.rs:219:143
```

```text
25145: error[E0433]: cannot find `extension_invocation_failure` in `crate`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️.rs:250:80
```

```text
25165: error[E0433]: cannot find `extension_invocation_failure` in `crate`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️.rs:250:150
```

```text
25185: error[E0433]: cannot find `extension_invocation_failure` in `crate`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️.rs:252:72
```

```text
25205: error[E0433]: cannot find `extension_invocation_failure` in `crate`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️.rs:252:142
```

```text
25225: error[E0425]: cannot find value `framework_reserved_job_factory` in module `crate::plugin_runtime`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/🦀️.rs:147:106
```

```text
25241: error[E0425]: cannot find value `framework_reserved_job_demands` in module `crate::plugin_runtime`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/🦀️.rs:147:168
```

```text
25257: error[E0433]: cannot find type `ValueRefusalKind` in this scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21080:166
```

```text
25272: error[E0433]: cannot find type `ValueRefusalKind` in this scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21089:184
```

```text
25287: error[E0433]: cannot find type `ValueRefusalKind` in this scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21090:44
```

```text
39465: error[E0053]: method `advance` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:80:71
```

```text
39481: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📸️checkpoint/🦀️.rs:128:125
```

```text
39499: error[E0061]: this function takes 5 arguments but 4 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📸️checkpoint/🦀️.rs:128:18
```

```text
39527: error[E0560]: struct `component::app::MountedTypedCommandFullOperation<A>` has no field named `terminal_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs:120:343
```

```text
39535: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs:123:63
```

```text
40152: error[E0599]: no associated function or constant named `try_new` found for struct `MountedWorkerJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:25916:83
```

```text
40170: error[E0599]: no method named `pump_one` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:25940:42
```

```text
40176: error[E0599]: no method named `take_checked_out_outcome` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:25960:93
```

```text
40192: error[E0599]: no method named `callback_verdict` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:25981:39
```

```text
40209: error[E0599]: no method named `resume` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:25996:29
```

```text
40223: error[E0599]: no method named `begin_close` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:25999:25
```

```text
40248: error[E0599]: no method named `retirement_demands` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26003:35
```

```text
40279: error[E0599]: no method named `close_step` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26003:390
```

```text
40316: error[E0599]: no method named `terminal_is_empty` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26005:97
```

```text
40376: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27618:44
```

```text
40388: error[E0433]: cannot find `factory` in `retirement`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/📑️copy/🦀️.rs:364:69
```

```text
41390: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28923:118
```

```text
41416: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29538:54
```

```text
41441: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29601:122
```

```text
41459: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29646:37
```

```text
41565: error[E0119]: conflicting implementations of trait `RetireOwned` for type `artifact::CameraJson`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../🧵️retained/🧬️fields/🦀️.rs:7:25
```

```text
42336: error[E0599]: no associated function or constant named `try_new` found for struct `MountedWorkerJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30570:99
```

```text
42342: error[E0061]: this method takes 3 arguments but 2 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30632:74
```

```text
42358: error[E0599]: no associated function or constant named `take_checked_out_outcome` found for struct `MountedWorkerJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30635:126
```

```text
42370: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30941:89
```

```text
42389: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:30942:62
```

```text
42408: error[E0053]: method `step` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:7303:83
```

```text
42422: error[E0046]: not all trait items implemented, missing: `borrow_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:7302:5
```

```text
42430: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:32001:87
```

```text
42449: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:32041:91
```

```text
42468: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:32042:58
```

```text
42487: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:5434:79
```

```text
42505: error[E0061]: this method takes 2 arguments but 1 argument was supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:5441:19
```

```text
42521: error[E0599]: no associated function or constant named `try_new` found for struct `MountedWorkerJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33314:99
```

```text
42527: error[E0061]: this method takes 2 arguments but 1 argument was supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:5447:19
```

```text
42543: error[E0061]: this function takes 2 arguments but 3 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33684:9
```

```text
42571: error[E0061]: this function takes 5 arguments but 2 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33687:19
```

```text
42587: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33688:17
```

```text
42602: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33689:17
```

```text
42618: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33693:17
```

```text
42634: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33911:55
```

```text
42645: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33912:62
```

```text
42656: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33913:61
```

```text
42667: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33914:67
```

```text
42678: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:33915:69
```

```text
42697: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:35357:79
```

```text
42710: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:35372:67
```

```text
42721: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:40994:68
```

```text
42739: error[E0053]: method `step` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19131:78
```

```text
42753: error[E0046]: not all trait items implemented, missing: `borrow_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19121:5
```

```text
42764: error[E0053]: method `step` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19517:78
```

```text
42778: error[E0046]: not all trait items implemented, missing: `borrow_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19516:5
```

```text
42786: error[E0053]: method `step` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19614:86
```

```text
42804: error[E0046]: not all trait items implemented, missing: `borrow_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19613:13
```

```text
43386: error[E0053]: method `step` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19907:78
```

```text
43400: error[E0046]: not all trait items implemented, missing: `borrow_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19906:5
```

```text
43408: error[E0053]: method `step` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21338:74
```

```text
43422: error[E0046]: not all trait items implemented, missing: `borrow_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21337:5
```

```text
43430: error[E0053]: method `step` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22475:78
```

```text
43444: error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_job::InteractiveJob::close_step` has 2
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22533:23
```

```text
43457: error[E0046]: not all trait items implemented, missing: `borrow_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22474:5
```

```text
43465: error[E0053]: method `step` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:38394:78
```

```text
43479: error[E0046]: not all trait items implemented, missing: `borrow_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:38393:5
```

```text
43487: error[E0053]: method `step` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41155:78
```

```text
43501: error[E0046]: not all trait items implemented, missing: `borrow_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41154:5
```

```text
43509: error[E0053]: method `step` has an incompatible type for trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41383:78
```

```text
43523: error[E0046]: not all trait items implemented, missing: `borrow_outcome`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41381:5
```

```text
43531: error[E0599]: no associated function or constant named `try_new` found for struct `MountedWorkerJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️.rs:428:33
```

```text
43537: error[E0061]: this method takes 1 argument but 0 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️.rs:458:38
```

```text
43553: error[E0599]: no associated function or constant named `take_checked_out_outcome` found for struct `MountedWorkerJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️.rs:466:78
```

```text
43565: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️.rs:473:57
```

```text
43583: error[E0061]: this function takes 3 arguments but 2 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:553:93
```

```text
43599: error[E0061]: this function takes 7 arguments but 6 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:553:31
```

```text
43615: error[E0061]: this function takes 3 arguments but 2 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1251:89
```

```text
43631: error[E0061]: this function takes 7 arguments but 6 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1251:27
```

```text
43647: error[E0061]: this function takes 3 arguments but 2 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1276:85
```

```text
43663: error[E0061]: this function takes 7 arguments but 6 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️.rs:1276:23
```

```text
43679: error[E0061]: this enum variant takes 2 arguments but 1 argument was supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:94:12
```

```text
43754: error[E0061]: this function takes 5 arguments but 10 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏯️tool-run/🦀️.rs:2137:47
```

```text
43776: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏯️tool-run/🦀️.rs:2137:17
```

```text
43794: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././⏯️tool-run/🦀️.rs:2155:51
```

```text
43812: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🛠️tool-machine/🦀️.rs:733:52
```

```text
43830: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🛠️tool-machine/🦀️.rs:776:60
```

```text
43868: error[E0277]: `?` couldn't convert the error to `semio_framework_dsl::Fault`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:10803:52
```

```text
43886: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:39:435
```

```text
43909: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:42:395
```

```text
43932: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:155:452
```

```text
43950: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:122:170
```

```text
43966: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:418:181
```

```text
43982: error[E0061]: this enum variant takes 2 arguments but 1 argument was supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:46:24
```

```text
43998: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🦀️.rs:97:118
```

```text
44009: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:353:497
```

```text
44027: error[E0599]: no method named `capacity` found for struct `SharedUtf8` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs:154:350
```

```text
44033: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs:155:423
```

```text
44049: error[E0599]: no method named `close_release` found for struct `PresenceCommandCursor` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12463:67
```

```text
44055: error[E0061]: this method takes 1 argument but 2 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14558:142
```

```text
44074: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:14558:137
```

```text
44097: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18211:36
```

```text
44116: error[E0277]: the trait bound `M: ArtifactCanonicalJsonTree` is not satisfied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18768:9
```

```text
44136: error[E0061]: this method takes 2 arguments but 3 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19191:33
```

```text
44155: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19192:20
```

```text
44163: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19193:23
```

```text
44184: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19519:115
```

```text
44193: error[E0369]: cannot add `usize` to `()`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19854:142
```

```text
44201: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19854:58
```

```text
44207: error[E0533]: expected value, found struct variant `Step::Refused`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21369:157
```

```text
44219: error[E0599]: no associated function or constant named `try_new` found for struct `BatchJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21409:71
```

```text
44235: error[E0599]: no method named `step` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21430:46
```

```text
44249: error[E0599]: no method named `take_outcome` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21433:45
```

```text
44263: error[E0061]: this method takes 1 argument but 2 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21448:33
```

```text
44279: error[E0599]: no method named `resume` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21453:46
```

```text
44293: error[E0599]: no method named `begin_close` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21457:17
```

```text
44318: error[E0599]: no method named `terminal_is_empty` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21459:24
```

```text
44354: error[E0599]: no method named `close_step` found for type `!` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21462:29
```

```text
44391: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21894:69
```

```text
44405: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:21895:70
```

```text
44419: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22537:84
```

```text
44427: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22537:103
```

```text
44435: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22541:80
```

```text
44443: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22541:99
```

```text
44451: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22545:84
```

```text
44459: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22545:103
```

```text
44467: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22548:80
```

```text
44475: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22548:99
```

```text
44483: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22552:84
```

```text
44491: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22552:103
```

```text
44499: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22555:80
```

```text
44507: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22555:99
```

```text
44515: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22559:84
```

```text
44523: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22559:103
```

```text
44531: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22562:80
```

```text
44539: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22562:99
```

```text
44547: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22566:84
```

```text
44555: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22566:103
```

```text
44563: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22569:80
```

```text
44571: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22569:99
```

```text
44579: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22573:84
```

```text
44587: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22573:103
```

```text
44595: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22576:80
```

```text
44603: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22576:99
```

```text
44611: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22580:84
```

```text
44619: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22580:103
```

```text
44627: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22583:80
```

```text
44635: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22583:99
```

```text
44643: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22587:84
```

```text
44651: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22587:103
```

```text
44659: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22590:80
```

```text
44667: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22590:99
```

```text
44675: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22594:84
```

```text
44683: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22594:103
```

```text
44691: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22597:80
```

```text
44699: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22597:99
```

```text
44707: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22601:84
```

```text
44715: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22601:103
```

```text
44723: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22604:80
```

```text
44731: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22604:99
```

```text
44739: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22608:84
```

```text
44747: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22608:103
```

```text
44755: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22611:80
```

```text
44763: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22611:99
```

```text
44771: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22615:84
```

```text
44779: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22615:103
```

```text
44787: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22618:80
```

```text
44795: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22618:99
```

```text
44803: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22622:84
```

```text
44811: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22622:103
```

```text
44819: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22625:80
```

```text
44827: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22625:99
```

```text
44835: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22629:84
```

```text
44843: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22629:103
```

```text
44851: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22632:80
```

```text
44859: error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22632:99
```

```text
44867: error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22635:13
```

```text
44878: error[E0599]: no associated function or constant named `try_new` found for struct `MountedWorkerJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23362:99
```

```text
44884: error[E0061]: this method takes 1 argument but 2 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23472:27
```

```text
44903: error[E0599]: no method named `take_checked_out_outcome` found for mutable reference `&mut MountedWorkerJobSession<ArtifactEnvelopeDecodeAuthority<P, Mutation>>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23474:41
```

```text
44915: error[E0599]: no associated function or constant named `try_new` found for struct `MountedWorkerJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23894:99
```

```text
44921: error[E0782]: expected a type, found a trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24102:179
```

```text
44937: error[E0782]: expected a type, found a trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24102:259
```

```text
44953: error[E0782]: expected a type, found a trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24102:347
```

```text
44969: error[E0782]: expected a type, found a trait
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24102:421
```

```text
44985: error[E0061]: this method takes 1 argument but 2 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24258:27
```

```text
45004: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24263:29
```

```text
45015: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24264:29
```

```text
45027: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24265:29
```

```text
45039: error[E0599]: no method named `ok_or_else` found for enum `Result<T, E>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24268:26
```

```text
45058: error[E0599]: no method named `take_checked_out_outcome` found for mutable reference `&mut MountedWorkerJobSession<component::app::ArtifactStoreInitializationJob<P, Mutation>>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24287:51
```

```text
45070: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26507:21
```

```text
45081: error[E0529]: expected an array or slice, found `Option<(ArtifactStoreConstructorKind, _, _, _)>`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26507:65
```

```text
45090: error[E0070]: invalid left-hand side of assignment
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26507:68
```

```text
45098: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26538:82
```

```text
45109: error[E0529]: expected an array or slice, found `Option<(ArtifactStoreConstructorKind, _, _, _)>`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26538:126
```

```text
45118: error[E0070]: invalid left-hand side of assignment
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:26538:129
```

```text
45126: error[E0599]: no method named `is_cancelled_now` found for struct `std::mem::ManuallyDrop<std::option::Option<CancelToken>>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27089:80
```

```text
45132: error[E0061]: this method takes 2 arguments but 3 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27689:81
```

```text
45151: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27689:76
```

```text
45163: error[E0599]: no method named `private_child_group_operation_close_byte_demand` found for mutable reference `&mut component::app::VcsArtifactApp<A, M>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27736:175
```

```text
45175: error[E0425]: cannot find function `mounted_private_child_grant` in this scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27737:17
```

```text
45181: error[E0616]: field `member` of struct `time_travel::TimeTravelLedger` is private
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27755:43
```

```text
45187: error[E0616]: field `member` of struct `time_travel::TimeTravelLedger` is private
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:27756:43
```

```text
45193: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28537:34
```

```text
45219: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28541:87
```

```text
45232: error[E0308]: mismatched types
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28603:25
```

```text
45241: error[E0061]: this method takes 1 argument but 2 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:31758:29
```

```text
45262: error[E0599]: no method named `is_some` found for struct `JobOutcomeSlot` in the current scope
--> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🚪️lifetime/🦀️.rs:76:96
```

```text
45268: error[E0533]: expected value, found struct variant `Step::Refused`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41095:118
```

```text
45280: error[E0533]: expected value, found struct variant `Step::Refused`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41101:217
```

```text
45292: error[E0164]: expected tuple struct or tuple variant, found struct variant `Worker::Refused`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41112:188
```

```text
45304: error[E0533]: expected value, found struct variant `Step::Refused`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41112:211
```

```text
45316: error[E0533]: expected value, found struct variant `Step::Refused`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41112:234
```

```text
45328: error[E0599]: no associated function or constant named `try_new` found for struct `BatchJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41345:57
```

```text
45334: error[E0061]: this method takes 1 argument but 0 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41354:20
```

```text
45350: error[E0599]: no method named `take_outcome` found for mutable reference `&mut BatchJobSession<RuntimeLiveCleanupJob<PA>>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41366:37
```

```text
45356: error[E0599]: no associated function or constant named `try_new` found for struct `BatchJobSession<J>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41719:57
```

```text
45362: error[E0061]: this method takes 1 argument but 0 arguments were supplied
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41730:31
```

```text
45378: error[E0599]: no method named `take_outcome` found for mutable reference `&mut BatchJobSession<RuntimeCloseCleanupJob<PA>>` in the current scope
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:41748:37
```

```text
45384: error[E0658]: use of unstable library feature `str_as_str`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../💡️inference/🚪️gateway/🦀️.rs:128:28
```

```text
45394: error[E0658]: use of unstable library feature `str_as_str`
--> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../💡️inference/🚪️gateway/🦀️.rs:129:79
```
