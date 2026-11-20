# Interaction revalidation after document publication

The six document command tests and the agent boot passed the canonical digest phase. Their next refusal came when the published document changed the persisted interaction state. GDB followed the call through mounted revalidation, interaction store dispatch, mutation replay, and operation identity encoding. The encoder had no original work lender.

Debugger checks confirmed that both the boot and document edit resumed their owned identity receipt. Mounted revalidation of an owned command restores the retained identity receipt with the current caller's work lender. It records performed work on that caller when the operation returns. The reserved-commit path now uses the same adapter. The mutation replay and the persistence of validated selection state remain part of the integration check.

Sources are `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs`, and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🔁️stall-witness-and-unit-advance/🦀️.rs`.
