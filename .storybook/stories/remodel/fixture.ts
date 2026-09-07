// #region 🧲️Header
// 💻️ .storybook/stories/remodel/fixture.ts
// Specs: Story-local, self-contained copies of the remodel plugin's own populated document fixture and of
// its editor config defaults + complete en/de label set — the data half of the `📸️remodel` scope's stories.
// Summary: `REMODEL_POPULATED_SCENE` is a verbatim transcription of the shared mutation-fixture document
// `🗿️artifacts/📸️remodeling/…/🧬️schema/🧬️mutations/*/🧪️tests/<toy case>/📸️snapshot/⬅️before/🔣️.json` (the shared
// toy scene every `toy`-role vector starts from — two streams, three assets with their durable leaves, two
// calibrated cameras, two GCPs in canonical id order, a mid-run `bundle-adjusting` job with a camera-pose
// preview, and finished sparse/dense/trajectory/tracks/geo/qc results). It is copied rather than
// imported because a `?raw`/`?json` import of a plugin-owned asset would couple this scope to that plugin
// directory's layout, which is actively being renamed. `REMODEL_EMPTY_SCENE` mirrors `default_remodeling_scene()`
// (`🗿️artifacts/📸️remodeling/🦀️.rs`) — everything empty, the placeholder mesh handle seeded — which is also what
// the shipped `📚️examples/🎬️demo` document contains (its `🗣️.dsl.semio` declares zero streams/gcps/cameras).
// `REMODEL_LABELS` transcribes every field of `app_labels! { RemodelingLabels }` (`✏️editor/🗣️terminology/🦀️.rs`)
// in both native locales, so a story renders the same strings the Rust panels would for `locale: "en-US"/"de-DE"`.
// ⚠️ The populated document now carries its `durableArtifacts` store, so every `assets[key]` handle in it
// resolves to real content — but no leaf exists for the composed `s.stdio.semio@v1/mesh` CHILD handle in
// `results.mesh.mesh`, which resolves to nothing, matching `resolve_bounded_remodeling_mesh`'s documented
// "unavailable content renders no mesh entity" behavior. Every mesh-shaped readout below is therefore honestly
// empty, exactly as the plugin's own render path would be for these documents.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

//#region 🔖️SceneModel
/** @emoji 🎞️ One sampled frame of a media stream — `RemodelingSnapshot::streams[].frames[]`. */
export type RemodelFrameRef = { readonly index: number; readonly timestampMs: number; readonly assetId: string };

/** @emoji 📼️ The decoded-container facts a video import records — `streams[].source`. */
export type RemodelStreamSource = { readonly name: string; readonly container: string; readonly codec: string; readonly durationMs: number; readonly frameCount: number; readonly width: number; readonly height: number };

/** @emoji 🎥️ One media stream — `RemodelingSnapshot::streams[]`. */
export type RemodelStream = {
  readonly id: string;
  readonly name: string;
  readonly kind: "video" | "image-sequence";
  readonly cameraId: string | null;
  readonly syncOffsetMs: number;
  readonly fpsHint: number;
  readonly frames: readonly RemodelFrameRef[];
  readonly source: RemodelStreamSource | null;
};

/** @emoji 🧩️ A composed child-artifact handle (`s.stdio.semio@v1/<subset>`) — opaque geometry/pixels live behind it. */
export type RemodelChildHandle = { readonly childId: string; readonly target: { readonly artifactId: string; readonly dialect: { readonly artifactKind: string; readonly standard: string; readonly subset: string } } };

/** @emoji 💧️ The watertightness audit a reconstructed mesh carries — `results.mesh.watertight` / `results.qc.watertight`. */
export type RemodelWatertightReport = {
  readonly vertexCount: number;
  readonly triangleCount: number;
  readonly boundaryEdgeCount: number;
  readonly boundaryLoopCount: number;
  readonly nonManifoldEdgeCount: number;
  readonly nonManifoldVertexCount: number;
  readonly connectedComponents: number;
  readonly consistentlyOriented: boolean;
  readonly eulerCharacteristic: number;
  readonly genus: number | null;
  readonly signedVolume: number;
  readonly selfIntersectionPairs: number;
  readonly closedFallbackUsed: boolean;
  readonly isClosed: boolean;
  readonly isTwoManifold: boolean;
  readonly isWatertight: boolean;
};

