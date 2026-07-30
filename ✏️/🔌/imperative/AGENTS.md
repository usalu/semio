---
technology: imperative
emoji: ⚙️
---

# Imperative

Imperative is a headless procedural language for ordered side effects: dictionary in, dictionary out, but over a **Path** (linear ordered steps), not a Tree/DAG.

# Path

A path is an ordered list of steps. Position is execution order; there are no edges or synapses.

# Step

A step is an instance of an action kind with concrete params.

# Action

An action triggers a side effect in scope (log, state mutation, delay, …).

# Scope

A running dictionary threaded from step to step; each step's output merges back into scope.

# Dictionary

Same as neural: immutable, unordered, collision-free key-value collection following a schema.
