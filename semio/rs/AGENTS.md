---
technology: semio
bundle:
 name: rs
 emoji: 🦀
 description: The rs bundle for semio.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

### CQRS Dual-Bus Actor Model

- Communication MUST be symmetrical.
- You MUST NOT use polling.
- You MUST use a Single FFI Boundary.
- You MUST use an Inbound Work Queue.
- You MUST use an Outbound Event Stream.
- You MUST NOT use os-depencies such as tokio due to lack of async in wasm.
- You MUST use fire-and-forget commands that return an execution id and return any associated result as an event. It is the repsonsability of the ui to associate results to the calls.

## 📛 Entities