/** @emoji 🧱️ One document-owned durable leaf — `durableArtifacts[childId]`, the content an asset handle addresses. */
export type RemodelDurableArtifact = { readonly kind: string; readonly mime: string | null; readonly width: number; readonly height: number; readonly chunks: readonly string[] };

/** @emoji 📷️ One calibrated camera — `calibration.cameras[]`. */
export type RemodelCamera = {
  readonly id: string;
  readonly label: string;
  readonly model: string;
  readonly fx: number;
  readonly fy: number;
  readonly cx: number;
  readonly cy: number;
  readonly skew: number;
  readonly distortion: readonly number[];
  readonly rmsReprojectionPx: number | null;
  readonly locked: boolean;
};

/** @emoji 🎯️ One ground control point and its per-frame pixel observations — `gcps[]`. */
export type RemodelGcp = {
  readonly id: string;
  readonly name: string;
  readonly worldPosition: readonly [number, number, number];
  readonly observations: readonly { readonly streamId: string; readonly frameIndex: number; readonly pixel: readonly [number, number] }[];
};

/** @emoji 🧭️ A recovered camera pose — `job.cameraPosesPreview[]` and `results.trajectory.poses[]`. */
export type RemodelPose = { readonly cameraId: string; readonly rotationWxyz: readonly number[]; readonly translation: readonly [number, number, number] };

/** @emoji 🚦️ The reconstruction job's persisted continuation state — `job`. */
export type RemodelJob = {
  readonly id: string;
  readonly stage: string;
  readonly progress01: number;
  readonly cancelRequested: boolean;
  readonly stageCursor: number;
  readonly startedAtMs: number | null;
  readonly error: string | null;
  readonly cameraPosesPreview: readonly RemodelPose[];
  readonly sparsePointCloudPreview: string;
};

/** @emoji 📸️ The remodeling document — a story-local mirror of `RemodelingSnapshot`'s camelCase wire shape. */
export type RemodelScene = {
  readonly schema: string;
  readonly id: string;
  readonly streams: readonly RemodelStream[];
  readonly assets: Readonly<Record<string, RemodelChildHandle>>;
  readonly durableArtifacts: Readonly<Record<string, RemodelDurableArtifact>>;
  readonly calibration: { readonly cameras: readonly RemodelCamera[]; readonly rig: readonly { readonly cameraId: string; readonly rotationWxyz: readonly number[]; readonly translationM: readonly number[] }[] };
  readonly params: {
    readonly ingest: { readonly frameSampleStride: number; readonly maxFrames: number; readonly downscaleLongEdgePx: number; readonly minSharpness: number };
    readonly feature: { readonly detector: string; readonly targetCount: number; readonly octaves: number; readonly edgeThreshold: number };
    readonly matching: { readonly matcher: string; readonly ratioTest: number; readonly crossCheck: boolean; readonly sequentialWindow: number; readonly maxPairsPerFrame: number; readonly loopClosure: boolean };
    readonly sfm: { readonly ransacIterations: number; readonly ransacThresholdPx: number; readonly minTrackLength: number; readonly baMaxIterations: number; readonly robustLoss: string; readonly huberDeltaPx: number };
    readonly dense: { readonly resolution: string; readonly windowRadiusPx: number; readonly minViewConsistency: number; readonly confidenceThreshold: number; readonly maxPoints: number };
    readonly mesh: { readonly tsdfVoxelSizeMm: number; readonly tsdfTruncationMm: number; readonly decimateTargetTriangles: number; readonly smoothingIterations: number; readonly textureEnabled: boolean; readonly textureSize: number; readonly guaranteeWatertight: boolean; readonly holeFillMaxBoundaryVerts: number; readonly selfIntersectionCheck: boolean };
    readonly motion: { readonly enabled: boolean; readonly maxTracks: number; readonly trackWindowPx: number; readonly minTrackQuality: number; readonly minTrackLengthFrames: number };
    readonly geo: { readonly enabled: boolean; readonly originLon: number | null; readonly originLat: number | null; readonly originAlt: number | null; readonly gsdM: number; readonly dsmCellM: number; readonly dtmFilterRadiusM: number; readonly orthoMaxPx: number };
  };
  readonly gcps: readonly RemodelGcp[];
  readonly job: RemodelJob;
  readonly results: {
    readonly sparse: { readonly points: string; readonly colors: string | null } | null;
    readonly dense: { readonly positions: string; readonly colors: string | null; readonly confidence: string | null; readonly classification: string | null } | null;
    readonly mesh: { readonly mesh: RemodelChildHandle; readonly source: string; readonly textureAssetId: string | null; readonly watertight: RemodelWatertightReport | null };
    readonly trajectory: { readonly poses: readonly RemodelPose[] } | null;
    readonly tracks: readonly { readonly id: string; readonly length: number; readonly class: string; readonly meanSpeedMS: number }[];
    readonly geo: { readonly dsmAssetId: string | null; readonly dtmAssetId: string | null; readonly orthoAssetId: string | null } | null;
    readonly qc: {
      readonly reprojectionRmsPx: number;
      readonly gcpCheckpointRmse: number | null;
      readonly watertight: RemodelWatertightReport | null;
      readonly meanTrackLength: number;
      readonly registeredFrameRatio: number;
      readonly denseCoverageRatio: number;
      readonly warnings: readonly string[];
    } | null;
  };
};
//#endregion 🔖️SceneModel

