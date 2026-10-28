# Sparse Original Source Ownership Design

This is the concrete next production ownership design, not an implemented or executed claim.

## Current Physical Boundary

The framework String clone cursor quotes and reserves its entire contiguous destination capacity before copying bounded UTF8 chunks. A one-million-byte String therefore needs one-million-byte capacity authority in a single turn. A262144-byte phase ceiling correctly denies that allocation; extra fuel ticks cannot make it possible. Vec allocation and final String/Vec release have the same real contiguous owner boundary. The original source remains borrowed and no oversized destination is born during denial.

The independent OriginalFixtureCaller initiating stock already supplies134217728 copy bytes,268435456 capacity bytes and536870912 release bytes over the whole fixture. Its generic retirement loan is a smaller per-turn restriction. The new neutral IFC `source-owners.json` authors a fixed2MiB contiguous capacity/release loan,32768 copy bytes and one item before work begins; it is below the initiating stock and is not calculated from descriptive demands. IFC retained tests for existing one-million-byte bodies now use this loan. A separate262144-byte short-loan regression reaches the actual String capacity frontier, verifies no copied/allocated body and no fuel progress, then cancels the original request and drains its small cursor/source owners. These tests remain unexecuted because compiler ingress is unavailable. This fixture does not change or prove the production mounted-owner policy.

## Next Sparse Retype Work

Retain the actual captured `SnapshotSource` and create a small native retype plan from borrowed static catalog declarations. Do not clone the entire IfcEntity before deciding which fields change. The plan stores the selected canonical declaration, old/new attribute indices, component/attribute/derived-scan indices, removal index, current selected native child clone cursor, pending primitive mutations and a close cursor. The input compact `$ifcType` command remains the same existing SnapshotEditEvent.

For each target attribute, advance one retained-component derivation lookup at a time. Preserve an unchanged same-index argument by leaving it borrowed from the captured source and emitting no leaf. If a preserved attribute moves to another serialization position, clone only that one native IfcValue through the existing original-source projection, retain it before every fallible frontier, and transfer it into the existing SetEntityArg/InsertEntityArg leaf. Optional/new placeholders and derived markers are authored one attribute at a time. Emit obsolete argument removals in descending order so remaining captured preimages stay valid. A GlobalId repair generates a small candidate in retained custody and advances one original entity/component collision comparison per turn before emitting the actual generated value.

For complex component normalization, derive effective markers from the complete retained component set using persistent indices. Only clone the native complex Vec when an existing SetEntityComplex leaf is actually required; the current whole-complex mutation payload still requires an owned Vec. A further sparse complex diff/leaf would need schema-first changes in every implementation before it could avoid that remaining copy. Do not introduce a second graph representation or pretend a borrowed original can be moved into an owned String.

## Publication And Input Integration

Original mutable command receiving, Native/Decimal owned value receiving, argument/header value editing and retained forward/inverse wire cursors already exist in source. Reuse these interfaces. Keep displaced values and partially emitted mutations in plan fields across every failure, and return them only through controlled close. Direct/cold reference decisions stay separate from native retained production.

The publication preflight currently admits only captured-preimage-safe batches and refuses arbitrary progressive dependencies before encoding bytes. Any later progressive overlay must retain the actual pending mutation owner, resolve each immediate preimage through preceding mutations with persistent indices, and supply a stable borrowed wire source. It must not clone a full snapshot or reconstruct inverse payloads eagerly. The selected-template planner emits distinct final inserted resources and one existing-target write, so that path can remain within the explicit sparse admission boundary.

## Required Remaining Evidence

Execute authentic native tests and heap/copy/release receipt measurements under the real mounted-owner policy. The current source has not type checked. Large schema/type metadata preparation, raw whole-entity/constituent receiving, reference/uniqueness scans, GlobalId scans and contextual marker normalization still contain synchronous work. Store publication, scheduler fairness, checkpoint restoration and all original cancellation prefixes have not been executed. No native infrastructure closure is claimed.
