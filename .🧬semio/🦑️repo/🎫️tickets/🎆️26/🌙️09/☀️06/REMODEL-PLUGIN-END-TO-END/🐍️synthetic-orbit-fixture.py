#!/usr/bin/env python3
"""🛰️ Independent (stdlib-only) reference implementation of the `synthetic-orbit` remodeling example
fixture generator.

This is the ORACLE and the BOOTSTRAP for
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🖼️assets/`.
The permanent generator is the Rust `#[test] #[ignore] regenerates_the_synthetic_orbit_example`
(`✏️editor/⚙️engine/🦀️.rs`, region 🔖️SyntheticOrbitFixture), reachable as
`bun ./📜️script.ts regenerate-example`. Both implement the SAME scene, camera, distortion and
rasterisation math in f64; the Rust example test asserts the committed PNG pixels equal the Rust
renderer's own output pixel-for-pixel, which is what makes this file a cross-language oracle rather
than a duplicate.

PNG container bytes are NOT expected to match between the two (different deflate encoders); the
DECODED RGBA rasters are.
"""

import json
import math
import os
import struct
import zlib

MASK64 = (1 << 64) - 1

# region 🔖️Rng


class SplitMix64:
    """🎲 splitmix64 — trivially reproducible in Rust with the same constants."""

    def __init__(self, seed):
        self.state = seed & MASK64

    def next_u64(self):
        self.state = (self.state + 0x9E3779B97F4A7C15) & MASK64
        z = self.state
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK64
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK64
        return z ^ (z >> 31)

    def next_f64(self):
        return (self.next_u64() >> 11) * (2.0**-53)

    def unit(self):
        return self.next_f64() * 2.0 - 1.0


# endregion 🔖️Rng

# region 🔖️Fixture constants

SEED = 0x5EED_0B17_5CE9_E000
FRAME_COUNT = 10
WIDTH = 320
HEIGHT = 240
FOCAL_RATIO = 0.85
FX = FOCAL_RATIO * WIDTH
FY = FX
CX = WIDTH / 2.0
CY = HEIGHT / 2.0
K1 = -0.02
K2 = 0.005
CUBE_HALF = 1.0
ORBIT_RADIUS = 2.6
ORBIT_ELEVATION = 2.2
MARKERS_PER_FACE = 22
FPS_HINT = 2.0
STREAM_ID = "synthetic-orbit"
CAMERA_ID = "synthetic-cam"
BACKGROUND = (35, 35, 40)
FACE_BASE_COLORS = [
    (150, 60, 60),
    (60, 60, 150),
    (60, 150, 60),
    (150, 150, 60),
    (150, 60, 150),
    (60, 150, 150),
]
MARKER_COLORS = [
    (250, 250, 250),
    (15, 15, 15),
    (245, 190, 40),
    (40, 200, 245),
    (245, 60, 130),
    (120, 245, 90),
]

# endregion 🔖️Fixture constants

# region 🔖️LinearAlgebra