//#region 🔖️Documents
/** 📸️ The shared populated fixture document, transcribed field-for-field from the plugin's own `⬅️before/🔣️.json`. */
export const REMODEL_POPULATED_SCENE: RemodelScene = {
  schema: "remodeling.scene",
  id: "remodeling-fixture",
  streams: [{
    id: "stream-a",
    name: "Front",
    kind: "video",
    cameraId: "cam-a",
    syncOffsetMs: 12.5,
    fpsHint: 30,
    frames: [{
      index: 0,
      timestampMs: 0,
      assetId: "asset-a"
    }, {
      index: 1,
      timestampMs: 40,
      assetId: "asset-a"
    }],
    source: {
      name: "front.mp4",
      container: "mp4",
      codec: "avc",
      durationMs: 6400,
      frameCount: 160,
      width: 1024,
      height: 768
    }
  }, {
    id: "stream-b",
    name: "Side",
    kind: "image-sequence",
    cameraId: null,
    syncOffsetMs: 0,
    fpsHint: 24,
    frames: [{
      index: 0,
      timestampMs: 0,
      assetId: "asset-a"
    }],
    source: null
  }],
  assets: {
    "asset-a": {
      childId: "remodeling-asset-45070beb0101de64",
      target: {
        artifactId: "asset-a-image",
        dialect: {
          artifactKind: "s.stdio.semio",
          standard: "v1",
          subset: "image"
        }
      }
    },
    "asset-dsm": {
      childId: "remodeling-asset-a462ec9257d3fccf",
      target: {
        artifactId: "asset-dsm-image",
        dialect: {
          artifactKind: "s.stdio.semio",
          standard: "v1",
          subset: "image"
        }
      }
    },
    "asset-spare": {
      childId: "remodeling-asset-84ec1bd76bb83a25",
      target: {
        artifactId: "asset-spare-image",
        dialect: {
          artifactKind: "s.stdio.semio",
          standard: "v1",
          subset: "image"
        }
      }
    }
  },
  durableArtifacts: {
    "remodeling-asset-45070beb0101de64": {
      kind: "image",
      mime: "image/jpeg",
      width: 640,
      height: 480,
      chunks: ["ZnJhbWUtYQ=="]
    },
    "remodeling-asset-84ec1bd76bb83a25": {
      kind: "image",
      mime: "image/png",
      width: 64,
      height: 64,
      chunks: ["c3BhcmUtcmFzdGVy"]
    },
    "remodeling-asset-a462ec9257d3fccf": {
      kind: "image",
      mime: "image/tiff",
      width: 128,
      height: 128,
      chunks: ["dG95LWRzbS10aWxl"]
    }
  },
  calibration: {
    cameras: [{
      id: "cam-a",
      label: "Front",
      model: "brownConrady",
      fx: 1000,
      fy: 1000,
      cx: 512,
      cy: 384,
      skew: 0,
      distortion: [0.0625, -0.03125, 0, 0, 0],
      rmsReprojectionPx: 0.5,
      locked: false
    }, {
      id: "cam-b",
      label: "Side",
      model: "pinhole",
      fx: 800,
      fy: 800,
      cx: 320,
      cy: 240,
      skew: 0,
      distortion: [0, 0, 0, 0, 0],
      rmsReprojectionPx: null,
      locked: true
    }],
    rig: [{
      cameraId: "cam-a",
      rotationWxyz: [1, 0, 0, 0],
      translationM: [0, 0, 0]
    }]
  },
  params: {
    ingest: {
      frameSampleStride: 5,
      maxFrames: 200,
      downscaleLongEdgePx: 1600,
      minSharpness: 0.25
    },
    feature: {
      detector: "orb",
      targetCount: 4000,
      octaves: 4,
      edgeThreshold: 10
    },
    matching: {
      matcher: "brute-force",
      ratioTest: 0.75,
      crossCheck: true,
      sequentialWindow: 8,
      maxPairsPerFrame: 16,
      loopClosure: true
    },
    sfm: {
      ransacIterations: 1000,
      ransacThresholdPx: 2,
      minTrackLength: 3,
      baMaxIterations: 50,
      robustLoss: "huber",
      huberDeltaPx: 1.5
    },
    dense: {
      resolution: "medium",
      windowRadiusPx: 3,
      minViewConsistency: 3,
      confidenceThreshold: 0.5,
      maxPoints: 500000
    },
    mesh: {
      tsdfVoxelSizeMm: 5,
      tsdfTruncationMm: 20,
      decimateTargetTriangles: 200000,
      smoothingIterations: 2,
      textureEnabled: true,
      textureSize: 2048,
      guaranteeWatertight: true,
      holeFillMaxBoundaryVerts: 512,
      selfIntersectionCheck: false
    },
    motion: {
      enabled: false,
      maxTracks: 64,
      trackWindowPx: 21,
      minTrackQuality: 0.25,
      minTrackLengthFrames: 5
    },
    geo: {
      enabled: false,
      originLon: null,
      originLat: null,
      originAlt: null,
      gsdM: 0.0625,
      dsmCellM: 0.125,
      dtmFilterRadiusM: 2,
      orthoMaxPx: 4096
    }
  },
  gcps: [{
    id: "gcp-corner",
    name: "Corner",
    worldPosition: [1, 2, 3],
    observations: [{
      streamId: "stream-b",
      frameIndex: 0,
      pixel: [10, 20]
    }]
  }, {
    id: "gcp-ridge",
    name: "Ridge",
    worldPosition: [4, 5, 6],
    observations: []
  }],
  job: {
    id: "job-a",
    stage: "bundle-adjusting",
    progress01: 0.5,
    cancelRequested: false,
    stageCursor: 3,
    startedAtMs: 1000,
    error: null,
    cameraPosesPreview: [{
      cameraId: "cam-a",
      rotationWxyz: [1, 0, 0, 0],
      translation: [0, 0, 0]
    }],
    sparsePointCloudPreview: "AAAAPwAAgD4AAAA+"
  },
  results: {
    sparse: {
      points: "AAAAAAAAAAAAAAAAAACAPwAAgD8AAIA/",
      colors: "/wAAAP8A"
    },
    dense: {
      positions: "AAAAAAAAAAAAAAAA",
      colors: "AAD/",
      confidence: "AAAAPw==",
      classification: "Ag=="
    },
    mesh: {
      mesh: {
        childId: "remodeling-mesh-901ccade3f60f8f1",
        target: {
          artifactId: "remodeling-mesh",
          dialect: {
            artifactKind: "s.stdio.semio",
            standard: "v1",
            subset: "mesh"
          }
        }
      },
      source: "reconstructed",
      textureAssetId: "asset-a",
      watertight: {
        vertexCount: 512,
        triangleCount: 1020,
        boundaryEdgeCount: 0,
        boundaryLoopCount: 0,
        nonManifoldEdgeCount: 0,
        nonManifoldVertexCount: 0,
        connectedComponents: 1,
        consistentlyOriented: true,
        eulerCharacteristic: 2,
        genus: 0,
        signedVolume: 12.5,
        selfIntersectionPairs: 0,
        closedFallbackUsed: false,
        isClosed: true,
        isTwoManifold: true,
        isWatertight: true
      }
    },
    trajectory: {
      poses: [{
        cameraId: "cam-a",
        rotationWxyz: [1, 0, 0, 0],
        translation: [0, 0, 0]
      }, {
        cameraId: "cam-a",
        rotationWxyz: [0.75, 0.25, 0, 0],
        translation: [0.5, 0, 0]
      }]
    },
    tracks: [{
      id: "track-a",
      length: 42,
      class: "moving",
      meanSpeedMS: 1.5
    }],
    geo: {
      dsmAssetId: "asset-dsm",
      dtmAssetId: null,
      orthoAssetId: null
    },
    qc: {
      reprojectionRmsPx: 0.5,
      gcpCheckpointRmse: 0.25,
      watertight: null,
      meanTrackLength: 6,
      registeredFrameRatio: 1,
      denseCoverageRatio: 0.75,
      warnings: ["low overlap on frame 12"]
    }
  }
};

