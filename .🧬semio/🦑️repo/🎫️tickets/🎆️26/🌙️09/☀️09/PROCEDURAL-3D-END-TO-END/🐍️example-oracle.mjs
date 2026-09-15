/** 🧿 THE committed oracle for the 8 bundled generation3d examples, read straight off the same
 * fixtures the native `example-geometry` test reads — so a runtime probe and `cargo test` can never
 * disagree about what an example IS.
 *
 * Source of truth: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/<dir>/🧫️fixtures/🧩️example/🔣️.json`
 * (schema `s.procedural.generation3d.example-geometry/v1`). The native binding is
 * `📦️packages/🦀️rust/Cargo.toml` `[[test]] name = "example-geometry"` →
 * `📚️examples/🧪️tests/🧩️geometry/🦀️.rs`; `assert_example` (`:610`) checks the tessellated geometry and
 * `assert_delivery` (`:904`) the published preview payload.
 *
 * 📐️ The three numbers a runtime probe can hold the DOM to, and the native line each mirrors:
 *   - `meshes` — `delivery.meshes`, the ONLY exact count in the fixture (`🦀️.rs:854`
 *     `assert_eq!(run.payload_meshes, delivery.meshes)`), and equally the instance count (`:855`).
 *   - `minTriangles` / `minEdgeSegments` — `delivery.*` floors, asserted at `🦀️.rs:849-850` against
 *     the PREVIEW node's own delivered mesh (`run.triangles`), not the payload sum.
 *   - `boundingBoxMin/Max` ± `boundingBoxTolerance` — `🦀️.rs:568-569`, computed by `bounds()`
 *     (`🦀️.rs:450`) over the preview mesh's `positions`, or over its `edgePositions` when the mesh
 *     carries no triangles (the wire example).
 *
 * 🆔️ The preview mesh is found by id: the editor publishes `eval-{node}@{channel}#{index}`
 * (`✏️editor/🦀️.rs:2852`, viewer `👁️viewer/…/👁️preview/🦀️.rs:243,303`), and `preview.node`/
 * `preview.channel` in the fixture name exactly the node the native test tessellates.
 *
 * 📡️ DOM lanes these oracles are measured against (`World3dHost/🟦️.tsx:6494-6502`):
 *   - `data-meshes-json` — `[{ id, data?: { positions, normals, colors?, uvs?, indices, faceIds?,
 *     vertexIds?, edgePositions?, edgeIds? }, url?, kind? }]`. Flat `[x,y,z,…]` float buffers;
 *     `indices.length / 3` is triangles and `edgePositions.length / 6` is edge segments (each segment
 *     is a start and an end point). A built-in `kind` carries no `data` — generation3d never uses one.
 *   - `data-instances-json` — `[{ id, meshId, position, rotation, scale, label, interactionId,
 *     selected, hovered }]`; `interactionId` is the id a pick reports.
 *   - `data-selection-json` — `worldSurfaceSelectionDomV1` (`World3dHost/🟦️.tsx:1438`):
 *     `{ selectedIds, activeObjectId, targetVolumeIds, referenceSelectedId, hoverTarget: { domain,
 *     id } | null, hoveredVortexFullId, hoveredKindId, gumballActive, gumballTarget, transformMode,
 *     activeUtility }`. 🪪️ This — NOT `data-interaction-json` — is where a hover/selection VALUE
 *     lives. `data-interaction-json` is the per-window utility/brush record and publishes `{}` on
 *     every generation3d run measured in this ticket, so a verdict reading it can only ever be
 *     vacuous.
 *   - `data-camera-json` / `data-viewport-camera-json` — `world3dCameraDomJson`
 *     (`World3dHost/🟦️.tsx:783`): `{ position: [x,y,z], target: [x,y,z], up, zoom, fov, projection }`.
 */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

export const EXAMPLES_DIR = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples";

/** 🏷️ The picker label a slug is painted as — `setActiveExample`'s rows are the slug in Title Case. */
export const labelOfSlug = (slug) => slug.split("-").map((word) => word[0].toUpperCase() + word.slice(1)).join(" ");
const slugOfLabel = (label) => label.trim().toLowerCase().replace(/\s+/g, "-");

