# Original Accepted Source Native Gate

Registered native5243/session74932 naturally returned Nx1 before the selected law. UI lib-test reported116 errors and140 warnings. The alias law did not run. The first five errors are existing cfg(test) raster parameter names; remaining errors are mandatory caller/close result fixture migrations. No native alias, collector, immutable interaction, DOM, GPU or full runtime claim follows.

## Complete Compiler Refusals


```text
error[E0425]: cannot find value `width` in this scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:1820:76
     |
1817 |     fn content_identity(&self, _width: u32, _height: u32) -> Option<RasterContentIdentity> {
     |                                ------ `_width` defined here
...
1820 |             Self::Contiguous(pixels) => RasterContentIdentity::test_pixels(width, height, pixels),
     |                                                                            ^^^^^
     |
help: the leading underscore in `_width` marks it as unused, consider renaming it to `width`
     |
1817 -     fn content_identity(&self, _width: u32, _height: u32) -> Option<RasterContentIdentity> {
1817 +     fn content_identity(&self, width: u32, _height: u32) -> Option<RasterContentIdentity> {
     |
```

```text
error[E0425]: cannot find value `height` in this scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:1820:83
     |
1817 |     fn content_identity(&self, _width: u32, _height: u32) -> Option<RasterContentIdentity> {
     |                                             ------- `_height` defined here
...
1820 |             Self::Contiguous(pixels) => RasterContentIdentity::test_pixels(width, height, pixels),
     |                                                                                   ^^^^^^
     |
help: the leading underscore in `_height` marks it as unused, consider renaming it to `height`
     |
1817 -     fn content_identity(&self, _width: u32, _height: u32) -> Option<RasterContentIdentity> {
1817 +     fn content_identity(&self, _width: u32, height: u32) -> Option<RasterContentIdentity> {
     |
```

```text
error[E0425]: cannot find value `start` in this scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:1829:52
     |
1826 |     fn with_rows<R>(&self, row: u32, _start: usize, _end: usize, _rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
     |                                      ------ `_start` defined here
...
1829 |             Self::Contiguous(pixels) => pixels.get(start..end).map(|page| read(page, rows)),
     |                                                    ^^^^^
     |
help: the leading underscore in `_start` marks it as unused, consider renaming it to `start`
     |
1826 -     fn with_rows<R>(&self, row: u32, _start: usize, _end: usize, _rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
1826 +     fn with_rows<R>(&self, row: u32, start: usize, _end: usize, _rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
     |
```

```text
error[E0425]: cannot find value `end` in this scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:1829:59
     |
1826 |     fn with_rows<R>(&self, row: u32, _start: usize, _end: usize, _rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
     |                                                     ---- `_end` defined here
...
1829 |             Self::Contiguous(pixels) => pixels.get(start..end).map(|page| read(page, rows)),
     |                                                           ^^^
     |
help: the leading underscore in `_end` marks it as unused, consider renaming it to `end`
     |
1826 -     fn with_rows<R>(&self, row: u32, _start: usize, _end: usize, _rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
1826 +     fn with_rows<R>(&self, row: u32, _start: usize, end: usize, _rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
     |
help: you might have meant to write `.` instead of `..`
     |
1829 -             Self::Contiguous(pixels) => pixels.get(start..end).map(|page| read(page, rows)),
1829 +             Self::Contiguous(pixels) => pixels.get(start.end).map(|page| read(page, rows)),
     |
```

```text
error[E0425]: cannot find value `rows` in this scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:1829:86
     |
1826 |     fn with_rows<R>(&self, row: u32, _start: usize, _end: usize, _rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
     |                                                                  ----- `_rows` defined here
...
1829 |             Self::Contiguous(pixels) => pixels.get(start..end).map(|page| read(page, rows)),
     |                                                                                      ^^^^
     |
note: function `crate::wgpu::tabs::tests::rows` exists but is inaccessible
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/../../🧱️elements/📑️Tabs/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-unit/🦀️.rs:6:1
     |
   6 | fn rows() -> Vec<TabRow> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^ not accessible
help: the leading underscore in `_rows` marks it as unused, consider renaming it to `rows`
     |
1826 -     fn with_rows<R>(&self, row: u32, _start: usize, _end: usize, _rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
1826 +     fn with_rows<R>(&self, row: u32, _start: usize, _end: usize, rows: u32, read: impl FnOnce(&[u8], u32) -> R) -> Option<R> {
     |
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:130:60
     |
 130 |     let (closed, born, freed) = measured(|| job.close_step(grant(0, capacity)));
     |                                                 ---------- ^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                                 |
     |                                                 arguments to this method are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:131:30
    |
131 |     assert!(matches!(closed, Step::Complete { .. }));
    |                      ------  ^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                      |
    |                      this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
131 |     assert!(matches!(closed, Ok(Step::Complete { .. })));
    |                              +++                     +
```

```text
error[E0599]: no method named `progress` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:133:23
     |
 133 |     assert_eq!(closed.progress(), RetainedCloneProgress { copied_items: 1, released_bytes: capacity, ..Default::default() });
     |                       ^^^^^^^^ method not found in `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
     |
note: the method `progress` exists on the type `InteractiveJobCloseStep`
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1312:5
     |
