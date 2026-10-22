import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "mut"))
from recipes import *

calls = [
    call("interactivityMountedEngineSurfaceLifetimeFailures", "engineCanvasTree", "rendererHost", "winitHost", "browserWorker", "glueTree", "nodeGraph", "flowHost", "tiledMap", "editorTree", "dagNodes"),
    call("interactivityMountedPreparedRenderFailures", "preparedRaster", "preparedRasterDraw", "preparedRasterGpu", "rendererGlueRs", "uiFrame", "uiScene"),
    call("interactivityMountedSurfaceLaneFailures", "surfaceLane", "uiEngine", "rendererHost", "winitHost", "glueTree"),
]
generate("⚖laws/🔬interactivity-audit/🖥mounted/🖼mounted-surface", "🖼mounted-surface", "🖼 The mounted surface laws accept the live engine, host and renderer sources and reject them when a required anchor disappears or a forbidden one appears.", mounted_sources(), calls)
