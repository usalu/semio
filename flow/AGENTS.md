# flow

flow is a gui for the [neural engine](../neural/engine/AGENTS.md#engine).


# Flow

A flow is a graphical [directed acyclic graph](../mathematical/graph/port/directed/dag) [neural tree](../neural/AGENTS.md#tree) along with sources and sinks.
The flow data and the tree data are kept separate in order to make sure that the gui is not leaking into the logic.

```json
{"flow":{"components":[{…}],…},"tree":{"neurons":[…],"synapes":…}}
```

# Component

A component is either a function, source or sink.

## Function

A function is the gui enhancement for a neuron.
A function exposes channels for building up the dictionary and splitting the resulting dictionary.

## Source

An source is a component that interactively produces a dictionary.

### Slider

A slider is component for creating a single number dictionary.

```json
{"number":3.1}
```

### Note

A note is a component for creating a single text dictionary.

```json
{"text":"Some text"}
```

## Sink

An sink is either a preview or an action.

### Preview

An sink that previews dictionaries either as text, image or video.

### Action

An action performs side effects for a dictionary.