/** 🌱️ The boot document — `default_remodeling_scene()` / the shipped `📚️examples/🎬️demo` DSL: everything empty, the placeholder mesh handle seeded. */
export const REMODEL_EMPTY_SCENE: RemodelScene = {
  ...REMODEL_POPULATED_SCENE,
  id: "remodeling",
  streams: [],
  assets: {},
  durableArtifacts: {},
  calibration: { cameras: [], rig: [] },
  gcps: [],
  job: { id: "", stage: "idle", progress01: 0, cancelRequested: false, stageCursor: 0, startedAtMs: null, error: null, cameraPosesPreview: [], sparsePointCloudPreview: "" },
  results: { sparse: null, dense: null, mesh: { mesh: REMODEL_POPULATED_SCENE.results.mesh.mesh, source: "placeholder", textureAssetId: null, watertight: null }, trajectory: null, tracks: [], geo: null, qc: null },
};
//#endregion 🔖️Documents

//#region 🔖️Config
/** @emoji 🎚️ Story-local mirror of `RemodelingConfig` (`✏️editor/🎚️config/🦀️.rs`). */
export type RemodelConfig = {
  readonly camera: { readonly position: readonly [number, number, number]; readonly target: readonly [number, number, number]; readonly fov: number };
  readonly layers: { readonly mesh: boolean; readonly dense: boolean; readonly sparse: boolean; readonly cameras: boolean; readonly gcps: boolean };
  readonly frameCursor: { readonly streamId: string | null; readonly frameIndex: number };
  readonly reportTable: string;
  readonly activeUtilityId: string;
  readonly locale: string;
};