/** 📚️ Every committed fixture, keyed by slug, with the delivery oracle flattened for a probe. */
export const loadExampleOracles = (dir = EXAMPLES_DIR) => {
  const oracles = {};
  for (const entry of readdirSync(dir)) {
    let fixture;
    try {
      fixture = JSON.parse(readFileSync(join(dir, entry, "🧫️fixtures", "🧩️example", "🔣️.json"), "utf8"));
    } catch {
      continue;
    }
    if (fixture?.schema !== "s.procedural.generation3d.example-geometry/v1") continue;
    oracles[fixture.example] = {
      slug: fixture.example,
      label: labelOfSlug(fixture.example),
      dir: entry,
      previewMeshId: `eval-${fixture.preview.node}@${fixture.preview.channel}#0`,
      previewKind: fixture.preview.kind,
      meshes: fixture.delivery.meshes,
      minTriangles: fixture.delivery.minTriangles,
      minEdgeSegments: fixture.delivery.minEdgeSegments,
      boundingBoxMin: fixture.expect.boundingBoxMin,
      boundingBoxMax: fixture.expect.boundingBoxMax,
      /** 📏️ The extent budget is ASYMMETRIC, because the two directions mean different things.
       *
       * OVERSHOOT is held to the fixture's own `boundingBoxTolerance`: every bundled example is a
       * solid or a wire whose facets are chords of the true surface, so a tessellation can only
       * INSCRIBE it. A delivered mesh reaching past `expect.boundingBoxMin/Max` is a wrong scale, a
       * wrong transform or a wrong example — never a level of detail. Measured across all 8 examples
       * in both roles, no runtime extent ever exceeded its fixture's (`🗑️generated/react-oracle/journey-1`).
       *
       * SHORTFALL gets a LOD envelope, because `delivery` commits NO extent of its own and the
       * delivered mesh is demonstrably coarser than the `expect` tessellation: `sphere-box-fuse`
       * delivers its sphere pole at exactly -1.2 but its equator at -1.1863 (0.0137 short of an
       * example tessellated at 0.0025), and `sphere-cut-with-torus` falls 0.047 short on one axis
       * while hitting ±2.2 and +2.1475 exactly. 2% of the box's own largest extent covers that and
       * still fails a missing feature or a halved dimension.
       *
       * 🪪️ The clean end of this is a committed `delivery.boundingBoxMin/Max/Tolerance` authored from
       * `run_delivery`'s own payload in `📚️examples/🧪️tests/🧩️geometry/🦀️.rs` — then both halves are
       * exact and this envelope goes away. That needs a native run this lane did not make. */
      boundingBoxTolerance: fixture.expect.boundingBoxTolerance,
      /** 📦️ The extent the DELIVERY commits, as of 2026-09-15 — hand-written into every fixture from
       * `run_delivery`'s own `preview_payload` and asserted natively by `assert_delivery_bounds`
       * (`📚️examples/🧪️tests/🧩️geometry/🦀️.rs`). This is what the runtime payload is graded against and
       * what the render hosts FRAME, so the shortfall envelope the note above described is gone: both
       * halves are now exact numbers (ticket 26/09/09, `📓️boot-camera-framing-2026-09-15.md`). */
      deliveryBoundingBoxMin: fixture.delivery.boundingBoxMin,
      deliveryBoundingBoxMax: fixture.delivery.boundingBoxMax,
      deliveryBoundingBoxTolerance: fixture.delivery.boundingBoxTolerance,
      boundingBoxShortfall: Math.max(
        fixture.expect.boundingBoxTolerance + fixture.tessellationTolerance,
        0.02 * Math.max(...fixture.expect.boundingBoxMax.map((max, axis) => max - fixture.expect.boundingBoxMin[axis])),
      ),
      tessellationTolerance: fixture.tessellationTolerance,
      closed: fixture.expect.closed,
      volume: fixture.expect.volume,
    };
  }
  return oracles;
};

export const ORACLES = loadExampleOracles();
/** 🔤️ The same oracles under the label the example picker paints. */
export const ORACLES_BY_LABEL = Object.fromEntries(Object.values(ORACLES).map((oracle) => [oracle.label, oracle]));
export const oracleForLabel = (label) => (label ? (ORACLES_BY_LABEL[label] ?? ORACLES[slugOfLabel(label)] ?? null) : null);

/** 🧮 The page-side reader: per preview host, the delivered mesh numbers this oracle is stated in.
 * Runs inside `page.evaluate` — it must stay self-contained and must never ship the position buffers
 * themselves across the bridge (`sphere-cut-with-torus` alone delivers > 1100 triangles). */
