#!/usr/bin/env python3
"""🏞️ The two base scenes every `📸️mutate-remodeling-1` vector starts from, plus the content-address
minting the documents' own asset handles carry.

`toy` is the two-stream unit scene the 34 pre-existing vectors were authored against, corrected on
two counts: `durableArtifacts` (the field every fixture predated) is present, and `stream-a`'s video
source is 1024x768 so it agrees with `cam-a`'s own principal point (cx=512, cy=384) instead of
claiming 1920x1080 against a 1024x768 intrinsic.

`realworld` is modelled on this subset's own committed `📚️examples/🛰️synthetic-orbit` example — the
same 320x240 raster, the same fx=fy=272 Brown-Conrady intrinsic, the same ten-frame orbit and the
same ground-truth point cloud — extended into the shapes a real survey document carries and the
committed toy scene never did: a two-camera rig, a GCP network whose points are seen from several
frames across BOTH streams, a durable-artifact store with a real leaf per captured frame, a sparse
cloud of 104 real ground-truth points, a dense cloud with confidence and classification lanes, a
ten-pose trajectory, motion tracks, geo products and a QC report.

Every `assets[key].childId` in both scenes is a REAL `image_asset_child_handle` digest — Rust
`std::collections::hash_map::DefaultHasher`, SipHash-1-3 keyed (0, 0), over the `mime` and `data`
strings — reproduced here and checked against the committed `create-asset` vector's own minted
handle, so no handle in either document is an invented string.
"""
from __future__ import annotations

import base64
import json
import struct
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
SUBSET = REPO / "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUTATIONS = SUBSET / "🧬️schema/🧬️mutations"
EXAMPLE = SUBSET / "📚️examples/🛰️synthetic-orbit/🖼️assets/🔮️ground-truth.json"

SNAPSHOT_ORDER = ["schema", "id", "streams", "assets", "durableArtifacts", "calibration", "params", "gcps", "job", "results"]
DIFF_ORDER = [
    "artifact", "schema", "id", "streams", "assets", "durableArtifacts", "calibration", "params", "gcps", "job",
    "results", "selection", "activeUtilityId", "reportTable", "frameCursor", "camera", "layers", "locale",
]

# region 🔖️ContentAddress
_M = (1 << 64) - 1


def _rotl(value: int, bits: int) -> int:
    return ((value << bits) | (value >> (64 - bits))) & _M


class DefaultHasher:
    """#️⃣ Rust `std::collections::hash_map::DefaultHasher` — SipHash-1-3 keyed `(0, 0)`."""

    def __init__(self) -> None:
        self.v0, self.v1 = 0x736F6D6570736575, 0x646F72616E646F6D
        self.v2, self.v3 = 0x6C7967656E657261, 0x7465646279746573
        self.tail: list[int] = []
        self.length = 0

    def _round(self) -> None:
        self.v0 = (self.v0 + self.v1) & _M
        self.v1 = _rotl(self.v1, 13) ^ self.v0
        self.v0 = _rotl(self.v0, 32)
        self.v2 = (self.v2 + self.v3) & _M
        self.v3 = _rotl(self.v3, 16) ^ self.v2
        self.v0 = (self.v0 + self.v3) & _M
        self.v3 = _rotl(self.v3, 21) ^ self.v0
        self.v2 = (self.v2 + self.v1) & _M
        self.v1 = _rotl(self.v1, 17) ^ self.v2
        self.v2 = _rotl(self.v2, 32)

    def _block(self, word: int) -> None:
        self.v3 ^= word
        self._round()
        self.v0 ^= word

    def write(self, data: bytes | list[int]) -> None:
        for byte in data:
            self.tail.append(byte)
            self.length += 1
            if len(self.tail) == 8:
                word = 0
                for index in range(7, -1, -1):
                    word = (word << 8) | self.tail[index]
                self._block(word)
                self.tail = []

    def write_str(self, text: str) -> None:
        """🔤 `impl Hash for str` — the bytes, then a `0xff` terminator."""
        self.write(text.encode("utf-8"))
        self.write([0xFF])

    def finish(self) -> int:
        last = (self.length & 0xFF) << 56
        for index in range(len(self.tail) - 1, -1, -1):
            last |= self.tail[index] << (8 * index)
        self._block(last)
        self.v2 ^= 0xFF
        self._round()
        self._round()
        self._round()
        return (self.v0 ^ self.v1 ^ self.v2 ^ self.v3) & _M


