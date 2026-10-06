"""🎥️ F7 landing wave (S5-PUZZLE): a board camera move leaves on the view lane, never inside `applyBoardEvents`.

Live fault (React, probe batch J step 19): a middle-button pan added one Commands row "Apply Board Events". The pan's
`camera` rows rode in the board batch, and `applyBoardEvents` is a mutation verb, so its dispatch is a command row; a
camera move is view state (design §20.13 / L4) and must dispatch the view verb `setCamera`.

Schema first: the shared coalescing corpus states the latest camera beside the board rows (`expect.camera`) and forbids
a `camera` row among them; then React's `coalesceBoard2dEvents` and its dispatch, their tests, and the wgpu corpus law.
The wgpu host's production code is NOT changed here: it still prepends the camera row to its board batch (and its
retained `FinishPan` plan publishes a camera row as `applyBoardEvents`), which the adapted wgpu law states exactly.

All or nothing: every anchor must resolve the stated number of times (or its replacement must already be present) before
any file is written. `--check` writes nothing and prints the pending hunks. Run: `python3 <this file> [--check]`.
"""

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
ENGINE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
HOST = f"{ENGINE}/🧱️elements/🖥️Board2dHost"
SCHEMA = f"{HOST}/🧬️schema/🔣️board-event-coalescing/🔣️.json"
CORPUS = f"{HOST}/🧫️fixtures/🧫️board-event-coalescing/🔣️.json"
REACT = f"{HOST}/🟦️.tsx"
CORPUS_LAW = f"{HOST}/🧪️tests/🧪️board-event-coalescing/🟦️.ts"
CONTRACT = f"{ENGINE}/🧪️tests/🔬️engine-contract/🟦️.ts"
WGPU_LAW = f"{ENGINE}/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs"

HUNKS = []


def hunk(path, old, new, count=1):
    HUNKS.append((path, old, new, count))


# 🧬️ The corpus schema: the camera is its own answer, and no board row is a camera row.
hunk(
    SCHEMA,
    "one drained batch of engine rows in, the dispatched batch and the flush-now verdict out. The React",
    "one drained batch of engine rows in; the board rows for `applyBoardEvents`, the latest camera for the view lane (`setCamera`) and the flush-now verdict out. A camera row never rides in the board batch: a camera move is view state, never a command row. The React",
)
hunk(
    SCHEMA,
    """          "required": [
            "flushNow",
            "events"
          ],
          "properties": {
            "flushNow": {
              "type": "boolean"
            },
            "events": {
              "type": "array",
              "items": {
                "$ref": "#/definitions/Row"
              }
            }
          }
""",
    """          "required": [
            "flushNow",
            "camera",
            "events"
          ],
          "properties": {
            "flushNow": {
              "type": "boolean"
            },
            "camera": {
              "oneOf": [
                {
                  "type": "null"
                },
                {
                  "$ref": "#/definitions/Camera"
                }
              ]
            },
            "events": {
              "type": "array",
              "items": {
                "$ref": "#/definitions/BoardRow"
              }
            }
          }
""",
)
hunk(
    SCHEMA,
    """    "Row": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "name",
        "payload"
      ],
""",
    """    "Camera": {
      "description": "🎥️ The board camera a host publishes on the view lane: the pan in board units and the zoom factor.",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "x",
        "y",
        "zoom"
      ],
      "properties": {
        "x": {
          "type": "number"
        },
        "y": {
          "type": "number"
        },
        "zoom": {
          "type": "number",
          "exclusiveMinimum": 0
        }
      }
    },
    "BoardRow": {
      "description": "📬️ One row a host dispatches inside `applyBoardEvents`: any engine row but `camera`.",
      "allOf": [
        {
          "$ref": "#/definitions/Row"
        },
        {
          "not": {
            "type": "object",
            "required": [
              "name"
            ],
            "properties": {
              "name": {
                "const": "camera"
              }
            }
          }
        }
      ]
    },
    "Row": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "name",
        "payload"
      ],
""",
)