1312 |     pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}=>progress,Self::Bloc...
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider using `Result::expect` to unwrap the `InteractiveJobCloseStep` value, panicking if the value is a `Result::Err`
     |
 133 |     assert_eq!(closed.expect("REASON").progress(), RetainedCloneProgress { copied_items: 1, released_bytes: capacity, ..Default::default() });
     |                      +++++++++++++++++
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:156:49
     |
 156 | ...ob.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:capacity,maximum_depth:1});a...
     |       ---------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |       |
     |       arguments to this method are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0599]: no method named `progress` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:156:197
     |
 156 | ...ert_eq!(closed.progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert!(job.termi...
     |                   ^^^^^^^^ method not found in `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
     |
note: the method `progress` exists on the type `InteractiveJobCloseStep`
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1312:5
     |
1312 |     pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}=>progress,Self::Bloc...
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider using `Result::expect` to unwrap the `InteractiveJobCloseStep` value, panicking if the value is a `Result::Err`
     |
 156 |     job.begin_close();let closed=job.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:capacity,maximum_depth:1});assert_eq!(closed.expect("REASON").progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert!(job.terminal_is_empty());
     |                                                                                                                                                                                                    +++++++++++++++++
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:211:111
     |
 211 | ...eed)=measured(||session.close_step(supplied));assert!(step.progress().fits(supplied));assert_eq!(step.progress().retained_capac...
     |                            ---------- ^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                            |
     |                            arguments to this method are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:3314:12
     |
3314 |     pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{
     |            ^^^^^^^^^^
```

```text
error[E0599]: no method named `progress` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:211:135
     |
 211 | ...ed));assert!(step.progress().fits(supplied));assert_eq!(step.progress().retained_capacity_bytes,born);assert_eq!(step.progress(...
     |                      ^^^^^^^^ method not found in `std::result::Result<WorkerJobCloseStep, OriginalTurnFailure>`
     |
note: the method `progress` exists on the type `WorkerJobCloseStep`
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2216:5
     |
2216 |     pub fn progress(self) -> RetainedCloneProgress {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider using `Result::expect` to unwrap the `WorkerJobCloseStep` value, panicking if the value is a `Result::Err`
     |
 211 |     for _ in 0..4096 {if session.terminal_is_empty(){break}let(step,born,freed)=measured(||session.close_step(supplied));assert!(step.expect("REASON").progress().fits(supplied));assert_eq!(step.progress().retained_capacity_bytes,born);assert_eq!(step.progress().released_bytes,freed);}
     |                                                                                                                                      +++++++++++++++++
```

```text
error[E0599]: no method named `progress` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:211:178
     |
 211 | ...);assert_eq!(step.progress().retained_capacity_bytes,born);assert_eq!(step.progress().released_bytes,freed);}
     |                      ^^^^^^^^ method not found in `std::result::Result<WorkerJobCloseStep, OriginalTurnFailure>`
     |
note: the method `progress` exists on the type `WorkerJobCloseStep`
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2216:5
     |
2216 |     pub fn progress(self) -> RetainedCloneProgress {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider using `Result::expect` to unwrap the `WorkerJobCloseStep` value, panicking if the value is a `Result::Err`
     |
 211 |     for _ in 0..4096 {if session.terminal_is_empty(){break}let(step,born,freed)=measured(||session.close_step(supplied));assert!(step.progress().fits(supplied));assert_eq!(step.expect("REASON").progress().retained_capacity_bytes,born);assert_eq!(step.progress().released_bytes,freed);}
     |                                                                                                                                                                                 +++++++++++++++++
```

```text
error[E0599]: no method named `progress` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:211:235
     |
 211 | ...);assert_eq!(step.progress().released_bytes,freed);}
     |                      ^^^^^^^^ method not found in `std::result::Result<WorkerJobCloseStep, OriginalTurnFailure>`
     |
note: the method `progress` exists on the type `WorkerJobCloseStep`
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2216:5
     |
2216 |     pub fn progress(self) -> RetainedCloneProgress {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider using `Result::expect` to unwrap the `WorkerJobCloseStep` value, panicking if the value is a `Result::Err`
     |
 211 |     for _ in 0..4096 {if session.terminal_is_empty(){break}let(step,born,freed)=measured(||session.close_step(supplied));assert!(step.progress().fits(supplied));assert_eq!(step.progress().retained_capacity_bytes,born);assert_eq!(step.expect("REASON").progress().released_bytes,freed);}
     |                                                                                                                                                                                                                                          +++++++++++++++++
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:220:121
     |
 220 | ...ssert_eq!(cancelled_job.close_step(supplied).progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default:...
     |                            ---------- ^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                            |
     |                            arguments to this method are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0599]: no method named `progress` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:220:131
     |
 220 | ...step(supplied).progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert!(cancelled...
     |                   ^^^^^^^^ method not found in `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
     |
note: the method `progress` exists on the type `InteractiveJobCloseStep`
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1312:5
     |
1312 |     pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}=>progress,Self::Bloc...
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider using `Result::expect` to unwrap the `InteractiveJobCloseStep` value, panicking if the value is a `Result::Err`
     |
 220 |     let mut cancelled_job=cancelled_job.take().unwrap();cancelled_job.begin_close();assert_eq!(cancelled_job.close_step(supplied).expect("REASON").progress(),RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()});assert!(cancelled_job.terminal_is_empty());
     |                                                                                                                                  +++++++++++++++++
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:57:78
     |
  57 |                     let (result, births, frees) = measured(|| job.close_step(grant(copy, capacity)));
     |                                                                   ---------- ^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                                                   |
     |                                                                   arguments to this method are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:58:46
   |
58 |                     assert!(matches!(result, Step::Blocked));
   |                                      ------  ^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |                                      |
   |                                      this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
58 |                     assert!(matches!(result, Ok(Step::Blocked)));
   |                                              +++             +
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:65:75
     |
  65 |                 let (refused, births, frees) = measured(|| job.close_step(grant(copy, capacity - 1)));
     |                                                                ---------- ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                                                |
     |                                                                arguments to this method are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:66:43
   |
66 |                 assert!(matches!(refused, Step::Pending { progress } if progress == RetainedCloneProgress::default()));
   |                                  -------  ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |                                  |
   |                                  this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