export const meshStatsScriptSource = `(previewMeshId) => {
  const bounds = (values) => {
    const min = [Infinity, Infinity, Infinity];
    const max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i + 2 < values.length; i += 3) for (let axis = 0; axis < 3; axis += 1) {
      const v = values[i + axis];
      if (v < min[axis]) min[axis] = v;
      if (v > max[axis]) max[axis] = v;
    }
    return Number.isFinite(min[0]) ? { min, max } : null;
  };
  const statsOf = (mesh) => {
    const data = mesh && mesh.data ? mesh.data : {};
    const positions = data.positions ?? [];
    const edgePositions = data.edgePositions ?? [];
    return {
      id: mesh?.id ?? null,
      triangles: (data.indices ?? []).length / 3,
      vertices: positions.length / 3,
      edgeSegments: edgePositions.length / 6,
      bounds: bounds(positions.length > 0 ? positions : edgePositions),
    };
  };
  return [...document.querySelectorAll("[data-meshes-json], [data-status-json]")].map((el) => {
    let meshes = [];
    try { const parsed = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(parsed) ? parsed : []; } catch {}
    let instances = [];
    try { const parsed = JSON.parse(el.getAttribute("data-instances-json") ?? "[]"); instances = Array.isArray(parsed) ? parsed : []; } catch {}
    const per = meshes.map(statsOf);
    const preview = per.find((entry) => entry.id === previewMeshId) ?? null;
    const framed = per.filter((entry) => entry.bounds);
    const all = framed.length === 0 ? null : {
      min: [0, 1, 2].map((axis) => Math.min(...framed.map((entry) => entry.bounds.min[axis]))),
      max: [0, 1, 2].map((axis) => Math.max(...framed.map((entry) => entry.bounds.max[axis]))),
    };
    return {
      allBounds: all,
      surfaceId: el.getAttribute("data-surface-id"),
      meshCount: meshes.length,
      instanceCount: instances.length,
      meshIds: per.map((entry) => entry.id),
      instanceIds: instances.map((instance) => instance.id),
      interactionIds: instances.map((instance) => instance.interactionId ?? null).filter(Boolean),
      totalTriangles: per.reduce((n, entry) => n + entry.triangles, 0),
      preview,
    };
  });
}`;

/** 🛰️ The same reader as a real function, which is what `page.evaluate(fn, arg)` wants: handed the
 * bare source string, Playwright evaluates it as an EXPRESSION and hands back nothing. */
export const meshStatsScript = new Function(`return ${meshStatsScriptSource}`)();

/** ⚖️ The verdict a runtime mesh payload gets against its committed fixture — the reasons are the
 * report's own wording, so a red row never has to be re-derived by hand. */
export const gradeMeshes = (oracle, host) => {
  const reasons = [];
  if (!oracle) return { ok: false, reasons: ["no committed fixture for this example"] };
  if (!host) return { ok: false, reasons: ["no preview host published"] };
  if (host.meshCount !== oracle.meshes) reasons.push(`meshes ${host.meshCount} vs oracle ${oracle.meshes}`);
  if (host.instanceCount !== oracle.meshes) reasons.push(`instances ${host.instanceCount} vs oracle ${oracle.meshes}`);
  const preview = host.preview;
  if (!preview) reasons.push(`preview mesh ${oracle.previewMeshId} absent from ${JSON.stringify(host.meshIds)}`);
  else {
    if (preview.triangles < oracle.minTriangles) reasons.push(`triangles ${preview.triangles} below floor ${oracle.minTriangles}`);
    if (preview.edgeSegments < oracle.minEdgeSegments) reasons.push(`edgeSegments ${preview.edgeSegments} below floor ${oracle.minEdgeSegments}`);
    if (!preview.bounds) reasons.push("preview mesh carries no positions and no edge positions");
    else
      for (let axis = 0; axis < 3; axis += 1) {
        const overMax = preview.bounds.max[axis] - oracle.boundingBoxMax[axis];
        const overMin = oracle.boundingBoxMin[axis] - preview.bounds.min[axis];
        if (overMax > oracle.boundingBoxTolerance) reasons.push(`bbox max axis ${axis} ${preview.bounds.max[axis]} reaches ${overMax} past the fixture's ${oracle.boundingBoxMax[axis]}`);
        if (overMin > oracle.boundingBoxTolerance) reasons.push(`bbox min axis ${axis} ${preview.bounds.min[axis]} reaches ${overMin} past the fixture's ${oracle.boundingBoxMin[axis]}`);
        if (-overMax > oracle.boundingBoxShortfall) reasons.push(`bbox max axis ${axis} ${preview.bounds.max[axis]} falls ${-overMax} short of ${oracle.boundingBoxMax[axis]}, beyond the ${oracle.boundingBoxShortfall} LOD envelope`);
        if (-overMin > oracle.boundingBoxShortfall) reasons.push(`bbox min axis ${axis} ${preview.bounds.min[axis]} falls ${-overMin} short of ${oracle.boundingBoxMin[axis]}, beyond the ${oracle.boundingBoxShortfall} LOD envelope`);
      }
  }
  return { ok: reasons.length === 0, reasons };
};

