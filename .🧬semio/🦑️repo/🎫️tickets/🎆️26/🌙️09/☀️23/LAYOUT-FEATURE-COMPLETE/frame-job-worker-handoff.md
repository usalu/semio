# Frame Job Worker Handoff

Worker session admission writes into persistent caller output slots. The frame handle supplies its existing session slot. Deadline admission keeps its output in the admission phase until the next paid phase transfer. If cancellation or receipt refusal interrupts that transfer, retirement closes the staged session through the same path used by the active deadline session.

The deadline, frame poll, and frame close paths return worker cancellation aliases under the original caller's witnessed effect. The receiver posts the returned physical progress and reported Work together once. A denied effect leaves ownership in place. Contention waits for the same session, and typed refusal returns through the original receipt.

The scoped Git diff check and Rustfmt parse passed. The FrameJob Bun verification passed all seven language-neutral laws with 89 assertions, including the SQLite-backed deadline admission law. Renderer compilation is tracked by the canonical renderer producer.

## Cancellation Alias Output Slots

FrameJob now supplies an output slot to the original cancellation alias admission in all three paths. A denied admission leaves the frame and its parameters vacant. If the caller deposits an alias and then refuses the receipt, the frame job or its parameters keep the alias for witnessed retirement before the whole failure is propagated.

The neutral law now verifies the same store-before-error order in AppFramePreparation and checks that the witnessed return posts the reported Work. The full suite passed all seven laws with 96 assertions. The scoped diff and Rust syntax checks passed again.
