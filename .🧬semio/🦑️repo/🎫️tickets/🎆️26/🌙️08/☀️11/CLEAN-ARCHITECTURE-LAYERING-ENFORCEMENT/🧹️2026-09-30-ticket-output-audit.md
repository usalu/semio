# Ticket Output Cleanup Audit

The reopened ticket contains twenty-four historical raw output files at its root, outside the required generated directory. Each file was read: they contain compiler diagnostics, command stdout or a Git status snapshot. They are tool output, not input scripts or written research reports. Their timestamps are September 26, before this execution. The user explicitly requires removal of tool-generated output on ticket completion; removal is pending the final aggregate.

All Markdown reports, the important Markdown note, Rust/TypeScript input snippets, JSONC configs, the Cargo manifest stub and ticket metadata are preserved. No source, peer cache, shared Git state or files outside this ticket are removed by this cleanup. This audit records identities without claiming any historical command is currently passing.

| Output File | SHA-256 Before Removal |
| --- | --- |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/w1-check-stdio-checks-output.txt` | `5751a4feea1d16d649a7f68618d987933c63f84c899de56e746b7efbcd549b11` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/w1-policy-output.txt` | `f89fd1e6ae2527fe914bfa2b736870eea7ebb60f23f4473a4d7bff51d58b450b` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/w1-verify-gate-output.txt` | `426e8e50aa713304133695e6b1add230b1afcbe33440335ce901d7639f5f59bf` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/w7-cargo-check-full.txt` | `f420fdb3c932dd8a2d72d54269dfd7c011c5f94c82c0d98cd68bec23f196d6b9` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/w7-verify-gate-full.txt` | `20e5e49e3c447655a4bd6cba188e1e4b8d3957ee07791ef4d1a4b0ad26f0bffc` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w1-verify-cargo-check-full.txt` | `878f459464ec4fa2a965bd71f4f49d9bded60824fa25fcd8a5edada1981f77a7` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w1-verify-cargo-check.txt` | `359297fa0abd74dd50d799c869727ef36aaf253032d9c39f3bba1c3901462cea` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w1-verify-gate-full.txt` | `467241814e1a2debda0101251386f61ed7d7020b57262e8fc7ffa6d866d4631d` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w1-verify-gate.txt` | `07257aea1d886849f837b7bf88b693b6d383a95fb1ba07a58b3b62c94e1b2eae` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w1-verify-registry-check-full.txt` | `00c4ca3dff123705d5d27a50a9d34ae97b89df7ca6ffd88433e7f528bc0e7846` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w1-verify-registry-check.txt` | `97350b5c61e92365cdd7f47bf93fdbc68cf1e3739420e5d4d09fffb60acd0046` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w2-verify-cargo-check-full.txt` | `160506c88f383f15564b93abfa82b32f7626f288aea451f0941bf09454d6ae4b` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w2-verify-cargo-check.txt` | `6be088f918add7030d1670174f20a96174851d5507a814aa643fff690fafd059` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w3-verify-cargo-check.txt` | `2a50f5a6632d605c851cf9c6d0225b1caeb6fd8b74aa5dd7a5eacd87b6cf6798` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w4a-verify-cargo-check.txt` | `11d3e78e406a34994c8b6932d2d7b43342c1fe6131b4fbbff03deb3bff08b3c2` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w4a5-verify-cargo-check.txt` | `59c36e9fe6d8be5965f7f1689d0c45d4fe3369c5005416aed0728fbc246032bc` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w4a6-final-workspace-check.txt` | `ba46fe4df39d2ad32b3ed39d49dc6c741effb9cfa92bb209c62cd0c9554bdd1e` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w4a6-verify-cargo-check.txt` | `5824b72934036c87bb6682b74c3be00f7e28c2704eae0564175e3b0a66f751a2` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w4b-verify-cargo-check.txt` | `5f5989918be5f30160bd16bada17c271066b02097322999a145b38739cd8011c` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w5a-verify-cargo-check.txt` | `5f5989918be5f30160bd16bada17c271066b02097322999a145b38739cd8011c` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w7-cargo-check-orchestrator.txt` | `ed67746fda4a4392897de89e8ecb2a7ad8f0e2456e8b1709f817502dd6a0369a` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📸️baseline-cargo-check.txt` | `25ad8d7e627d2b0f279f53ea0a2543a88c2cedca85a166d737550a68f18d1a37` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📸️baseline-git-status.txt` | `612a6e1fba6964ce793e0b3376087ade7098b14f2b2ccb6a0cbe6164f2fa4595` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📸️baseline-verify-gate.txt` | `4ec5c0f9222260e783600fe47ab363ba053f4b542ccae7032fd6e1f727dce4e8` |