DURABLE_CHUNK_RAW_BYTES = 4096
RASTER_CONTENT_BYTES = 1_114_112
CONTENT_HANDLE_PREFIX = "remodeling-content:"
MESH_STAGE_HANDLE_PREFIX = "mesh-stage:"


def image_asset_child_handle(asset_id: str, asset: dict) -> dict:
    """🕸️ `image_asset_child_handle` — the content-addressed CHILD handle for one bounded asset."""
    hasher = DefaultHasher()
    hasher.write_str(asset["mime"])
    hasher.write_str(asset["data"])
    child_id = "remodeling-asset-%016x" % hasher.finish()
    return {"childId": child_id, "target": {"artifactId": f"{asset_id}-image", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "image"}}}


def durable_remodeling_asset(asset: dict) -> dict | None:
    """🧩️ `durable_remodeling_asset` — bounded 4 KiB leaves, or `None` when the payload is too large."""
    try:
        raw = base64.b64decode(asset["data"], validate=True)
    except Exception:
        return None
    if len(raw) > RASTER_CONTENT_BYTES:
        return None
    chunks = [base64.b64encode(raw[offset : offset + DURABLE_CHUNK_RAW_BYTES]).decode("ascii") for offset in range(0, len(raw), DURABLE_CHUNK_RAW_BYTES)]
    return {"kind": "image", "mime": asset["mime"], "width": asset["width"], "height": asset["height"], "chunks": chunks}
# endregion 🔖️ContentAddress


# region 🔖️Packing
def pack_f32(values) -> str:
    """📦 base64 of little-endian f32 lanes — the `PackedF32` carrier every point buffer uses."""
    return base64.b64encode(b"".join(struct.pack("<f", float(v)) for v in values)).decode("ascii")


def pack_u8(values) -> str:
    """📦 base64 of raw bytes — the `PackedU8` carrier colours and classification use."""
    return base64.b64encode(bytes(int(v) & 0xFF for v in values)).decode("ascii")
# endregion 🔖️Packing


# region 🔖️CanonicalOrder
def canonical(scene: dict) -> dict:
    """🔢️ The document invariant every `create-*`/`add-*` diff maintains: each keyed collection in
    ascending key order. A base scene that violated it would make an otherwise exact inverse look
    broken, so both scenes are authored through this and `assert_canonical` proves it stayed true."""
    scene["streams"] = sorted(scene["streams"], key=lambda stream: stream["id"])
    for stream in scene["streams"]:
        stream["frames"] = sorted(stream["frames"], key=lambda frame: (frame["index"], frame["assetId"]))
    scene["calibration"]["cameras"] = sorted(scene["calibration"]["cameras"], key=lambda camera: camera["id"])
    scene["calibration"]["rig"] = sorted(scene["calibration"]["rig"], key=lambda entry: entry["cameraId"])
    scene["gcps"] = sorted(scene["gcps"], key=lambda gcp: gcp["id"])
    for gcp in scene["gcps"]:
        gcp["observations"] = sorted(gcp["observations"], key=lambda observation: (observation["streamId"], observation["frameIndex"]))
    scene["assets"] = {key: scene["assets"][key] for key in sorted(scene["assets"])}
    scene["durableArtifacts"] = {key: scene["durableArtifacts"][key] for key in sorted(scene["durableArtifacts"])}
    return scene


def assert_canonical(scene: dict) -> None:
    import copy as _copy

    assert canonical(_copy.deepcopy(scene)) == scene, f"scene {scene['id']!r} is not in canonical key order"
# endregion 🔖️CanonicalOrder