/** 🎥️ Does the camera actually FRAME this example's bounding box?
 *
 * ⚖️ Graded by PROJECTION, not by reproducing an implementation constant: every corner of the box
 * must land inside the viewport's normalized device square, in front of the eye, with margin. That is
 * the user-facing statement ("the whole example is on screen"), and it stays true across any change to
 * the host's fit arithmetic — where the previous grader, which re-derived `max(radius * 1.8, 2)` from
 * `fitCameraFromBounds`, could only ever agree with whatever the host did, including standing so close
 * that the box's own corners fell outside the frustum
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
 *
 * `bounds` is the box to contain — the pane's whole published payload (`stats.allBounds`) for a fit
 * over what is on screen, or the fixture's committed `delivery.boundingBox*` for a boot framing.
 * `aspect` is the preview canvas's own width/height. */
export const CAMERA_FRAME_NDC_LIMIT = 1.0;
export const gradeCameraFrames = (camera, bounds, aspect = 1) => {
  const reasons = [];
  if (!camera || !Array.isArray(camera.position) || !Array.isArray(camera.target)) return { ok: false, reasons: [`no camera published: ${JSON.stringify(camera)}`] };
  if (!bounds || !Array.isArray(bounds.min) || !Array.isArray(bounds.max)) return { ok: false, reasons: ["no geometry to frame"] };
  const forward = camera.target.map((value, axis) => value - camera.position[axis]);
  const forwardLength = Math.hypot(...forward);
  if (!(forwardLength > 1e-6)) return { ok: false, reasons: ["the camera sits on its own target"] };
  const unitForward = forward.map((value) => value / forwardLength);
  const up = Array.isArray(camera.up) && Math.hypot(...camera.up) > 1e-6 ? camera.up : [0, 0, 1];
  const right = [unitForward[1] * up[2] - unitForward[2] * up[1], unitForward[2] * up[0] - unitForward[0] * up[2], unitForward[0] * up[1] - unitForward[1] * up[0]];
  const rightLength = Math.hypot(...right);
  if (!(rightLength > 1e-6)) return { ok: false, reasons: ["the camera looks straight along its own up axis"] };
  const unitRight = right.map((value) => value / rightLength);
  const unitUp = [
    unitRight[1] * unitForward[2] - unitRight[2] * unitForward[1],
    unitRight[2] * unitForward[0] - unitRight[0] * unitForward[2],
    unitRight[0] * unitForward[1] - unitRight[1] * unitForward[0],
  ];
  const halfVertical = Math.tan((((typeof camera.fov === "number" && camera.fov > 0 ? camera.fov : 45) * Math.PI) / 180) * 0.5);
  let worstX = 0;
  let worstY = 0;
  let behind = 0;
  for (let corner = 0; corner < 8; corner += 1) {
    const point = [corner & 1 ? bounds.max[0] : bounds.min[0], corner & 2 ? bounds.max[1] : bounds.min[1], corner & 4 ? bounds.max[2] : bounds.min[2]];
    const relative = point.map((value, axis) => value - camera.position[axis]);
    const depth = relative.reduce((sum, value, axis) => sum + value * unitForward[axis], 0);
    if (!(depth > 0)) { behind += 1; continue; }
    const ndcY = relative.reduce((sum, value, axis) => sum + value * unitUp[axis], 0) / (depth * halfVertical);
    const ndcX = relative.reduce((sum, value, axis) => sum + value * unitRight[axis], 0) / (depth * halfVertical * Math.max(aspect, 0.05));
    worstX = Math.max(worstX, Math.abs(ndcX));
    worstY = Math.max(worstY, Math.abs(ndcY));
  }
  if (behind > 0) reasons.push(`${behind} of the box's 8 corners are behind the camera`);
  if (worstX > CAMERA_FRAME_NDC_LIMIT) reasons.push(`the box reaches ${worstX.toFixed(3)} of the half-width — clipped left/right`);
  if (worstY > CAMERA_FRAME_NDC_LIMIT) reasons.push(`the box reaches ${worstY.toFixed(3)} of the half-height — clipped top/bottom`);
  /** 🔭️ A camera parked in the next county contains every corner and shows the user a dot. */
  if (worstX < 0.12 && worstY < 0.12) reasons.push(`the box fills only ${Math.max(worstX, worstY).toFixed(3)} of the viewport — framed from too far away`);
  return { ok: reasons.length === 0, reasons, worstX, worstY, behind, aspect };
};
