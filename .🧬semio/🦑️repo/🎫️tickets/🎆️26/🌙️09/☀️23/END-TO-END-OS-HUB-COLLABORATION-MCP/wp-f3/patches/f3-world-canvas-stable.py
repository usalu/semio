#!/usr/bin/env python3
"""🌍️ F3 (session 14b) — `<Canvas>` of `WorldCanvas` renders only when its own configuration changes.

Every World3dHost render re-rendered r3f's `<Canvas>` (inline style, dpr, gl, handler props, fresh children), and each Canvas
render re-runs r3f 9.7's `configure`, whose size check compares the measured container rect (react-use-measure: 8 keys)
against the stored size (4 keys) with `is.equ(…, shallowLoose)` — never equal — so it ALWAYS publishes a fresh `size` and
`viewport`: every `useThree` subscriber of `size` or of the whole store (grid helper, LOD runner, hit stamps, auto-fit,
orbit gizmo + its 120-fiber hit heads) re-rendered 2× per canvas per world hover (`generated/f3-render-census-puzzle-r3f1.json`).
After: the Canvas element is memoized on its configuration, handlers read the latest props through a ref, and the children
reach the r3f root through a slot component that re-renders alone.

Idempotent; `--dry-run` reports without writing. usage: python3 f3-world-canvas-stable.py [--dry-run]
"""
import sys

R3F = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx"

