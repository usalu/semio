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

### Dual Channel Actor Model

This section outlines the migration of our application logic into a **symmetrical, zero-polling WebWorker environment** using a strict dual-channel Actor Model.

We are moving away from traditional request/response hierarchies. Instead, the Main JavaScript Thread and the WASM WebWorker act as decoupled peers communicating across two distinct logical channels, using `async-graphql` strictly as a schema-validated message broker.

**Core Directives:**

1. **Zero Polling:** Threads suspend natively (0% CPU) until a memory channel waker fires.
2. **Dual-Channel Bus:** Strict separation of concerns:
   - **Inbound:** JS → WASM Work Queue (Commands).
   - **Outbound:** WASM → JS Event Stream (Results/Events).
3. **Single FFI Boundary:** All communication crosses the WASM boundary via a single `execute` function.

---

#### 2. Target Architecture: The CQRS Dual-Bus

The architecture relies on two independent `async-channel` buses to prevent cyclic message consumption and enforce a clear unidirectional data flow.

##### A. The Inbound Work Queue (JS → WASM)

- **Purpose:** Carries commands, intents, and data payloads from the UI to the WASM background engine.
- **Mechanism:** JS executes a GraphQL `Mutation`. The Rust mutation resolver validates the input and pushes a `Command` payload into the inbound `Sender`.
- **Processing:** A spawned background task in WASM (`wasm_bindgen_futures::spawn_local`) actively waits on the inbound `Receiver`, processes the work, and produces results.

##### B. The Outbound Event Stream (WASM → JS)

- **Purpose:** Carries state changes, processing results, and asynchronous events back to the UI.
- **Mechanism:** On startup, JS opens a permanent GraphQL `Subscription`. Rust returns a clone of the outbound `Receiver` stream.
- **Processing:** When the WASM background task finishes processing an inbound command (or a spontaneous internal event occurs), it pushes the result into the outbound `Sender`. The subscription waker fires, instantly yielding the data to the JS callback.

---

##### 3. The Single Boundary Definition & State Management

```rust
use async_channel::{unbounded, Receiver, Sender};
use std::sync::OnceLock;
use wasm_bindgen::prelude::*;

// 1. The Dual Channels
static INBOUND_CHANNEL: OnceLock<(Sender<String>, Receiver<String>)> = OnceLock::new();
static OUTBOUND_CHANNEL: OnceLock<(Sender<String>, Receiver<String>)> = OnceLock::new();

// 2. The WASM Actor Loop (Spawned Once)
pub fn start_wasm_actor() {
    let (_, inbound_rx) = INBOUND_CHANNEL.get_or_init(unbounded);
    let (outbound_tx, _) = OUTBOUND_CHANNEL.get_or_init(unbounded);

    // Spawn a background task on the JS microtask queue
    wasm_bindgen_futures::spawn_local(async move {
        // Sleep at 0% CPU until a command arrives in the Inbound Queue
        while let Ok(work_item) = inbound_rx.recv().await {
            // Process work... (e.g., update in-memory graph)
            let result = format!("Processed: {}", work_item);

            // Push result to Outbound Event Stream
            let _ = outbound_tx.try_send(result);
        }
    });
}

// 3. The Single WASM Boundary
#[wasm_bindgen(js_name = execute)]
pub async fn execute(request_json: String, on_message: js_sys::Function) -> Result<(), JsValue> {
    let schema = get_schema();
    let req = serde_json::from_str(&request_json).unwrap();

    let mut stream = schema.execute_stream(req);
    let this = JsValue::NULL;

    // Resolves Mutations (push to INBOUND) and Subscriptions (read from OUTBOUND)
    while let Some(response) = stream.next().await {
        let msg = JsValue::from_str(&serde_json::to_string(&response).unwrap());
        if on_message.call1(&this, &msg).is_err() { break; }
    }
    Ok(())
}
```

#### 4. Migration Execution Plan

##### Phase 1: Strip OS Dependencies & Setup Actor

1. Remove all traces of `tokio` and native I/O.
2. Setup `wasm32-unknown-unknown` target.
3. Implement the dual `async-channel` setup.
4. Create a `boot()` function exported to WASM to initialize `OnceLock` states and spawn the `wasm_bindgen_futures` background actor loop.

##### Phase 2: Schema Conversion (Command / Event Separation)

Restructure the GraphQL schema to reflect the dual-channel flow:

- **Mutations (Commands):** Resolvers do not contain heavy business logic. They purely validate the request and push the payload to the Inbound Work Queue. They return a simple Boolean (e.g., true for "queued").
- **Subscriptions (Events):** Resolvers return a clone of the Outbound Event Stream.
- **Queries (Reads):** Strictly reserved for immediate, synchronous state reads of the in-memory graph (e.g., `getCurrentState()`), completely bypassing the queues.

##### Phase 3: JS Host Integration & Lifecycle

The frontend must manage the connection lifecycle and map UI interactions to the queues.

1. **Boot:** JS Thread loads the WASM module and calls `boot()` to start the internal actor loop.
2. **Establish Outbound Pipe:** JS opens the main event stream:
   ```javascript
   execute("{ subscription { eventStream { type, payload } } }", handleWasmEvent);
   ```
3. **Produce Commands (JS → Inbound):** UI interactions fire fire-and-forget mutations:
   ```javascript
   // Resolves instantly; actual work happens asynchronously in the actor loop
   execute('mutation { enqueueWork(command: "SYNC_DATA") }', () => {});
   ```

#### Phase 4: Error Handling & Resilience

1. Implement `console_error_panic_hook` so Rust panics gracefully log to the browser console.
2. Ensure the background Actor loop wraps processing logic in standard `Result` handling, pushing `ErrorEvents` to the Outbound channel so the JS frontend can display toasts or warnings without the worker crashing.

## 📛 Entities

```

```