/** 🎚️ `RemodelingConfig::default()` verbatim. */
export const REMODEL_DEFAULT_CONFIG: RemodelConfig = {
  camera: { position: [4, -4, 3], target: [0, 0, 0], fov: 45 },
  layers: { mesh: true, dense: true, sparse: true, cameras: true, gcps: true },
  frameCursor: { streamId: null, frameIndex: 0 },
  reportTable: "frames",
  activeUtilityId: "select",
  locale: "en-US",
};
//#endregion 🔖️Config

//#region 🔖️Labels
/** @emoji 🗣️ One locale's resolution of `RemodelingLabels` — one field per `app_labels!` row, native variant. */
export type RemodelLabels = Readonly<Record<RemodelLabelKey, string>>;

export type RemodelLabelKey =
  | "model"
  | "capture"
  | "analyze"
  | "reconstruction"
  | "error"
  | "status"
  | "running"
  | "idle"
  | "utility"
  | "mesh"
  | "vertices"
  | "triangles"
  | "streams"
  | "assets"
  | "noStreams"
  | "streamKindVideo"
  | "streamKindImageSequence"
  | "frames"
  | "syncOffset"
  | "sparseCloud"
  | "denseCloud"
  | "resultsNone"
  | "trajectory"
  | "poses"
  | "geoProducts"
  | "available"
  | "paramsIngest"
  | "paramsFeature"
  | "paramsMatching"
  | "paramsSfm"
  | "paramsDense"
  | "paramsMesh"
  | "paramsMotion"
  | "paramsGeo"
  | "strideShort"
  | "maxShort"
  | "downscaleShort"
  | "targetShort"
  | "octavesShort"
  | "ratioShort"
  | "windowShort"
  | "ransacShort"
  | "minTrackShort"
  | "baShort"
  | "voxelShort"
  | "enabled"
  | "disabled"
  | "camerasCalibrated"
  | "rigExtrinsics"
  | "gcps"
  | "tracks"
  | "tracksNone"
  | "motionNotImplemented"
  | "qcNone"
  | "qcReprojection"
  | "qcTrackLength"
  | "qcRegisteredRatio"
  | "qcDenseCoverage"
  | "qcGcpRmse"
  | "qcWatertight"
  | "qcBoundaryEdges"
  | "qcComponents"
  | "qcEuler"
  | "qcGenus"
  | "qcClosedFallback"
  | "panelMedia"
  | "panelPipeline"
  | "panelResults"
  | "panelParameters"
  | "panelCalibration"
  | "panelTracks"
  | "panelQc"
  | "windowFrames"
  | "windowReport"
  | "layers"
  | "layerMesh"
  | "layerDense"
  | "layerSparse"
  | "layerCameras"
  | "layerGcps";

