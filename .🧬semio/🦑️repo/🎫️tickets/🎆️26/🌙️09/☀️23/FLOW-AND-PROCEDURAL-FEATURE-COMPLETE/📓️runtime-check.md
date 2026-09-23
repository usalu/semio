# Generation2d runtime check

Server: `http://127.0.0.1:6021/?plugin=generation2d` (react, port 6021).

## Seen

- The Flow window paints the demo graph. A wire runs from the Number slider (value 3.0) into Add. Add shows input labels a and b and an outgoing wire.
- The Catalogue panel lists the Math section (Abs, Add, Add Variadic, Ceil, Cos, and the rest of that group) plus Inputs and Outputs. Each component row carries `data-draggable="true"`, `data-drag-mime="application/x-flow-widget"`, and a payload such as `{"kind":"neuron","neuronKind":"math.add"}`. A move grip is drawn on the row.

## Not confirmed

- Dragging a catalogue row with the browser drag tool did not leave a new widget on the canvas, and no ghost was visible after the gesture ended. The row's HTML `draggable` attribute is `false`; the app drag is the `data-draggable` pointer gesture. That gesture was not driven, so live preview on the canvas is still unverified in the browser.
