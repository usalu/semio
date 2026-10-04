# Shared Provider Allocation Law Staging

## Scope And Current Evidence

Six actual Native laws are mounted under `artifact::shared_provider_allocation_tests` in the existing shared physical SQLite Native route. No provider behavior is mounted by this stage; no Cargo invocation was made by this agent. Root owns the Native baseline and must record genuine assertion failures before authorizing implementation. Compiler failures are not feature RED.

Registered Source execution completed: `bun nx run @semio-tech/framework-sqlite-snapshot-rs:test-refusal --skip-nx-cache`. The target runs strict TypeScript checking before Bun tests. Result: **6 passed, 0 failed, 116 expect calls**, including **3 new shared-provider Source laws** and the **3 existing refusal laws**. The runtime produced the three new `[DEBUG]` independent-authority receipts. Log: `🗑️generated/shared-provider-allocation-source.log`. This Source success establishes fixture and semantic authority; it does not establish Native backing admission.

## Staged Paths

- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧫️fixtures/🧮️allocation/🔣️.json`: language-neutral relational plan.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧫️fixtures/🧮️allocation/🧬️schema/🔣️.json`: closed fixture authority.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧪️tests/🧮️allocation/🦀️.rs`: six Native laws and a thread-local test-only system allocator observer.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧪️tests/🧮️allocation/🟦️.ts`: three independent Source laws.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🦀️.rs`: only `cfg(test)` Native module wiring added.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/🧪️tests/🟦️.ts`: import mounts the Source laws in the existing registered route.

No executable target was added; existing `test-native` and `test-refusal` targets retain their launch registration. The exact Native selector is **`shared_provider_allocation`**, matching only these six test names. The whole Native route remains `bun nx run @semio-tech/framework-sqlite-snapshot-rs:test-native --skip-nx-cache`.

## Neutral Fixture Invariants And Independent Proof

The schema requires exactly 25 fields and rejects extra fields. Seven hostile plans are rejected by both the in-repo subset validator and independent strict Ajv: invented ABI backing bytes, incomplete tiny allowances, conflated semantic/allocation ceilings, wrong refusal kind, extra owner fields, wrong UTF-8 byte length and replacement of duplicate ordinals.

The authored SQL contains two tables with a foreign-key relationship. Two owners exercise empty TEXT/BLOB and nonempty Unicode/octet ownership. Binary64 companions retain negative zero (`8000000000000000`) and a NaN payload (`7ff8000000000011`) through signed INTEGER storage. Independent DataView verifies the bit-to-class relationship; independent Bun SQLite verifies storage classes, companion identity, three ordinal relations ordered as `[2,3,1]`, duplicate ordinal detection, integrity, foreign keys, serialization and reopen.

The text is 50000 repeats of `é`, independently measured by Buffer as 100000 UTF-8 bytes. The blob repeats `[0,255,128,1]` 25000 times, yielding 100000 octets. The 65536-byte frontier lies inside both fields and at a valid UTF-8 boundary. SQLite independently measures payload byte lengths. A separate SQLite admission table demonstrates the arithmetic for two retired `same` copies at an eight-byte allowance and a refused third copy. Semantic ceilings are 4000000 bytes, schema ceiling 4000000 bytes, file/allocation ceilings 8000000 bytes, independent of allowances 0/1 or measured exact backing.

## Native Laws And Allocation Observation

1. `shared_provider_allocation_schema_tables_exact_refused_repeated_and_canceled`: allowances 0/1 refuse before schema backing; successful schema requests equal cumulative admission; exact repeated schema constructions exhaust one control after retirement; one byte less refuses; schema lexing checks cancellation inside the supplied SQL.
2. `shared_provider_allocation_numeric_cells_and_rows_have_actual_exact_backing`: integer-only insertion owns four `SqliteValue` slots and row-vector backing despite no text/blob bytes; observed requests equal admission; exact, exact-minus-one and zero allowance variants retain a typed refusal and no committed row.
3. `shared_provider_allocation_text_blob_exact_and_retired_copies_remain_cumulative`: project text plus reconstructed blob exhaust exactly 200000 backing bytes; exact-minus-one refuses before the blob; separate calls on one control never reset retired copies; empty copies succeed at zero backing and nonempty copies refuse.
4. `shared_provider_allocation_text_blob_interior_cancellation_retains_backing`: cancellation inside each 100000-byte field retains its full admitted backing; the next tiny field refuses; canceled-before-copy projection preserves remaining admission.
5. `shared_provider_allocation_ordered_frontier_exact_refused_duplicate_and_canceled`: ordinal ordering owns admitted borrowed-row frontier backing; exact/minus-one/zero allowances are checked; duplicate ordinals retain `InvalidValue`; cancellation inside 300 relationships retains admitted frontier ownership.
6. `shared_provider_allocation_ieee_cells_exact_refused_duplicate_and_canceled`: expanded borrowed cells, any duplicate-validation frontier, final owned values and row backing all contribute to observed requests; exact/minus-one/zero allowances are checked; an armed cancellation refuses before temporary IEEE cells; duplicate scalar slots retain `InvalidValue`.

The Native observer wraps the system allocator and accumulates **actual alloc/alloc_zeroed/realloc requested bytes**, including full replacement allocations. It is enabled only on the running test thread, with fixture parsing, payload construction, expectation vectors, printing and SQL setup outside the observed interval. Successful producer requests must exactly equal the transfer allocation ledger; no guessed Rust/BTree ABI values enter the fixture. Minimum Vec bounds use the actual declared `size_of::<T>()` only in Native laws. Refusal assertions allow only already-admitted backing plus the returned typed error message allocation. Late cancellation can produce nested diagnostic messages, so its observed total is a lower bound containing the payload and returned error, while exact payload admission remains required. Diagnostic message allocations are not represented as domain backing.

## Repair Authority After Root RED

The existing transfer module already contains controlled lexing, table parsing, controllable `reserve`, chunked `copy_text`, bounded ASCII comparisons and in-place heap sorting. Extend the shared controlled schema authority instead of routing `Projection::new` through the uncontrolled parser and validation. Existing declaration construction/validation can be exposed or reused without copying the Reader. Avoid treating semantic scalar bytes as row/cell backing, reset controls, guessed BTree node charging, or direct unchecked collection allocation.

No Reader code, schema implementation, projection behavior, reconstruction behavior, permanent scripts, launch catalogs, runtime dependency declarations, Git state or AGENTS files were changed in this stage.
