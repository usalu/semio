# S7 — proposed diff for `🔌️plugin/🧬️schema/📜️component.wit`

Not applied to the live tree (per instructions — four packets are mid-flight and the schema is
shared). Hand-verified against the real file's current line numbers as of this spike; re-check
line numbers before applying if the file has moved on.

## 1. Hoist `job-budget`/`job-step` into `interface types` (insert before its closing `}`, i.e.
   right after the existing last record in `interface types`, ~line 117, before line 118's `}`)

```wit
  /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-probe-spikes S7). Hoisted out of
  /// `interface jobs` so both the sync `jobs` (world `actor`) and async `jobs-async` (world
  /// `actor-async`) interfaces can `use types.{job-budget, job-step}` and share the SAME type —
  /// a `use`d WIT type is an alias, not a distinct type, so this is required for the two
  /// interfaces' `step-job` signatures to be interchangeable at the Rust binding level.
  record job-budget {
    fuel: u64,
    deadline-ms: u32,
  }
  variant job-step {
    running(option<list<u8>>),
    done(list<u8>),
    failed(list<u8>),
  }
```

## 2. `interface jobs` (currently ~line 980-1002): remove the local record/variant defs, `use` the
   hoisted ones instead — `world actor`'s export is otherwise BYTE-IDENTICAL, same func signatures

```diff
 interface jobs {
-  use types.{plugin-error};
-
-  record job-budget {
-    fuel: u64,
-    deadline-ms: u32,
-  }
-  variant job-step {
-    running(option<list<u8>>),
-    done(list<u8>),
-    failed(list<u8>),
-  }
+  use types.{plugin-error, job-budget, job-step};
+
   start-job: func(job: u64, kind: string, input: list<u8>) -> result<_, plugin-error>;
   step-job: func(job: u64, budget: job-budget) -> result<job-step, plugin-error>;
   cancel-job: func(job: u64);
 }
```

## 3. Add `interface jobs-async` right after `interface jobs` (new)

```wit
/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-probe-spikes S7). ASYNC counterpart to
/// `jobs` above, for `world actor-async` — identical signatures, `async func` instead of `func`.
/// terra-probe-spikes S7 found empirically (wasmtime 47.0.3) that a plain sync `func` export is
/// UNCALLABLE (not merely deadlock-risky) on ANY `Store` configured with
/// `wasm_component_model_async(true)` — every call shape tested (reentrant inside
/// `run_concurrent` via `Accessor::with`, AND the classic `&mut Store` call OUTSIDE any
/// `run_concurrent` session on an otherwise-idle store) failed immediately with `"store
/// configuration requires that *_async functions are used instead"`. `world actor-async`'s guest
/// is only ever instantiated on such a store, so its `jobs`/`checkpoint` MUST be this async
/// interface, not the sync one above. See TICKET_DIR/📓️terra-probe-spikes-report.md, S7 section.
interface jobs-async {
  use types.{plugin-error, job-budget, job-step};

  start-job: async func(job: u64, kind: string, input: list<u8>) -> result<_, plugin-error>;
  step-job: async func(job: u64, budget: job-budget) -> result<job-step, plugin-error>;
  cancel-job: async func(job: u64);
}
```

## 4. Add `interface checkpoint-async` right after `interface checkpoint` (new; `checkpoint` itself
   is UNCHANGED, still exported by `world actor`)

```wit
/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-probe-spikes S7). ASYNC counterpart to
/// `checkpoint` above, for `world actor-async` — same reasoning as `jobs-async`. Checkpointing
/// itself never awaits (in-flight async tasks are deliberately never serialised, only marked
/// re-run-on-restore) so this interface has no async WORK to do — it exists solely because a
/// sync `func` export cannot be called at all on this world's store, full stop.
interface checkpoint-async {
  use types.{plugin-error};

  checkpoint: async func() -> result<list<u8>, plugin-error>;
  restore: async func(state: list<u8>) -> result<_, plugin-error>;
}
```

## 5. `world actor-async` (currently ~line 1044-1051): export the `-async` interfaces.
   `world actor` (~line 1029-1043) is NOT touched — still exports plain `jobs`/`checkpoint`.

```diff
 world actor-async {
   import pure;
   import host-async;
   export runner;
-  export jobs;
-  export checkpoint;
+  export jobs-async;
+  export checkpoint-async;
   export describe;
 }
```

## parity-test fallout (plugin-host, NOT touched here — flagging for the coordinator)

`both_worlds_share_the_same_export_surface_and_actor_is_untouched` will need re-specifying: the
export *interface names* now differ between `actor` (`jobs`, `checkpoint`) and `actor-async`
(`jobs-async`, `checkpoint-async`), by design. What stays genuinely identical and is what the
re-specified test should assert instead:
- `world actor` itself: byte-identical WIT, zero lines changed.
- The underlying types: `job-budget`, `job-step`, `plugin-error` are the exact same hoisted type
  in both worlds (that's the whole point of the hoist — `use`d WIT types are aliases).
- The function names/params/returns are identical modulo the interface suffix and `func` vs
  `async func` — `jobs::step-job` and `jobs-async::step-job` take the same `job-budget`, return
  the same `job-step`.