# region 🔖️ToyScene
SPARE_ASSET = {"mime": "image/png", "data": base64.b64encode(b"spare-raster").decode("ascii"), "width": 64, "height": 64}
# 🗺️ The raster `results.geo.dsmAssetId` names. The committed toy scene named it without carrying it,
# which is exactly the dangling reference `delete-asset`'s guard exists to prevent — so the scene now
# carries the leaf it points at.
TOY_DSM_ASSET = {"mime": "image/tiff", "data": base64.b64encode(b"toy-dsm-tile").decode("ascii"), "width": 128, "height": 128}
def toy_base() -> dict:
    """🧸 The committed two-stream unit scene, with `durableArtifacts` supplied and `stream-a`'s
    source resolution reconciled with `cam-a`'s principal point."""
    source = MUTATIONS / "🌱create-stream/🧪️tests/🎥️adds-stream-c-458900/📸️snapshot/⬅️before/🔣️.json"
    scene = json.loads(source.read_text(encoding="utf-8"))
    # 🔑 `asset-a`'s committed handle is the DefaultHasher digest of exactly this pair — brute-forced
    # against the committed value and confirmed — so its durable leaf is the document's own content,
    # not an invention. Without it `remodeling_asset` cannot reconstitute the handle and `delete-asset`
    # has no inverse at all (`🦀️.rs:233`).
    asset_a = {"mime": "image/jpeg", "data": "ZnJhbWUtYQ==", "width": 640, "height": 480}
    handle = scene["assets"]["asset-a"]
    assert handle["childId"] == image_asset_child_handle("asset-a", asset_a)["childId"], "the toy asset handle is no longer this payload's digest"
    durable = {handle["childId"]: durable_remodeling_asset(asset_a)}
    scene = {key: scene[key] for key in ("schema", "id", "streams", "assets")} | {"durableArtifacts": durable} | {key: scene[key] for key in ("calibration", "params", "gcps", "job", "results")}
    stream_a = scene["streams"][0]
    stream_a["source"]["width"] = 1024
    stream_a["source"]["height"] = 768
    # 🧷 An asset NOTHING references: `delete-asset` refuses a referenced key, so a scene whose only
    # raster is bound to a frame could not carry an applied delete at all.
    spare = image_asset_child_handle("asset-spare", SPARE_ASSET)
    scene["assets"]["asset-spare"] = spare
    scene["durableArtifacts"][spare["childId"]] = durable_remodeling_asset(SPARE_ASSET)
    dsm = image_asset_child_handle("asset-dsm", TOY_DSM_ASSET)
    scene["assets"]["asset-dsm"] = dsm
    scene["durableArtifacts"][dsm["childId"]] = durable_remodeling_asset(TOY_DSM_ASSET)
    return canonical(scene)
# endregion 🔖️ToyScene


# region 🔖️RealWorldScene
FRAME_COUNT = 10
FRAME_INTERVAL_MS = 500.0
RASTER = (320, 240)


def _frame_asset(index: int) -> dict:
    """🖼️ The bounded raster leaf standing for one captured 320x240 orbit frame."""
    return {"mime": "image/png", "data": base64.b64encode(f"orbit-frame-{index:02d}".encode()).decode("ascii"), "width": RASTER[0], "height": RASTER[1]}


DSM_ASSET = {"mime": "image/tiff", "data": base64.b64encode(b"orbit-dsm-tile").decode("ascii"), "width": 512, "height": 512}
# 🗑️ A raster the survey captured and then never bound to a frame, a texture or a geo product — the
# only asset `delete-asset`'s referential guard lets through, and the survey's own calibration-target
# shot is exactly that in practice.
ORPHAN_ASSET = {"mime": "image/png", "data": base64.b64encode(b"orbit-calibration-target").decode("ascii"), "width": 320, "height": 240}


def _ground_truth() -> dict:
    return json.loads(EXAMPLE.read_text(encoding="utf-8"))


def f32_lexeme_agrees(value: float) -> bool:
    """🔍 Whether this number's shortest f32 lexeme is its shortest f64 lexeme — the property that makes
    a committed document printer-agnostic. Requires the value to be exact in f32 first."""
    if not isinstance(value, float) or value != value or value in (float("inf"), float("-inf")):
        return False
    if struct.unpack("<f", struct.pack("<f", value))[0] != value:
        return False
    for digits in range(1, 18):
        candidate = f"%.{digits}g" % value
        if struct.unpack("<f", struct.pack("<f", float(candidate)))[0] == value:
            return float(candidate) == value and repr(float(candidate)) == repr(value)
    return False


def _quantize(value: float) -> float:
    """🎚️ Snaps to the most precise binary fraction whose SHORTEST f32 lexeme is also its shortest f64
    lexeme, so a committed number cannot depend on whether the writer treats its lane as an `f32` or
    widens it to an `f64` first — the disagreement F1 records between `serde_json::serialize_f32` and
    `🌱️value`'s `*self as f64`. The scale adapts to the magnitude (an f32 near 1.0 resolves ~7 decimal
    digits, one near 0.005 far more), and `f32_lexeme_agrees` proves the result for every number."""
    for exponent in range(23, -1, -1):
        candidate = round(float(value) * (2**exponent)) / (2**exponent)
        if f32_lexeme_agrees(candidate):
            return candidate
    raise AssertionError(f"no printer-agnostic binary fraction near {value!r}")