# 🧫️ The corpus: every case states its camera; the two camera cases move it out of the board rows.
hunk(
    CORPUS,
    """      "name": "only the latest camera leads the batch and a camera alone waits",
      "rows": [
        { "name": "camera", "payload": { "x": 1, "y": 1, "zoom": 1 } },
        { "name": "camera", "payload": { "x": 2, "y": 3, "zoom": 1.5 } }
      ],
      "expect": { "flushNow": false, "events": [{ "name": "camera", "payload": { "x": 2, "y": 3, "zoom": 1.5 } }] }
""",
    """      "name": "a pan is camera rows only: its latest camera leaves on the view lane, no board row, and it waits",
      "rows": [
        { "name": "camera", "payload": { "x": 1, "y": 1, "zoom": 1 } },
        { "name": "camera", "payload": { "x": 2, "y": 3, "zoom": 1.5 } }
      ],
      "expect": { "flushNow": false, "camera": { "x": 2, "y": 3, "zoom": 1.5 }, "events": [] }
""",
)
hunk(
    CORPUS,
    """      "name": "an untagged select flushes at once behind the latest camera",
      "rows": [
        { "name": "camera", "payload": { "x": 1, "y": 1, "zoom": 1 } },
        { "name": "select", "payload": { "ids": ["node-a"], "exitHighlightIds": [] } },
        { "name": "camera", "payload": { "x": 4, "y": 4, "zoom": 2 } }
      ],
      "expect": {
        "flushNow": true,
        "events": [
          { "name": "camera", "payload": { "x": 4, "y": 4, "zoom": 2 } },
          { "name": "select", "payload": { "ids": ["node-a"], "exitHighlightIds": [] } }
        ]
      }
""",
    """      "name": "an untagged select flushes at once and the latest camera leaves beside it",
      "rows": [
        { "name": "camera", "payload": { "x": 1, "y": 1, "zoom": 1 } },
        { "name": "select", "payload": { "ids": ["node-a"], "exitHighlightIds": [] } },
        { "name": "camera", "payload": { "x": 4, "y": 4, "zoom": 2 } }
      ],
      "expect": {
        "flushNow": true,
        "camera": { "x": 4, "y": 4, "zoom": 2 },
        "events": [{ "name": "select", "payload": { "ids": ["node-a"], "exitHighlightIds": [] } }]
      }
""",
)
hunk(CORPUS, '"expect": { "flushNow": false, "events": ', '"expect": { "flushNow": false, "camera": null, "events": ', 4)
hunk(CORPUS, '"expect": { "flushNow": true, "events": ', '"expect": { "flushNow": true, "camera": null, "events": ', 7)
hunk(
    CORPUS,
    """      "expect": {
        "flushNow": true,
        "events": [""",
    """      "expect": {
        "flushNow": true,
        "camera": null,
        "events": [""",
    5,
)

