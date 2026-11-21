# Retained Decimation Surface Fidelity

The Flow and Generation3d mesh catalogues have registered Decimate routes. Neither catalogue declares a Compact or Combine Mesh operator. Before this change, accepted Decimate collapses passed through the generic attribute remapper but had no raw surface projection. The rebuild created geometric placeholder point normals, omitted source face shading and zeroed raw halfedge UVs.

The retained collapse now drives the funded surface cursor before publishing its existing attribute remap. Surviving original point normals and UVs keep their samples. A moved point’s normal uses the same edge weights as its position and normalizes the resulting direction. A moved corner reads only the endpoints present on its original face. A face with both endpoints blends them, while each single-endpoint face keeps its independent source UV. Face shading and the existing directed edge seam provenance are retained. Missing or cancelling endpoint normals leave the existing geometric shading fallback available.

Flow and Generation3d use the same physical quote methods. Decimate already posts Work per event, and it carries one Work charge through each admitted surface event. A deficient quote pauses before the surface mutates. The registered Flow fixture checks three distinct corner UVs at the collapsed point and cancellation before publication.

The shared fixture has a quad with both collapse endpoints and two surviving triangles with one endpoint each. Independent Three.js calculations confirmed the expected simplified volume and normalized midpoint direction. The portable Mesh suite passed 21 tests with 4,251 assertions. The modified production reconstruction and 3D crates and the isolated runtime fixture typechecked with metadata-only compilation. Registered Flow source parsing and the repository source-size boundaries passed. The isolated runtime witness and direct Three.js comparison of its GLB await safe disk capacity. No new linked build ran on the volatile volume.

The last free-space check showed 317 MiB after a 365 MiB check earlier in the same pass. The runtime fixture was kept ready without requesting a native link on that volume.
