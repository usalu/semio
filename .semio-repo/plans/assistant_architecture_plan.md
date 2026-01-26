# Design Assistance System — Detailed Architecture (Markdown)

This document is a **deep‑dive architectural specification** for the **design assistance system** described.  
It is intended to be **directly actionable** for implementing a single‑file Go binary at:

```
go/assistant/main.go
```

---

## 1. Problem Statement

We operate a **design assistance loop** with the following characteristics:

- **One source design**
  - Authored and stored in an MCP‑enabled design platform (example: `semio`)
- **Multiple compliance targets**
  - Example targets:
    - `BerlinBuildingCode`
    - `RoomProgram`
- **Automated validation**
  - Each target defines *rules*, *clauses*, and *measures*
  - A violation occurs when **any clause is unsatisfied**

The system must:
1. Translate the design into each target format
2. Validate the translated design
3. Aggregate violations into a report
4. Iteratively fix violations via an automated changer
5. Repeat until compliance or exhaustion

---

## 2. High‑Level Architecture

### Core Components

| Component | Type | Responsibility |
|---------|-----|----------------|
| **assistant** | Go binary | Orchestrates translation, validation, aggregation, and iteration |
| **design MCP server** | Service | Provides read/write access to the design model |
| **translator** | Agent | Converts `semio → target format` |
| **validator** | Binary | Evaluates rules and produces violations |
| **changer** | Agent | Modifies the design to fix violations |

---

## 3. Execution Model

### Key Principles

- **Fan‑out / fan‑in concurrency**
- **Translator → Validator streaming**
- **Per‑target isolation**
- **Iterative improvement loop**

### Lifecycle Overview

```
Design (semio)
   ↓
assistant
   ├─ translator: semio → BerlinBuildingCode ─▶ validator
   ├─ translator: semio → RoomProgram        ─▶ validator
   ↓
Aggregate Report
   ↓
changer (MCP analyze + change)
   ↓
Updated Design
   ↓
Repeat
```

---

## 4. Domain Model

### 4.1 Rules, Clauses, Measures

- **Rule**
  - A compliance requirement (e.g. “Minimum egress count”)
- **Clause**
  - Atomic check within a rule
- **Measure**
  - Observed value supporting a clause decision

### Violation Definition

A **violation exists when at least one clause is not satisfied**.

---

## 5. Data Structures

### 5.1 Measure

```json
{
  "id": "m-egress-count",
  "clauseId": "bbc.R12.C3",
  "name": "egressRoutes",
  "value": 1,
  "unit": "count",
  "threshold": 2
}
```

---

### 5.2 Violation

```json
{
  "id": "bbc.R12.C3",
  "target": "BerlinBuildingCode",
  "ruleId": "bbc.R12",
  "clauseId": "bbc.R12.C3",
  "severity": "high",
  "message": "Second egress route missing",
  "location": {
    "designNodeId": "semio:node:room:2.14"
  },
  "measures": [],
  "remediationHints": []
}
```

---

### 5.3 Report

```json
{
  "designId": "semio:project:abc",
  "iteration": 2,
  "targets": [],
  "violations": []
}
```

---

## 6. Translators

### Responsibilities

- Read the **current design state**
- Emit a **target‑specific document**
- Remain stateless and deterministic

### Contract

- **Input**: design reference
- **Output**: serialized target model (JSON, XML, etc.)

```json
{
  "target": "RoomProgram",
  "format": "room_program/v1",
  "payload": {}
}
```

---

## 7. Validators

### Responsibilities

- Parse translator output
- Evaluate rules → clauses
- Emit violations

### Validator Properties

- Executable binary
- Reads from `stdin`
- Writes JSON to `stdout`
- No side effects

---

## 8. Orchestration Logic (assistant)

### 8.1 Concurrency Model

- One goroutine per target
- Each goroutine:
  1. Runs translator
  2. Pipes output into validator
  3. Collects result

Use:

```go
errgroup.Group
context.Context
io.Pipe
```

---

### 8.2 Failures

| Failure Type | Handling |
|-------------|----------|
| Translator error | Target marked failed |
| Validator error | Target marked failed |
| MCP failure | Abort iteration |
| Partial target failure | Still aggregate remaining results |

---

## 9. Iterative Fixing (changer)

### Inputs

- Full aggregated report
- Stable design node IDs

### Tooling

**Analyze tools**
- Inspect nodes
- Compute geometry
- Explore graph topology

**Change tools**
- Add/remove nodes
- Modify parameters
- Rewire connections

---

### Suggested Algorithm

1. Group violations by `designNodeId`
2. Analyze context via MCP
3. Select remediation strategy
4. Apply transactional changes
5. Signal completion

---

## 10. Iteration Control

Stop when **any** of the following holds:

- No violations remain
- No progress detected
- Max iterations reached
- Max change operations exceeded

---

## 11. File Layout (Single‑File Constraint)

All code resides in:

```
go/assistant/main.go
```

### Internal Sections

1. Configuration
2. Type definitions
3. Process runners
4. Orchestrator
5. Changer adapter
6. Reporting
7. `main()`

---

## 12. Pseudocode Overview

```go
for iter := 1; iter <= maxIter; iter++ {
    report := runOnce(design)
    if report.HasNoViolations() {
        return COMPLIANT
    }
    result := runChanger(design, report)
    if !result.Modified {
        return STUCK
    }
    design = result.NewDesign
}
```

---

## 13. Outputs

Per iteration:

- `report.iter-N.json`
- `report.iter-N.md`

Final outcome:

- compliant
- stuck
- budget_exhausted

---

## 14. Design Guarantees

- Deterministic
- Parallel
- Auditable
- Extensible to new targets
- Safe via MCP transactions

---

## 15. Future Extensions (Non‑Blocking)

- Target prioritization
- Rule severity weighting
- Speculative changes
- Partial re‑validation
- Visual diff generation

---

## 16. Summary

This architecture provides:

- Clear separation of concerns
- High concurrency
- Strong traceability
- Automated corrective feedback loop

It is optimized for **regulatory design validation at scale** while remaining implementable as a **single Go binary**.

---

**End of document**