66 |                 assert!(matches!(refused, Ok(Step::Pending { progress }) if progress == RetainedCloneProgress::default()));
   |                                           +++                          +
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:72:76
     |
  72 |                 let (terminal, births, frees) = measured(|| job.close_step(grant(copy, capacity)));
     |                                                                 ---------- ^^^^^^^^^^^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                                                 |
     |                                                                 arguments to this method are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:73:44
   |
73 | ...ches!(terminal, Step::Complete { progress } if progress.copied_items == 1 && progress.copied_bytes == 0 && progress.retained_capa...
   |          --------  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |          |
   |          this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
73 |                 assert!(matches!(terminal, Ok(Step::Complete { progress }) if progress.copied_items == 1 && progress.copied_bytes == 0 && progress.retained_capacity_bytes == births && progress.released_bytes == frees));
   |                                            +++                           +
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:76:74
     |
  76 |                 let (repeat, births, frees) = measured(|| job.close_step(grant(0, 0)));
     |                                                               ---------- ^^^^^^^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                                               |
     |                                                               arguments to this method are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/../../../🧪️tests/♻️physical-job-close/🦀️.rs:77:42
   |
77 |                 assert!(matches!(repeat, Step::Complete { progress } if progress == RetainedCloneProgress::default()));
   |                                  ------  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |                                  |
   |                                  this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
77 |                 assert!(matches!(repeat, Ok(Step::Complete { progress }) if progress == RetainedCloneProgress::default()));
   |                                          +++                           +
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/../../../🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs:671:109
     |
 671 |         let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.close_step(grant));
     |                                                                                                  ---------- ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                                                                                  |
     |                                                                                                  arguments to this method are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0599]: no method named `progress` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/../../../🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs:672:29
     |
 672 |         let progress = step.progress();
     |                             ^^^^^^^^ method not found in `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
     |
note: the method `progress` exists on the type `InteractiveJobCloseStep`
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1312:5
     |
1312 |     pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}=>progress,Self::Bloc...
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider using `Result::expect` to unwrap the `InteractiveJobCloseStep` value, panicking if the value is a `Result::Err`
     |
 672 |         let progress = step.expect("REASON").progress();
     |                            +++++++++++++++++
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:1261:57
     |
1261 |         assert_eq!(InteractiveJob::close_step(&mut job, grant), semio_framework_job::InteractiveJobCloseStep::Blocked);
     |                    --------------------------           ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                    |
     |                    arguments to this function are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0369]: binary operation `==` cannot be applied to type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:1261:9
     |
1261 |         assert_eq!(InteractiveJob::close_step(&mut job, grant), semio_framework_job::InteractiveJobCloseStep::Blocked);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |         |
     |         std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>
     |         InteractiveJobCloseStep
     |
note: `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>` does not implement `PartialEq<InteractiveJobCloseStep>`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
     |
 557 | pub enum Result<T, E> {
     | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>` is defined in another crate
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:1263:60
     |
1263 |         let outcome = InteractiveJob::close_step(&mut job, grant);
     |                       --------------------------           ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                       |
     |                       arguments to this function are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0369]: binary operation `==` cannot be applied to type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:1265:9
     |
1265 |         assert_eq!(outcome, semio_framework_job::InteractiveJobCloseStep::Pending { progress: expected });
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |         |
     |         std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>
     |         InteractiveJobCloseStep
     |
note: `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>` does not implement `PartialEq<InteractiveJobCloseStep>`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
     |
 557 | pub enum Result<T, E> {
     | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>` is defined in another crate
```

```text
error[E0599]: no method named `progress` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:1266:25
     |
1266 |         assert!(outcome.progress().fits(grant));
     |                         ^^^^^^^^ method not found in `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
     |
note: the method `progress` exists on the type `InteractiveJobCloseStep`
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1312:5
     |
1312 |     pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}=>progress,Self::Bloc...
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider using `Result::expect` to unwrap the `InteractiveJobCloseStep` value, panicking if the value is a `Result::Err`
     |
1266 |         assert!(outcome.expect("REASON").progress().fits(grant));
     |                        +++++++++++++++++
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:1275:57
     |
