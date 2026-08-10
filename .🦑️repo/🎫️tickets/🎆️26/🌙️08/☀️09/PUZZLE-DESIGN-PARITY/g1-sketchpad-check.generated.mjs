function sketchpadConnectionTransformParamsFromDto(connection) {
  const out = {};
  for (const key of ["gap", "shift", "rise", "rotation", "turn", "tilt"]) {
    const value = connection[key];
    if (typeof value === "number" && Number.isFinite(value)) {
      out[key] = value;
    }
  }
  const x = connection["x"] ?? connection["u"];
  const y = connection["y"] ?? connection["v"];
  if (typeof x === "number" && Number.isFinite(x)) out.x = x;
  if (typeof y === "number" && Number.isFinite(y)) out.y = y;
  return out;
}

function sketchpadPiecePuzzleAnchor(piece) {
  const raw = piece["connectionKind"] ?? piece["connection_kind"];
  if (typeof raw === "string") {
    const kind = raw.trim().toLowerCase();
    if (kind === "fixed") return "fixed";
    if (kind === "connected" || kind === "derived") return "derived";
  }
  const authored = piece["position"] ;
  const hasAuthoredPose = Boolean(authored?.center || authored?.plane);
  return hasAuthoredPose ? "fixed" : "derived";
}

function sketchpadPieceAuthoredPose(piece) {
  const position = piece["position"] ;
  if (position?.center || position?.plane) return position;
  return undefined;
}

export function runChecks(src) {
  const failures = [];
  const assert = (cond, msg) => { if (!cond) failures.push(msg); };
  const conn = sketchpadConnectionTransformParamsFromDto({ u: 0.5, v: 0.25, gap: 1, shift: 2, rise: 3, rotation: 4, turn: 5, tilt: 6 });
  assert(conn.x === 0.5, "u maps to x");
  assert(conn.y === 0.25, "v maps to y");
  assert(conn.gap === 1 && conn.shift === 2 && conn.rise === 3, "keeps gap/shift/rise");
  assert(conn.rotation === 4 && conn.turn === 5 && conn.tilt === 6, "keeps rotation/turn/tilt");
  assert(conn.u === undefined && conn.v === undefined, "does not emit u/v");
  const alreadyXy = sketchpadConnectionTransformParamsFromDto({ x: 9, y: 8, u: 1, v: 2 });
  assert(alreadyXy.x === 9 && alreadyXy.y === 8, "prefers explicit x/y over u/v");
  assert(sketchpadPiecePuzzleAnchor({ connectionKind: "FIXED" }) === "fixed", "FIXED -> fixed");
  assert(sketchpadPiecePuzzleAnchor({ connectionKind: "CONNECTED" }) === "derived", "CONNECTED -> derived");
  assert(sketchpadPiecePuzzleAnchor({ connectionKind: "Connected" }) === "derived", "Connected -> derived");
  assert(sketchpadPiecePuzzleAnchor({ position: { center: { u: 1, v: 2 } } }) === "fixed", "authored pose -> fixed");
  assert(sketchpadPiecePuzzleAnchor({ flatPosition: { center: { u: 1, v: 2 } } }) === "derived", "flatPosition alone -> derived");
  assert(sketchpadPiecePuzzleAnchor({}) === "derived", "empty -> derived");
  const authored = sketchpadPieceAuthoredPose({ position: { center: { u: 1, v: 2 } }, flatPosition: { center: { u: 99, v: 99 } } });
  assert(authored?.center?.u === 1, "authored pose ignores flatPosition");
  assert(sketchpadPieceAuthoredPose({ flatPosition: { center: { u: 99, v: 99 } } }) === undefined, "flatPosition is not authored");
  const fixture2d = src.slice(src.indexOf("export function sketchpadDesignPuzzle2dFixtureFromDesign"), src.indexOf("export function sketchpadDesignVolumeFixtureFromDesign"));
  const fixture3d = src.slice(src.indexOf("export function sketchpadDesignVolumeFixtureFromDesign"), src.indexOf("function sketchpadSceneCameraFromDesign"));
  assert(fixture2d.includes("anchor"), "2d fixture emits anchor");
  assert(fixture2d.includes('anchor === "fixed"'), "2d seeds Fixed only");
  assert(fixture2d.includes("sketchpadPieceAuthoredPose"), "2d uses authored pose helper");
  assert(fixture2d.includes("sketchpadConnectionTransformParamsFromDto"), "2d spreads connection params");
  assert(!/flatPosition/.test(fixture2d), "2d fixture does not read flatPosition");
  assert(fixture3d.includes("anchor"), "3d fixture emits anchor");
  assert(fixture3d.includes('anchor === "fixed"'), "3d seeds Fixed only");
  assert(fixture3d.includes("sketchpadPieceAuthoredPose"), "3d uses authored pose helper");
  assert(fixture3d.includes("sketchpadPlaneAxesToQuaternion(storedPlane)"), "3d orientation from authored plane");
  assert(!/flatPosition/.test(fixture3d), "3d fixture does not read flatPosition");
  assert(/\n\s*connectionKind\n/.test(src), "GraphQL selection includes connectionKind");
  return failures;
}
