# flow

flow is a gui for the [neural engine](../neural/engine/AGENTS.md#engine).

# Flow

A flow is a graphical [directed acyclic graph](../mathematical/graph/port/directed/dag) [neural tree](../neural/AGENTS.md#tree) along with sources and sinks.
The flow data and the tree data are kept separate in order to make sure that the gui is not leaking into the logic.

```json
{"flow":{"components":[{…}],…},"tree":{"neurons":[…],"synapes":…}}
```

# Widget

A widget is either a component, source or sink.

## Component

A component is the gui enhancement for a neuron.
A component exposes all keys of the in dictionary as input channels.
A component exposes all keys of the out dictionary as output channels.

## Source

An source is a component that interactively produces a dictionary.

### Slider

A slider is component for creating a single number dictionary.

```json
{ "number": 3.1 }
```

### Note

A note is a component for creating a single text dictionary.

```json
{ "text": "Some text" }
```

## Sink

An sink is either a preview or an action.

### Display

An sink that displays tries to rich-rendered keys as text, image, video, list or when it can't rich-render specific schemas then it rich-renders dictionaries.

### Action

An action performs side effects for a dictionary.

# Channel

A channel is visual port for a key for a dictionary.

A channel has icon and four textual representations: Code, Abbreviation, Name, FullName e.g. S, Srf, Surface, EvaluatedSurface

## Input

Input is a channel for the in dictionary.
All textual representations must be unique among all inputs (they can be appear again in outputs).

## Output

Output is a channel for the out dictionary.
All textual representations must be unique among all outputs (they can be appear again in inputs).
