---
technology: sequence
emoji: 📜
---

# Sequence

Sequence is a GUI for the [imperative engine](../imperative/engine/AGENTS.md).

# Sequence

A sequence is a graphical execution-flow graph of imperative steps on an infinite canvas.
Sequence data (steps + flow edges) compiles to an imperative Path and then to text (one line per step).

```json
{"schema":"sequence.fixture/v1","steps":[…],"edges":[…]}
```

# Step

A step is a box on the canvas bound to an imperative action kind.

# Flow Channel

Each step exposes exactly one incoming (`prev`) and one outgoing (`next`) execution-flow channel.
At most one connection each way — the graph must reduce to a single total order.

# Compile

The ordered path can be emitted as imperative source text, one action call per line.
