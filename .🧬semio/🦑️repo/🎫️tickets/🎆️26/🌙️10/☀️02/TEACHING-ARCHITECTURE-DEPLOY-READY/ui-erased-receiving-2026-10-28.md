# Genuine Erased UI Locale Receiving Laws

## Baseline

The existing named Rust law declared_axes_and_labels_receive_actual_erased_sources only constrained Locale, Terminology and LocalizedLabel to NativeReceive<dyn NativeValueSource>. It directly called NativeScalarValue::read for the two locale and two terminology strings. It never called begin_receive, advance, take, cursor closure, output retirement or source closure. Its old name did not establish actual funded erased receiving.

The actual mounted path is locale/🧾️value/🦀️.rs → locale_terminology_value_round_trip_tests → locale/🧪️tests/🌐️axis-value/🦀️.rs, under package **semio-framework-ui-locale** in the framework workspace.

## Strengthened Candidate

The existing named law now drives nine language-neutral cases: locale en/de, terminology native/reuse, and all five valid existing label matrices (ordinary, quoted text, locale-invariant text, empty cells and Unicode).

Axis inputs are real owned Strings. Label inputs are typed original records holding their actual String fields. Their NativeValueSource implementations lend references directly to those fields; pointer assertions check original native.en and reuse.de children. The new receiving path does not convert them to DslValue or construct unrelated retained references. RetainedCloneSource::admit_owned supplies the real retained source header, and the erased projection borrows that same source.

For every case, the law performs an actual successful T::begin_receive / NativeReceiver::demands / advance / take sequence. A scoped NativeDecodeConstructionRecipient checks each real grant and accepted receipt during advancement. It then measures the paid cursor close, paid output retirement, and paid source retirement through the existing first-party controllers. The original source contents remain equal after every denied and admitted advance and before final closure.

The successful run first determines the actual number of receiving frontiers. A fresh source and actual receiver are then cancelled at **every prefix from zero through output-ready before transfer**. Each partial or complete untransferred output must finish measured funded closure, with no hidden allocation on final cursor drop. The fixture bounds body size and turns at 4096.

Denials reduce items or an actually positive quoted copy/capacity/release/depth currency. Birth refusal must produce no physical allocation. Advance and take denials preserve readiness, demand and original contents with zero progress/heap work. Receiver closure is borrowed through the canonical ErasedSnapshotRetirement port and select_retirement_cursor: the immutable RetirementSelection admits all currencies before the actual NativeReceiver close_step. This avoids deriving a new action from a reduced copy grant. Source/output controlled retirement is paid with the same 4096-byte body policy, and progress must fit every admitted grant.

Physical allocation measurements reuse the existing locale allocator witness. The only wiring changes outside test files are cfg(test)-only reexports; production behavior and runtime dependencies are unchanged.

## Portable Validation

Schema and fixture were authored before the driver. A new Bun oracle validates the closed fixture with Ajv. Independent Intl parsing checks locale outputs; Lodash lookup checks canonical terminology data; jsonc-parser plus Lodash matrix projection checks the same label outputs against the existing first-party TS implementation.

The final portable run completed with **exit 0, 1 passed, 0 failed, 29 assertions**. Evidence: 🗑️generated/ui-erased-receiving-oracle.log. An initial schema count incorrectly allowed seven cases while the existing corpus contained five valid matrices; the schema was corrected to its actual nine cases before the passing run.

**Rust execution is pending.** No compiler or application was started by this agent for this task. Parent owns the single actual Rust run after the other native compiler settles. The relevant filter is declared_axes_and_labels_receive_actual_erased_sources under framework package semio-framework-ui-locale --lib; require one actual named test, not a zero-test filter. These are source and portable-oracle results, not a Rust pass claim.

## Changed Files

Under 🧰️framework/🔨️modules/🖱️ui/🌐️locale/:

- 🧪️tests/🌐️axis-value/🦀️.rs — existing named law and shared typed receiving driver.
- 🧪️tests/🌐️axis-value/🔣️.json — new neutral nine-case corpus and frontier policies.
- 🧪️tests/🌐️axis-value/📋️schema.json — new closed corpus schema.
- 🧪️tests/🌐️axis-value/🟦️.ts — new independent portable oracle.
- 🧪️tests/♻️localized-label-retirement/🦀️.rs — existing observed allocator helper made crate-visible for test reuse.
- 🏷️label/♻️retirement/🦀️.rs — cfg(test)-only witness reexport.
- 🏷️label/🦀️.rs — cfg(test)-only witness reexport to the sibling axis law.

No broad production edit, source restore, modifying Git command, external runtime dependency, fake Script Original or compiler job was introduced.

## Actual Rust Result

Parent reports authoritative UI handle75081 exit0: the actual named strengthened law executed **1passed/0failed/16filtered**, release compilation13.41seconds and runtime0.22seconds, retained in generated/ui-erased-receiving.log. This supersedes the pending Rust status above. No additional source change was needed after Source readiness.

## Independent Strengthened Law Review

Independently inspected actual final Rust loops/nine-case fixture/oracle and both retained logs. Actual Rust log executes the exact named strengthened law1pass/0fail/16filtered/0.22s then repo stagecomplete; parent supplies authoritative75081exit0. Portable log preserves initial schema RED then actual1pass/0fail/29assertions. This supersedes old25385 scalar-only scope; no gate/Source edit/namespace poll in this review.

Nine cases actually dispatch into real generic NativeReceive paths:2String locale,2String terminology and5typed label records. OriginalLabels/OriginalLanguages implement record ports lending actual nested String fields; pointer assertions directly prove two representative leaf references, while all4cells are received and output-compared. RetainedCloneSource admission and borrow().project(...as_source) retain that same Original; serialization only supplies comparison values and is not substituted ingress. No mock helper, Proxy or fabricated Original is used.

Successful path really begins, quotes/advances/takes receiver output and compares actual serialized output. every_prefix obtains successfulturn count then repeats0..=turns, so cancellation includes birth-only and output-ready BEFORE transfer. Source equality is asserted throughout. Birth/advance/take and selected receiver close deny items and each POSITIVE quoted currency; denial preserves progress/readiness/demand/source/physical heap as applicable. select_retirement_cursor fixes the BODY action before step; it does not requote raw close against reduced copy. Measured successful operations compare exact capacity/release receipt to allocator observation; receiver final drop is zeroheap. Output and retained source both undergo real funded ControlledRetirement to terminal.

Limits remain precise: source/output retirement is FUNDED and measured, but all-currency denial matrix is specifically receiver birth/advance/take/selectedclose, not every source/output retirement phase. Pointer witnesses explicitly cover2leafs, not every nested pointer. Invalid label/axis inputs and the other16Rust tests are outside this filter. Portable Intl/Lodash/jsonc-parser/Ajv validate nine expected outputs/schema independently; they do not run the Rust cursor or replace its allocation evidence. No concrete discrepancy found between strengthened claims and actual loops with these boundaries.