/** 🇬🇧️ `native_en` column of `app_labels! { RemodelingLabels }`, transcribed verbatim. */
const REMODEL_LABELS_EN: RemodelLabels = {
  model: "Model",
  capture: "Capture",
  analyze: "Analyze",
  reconstruction: "Reconstruction",
  error: "error",
  status: "Status",
  running: "Running",
  idle: "Idle",
  utility: "Utility",
  mesh: "Mesh",
  vertices: "vertices",
  triangles: "triangles",
  streams: "Streams",
  assets: "Assets",
  noStreams: "No media streams imported yet",
  streamKindVideo: "video",
  streamKindImageSequence: "image sequence",
  frames: "frames",
  syncOffset: "sync offset",
  sparseCloud: "Sparse point cloud",
  denseCloud: "Dense point cloud",
  resultsNone: "none",
  trajectory: "Trajectory",
  poses: "poses",
  geoProducts: "Geo products",
  available: "available",
  paramsIngest: "Ingest",
  paramsFeature: "Feature",
  paramsMatching: "Matching",
  paramsSfm: "SfM",
  paramsDense: "Dense",
  paramsMesh: "Mesh",
  paramsMotion: "Motion",
  paramsGeo: "Geo",
  strideShort: "stride",
  maxShort: "max",
  downscaleShort: "downscale",
  targetShort: "target",
  octavesShort: "octaves",
  ratioShort: "ratio",
  windowShort: "window",
  ransacShort: "ransac",
  minTrackShort: "min track",
  baShort: "ba",
  voxelShort: "voxel",
  enabled: "enabled",
  disabled: "disabled",
  camerasCalibrated: "Calibrated cameras",
  rigExtrinsics: "Rig extrinsics",
  gcps: "Ground control points",
  tracks: "Motion tracks",
  tracksNone: "No motion tracks",
  motionNotImplemented: "Motion tracking is not yet driven by the reconstruction engine",
  qcNone: "No quality report yet",
  qcReprojection: "Mean reprojection error",
  qcTrackLength: "Mean track length",
  qcRegisteredRatio: "Registered frame ratio",
  qcDenseCoverage: "Dense coverage ratio",
  qcGcpRmse: "GCP checkpoint RMSE",
  qcWatertight: "Watertight",
  qcBoundaryEdges: "Boundary edges",
  qcComponents: "Connected components",
  qcEuler: "Euler characteristic",
  qcGenus: "Genus",
  qcClosedFallback: "Closed via fallback",
  panelMedia: "Media",
  panelPipeline: "Pipeline",
  panelResults: "Results",
  panelParameters: "Parameters",
  panelCalibration: "Calibration",
  panelTracks: "Tracks",
  panelQc: "Quality",
  windowFrames: "Frames",
  windowReport: "Report",
  layers: "Layers",
  layerMesh: "Mesh",
  layerDense: "Dense cloud",
  layerSparse: "Sparse cloud",
  layerCameras: "Cameras",
  layerGcps: "GCPs",
};