# ⚛️ React: the coalescer answers the camera beside the rows, and the flush dispatches it as the view verb.
hunk(
    REACT,
    """/**
 * 📬️ Drops transient rows, keeps only the latest `camera` (first), keeps every other row in order, and flags
 * whether the buffer should flush now: any terminal row does, except a `select` tagged with a `gestureId`
 * whose `gesture` record the batch does not carry — an open gesture's selection leaves with its record, never
 * mid-gesture. The wgpu twin `coalesce_owned_board_events` is pinned to this one by the shared corpus
 * `🧫️fixtures/🧫️board-event-coalescing/🔣️.json`.
 */
export function coalesceBoard2dEvents(rows: readonly BoardEventRow[]): { readonly flushNow: boolean; readonly eventsJson: string } {
  const recorded = new Set(rows.filter((row) => row.name === "gesture").map(board2dGestureId));
  let flushNow = false;
  let lastCamera: BoardEventRow | null = null;
  const rest: BoardEventRow[] = [];
  for (const row of rows) {
    if (PUZZLE2D_TRANSIENT_EVENT_NAMES.has(row.name)) continue;
    if (row.name === "camera") {
      lastCamera = row;
      continue;
    }
    const gestureId = row.name === "select" ? board2dGestureId(row) : undefined;
    if (PUZZLE2D_FLUSH_NOW_EVENT_NAMES.has(row.name) && (gestureId === undefined || recorded.has(gestureId))) flushNow = true;
    rest.push(row);
  }
  return { flushNow, eventsJson: JSON.stringify(lastCamera ? [lastCamera, ...rest] : rest) };
}
""",
    """/** 🎥️ The camera a `camera` row carries; `null` for a row that carries none. */
function board2dRowCamera(row: BoardEventRow): BoardCamera | null {
  const payload = row.payload as Partial<BoardCamera> | undefined;
  return typeof payload?.x === "number" && typeof payload.y === "number" && typeof payload.zoom === "number" ? { x: payload.x, y: payload.y, zoom: payload.zoom } : null;
}

/**
 * 📬️ Drops transient rows, takes the latest `camera` out of the batch, keeps every other row in order, and flags
 * whether the buffer should flush now: any terminal row does, except a `select` tagged with a `gestureId`
 * whose `gesture` record the batch does not carry — an open gesture's selection leaves with its record, never
 * mid-gesture. The camera is view state: it leaves on the view lane (`setCamera`) and never inside
 * `applyBoardEvents`, so a pan is never a command row. The shared corpus
 * `🧫️fixtures/🧫️board-event-coalescing/🔣️.json` pins this and the wgpu twin `coalesce_owned_board_events`.
 */
export function coalesceBoard2dEvents(rows: readonly BoardEventRow[]): { readonly flushNow: boolean; readonly eventsJson: string; readonly camera: BoardCamera | null } {
  const recorded = new Set(rows.filter((row) => row.name === "gesture").map(board2dGestureId));
  let flushNow = false;
  let camera: BoardCamera | null = null;
  const rest: BoardEventRow[] = [];
  for (const row of rows) {
    if (PUZZLE2D_TRANSIENT_EVENT_NAMES.has(row.name)) continue;
    if (row.name === "camera") {
      camera = board2dRowCamera(row) ?? camera;
      continue;
    }
    const gestureId = row.name === "select" ? board2dGestureId(row) : undefined;
    if (PUZZLE2D_FLUSH_NOW_EVENT_NAMES.has(row.name) && (gestureId === undefined || recorded.has(gestureId))) flushNow = true;
    rest.push(row);
  }
  return { flushNow, eventsJson: JSON.stringify(rest), camera };
}
""",
)
hunk(
    REACT,
    """    const { eventsJson } = coalesceBoard2dEvents(pendingEventRowsRef.current);
    pendingEventRowsRef.current = [];
    boardStatusRef.current.pendingEvents = 0;
    publishBoardVitals();
    if (eventsJson && eventsJson !== "[]") dispatch("applyBoardEvents", { eventsJson });
""",
    """    const { eventsJson, camera } = coalesceBoard2dEvents(pendingEventRowsRef.current);
    pendingEventRowsRef.current = [];
    boardStatusRef.current.pendingEvents = 0;
    publishBoardVitals();
    if (camera) dispatch("setCamera", { camera });
    if (eventsJson !== "[]") dispatch("applyBoardEvents", { eventsJson });
""",
)

