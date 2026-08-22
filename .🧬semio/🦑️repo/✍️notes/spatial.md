I want to create a pure typescript library for generating shapes called factories.
I have a custom brep kernel (it internally uses brepjs)
Every factory is a state machine (I want to use stately but behind an interface).
Optionally a factory can be passed to a renderer for interactive usage (such as r3f https://www.brepjs.dev/integration/r3f) with undo/redo support, dynamic display on every state, etc
The factories are pure typescript functions they must work headless and interactive.
How would you architect this?

I’ll outline an architecture that keeps the modeling core headless and deterministic, while letting interactive renderers subscribe to the same factory runtime. I’ll verify the current Stately/XState and brepjs integration details before locking the design.

Confirmed the relevant constraints: XState is currently positioned around machines, actors, event-driven workflows, and TypeScript; R3F is a React renderer for Three.js and must stay outside the headless core. BREP.js exposes solids/features/booleans and visualization helpers, so I’d treat it as a kernel backend, not as the public factory API.

Thought for 26s
Core idea

Architect factories as deterministic domain state machines, not as UI tools.

The renderer should never “own” factory logic. It should only:

send typed events into a factory runtime,
subscribe to typed snapshots,
render the current preview/final/display model,
map pointer/keyboard/selection events into factory events.

XState/Stately can power the runtime, but your public API should expose your own StateEngine abstraction. XState is a good fit because it models event-driven workflows, state machines/statecharts, and actors in TypeScript, and can run outside React as well as inside UI frameworks.

Recommended package split
@shape-factories/core
Pure TS contracts:

- FactoryDefinition
- FactoryRuntime
- FactorySnapshot
- FactoryEvent
- FactoryContext
- DisplayModel
- History/transaction interfaces
- kernel-agnostic shape/document types

@shape-factories/kernel
Your public BREP kernel abstraction:

- Kernel
- SolidRef
- FaceRef
- EdgeRef
- MeshPreview
- ShapeOperation
- Boolean, extrude, revolve, sweep, fillet, etc.

@shape-factories/kernel-brepjs
Adapter from your Kernel interface to brepjs.
brepjs should remain private/internal here.

@shape-factories/machines-xstate
XState/Stately implementation of StateEngine.
No renderer dependencies.

@shape-factories/history
Undo/redo, event log, transaction stack, command stack.

@shape-factories/renderer-core
Renderer-neutral interaction adapter:

- pointer events
- selection events
- camera ray input
- display artifact protocol

@shape-factories/r3f
React Three Fiber bindings:

- <FactoryCanvas />
- useFactory()
- useFactoryRuntime()
- DisplayModel → Three/R3F scene objects

@shape-factories/react
Optional UI components:

- command palette
- parameter panels
- state prompts
- history controls

Keep @shape-factories/core with zero React, zero Three.js, zero XState, zero brepjs.

Dependency direction
r3f/react UI
↓
renderer-core
↓
core ← machines-xstate
↓
kernel interface
↓
kernel-brepjs
↓
brepjs

Nothing below renderer-core should know R3F exists. R3F is specifically a React renderer for Three.js, so it belongs at the edge, not in the factory model.

Public mental model

A factory is a typed state machine that produces:

input events → state/context → preview/final geometry/display instructions

A factory is not “a React component”, not “a Three.js object”, and not “a brepjs object”.

It is closer to this:

export interface FactoryDefinition<
TContext,
TEvent,
TResult,
TDisplay = DisplayModel

> {
> id: string;
> version: string;

createInitialContext(input?: unknown): TContext;

machine: FactoryMachineSpec<TContext, TEvent>;

selectors: {
result(ctx: TContext, env: FactoryEnv): TResult | null;
display(ctx: TContext, env: FactoryEnv): TDisplay;
canCommit(ctx: TContext): boolean;
diagnostics?(ctx: TContext): Diagnostic[];
};

history?: FactoryHistoryPolicy<TContext, TEvent>;
}

The important part: the factory state machine owns intent, while the kernel owns geometry execution, and the renderer owns pixels.

Core runtime contract
export interface FactoryRuntime<TEvent, TSnapshot, TResult> {
readonly id: string;

send(event: TEvent): void;

getSnapshot(): TSnapshot;

subscribe(listener: (snapshot: TSnapshot) => void): () => void;

canUndo(): boolean;
canRedo(): boolean;
undo(): void;
redo(): void;

commit(): Promise<TResult>;
cancel(): void;
}

The runtime can be backed by XState, a custom reducer, or a test engine. Your users should not care.

export interface StateEngine<TContext, TEvent> {
start(): void;
stop(): void;
send(event: TEvent): void;
getSnapshot(): FactorySnapshot<TContext>;
subscribe(fn: (s: FactorySnapshot<TContext>) => void): () => void;
}

Then:

export interface StateEngineProvider {
create<TContext, TEvent>(
spec: FactoryMachineSpec<TContext, TEvent>,
options: StateEngineOptions<TContext, TEvent>
): StateEngine<TContext, TEvent>;
}

Your @shape-factories/machines-xstate package implements this using XState. XState exposes createMachine, createActor, send, and subscribe patterns, so it maps naturally to this runtime shape.

Kernel abstraction

Do not leak brepjs types from public factories.

brepjs/BREP.js exposes solids, primitive solids, feature solids, booleans, visualization helpers, etc., including Solid, Face, Edge, Vertex, primitives like cube/cylinder/sphere, and operations such as boolean application. That is enough reason to wrap it behind your own stable kernel boundary.

export interface BrepKernel {
box(params: BoxParams): Promise<SolidRef>;
cylinder(params: CylinderParams): Promise<SolidRef>;
sphere(params: SphereParams): Promise<SolidRef>;

extrude(profile: ProfileRef, params: ExtrudeParams): Promise<SolidRef>;
revolve(profile: ProfileRef, params: RevolveParams): Promise<SolidRef>;
sweep(profile: ProfileRef, path: CurveRef): Promise<SolidRef>;

boolean(
op: "union" | "subtract" | "intersect",
targets: SolidRef[],
tools: SolidRef[]
): Promise<SolidRef>;

tessellate(solid: SolidRef, options?: TessellationOptions): Promise<MeshPreview>;

dispose?(ref: GeometryRef): void;
}

Use opaque refs:

export type SolidRef = Brand<string, "SolidRef">;
export type FaceRef = Brand<string, "FaceRef">;
export type EdgeRef = Brand<string, "EdgeRef">;
export type VertexRef = Brand<string, "VertexRef">;

This gives you freedom to replace brepjs internals later, cache geometry, run workers, serialize operations, or support another kernel.

Factory output: separate geometry from display

A factory should produce two things:

1. Domain result
   export interface FactoryResult {
   operation: ShapeOperation;
   solids: SolidRef[];
   metadata?: Record<string, unknown>;
   }
2. Display model
   export interface DisplayModel {
   prompt?: string;

previews: DisplayPrimitive[];

handles: InteractionHandle[];

selectable?: SelectionQuery[];

constraints?: ConstraintHint[];

diagnostics?: Diagnostic[];
}

Example:

export type DisplayPrimitive =
| { kind: "mesh"; id: string; mesh: MeshPreview; role: "preview" | "final" | "ghost" }
| { kind: "curve"; id: string; points: Vec3[]; role: "guide" | "axis" }
| { kind: "point"; id: string; position: Vec3; role: "handle" }
| { kind: "label"; id: string; text: string; position: Vec3 };

This is critical. Interactive rendering needs more than final BREP geometry: guides, ghosts, handles, snap points, prompts, invalid-state indicators, selection hints, etc. Headless execution only needs the result.

State machine pattern

Every factory should use the same lifecycle envelope:

idle
↓
collecting-input
↓
previewing
↓
ready
↓
committed

cancelled
error

But each factory owns its detailed substates.

Example: box factory

box
idle
pickingFirstCorner
pickingSecondCorner
pickingHeight
ready
committed

Example event shape:

export type BoxFactoryEvent =
| { type: "start" }
| { type: "pointer.move"; point: Vec3; modifiers: InputModifiers }
| { type: "pointer.down"; point: Vec3; modifiers: InputModifiers }
| { type: "set.width"; value: number }
| { type: "set.depth"; value: number }
| { type: "set.height"; value: number }
| { type: "confirm" }
| { type: "cancel" }
| { type: "undo.step" }
| { type: "redo.step" };

Factory context:

export interface BoxFactoryContext {
origin?: Vec3;
corner?: Vec3;
height?: number;

width?: number;
depth?: number;

mode: "two-point" | "centered" | "numeric";

previewSolid?: SolidRef;
}

The factory should not mutate brepjs objects directly inside state transitions. Prefer:

event → context update → derived preview command → kernel execution/cache → snapshot
Headless API

Headless users should be able to do this:

const result = await factories.box.run({
origin: [0, 0, 0],
width: 10,
depth: 20,
height: 5
}, {
kernel
});

Or with state-machine control:

const runtime = factories.box.createRuntime({ kernel });

runtime.send({ type: "start" });
runtime.send({ type: "set.width", value: 10 });
runtime.send({ type: "set.depth", value: 20 });
runtime.send({ type: "set.height", value: 5 });
runtime.send({ type: "confirm" });

const result = await runtime.commit();

Same factory. Different driver.

Interactive API

Interactive R3F usage should look like this:

const runtime = useFactoryRuntime(factories.box, {
kernel,
history,
document,
});

return (
<FactoryInteractionLayer runtime={runtime}>
<FactoryDisplay runtime={runtime} />
</FactoryInteractionLayer>
);

The R3F layer maps pointer events to factory events:

function onPointerMove(hit: SceneHit) {
runtime.send({
type: "pointer.move",
point: hit.worldPoint,
modifiers: getModifiers(),
});
}

And maps DisplayModel into meshes/curves/handles.

R3F supports React v18/v19 pairings depending on major version, so keep it in a dedicated package to avoid forcing React/R3F version constraints onto headless users.

Undo/redo model

Use two history layers.

1. Factory-local step history

For in-progress interactions:

pick origin → pick corner → pick height

Undo means “go back one input step.”

export interface FactoryHistoryPolicy<TContext, TEvent> {
includeEvent(event: TEvent): boolean;
squashWithPrevious?(prev: TEvent, next: TEvent): boolean;
snapshot(ctx: TContext): unknown;
restore(snapshot: unknown): TContext;
}

Pointer moves should usually be transient and not enter history.

includeEvent(event) {
return !event.type.startsWith("pointer.move");
} 2. Document-level command history

After commit:

CreateBoxOperation
DeleteOperation
BooleanOperation
TransformOperation

Undo means “remove or reverse committed document operation.”

export interface DocumentCommand {
id: string;
label: string;

do(doc: ModelDocument, kernel: BrepKernel): Promise<ModelDocument>;
undo(doc: ModelDocument, kernel: BrepKernel): Promise<ModelDocument>;
}

Recommended rule:

Before commit: undo edits the factory state.
After commit: undo edits the document.
Preview strategy

Preview generation should be lazy and cached.

export interface PreviewService {
getPreview(request: PreviewRequest): Promise<DisplayPrimitive[]>;
}

Use content-addressed keys:

const key = hash({
factory: "box",
state: "pickingHeight",
origin,
corner,
height,
tolerances,
});

This gives you:

Case Behavior
pointer move cheap guide/mesh preview
precise numeric input recompute exact BREP
invalid geometry display diagnostic, avoid commit
repeated state cache hit
cancelled factory dispose ephemeral refs

For performance, consider multiple preview qualities:

type PreviewQuality = "fast-mesh" | "exact-brep" | "final";

Interactive pointer movement should usually use fast-mesh; commit should use final.

Transactions

Introduce explicit transactions:

export interface FactoryTransaction {
id: string;
factoryId: string;

transientRefs: GeometryRef[];
committedRefs: GeometryRef[];

commit(): Promise<FactoryResult>;
rollback(): Promise<void>;
}

Interactive usage:

start factory → open transaction
pointer events → transient previews
confirm → commit transaction
cancel → rollback transient refs

Headless usage:

run factory → create operation → commit immediately
Document model

Keep the document as an operation graph, not just a pile of solids.

export interface ModelDocument {
id: string;

nodes: Record<NodeId, ShapeNode>;

history: DocumentCommand[];

selection?: SelectionState;
}
export interface ShapeNode {
id: NodeId;
operation: ShapeOperation;
inputs: NodeId[];
result: SolidRef[];
metadata?: Record<string, unknown>;
}

Example operation:

export type ShapeOperation =
| { type: "primitive.box"; params: BoxParams }
| { type: "primitive.cylinder"; params: CylinderParams }
| { type: "feature.extrude"; profile: ProfileRef; params: ExtrudeParams }
| { type: "boolean.subtract"; targets: NodeId[]; tools: NodeId[] };

This gives you parametric rebuild, undo/redo, replay, serialization, and deterministic testing.

Factory definition example
export const boxFactory: FactoryDefinition<
BoxFactoryContext,
BoxFactoryEvent,
FactoryResult

> = {
> id: "primitive.box",
> version: "1.0.0",

createInitialContext() {
return {
mode: "two-point",
};
},

machine: {
initial: "idle",

    states: {
      idle: {
        on: {
          start: "pickingFirstCorner",
        },
      },

      pickingFirstCorner: {
        on: {
          "pointer.down": {
            target: "pickingSecondCorner",
            assign: ({ event }) => ({
              origin: event.point,
            }),
          },
          cancel: "cancelled",
        },
      },

      pickingSecondCorner: {
        on: {
          "pointer.move": {
            assign: ({ context, event }) => ({
              corner: event.point,
            }),
            transient: true,
          },
          "pointer.down": {
            target: "pickingHeight",
            assign: ({ event }) => ({
              corner: event.point,
            }),
          },
          "set.width": {
            assign: ({ event }) => ({ width: event.value }),
          },
          cancel: "cancelled",
        },
      },

      pickingHeight: {
        on: {
          "pointer.move": {
            assign: ({ event }) => ({
              height: event.point[2],
            }),
            transient: true,
          },
          "set.height": {
            target: "ready",
            assign: ({ event }) => ({
              height: event.value,
            }),
          },
          confirm: {
            target: "ready",
            guard: "hasValidBox",
          },
        },
      },

      ready: {
        on: {
          confirm: "committed",
          cancel: "cancelled",
        },
      },

      committed: { final: true },
      cancelled: { final: true },
    },

},

selectors: {
canCommit(ctx) {
return !!ctx.origin && !!ctx.corner && !!ctx.height && ctx.height > 0;
},

    result(ctx) {
      if (!ctx.origin || !ctx.corner || !ctx.height) return null;

      return {
        operation: {
          type: "primitive.box",
          params: boxParamsFromCorners(ctx.origin, ctx.corner, ctx.height),
        },
        solids: [],
      };
    },

    display(ctx, env) {
      return createBoxDisplayModel(ctx, env);
    },

},

history: {
includeEvent(event) {
return event.type !== "pointer.move";
},

    snapshot(ctx) {
      return structuredClone(ctx);
    },

    restore(snapshot) {
      return snapshot as BoxFactoryContext;
    },

},
};
Renderer adapter

Renderer-neutral core:

export interface InteractionAdapter<TRawEvent> {
toFactoryEvent(raw: TRawEvent, scene: SceneQuery): FactoryEvent | null;
}

R3F adapter:

export function createR3FInteractionAdapter(): InteractionAdapter<ThreeEvent<PointerEvent>> {
return {
toFactoryEvent(event, scene) {
return {
type: event.type === "pointermove" ? "pointer.move" : "pointer.down",
point: event.point.toArray() as Vec3,
modifiers: readModifiers(event.nativeEvent),
};
},
};
}

Display adapter:

export interface DisplayAdapter<TTarget> {
render(display: DisplayModel): TTarget;
}

R3F implementation:

function FactoryDisplay({ runtime }: { runtime: FactoryRuntime<any, any, any> }) {
const snapshot = useFactorySnapshot(runtime);
const display = snapshot.display;

return (
<>
{display.previews.map(renderPrimitive)}
{display.handles.map(renderHandle)}
</>
);
}
State snapshots

Snapshots should be stable, serializable, and renderer-friendly.

export interface FactorySnapshot<TContext = unknown> {
factoryId: string;
state: string;
context: TContext;

display: DisplayModel;

canCommit: boolean;
canCancel: boolean;
canUndo: boolean;
canRedo: boolean;

diagnostics: Diagnostic[];

revision: number;
}

Do not expose internal XState snapshots directly. They are implementation details.

Error handling

Use typed diagnostics instead of thrown errors during interaction.

export interface Diagnostic {
severity: "info" | "warning" | "error";
code: string;
message: string;
target?: string;
}

Examples:

BOX_ZERO_HEIGHT
PROFILE_NOT_CLOSED
BOOLEAN_FAILED
SELF_INTERSECTING_SKETCH
KERNEL_TIMEOUT

During interactive use, errors become display hints. During headless use, commit can throw a typed FactoryCommitError.

Testing strategy

Test each layer separately.

Factory tests
it("creates a box from two corners and height", async () => {
const runtime = createTestRuntime(boxFactory, { kernel: fakeKernel });

runtime.send({ type: "start" });
runtime.send({ type: "pointer.down", point: [0, 0, 0], modifiers: {} });
runtime.send({ type: "pointer.down", point: [10, 20, 0], modifiers: {} });
runtime.send({ type: "set.height", value: 5 });
runtime.send({ type: "confirm" });

expect(runtime.getSnapshot().canCommit).toBe(true);
});
Kernel adapter tests
it("maps primitive.box operation to brepjs cube creation", async () => {
const solid = await kernel.box({
width: 10,
depth: 20,
height: 5,
});

expect(solid).toBeDefined();
});
Renderer tests

Use fake snapshots. Do not require real BREP geometry for most UI tests.

Main design rule

Your public factory API should look like this:

const factory = factories.extrudeProfile;

const runtime = factory.createRuntime({
kernel,
stateEngine,
history,
});

runtime.send({ type: "select.profile", profileId });
runtime.send({ type: "set.distance", value: 25 });
runtime.send({ type: "confirm" });

const result = await runtime.commit();

The same factory should work in:

Node script
browser app
CLI generator
unit test
R3F viewport
future non-React renderer
server-side batch generation
Recommended architecture in one sentence

Build pure, typed, event-driven factory definitions in core; run them through an abstract StateEngine; inject a kernel interface backed by brepjs; expose snapshots with DisplayModel; put undo/redo in factory/document transactions; and keep R3F as a thin adapter that converts pointer events to factory events and snapshots to Three.js objects.

A factory must be entirely static and not depend on typescript. It should be implemtable in the future by other programming languages, with other kernels, etc
Thought for 13s

Then the earlier architecture should be tightened:

A factory is not a TypeScript function. A factory is a static, declarative artifact.
The TypeScript library is only one runtime/interpreter for that artifact.

That changes the design materially.

Revised architecture
Static factory definition
JSON / YAML / binary schema / DSL
↓
Factory runtime / interpreter
TypeScript today
Rust / Python / C++ later
↓
State engine adapter
XState/Stately today
custom interpreter later
↓
Kernel adapter
brepjs today
OCCT / Parasolid / custom kernel later
↓
Renderer adapter
R3F today
CLI / native / webgpu / server later

So the main split becomes:

factory spec ≠ factory runtime ≠ kernel ≠ renderer
Core principle

A factory should be a portable statechart + operation graph definition.

It should contain:

metadata
input/event schemas
context schema
state machine
guards
actions
kernel operation templates
display model templates
history policy
commit behavior
capability requirements

It should not contain:

TypeScript functions
JS closures
brepjs objects
React components
Three.js objects
XState-specific actions
renderer-specific callbacks
kernel-specific geometry objects
Recommended terminology

Use these layers:

Layer Purpose Portable?
Factory Spec Static definition of factory behavior Yes
Factory Runtime Executes/interprets a factory spec Per language
State Engine Runs the statechart Pluggable
Kernel Adapter Executes geometry operations Pluggable
Renderer Adapter Turns display model into UI Pluggable
Host App Owns document, selection, persistence App-specific
Factory spec format

Use a versioned schema:

.factory.json
.factory.yaml
.factory.bin

Example:

{
"kind": "shape-factory",
"schemaVersion": "1.0",
"id": "primitive.box",
"version": "1.0.0",

"requires": {
"kernel": {
"operations": ["primitive.box", "tessellate"],
"units": ["length"],
"coordinateSystem": "right-handed-z-up"
}
},

"context": {
"type": "object",
"properties": {
"origin": { "$ref": "types/Vec3" },
      "corner": { "$ref": "types/Vec3" },
"height": { "type": "number", "unit": "length" },
"mode": {
"type": "string",
"enum": ["two-point", "centered", "numeric"],
"default": "two-point"
}
}
},

"events": {
"start": {},
"pointer.down": {
"point": { "$ref": "types/Vec3" },
      "modifiers": { "$ref": "types/InputModifiers" }
},
"pointer.move": {
"point": { "$ref": "types/Vec3" },
      "modifiers": { "$ref": "types/InputModifiers" }
},
"set.height": {
"value": { "type": "number", "unit": "length" }
},
"confirm": {},
"cancel": {}
},

"machine": {
"initial": "idle",
"states": {
"idle": {
"on": {
"start": {
"target": "pickingFirstCorner"
}
}
},

      "pickingFirstCorner": {
        "on": {
          "pointer.down": {
            "target": "pickingSecondCorner",
            "actions": [
              {
                "op": "assign",
                "path": "origin",
                "value": { "$event": "point" }
              }
            ]
          }
        }
      },

      "pickingSecondCorner": {
        "on": {
          "pointer.move": {
            "transient": true,
            "actions": [
              {
                "op": "assign",
                "path": "corner",
                "value": { "$event": "point" }
              }
            ]
          },
          "pointer.down": {
            "target": "pickingHeight",
            "actions": [
              {
                "op": "assign",
                "path": "corner",
                "value": { "$event": "point" }
              }
            ]
          }
        }
      },

      "pickingHeight": {
        "on": {
          "set.height": {
            "actions": [
              {
                "op": "assign",
                "path": "height",
                "value": { "$event": "value" }
              }
            ]
          },
          "confirm": {
            "target": "ready",
            "guard": "hasValidBox"
          }
        }
      },

      "ready": {
        "on": {
          "confirm": {
            "target": "committed"
          }
        }
      },

      "committed": {
        "final": true
      }
    }

}
}

That is the factory.

The TypeScript package merely loads this and interprets it.

Guards must be declarative

Avoid this:

guard: (ctx) => ctx.height > 0

Use a declarative expression instead:

{
"guards": {
"hasValidBox": {
"all": [
{ "exists": { "path": "origin" } },
{ "exists": { "path": "corner" } },
{ ">": [{ "path": "height" }, 0] }
]
}
}
}

Or:

{
"guards": {
"hasValidBox": {
"expr": "exists(origin) && exists(corner) && height > 0"
}
}
}

I would prefer the first form for maximum portability because it avoids inventing a parser too early.

Actions must also be declarative

Avoid arbitrary code.

Use a small operation vocabulary:

{
"op": "assign",
"path": "height",
"value": { "$event": "value" }
}

Other useful static actions:

assign
clear
append
remove
emit
raise
openTransaction
rollbackTransaction
commitTransaction
requestPreview
requestSelection
requestKernelEval
setDiagnostic
clearDiagnostic

Example:

{
"op": "requestPreview",
"id": "box-preview",
"quality": "fast",
"operation": {
"$template": "preview.box"
}
}
Use a factory IR, not TypeScript

You want a Factory Intermediate Representation.

Factory Spec
↓ parse/validate
Factory IR
↓ execute
Runtime Snapshot

The TypeScript package should expose something like:

const spec = await loadFactorySpec("primitive.box.factory.json");

const factory = compileFactory(spec);

const runtime = createFactoryRuntime(factory, {
stateEngine,
kernel,
renderer: optionalRenderer,
history
});

But compileFactory should mean:

validate schema
normalize defaults
resolve references
check operation compatibility
prepare runtime tables

Not “compile to TypeScript.”

Kernel independence

Do not let factories emit brepjs calls.

Bad:

brepjs.Solid.makeBox(...)

Good:

{
"commit": {
"operation": {
"type": "primitive.box",
"params": {
"origin": { "path": "origin" },
"corner": { "path": "corner" },
"height": { "path": "height" }
}
}
}
}

The factory emits a kernel operation plan:

{
"type": "primitive.box",
"params": {
"origin": [0, 0, 0],
"width": 10,
"depth": 20,
"height": 5
}
}

Then each kernel adapter decides how to execute it:

brepjs adapter → brepjs calls
OCCT adapter → OCCT calls
remote CAD service → HTTP/RPC calls
mock adapter → deterministic test geometry
Kernel capability negotiation

Every factory should declare what it needs:

{
"requires": {
"kernel": {
"operations": [
"primitive.box",
"boolean.subtract",
"tessellate"
],
"topology": [
"solid",
"face",
"edge",
"vertex"
],
"selection": [
"face-picking",
"edge-picking"
]
}
}
}

The runtime can reject unsupported factories before use:

const result = runtime.checkCompatibility(factory, kernel);

if (!result.ok) {
throw new UnsupportedFactoryError(result.missingCapabilities);
}

Example diagnostic:

Factory "feature.fillet" requires kernel capability "edge.fillet",
but current kernel "brepjs-adapter" does not provide it.
Display must be static too

Do not emit Three.js/R3F objects.

The factory should emit renderer-neutral display instructions:

{
"display": {
"states": {
"pickingSecondCorner": [
{
"kind": "point",
"id": "origin-marker",
"position": { "path": "origin" },
"role": "anchor"
},
{
"kind": "box-preview",
"id": "preview",
"params": {
"origin": { "path": "origin" },
"corner": { "path": "corner" },
"height": { "const": 0.01 }
},
"role": "preview"
}
],

      "pickingHeight": [
        {
          "kind": "box-preview",
          "id": "preview",
          "params": {
            "origin": { "path": "origin" },
            "corner": { "path": "corner" },
            "height": { "path": "height" }
          },
          "role": "preview"
        },
        {
          "kind": "linear-handle",
          "id": "height-handle",
          "axis": [0, 0, 1],
          "origin": { "path": "corner" },
          "role": "handle"
        }
      ]
    }

}
}

Renderer adapters interpret this:

R3F adapter → Three meshes, handles, labels
CLI adapter → textual prompts
headless adapter → no display
native adapter → native viewport entities
Static selection model

Interactive factories often require selection.

Do not make selection renderer-specific.

Use declarative selection requests:

{
"selection": {
"states": {
"selectProfile": {
"accept": ["profile.closed", "face.planar"],
"multiple": false,
"prompt": "Select a closed profile or planar face"
},

      "selectPath": {
        "accept": ["curve", "edge"],
        "multiple": false,
        "prompt": "Select sweep path"
      }
    }

}
}

Renderer provides hits in a normalized form:

{
"type": "selection.hit",
"target": {
"kind": "face",
"id": "face-123",
"nodeId": "body-7"
},
"point": [1, 2, 3],
"normal": [0, 0, 1]
}

The factory stays portable.

History should be declarative
{
"history": {
"factory": {
"mode": "snapshot",
"excludeEvents": ["pointer.move"],
"transactionalEvents": ["pointer.down", "set.height", "confirm"]
},

    "document": {
      "commitAs": {
        "type": "create-node",
        "label": "Create Box"
      }
    }

}
}

This lets the runtime decide:

pointer.move → transient, no history entry
pointer.down → factory-local undo checkpoint
set.height → factory-local undo checkpoint
confirm/commit → document command
State engine independence

Do not expose XState/Stately in the factory spec.

Use your own statechart schema:

{
"machine": {
"initial": "idle",
"states": {
"idle": {
"on": {
"start": {
"target": "active"
}
}
}
}
}
}

Then the TypeScript runtime can translate this into XState internally:

Factory statechart schema
↓
XState machine config
↓
XState actor

A future Rust runtime can interpret the same statechart directly.

So Stately becomes a backend:

interface StateEngineBackend {
createMachine(ir: StateMachineIR): StateMachineInstance;
}

Implementations:

xstate-backend
pure-ts-backend
rust-backend
python-backend
debug-visualizer-backend
Static expression system

You need a small portable expression language.

Use it for:

guards
computed params
display params
derived measurements
validations
diagnostics

Example expression AST:

{
"let": {
"dx": {
"-": [
{ "path": "corner.0" },
{ "path": "origin.0" }
]
},
"dy": {
"-": [
{ "path": "corner.1" },
{ "path": "origin.1" }
]
}
},
"in": {
"all": [
{ ">": [{ "abs": { "var": "dx" } }, 0] },
{ ">": [{ "abs": { "var": "dy" } }, 0] },
{ ">": [{ "path": "height" }, 0] }
]
}
}

Recommended primitive expression ops:

path
const
event
var
exists
all
any
not
==

!=

> =
> <
> <=

-

*

- /
  abs
  min
  max
  clamp
  round
  distance
  normalize
  dot
  cross
  project
  planeIntersect

Keep it intentionally small.

Factory lifecycle

Every runtime, regardless of language, should expose the same conceptual lifecycle:

load spec
validate spec
compile to IR
check host/kernel capabilities
create runtime instance
send event
transition machine
evaluate guards/actions
update context
evaluate display
evaluate preview plan
emit snapshot
commit or cancel
dispose transient resources
Snapshot format

Snapshots should also be language-neutral:

{
"factoryId": "primitive.box",
"state": "pickingHeight",
"revision": 17,

"context": {
"origin": [0, 0, 0],
"corner": [10, 20, 0],
"height": 5
},

"display": {
"prompt": "Set height",
"items": [
{
"kind": "box-preview",
"id": "preview",
"role": "preview",
"params": {
"origin": [0, 0, 0],
"corner": [10, 20, 0],
"height": 5
}
}
]
},

"capabilities": {
"canCommit": true,
"canCancel": true,
"canUndo": true,
"canRedo": false
},

"diagnostics": []
}

R3F subscribes to this. Headless code can ignore display.

Suggested package architecture
@factories/spec
JSON Schema / canonical schema definitions
shared vocabulary
statechart schema
expression schema
kernel operation schema
display schema

@factories/core
TypeScript parser
validator
IR builder
runtime interfaces
snapshot model
event model

@factories/runtime-ts
Pure TS factory interpreter
no React
no brepjs
no XState required

@factories/runtime-xstate
State engine adapter using XState internally

@factories/kernel-api
Kernel capability and operation interfaces

@factories/kernel-brepjs
brepjs adapter

@factories/renderer-api
Display model and interaction contracts

@factories/r3f
React Three Fiber renderer adapter

@factories/cli
Optional headless/terminal driver

@factories/conformance
Test vectors for factory/runtime/kernel compatibility

Most important package:

@factories/spec

That is your real product boundary.

Factory authoring model

You can offer multiple authoring paths:

1. Raw JSON/YAML

Portable, verbose, canonical.

box.factory.json 2. TypeScript builder

Developer-friendly, but outputs static spec.

const box = defineFactory(...)
export default box.toJSON()

The generated JSON is the factory. The TS builder is only tooling.

3. Visual Stately authoring

If Stately is useful, let users design statecharts visually, then export into your canonical factory statechart schema.

4. Future Python/Rust builders

Same idea:

box = Factory(...)
box.write_json("box.factory.json")

Again: builders are not factories. They generate factories.

Important distinction

Avoid this API as the source of truth:

export const boxFactory = defineFactory({
guards: {
valid: (ctx) => ctx.height > 0
}
});

Prefer this:

export const boxFactorySpec = defineFactory({
guards: {
valid: gt(path("height"), constValue(0))
}
}).toJSON();

The first embeds TypeScript behavior.

The second generates static data.

Handling non-trivial logic

Eventually some factories may need complex geometry-specific behavior.

Examples:

infer profile plane
find nearest tangent edge chain
compute robust sweep frame
derive fillet propagation

Do not solve this with arbitrary TypeScript inside the factory.

Use one of these:

Option A: add declarative operations to the factory vocabulary

Best for common modeling patterns.

{
"op": "derive.edgeChain",
"input": { "path": "selectedEdge" },
"mode": "tangent-continuous"
}
Option B: delegate to kernel capability

Good when the operation is kernel-dependent.

{
"op": "kernel.query",
"capability": "edge.tangentChain",
"params": {
"edge": { "path": "selectedEdge" },
"tolerance": 0.001
}
}
Option C: use registered host services

Good for non-geometry external behavior.

{
"op": "service.call",
"service": "units.convert",
"params": {
"value": { "path": "height" },
"from": "mm",
"to": "document.lengthUnit"
}
}
Option D: use WASM plugins

Only if unavoidable.

{
"extension": {
"kind": "wasm",
"id": "com.example.advanced-loft",
"exports": ["deriveSections", "validateContinuity"]
}
}

But this weakens the “entirely static” goal. I would keep WASM as an escape hatch, not the default.

Recommended factory schema sections

A mature static factory could look like this:

{
"kind": "shape-factory",
"schemaVersion": "1.0",

"id": "feature.extrude",
"version": "1.0.0",
"label": "Extrude",

"parameters": {},
"context": {},
"events": {},
"guards": {},
"actions": {},
"machine": {},
"selection": {},
"display": {},
"preview": {},
"commit": {},
"history": {},
"requires": {},
"diagnostics": {},
"metadata": {}
}

Where:

Section Purpose
parameters external factory parameters
context internal state
events accepted events
guards declarative conditions
actions named reusable static actions
machine statechart
selection selection requirements
display renderer-neutral visual model
preview transient geometry/display
commit final document operation
history undo/redo behavior
requires capabilities
diagnostics validation messages
metadata labels, docs, icons, tags
Static commit operation

Example:

{
"commit": {
"when": "hasValidBox",
"documentCommand": {
"type": "create-node",
"label": "Create Box",
"node": {
"operation": {
"type": "primitive.box",
"params": {
"origin": { "path": "origin" },
"corner": { "path": "corner" },
"height": { "path": "height" }
}
}
}
}
}
}

The runtime evaluates this into:

{
"type": "create-node",
"label": "Create Box",
"node": {
"operation": {
"type": "primitive.box",
"params": {
"origin": [0, 0, 0],
"corner": [10, 20, 0],
"height": 5
}
}
}
}

Then the document system applies it.

What TypeScript should provide

Your TypeScript library should provide:

schema validation
spec loading
IR normalization
runtime execution
XState backend adapter
brepjs kernel adapter
R3F renderer adapter
test harness
authoring helpers
developer tooling

But it should not make TypeScript the factory format.

The runtime API can still be TypeScript:

const spec = await loadFactorySpec(url);

const runtime = createRuntime(spec, {
kernel,
stateEngine,
document,
history
});

runtime.send({
type: "pointer.down",
point: [0, 0, 0],
modifiers: {}
});

But the factory remains static.

Best architecture statement
Factories are static, portable, declarative statechart documents.

A runtime interprets a factory document against:

- a state engine,
- a kernel capability provider,
- a document model,
- an optional renderer,
- an optional history provider.

TypeScript, XState, brepjs, and R3F are replaceable implementation details.
Main recommendation

Design the system around a canonical Factory Specification, not around TypeScript APIs.

The TypeScript implementation should be only the first conforming runtime.

Use:

static schema
portable statechart
declarative guards/actions
kernel operation IR
renderer-neutral display IR
capability negotiation
conformance test suite

That gives you:

headless execution
interactive rendering
undo/redo
future language runtimes
future kernels
future renderers
deterministic replay
serializable factories
testable behavior
long-term compatibility
The brep kernel entirely uses this terminology:
ditable classes:

Vertex: A Vertex is a zero-dimensional entity equivalent to a geometry point.
Edge: An Edge is a one-dimensional entity defined by two vertices. It is important to note that while a topologic edge is made of two vertices, its geometry can be a curve with multiple control vertices.
Wire: A Wire is a contiguous collection of Edges where adjacent Edges are connected by shared Vertices. It may be open or closed and may be manifold or non-manifold.
Face: A Face is a two-dimensional region defined by a collection of closed Wires. The geometry of a face can be flat or undulating.
Shell: A Shell is a contiguous collection of Faces, where adjacent Faces are connected by shared Edges. It may be open or closed and may be manifold or non-manifold.
Cell: A Cell is a three-dimensional region defined by a collection of closed Shells. It may be manifold or non- manifold.
CellComplex: A CellComplex is a contiguous collection of Cells where adjacent Cells are connected by shared Faces. It is non- manifold.
Cluster: A Cluster is a collection of any topologic entities. It may be contiguous or not and may be manifold or non- manifold. Clusters can be nested within other Clusters.

Non-editable classes:

Surfaces are derived faces that are a combination of Exposure (External or Internal) and Stance (Horizontal or Vertical).
e.g. two coplanar faces are merged into a single surface
e.g. when two cells intersect the surface will not be the complete face but it is split into external and internal faces.
Surfaces are just a different way of "splitting the faces semantically". e.g. the total area or shape doesnt change

Parts are derived cells that are a combination of Overlap (None, Difference, Intersection)
Parts are just a different way of "splitting the cells semantically". e.g. the total volume or shape doesnt change
Thought for 4s

This terminology fits well, but it changes the architecture slightly:

Factories should operate on editable topological entities, while Surfaces and Parts should be treated as derived semantic views.

So the static factory system should distinguish:

Editable topology:
Vertex → Edge → Wire → Face → Shell → Cell → CellComplex → Cluster

Derived semantic views:
Surface = semantic partition / merge of Faces
Part = semantic partition / classification of Cells
Revised kernel model

1. Editable topology graph

Your core model should be a topological graph:

Cluster
contains any topology entity

CellComplex
contains Cells

Cell
bounded by closed Shells

Shell
contains connected Faces

Face
bounded by closed Wires

Wire
contains connected Edges

Edge
bounded by Vertices

Vertex
has point geometry

In static factory schema terms:

{
"topologyKinds": [
"vertex",
"edge",
"wire",
"face",
"shell",
"cell",
"cellComplex",
"cluster"
]
}

Every editable entity should have:

type TopologyKind =
| "vertex"
| "edge"
| "wire"
| "face"
| "shell"
| "cell"
| "cellComplex"
| "cluster";

interface TopologyRef {
id: string;
kind: TopologyKind;
}

Keep refs opaque and portable.

2. Geometry is not topology

Your terminology already makes this distinction clear:

Topology Geometry
Vertex point
Edge curve
Wire curve chain / loop
Face surface patch / region
Shell connected face set
Cell volume region
CellComplex connected cell set
Cluster arbitrary collection

Important implication:

Edge ≠ curve
Face ≠ surface
Cell ≠ solid

An edge can have a complex curve. A face can have flat or undulating geometry. A wire can be open, closed, manifold, or non-manifold. A shell can be open or closed.

So avoid APIs named only after geometry concepts like:

SolidRef
MeshRef
SurfaceRef
CurveRef

Prefer kernel terms:

VertexRef
EdgeRef
WireRef
FaceRef
ShellRef
CellRef
CellComplexRef
ClusterRef

For geometry attachments:

interface GeometryDescriptor {
kind: "point" | "curve" | "surface" | "volume" | "mesh";
dataRef: string;
} 3. Surfaces and Parts are derived views, not edit targets

Your non-editable classes should not be modeled as independent editable topology.

They are semantic projections.

Surface = view over one or more Face regions
Part = view over one or more Cell regions

That means:

Face is editable.
Surface is selectable/queryable/displayable.

Cell is editable.
Part is selectable/queryable/displayable.

A factory can select a Surface or Part, but any mutation must resolve back to the underlying editable entities.

Suggested classification
type EditableEntityKind =
| "vertex"
| "edge"
| "wire"
| "face"
| "shell"
| "cell"
| "cellComplex"
| "cluster";

type DerivedEntityKind =
| "surface"
| "part";

type KernelEntityKind = EditableEntityKind | DerivedEntityKind;

But keep hard rules:

editable(entity.kind) === true:
vertex, edge, wire, face, shell, cell, cellComplex, cluster

editable(entity.kind) === false:
surface, part
Static schema consequence

Factories should declare which entity kinds they accept:

{
"selection": {
"states": {
"selectTarget": {
"accept": [
"face",
"surface",
"cell",
"part"
],
"multiple": false
}
}
}
}

But mutation capabilities should target editable kinds:

{
"commit": {
"operation": {
"type": "topology.offsetFaces",
"targets": {
"fromSelection": "selectTarget",
"resolve": "editableFaces"
},
"params": {
"distance": { "path": "distance" }
}
}
}
}

Meaning:

selected Face → mutate that Face
selected Surface → resolve to contributing Faces, then mutate those Faces
selected Cell → mutate that Cell
selected Part → resolve to contributing Cells / split regions, then mutate Cells
Entity identity model

You likely need two ID classes:

interface EditableRef {
id: string;
kind: EditableEntityKind;
}

interface DerivedRef {
id: string;
kind: DerivedEntityKind;

derivedFrom: EditableRef[];
classification: Record<string, string>;
}

Example Surface:

{
"id": "surface-42",
"kind": "surface",
"derivedFrom": [
{ "id": "face-1", "kind": "face" },
{ "id": "face-2", "kind": "face" }
],
"classification": {
"exposure": "external",
"stance": "vertical"
}
}

Example Part:

{
"id": "part-8",
"kind": "part",
"derivedFrom": [
{ "id": "cell-3", "kind": "cell" }
],
"classification": {
"overlap": "intersection"
}
}
Surface model

A Surface is not a BREP surface in the classical geometric sense here. In your terminology it is a semantic face-region view.

So I would define it like this:

interface SurfaceView {
id: string;
kind: "surface";

sourceFaces: FaceRef[];

exposure: "external" | "internal";
stance: "horizontal" | "vertical";

area: number;
boundary?: WireRef[];

displayGeometryRef?: string;
}

Important rules:

Surface may merge coplanar Faces.
Surface may split Faces into semantic external/internal portions.
Surface does not alter total area.
Surface does not own editable topology.
Surface is recomputed after topology-changing operations.

Factory use cases:

Factory Can select Surface? Mutates Surface directly?
paint/material assignment Yes Maybe metadata only
offset/extrude Yes No, resolves to Faces
measure area Yes No
classify envelope Yes No
delete Dangerous Should resolve to Faces or reject
Part model

A Part is a semantic cell-region view.

interface PartView {
id: string;
kind: "part";

sourceCells: CellRef[];

overlap: "none" | "difference" | "intersection";

volume: number;
displayGeometryRef?: string;
}

Rules:

Part may split Cell regions semantically.
Part does not alter total volume.
Part does not own editable topology.
Part is recomputed after cell/topology operations.

Factory use cases:

Factory Can select Part? Mutates Part directly?
volume analysis Yes No
overlap inspection Yes No
create room/zone labels Yes Metadata only
boolean-derived editing Yes Resolves to Cells / operation graph
delete geometry Should resolve or reject No direct mutation
Operation vocabulary should use your topology terms

Avoid CAD-generic names like:

solid.create
curve.create
surface.create
body.boolean

Use your kernel vocabulary:

vertex.create
edge.create
wire.create
face.create
shell.create
cell.create
cellComplex.create
cluster.create

cell.boolean
cell.split
face.offset
wire.extrudeToCell
face.thickenToCell
shell.closeToCell
cluster.group
cluster.ungroup

Examples:

{
"type": "cell.createBox",
"params": {
"origin": { "path": "origin" },
"xSize": { "path": "width" },
"ySize": { "path": "depth" },
"zSize": { "path": "height" }
}
}
{
"type": "wire.extrudeToCell",
"params": {
"wire": { "path": "profileWire" },
"distance": { "path": "distance" },
"direction": { "path": "direction" }
}
}
{
"type": "cell.boolean",
"params": {
"operation": "difference",
"targets": { "path": "targetCells" },
"tools": { "path": "toolCells" }
}
}
Factory capability declarations

Factories should declare topology capabilities, not brepjs capabilities.

Example: box factory

{
"requires": {
"kernel": {
"editableEntities": ["vertex", "edge", "wire", "face", "shell", "cell"],
"operations": ["cell.createBox", "cell.tessellate"],
"queries": ["cell.boundary", "face.area"]
}
}
}

Example: surface-based offset factory

{
"requires": {
"kernel": {
"editableEntities": ["face", "shell", "cell"],
"derivedEntities": ["surface"],
"operations": ["face.offset", "shell.rebuild", "cell.rebuild"],
"queries": [
"surface.resolveFaces",
"face.normal",
"face.area"
]
}
}
}

Example: overlap/part analysis factory

{
"requires": {
"kernel": {
"editableEntities": ["cell"],
"derivedEntities": ["part"],
"operations": ["cell.intersect", "part.compute"],
"queries": ["part.volume", "part.sourceCells"]
}
}
}
Derived view lifecycle

Surfaces and Parts should be managed by a derived view service.

Editable topology changes
↓
invalidate derived views
↓
recompute Surfaces / Parts
↓
emit stable derived refs where possible
↓
renderer updates semantic overlays

Architecturally:

interface DerivedViewService {
computeSurfaces(input: SurfaceComputationInput): Promise<SurfaceView[]>;
computeParts(input: PartComputationInput): Promise<PartView[]>;

resolveSurface(surface: SurfaceRef): Promise<FaceRef[]>;
resolvePart(part: PartRef): Promise<CellRef[]>;
}

Factories should not own this logic directly. They should request it declaratively:

{
"op": "query.derived",
"kind": "surface",
"params": {
"exposure": "external",
"stance": "vertical"
},
"assignTo": "externalVerticalSurfaces"
}
Static factory example: select Surface, resolve to Faces
{
"id": "feature.offsetSurface",
"schemaVersion": "1.0",

"context": {
"selectedSurface": { "kind": "surface" },
"resolvedFaces": {
"type": "array",
"items": { "kind": "face" }
},
"distance": { "type": "number", "unit": "length" }
},

"selection": {
"states": {
"selectSurface": {
"accept": ["surface"],
"multiple": false,
"prompt": "Select surface"
}
}
},

"machine": {
"initial": "selectSurface",
"states": {
"selectSurface": {
"on": {
"selection.changed": {
"target": "setDistance",
"actions": [
{
"op": "assign",
"path": "selectedSurface",
"value": { "$event": "selection.0" }
},
{
"op": "kernel.query",
"query": "surface.resolveFaces",
"params": {
"surface": { "$event": "selection.0" }
},
"assignTo": "resolvedFaces"
}
]
}
}
},

      "setDistance": {
        "on": {
          "set.distance": {
            "actions": [
              {
                "op": "assign",
                "path": "distance",
                "value": { "$event": "value" }
              }
            ]
          },
          "confirm": {
            "target": "ready",
            "guard": "hasFacesAndDistance"
          }
        }
      },

      "ready": {
        "on": {
          "confirm": {
            "target": "committed"
          }
        }
      },

      "committed": {
        "final": true
      }
    }

},

"guards": {
"hasFacesAndDistance": {
"all": [
{ "notEmpty": { "path": "resolvedFaces" } },
{ "!=": [{ "path": "distance" }, 0] }
]
}
},

"commit": {
"operation": {
"type": "face.offset",
"params": {
"faces": { "path": "resolvedFaces" },
"distance": { "path": "distance" }
}
}
}
}

This keeps the factory portable and static while respecting your derived Surface concept.

Renderer model

The renderer should understand both editable and derived entities.

type RenderableEntity =
| EditableRef
| DerivedRef;

Display roles:

editable topology:
normal geometry display
selectable topology
edit handles
preview topology

derived views:
semantic overlay
classification colors
analytical labels
hover/selection layer

Example display instruction:

{
"kind": "entity-highlight",
"target": { "path": "selectedSurface" },
"role": "semantic-selection",
"style": {
"classification": ["exposure", "stance"]
}
}

For R3F:

Face/Cell display → geometry layer
Surface/Part display → semantic overlay layer
Handles → interaction layer
Diagnostics → annotation layer
Document model

The document should primarily store editable topology and operation history.

interface ModelDocument {
topology: TopologyGraph;
operations: OperationNode[];
derivedViews: DerivedViewCache;
metadata: DocumentMetadata;
}

Do not store Surfaces and Parts as authoritative geometry unless needed as a cache.

interface DerivedViewCache {
surfaces: Record<string, SurfaceView>;
parts: Record<string, PartView>;
revision: number;
validForTopologyRevision: number;
}

After an operation:

apply editable operation
increment topology revision
invalidate derived views
recompute derived views lazily or eagerly
update renderer
Static selection semantics

Selection events should include the entity kind and whether it is editable.

{
"type": "selection.changed",
"selection": [
{
"id": "surface-12",
"kind": "surface",
"editable": false,
"derivedFrom": [
{ "id": "face-3", "kind": "face" },
{ "id": "face-4", "kind": "face" }
]
}
]
}

This allows factories to make explicit decisions:

{
"guard": {
"==": [
{ "path": "selected.kind" },
"surface"
]
}
}

Or:

{
"op": "resolveEditable",
"input": { "path": "selected" },
"accept": ["face"],
"assignTo": "targetFaces"
}
Recommended static operation families
Topology construction
vertex.create
edge.create
wire.create
face.create
shell.create
cell.create
cellComplex.create
cluster.create
Topology modification
vertex.move
edge.split
wire.close
face.split
face.offset
shell.stitch
shell.close
cell.split
cell.boolean
cellComplex.merge
cluster.add
cluster.remove
Topology queries
entity.kind
entity.bounds
entity.children
entity.parents
entity.adjacency

vertex.point
edge.vertices
edge.curve
wire.edges
wire.isClosed
face.wires
face.area
face.normal
shell.faces
shell.isClosed
cell.shells
cell.volume
cellComplex.cells
cluster.members
Derived semantic queries
surface.compute
surface.resolveFaces
surface.exposure
surface.stance
surface.area

part.compute
part.resolveCells
part.overlap
part.volume
Rendering / preview
entity.tessellate
entity.outline
entity.centroid
surface.displayMesh
part.displayMesh
Design rule for factories

A factory should declare three separate things:

What it can select
What it can resolve
What it can mutate

Example:

{
"selection": {
"accept": ["surface", "face"]
},

"resolution": {
"surface": "face[]",
"face": "face"
},

"mutation": {
"targets": ["face"],
"operation": "face.offset"
}
}

That prevents ambiguity around derived entities.

Naming recommendation

Because your Surface is semantic and not simply geometric, consider using explicit names internally:

SurfaceView
PartView

Public user-facing names can remain:

Surface
Part

But the architecture benefits from explicit internal separation:

Face editable topology
SurfaceView derived semantic face view

Cell editable topology
PartView derived semantic cell view

This prevents future confusion with geometric surfaces.

Best-fit final architecture
Static Factory Spec
uses:
events
guards
actions
selection rules
display rules
topology operations
derived-view queries

Factory Runtime
interprets the spec
owns state/context/history
emits snapshots

Kernel Adapter
executes topology operations using:
Vertex
Edge
Wire
Face
Shell
Cell
CellComplex
Cluster

Derived View Service
computes non-editable:
Surface = classified/merged/split Face view
Part = classified/split Cell view

Renderer Adapter
displays:
editable topology
semantic derived views
handles
previews
diagnostics
Critical invariant
Factories may select Surfaces and Parts.
Factories may display Surfaces and Parts.
Factories may query Surfaces and Parts.

Factories must not directly mutate Surfaces or Parts.

Any mutation must resolve back to:
Face / Shell / Cell / CellComplex / Cluster
or create a new editable topology operation.

That gives you a clean, portable, static architecture compatible with TypeScript today, other languages later, brepjs now, and other kernels later.