HUNKS = [
    ("""/** @emoji 🌍️ Generic infinite-world r3f canvas shell (`frameloop="demand"`).
""", """/** 🧺️ How a {@link WorldCanvas}'s children reach its r3f root without re-rendering `<Canvas>`: the latest children and the
 * setters of the mounted {@link WorldCanvasChildren}. */
type WorldCanvasChildrenSlotV1 = { current: ReactNode; readonly listeners: Set<React.Dispatch<React.SetStateAction<ReactNode>>> };

/** 🪣️ Renders the latest children of its {@link WorldCanvasChildrenSlotV1} inside the r3f root — the only part of the canvas a
 * parent render reaches. */
function WorldCanvasChildren(props: { readonly slot: WorldCanvasChildrenSlotV1 }): ReactNode {
  const [children, setChildren] = reactHostPort.useState<ReactNode>(() => props.slot.current);
  reactHostPort.useLayoutEffect(() => {
    const slot = props.slot;
    slot.listeners.add(setChildren);
    setChildren(() => slot.current);
    return () => {
      slot.listeners.delete(setChildren);
    };
  }, [props.slot]);
  return children;
}

const WORLD_CANVAS_STYLE: CSSProperties = { height: "100%", width: "100%" };
const WORLD_CANVAS_DEFAULT_DPR: [number, number] = [1, 2];
const WORLD_CANVAS_DEFAULT_GL = { antialias: true };

/** @emoji 🌍️ Generic infinite-world r3f canvas shell (`frameloop="demand"`).
"""),
    (""" * nothing else, so the event scope is exactly the one r3f's inner div had and `props.overlay` stays
 * outside it. */
export function WorldCanvas(props: WorldCanvasProps): ReactElement {""", """ * nothing else, so the event scope is exactly the one r3f's inner div had and `props.overlay` stays
 * outside it.
 *
 * 📐️ `<Canvas>` renders only when its own configuration changes: a Canvas render re-runs r3f's `configure`, which publishes a
 * fresh `size` + `viewport` on EVERY call (it compares the 8-key measured rect with the 4-key stored size), re-rendering every
 * `useThree` size subscriber — twice per canvas per world hover (ticket 26/09/23 F3). Handlers read the latest props through
 * a ref and the children travel through {@link WorldCanvasChildren}. */
export function WorldCanvas(props: WorldCanvasProps): ReactElement {"""),
    ("""  const onWheelRef = reactHostPort.useRef(props.onWheel);
  const [canvasEventSource, setCanvasEventSource] = reactHostPort.useState<HTMLDivElement | null>(null);
  onWheelRef.current = props.onWheel;""", """  const onWheelRef = reactHostPort.useRef(props.onWheel);
  const [canvasEventSource, setCanvasEventSource] = reactHostPort.useState<HTMLDivElement | null>(null);
  onWheelRef.current = props.onWheel;
  const propsRef = reactHostPort.useRef(props);
  propsRef.current = props;
  const [childrenSlot] = reactHostPort.useState<WorldCanvasChildrenSlotV1>(() => ({ current: props.children, listeners: new Set() }));
  reactHostPort.useLayoutEffect(() => {
    childrenSlot.current = props.children;
    for (const listener of childrenSlot.listeners) listener(() => props.children);
  }, [childrenSlot, props.children]);
  const cameraOptions = ownedCamera
    ? {
        up: [...cameraUp] as [number, number, number],
        position: [...props.cameraPosition!] as [number, number, number],
        fov: props.cameraFov ?? 45,
        ...(props.cameraNear !== undefined ? { near: props.cameraNear } : {}),
        ...(props.cameraFar !== undefined ? { far: props.cameraFar } : {}),
      }
    : undefined;
  const cameraKey = JSON.stringify(cameraOptions ?? null);
  const dpr = props.dpr ?? WORLD_CANVAS_DEFAULT_DPR;
  const dprKey = JSON.stringify(dpr);
  const gl = props.gl ?? WORLD_CANVAS_DEFAULT_GL;
  const glKey = typeof gl === "function" ? gl : JSON.stringify(gl);
  const canvas = reactHostPort.useMemo(
    () =>
      canvasEventSource ? (
        <Canvas
          eventSource={canvasEventSource}
          frameloop={frameloop}
          style={WORLD_CANVAS_STYLE}
          dpr={dpr}
          shadows={props.shadows}
          camera={cameraOptions}
          gl={gl}
          onPointerDown={(event) => propsRef.current.onPointerDown?.(event.nativeEvent)}
          onPointerMove={(event) => propsRef.current.onPointerMove?.(event.nativeEvent)}
          onPointerUp={(event) => propsRef.current.onPointerUp?.(event.nativeEvent)}
          onPointerLeave={(event) => propsRef.current.onPointerLeave?.(event.nativeEvent)}
          onPointerCancel={(event) => propsRef.current.onPointerCancel?.(event.nativeEvent)}
          onWheel={(event) => propsRef.current.onWheel?.(event.nativeEvent)}
          onContextMenu={(event) => propsRef.current.onContextMenu?.(event)}
          onDoubleClick={(event) => propsRef.current.onDoubleClick?.(event.nativeEvent)}
          onLostPointerCapture={(event) => propsRef.current.onLostPointerCapture?.(event.nativeEvent)}
          onPointerMissed={(event) => propsRef.current.onPointerMissed?.(event)}
          onCreated={({ camera, gl: renderer }) => {
            propsRef.current.onCanvasReady?.({ camera, domElement: renderer.domElement });
          }}
        >
          {frameloop === "demand" ? <DemandFrameloopKick /> : null}
          {props.background ? <color attach="background" args={[props.background]} /> : null}
          <WorldLayerStack>
            <WorldCanvasChildren slot={childrenSlot} />
          </WorldLayerStack>
        </Canvas>
      ) : null,
    [canvasEventSource, frameloop, dprKey, props.shadows, cameraKey, glKey, props.background, childrenSlot],
  );"""),
    ("""      <div ref={setCanvasEventSource} style={{ width: "100%", height: "100%" }}>
        {canvasEventSource ? (
          <Canvas
            eventSource={canvasEventSource}
            frameloop={frameloop}
            style={{ height: "100%", width: "100%" }}
            dpr={props.dpr ?? [1, 2]}
            shadows={props.shadows}
            camera={
              ownedCamera
                ? {
                    up: [...cameraUp] as [number, number, number],
                    position: [...props.cameraPosition!] as [number, number, number],
                    fov: props.cameraFov ?? 45,
                    ...(props.cameraNear !== undefined ? { near: props.cameraNear } : {}),
                    ...(props.cameraFar !== undefined ? { far: props.cameraFar } : {}),
                  }
                : undefined
            }
            gl={props.gl ?? { antialias: true }}
            onPointerDown={(event) => props.onPointerDown?.(event.nativeEvent)}
            onPointerMove={(event) => props.onPointerMove?.(event.nativeEvent)}
            onPointerUp={(event) => props.onPointerUp?.(event.nativeEvent)}
            onPointerLeave={(event) => props.onPointerLeave?.(event.nativeEvent)}
            onPointerCancel={(event) => props.onPointerCancel?.(event.nativeEvent)}
            onWheel={(event) => props.onWheel?.(event.nativeEvent)}
            onContextMenu={props.onContextMenu}
            onDoubleClick={(event) => props.onDoubleClick?.(event.nativeEvent)}
            onLostPointerCapture={(event) => props.onLostPointerCapture?.(event.nativeEvent)}
            onPointerMissed={props.onPointerMissed}
            onCreated={({ camera, gl: renderer }) => {
              props.onCanvasReady?.({ camera, domElement: renderer.domElement });
            }}
          >
            {frameloop === "demand" ? <DemandFrameloopKick /> : null}
            {props.background ? <color attach="background" args={[props.background]} /> : null}
            <WorldLayerStack>{props.children}</WorldLayerStack>
          </Canvas>
        ) : null}
      </div>""", """      <div ref={setCanvasEventSource} style={{ width: "100%", height: "100%" }}>
        {canvas}
      </div>"""),
]


def main() -> int:
    dry = "--dry-run" in sys.argv
    text = open(R3F, encoding="utf-8").read()
    changed = text
    failures = 0
    for index, (old, new) in enumerate(HUNKS):
        if changed.count(old) == 0 and changed.count(new) >= 1:
            print(f"skip   #{index} (applied)")
            continue
        if changed.count(old) != 1:
            print(f"FAIL   #{index}: anchor found {changed.count(old)}x")
            failures += 1
            continue
        changed = changed.replace(old, new)
        print(f"apply  #{index}")
    if failures:
        print(f"{failures} hunk(s) failed — nothing written")
        return 1
    if not dry and changed != text:
        open(R3F, "w", encoding="utf-8").write(changed)
    print("dry run clean" if dry else "written")
    return 0


if __name__ == "__main__":
    sys.exit(main())