1275 | ...   assert_eq!(InteractiveJob::close_step(&mut job, grant), semio_framework_job::InteractiveJobCloseStep::Complete { progress: s...
     |                  --------------------------           ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                  |
     |                  arguments to this function are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0369]: binary operation `==` cannot be applied to type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:1275:9
     |
1275 | ...   assert_eq!(InteractiveJob::close_step(&mut job, grant), semio_framework_job::InteractiveJobCloseStep::Complete { progress: semio_framework_job::RetainedCloneProgress::default() });
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |       |
     |       std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>
     |       InteractiveJobCloseStep
     |
note: `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>` does not implement `PartialEq<InteractiveJobCloseStep>`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
     |
 557 | pub enum Result<T, E> {
     | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>` is defined in another crate
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:755:20
     |
 755 |     assert!(!first.close_step());
     |                    ^^^^^^^^^^-- argument #1 of type `semio_framework_value::RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1432:12
     |
1432 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
     |            ^^^^^^^^^^            -------------------------
help: provide the argument
     |
 755 |     assert!(!first.close_step(/* semio_framework_value::RetainedCloneGrant */));
     |                               +++++++++++++++++++++++++++++++++++++++++++++++
```

```text
error[E0600]: cannot apply unary operator `!` to type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:755:13
    |
755 |     assert!(!first.close_step());
    |             ^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
    |
note: `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` does not implement `Not`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
    |
557 | pub enum Result<T, E> {
    | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` is defined in another crate
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:758:18
     |
 758 |     while !first.close_step() {}
     |                  ^^^^^^^^^^-- argument #1 of type `semio_framework_value::RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1432:12
     |
1432 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
     |            ^^^^^^^^^^            -------------------------
help: provide the argument
     |
 758 |     while !first.close_step(/* semio_framework_value::RetainedCloneGrant */) {}
     |                             +++++++++++++++++++++++++++++++++++++++++++++++
```

```text
error[E0600]: cannot apply unary operator `!` to type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:758:11
    |
758 |     while !first.close_step() {}
    |           ^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
    |
note: `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` does not implement `Not`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
    |
557 | pub enum Result<T, E> {
    | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` is defined in another crate
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:764:19
     |
 764 |     while !second.close_step() {}
     |                   ^^^^^^^^^^-- argument #1 of type `semio_framework_value::RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1432:12
     |
1432 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
     |            ^^^^^^^^^^            -------------------------
help: provide the argument
     |
 764 |     while !second.close_step(/* semio_framework_value::RetainedCloneGrant */) {}
     |                              +++++++++++++++++++++++++++++++++++++++++++++++
```

```text
error[E0600]: cannot apply unary operator `!` to type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:764:11
    |
764 |     while !second.close_step() {}
    |           ^^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
    |
note: `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` does not implement `Not`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
    |
557 | pub enum Result<T, E> {
    | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` is defined in another crate
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:25:90
     |
  25 |             let (step, births, frees) = measured(|| InteractiveJob::close_step(&mut job, grant));
     |                                                     --------------------------           ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                                     |
     |                                                     arguments to this function are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:27:17
   |
26 |             let progress = match step {
   |                                  ---- this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
27 |                 Close::Pending { progress } | Close::Complete { progress } => progress,
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
27 |                 Ok(Close::Pending { progress }) | Close::Complete { progress } => progress,
   |                 +++                           +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:27:47
   |
26 |             let progress = match step {
   |                                  ---- this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
27 |                 Close::Pending { progress } | Close::Complete { progress } => progress,
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
27 |                 Close::Pending { progress } | Ok(Close::Complete { progress }) => progress,
   |                                               +++                            +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:28:17
   |
26 |             let progress = match step {
   |                                  ---- this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
27 |                 Close::Pending { progress } | Close::Complete { progress } => progress,
28 |                 Close::Blocked => panic!("requested prepared close was blocked"),
   |                 ^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
28 |                 Ok(Close::Blocked) => panic!("requested prepared close was blocked"),
   |                 +++              +
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:29:24
   |
29 |                 Close::Refused { kind, progress } => panic!("admitted prepared close refused: {kind:?}, progress: {progress:?}"),
   |                        ^^^^^^^ variant not found in `InteractiveJobCloseStep`
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:33:31
   |
33 |             if matches!(step, Close::Complete { .. }) { complete = true; break; }
   |                         ----  ^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |                         |
   |                         this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
33 |             if matches!(step, Ok(Close::Complete { .. })) { complete = true; break; }
   |                               +++                      +
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:67:90
     |
  67 |             let (step, births, frees) = measured(|| InteractiveJob::close_step(&mut job, grant));
     |                                                     --------------------------           ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                                     |
     |                                                     arguments to this function are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:68:36
   |
68 | ...atches!(step, Close::Pending { progress } if progress == Default::default()) || matches!(step, Close::Refused { kind: semio_frame...
   |            ----  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |            |
   |            this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
68 |             assert!(matches!(step, Ok(Close::Pending { progress }) if progress == Default::default()) || matches!(step, Close::Refused { kind: semio_framework_value::ValueRefusalKind::DepthLimit, progress } if progress == Default::default()));
   |                                    +++                           +
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:68:124
   |
68 | ...:default()) || matches!(step, Close::Refused { kind: semio_framework_value::ValueRefusalKind::DepthLimit, progress } if progress ...
   |                                         ^^^^^^^ variant not found in `InteractiveJobCloseStep`
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:78:90
     |
  78 |             let (step, births, frees) = measured(|| InteractiveJob::close_step(&mut job, grant));
     |                                                     --------------------------           ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                                     |
     |                                                     arguments to this function are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:79:41
   |
79 | ...= match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("funded atlas close failed:...
   |            ----   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |            |
   |            this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
79 |             let progress = match step { Ok(Close::Pending { progress }) | Close::Complete { progress } => progress, step => panic!("funded atlas close failed: {step:?}") };
   |                                         +++                           +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:79:71
   |
79 | ...    let progress = match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("funded at...
   |                             ----                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |                             |
   |                             this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
79 |             let progress = match step { Close::Pending { progress } | Ok(Close::Complete { progress }) => progress, step => panic!("funded atlas close failed: {step:?}") };
   |                                                                       +++                            +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:84:31
   |
84 |             if matches!(step, Close::Complete { .. }) { complete = true; break; }
   |                         ----  ^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |                         |
   |                         this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
84 |             if matches!(step, Ok(Close::Complete { .. })) { complete = true; break; }
   |                               +++                      +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:110:36
    |
110 |             assert!(matches!(step, Close::Pending { progress } if progress == Default::default()));
    |                              ----  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                              |
    |                              this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
110 |             assert!(matches!(step, Ok(Close::Pending { progress }) if progress == Default::default()));
    |                                    +++                           +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:122:40
    |
122 |                 assert!(matches!(step, Close::Pending { progress } if progress == Default::default()));
    |                                  ----  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                                  |
    |                                  this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
122 |                 assert!(matches!(step, Ok(Close::Pending { progress }) if progress == Default::default()));
    |                                        +++                           +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:127:41
    |
127 | ... match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("original abandoned input f...
    |           ----   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |           |
    |           this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
127 |             let progress = match step { Ok(Close::Pending { progress }) | Close::Complete { progress } => progress, step => panic!("original abandoned input failed: {step:?}") };
    |                                         +++                           +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:127:71
    |
127 | ...    let progress = match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("original...
    |                             ----                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                             |
    |                             this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
127 |             let progress = match step { Close::Pending { progress } | Ok(Close::Complete { progress }) => progress, step => panic!("original abandoned input failed: {step:?}") };
    |                                                                       +++                            +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:131:31
    |
131 |             if matches!(step, Close::Complete { .. }) { complete = true; break; }
    |                         ----  ^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                         |
    |                         this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
131 |             if matches!(step, Ok(Close::Complete { .. })) { complete = true; break; }
    |                               +++                      +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:148:32
    |
148 |         assert!(matches!(step, Close::Pending { progress } if progress == Default::default()));
    |                          ----  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                          |
    |                          this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
148 |         assert!(matches!(step, Ok(Close::Pending { progress }) if progress == Default::default()));
    |                                +++                           +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:156:41
    |
156 | ... match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("original abandoned atlas f...
    |           ----   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |           |
    |           this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
156 |             let progress = match step { Ok(Close::Pending { progress }) | Close::Complete { progress } => progress, step => panic!("original abandoned atlas failed: {step:?}") };
    |                                         +++                           +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:156:71
    |
156 | ...    let progress = match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("original...
    |                             ----                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                             |
    |                             this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
156 |             let progress = match step { Close::Pending { progress } | Ok(Close::Complete { progress }) => progress, step => panic!("original abandoned atlas failed: {step:?}") };
    |                                                                       +++                            +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:160:31
    |
160 |             if matches!(step, Close::Complete { .. }) { complete = true; break; }
    |                         ----  ^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                         |
    |                         this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
160 |             if matches!(step, Ok(Close::Complete { .. })) { complete = true; break; }
    |                               +++                      +
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:177:20
    |
177 |         let Close::Refused { kind, progress: received } = step else { panic!("original refusal changed: {step:?}") };
    |                    ^^^^^^^ variant not found in `InteractiveJobCloseStep`
```

```text
error[E0599]: no method named `progress` found for enum `std::result::Result<T, E>` in the current scope
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:181:25
     |
 181 |         assert_eq!(step.progress(), received);
     |                         ^^^^^^^^ method not found in `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
     |
note: the method `progress` exists on the type `InteractiveJobCloseStep`
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1312:5
     |
1312 |     pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}=>progress,Self::Bloc...
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider using `Result::expect` to unwrap the `InteractiveJobCloseStep` value, panicking if the value is a `Result::Err`
     |
 181 |         assert_eq!(step.expect("REASON").progress(), received);
     |                        +++++++++++++++++
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:236:90
     |
 236 |             let (step, births, frees) = measured(|| InteractiveJob::close_step(&mut job, grant));
     |                                                     --------------------------           ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |                                                     |
     |                                                     arguments to this function are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:237:41
    |
237 | ... match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("prepared funded semantic c...
    |           ----   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |           |
    |           this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
237 |             let progress = match step { Ok(Close::Pending { progress }) | Close::Complete { progress } => progress, step => panic!("prepared funded semantic close failed: {step:?}") };
    |                                         +++                           +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:237:71
    |
237 | ...    let progress = match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("prepared...
    |                             ----                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                             |
    |                             this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
237 |             let progress = match step { Close::Pending { progress } | Ok(Close::Complete { progress }) => progress, step => panic!("prepared funded semantic close failed: {step:?}") };
    |                                                                       +++                            +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:240:31
    |
240 |             if matches!(step, Close::Complete { .. }) { closed = true; break }
    |                         ----  ^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                         |
    |                         this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
240 |             if matches!(step, Ok(Close::Complete { .. })) { closed = true; break }
    |                               +++                      +
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:325:21
     |
 325 |     while !producer.close_step() {}
     |                     ^^^^^^^^^^-- argument #1 of type `semio_framework_value::RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1432:12
     |
1432 |     pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
     |            ^^^^^^^^^^            -------------------------
help: provide the argument
     |
 325 |     while !producer.close_step(/* semio_framework_value::RetainedCloneGrant */) {}
     |                                +++++++++++++++++++++++++++++++++++++++++++++++
```

```text
error[E0600]: cannot apply unary operator `!` to type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:325:11
    |
325 |     while !producer.close_step() {}
    |           ^^^^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
    |
note: `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` does not implement `Not`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
    |
557 | pub enum Result<T, E> {
    | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` is defined in another crate
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:337:23
     |
 337 |     assert!(!rejected.close_step(), "one rejection grant retires one source page only");
     |                       ^^^^^^^^^^-- argument #1 of type `semio_framework_value::RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1146:12
     |
1146 |     pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
     |            ^^^^^^^^^^           ------------------------
help: provide the argument
     |
 337 |     assert!(!rejected.close_step(/* semio_framework_value::RetainedCloneGrant */), "one rejection grant retires one source page only");
     |                                  +++++++++++++++++++++++++++++++++++++++++++++++
```

```text
error[E0600]: cannot apply unary operator `!` to type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:337:13
    |
337 |     assert!(!rejected.close_step(), "one rejection grant retires one source page only");
    |             ^^^^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
    |
note: `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` does not implement `Not`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
    |
557 | pub enum Result<T, E> {
    | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` is defined in another crate
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:338:21
     |
 338 |     while !rejected.close_step() {}
     |                     ^^^^^^^^^^-- argument #1 of type `semio_framework_value::RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1146:12
     |
1146 |     pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
     |            ^^^^^^^^^^           ------------------------
help: provide the argument
     |
 338 |     while !rejected.close_step(/* semio_framework_value::RetainedCloneGrant */) {}
     |                                +++++++++++++++++++++++++++++++++++++++++++++++
```

```text
error[E0600]: cannot apply unary operator `!` to type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:338:11
    |
338 |     while !rejected.close_step() {}
    |           ^^^^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
    |
note: `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` does not implement `Not`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
    |
557 | pub enum Result<T, E> {
    | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` is defined in another crate
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:353:89
    |
353 |     while !matches!(admitted.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }) {
    |                     ------------------------------------------------------------------  ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                     |
    |                     this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
353 |     while !matches!(admitted.close_original_step(atlas_fixture_authority().retirement), Ok(CloseStep::Complete { .. })) {
    |                                                                                         +++                          +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:371:88
    |
371 |     assert!(!matches!(pages.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }));
    |                       ---------------------------------------------------------------  ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                       |
    |                       this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
371 |     assert!(!matches!(pages.close_original_step(atlas_fixture_authority().retirement), Ok(CloseStep::Complete { .. })));
    |                                                                                        +++                          +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:375:86
    |
375 |     while !matches!(pages.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }) {
    |                     ---------------------------------------------------------------  ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                     |
    |                     this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
375 |     while !matches!(pages.close_original_step(atlas_fixture_authority().retirement), Ok(CloseStep::Complete { .. })) {
    |                                                                                      +++                          +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:395:90
    |
395 |         while !matches!(owner.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }) {}
    |                         ---------------------------------------------------------------  ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                         |
    |                         this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
395 |         while !matches!(owner.close_original_step(atlas_fixture_authority().retirement), Ok(CloseStep::Complete { .. })) {}
    |                                                                                          +++                          +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:431:88
    |
431 |     assert!(!matches!(owner.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }));
    |                       ---------------------------------------------------------------  ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                       |
    |                       this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
431 |     assert!(!matches!(owner.close_original_step(atlas_fixture_authority().retirement), Ok(CloseStep::Complete { .. })));
    |                                                                                        +++                          +
```

```text
error[E0308]: mismatched types
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:459:98
    |
459 |                 while !matches!(owner.close_original_step(atlas_fixture_authority().retirement), CloseStep::Complete { .. }) {}
    |                                 ---------------------------------------------------------------  ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
    |                                 |
    |                                 this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
    |
    = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
               found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
    |
459 |                 while !matches!(owner.close_original_step(atlas_fixture_authority().retirement), Ok(CloseStep::Complete { .. })) {}
    |                                                                                                  +++                          +
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:481:21
     |
 481 |     while !rejected.close_step() {}
     |                     ^^^^^^^^^^-- argument #1 of type `semio_framework_value::RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1146:12
     |
1146 |     pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
     |            ^^^^^^^^^^           ------------------------
help: provide the argument
     |
 481 |     while !rejected.close_step(/* semio_framework_value::RetainedCloneGrant */) {}
     |                                +++++++++++++++++++++++++++++++++++++++++++++++
```

```text
error[E0600]: cannot apply unary operator `!` to type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:481:11
    |
481 |     while !rejected.close_step() {}
    |           ^^^^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
    |
note: `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` does not implement `Not`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
    |
557 | pub enum Result<T, E> {
    | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` is defined in another crate
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:487:21
     |
 487 |     while !rejected.close_step() {}
     |                     ^^^^^^^^^^-- argument #1 of type `semio_framework_value::RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1146:12
     |
1146 |     pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
     |            ^^^^^^^^^^           ------------------------
help: provide the argument
     |
 487 |     while !rejected.close_step(/* semio_framework_value::RetainedCloneGrant */) {}
     |                                +++++++++++++++++++++++++++++++++++++++++++++++
```

```text
error[E0600]: cannot apply unary operator `!` to type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:487:11
    |
487 |     while !rejected.close_step() {}
    |           ^^^^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
    |
note: `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` does not implement `Not`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
    |
557 | pub enum Result<T, E> {
    | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` is defined in another crate
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:505:21
     |
 505 |     while !rejected.close_step() {}
     |                     ^^^^^^^^^^-- argument #1 of type `semio_framework_value::RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1146:12
     |
1146 |     pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
     |            ^^^^^^^^^^           ------------------------
help: provide the argument
     |
 505 |     while !rejected.close_step(/* semio_framework_value::RetainedCloneGrant */) {}
     |                                +++++++++++++++++++++++++++++++++++++++++++++++
```

```text
error[E0600]: cannot apply unary operator `!` to type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:505:11
    |
505 |     while !rejected.close_step() {}
    |           ^^^^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
    |
note: `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` does not implement `Not`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
    |
557 | pub enum Result<T, E> {
    | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` is defined in another crate
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:510:21
     |
 510 |     while !rejected.close_step() {}
     |                     ^^^^^^^^^^-- argument #1 of type `semio_framework_value::RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1146:12
     |
1146 |     pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
     |            ^^^^^^^^^^           ------------------------
help: provide the argument
     |
 510 |     while !rejected.close_step(/* semio_framework_value::RetainedCloneGrant */) {}
     |                                +++++++++++++++++++++++++++++++++++++++++++++++
```

```text
error[E0600]: cannot apply unary operator `!` to type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:510:11
    |
510 |     while !rejected.close_step() {}
    |           ^^^^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
    |
note: `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` does not implement `Not`
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:557:1
    |
557 | pub enum Result<T, E> {
    | ^^^^^^^^^^^^^^^^^^^^^ `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>` is defined in another crate
```

```text
error[E0308]: mismatched types
 --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:7:9
  |
6 |     match packet.close_step(grant) {
  |           ------------------------ this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
7 |         Close::Complete { progress } => { assert!(progress.fits(grant)); assert!(packet.retirement_is_empty()); true },
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
  |
  = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
             found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
  |
7 |         Ok(Close::Complete { progress }) => { assert!(progress.fits(grant)); assert!(packet.retirement_is_empty()); true },
  |         +++                            +
```

```text
error[E0308]: mismatched types
 --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:8:9
  |
6 |     match packet.close_step(grant) {
  |           ------------------------ this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
7 |         Close::Complete { progress } => { assert!(progress.fits(grant)); assert!(packet.retirement_is_empty()); true },
8 |         Close::Pending { progress } => { assert!(progress.fits(grant)); false },
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
  |
  = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
             found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
  |
8 |         Ok(Close::Pending { progress }) => { assert!(progress.fits(grant)); false },
  |         +++                           +
```

```text
error[E0308]: mismatched types
 --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:9:9
  |
6 |     match packet.close_step(grant) {
  |           ------------------------ this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
...
9 |         Close::Blocked => false,
  |         ^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
  |
  = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
             found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
  |
9 |         Ok(Close::Blocked) => false,
  |         +++              +
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:10:16
   |
10 |         Close::Refused { kind, progress } => panic!("original packet refused admitted close: {kind:?}, progress: {progress:?}"),
   |                ^^^^^^^ variant not found in `InteractiveJobCloseStep`
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:17:9
   |
16 |     match PreparedRenderPacket::close_abandoned_step(grant) {
   |           ------------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
17 |         Close::Complete { progress } => { assert!(progress.fits(grant)); true },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
17 |         Ok(Close::Complete { progress }) => { assert!(progress.fits(grant)); true },
   |         +++                            +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:18:9
   |
16 |     match PreparedRenderPacket::close_abandoned_step(grant) {
   |           ------------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
17 |         Close::Complete { progress } => { assert!(progress.fits(grant)); true },
18 |         Close::Pending { progress } => { assert!(progress.fits(grant)); false },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
18 |         Ok(Close::Pending { progress }) => { assert!(progress.fits(grant)); false },
   |         +++                           +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:19:9
   |
16 |     match PreparedRenderPacket::close_abandoned_step(grant) {
   |           ------------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
...
19 |         Close::Blocked => false,
   |         ^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
19 |         Ok(Close::Blocked) => false,
   |         +++              +
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:20:16
   |
20 | ...   Close::Refused { kind, progress } => panic!("original abandoned packet refused admitted close: {kind:?}, progress: {progress:?...
   |              ^^^^^^^ variant not found in `InteractiveJobCloseStep`
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:27:9
   |
26 |     match PreparedRenderReceiver::close_abandoned_step(grant) {
   |           --------------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
27 |         Close::Complete { progress } => { assert!(progress.fits(grant)); true },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
27 |         Ok(Close::Complete { progress }) => { assert!(progress.fits(grant)); true },
   |         +++                            +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:28:9
   |
26 |     match PreparedRenderReceiver::close_abandoned_step(grant) {
   |           --------------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
27 |         Close::Complete { progress } => { assert!(progress.fits(grant)); true },
28 |         Close::Pending { progress } => { assert!(progress.fits(grant)); false },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
28 |         Ok(Close::Pending { progress }) => { assert!(progress.fits(grant)); false },
   |         +++                           +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:29:9
   |
26 |     match PreparedRenderReceiver::close_abandoned_step(grant) {
   |           --------------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
...
29 |         Close::Blocked => false,
   |         ^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
29 |         Ok(Close::Blocked) => false,
   |         +++              +
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:30:16
   |
30 | ...   Close::Refused { kind, progress } => panic!("original abandoned mailbox refused admitted close: {kind:?}, progress: {progress:...
   |              ^^^^^^^ variant not found in `InteractiveJobCloseStep`
```

```text
error[E0308]: mismatched types
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:39:43
     |
  39 |     match InteractiveJob::close_step(job, grant) {
     |           --------------------------      ^^^^^ expected `&mut StepContext<'_>`, found `RetainedCloneGrant`
     |           |
     |           arguments to this function are incorrect
     |
note: method defined here
    --> 🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:1345:8
     |
1345 |     fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
     |        ^^^^^^^^^^
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:40:9
   |
39 |     match InteractiveJob::close_step(job, grant) {
   |           -------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
40 |         Close::Complete { progress } => { assert!(progress.fits(grant)); assert!(job.terminal_is_empty()); true },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
40 |         Ok(Close::Complete { progress }) => { assert!(progress.fits(grant)); assert!(job.terminal_is_empty()); true },
   |         +++                            +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:41:9
   |
39 |     match InteractiveJob::close_step(job, grant) {
   |           -------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
40 |         Close::Complete { progress } => { assert!(progress.fits(grant)); assert!(job.terminal_is_empty()); true },
41 |         Close::Pending { progress } => { assert!(progress.fits(grant)); false },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
41 |         Ok(Close::Pending { progress }) => { assert!(progress.fits(grant)); false },
   |         +++                           +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:42:9
   |
39 |     match InteractiveJob::close_step(job, grant) {
   |           -------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
...
42 |         Close::Blocked => false,
   |         ^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, OriginalTurnFailure>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
42 |         Ok(Close::Blocked) => false,
   |         +++              +
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:43:16
   |
43 | ...   Close::Refused { kind, progress } => panic!("original prepared owner refused admitted close: {kind:?}, progress: {progress:?}"),
   |              ^^^^^^^ variant not found in `InteractiveJobCloseStep`
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:50:9
   |
49 |     match PreparedRenderJob::close_abandoned_step(grant) {
   |           ---------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
50 |         Close::Complete { progress } => { assert!(progress.fits(grant)); true },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
50 |         Ok(Close::Complete { progress }) => { assert!(progress.fits(grant)); true },
   |         +++                            +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:51:9
   |
49 |     match PreparedRenderJob::close_abandoned_step(grant) {
   |           ---------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
50 |         Close::Complete { progress } => { assert!(progress.fits(grant)); true },
51 |         Close::Pending { progress } => { assert!(progress.fits(grant)); false },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
51 |         Ok(Close::Pending { progress }) => { assert!(progress.fits(grant)); false },
   |         +++                           +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:52:9
   |
49 |     match PreparedRenderJob::close_abandoned_step(grant) {
   |           ---------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
...
52 |         Close::Blocked => false,
   |         ^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
52 |         Ok(Close::Blocked) => false,
   |         +++              +
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:53:16
   |
53 | ...   Close::Refused { kind, progress } => panic!("original abandoned owner refused admitted close: {kind:?}, progress: {progress:?}"),
   |              ^^^^^^^ variant not found in `InteractiveJobCloseStep`
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:61:9
   |
60 |     match PreparedRenderInput::close_abandoned_step(grant) {
   |           ------------------------------------------------ this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
61 |         Close::Complete { progress } => { assert!(progress.fits(grant)); true },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
61 |         Ok(Close::Complete { progress }) => { assert!(progress.fits(grant)); true },
   |         +++                            +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:62:9
   |
60 |     match PreparedRenderInput::close_abandoned_step(grant) {
   |           ------------------------------------------------ this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
61 |         Close::Complete { progress } => { assert!(progress.fits(grant)); true },
62 |         Close::Pending { progress } => { assert!(progress.fits(grant)); false },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
62 |         Ok(Close::Pending { progress }) => { assert!(progress.fits(grant)); false },
   |         +++                           +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:63:9
   |
60 |     match PreparedRenderInput::close_abandoned_step(grant) {
   |           ------------------------------------------------ this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
...
63 |         Close::Blocked => false,
   |         ^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
63 |         Ok(Close::Blocked) => false,
   |         +++              +
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:64:16
   |
64 | ...   Close::Refused { kind, progress } => panic!("original abandoned input refused admitted close: {kind:?}, progress: {progress:?}"),
   |              ^^^^^^^ variant not found in `InteractiveJobCloseStep`
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:93:9
   |
92 |     match PreparedAtlasPages::close_abandoned_step(grant) {
   |           ----------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
93 |         Close::Complete { progress } => { assert!(progress.fits(grant)); true },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
93 |         Ok(Close::Complete { progress }) => { assert!(progress.fits(grant)); true },
   |         +++                            +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:94:9
   |
92 |     match PreparedAtlasPages::close_abandoned_step(grant) {
   |           ----------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
93 |         Close::Complete { progress } => { assert!(progress.fits(grant)); true },
94 |         Close::Pending { progress } => { assert!(progress.fits(grant)); false },
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
94 |         Ok(Close::Pending { progress }) => { assert!(progress.fits(grant)); false },
   |         +++                           +
```

```text
error[E0308]: mismatched types
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:95:9
   |
92 |     match PreparedAtlasPages::close_abandoned_step(grant) {
   |           ----------------------------------------------- this expression has type `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
...
95 |         Close::Blocked => false,
   |         ^^^^^^^^^^^^^^ expected `Result<InteractiveJobCloseStep, _>`, found `InteractiveJobCloseStep`
   |
   = note: expected enum `std::result::Result<InteractiveJobCloseStep, semio_framework_value::ValueError>`
              found enum `InteractiveJobCloseStep`
help: try wrapping the pattern in `Ok`
   |
95 |         Ok(Close::Blocked) => false,
   |         +++              +
```

```text
error[E0599]: no variant named `Refused` found for enum `InteractiveJobCloseStep`
  --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:96:16
   |
96 | ...   Close::Refused { kind, progress } => panic!("original abandoned atlas refused admitted close: {kind:?}, progress: {progress:?}"),
   |              ^^^^^^^ variant not found in `InteractiveJobCloseStep`
```

```text
error: could not compile `semio-framework-ui` (lib test) due to 116 previous errors; 140 warnings emitted
```

```text
error: command `/Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/cargo test --no-run --message-format json-render-diagnostics --package semio-framework-ui --lib --features wgpu-engine --offline` exited with code 101
38 |       child.once("close", (code, signal) => accept({ code, signal }));
39 |     });
40 |     const failure = reason === "timeout" ? `${command} exceeded its ${options.budgetMs}ms budget` : reason ? `${command} ${reason}` : signal ? `${command} was killed by ${signal}` : code !== 0 ? `${command} exited with status ${code}` : undefined;
41 |     if (!failure) return;
42 |     if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}`);
43 |     if (options.throwOnFailure) throw Error(failure);
                                           ^
```

```text
error: cargo exited with status 101
      at runBudgetedTestCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts:43:39)

Bun v1.3.14 (macOS arm64)
142 |     stopProgress();
143 |     await Promise.race([Promise.all([...pending]), interrupted]);
144 |     await Promise.all([process.stdout,process.stderr].map(output=>new Promise<void>((accept,reject)=>output.write("",error=>error?reject(error):accept()))));
145 |     if (publicationRefused) throw publicationFailure;
146 |     if (stopped) throw new Error(`${label} stopped: ${stopped}`);
147 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: native:owner-command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},

      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:147:60)

Bun v1.3.14 (macOS arm64)



 NX   Running target test-original-presented-source-native for project @semio-tech/ui-rs failed

Failed tasks:

- @semio-tech/ui-rs:test-original-presented-source-native:prototype

Hint: run the command with --verbose for more details.

  Run duration:      35.0s
  Cache:             Skipped (--skip-nx-cache)
  Critical path:     34.6s (1 task)
  Recoverable time:  <1ms

  Recommendations:
    - Cache: drop --skip-nx-cache to restore unchanged tasks instantly.
    - Speed up or split the longest tasks on the critical path:
        @semio-tech/ui-rs:test-original-presented-source-native:prototype    34.6s
```

```text
error: script "nx" exited with code 1
```