def sub3(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def cross3(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def norm3(a):
    return (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]) ** 0.5


def normalize3(a):
    n = norm3(a)
    return a if n < 1e-300 else (a[0] / n, a[1] / n, a[2] / n)


def mat_act(m, v):
    return (
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    )


def transpose(m):
    return [[m[j][i] for j in range(3)] for i in range(3)]


def look_at_world_to_camera(eye, target, up):
    """🎥️ Mirrors `📸️sfm/🦀️.rs`'s private `look_at_pose`: columns (right, true_up, forward)."""
    forward = normalize3(sub3(target, eye))
    right = normalize3(cross3(up, forward))
    true_up = cross3(forward, right)
    r_cw = [[right[i], true_up[i], forward[i]] for i in range(3)]
    r_wc = transpose(r_cw)
    t = mat_act(r_wc, eye)
    return r_wc, (-t[0], -t[1], -t[2]), r_cw


def quat_from_mat(m):
    """🔁 `So3::to_quat` — mirrors the branchless trace form used by `🔷️lie-internals`."""
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0.0:
        s = (trace + 1.0) ** 0.5 * 2.0
        return (0.25 * s, (m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s)
    if m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = (1.0 + m[0][0] - m[1][1] - m[2][2]) ** 0.5 * 2.0
        return ((m[2][1] - m[1][2]) / s, 0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s)
    if m[1][1] > m[2][2]:
        s = (1.0 + m[1][1] - m[0][0] - m[2][2]) ** 0.5 * 2.0
        return ((m[0][2] - m[2][0]) / s, (m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s)
    s = (1.0 + m[2][2] - m[0][0] - m[1][1]) ** 0.5 * 2.0
    return ((m[1][0] - m[0][1]) / s, (m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s)


# endregion 🔖️LinearAlgebra

# region 🔖️Camera


def distort(p):
    x, y = p
    r2 = x * x + y * y
    radial = 1.0 + K1 * r2 + K2 * r2 * r2
    return (x * radial, y * radial)


def newton_undistort(distorted):
    """🔬️ Byte-for-byte mirror of `📷️camera/🦀️.rs`'s `Intrinsics::newton_undistort`."""
    p = distorted
    eps = 1e-6
    for _ in range(8):
        fp = distort(p)
        residual = (fp[0] - distorted[0], fp[1] - distorted[1])
        if abs(residual[0]) < 1e-14 and abs(residual[1]) < 1e-14:
            break
        fx0 = distort((p[0] + eps, p[1]))
        fy0 = distort((p[0], p[1] + eps))
        j = [
            [(fx0[0] - fp[0]) / eps, (fy0[0] - fp[0]) / eps],
            [(fx0[1] - fp[1]) / eps, (fy0[1] - fp[1]) / eps],
        ]
        det = j[0][0] * j[1][1] - j[0][1] * j[1][0]
        if abs(det) < 1e-300:
            break
        dx = (j[1][1] * residual[0] - j[0][1] * residual[1]) / det
        dy = (j[0][0] * residual[1] - j[1][0] * residual[0]) / det
        p = (p[0] - dx, p[1] - dy)
    return p


def unproject_ray(px, py):
    yd = (py - CY) / FY
    xd = (px - CX) / FX
    x, y = newton_undistort((xd, yd))
    return (x, y, 1.0)


def project(p_cam):
    if p_cam[2] <= 0.0:
        return None
    xd, yd = distort((p_cam[0] / p_cam[2], p_cam[1] / p_cam[2]))
    return (FX * xd + CX, FY * yd + CY)


# endregion 🔖️Camera

# region 🔖️Scene


def face_index(axis, positive):
    return axis * 2 + (0 if positive else 1)


def generate_markers(seed, half):
    rng = SplitMix64(seed)
    faces = []
    for _ in range(6):
        markers = []
        for _ in range(MARKERS_PER_FACE):
            u = rng.unit() * half * 0.78
            v = rng.unit() * half * 0.78
            radius = half * (0.055 + 0.04 * rng.next_f64())
            color = MARKER_COLORS[rng.next_u64() % len(MARKER_COLORS)]
            markers.append((u, v, radius, color))
        faces.append(markers)
    return faces


def marker_world_position(axis, positive, u, v, half):
    signed = half if positive else -half
    if axis == 0:
        return (signed, u, v)
    if axis == 1:
        return (u, signed, v)
    return (u, v, signed)


def ground_truth_points(markers, half):
    points = []
    for axis in range(3):
        for positive in (True, False):
            for (u, v, _r, _c) in markers[face_index(axis, positive)]:
                points.append(marker_world_position(axis, positive, u, v, half))
    for sx in (-half, half):
        for sy in (-half, half):
            for sz in (-half, half):
                points.append((sx, sy, sz))
    return points


def face_color(p, axis, markers, half):
    positive = p[axis] > 0.0
    if axis == 0:
        u, v = p[1], p[2]
    elif axis == 1:
        u, v = p[0], p[2]
    else:
        u, v = p[0], p[1]
    idx = face_index(axis, positive)
    for (mu, mv, radius, color) in markers[idx]:
        if ((u - mu) ** 2 + (v - mv) ** 2) ** 0.5 <= radius:
            return color
    _ = half
    return FACE_BASE_COLORS[idx]


def ray_box_intersect(origin, direction, half):
    t_min = -1.0e30
    t_max = 1.0e30
    hit_axis = 0
    for axis in range(3):
        d = direction[axis]
        o = origin[axis]
        if abs(d) < 1e-12:
            if abs(o) > half:
                return None
            continue
        t1 = (-half - o) / d
        t2 = (half - o) / d
        lo, hi = (t1, t2) if t1 <= t2 else (t2, t1)
        if lo > t_min:
            t_min = lo
            hit_axis = axis
        if hi < t_max:
            t_max = hi
    if t_max < max(t_min, 0.0):
        return None
    t = t_min if t_min > 0.0 else t_max
    if t <= 0.0:
        return None
    p = (origin[0] + direction[0] * t, origin[1] + direction[1] * t, origin[2] + direction[2] * t)
    return p, hit_axis


def camera_eyes():
    rng = SplitMix64(SEED ^ 0xA5A5A5A5A5A5A5A5)
    eyes = []
    for i in range(FRAME_COUNT):
        angle = math.tau * (i + 0.5) / FRAME_COUNT + rng.unit() * 0.06
        elevation = ORBIT_ELEVATION + rng.unit() * 0.10
        radius = ORBIT_RADIUS + rng.unit() * 0.06
        eyes.append((radius * math.cos(angle), elevation, radius * math.sin(angle)))
    return eyes


def render_frame(r_cw, eye, markers, half):
    """\U0001f5bc\ufe0f Analytic ray/box raster: `r_cw` is the camera-to-world rotation, `eye` the camera centre."""
    pixels = bytearray(WIDTH * HEIGHT * 4)
    for y in range(HEIGHT):
        for x in range(WIDTH):
            ray_world = normalize3(mat_act(r_cw, unproject_ray(x + 0.5, y + 0.5)))
            hit = ray_box_intersect(eye, ray_world, half)
            color = face_color(hit[0], hit[1], markers, half) if hit else BACKGROUND
            idx = (y * WIDTH + x) * 4
            pixels[idx] = color[0]
            pixels[idx + 1] = color[1]
            pixels[idx + 2] = color[2]
            pixels[idx + 3] = 255
    return bytes(pixels)


# endregion 🔖️Scene

# region 🔖️Png


def encode_png(width, height, rgba):
    raw = bytearray()
    stride = width * 4
    for y in range(height):
        raw.append(0)
        raw.extend(rgba[y * stride:(y + 1) * stride])

    def chunk(tag, payload):
        return struct.pack(">I", len(payload)) + tag + payload + struct.pack(">I", zlib.crc32(tag + payload) & 0xFFFFFFFF)

    header = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header) + chunk(b"IDAT", zlib.compress(bytes(raw), 9)) + chunk(b"IEND", b"")


# endregion 🔖️Png

# region 🔖️Emit


def f32(value):
    """\U0001f52c Prints a `f32` document field the way the DSL printer does — widened to its exact f64 value."""
    import struct

    return num(struct.unpack("f", struct.pack("f", value))[0])


def num(value):
    """\U0001f522 Prints a float the way the DSL printer prints a `NUM` cell (integral values lose the `.0`)."""
    return str(int(value)) if float(value) == int(value) else repr(float(value))


def dsl_document(frames):
    """\U0001f5e3\ufe0f Hand-printed mirror of `RemodelingSnapshot::print_dsl` for this fixture.

    The permanent Rust generator prints the same document through the real `store::ArtifactDsl`
    printer; this bootstrap exists so the example directory is complete (and the crate compiles)
    before the remodel crate has been built once.
    """
    rows = " ".join(f'{{index={row["index"]} timestamp-ms={num(row["timestampMs"])} asset-id={row["assetId"]}}}' for row in frames)
    return f"""semio remodeling.remodeling.dsl v1
schema=remodeling.scene id=remodeling assets={{
}}
calibration {{
  cameras [id:TEXT label:TEXT model:TEXT fx:NUM fy:NUM cx:NUM cy:NUM skew:NUM distortion:TUPLE rms-reprojection-px:NUM locked:BOOL] {{
    {CAMERA_ID} "Synthetic Orbit Camera" brownConrady {num(FX)} {num(FY)} {num(CX)} {num(CY)} 0 {f32(K1)},{f32(K2)},0,0,0 0.5 true
  }}
  rig [camera-id:TEXT rotation-wxyz:TUPLE translation-m:CRD] {{
  }}
}}
params {{
  ingest {{
    frame-sample-stride=1 max-frames=32 downscale-long-edge-px=320 min-sharpness=0
  }}
  feature {{
    detector=akaze target-count=600 octaves=4 edge-threshold=10
  }}
  matching {{
    matcher=brute-force ratio-test=0.8500000238418579 cross-check=true sequential-window=6 max-pairs-per-frame=16 loop-closure=true
  }}
  sfm {{
    ransac-iterations=1000 ransac-threshold-px=2 min-track-length=2 ba-max-iterations=25 robust-loss=huber huber-delta-px=1.5
  }}
  dense {{
    resolution=low window-radius-px=2 min-view-consistency=3 confidence-threshold=0.5 max-points=50000
  }}
  mesh {{
    tsdf-voxel-size-mm=100mm tsdf-truncation-mm=330mm decimate-target-triangles=20000 smoothing-iterations=1 texture-enabled=false texture-size=256 guarantee-watertight=true hole-fill-max-boundary-verts=512 self-intersection-check=false
  }}
  motion {{
    enabled=false max-tracks=64 track-window-px=21 min-track-quality=0.30000001192092896 min-track-length-frames=5
  }}
  geo {{
    enabled=false gsd-m=0.05000000074505806m dsm-cell-m=0.10000000149011612m dtm-filter-radius-m=2m ortho-max-px=4096
  }}
}}
job {{
  id="" stage=idle progress-0-1=0 cancel-requested=false stage-cursor=0 sparse-point-cloud-preview=""
  camera-poses-preview [camera-id:TEXT rotation-wxyz:TUPLE translation:CRD] {{
  }}
}}
results {{
  mesh {{
    source=placeholder
    mesh {{
      child_id=remodeling-mesh-901ccade3f60f8f1 target="remodeling-mesh!s.stdio.semio@v1/mesh"
    }}
  }}
  tracks [id:TEXT length:UINT class:ENUM mean-speed-m-s:QTY] {{
  }}
}}
streams [id:TEXT name:TEXT kind:ENUM camera-id:TEXT sync-offset-ms:NUM fps-hint:NUM frames:TABLE source:BLOCK] {{
  {STREAM_ID} "Synthetic Orbit" image-sequence {CAMERA_ID} 0 {num(FPS_HINT)} [ {rows} ] _
}}
gcps [id:TEXT name:TEXT world-position:CRD observations:TABLE] {{
}}
"""


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    repo = os.path.abspath(os.path.join(here, "..", "..", "..", "..", "..", "..", ".."))
    assets = os.path.join(
        repo,
        "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🖼️assets",
    )
    os.makedirs(assets, exist_ok=True)

    markers = generate_markers(SEED, CUBE_HALF)
    points = ground_truth_points(markers, CUBE_HALF)
    eyes = camera_eyes()

    frames = []
    extrinsics = []
    for i, eye in enumerate(eyes):
        r_wc, t_wc, r_cw = look_at_world_to_camera(eye, (0.0, 0.0, 0.0), (0.0, 1.0, 0.0))
        png = encode_png(WIDTH, HEIGHT, render_frame(r_cw, eye, markers, CUBE_HALF))
        name = f"🎞️frame-{i:02d}.png"
        with open(os.path.join(assets, name), "wb") as handle:
            handle.write(png)
        q = quat_from_mat(r_wc)
        q_cw = quat_from_mat(r_cw)
        frames.append({"index": i, "timestampMs": i * 1000.0 / FPS_HINT, "assetId": f"{STREAM_ID}-frame-{i}", "file": name})
        extrinsics.append(
            {
                "frameIndex": i,
                "cameraCenterM": [eye[0], eye[1], eye[2]],
                "rotationWxyzWorldToCamera": [q[0], q[1], q[2], q[3]],
                "rotationWxyzCameraToWorld": [q_cw[0], q_cw[1], q_cw[2], q_cw[3]],
                "translationWorldToCameraM": [t_wc[0], t_wc[1], t_wc[2]],
            }
        )

    truth = {
        "schema": "semio.remodeling.synthetic-orbit-ground-truth/1",
        "seed": SEED,
        "image": {"width": WIDTH, "height": HEIGHT},
        "intrinsics": {"fx": FX, "fy": FY, "cx": CX, "cy": CY, "skew": 0.0, "model": "brownConrady", "distortion": [K1, K2, 0.0, 0.0, 0.0]},
        "scene": {"kind": "textured-cube", "halfExtentM": CUBE_HALF, "orbitRadiusM": ORBIT_RADIUS, "orbitElevationM": ORBIT_ELEVATION},
        "streamId": STREAM_ID,
        "cameraId": CAMERA_ID,
        "fpsHint": FPS_HINT,
        "frames": frames,
        "extrinsics": extrinsics,
        "pointsWorldM": [list(p) for p in points],
    }
    with open(os.path.join(assets, "🔮️ground-truth.json"), "w", encoding="utf-8") as handle:
        json.dump(truth, handle, indent=2, ensure_ascii=False)
        handle.write("\n")
    with open(os.path.join(assets, "🗣️.dsl.semio"), "w", encoding="utf-8") as handle:
        handle.write(dsl_document(frames))
    print(f"wrote {len(frames)} frames, {len(points)} ground-truth points to {assets}")


if __name__ == "__main__":
    main()
