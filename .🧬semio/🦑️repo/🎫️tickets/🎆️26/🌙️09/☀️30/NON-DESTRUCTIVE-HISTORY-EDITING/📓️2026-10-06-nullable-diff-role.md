# Typed Nullable Diff Role

The current Puzzle5d transform compilation executed zero editor assertions and failed with9E0277 diagnostics. Its sparse delta contains three authored `Option<Option<T>>` fields, preserving omission, explicit clearing and replacement. The generic record derive strips one outer optional, so its inner `Option<T>` needs a field role. Flattening both layers would destroy clearing semantics.

The shared field implementation now uses the existing typed braced record vocabulary: an empty block clears; a block with its optional `value` field replaces; the outer delta omission remains distinct. Owned and borrowed schema producers preserve the actual inner type. Source projection borrows the original inner field, controlled construction and projection use the existing cumulative allocation/cancellation controls, and retirement delegates to the inner owner. No new dynamic carrier or representation primitive is introduced. The original JSON sparse-delta schema and values remain unchanged.

The authored neutral JSON contains omitted, cleared and replaced rows plus wrong-type, unknown-field and null-block refusals. Independent Ajv admission and standard JSON roundtrip validate the exact typed physical records; the shared native serde_json law exercises the same corpus through ordinary Document projection/reconstruction, controlled projection/reconstruction, zero-allocation borrowed measurement and cancellation. The5d native law also checks text/binary parity for label intent and both catalog clear fields.

TDD source red executed **34passed/1failed/826assertions/4.70s** on the missing optional field module after all independent neutral rows passed. An earlier red invocation was an erroneous source import path introduced while adding the new law, corrected before the actual feature red; it is not cited as TDD evidence. Current source green, syntax and narrow shared native captures are pending. Core released only this DSL/schema scope; broad editor matrix reruns remain held for its retained fold stability boundary.

Canonical launch seed rows900.0580995/6/7 expose the5dScale, shared nullable delta and5dnullable diff native controls. The actual registry owner will publish those along with the current Core/UI rows.

Current independent source green: **35passed/0failed/826assertions/14.71s**. Strict Rustfmt source parsing accepted **384explicitowners** in5d and the shared optional role/test, exit0. Focused shared native88824 has reached owner-command preparation; no selected native assertions have executed yet. No broader editor acceptance claim follows from these source and syntax witnesses.

The first narrow native invocation88824 exited before compilation: `--nocapture` was passed to Cargo Nextest's list phase and refused. The canonical seed and focused invocation now use the existing owned reporter options `--status-level all --final-status-level all`, with retry93775 pending. That command rejection executed zero assertions and is not a native law result.

The generic neutral corpus now belongs to the shared framework optional-field facet, and both shared/puzzle witnesses consume that exact file. This removes the lower framework test dependency on a Puzzle fixture owner. Its authored Document inputs explicitly admit omission, `label {}` clearing and `label { value="new label" }` replacement, independently of the implementation printer. Native law parity checks both these authored inputs and produced roundtrips.

After moving the corpus into its framework owner, the actual5d source suite again executed **35passed/0failed/826assertions/9.15s**. UI's final canonical publication and preflight exited0 with zero seed drift; all six current tool rows900.0580992–7 remain, including the corrected nullable reporter flags. Shared native93775 is still in Cargo build, with zero selected assertions observed.

Current native93775 completed its build and actually selected1law: **0passed/1failed/97excluded/11ms** (Nx10min). The failure was the invalid authored terminal-input check calling the generic prefix `parse` API, which intentionally does not enforce full input. The law now uses the existing canonical `parse_exact`; invalid assertions remain intact and name the offending input. Its corrected native retry is pending.

The same audit found the existing physical `diff_text!` macro calling prefix parsing at its authoritative terminal input boundary. A new5dsource assertion and native malformed-input check now require exact parsing; the source TDD red is pending before production macro repair. This boundary issue is distinct from the successfully compiled generic optional field role.

The authoritative DiffText source gate executed **34passed/1failed/827assertions/2.62s** on the physical macro's prefix parser. The macro now calls existing `parse_exact`, changing only its terminal consumption boundary. Both the generic law and5dDiffText native law retain malformed authored input assertions; the source green2316 and corrected shared native83681 remain pending.

Physical terminal boundary source green: **35passed/0failed/827assertions/2.11s**. Core's current native boundary is now stable; parent lifted the broad verification hold and current96-crate/148-registration matrix captures have resumed. Corrected shared nullable83681 and focused5d physical diff assertions remain pending.

Corrected shared native83681 actually passed **1selected/1passed/97excluded**, Nextest03619008-c835-4517-b63f-98d7ec54a91e,36ms (summary41ms; Nx8m8). The exact law covers all authored3-way inputs and malformed terminal inputs, ordinary and controlled reconstruction/projection, static typed metadata, zero-allocation borrowed measurement and cancellation. This is current native runtime proof of the shared optional role;5d physical text/binary and broader editor acceptance still await their own current captures.

Current strict syntax9323 accepted5owners, exit0, including both parent-owned Store message-clamp production/test files and the current nullable implementation/test plus physical DiffText macro. This does not claim clamp runtime behavior.