/** 🇩🇪️ `native_de` column of `app_labels! { RemodelingLabels }`, transcribed verbatim (umlauts included, as authored). */
const REMODEL_LABELS_DE: RemodelLabels = {
  model: "Modell",
  capture: "Aufnahme",
  analyze: "Analyse",
  reconstruction: "Rekonstruktion",
  error: "Fehler",
  status: "Status",
  running: "Läuft",
  idle: "Leerlauf",
  utility: "Werkzeug",
  mesh: "Mesh",
  vertices: "Vertices",
  triangles: "Dreiecke",
  streams: "Streams",
  assets: "Assets",
  noStreams: "Noch keine Medien-Streams importiert",
  streamKindVideo: "Video",
  streamKindImageSequence: "Bildsequenz",
  frames: "Frames",
  syncOffset: "Sync-Versatz",
  sparseCloud: "Dünne Punktwolke",
  denseCloud: "Dichte Punktwolke",
  resultsNone: "keine",
  trajectory: "Trajektorie",
  poses: "Posen",
  geoProducts: "Geo-Produkte",
  available: "verfügbar",
  paramsIngest: "Ingest",
  paramsFeature: "Feature",
  paramsMatching: "Matching",
  paramsSfm: "SfM",
  paramsDense: "Dense",
  paramsMesh: "Mesh",
  paramsMotion: "Bewegung",
  paramsGeo: "Geo",
  strideShort: "Schrittweite",
  maxShort: "max",
  downscaleShort: "Verkleinerung",
  targetShort: "Ziel",
  octavesShort: "Oktaven",
  ratioShort: "Verhältnis",
  windowShort: "Fenster",
  ransacShort: "Ransac",
  minTrackShort: "min. Spur",
  baShort: "BA",
  voxelShort: "Voxel",
  enabled: "aktiviert",
  disabled: "deaktiviert",
  camerasCalibrated: "Kalibrierte Kameras",
  rigExtrinsics: "Rig-Extrinsik",
  gcps: "Passpunkte",
  tracks: "Bewegungsspuren",
  tracksNone: "Keine Bewegungsspuren",
  motionNotImplemented: "Bewegungsverfolgung wird von der Rekonstruktions-Engine noch nicht ausgeführt",
  qcNone: "Noch kein Qualitätsbericht",
  qcReprojection: "Mittlerer Reprojektionsfehler",
  qcTrackLength: "Mittlere Spurlänge",
  qcRegisteredRatio: "Anteil registrierter Frames",
  qcDenseCoverage: "Dense-Abdeckungsanteil",
  qcGcpRmse: "Passpunkt-Kontroll-RMSE",
  qcWatertight: "Wasserdicht",
  qcBoundaryEdges: "Ränder",
  qcComponents: "Zusammenhangskomponenten",
  qcEuler: "Euler-Charakteristik",
  qcGenus: "Genus",
  qcClosedFallback: "Über Fallback geschlossen",
  panelMedia: "Medien",
  panelPipeline: "Pipeline",
  panelResults: "Ergebnisse",
  panelParameters: "Parameter",
  panelCalibration: "Kalibrierung",
  panelTracks: "Spuren",
  panelQc: "Qualität",
  windowFrames: "Frames",
  windowReport: "Bericht",
  layers: "Ebenen",
  layerMesh: "Mesh",
  layerDense: "Dichte Punktwolke",
  layerSparse: "Dünne Punktwolke",
  layerCameras: "Kameras",
  layerGcps: "Passpunkte",
};

/** 🌐️ Story-local mirror of `resolve_labels_for_locale::<RemodelingLabels>` — any non-`de` BCP-47 tag falls back to English, matching the framework resolver's language-prefix match. */
export function remodelLabelsFor(locale: string): RemodelLabels {
  return locale.toLowerCase().startsWith("de") ? REMODEL_LABELS_DE : REMODEL_LABELS_EN;
}
//#endregion 🔖️Labels
