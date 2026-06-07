# flow

flow is a gui for the [neural engine](../neural/engine/AGENTS.md#engine).


# Flow

A flow is a graphical [directed acyclic graph](../mathematical/graph/port/directed/dag) [neural tree](../neural/AGENTS.md#tree) along with inputs and outputs.
The flow data and the tree data are kept separate in order to make sure that the gui is not leaking into the logic.

```json
{"flow":{¡¿³[]},"tree":{"neurons":[…],"synapes":…}}
```

# Widet

A widget is either a neuron, input or output.

## Input

An input is a widget that interactively produces a dictionary.

### Slider

A slider is widget for creating a single number dictionary.

```json
{"number":3.1}
```

### Note

A note is a widget for creating a single text dictionary.

```json
{"text":"Some text"}
```

## Output

An output is either a preview or an action.

### Preview

An output that previews dictionaries either as text, image or video.

### Action

An action performs side effects for a dictionary.