# 🧪️ The React laws.
hunk(
    CORPUS_LAW,
    """      const { flushNow, eventsJson } = coalesceBoard2dEvents(entry.rows);
      expect(JSON.parse(eventsJson)).toEqual(entry.expect.events);
      expect(flushNow).toBe(entry.expect.flushNow);
""",
    """      const { flushNow, eventsJson, camera } = coalesceBoard2dEvents(entry.rows);
      expect(JSON.parse(eventsJson)).toEqual(entry.expect.events);
      expect(camera).toEqual(entry.expect.camera);
      expect(flushNow).toBe(entry.expect.flushNow);
""",
)
hunk(
    CORPUS_LAW,
    """    delete record?.payload.dx;
    expect(validate(broken)).toBe(false);
""",
    """    delete record?.payload.dx;
    expect(validate(broken)).toBe(false);
    const panned = structuredClone(corpus) as { cases: { expect: { events: unknown[] } }[] };
    panned.cases[0]?.expect.events.push({ name: "camera", payload: { x: 0, y: 0, zoom: 1 } });
    expect(validate(panned)).toBe(false);
""",
)
hunk(
    CORPUS_LAW,
    "  it(\"is valid against its schema of record, and the schema refuses a drag record without its offset\", () => {\n",
    "  it(\"is valid against its schema of record, and the schema refuses a drag record without its offset and a camera row among the board rows\", () => {\n",
)
hunk(
    CONTRACT,
    """  it("coalesces puzzle 2d board events: drops transients and live nodeMove frames, keeps the latest camera", () => {
""",
    """  it("coalesces puzzle 2d board events: drops transients and live nodeMove frames, takes the latest camera out for the view lane", () => {
""",
)
hunk(
    CONTRACT,
    """    const { flushNow, eventsJson } = coalesceBoard2dEvents(rows);
    expect(flushNow).toBe(false);
    expect(JSON.parse(eventsJson)).toEqual([{ name: "camera", payload: { x: 2, y: 2, zoom: 1.5 } }]);
""",
    """    const { flushNow, eventsJson, camera } = coalesceBoard2dEvents(rows);
    expect(flushNow).toBe(false);
    expect(JSON.parse(eventsJson)).toEqual([]);
    expect(camera).toEqual({ x: 2, y: 2, zoom: 1.5 });
""",
)

# 🧊️ The wgpu corpus law: the host still leads its board batch with the camera row, stated exactly.
hunk(
    WGPU_LAW,
    """/// ⚖️ Law: every corpus case coalesces here exactly as React coalesces it — same dispatched rows in the same
/// order, same flush verdict — so one drag reaches the guest as ONE batch on both hosts.
""",
    """/// ⚖️ Law: every corpus case coalesces here exactly as React coalesces it — same board rows in the same order,
/// same flush verdict — so one drag reaches the guest as ONE batch on both hosts. The corpus states the latest camera
/// beside the rows (the view lane, `setCamera`); this host still leads its board batch with that camera row, so the
/// law requires exactly the corpus rows behind exactly the corpus camera.
""",
)
hunk(
    WGPU_LAW,
    """        assert_eq!(serde_json::from_str::<Value>(&coalesced.events_json).expect("dispatched rows parse"), case["expect"]["events"], "{name}");
""",
    """        let camera = &case["expect"]["camera"];
        let mut expected: Vec<Value> = if camera.is_null() { Vec::new() } else { vec![serde_json::json!({ "name": "camera", "payload": camera })] };
        expected.extend(case["expect"]["events"].as_array().expect("events").iter().cloned());
        assert_eq!(serde_json::from_str::<Value>(&coalesced.events_json).expect("dispatched rows parse"), Value::Array(expected), "{name}");
""",
)


def main():
    check = "--check" in sys.argv[1:]
    contents, pending = {}, 0
    for path, old, new, expected in HUNKS:
        text = contents.get(path)
        if text is None:
            text = (ROOT / path).read_text(encoding="utf-8")
        count = text.count(old)
        if count == 0 and new in text:
            contents[path] = text
            continue
        if count != expected:
            sys.exit(f"{path}: anchor resolves {count} times, expected {expected}:\n{old[:200]}")
        contents[path] = text.replace(old, new)
        pending += 1
        if check:
            print(f"pending: {path}: {old.splitlines()[0][:100]}")
    if check:
        print(f"{pending} pending of {len(HUNKS)} hunks in {len(contents)} files")
        return
    for path, text in contents.items():
        if (ROOT / path).read_text(encoding="utf-8") != text:
            (ROOT / path).write_text(text, encoding="utf-8")
    print(f"applied {pending} of {len(HUNKS)} hunks in {len(contents)} files")


if __name__ == "__main__":
    main()