def _f64(value: float) -> float:
    return float(value)


def realworld_base() -> dict:
    return canonical(_realworld_scene())


def _realworld_scene() -> dict:
    truth = _ground_truth()
    points = truth["pointsWorldM"][:104]
    extrinsics = truth["extrinsics"]
    frames = [{"index": index, "timestampMs": _quantize(index * FRAME_INTERVAL_MS), "assetId": f"orbit-frame-{index:02d}"} for index in range(FRAME_COUNT)]
    secondary_frames = [{"index": index, "timestampMs": _quantize(index * FRAME_INTERVAL_MS + 40.0), "assetId": f"orbit-frame-{index:02d}"} for index in range(FRAME_COUNT)]

    assets: dict[str, dict] = {}
    durable: dict[str, dict] = {}
    for index in range(FRAME_COUNT):
        asset = _frame_asset(index)
        handle = image_asset_child_handle(f"orbit-frame-{index:02d}", asset)
        assets[f"orbit-frame-{index:02d}"] = handle
        durable[handle["childId"]] = durable_remodeling_asset(asset)
    dsm_handle = image_asset_child_handle("orbit-dsm", DSM_ASSET)
    assets["orbit-dsm"] = dsm_handle
    durable[dsm_handle["childId"]] = durable_remodeling_asset(DSM_ASSET)
    orphan_handle = image_asset_child_handle("orbit-orphan", ORPHAN_ASSET)
    assets["orbit-orphan"] = orphan_handle
    durable[orphan_handle["childId"]] = durable_remodeling_asset(ORPHAN_ASSET)
    assets = {key: assets[key] for key in sorted(assets)}
    durable = {key: durable[key] for key in sorted(durable)}

    sparse_lanes = [_quantize(component) for point in points for component in point]
    sparse_colors = [(index * 37) % 256 for index in range(len(points) * 3)]
    dense_points = points[:64]
    dense_lanes = [_quantize(component + 0.015625) for point in dense_points for component in point]
    dense_colors = [(index * 53) % 256 for index in range(len(dense_points) * 3)]
    dense_confidence = [_quantize(0.5 + (index % 8) / 16.0) for index in range(len(dense_points))]
    dense_classification = [2 if index % 4 else 6 for index in range(len(dense_points))]

    trajectory = [
        {
            "cameraId": "orbit-cam-primary",
            "rotationWxyz": [_quantize(component) for component in pose["rotationWxyzWorldToCamera"]],
            "translation": [_quantize(component) for component in pose["cameraCenterM"]],
        }
        for pose in extrinsics
    ]

    return {
        "schema": "remodeling.scene",
        "id": "orbit-survey",
        "streams": [
            {
                "id": "orbit-primary",
                "name": "Orbit Primary",
                "kind": "video",
                "cameraId": "orbit-cam-primary",
                "syncOffsetMs": 0.0,
                "fpsHint": 2.0,
                "frames": frames,
                "source": {"name": "orbit-primary.mp4", "container": "mp4", "codec": "avc", "durationMs": 5000.0, "frameCount": 10, "width": RASTER[0], "height": RASTER[1]},
            },
            {
                "id": "orbit-secondary",
                "name": "Orbit Secondary",
                "kind": "image-sequence",
                "cameraId": "orbit-cam-secondary",
                "syncOffsetMs": 40.0,
                "fpsHint": 2.0,
                "frames": secondary_frames,
                "source": None,
            },
            {
                "id": "orbit-spare",
                "name": "Orbit Spare Pass",
                "kind": "image-sequence",
                "cameraId": None,
                "syncOffsetMs": 0.0,
                "fpsHint": 1.0,
                "frames": [dict(frames[0]), dict(frames[1])],
                "source": None,
            },
        ],
        "assets": assets,
        "durableArtifacts": durable,
        "calibration": {
            "cameras": [
                {"id": "orbit-cam-primary", "label": "Orbit Primary", "model": "brownConrady", "fx": 272.0, "fy": 272.0, "cx": 160.0, "cy": 120.0, "skew": 0.0, "distortion": [_quantize(-0.02), _quantize(0.005), 0.0, 0.0, 0.0], "rmsReprojectionPx": _quantize(0.42), "locked": False},
                {"id": "orbit-cam-secondary", "label": "Orbit Secondary", "model": "pinhole", "fx": 272.0, "fy": 272.0, "cx": 160.0, "cy": 120.0, "skew": 0.0, "distortion": [0.0, 0.0, 0.0, 0.0, 0.0], "rmsReprojectionPx": _quantize(0.58), "locked": False},
                {"id": "orbit-cam-tertiary", "label": "Orbit Tertiary", "model": "pinhole", "fx": 272.0, "fy": 272.0, "cx": 160.0, "cy": 120.0, "skew": 0.0, "distortion": [0.0, 0.0, 0.0, 0.0, 0.0], "rmsReprojectionPx": None, "locked": True},
            ],
            "rig": [
                {"cameraId": "orbit-cam-primary", "rotationWxyz": [1.0, 0.0, 0.0, 0.0], "translationM": [0.0, 0.0, 0.0]},
                {"cameraId": "orbit-cam-secondary", "rotationWxyz": [_quantize(0.99875), 0.0, _quantize(0.04998), 0.0], "translationM": [0.125, 0.0, 0.0]},
            ],
        },
        "params": {
            "ingest": {"frameSampleStride": 1, "maxFrames": 32, "downscaleLongEdgePx": 320, "minSharpness": _quantize(0.125)},
            "feature": {"detector": "akaze", "targetCount": 600, "octaves": 4, "edgeThreshold": 10.0},
            "matching": {"matcher": "brute-force", "ratioTest": _quantize(0.85), "crossCheck": True, "sequentialWindow": 6, "maxPairsPerFrame": 16, "loopClosure": True},
            "sfm": {"ransacIterations": 1000, "ransacThresholdPx": 2.0, "minTrackLength": 2, "baMaxIterations": 25, "robustLoss": "huber", "huberDeltaPx": 1.5},
            "dense": {"resolution": "low", "windowRadiusPx": 2, "minViewConsistency": 3, "confidenceThreshold": 0.5, "maxPoints": 50000},
            "mesh": {"tsdfVoxelSizeMm": 100.0, "tsdfTruncationMm": 330.0, "decimateTargetTriangles": 20000, "smoothingIterations": 1, "textureEnabled": False, "textureSize": 256, "guaranteeWatertight": True, "holeFillMaxBoundaryVerts": 512, "selfIntersectionCheck": False},
            "motion": {"enabled": True, "maxTracks": 64, "trackWindowPx": 21, "minTrackQuality": _quantize(0.3), "minTrackLengthFrames": 5},
            "geo": {"enabled": True, "originLon": _quantize(8.5), "originLat": _quantize(47.375), "originAlt": 408.0, "gsdM": _quantize(0.05), "dsmCellM": _quantize(0.1), "dtmFilterRadiusM": 2.0, "orthoMaxPx": 4096},
        },
        "gcps": [
            {
                "id": "gcp-north-pier",
                "name": "North Pier",
                "worldPosition": [_quantize(1.0), _quantize(0.5), _quantize(-0.25)],
                "observations": [
                    {"streamId": "orbit-primary", "frameIndex": 0, "pixel": [_quantize(184.5), _quantize(96.25)]},
                    {"streamId": "orbit-primary", "frameIndex": 4, "pixel": [_quantize(140.75), _quantize(101.5)]},
                    {"streamId": "orbit-secondary", "frameIndex": 4, "pixel": [_quantize(151.0), _quantize(104.75)]},
                ],
            },
            {
                "id": "gcp-south-pier",
                "name": "South Pier",
                "worldPosition": [_quantize(-1.0), _quantize(-0.5), _quantize(-0.25)],
                "observations": [
                    {"streamId": "orbit-primary", "frameIndex": 2, "pixel": [_quantize(122.0), _quantize(150.5)]},
                    {"streamId": "orbit-secondary", "frameIndex": 7, "pixel": [_quantize(198.25), _quantize(143.0)]},
                ],
            },
            {
                "id": "gcp-roof-vent",
                "name": "Roof Vent",
                "worldPosition": [_quantize(0.25), _quantize(-0.75), _quantize(1.0)],
                "observations": [
                    {"streamId": "orbit-primary", "frameIndex": 6, "pixel": [_quantize(171.5), _quantize(58.0)]},
                    {"streamId": "orbit-secondary", "frameIndex": 6, "pixel": [_quantize(163.25), _quantize(61.75)]},
                ],
            },
            {"id": "gcp-unsurveyed", "name": "Unsurveyed Mark", "worldPosition": [_quantize(0.0), _quantize(1.25), _quantize(0.0)], "observations": []},
        ],
        "job": {
            "id": "orbit-run-7",
            "stage": "dense-stereo",
            "progress01": _quantize(0.625),
            "cancelRequested": False,
            "stageCursor": 8,
            "startedAtMs": _f64(1738368000000.0),
            "error": None,
            "cameraPosesPreview": trajectory[:2],
            "sparsePointCloudPreview": pack_f32(sparse_lanes[:24]),
        },
        "results": {
            "sparse": {"points": pack_f32(sparse_lanes), "colors": pack_u8(sparse_colors)},
            "dense": {"positions": pack_f32(dense_lanes), "colors": pack_u8(dense_colors), "confidence": pack_f32(dense_confidence), "classification": pack_u8(dense_classification)},
            "mesh": {
                "mesh": {"childId": "remodeling-mesh-901ccade3f60f8f1", "target": {"artifactId": "orbit-survey-mesh", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "mesh"}}},
                "source": "reconstructed",
                "textureAssetId": "orbit-frame-00",
                "watertight": {
                    "vertexCount": 4820, "triangleCount": 9636, "boundaryEdgeCount": 0, "boundaryLoopCount": 0, "nonManifoldEdgeCount": 0, "nonManifoldVertexCount": 0,
                    "connectedComponents": 1, "consistentlyOriented": True, "eulerCharacteristic": 2, "genus": 0, "signedVolume": _quantize(7.9375),
                    "selfIntersectionPairs": 0, "closedFallbackUsed": False, "isClosed": True, "isTwoManifold": True, "isWatertight": True,
                },
            },
            "trajectory": {"poses": trajectory},
            "tracks": [
                {"id": "orbit-track-a", "length": 9, "class": "static", "meanSpeedMS": 0.0},
                {"id": "orbit-track-b", "length": 7, "class": "moving", "meanSpeedMS": _quantize(0.75)},
                {"id": "orbit-track-c", "length": 4, "class": "moving", "meanSpeedMS": _quantize(0.125)},
            ],
            "geo": {"dsmAssetId": "orbit-dsm", "dtmAssetId": None, "orthoAssetId": None},
            "qc": {
                "reprojectionRmsPx": _quantize(0.42), "gcpCheckpointRmse": _quantize(0.1875), "watertight": None,
                "meanTrackLength": _quantize(6.5), "registeredFrameRatio": 1.0, "denseCoverageRatio": _quantize(0.625),
                "warnings": ["only two streams observe gcp-south-pier"],
            },
        },
    }
