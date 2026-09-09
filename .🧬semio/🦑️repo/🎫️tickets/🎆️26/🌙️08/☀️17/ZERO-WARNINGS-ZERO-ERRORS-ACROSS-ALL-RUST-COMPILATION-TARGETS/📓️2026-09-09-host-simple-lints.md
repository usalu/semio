# Host Simple Lint Review

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:392

```rust
        let (frame, rearmed_epoch) = self.state.lock().unwrap_or_else(PoisonError::into_inner).shard.as_mut().map(ShardLoop::take_terminal_frame_and_rearm).unwrap_or((None, None));
```

Replacement: `map_or`

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:392

```rust
        let (frame, rearmed_epoch) = self.state.lock().unwrap_or_else(PoisonError::into_inner).shard.as_mut().map(ShardLoop::take_terminal_frame_and_rearm).unwrap_or((None, None));
```

Replacement: ``

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:392

```rust
        let (frame, rearmed_epoch) = self.state.lock().unwrap_or_else(PoisonError::into_inner).shard.as_mut().map(ShardLoop::take_terminal_frame_and_rearm).unwrap_or((None, None));
```

Replacement: `(None, None), `

## clippy::needless_borrow

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1813

```rust
            self.runtime.execute_turn(instance, &events, turn_budget.await).await
```

Replacement: `events`

## clippy::needless_borrow

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1928

```rust
            Err(fault) if super::retryable_lifecycle_turn(&fault, &events) => return Ok(true),
```

Replacement: `events`

## clippy::clone_on_copy

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs:909

```rust
        self.metrics.record_drain(DrainOwner::Package, report.clone());
```

Replacement: `report`

## clippy::clone_on_copy

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs:991

```rust
                    self.dispatch_router_effect(ctx.await, scope.clone(), *req, RouterEffect::BlobWrite { media_type: media_type.clone(), bytes: bytes.clone() }).await;
```

Replacement: `*media_type`

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs:567

```rust
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
```

Replacement: `map_or`

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs:567

```rust
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
```

Replacement: ``

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs:567

```rust
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
```

Replacement: `0, `

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:1643

```rust
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
```

Replacement: `map_or`

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:1643

```rust
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
```

Replacement: ``

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:1643

```rust
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
```

Replacement: `0, `

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:2556

```rust
    let body_key = surface.split_once(':').map(|(_, body_key)| body_key).unwrap_or(surface);
```

Replacement: `map_or`

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:2556

```rust
    let body_key = surface.split_once(':').map(|(_, body_key)| body_key).unwrap_or(surface);
```

Replacement: ``

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:2556

```rust
    let body_key = surface.split_once(':').map(|(_, body_key)| body_key).unwrap_or(surface);
```

Replacement: `surface, `

## clippy::redundant_async_block

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:4390

```rust
    semio_framework_async::block_on(async move { receiver.await }).map_err(|_| "relay lifecycle worker barrier closed".to_string())
```

Replacement: `receiver`

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6674

```rust
            if owner != plugin_id && !state.dependencies.get(plugin_id).map(|deps| deps.contains(&owner)).unwrap_or(false) {
```

Replacement: `is_some_and`

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6674

```rust
            if owner != plugin_id && !state.dependencies.get(plugin_id).map(|deps| deps.contains(&owner)).unwrap_or(false) {
```

Replacement: ``

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6788

```rust
                let has_surface = state.surfaces.get(&(dialect.clone(), role)).map(|refs| !refs.is_empty()).unwrap_or(false);
```

Replacement: `is_some_and`

## clippy::map_unwrap_or

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6788

```rust
                let has_surface = state.surfaces.get(&(dialect.clone(), role)).map(|refs| !refs.is_empty()).unwrap_or(false);
```

Replacement: ``

## Guarded Repairs

Applied twenty-one reviewed compiler span suggestions across five files. Removed the two blank lines separating documentation from its item and ended the capability registry lock before awaiting token cancellation. No error layout changes were made before the baseline measurement. All five sources parsed before guarded writes.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs


The terminal surface conflict branch transfers app_ref.plugin_id and app_ref.app_id into FaultScope before breaking, avoiding two redundant clones. The host-layout932 dependency and host build completed after this source adjustment.
