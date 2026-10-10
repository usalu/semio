# Original Kernel Receiving Fourth Full Compiler Gate

The exact registered full Kernel launch, child44140 / tool session78187, terminated exit1. The unchanged actual Cargo command was `cargo test --no-run --message-format json-render-diagnostics --all-targets --features mutation-testing --config unstable.build-dir-new-layout=false`. No assertion ran. The original launch environment, build and assertion deadlines, full inventory and runner policy were preserved.

The live capture used separate sinks for stdout and stderr targeting one file, which overlapped some prefixes. Only these unambiguous actual diagnostic blocks and the compiler/launch exit footer are credited; this report does not claim a complete ordered stdout/stderr transcript. Future launch capture uses one shared open descriptor.

| Actual source | Diagnostic |
| --- | --- |
| framework/actor/🦀️.rs:703,704,705,706 | Four E0560: lane defaults explicitly pass retained but Budget lacks that field |
| framework/ui/wgpu/prepared/🦀️.rs:3571,3572 | E0053: InteractiveJob::step still returns StepOutcome; E0046: borrow_outcome missing |
| os/store/🦀️.rs:10104,10109 | E0053: ArtifactEnvelopeDecodeAuthority::step still returns StepOutcome; E0046: borrow_outcome missing |

The actual new InteractiveJob trait requires `step<'a>(&'a mut self, &mut StepContext) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError>` and a borrow_outcome method retaining the original descriptor/payload relationship. These are real shared interface compiler floors. No erased-read or sealed-peer runtime refusal law was reached, so neither producer has been repaired here.

The compiler footer identifies actor4, ui2 and Kernel2 errors. Cargo exited101. Saving the runner's build-failure.stdout.txt subsequently encountered actual ENFILE file table overflow, then the registered launch exited1. This later artifact-save error is distinct from the already emitted compiler diagnostics.

The canonical JSON migration floors from red3 no longer appear in this observed compiler frontier. That observation is not native acceptance of those implementations.

Root and Media received the actual floor locations. Current Store producers and all original copy-refusal fixture rows remain intact pending genuine runtime RED. The separately authored restored-actor missing-helper law is now in Media's unchanged full Plugin red4 gate, child49587 / session90538; no local restore producer or receiving API/body has changed.