# endregion 🔖️RealWorldScene


def _numbers(node):
    """🔢️ Every float literal a scene carries, so the printer-agnostic check can reach all of them."""
    if isinstance(node, float):
        yield node
    elif isinstance(node, dict):
        for value in node.values():
            yield from _numbers(value)
    elif isinstance(node, list):
        for value in node:
            yield from _numbers(value)


if __name__ == "__main__":
    toy = toy_base()
    real = realworld_base()
    assert list(toy) == SNAPSHOT_ORDER, list(toy)
    assert list(real) == SNAPSHOT_ORDER, list(real)
    assert_canonical(toy)
    assert_canonical(real)
    for name, scene in (("toy", toy), ("realworld", real)):
        for number in _numbers(scene):
            # 🎚️ A value no `f32` can hold cannot be on an `f32` lane, so its lexeme has no ambiguity
            # to resolve; every value that COULD be one must print the same at either width.
            if struct.unpack("<f", struct.pack("<f", number))[0] != number:
                continue
            assert f32_lexeme_agrees(number), f"{name}: {number!r} prints differently as an f32 than as an f64"
    assert image_asset_child_handle("asset-b", {"mime": "image/jpeg", "data": "ZnJhbWUtYg=="})["childId"] == "remodeling-asset-75b20f8d69a86e9a"
    assert toy["assets"]["asset-a"]["childId"] == image_asset_child_handle("asset-a", {"mime": "image/jpeg", "data": "ZnJhbWUtYQ=="})["childId"]
    print("toy bytes    ", len(json.dumps(toy, indent=2, ensure_ascii=False)))
    print("realworld    ", len(json.dumps(real, indent=2, ensure_ascii=False)))
    print("sparse points", len(base64.b64decode(real["results"]["sparse"]["points"])) // 12)
    print("durable      ", len(real["durableArtifacts"]))
