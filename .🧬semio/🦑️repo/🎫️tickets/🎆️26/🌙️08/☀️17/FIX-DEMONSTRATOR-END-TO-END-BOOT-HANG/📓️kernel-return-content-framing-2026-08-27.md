# Kernel Return Content Framing

## Scope

The shared four-file content declaration was read before implementation. KernelReturnContentFraming consumes exactly one input byte per push, retains only bounded scalar state, validates canonical unsigned lengths, and enforces metadata/lifecycle/UI/effect/presence/command/status/end section order and declared counts. Metadata is exposed as a small frozen value. It does not allocate any declared body or keep a raw page reference.

UI surface bytes, operation bodies, lifecycle receipts, effects, presence and status payloads still require their domain semantic validators and retained storage. In particular, preserving split Unicode bytes is not UTF-8 validation, preserving an Invocation effect is not extension execution, and section completion is not raw input ACK or UI publication. No field/fragment source authority has been minted by this parser.

## Executed Evidence

- 🧪️kernel-return-content-red-1.log: actual five failures / 41 skipped / 46 collected, two files, 5.24 s, start 20:51:25. The new class was absent.
- 🧪️kernel-return-content-green-1.log: five passes / 41 skipped / 46 collected, 4.28 s, start 20:52:54.
- 🧪️kernel-return-content-full-1.log: full Kernel TS 46/46, two files, 2.50 s, start 20:55:05, after the public Kernel re-export.
- 🧪️kernel-return-content-strict-1.log: 14 diagnostics, not a pass: known tutorial seven and seven library builder typing errors. The previous UI private-constructor fixture error is clear. Builder errors were routed to Taxonomy; no content or actor diagnostic appears in this capture.

Strict Ajv validates both shared declarations. Independent webassemblyjs LEB128 plus Node Buffer reproduces every authored record frame. Tests consume the stream at 1/64/4096-byte splits, every prefix split, every truncated prefix and each authored invalid section sequence. They cover exact effect/presence counts, all four statuses and conditional status payloads, duplicate lifecycle rejection, opaque Invocation preservation, a surface and operation exceeding one raw page, malformed lengths/counts, and sticky faults. An announced maximum-u64 body consumes one byte without constructing any typed payload buffer.

## Stable Inputs

Content TS: 53f1e33e91db067b0e35114eb7a6c988edfcb5091e76a19b44b9d32c3555b3a1.

Kernel test config: 67083387329300c9c50117041a8faef9640d68cf2a653d556c0be640c9ad4020.

Kernel public source: 962cae4b60de32dc09e0990031491363c1b6762c1882d7748ffa1508fe1750ad.

Taxonomy confirms exact launch registration at 400.6, ⚖️gate🎠️kernel📤️return-content, using the existing Kernel Nx/script target with --testNamePattern=KernelReturnContentFraming. The tests above actually ran; registration itself is not execution evidence.

## Ownership And Remaining Integration

UI owns the shared host resident pool and paged semantic storage. This lane owns its actual composition injection and the captured source/field/fragment return handshake. Native resident permits were inspected but are not a fulfilled JS byte lease.

The original captured activation/worker must retain raw return-control authority after operation revocation. A parser, format-valid receipt, ordinary operation lease or current actor-name lookup cannot substitute for that authority. Live source-before-WIT paging, independently owned semantic storage, host/native retirement and all-six-app verification remain open.

No native compilation, generated publication, browser-policy bypass, cleanup or evidence deletion occurred. This ticket remains active, so its inputs, outputs and reports are retained for the ongoing work.
